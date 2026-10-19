//! PTY 管理模块
//! 基于 portable-pty 实现 Claude CLI 进程管理

use anyhow::{anyhow, Context, Result};
use parking_lot::Mutex;
use portable_pty::{native_pty_system, Child, CommandBuilder, PtyPair, PtySize};
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::sync::LazyLock;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

/// PTY 实例信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct PtyInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub pty_type: String,
    pub cwd: String,
}

/// PTY 输出事件 payload
#[derive(Debug, Clone, serde::Serialize)]
pub struct PtyOutputPayload {
    pub id: String,
    pub data: String,
}

/// PTY 退出事件 payload
#[derive(Debug, Clone, serde::Serialize)]
pub struct PtyExitPayload {
    pub id: String,
    pub exit_code: i32,
    pub signal: Option<String>,
}

/// PTY 错误事件 payload
#[derive(Debug, Clone, serde::Serialize)]
pub struct PtyErrorPayload {
    pub id: String,
    pub error: String,
    pub stage: String,
}

/// PTY 进程状态
#[derive(Debug, Clone, PartialEq)]
pub enum PtyStatus {
    Running,
    Stopping,
}

/// PTY 实例内部数据
struct PtyInstanceData {
    pair: PtyPair,
    child: Box<dyn Child + Send>,
    status: PtyStatus,
}

/// PTY 管理器（全局）
pub struct PtyManager {
    /// PTY 实例（包含 pair 和元数据）
    instances: Mutex<HashMap<String, PtyInstanceData>>,
    /// Writer 句柄（需要单独存储因为实现了 Send）
    writers: Mutex<HashMap<String, Box<dyn Write + Send>>>,
    /// Tauri AppHandle 用于发送事件
    app_handle: AppHandle,
}

impl PtyManager {
    /// 创建 PTY 管理器
    pub fn new(app_handle: AppHandle) -> Self {
        log::info!("PTY Manager initialized");
        Self {
            instances: Mutex::new(HashMap::new()),
            writers: Mutex::new(HashMap::new()),
            app_handle,
        }
    }

    /// 检测 Git Bash 路径（Windows）
    fn detect_git_bash() -> Option<String> {
        if !cfg!(target_os = "windows") {
            return None;
        }

        log::debug!("Detecting Git Bash on Windows...");

        // 1. 配置文件
        if let Ok(config) = crate::store::get_app_config() {
            if let Some(ref path) = config.git_bash_path {
                if Path::new(path).exists() {
                    log::info!("Git Bash path from config: {}", path);
                    return Some(path.clone());
                }
            }
        }

        // 2. 环境变量
        if let Ok(path) = env::var("CLAUDE_CODE_GIT_BASH_PATH") {
            if Path::new(&path).exists() {
                log::info!("Git Bash found via env var: {}", path);
                return Some(path);
            }
        }

        // 3. where git → 同目录下找 bash.exe
        if let Some(git_path) = crate::platform::find_executable("git") {
            if let Some(parent) = Path::new(&git_path).parent() {
                let bash_path = parent.join("bash.exe");
                if bash_path.exists() {
                    log::info!("Git Bash found via 'where git': {}", bash_path.display());
                    return Some(bash_path.to_string_lossy().to_string());
                }
            }
        }

        log::warn!("Git Bash not found - Claude CLI may not work properly on Windows");
        None
    }

    /// 获取 Claude CLI 路径（优先使用配置，其次自动检测）
    fn get_claude_path() -> Option<String> {
        // 1. 配置文件中的自定义路径
        if let Ok(config) = crate::store::get_app_config() {
            if let Some(ref path) = config.claude_path {
                if Path::new(path).exists() {
                    log::info!("Claude CLI path from config: {}", path);
                    return Some(path.clone());
                }
            }
        }

        // 2. 自动检测（where/which）
        crate::platform::find_executable("claude")
    }

