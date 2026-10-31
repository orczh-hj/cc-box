//! 扩展编辑域：MCP / Skill / Agent / Hook 的表单化新增与编辑。
//!
//! 直接写 Claude Code 原生配置——`~/.claude.json`、`<proj>/.mcp.json`、
//! `skills/agents` 目录、`settings.json` 的 hooks——不建第二份状态。
//! 约定：
//! - 所有 JSON 写入走 `write_json_atomic`（临时文件 + rename）；
//! - 跨进程互斥锁文件放 `~/.cc-box/locks/<hash>.lock`（不能在项目根目录
//!   落 `config.lock`，那会污染用户的 git 工作区）；
//! - 删除类操作先在 `~/.cc-box/disabled/` 留备份（可逆原则）；
//! - 注入式核心 `*_in` 便于测试用临时目录替换真实路径。

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::providers::write_file_atomic;
use crate::store::{
    claude_config_dir, get_disabled_subdir, get_gui_config_dir, move_entry, path_entry_exists,
    validate_simple_name, write_json_atomic, HookItem,
};

// ==================== 通用工具 ====================

/// 路径/标识的稳定短哈希（跨进程一致：DefaultHasher 固定 key）
pub(crate) fn short_hash(input: &str) -> String {
    let mut h = DefaultHasher::new();
    input.hash(&mut h);
    format!("{:016x}", h.finish())
}

/// 项目级资源的 disabled 隔离目录名（按项目路径哈希，避免路径字符不安全）。
/// `p` 前缀 + 16 位 hex：get_hook_list 扫描 disabled/hooks 子目录时依赖此形态
pub(crate) fn project_key(project_path: &str) -> String {
    format!("p{}", short_hash(project_path))
}

/// 资源名的额外校验：目录/文件名形态（比 validate_simple_name 严：禁空格，
/// skill/agent 目录名即命令名，空格会让 CLI 调用歧义）
fn validate_resource_name(name: &str) -> Result<()> {
    validate_simple_name(name)?;
    if name.trim() != name || name.contains(' ') || name.len() > 64 {
        bail!("Invalid resource name: {:?}", name);
    }
    Ok(())
}

/// 目标文件的跨进程互斥锁（锁文件集中放 ~/.cc-box/locks/，不污染目标目录）。
/// 最多等 1s，超时放弃锁继续写（与 store.rs 的 acquire_config_file_lock 同策略）
fn lock_target(target: &Path) -> Option<crate::file_lock::FileLock> {
    let dir = get_gui_config_dir().ok()?.join("locks");
    let _ = std::fs::create_dir_all(&dir);
    let lock_path = dir.join(format!("{}.lock", short_hash(&target.to_string_lossy())));
    for _ in 0..40 {
        if let Ok(lock) = crate::file_lock::FileLock::try_exclusive(&lock_path) {
            return Some(lock);
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    log::warn!("[store-edit] lock timeout for {}; proceeding", target.display());
    None
}

fn read_json_or_empty(path: &Path) -> Result<serde_json::Value> {
    if !path.exists() {
        return Ok(serde_json::json!({}));
    }
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("Failed to parse {}", path.display()))
}

/// 读 JSON 文件；不存在时报错（删除/定位场景要求目标必须存在）
fn read_json_required(path: &Path, what: &str) -> Result<serde_json::Value> {
    if !path.exists() {
        bail!("{} not found: {}", what, path.display());
    }
    read_json_or_empty(path)
}

/// scope → 项目路径校验：project/local 必须带非空 project_path
fn require_project_path<'a>(scope: &str, project_path: Option<&'a str>) -> Result<&'a str> {
    match scope {
        "user" => Ok(""),
        "project" | "local" => {
            let p = project_path
                .map(|p| p.trim())
                .filter(|p| !p.is_empty())
                .context(format!("scope '{}' requires projectPath", scope))?;
            // 防御：远程端传任意路径在盘上乱写（项目路径必须真实存在）
            if !Path::new(p).is_dir() {
                bail!("Project path does not exist: {}", p);
            }
            Ok(p)
        }
        _ => bail!("Invalid scope: {:?}", scope),
    }
}

// ==================== MCP（三作用域读写） ====================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMcpRequest {
    /// user | project | local
    pub scope: String,
    pub name: String,
    /// server 配置对象（stdio: command/args/env；http/sse: url/headers）
    pub config: serde_json::Value,
    pub project_path: Option<String>,
    /// 重命名场景：原 key 名（存在则先移除）
    pub original_name: Option<String>,
    /// 跨作用域编辑：原作用域（不同则从旧作用域配置移除原条目）
    pub original_scope: Option<String>,
}

/// MCP server 配置的基本校验：object 且 stdio 必须有 command、http/sse 必须有 url
fn validate_mcp_config(config: &serde_json::Value) -> Result<()> {
    let obj = config
        .as_object()
        .context("MCP server config must be an object")?;
    if obj.contains_key("command") {
        if obj["command"].as_str().map(|s| s.trim().is_empty()).unwrap_or(true) {
            bail!("MCP stdio server requires non-empty command");
        }
        return Ok(());
    }
    if obj.contains_key("url") {
        if obj["url"].as_str().map(|s| s.trim().is_empty()).unwrap_or(true) {
            bail!("MCP http/sse server requires non-empty url");
        }
        return Ok(());
    }
    bail!("MCP server config requires command (stdio) or url (http/sse)");
}

/// MCP 配置文件定位：(文件路径, 嵌套 project 路径[local scope 用])
/// - user   → (~/.claude.json, None)
/// - local  → (~/.claude.json, Some(proj))：projects[proj].mcpServers
/// - project → (<proj>/.mcp.json, None)：根 mcpServers
/// （.claude.json 尊重 CLAUDE_CONFIG_DIR，与 Claude CLI 同一解析）
fn mcp_target(scope: &str, project_path: Option<&str>) -> Result<(PathBuf, Option<String>)> {
    let proj = require_project_path(scope, project_path)?;
    match scope {
        // user/local 共用 ~/.claude.json（local 在 projects[cwd] 嵌套）——与读取端
        // collect_mcp_config_sources 同一文件，绝不能写到 ~/.claude/.claude.json
        "user" => Ok((crate::store::claude_state_json_path()?, None)),
        "local" => Ok((crate::store::claude_state_json_path()?, Some(proj.to_string()))),
        "project" => Ok((PathBuf::from(proj).join(".mcp.json"), None)),
        _ => bail!("Invalid scope: {:?}", scope),
    }
}

