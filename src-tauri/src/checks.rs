//! 环境检测模块
//!
//! 检查 Claude CLI 和 Git Bash 是否可用。
//! 与执行层共用 `platform::cli_target` 的判定；检查是只读的——
//! 自动检测结果只作运行态返回，不写持久化配置。

use serde::{Deserialize, Serialize};
use std::path::Path;

/// 检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    /// 稳定结论码（前端 i18n 渲染键；message 仅作日志与无 code 旧端的回退）
    pub code: String,
    pub message: String,
    /// 检测到的路径（无论通过与否，只要找到了就带上）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_path: Option<String>,
    /// 来源标记："configured"（用户配置）| "path"（PATH 检测）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 动作 i18n 键（如 viewInstallationGuide）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CheckResult {
    fn pass_with_path(name: &str, code: &str, message: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            passed: true,
            code: code.to_string(),
            message: message.to_string(),
            detected_path: Some(path.to_string()),
            source: None,
            action: None,
            url: None,
        }
    }

    fn pass_with_path_source(
        name: &str,
        code: &str,
        message: &str,
        path: &str,
        source: &str,
    ) -> Self {
        Self {
            source: Some(source.to_string()),
            ..Self::pass_with_path(name, code, message, path)
        }
    }

    fn fail_with_path(
        name: &str,
        code: &str,
        message: String,
        path: Option<String>,
        action: &str,
        url: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            passed: false,
            code: code.to_string(),
            message,
            detected_path: path,
            source: None,
            action: Some(action.to_string()),
            url: Some(url.to_string()),
        }
    }
}

/// 检查结果集合
#[derive(Debug, Clone, Serialize)]
pub struct ChecksResult {
    pub checks: Vec<CheckResult>,
}

impl ChecksResult {
    pub fn all_passed(&self) -> bool {
        self.checks.iter().all(|c| c.passed)
    }

    pub fn failed_checks(&self) -> Vec<&CheckResult> {
        self.checks.iter().filter(|c| !c.passed).collect()
    }
}

/// 运行所有检查
///
/// 只读：自动检测结果仅作为运行时结论返回，不写持久化配置。
/// 持久化路径只能由用户设置或安装器写入。
pub fn run_checks() -> ChecksResult {
    // 修复 GUI 应用 PATH 继承问题（PATH 来源须与执行 shell 一致）
    crate::platform::refresh_path();

    let (claude_path, git_bash_path) = read_config_paths();

    let checks = vec![
        check_claude_cli(&claude_path),
        #[cfg(target_os = "windows")]
        check_git_bash(&git_bash_path),
    ];

    ChecksResult { checks }
}

fn read_config_paths() -> (Option<String>, Option<String>) {
    match crate::store::get_app_config() {
        Ok(config) => (config.claude_path, config.git_bash_path),
        Err(_) => (None, None),
    }
}

/// 用 where 查找可执行文件（仅 Git Bash 检测使用）
#[cfg(target_os = "windows")]
fn find_all_executables(name: &str) -> Vec<String> {
    crate::platform::find_all_executables(name)
}