    /// 启动 Claude CLI（通过 shell 执行，模拟终端行为）
    ///
    /// 核心思路：PTY 启动 shell，然后在 shell 里运行 `claude` 命令。
    /// 这与用户在终端里直接输入 `claude` 的行为完全一致，
    /// 不管 claude 是 npm 安装还是原生可执行文件，都能正常工作。
    pub fn spawn_claude(
        &self,
        cwd: &str,
        cols: u16,
        rows: u16,
        args: Option<Vec<String>>,
    ) -> Result<PtyInfo> {
        let id = Uuid::new_v4().to_string();
        log::info!(
            "Spawning Claude CLI via shell with id={}, cwd={}, size={}x{}",
            id,
            cwd,
            cols,
            rows
        );

        // 验证工作目录
        if !Path::new(cwd).exists() {
            let err_msg = format!("Working directory does not exist: {}", cwd);
            log::error!("{}", err_msg);
            self.emit_error(&id, &err_msg, "validation");
            return Err(anyhow!("{}", err_msg));
        }

        // 创建 PTY
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .with_context(|| format!("Failed to open PTY with size {}x{}", cols, rows))?;

        // 获取 Claude CLI 路径（配置优先，其次自动检测）
        match Self::get_claude_path() {
            Some(_) => {} // 仅用于验证 Claude CLI 是否可用
            None => {
                let err_msg = "Claude CLI not found. Please install Claude CLI first (npm install -g @anthropic-ai/claude-code)";
                log::error!("{}", err_msg);
                self.emit_error(&id, err_msg, "detection");
                return Err(anyhow!("{}", err_msg));
            }
        };

        // 构建启动命令
        // 如果是自定义路径（不在标准 PATH 中），使用完整路径
        // 如果是自动检测的路径（在 PATH 中），直接使用 "claude" 命令名
        let claude_cmd = if let Ok(config) = crate::store::get_app_config() {
            if let Some(ref custom_path) = config.claude_path {
                // 用户配置了自定义路径，使用完整路径确保正确
                let base = if let Some(extra_args) = &args {
                    format!("\"{}\" {}", custom_path, extra_args.join(" "))
                } else {
                    format!("\"{}\"", custom_path)
                };
                log::info!("Using custom Claude path: {}", custom_path);
                base
            } else {
                // 自动检测的路径，使用命令名（shell 自动从 PATH 查找）
                if let Some(extra_args) = &args {
                    format!("claude {}", extra_args.join(" "))
                } else {
                    "claude".to_string()
                }
            }
        } else {
            if let Some(extra_args) = &args {
                format!("claude {}", extra_args.join(" "))
            } else {
                "claude".to_string()
            }
        };

        // 追加 --plugin-dir（hook 监控 plugin）
        let plugin_dir = crate::hook_config::plugin_dir();
        let claude_cmd = if plugin_dir.exists() {
            format!("{} --plugin-dir \"{}\"", claude_cmd, plugin_dir.display())
        } else {
            claude_cmd
        };

        // 构建命令：不同平台使用不同的 shell
        let git_bash = if cfg!(target_os = "windows") {
            Self::detect_git_bash()
        } else {
            None
        };
        let (program, args) = crate::platform::get_claude_shell(&claude_cmd, git_bash.as_deref());

        log::info!("Using shell: {} with args: {:?}", program, args);

        let mut cmd = CommandBuilder::new(&program);
        for arg in &args {
            cmd.arg(arg);
        }

        cmd.cwd(cwd);

        // 关键：继承所有父进程环境变量（确保 PATH 正确）
        for (key, value) in env::vars() {
            cmd.env(key, value);
        }

        // 添加终端环境变量
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");

        // Hook 监控：传递 HTTP 服务器端口 + 当前 PTY 标识
        if let Some(hook_port) = crate::hook_server::get_port() {
            cmd.env("CC_BOX_HOOK_PORT", hook_port.to_string());
            cmd.env("CC_BOX_SESSION_ID", &id);
            log::debug!("Set CC_BOX_HOOK_PORT={}, CC_BOX_SESSION_ID={}", hook_port, id);
        }

        // Windows 特定：Claude CLI 需要知道 Git Bash 的位置
        // 即使通过 bash 运行，非交互模式下 Claude 可能无法自动检测
        if cfg!(target_os = "windows") {
            if let Some(git_bash) = Self::detect_git_bash() {
                cmd.env("CLAUDE_CODE_GIT_BASH_PATH", &git_bash);
                log::debug!("Set CLAUDE_CODE_GIT_BASH_PATH={}", git_bash);
            }
        }

        // 注入 cc-box 管理的环境变量（从 ~/.cc-box/config.json 读取）
        if let Ok(config) = crate::store::get_app_config() {
            if let Some(env_vars) = config.claude_env_vars {
                for (key, value) in &env_vars {
                    cmd.env(key, value);
                }
            }
        }

        log::debug!("Shell command: {:?}", claude_cmd);

        // 启动进程
        let child = match pair.slave.spawn_command(cmd) {
            Ok(c) => c,
            Err(e) => {
                let err_msg = format!("Failed to spawn shell command '{}': {}", claude_cmd, e);
                log::error!("{}", err_msg);
                self.emit_error(&id, &err_msg, "spawn");
                return Err(anyhow!("{}", err_msg));
            }
        };

        // 获取 writer 和 reader
        let writer = match pair.master.take_writer() {
            Ok(w) => w,
            Err(e) => {
                let err_msg = format!("Failed to take PTY writer: {}", e);
                log::error!("{}", err_msg);
                self.emit_error(&id, &err_msg, "writer");
                return Err(anyhow!("{}", err_msg));
            }
        };

        let reader = match pair.master.try_clone_reader() {
            Ok(r) => r,
            Err(e) => {
                let err_msg = format!("Failed to clone PTY reader: {}", e);
                log::error!("{}", err_msg);
                self.emit_error(&id, &err_msg, "reader");
                return Err(anyhow!("{}", err_msg));
            }
        };

        // 启动输出读取线程
        let pty_id = id.clone();
        let app_handle = self.app_handle.clone();
        let cwd_for_log = cwd.to_string();

        thread::Builder::new()
            .name(format!("pty-reader-{}", &pty_id[..8.min(pty_id.len())]))
            .spawn(move || {
                log::debug!("[{}] Output reader thread started", pty_id);
                Self::read_output_loop(pty_id, reader, app_handle, &cwd_for_log);
            })
            .with_context(|| format!("Failed to spawn reader thread for PTY {}", id))?;

        // 给进程启动一点时间
        thread::sleep(Duration::from_millis(50));

        // 存储 PTY 实例（包含 child）
        let instance = PtyInstanceData {
            pair,
            child,
            status: PtyStatus::Running,
        };

        self.instances.lock().insert(id.clone(), instance);
        self.writers.lock().insert(id.clone(), writer);

        crate::remote_state::REMOTE_STATE.register_pty(&id, "claude", cwd);

        log::info!("[{}] Claude CLI spawned successfully via shell", id);

        Ok(PtyInfo {
            id,
            pty_type: "claude".to_string(),
            cwd: cwd.to_string(),
        })
    }