/// 定位（或创建）mcpServers 对象；project_scope=Some 时按
/// projects[<p>].mcpServers 嵌套取
fn mcp_servers_obj<'a>(
    root: &'a mut serde_json::Value,
    project_scope: Option<&str>,
) -> Result<&'a mut serde_json::Map<String, serde_json::Value>> {
    if let Some(proj) = project_scope {
        let projects = root
            .as_object_mut()
            .context("config root is not an object")?
            .entry("projects".to_string())
            .or_insert_with(|| serde_json::json!({}));
        let entry = projects
            .as_object_mut()
            .context("projects is not an object")?
            .entry(proj.to_string())
            .or_insert_with(|| serde_json::json!({}));
        return entry
            .as_object_mut()
            .context("project entry is not an object")?
            .entry("mcpServers".to_string())
            .or_insert_with(|| serde_json::json!({}))
            .as_object_mut()
            .context("mcpServers is not an object");
    }
    root.as_object_mut()
        .context("config root is not an object")?
        .entry("mcpServers".to_string())
        .or_insert_with(|| serde_json::json!({}))
        .as_object_mut()
        .context("mcpServers is not an object")
}

/// 新增/更新单个 MCP server 条目（覆盖写语义：编辑即覆盖）
pub(crate) fn save_mcp_entry_in(
    file: &Path,
    project_scope: Option<&str>,
    name: &str,
    config: &serde_json::Value,
    original_name: Option<&str>,
) -> Result<()> {
    validate_simple_name(name)?;
    validate_mcp_config(config)?;
    let _lock = lock_target(file);
    let mut root = read_json_or_empty(file)?;

    {
        let obj = mcp_servers_obj(&mut root, project_scope)?;
        // 重命名：先移除旧 key（不存在不报错——可能与新名相同或已被外部删除）
        if let Some(orig) = original_name {
            if orig != name {
                obj.remove(orig);
            }
        }
        obj.insert(name.to_string(), config.clone());
    }
    write_json_atomic(file, &root)
}

/// 删除单个 MCP server 条目（备份到 disabled 目录，可逆）
pub(crate) fn delete_mcp_entry_in(
    file: &Path,
    project_scope: Option<&str>,
    disabled_dir: &Path,
    name: &str,
) -> Result<()> {
    validate_simple_name(name)?;
    let backup = disabled_dir.join(format!("{}.json", name));

    // 配置里移除；不存在不算错——条目可能正处于禁用态（只存在于 disabled 备份）
    let mut found = false;
    if file.exists() {
        let _lock = lock_target(file);
        let mut root = read_json_required(file, "config")?;
        let removed = {
            let obj = mcp_servers_obj(&mut root, project_scope)?;
            obj.remove(name)
        };
        if removed.is_some() {
            write_json_atomic(file, &root)?;
            found = true;
        }
    }

    // 彻底删除（二次确认后）：不再写 disabled 备份，历史/停用备份一并清理；
    // 可逆性由"开关"承担，删除即终态
    let backup_meta = std::fs::symlink_metadata(&backup);
    if let Ok(m) = &backup_meta {
        if m.is_dir() && !m.file_type().is_symlink() {
            std::fs::remove_dir_all(&backup)
                .with_context(|| format!("Failed to remove backup {}", backup.display()))?;
        } else {
            std::fs::remove_file(&backup)
                .with_context(|| format!("Failed to remove backup {}", backup.display()))?;
        }
    }

    if !found && backup_meta.is_err() {
        bail!("MCP server '{}' not found", name);
    }
    Ok(())
}

/// 公开 API：新增/更新 MCP server
pub fn save_mcp_server(req: SaveMcpRequest) -> Result<()> {
    // 跨作用域编辑：先从旧作用域配置移除原条目（直接移除不留备份——条目
    // 已随保存迁移到新作用域，不是删除）
    if let Some(orig_scope) = req.original_scope.as_deref() {
        if orig_scope != req.scope {
            let orig_name = req.original_name.as_deref().unwrap_or(&req.name);
            let (old_file, old_project_scope) = mcp_target(orig_scope, req.project_path.as_deref())?;
            if old_file.exists() {
                let _lock = lock_target(&old_file);
                let mut root = read_json_or_empty(&old_file)?;
                if let Ok(obj) = mcp_servers_obj(&mut root, old_project_scope.as_deref()) {
                    obj.remove(orig_name);
                }
                write_json_atomic(&old_file, &root)?;
            }
            log::info!("[store-edit] mcp '{}' moved {} -> {}", req.name, orig_scope, req.scope);
        }
    }
    let (file, project_scope) = mcp_target(&req.scope, req.project_path.as_deref())?;
    log::info!(
        "[store-edit] save mcp '{}' scope={} file={}",
        req.name,
        req.scope,
        file.display()
    );
    save_mcp_entry_in(&file, project_scope.as_deref(), &req.name, &req.config, req.original_name.as_deref())
}

/// 公开 API：删除 MCP server（user 备份到 disabled/mcp，project/local 各自隔离）
pub fn delete_mcp_server(scope: &str, name: &str, project_path: Option<&str>) -> Result<()> {
    let (file, project_scope) = mcp_target(scope, project_path)?;
    let backup_dir = match scope {
        "user" => get_disabled_subdir("mcp")?,
        "project" | "local" => {
            let dir = get_disabled_subdir("mcp")?
                .join(project_key(project_path.context("projectPath required")?));
            std::fs::create_dir_all(&dir)?;
            dir
        }
        _ => bail!("Invalid scope: {:?}", scope),
    };
    log::info!("[store-edit] delete mcp '{}' scope={}", name, scope);
    delete_mcp_entry_in(&file, project_scope.as_deref(), &backup_dir, name)
}

// ==================== Skill / Agent（目录 + Markdown 文件） ====================

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveSkillRequest {
    /// user | project
    pub scope: String,
    pub name: String,
    pub project_path: Option<String>,
    /// 重命名场景：原目录名
    pub original_name: Option<String>,
    /// 跨作用域编辑：原作用域（不同则先把旧目录整体 move 到新作用域）
    pub original_scope: Option<String>,
    /// false=表单模式（description + content 由后端拼 frontmatter）；
    /// true=源码模式（content 为 SKILL.md 全文）
    pub as_source: bool,
    pub description: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveAgentRequest {
    pub scope: String,
    pub name: String,
    pub project_path: Option<String>,
    pub original_name: Option<String>,
    pub original_scope: Option<String>,
    pub as_source: bool,
    pub description: Option<String>,
    /// 逗号分隔工具清单（可选）
    pub tools: Option<String>,
    /// 模型（可选）
    pub model: Option<String>,
    pub content: String,
}

/// YAML 双引号标量转义（serde_json 字符串转义与 YAML 双引号兼容，
/// 覆盖换行/引号/反斜杠等常见字符）
fn yaml_quote(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string())
}