/// 检查 Claude CLI
///
/// 与执行层共用 `platform::cli_target`：显式配置必须可运行，否则视为
/// 未配置并走 PATH 检测。检测结果不写回配置（运行时只读）。
fn check_claude_cli(config_path: &Option<String>) -> CheckResult {
    // 1. 用户显式配置且可运行
    if let crate::platform::CliTarget::Explicit(path) =
        crate::platform::cli_target(config_path.as_deref())
    {
        return CheckResult::pass_with_path_source(
            "Claude CLI",
            "checkFoundConfigured",
            &format!("Found (configured): {}", path),
            &path,
            "configured",
        );
    }

    // 2. PATH 检测（仅展示/检查结论，执行时仍由平台自己查找）
    if let Some(path) = crate::platform::probe_cli() {
        return CheckResult::pass_with_path_source(
            "Claude CLI",
            "checkFoundPath",
            &format!("Found: {}", path),
            &path,
            "path",
        );
    }

    // 3. 区分「找不到」与「配置了但不可运行」，给出可操作的修复提示
    if let Some(path) = config_path {
        if Path::new(path).exists() {
            return CheckResult::fail_with_path(
                "Claude CLI",
                "checkClaudeNotRunnable",
                format!(
                    "Claude CLI at '{}' is not runnable. Reinstall Claude CLI or set a valid path below.",
                    path
                ),
                Some(path.clone()),
                "viewInstallationGuide",
                "https://code.claude.com/docs",
            );
        }
    }

    CheckResult::fail_with_path(
        "Claude CLI",
        "checkClaudeNotFound",
        "Claude CLI not found. You can set a custom path below.".to_string(),
        None,
        "viewInstallationGuide",
        "https://code.claude.com/docs",
    )
}

/// 检查 Git Bash（仅 Windows）
///
/// Git Bash 是 Windows 上 Claude CLI 的必需依赖。
#[cfg(target_os = "windows")]
fn check_git_bash(config_path: &Option<String>) -> CheckResult {
    // 1. 配置中保存的路径（优先）
    // 如果配置了路径且存在，直接通过
    if let Some(ref path) = config_path {
        if Path::new(path).exists() {
            return CheckResult::pass_with_path(
                "Git Bash",
                "checkFoundConfigured",
                &format!("Found (config): {}", path),
                path,
            );
        }
        // 配置路径不存在，记录警告，继续尝试其他方式查找
        log::warn!("[Check] Configured Git Bash path not found: {}, trying other methods...", path);
    }

    // 2. 环境变量
    if let Ok(path) = std::env::var("CLAUDE_CODE_GIT_BASH_PATH") {
        if Path::new(&path).exists() {
            return CheckResult::pass_with_path(
                "Git Bash",
                "checkFoundEnv",
                &format!("Found (env): {}", path),
                &path,
            );
        }
    }

    // 3. where git → 在 git 安装目录下找 bash.exe
    if let Some(bash_path) = detect_git_bash_from_git() {
        return CheckResult::pass_with_path(
            "Git Bash",
            "checkFoundAuto",
            &format!("Found: {}", bash_path),
            &bash_path,
        );
    }

    CheckResult::fail_with_path(
        "Git Bash",
        "checkGitBashNotFound",
        "Git Bash not found. You can set a custom path below.".to_string(),
        None,
        "installGitForWindows",
        "https://git-scm.com/download/win",
    )
}

/// 通过 where git 查找 Git 安装目录下的 bash.exe
///
/// 启动检查与 pty 的 spawn 路径共用（git.exe 在 `<install>\cmd`，
/// bash.exe 在 `<install>\bin`，不能只看同级目录）。
#[cfg(target_os = "windows")]
pub(crate) fn detect_git_bash_from_git() -> Option<String> {
    let exe_name = if cfg!(target_os = "windows") {
        "git.exe"
    } else {
        "git"
    };
    let git_paths = find_all_executables(exe_name);

    for git_path in &git_paths {
        // git.exe 通常位于 <git-install>/cmd/git.exe 或 <git-install>/bin/git.exe
        let path = Path::new(git_path);
        if let Some(parent) = path.parent() {
            let git_install_dir = if parent.file_name().map(|n| n == "cmd").unwrap_or(false)
                || parent.file_name().map(|n| n == "bin").unwrap_or(false)
            {
                parent.parent()
            } else {
                Some(parent)
            };

            if let Some(install_dir) = git_install_dir {
                let bash_path = install_dir.join("bin").join("bash.exe");
                if bash_path.exists() {
                    log::info!(
                        "[Check] Git Bash found via 'where git': {}",
                        bash_path.display()
                    );
                    return Some(bash_path.to_string_lossy().to_string());
                }
            }
        }
    }

    None
}