    /// 输出读取循环（静态方法，在线程中运行）
    fn read_output_loop(
        pty_id: String,
        mut reader: Box<dyn std::io::Read + Send>,
        app_handle: AppHandle,
        _cwd: &str,
    ) {
        let mut buf = [0u8; 4096];
        let mut decoder = crate::pty_decoder::PtyDecoder::new();
        let mut consecutive_errors = 0;
        const MAX_CONSECUTIVE_ERRORS: i32 = 5;

        loop {
            use std::io::Read;
            match reader.read(&mut buf) {
                Ok(0) => {
                    // EOF：刷出残留字节后退出
                    let output = decoder.flush();
                    if !output.is_empty() {
                        let _ = app_handle.emit(
                            "pty-output",
                            PtyOutputPayload {
                                id: pty_id.clone(),
                                data: output.clone(),
                            },
                        );
                        crate::remote_state::REMOTE_STATE.push_output(&pty_id, &output);
                    }
                    log::info!("[{}] PTY EOF reached, process exited normally", pty_id);
                    app_handle
                        .emit(
                            "pty-exit",
                            PtyExitPayload {
                                id: pty_id.clone(),
                                exit_code: 0,
                                signal: None,
                            },
                        )
                        .ok();
                    crate::remote_state::REMOTE_STATE.remove_pty(&pty_id);
                    break;
                }
                Ok(n) => {
                    consecutive_errors = 0;
                    let output = decoder.decode(&buf[..n]);
                    if !output.is_empty() {
                        let _ = app_handle.emit(
                            "pty-output",
                            PtyOutputPayload {
                                id: pty_id.clone(),
                                data: output.clone(),
                            },
                        );
                        crate::remote_state::REMOTE_STATE.push_output(&pty_id, &output);
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        // 非阻塞模式下的暂时性错误，继续等待
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    }

                    consecutive_errors += 1;
                    log::warn!(
                        "[{}] Read error ({}/{}): {}",
                        pty_id,
                        consecutive_errors,
                        MAX_CONSECUTIVE_ERRORS,
                        e
                    );

                    if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                        log::error!(
                            "[{}] Too many consecutive read errors, stopping reader",
                            pty_id
                        );
                        app_handle
                            .emit(
                                "pty-exit",
                                PtyExitPayload {
                                    id: pty_id.clone(),
                                    exit_code: -1,
                                    signal: Some(format!("read_error: {}", e)),
                                },
                            )
                            .ok();
                        crate::remote_state::REMOTE_STATE.remove_pty(&pty_id);
                        break;
                    }

                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }

    /// 发送错误事件
    fn emit_error(&self, id: &str, error: &str, stage: &str) {
        self.app_handle
            .emit(
                "pty-error",
                PtyErrorPayload {
                    id: id.to_string(),
                    error: error.to_string(),
                    stage: stage.to_string(),
                },
            )
            .ok();
    }

    /// 启动普通 Shell
    pub fn spawn_shell(&self, cwd: &str, cols: u16, rows: u16) -> Result<PtyInfo> {
        let id = Uuid::new_v4().to_string();
        log::info!(
            "Spawning shell with id={}, cwd={}, size={}x{}",
            id,
            cwd,
            cols,
            rows
        );

        // 验证工作目录
        if !Path::new(cwd).exists() {
            let err_msg = format!("Working directory does not exist: {}", cwd);
            log::error!("{}", err_msg);
            self.emit_error(&id, &err_msg, "validation");
            return Err(anyhow!("{}", err_msg));
        }

        // 创建 PTY
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .with_context(|| format!("Failed to open PTY with size {}x{}", cols, rows))?;

        // 确定使用的 shell
        let (program, args) = crate::platform::get_default_shell();

        log::debug!("Using shell: {} with args: {:?}", program, args);

        // 构建命令
        let mut cmd = CommandBuilder::new(program);
        for arg in &args {
            cmd.arg(*arg);
        }
        cmd.cwd(cwd);

        // 继承父进程环境变量
        for (key, value) in env::vars() {
            cmd.env(key, value);
        }

        // 添加终端环境变量
        cmd.env("TERM", "xterm-256color");

        // 注入 cc-box 管理的环境变量（从 ~/.cc-box/config.json 读取）
        if let Ok(config) = crate::store::get_app_config() {
            if let Some(env_vars) = config.claude_env_vars {
                for (key, value) in &env_vars {
                    cmd.env(key, value);
                }
            }
        }

        // 启动进程 - 存储 child 句柄
        let child = match pair.slave.spawn_command(cmd) {
            Ok(c) => c,
            Err(e) => {
                let err_msg = format!("Failed to spawn shell '{}': {}", program, e);
                log::error!("{}", err_msg);
                self.emit_error(&id, &err_msg, "spawn");
                return Err(anyhow!("{}", err_msg));
            }
        };

        // 获取 writer 和 reader
        let writer = match pair.master.take_writer() {
            Ok(w) => w,
            Err(e) => {
                let err_msg = format!("Failed to take PTY writer: {}", e);
                log::error!("{}", err_msg);
                self.emit_error(&id, &err_msg, "writer");
                return Err(anyhow!("{}", err_msg));
            }
        };

        let reader = match pair.master.try_clone_reader() {
            Ok(r) => r,
            Err(e) => {
                let err_msg = format!("Failed to clone PTY reader: {}", e);
                log::error!("{}", err_msg);
                self.emit_error(&id, &err_msg, "reader");
                return Err(anyhow!("{}", err_msg));
            }
        };

        // 启动输出读取线程
        let pty_id = id.clone();
        let app_handle = self.app_handle.clone();
        let cwd_for_log = cwd.to_string();

        thread::Builder::new()
            .name(format!("pty-reader-{}", &pty_id[..8.min(pty_id.len())]))
            .spawn(move || {
                log::debug!("[{}] Shell output reader thread started", pty_id);
                Self::read_output_loop(pty_id, reader, app_handle, &cwd_for_log);
            })
            .with_context(|| format!("Failed to spawn reader thread for PTY {}", id))?;

        // 给进程启动一点时间
        thread::sleep(Duration::from_millis(50));

        // 存储 PTY 实例（包含 child）
        let instance = PtyInstanceData {
            pair,
            child,
            status: PtyStatus::Running,
        };

        self.instances.lock().insert(id.clone(), instance);
        self.writers.lock().insert(id.clone(), writer);

        crate::remote_state::REMOTE_STATE.register_pty(&id, "shell", cwd);

        log::info!("[{}] Shell spawned successfully", id);

        Ok(PtyInfo {
            id,
            pty_type: "shell".to_string(),
            cwd: cwd.to_string(),
        })
    }

    /// 写入输入到 PTY
    pub fn write(&self, id: &str, data: &str) -> Result<()> {
        log::trace!("[{}] Writing {} bytes to PTY", id, data.len());

        let mut writers = self.writers.lock();
        let writer = writers.get_mut(id).ok_or_else(|| {
            let err = format!("PTY writer not found for id: {}", id);
            log::warn!("{}", err);
            anyhow!("{}", err)
        })?;

        writer
            .write_all(data.as_bytes())
            .with_context(|| format!("Failed to write {} bytes to PTY {}", data.len(), id))?;

        writer
            .flush()
            .with_context(|| format!("Failed to flush PTY {}", id))?;

        log::trace!("[{}] Write successful", id);
        Ok(())
    }

    /// 调整 PTY 大小
    pub fn resize(&self, id: &str, cols: u16, rows: u16) -> Result<()> {
        log::debug!("[{}] Resizing PTY to {}x{}", id, cols, rows);

        let instances = self.instances.lock();
        let instance = instances.get(id).ok_or_else(|| {
            let err = format!("PTY instance not found for id: {}", id);
            log::warn!("{}", err);
            anyhow!("{}", err)
        })?;

        instance
            .pair
            .master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .with_context(|| format!("Failed to resize PTY {} to {}x{}", id, cols, rows))?;

        log::debug!("[{}] Resize successful", id);
        Ok(())
    }

    /// 杀掉 PTY 进程
    pub fn kill(&self, id: &str) -> Result<()> {
        log::info!("[{}] Killing PTY", id);

        // 从 instances 中取出（而非只是移除引用）
        let instance = self.instances.lock().remove(id);

        if let Some(mut instance) = instance {
            // 更新状态
            instance.status = PtyStatus::Stopping;

            // 强制终止进程
            instance
                .child
                .kill()
                .with_context(|| format!("Failed to kill child process {}", id))?;

            // 等待进程退出（最多等待 5 秒）
            let _ = instance.child.wait();

            // 移除 writer
            self.writers.lock().remove(id);

            // 发送退出事件
            self.app_handle
                .emit(
                    "pty-exit",
                    PtyExitPayload {
                        id: id.to_string(),
                        exit_code: 137, // SIGKILL
                        signal: Some("killed".to_string()),
                    },
                )
                .ok();

            crate::remote_state::REMOTE_STATE.remove_pty(id);

            log::info!("[{}] PTY killed successfully", id);
        } else {
            log::warn!("[{}] PTY not found when trying to kill", id);
        }

        Ok(())
    }

    /// 杀掉所有 PTY 进程
    pub fn kill_all(&self) {
        let mut instances = self.instances.lock();
        let mut writers = self.writers.lock();

        let count = instances.len();
        log::info!("Killing all {} PTY instances", count);

        // 终止所有进程
        for (id, mut instance) in instances.drain() {
            instance.status = PtyStatus::Stopping;

            // 强制终止
            if instance.child.kill().is_ok() {
                log::info!("[{}] Child killed", id);
            }

            // 等待退出
            let _ = instance.child.wait();
        }

        writers.clear();

        log::info!("Killed {} PTY instances", count);
    }
}

/// 全局 PTY 管理器存储
static PTY_MANAGER: LazyLock<Mutex<Option<Arc<PtyManager>>>> = LazyLock::new(|| Mutex::new(None));

/// Claude CLI 启动参数拼接（统一管理：会话创建单一核心）
///
/// 规则：--resume <id> → --dangerously-skip-permissions → customArgs 按空白拆分。
/// 前端 buildClaudeArgs（XTermTerminal.vue）语义与此一致（远程 session.create
/// 的结构化选项经此函数拼接，两端不漂移）。
pub fn build_claude_args(
    resume: Option<&str>,
    skip_permissions: bool,
    custom_args: &str,
) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    if let Some(sid) = resume {
        if !sid.is_empty() {
            args.push("--resume".to_string());
            args.push(sid.to_string());
        }
    }
    if skip_permissions {
        args.push("--dangerously-skip-permissions".to_string());
    }
    args.extend(custom_args.split_whitespace().map(str::to_string));
    args
}

/// 初始化 PTY 管理器
pub fn init_pty_manager(app_handle: AppHandle) {
    let manager = Arc::new(PtyManager::new(app_handle));
    *PTY_MANAGER.lock() = Some(manager);
    log::info!("PTY manager initialized");
}

/// 获取 PTY 管理器
pub fn get_pty_manager() -> Option<Arc<PtyManager>> {
    PTY_MANAGER.lock().clone()
}