/// 表单模式 → SKILL.md 全文
pub(crate) fn build_skill_md(name: &str, description: &str, content: &str) -> String {
    format!(
        "---\nname: {}\ndescription: {}\n---\n\n{}",
        name,
        yaml_quote(description),
        content.trim_end()
    )
    .trim_end()
    .to_string()
    + "\n"
}

/// 表单模式 → agent md 全文（tools/model 可选字段）
pub(crate) fn build_agent_md(
    name: &str,
    description: &str,
    tools: Option<&str>,
    model: Option<&str>,
    content: &str,
) -> String {
    let mut fm = format!(
        "---\nname: {}\ndescription: {}\n",
        name,
        yaml_quote(description)
    );
    if let Some(t) = tools.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        fm.push_str(&format!("tools: {}\n", t));
    }
    if let Some(m) = model.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        fm.push_str(&format!("model: {}\n", m));
    }
    fm.push_str("---\n\n");
    format!("{}{}", fm, content.trim_end()).trim_end().to_string() + "\n"
}

/// 校验 Markdown 源码模式的 frontmatter：必须以 --- 开头且有闭合 ---，
/// 且 name 字段（若存在）与资源名一致（Claude 以 frontmatter name 为命令名）
pub(crate) fn validate_md_frontmatter(source: &str, name: &str) -> Result<()> {
    let lines: Vec<&str> = source.lines().collect();
    if lines.first().map(|l| l.trim()) != Some("---") {
        bail!("Frontmatter must start with '---'");
    }
    let mut closed = false;
    let mut fm_name: Option<String> = None;
    let mut i = 1;
    while i < lines.len() {
        let line = lines[i].trim_end();
        if line.trim() == "---" {
            closed = true;
            break;
        }
        if let Some(rest) = line.strip_prefix("description:") {
            let v = rest.trim();
            if crate::store::is_block_scalar_indicator(v) {
                // 块标量体可能包含 --- 或 name:，跳过以免误判
                i = crate::store::parse_block_scalar(&lines, i + 1, v).1;
                continue;
            }
        }
        if let Some(rest) = line.strip_prefix("name:") {
            fm_name = Some(crate::store::unquote_yaml(rest));
        }
        i += 1;
    }
    if !closed {
        bail!("Frontmatter must be closed with '---'");
    }
    if let Some(n) = fm_name {
        if n != name {
            bail!(
                "Frontmatter name '{}' does not match resource name '{}' — keep them identical",
                n,
                name
            );
        }
    }
    Ok(())
}

fn skill_dir(scope: &str, project_path: Option<&str>) -> Result<PathBuf> {
    let proj = require_project_path(scope, project_path)?;
    match scope {
        "user" => Ok(claude_config_dir()?.join("skills")),
        "project" => Ok(PathBuf::from(proj).join(".claude").join("skills")),
        _ => bail!("Invalid scope: {:?}", scope),
    }
}

fn agent_dir(scope: &str, project_path: Option<&str>) -> Result<PathBuf> {
    let proj = require_project_path(scope, project_path)?;
    match scope {
        "user" => Ok(claude_config_dir()?.join("agents")),
        "project" => Ok(PathBuf::from(proj).join(".claude").join("agents")),
        _ => bail!("Invalid scope: {:?}", scope),
    }
}

fn scoped_disabled_dir(category: &str, scope: &str, project_path: Option<&str>) -> Result<PathBuf> {
    let base = get_disabled_subdir(category)?;
    let dir = if scope == "user" {
        base
    } else {
        base.join(project_key(project_path.context("projectPath required")?))
    };
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// 新增/更新 skill（目录 + SKILL.md）。重命名 = 整目录 move 后写入。
/// 返回写入的文件路径。
pub(crate) fn save_skill_in(
    skills_base: &Path,
    disabled_base: &Path,
    req: &SaveSkillRequest,
) -> Result<PathBuf> {
    validate_resource_name(&req.name)?;
    let dir = skills_base.join(&req.name);

    // 重命名：原目录 move 到新名（新目录已存在则报错，防覆盖他人内容）
    if let Some(orig) = req.original_name.as_deref() {
        if orig != req.name {
            validate_resource_name(orig)?;
            let old = skills_base.join(orig);
            if !path_entry_exists(&old) {
                bail!("Original skill '{}' not found", orig);
            }
            if path_entry_exists(&dir) {
                bail!("Skill '{}' already exists (conflict)", req.name);
            }
            move_entry(&old, &dir)
                .with_context(|| format!("Failed to rename '{}' to '{}'", orig, req.name))?;
        }
    }

    let content = if req.as_source {
        validate_md_frontmatter(&req.content, &req.name)?;
        req.content.clone()
    } else {
        let desc = req
            .description
            .as_deref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .context("description is required in form mode")?;
        build_skill_md(&req.name, desc, &req.content)
    };

    std::fs::create_dir_all(&dir)?;
    let file = dir.join("SKILL.md");
    write_file_atomic(&file, &content)?;
    let _ = disabled_base; // skill 保存不碰 disabled（开关走 set_skill_enabled）
    Ok(file)
}

/// 新增/更新 agent（单 md 文件）。返回写入路径。
pub(crate) fn save_agent_in(
    agents_base: &Path,
    req: &SaveAgentRequest,
) -> Result<PathBuf> {
    validate_resource_name(&req.name)?;
    std::fs::create_dir_all(agents_base)?;

    // 重命名：旧文件 move 到新名
    let file = agents_base.join(format!("{}.md", req.name));
    if let Some(orig) = req.original_name.as_deref() {
        if orig != req.name {
            validate_resource_name(orig)?;
            let old = agents_base.join(format!("{}.md", orig));
            if !path_entry_exists(&old) {
                bail!("Original agent '{}' not found", orig);
            }
            if path_entry_exists(&file) {
                bail!("Agent '{}' already exists (conflict)", req.name);
            }
            move_entry(&old, &file)
                .with_context(|| format!("Failed to rename '{}' to '{}'", orig, req.name))?;
        }
    }

    let content = if req.as_source {
        validate_md_frontmatter(&req.content, &req.name)?;
        req.content.clone()
    } else {
        let desc = req
            .description
            .as_deref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .context("description is required in form mode")?;
        build_agent_md(&req.name, desc, req.tools.as_deref(), req.model.as_deref(), &req.content)
    };

    write_file_atomic(&file, &content)?;
    Ok(file)
}

/// 彻底删除资源条目：active 侧删除 + disabled 侧同名备份一并清理（彻底删除的
/// 语义就是不留）。符号链接只删链接本身。
/// - skill：条目 = `<name>/` 目录（entry_file = None）
/// - agent：条目 = `<name>.md` 文件（entry_file = Some("<name>.md")，
///   条目名带 .md 后缀——曾漏拼后缀导致删除静默 not found）
pub(crate) fn delete_resource_in(
    active: &Path,
    disabled: &Path,
    name: &str,
    entry_file: Option<&str>,
) -> Result<()> {
    validate_simple_name(name)?;
    let entry = entry_file.unwrap_or(name);
    let target = active.join(entry);
    let active_meta = std::fs::symlink_metadata(&target).ok();
    if let Some(meta) = &active_meta {
        if meta.file_type().is_symlink() || meta.is_file() {
            std::fs::remove_file(&target)
                .with_context(|| format!("Failed to remove {}", target.display()))?;
        } else {
            std::fs::remove_dir_all(&target)
                .with_context(|| format!("Failed to remove {}", target.display()))?;
        }
    }
    // disabled 侧：条目处于禁用态时它就是全部内容；否则是历史备份，一并清理
    // （symlink_metadata：断链也列出/清理）
    let backup = disabled.join(entry);
    let backup_meta = std::fs::symlink_metadata(&backup).ok();
    if let Some(m) = &backup_meta {
        if m.is_dir() && !m.file_type().is_symlink() {
            let _ = std::fs::remove_dir_all(&backup);
        } else {
            let _ = std::fs::remove_file(&backup);
        }
    }
    if active_meta.is_none() && backup_meta.is_none() {
        bail!("'{}' not found in {}", name, active.display());
    }
    Ok(())
}

/// 读 SKILL.md 全文（编辑器源码模式）
pub(crate) fn read_resource_text(base: &Path, rel: &str) -> Result<(String, String)> {
    let path = base.join(rel);
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    Ok((content, path.to_string_lossy().to_string()))
}

// ---- 公开 API：Skill ----

/// 跨作用域 move：旧 base 的条目（含 disabled 备份，若在）搬到新 base。
/// 名称可同时变更（一步到位 move，之后 save_*_in 不再做同 base 重命名）。
fn move_resource_across_scopes(
    category: &str,
    old_base: &Path,
    new_base: &Path,
    old_name: &str,
    new_name: &str,
    scope_old: &str,
    scope_new: &str,
    project_path: Option<&str>,
) -> Result<()> {
    let old = old_base.join(old_name);
    if !path_entry_exists(&old) {
        return Ok(()); // 旧作用域已无此条目（可能被外部删除），直接按新增处理
    }
    let new = new_base.join(new_name);
    if path_entry_exists(&new) {
        bail!("'{}' already exists in {} scope (conflict)", new_name, scope_new);
    }
    std::fs::create_dir_all(new_base)?;
    move_entry(&old, &new).with_context(|| format!("Failed to move '{}' to {} scope", old_name, scope_new))?;
    // 旧作用域的 disabled 备份随迁（条目禁用状态跟随作用域）
    if let (Ok(old_disabled), Ok(new_disabled)) = (
        scoped_disabled_dir(category, scope_old, project_path),
        scoped_disabled_dir(category, scope_new, project_path),
    ) {
        let backup = old_disabled.join(old_name);
        if path_entry_exists(&backup) {
            std::fs::create_dir_all(&new_disabled).ok();
            let _ = move_entry(&backup, &new_disabled.join(new_name));
        }
    }
    Ok(())
}

pub fn save_skill(req: SaveSkillRequest) -> Result<String> {
    let mut req = req;
    if let Some(orig_scope) = req.original_scope.clone() {
        if orig_scope != req.scope {
            let old_base = skill_dir(&orig_scope, req.project_path.as_deref())?;
            let base = skill_dir(&req.scope, req.project_path.as_deref())?;
            let old_name = req.original_name.clone().unwrap_or_else(|| req.name.clone());
            move_resource_across_scopes(
                "skills",
                &old_base,
                &base,
                &old_name,
                &req.name,
                &orig_scope,
                &req.scope,
                req.project_path.as_deref(),
            )?;
            // move 已含改名与跨目录，save_skill_in 不再重复处理重命名
            req.original_name = None;
            log::info!("[store-edit] skill '{}' moved {} -> {}", req.name, orig_scope, req.scope);
        }
    }
    let base = skill_dir(&req.scope, req.project_path.as_deref())?;
    let disabled_base = scoped_disabled_dir("skills", &req.scope, req.project_path.as_deref())?;
    let path = save_skill_in(&base, &disabled_base, &req)?;
    log::info!("[store-edit] save skill '{}' scope={}", req.name, req.scope);
    Ok(path.to_string_lossy().to_string())
}

pub fn delete_skill(scope: &str, name: &str, project_path: Option<&str>) -> Result<()> {
    let base = skill_dir(scope, project_path)?;
    let disabled = scoped_disabled_dir("skills", scope, project_path)?;
    log::info!("[store-edit] delete skill '{}' scope={}", name, scope);
    delete_resource_in(&base, &disabled, name, None)
}

pub fn get_skill_source(scope: &str, name: &str, project_path: Option<&str>) -> Result<(String, String)> {
    let base = skill_dir(scope, project_path)?;
    validate_simple_name(name)?;
    if path_entry_exists(&base.join(name)) {
        return read_resource_text(&base, &format!("{}/SKILL.md", name));
    }
    // 条目已被禁用（移入 ~/.cc-box/disabled/）：从备份目录读，
    // 否则面板灰显条目点开编辑会报「Failed to read ...\.claude\skills\...」
    let disabled = scoped_disabled_dir("skills", scope, project_path)?;
    let backup = disabled.join(name).join("SKILL.md");
    if backup.exists() {
        let content = std::fs::read_to_string(&backup)
            .with_context(|| format!("Failed to read {}", backup.display()))?;
        return Ok((content, backup.to_string_lossy().to_string()));
    }
    read_resource_text(&base, &format!("{}/SKILL.md", name))
}

// ---- 公开 API：Agent ----

pub fn save_agent(req: SaveAgentRequest) -> Result<String> {
    let mut req = req;
    if let Some(orig_scope) = req.original_scope.clone() {
        if orig_scope != req.scope {
            let old_base = agent_dir(&orig_scope, req.project_path.as_deref())?;
            let base = agent_dir(&req.scope, req.project_path.as_deref())?;
            let old_name = req.original_name.clone().unwrap_or_else(|| req.name.clone());
            move_resource_across_scopes(
                "agents",
                &old_base,
                &base,
                &old_name,
                &req.name,
                &orig_scope,
                &req.scope,
                req.project_path.as_deref(),
            )?;
            req.original_name = None;
            log::info!("[store-edit] agent '{}' moved {} -> {}", req.name, orig_scope, req.scope);
        }
    }
    let base = agent_dir(&req.scope, req.project_path.as_deref())?;
    let path = save_agent_in(&base, &req)?;
    log::info!("[store-edit] save agent '{}' scope={}", req.name, req.scope);
    Ok(path.to_string_lossy().to_string())
}

pub fn delete_agent(scope: &str, name: &str, project_path: Option<&str>) -> Result<()> {
    let base = agent_dir(scope, project_path)?;
    let disabled = scoped_disabled_dir("agents", scope, project_path)?;
    log::info!("[store-edit] delete agent '{}' scope={}", name, scope);
    delete_resource_in(&base, &disabled, name, Some(&format!("{}.md", name)))
}

pub fn get_agent_source(scope: &str, name: &str, project_path: Option<&str>) -> Result<(String, String)> {
    let base = agent_dir(scope, project_path)?;
    validate_simple_name(name)?;
    if path_entry_exists(&base.join(format!("{}.md", name))) {
        return read_resource_text(&base, &format!("{}.md", name));
    }
    // 禁用条目兜底：从 disabled 备份目录读（见 get_skill_source）
    let disabled = scoped_disabled_dir("agents", scope, project_path)?;
    let backup = disabled.join(format!("{}.md", name));
    if backup.exists() {
        let content = std::fs::read_to_string(&backup)
            .with_context(|| format!("Failed to read {}", backup.display()))?;
        return Ok((content, backup.to_string_lossy().to_string()));
    }
    read_resource_text(&base, &format!("{}.md", name))
}

// ---- 项目级开关补齐（skill/agent）----

/// 项目级 skill 开关：active=<proj>/.claude/skills，disabled 按 project_key 隔离
pub fn set_skill_enabled_scoped(
    scope: &str,
    name: &str,
    enabled: bool,
    project_path: Option<&str>,
) -> Result<()> {
    if scope == "user" {
        return crate::store::set_skill_enabled(name, enabled);
    }
    let proj = require_project_path(scope, project_path)?;
    let claude_base = PathBuf::from(proj).join(".claude").join("skills");
    let disabled_base = scoped_disabled_dir("skills", scope, project_path)?;
    crate::store::set_skill_enabled_in(&claude_base, &disabled_base, name, enabled)
}

/// 项目级 agent 开关
pub fn set_agent_enabled_scoped(
    scope: &str,
    name: &str,
    enabled: bool,
    project_path: Option<&str>,
) -> Result<()> {
    if scope == "user" {
        return crate::store::set_agent_enabled(name, enabled);
    }
    let proj = require_project_path(scope, project_path)?;
    let claude_base = PathBuf::from(proj).join(".claude").join("agents");
    let disabled_base = scoped_disabled_dir("agents", scope, project_path)?;
    crate::store::set_agent_enabled_in(&claude_base, &disabled_base, name, enabled)
}

/// 项目/本地级 MCP 开关：
/// - project → <proj>/.mcp.json（根 mcpServers，形状与 ~/.claude.json 相同，
///   直接复用 set_mcp_server_enabled_in）
/// - local   → ~/.claude.json 的 projects[<p>].mcpServers（嵌套路径单独实现）
pub fn set_mcp_server_enabled_scoped(
    scope: &str,
    name: &str,
    enabled: bool,
    project_path: Option<&str>,
) -> Result<()> {
    if scope == "user" {
        return crate::store::set_mcp_server_enabled(name, enabled);
    }
    let proj = require_project_path(scope, project_path)?;
    let disabled_dir = scoped_disabled_dir("mcp", scope, project_path)?;
    match scope {
        "project" => {
            let file = PathBuf::from(proj).join(".mcp.json");
            crate::store::set_mcp_server_enabled_in(&file, &disabled_dir, name, enabled)
        }
        "local" => {
            // 与 mcp_target 同源（尊重 CLAUDE_CONFIG_DIR），避免开关与编辑读写不同文件
            let file = crate::store::claude_state_json_path()?;
            set_mcp_server_local_in(&file, &disabled_dir, proj, name, enabled)
        }
        _ => bail!("Invalid scope: {:?}", scope),
    }
}

/// local 作用域的 MCP 剪切/粘贴（projects[<proj>].mcpServers 嵌套）
pub(crate) fn set_mcp_server_local_in(
    claude_json: &Path,
    disabled_dir: &Path,
    project: &str,
    name: &str,
    enabled: bool,
) -> Result<()> {
    validate_simple_name(name)?;
    let backup_file = disabled_dir.join(format!("{}.json", name));
    let _lock = lock_target(claude_json);
    let root = if enabled {
        // backup → projects[proj].mcpServers.<name>
        if !backup_file.exists() {
            bail!("MCP server '{}' is not disabled", name);
        }
        let entry: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&backup_file)
                .with_context(|| format!("Failed to read backup for '{}'", name))?,
        )
        .with_context(|| format!("Invalid JSON in backup for '{}'", name))?;
        let mut root = read_json_or_empty(claude_json)?;
        let obj = mcp_servers_obj(&mut root, Some(project))?;
        if obj.contains_key(name) {
            bail!("MCP server '{}' already exists in active config (conflict)", name);
        }
        obj.insert(name.to_string(), entry);
        std::fs::remove_file(&backup_file)
            .with_context(|| format!("Failed to remove backup for '{}'", name))?;
        root
    } else {
        // projects[proj].mcpServers.<name> → backup
        let mut root = read_json_required(claude_json, ".claude.json")?;
        let obj = mcp_servers_obj(&mut root, Some(project))?;
        let entry = obj
            .remove(name)
            .with_context(|| format!("MCP server '{}' not found in active config", name))?;
        std::fs::create_dir_all(disabled_dir)?;
        std::fs::write(&backup_file, serde_json::to_string_pretty(&entry)?)?;
        root
    };
    // 顺带清理空的 projects[proj].mcpServers / 空的项目节点？不清理——
    // 保留结构，避免 Claude 侧 projects 元数据被误动
    write_json_atomic(claude_json, &root)
}

// ==================== Hook（settings.json hooks 合并） ====================

/// 已知 hook 事件（与 Claude Code HooksSchema 的 HOOK_EVENTS 对齐；新事件
/// 由前端下拉提供，后端只做形态校验，向前兼容新事件名）
pub(crate) fn validate_hook_event(event: &str) -> Result<()> {
    if event.len() < 2
        || event.len() > 40
        || !event
            .chars()
            .all(|c| c.is_ascii_alphabetic() || c == '_')
    {
        bail!("Invalid hook event: {:?}", event);
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveHookRequest {
    /// user | project | local
    pub scope: String,
    pub event: String,
    pub matcher: Option<String>,
    /// 要写入的完整 hook 对象（type: command|http|prompt|agent|mcp_tool…，
    /// 含 command/url/prompt/timeout 等各自字段）
    pub hook: serde_json::Value,
    pub project_path: Option<String>,
    /// 编辑场景：被替换条目的原定位（key = 列表下发的条目键）
    pub original: Option<HookOriginal>,
    /// 跨作用域编辑：原作用域（不同则从旧 settings 文件移除原条目后按新增写入）
    pub original_scope: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HookOriginal {
    pub event: String,
    pub matcher: Option<String>,
    /// 被替换条目的定位键（get_hook_list 下发的 HookListItem.key）
    pub key: String,
}

/// hook 条目的定位键（settings.json 的 hooks 条目无 id，用三元组哈希）
pub(crate) fn hook_key(event: &str, matcher: Option<&str>, signature: &str) -> String {
    short_hash(&format!(
        "{}|{}|{}",
        event,
        matcher.unwrap_or(""),
        signature
    ))
}

/// hook 对象的稳定签名：command 类型（老数据、老 disabled 备份兼容）保持
/// command 原文；其他类型（http/prompt/agent/mcp_tool…）用 hook 对象的
/// canonical JSON（**显式按键排序**——不依赖 serde_json 非 preserve_order
/// feature 的 Map 实现，键序不受依赖 feature 影响）
pub(crate) fn hook_signature(hook0: &serde_json::Value) -> String {
    let is_command = hook0
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("command")
        == "command";
    if is_command {
        if let Some(c) = hook0.get("command").and_then(|v| v.as_str()) {
            return c.to_string();
        }
    }
    canonical_json(hook0)
}

/// 递归键排序的 canonical JSON 序列化（签名用）
fn canonical_json(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::Object(m) => {
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort();
            let body: Vec<String> = keys
                .into_iter()
                .map(|k| {
                    format!(
                        "{}:{}",
                        serde_json::to_string(k).unwrap_or_default(),
                        canonical_json(&m[k])
                    )
                })
                .collect();
            format!("{{{}}}", body.join(","))
        }
        serde_json::Value::Array(a) => {
            let body: Vec<String> = a.iter().map(canonical_json).collect();
            format!("[{}]", body.join(","))
        }
        other => serde_json::to_string(other).unwrap_or_default(),
    }
}

/// matcher 层条目的定位键 = hash(event | matcher | hook0 签名)
pub(crate) fn hook_entry_key(event: &str, matcher: Option<&str>, entry: &serde_json::Value) -> String {
    let sig = entry
        .get("hooks")
        .and_then(|h| h.as_array())
        .and_then(|a| a.first())
        .map(hook_signature)
        .unwrap_or_default();
    hook_key(event, matcher, &sig)
}

fn hook_settings_path(scope: &str, project_path: Option<&str>) -> Result<PathBuf> {
    let proj = require_project_path(scope, project_path)?;
    match scope {
        "user" => Ok(claude_config_dir()?.join("settings.json")),
        "project" => Ok(PathBuf::from(proj).join(".claude").join("settings.json")),
        "local" => Ok(PathBuf::from(proj).join(".claude").join("settings.local.json")),
        _ => bail!("Invalid scope: {:?}", scope),
    }
}

/// 构造 matcher 层条目：{ matcher?, hooks: [<hook 对象>] }
pub(crate) fn build_hook_entry(matcher: Option<&str>, hook: &serde_json::Value) -> serde_json::Value {
    let mut entry = serde_json::json!({ "hooks": [hook] });
    if let Some(m) = matcher.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        entry["matcher"] = serde_json::json!(m);
    }
    entry
}

/// 在 hooks[event] 数组中按条目键定位（matcher 相等 + hook0 签名一致的
/// 哈希键，见 hook_entry_key）。返回数组内下标。
pub(crate) fn find_hook_index(
    entries: &[serde_json::Value],
    event: &str,
    matcher: Option<&str>,
    key: &str,
) -> Option<usize> {
    let want = matcher.map(|s| s.trim()).filter(|s| !s.is_empty());
    entries.iter().position(|e| {
        let m = e
            .get("matcher")
            .and_then(|v| v.as_str())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());
        m == want && hook_entry_key(event, matcher, e) == key
    })
}

/// 在 settings.json 的 hooks 中定位条目（同 find_hook_index，供删除/替换公用）
fn locate_hook(
    root: &serde_json::Value,
    event: &str,
    matcher: Option<&str>,
    key: &str,
) -> Option<usize> {
    root.get("hooks")?
        .get(event)?
        .as_array()
        .and_then(|entries| find_hook_index(entries, event, matcher, key))
}

/// 校验 hook 对象基本形态：object；command 类型必须有非空 command
fn validate_hook_object(hook: &serde_json::Value) -> Result<()> {
    let obj = hook.as_object().context("hook must be an object")?;
    let ty = obj.get("type").and_then(|v| v.as_str()).unwrap_or("command");
    if ty == "command" {
        let cmd = obj.get("command").and_then(|v| v.as_str()).unwrap_or("");
        if cmd.trim().is_empty() {
            bail!("Hook command is required");
        }
    }
    Ok(())
}

/// 新增/替换 hook 条目
pub(crate) fn save_hook_in(settings: &Path, req: &SaveHookRequest) -> Result<()> {
    validate_hook_event(&req.event)?;
    validate_hook_object(&req.hook)?;
    let _lock = lock_target(settings);
    let mut root = read_json_or_empty(settings)?;

    {
        let hooks = root
            .as_object_mut()
            .context("settings root is not an object")?
            .entry("hooks".to_string())
            .or_insert_with(|| serde_json::json!({}))
            .as_object_mut()
            .context("hooks is not an object")?;
        let new_entry = build_hook_entry(req.matcher.as_deref(), &req.hook);
        let new_key = hook_entry_key(&req.event, req.matcher.as_deref(), &new_entry);

        match &req.original {
            Some(orig) if orig.event != req.event => {
                // 跨事件编辑：从原事件数组移除，向新事件数组追加
                validate_hook_event(&orig.event)?;
                let old_list = hooks
                    .get_mut(&orig.event)
                    .and_then(|v| v.as_array_mut())
                    .context("original hook event not found or not an array")?;
                let idx = find_hook_index(old_list, &orig.event, orig.matcher.as_deref(), &orig.key)
                    .context("配置已被外部修改，请刷新后重试")?;
                old_list.remove(idx);
                if old_list.is_empty() {
                    hooks.remove(&orig.event);
                }
                let list = hooks
                    .entry(req.event.clone())
                    .or_insert_with(|| serde_json::json!([]))
                    .as_array_mut()
                    .context("hook event value is not an array")?;
                if find_hook_index(list, &req.event, req.matcher.as_deref(), &new_key).is_some() {
                    bail!("相同的钩子已存在");
                }
                list.push(new_entry);
            }
            Some(orig) => {
                let list = hooks
                    .entry(req.event.clone())
                    .or_insert_with(|| serde_json::json!([]))
                    .as_array_mut()
                    .context("hook event value is not an array")?;
                let idx = find_hook_index(list, &req.event, orig.matcher.as_deref(), &orig.key)
                    .context("配置已被外部修改，请刷新后重试")?;
                // 替换后不得与另一条目重复（编辑成相同 matcher+hook）
                for (i, e) in list.iter().enumerate() {
                    if i != idx
                        && find_hook_index(std::slice::from_ref(e), &req.event, req.matcher.as_deref(), &new_key)
                            .is_some()
                    {
                        bail!("相同的钩子已存在");
                    }
                }
                list[idx] = new_entry;
            }
            None => {
                let list = hooks
                    .entry(req.event.clone())
                    .or_insert_with(|| serde_json::json!([]))
                    .as_array_mut()
                    .context("hook event value is not an array")?;
                if find_hook_index(list, &req.event, req.matcher.as_deref(), &new_key).is_some() {
                    bail!("相同的钩子已存在");
                }
                list.push(new_entry);
            }
        }
    }

    write_json_atomic(settings, &root)
}

/// 删除 hook 条目（彻底删除：二次确认后直接移除，连同历史 disabled 备份清理）。
/// 条目处于禁用态时只存在于 disabled 备份——删除备份即完成删除。
/// hook 备份文件名：带 scope 区分（project/local 共用 project_key 目录，
/// 同名条目跨 scope 的备份不能互相覆盖）；早期无 scope 版本一并兼容清理
fn hook_backup_file(disabled_dir: &Path, key: &str, scope: &str) -> PathBuf {
    disabled_dir.join(format!("hook-{}-{}.json", scope, key))
}

pub(crate) fn delete_hook_in(
    settings: &Path,
    disabled_dir: &Path,
    event: &str,
    key: &str,
    scope: &str,
) -> Result<()> {
    validate_hook_event(event)?;
    if key.is_empty() {
        bail!("Hook key is required");
    }
    let backup = hook_backup_file(disabled_dir, key, scope);
    let legacy_backup = disabled_dir.join(format!("hook-{}.json", key));

    // settings 里定位；不存在（未配置/条目已禁用）不算错
    let mut root = serde_json::json!({});
    let mut idx: Option<usize> = None;
    if settings.exists() {
        let _lock = lock_target(settings);
        root = read_json_required(settings, "settings")?;
        idx = root
            .get("hooks")
            .and_then(|h| h.get(event))
            .and_then(|v| v.as_array())
            .and_then(|list| {
                list.iter()
                    .position(|e| hook_entry_key(event, entry_matcher(e), e) == key)
            });
        if let Some(i) = idx {
            let hooks = root
                .get_mut("hooks")
                .and_then(|v| v.as_object_mut())
                .context("hooks is not an object")?;
            let list = hooks
                .get_mut(event)
                .and_then(|v| v.as_array_mut())
                .context("hook event not found")?;
            list.remove(i);
            if list.is_empty() {
                hooks.remove(event);
            }
            write_json_atomic(settings, &root)?;
        }
    }

    // 彻底删除（二次确认后）：不再写 disabled 备份，历史/停用备份一并清理
    let mut had_backup = backup.exists();
    if had_backup {
        std::fs::remove_file(&backup)
            .with_context(|| format!("Failed to remove backup {}", backup.display()))?;
    }
    if legacy_backup.exists() {
        had_backup = true;
        let _ = std::fs::remove_file(&legacy_backup);
    }

    if idx.is_none() && !had_backup {
        bail!("配置已被外部修改，请刷新后重试");
    }
    Ok(())
}

/// 读条目的 matcher（删除按 key 定位时不再经过参数传递）
fn entry_matcher(entry: &serde_json::Value) -> Option<&str> {
    entry
        .get("matcher")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
}

/// hook 启停：禁用 = 从 settings 摘出存 disabled；启用 = 从 disabled 找回写回
pub(crate) fn set_hook_enabled_in(
    settings: &Path,
    disabled_dir: &Path,
    event: &str,
    key: &str,
    enabled: bool,
    scope: &str,
    project_path: Option<&str>,
) -> Result<()> {
    let backup_file = hook_backup_file(disabled_dir, key, scope);
    if enabled {
        if !backup_file.exists() {
            bail!("Hook is not disabled");
        }
        let backup: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&backup_file)
                .with_context(|| "Failed to read hook backup")?,
        )
        .with_context(|| "Invalid JSON in hook backup")?;
        let entry = backup
            .get("entry")
            .cloned()
            .context("hook backup missing entry")?;
        let _lock = lock_target(settings);
        let mut root = read_json_or_empty(settings)?;
        {
            let hooks = root
                .as_object_mut()
                .context("settings root is not an object")?
                .entry("hooks".to_string())
                .or_insert_with(|| serde_json::json!({}))
                .as_object_mut()
                .context("hooks is not an object")?;
            let list = hooks
                .entry(event.to_string())
                .or_insert_with(|| serde_json::json!([]))
                .as_array_mut()
                .context("hook event value is not an array")?;
            let entry_key = hook_entry_key(event, entry_matcher(&entry), &entry);
            if list
                .iter()
                .any(|e| hook_entry_key(event, entry_matcher(e), e) == entry_key)
            {
                bail!("相同的钩子已存在");
            }
            list.push(entry);
        }
        write_json_atomic(settings, &root)?;
        std::fs::remove_file(&backup_file)
            .with_context(|| "Failed to remove hook backup")?;
    } else {
        // 禁用 = 摘出存 disabled 备份（可逆；与删除的彻底移除区分）
        validate_hook_event(event)?;
        if !settings.exists() {
            bail!("Settings file not found: {}", settings.display());
        }
        let _lock = lock_target(settings);
        let mut root = read_json_required(settings, "settings")?;
        let idx = {
            let list = root
                .get("hooks")
                .and_then(|h| h.get(event))
                .and_then(|v| v.as_array())
                .context("配置已被外部修改，请刷新后重试")?;
            list.iter()
                .position(|e| hook_entry_key(event, entry_matcher(e), e) == key)
                .context("配置已被外部修改，请刷新后重试")?
        };
        let entry = root["hooks"][event][idx].clone();
        std::fs::create_dir_all(disabled_dir)?;
        let backup = serde_json::json!({
            "key": key,
            "event": event,
            "entry": entry,
            "scope": scope,
            "projectPath": project_path,
        });
        std::fs::write(
            &backup_file,
            serde_json::to_string_pretty(&backup)?,
        )?;
        {
            let hooks = root
                .get_mut("hooks")
                .and_then(|v| v.as_object_mut())
                .context("hooks is not an object")?;
            let list = hooks
                .get_mut(event)
                .and_then(|v| v.as_array_mut())
                .context("hook event not found")?;
            list.remove(idx);
            if list.is_empty() {
                hooks.remove(event);
            }
        }
        write_json_atomic(settings, &root)?;
    }
    Ok(())
}

/// 面板数据：settings 三来源 + disabled 备份合并，含 enabled 标记与定位 key
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HookListItem {
    #[serde(flatten)]
    pub item: HookItem,
    pub enabled: bool,
    /// 定位/开关用的稳定键
    pub key: String,
    /// 原始作用域（停用条目启用时需要回到原 settings 文件）
    pub scope: String,
    /// 项目路径（project/local 作用域）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_path: Option<String>,
}

/// 按 key 从 settings 移除条目（不备份——跨作用域 move 语义，条目已迁移）。
/// 条目不存在时静默成功。
fn remove_hook_entry_in(settings: &Path, event: &str, key: &str) -> Result<()> {
    if !settings.exists() {
        return Ok(());
    }
    let _lock = lock_target(settings);
    let mut root = read_json_or_empty(settings)?;
    let mut removed = false;
    {
        let Some(hooks) = root.get_mut("hooks").and_then(|v| v.as_object_mut()) else {
            return Ok(());
        };
        let Some(list) = hooks.get_mut(event).and_then(|v| v.as_array_mut()) else {
            return Ok(());
        };
        if let Some(idx) = list.iter().position(|e| hook_entry_key(event, entry_matcher(e), e) == key) {
            list.remove(idx);
            removed = true;
            if list.is_empty() {
                hooks.remove(event);
            }
        }
    }
    if removed {
        write_json_atomic(settings, &root)?;
    }
    Ok(())
}

/// 公开 API：新增/替换 hook
pub fn save_hook(req: SaveHookRequest) -> Result<()> {
    let mut req = req;
    // 跨作用域：先从旧 settings 文件移除原条目，再按新增写入新作用域
    if let (Some(orig_scope), Some(orig)) = (req.original_scope.clone(), req.original.clone()) {
        if orig_scope != req.scope {
            let old_path = hook_settings_path(&orig_scope, req.project_path.as_deref())?;
            remove_hook_entry_in(&old_path, &orig.event, &orig.key)?;
            req.original = None;
            log::info!("[store-edit] hook '{}' moved {} -> {}", req.event, orig_scope, req.scope);
        }
    }
    let path = hook_settings_path(&req.scope, req.project_path.as_deref())?;
    log::info!("[store-edit] save hook {} scope={}", req.event, req.scope);
    save_hook_in(&path, &req)
}

/// 公开 API：删除 hook
pub fn delete_hook(
    scope: &str,
    event: &str,
    key: &str,
    project_path: Option<&str>,
) -> Result<()> {
    let path = hook_settings_path(scope, project_path)?;
    let disabled = scoped_disabled_dir("hooks", scope, project_path)?;
    log::info!("[store-edit] delete hook {} scope={}", event, scope);
    delete_hook_in(&path, &disabled, event, key, scope)
}

/// 公开 API：hook 启停
pub fn set_hook_enabled(
    scope: &str,
    event: &str,
    key: &str,
    enabled: bool,
    project_path: Option<&str>,
) -> Result<()> {
    let path = hook_settings_path(scope, project_path)?;
    let disabled = scoped_disabled_dir("hooks", scope, project_path)?;
    log::info!("[store-edit] set hook {} enabled={} scope={}", event, enabled, scope);
    set_hook_enabled_in(&path, &disabled, event, key, enabled, scope, project_path)
}

/// 公开 API：hook 面板列表（三来源 + disabled 合并）
pub fn get_hook_list(project_path: &str) -> Result<Vec<HookListItem>> {
    let mut out: Vec<HookListItem> = Vec::new();

    // 三来源启用条目（复用 store 的读取实现）
    for item in crate::store::read_hooks_config(project_path)? {
        let key = item
            .raw
            .as_ref()
            .map(|e| hook_entry_key(&item.event, item.matcher.as_deref(), e))
            .unwrap_or_default();
        let scope = item.source.source_type.clone();
        let project_path =
            if scope == "user" { None } else { Some(project_path.to_string()) };
        out.push(HookListItem {
            item,
            enabled: true,
            key,
            scope,
            project_path,
        });
    }

    // disabled 备份（各 scope 的隔离目录都扫，前端按 source 展示）
    let base = get_disabled_subdir("hooks")?;
    let mut dirs: Vec<(PathBuf, &str)> = vec![(base.clone(), "user")];
    if let Ok(entries) = std::fs::read_dir(&base) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with('p') && name.len() == 17 && e.path().is_dir() {
                dirs.push((e.path(), "project"));
            }
        }
    }
    for (dir, scope_label) in dirs {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().map(|x| x != "json").unwrap_or(true) {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(&p) else { continue };
            let Ok(backup) = serde_json::from_str::<serde_json::Value>(&content) else { continue };
            let (Some(event), Some(entry)) = (
                backup.get("event").and_then(|v| v.as_str()),
                backup.get("entry"),
            ) else { continue };
            let matcher = entry.get("matcher").and_then(|v| v.as_str()).map(|s| s.to_string());
            let first = crate::store::hook_first(entry);
            let command = crate::store::hook_summary(first);
            // 停用条目按备份记录的原始 scope 还原开关（来源标签仍标 disabled 便于区分）
            let scope = backup
                .get("scope")
                .and_then(|v| v.as_str())
                .unwrap_or("user")
                .to_string();
            let project_path = backup
                .get("projectPath")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let hook_count = entry
                .get("hooks")
                .and_then(|h| h.as_array())
                .map(|a| a.len())
                .unwrap_or(1);
            let timeout = first
                .and_then(|h| h.get("timeout"))
                .and_then(|v| v.as_u64());
            out.push(HookListItem {
                item: HookItem {
                    event: event.to_string(),
                    matcher,
                    command,
                    hook_type: first
                        .and_then(|h| h.get("type"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    hook_count,
                    timeout,
                    source: crate::store::ConfigSource {
                        source_type: scope.clone(),
                        label: format!("{} (disabled)", scope_label),
                        path: Some(p.to_string_lossy().to_string()),
                    },
                    raw: Some(entry.clone()),
                },
                enabled: false,
                key: backup
                    .get("key")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                scope,
                project_path,
            });
        }
    }

    // 稳定排序：事件 → matcher → command
    out.sort_by(|a, b| {
        a.item
            .event
            .cmp(&b.item.event)
            .then_with(|| a.item.matcher.cmp(&b.item.matcher))
            .then_with(|| a.item.command.cmp(&b.item.command))
    });
    Ok(out)
}

// ==================== 外部打开（在文件管理器中定位） ====================

/// 在系统文件管理器中定位文件（实现见 platform，内部统一封装各平台命令）
pub fn reveal_in_file_manager(path: &str) -> Result<()> {
    crate::platform::reveal_in_file_manager(path).map_err(anyhow::Error::msg)
}
