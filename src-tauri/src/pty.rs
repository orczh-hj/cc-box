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

/// 启动期输出缓存：spawn 确认进程存活前，reader 线程把输出暂存于此
/// （旁路收集，不影响正常 emit）。进程秒退时取尾部拼进错误信息——
/// claude 自身的报错文本（usage/崩溃信息）不再随「前端终端尚未建立」丢失
pub(crate) struct StartupBuf {
    /// false = spawn 已收尾（存活或失败），不再收集
    pub(crate) active: bool,
    pub(crate) data: Vec<u8>,
}

/// 缓存上限：足够容纳 CLI 启动报错，又不至于在极端刷屏时占内存
pub(crate) const STARTUP_BUF_MAX: usize = 4096;

impl StartupBuf {
    pub(crate) fn append(&mut self, bytes: &[u8]) {
        if !self.active {
            return;
        }
        self.data.extend_from_slice(bytes);
        if self.data.len() > STARTUP_BUF_MAX {
            let overflow = self.data.len() - STARTUP_BUF_MAX;
            self.data.drain(..overflow);
        }
    }
}

/// 去除 ANSI 转义序列（秒退错误信息进日志/终端红字前清理，避免脏字符）
pub(crate) fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\x1b' {
            out.push(c);
            continue;
        }
        match chars.next() {
            // CSI：ESC [ <参数字节 0x30–0x3F> <中间字节 0x20–0x2F> <终结字节 0x40–0x7E>
            Some('[') => {
                for c2 in chars.by_ref() {
                    if ('@'..='~').contains(&c2) {
                        break;
                    }
                }
            }
            // OSC：ESC ] ... BEL 或 ESC \（ST）
            Some(']') => {
                for c2 in chars.by_ref() {
                    if c2 == '\x07' {
                        break;
                    }
                    if c2 == '\x1b' {
                        let _ = chars.next();
                        break;
                    }
                }
            }
            // 单字符转义（ESC c 等）：跳过
            Some(_) => {}
            None => break,
        }
    }
    out
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

        // 获取 Claude CLI 路径（配置优先，其次自动检测）——检测结果即执行
        // 路径，不再让 shell 用裸命令名从 PATH 二次查找（GUI 应用继承的
        // PATH 常缺 Homebrew/~/.local/bin 前缀，检测命中 ≠ shell 能找到）
        let claude_path = match Self::get_claude_path() {
            Some(p) => p,
            None => {
                let err_msg = "Claude CLI not found. Please install Claude CLI first (npm install -g @anthropic-ai/claude-code)";
                log::error!("{}", err_msg);
                self.emit_error(&id, err_msg, "detection");
                return Err(anyhow!("{}", err_msg));
            }
        };

        // --plugin-dir（hook 监控 plugin），目录存在才追加
        let plugin_dir = {
            let dir = crate::hook_config::plugin_dir();
            if dir.exists() { Some(dir) } else { None }
        };

        let mut cmd = if cfg!(target_os = "windows") {
            // Windows：npm 安装的 claude 是 .cmd/batch 包装，必须经 shell 中转
            // （Git Bash，未检出回退 PowerShell）；引号包裹完整路径防空格分词
            let mut claude_cmd = format!("\"{}\"", claude_path);
            if let Some(extra_args) = &args {
                claude_cmd.push(' ');
                claude_cmd.push_str(&extra_args.join(" "));
            }
            if let Some(dir) = &plugin_dir {
                claude_cmd.push_str(&format!(" --plugin-dir \"{}\"", dir.display()));
            }
            let git_bash = Self::detect_git_bash();
            let (program, shell_args) =
                crate::platform::get_claude_shell(&claude_cmd, git_bash.as_deref());
            log::info!("Using shell: {} with args: {:?}", program, shell_args);
            let mut cmd = CommandBuilder::new(&program);
            for arg in &shell_args {
                cmd.arg(arg);
            }
            cmd
        } else {
            // Unix（macOS/Linux）：直接 exec 完整路径，不经 shell 中转——
            // 1) `bash -i -c` 交互模式的 job control 在 PTY 下可能把 TUI 子
            //    进程停入后台（SIGTTIN/SIGTTOU），表现为终端白屏（Mac 打开
            //    历史会话白屏根因）；2) 免 PATH 二次查找与 rc 文件差异；
            //    3) 结构化参数免引号拼接与分词歧义
            let mut cmd = CommandBuilder::new(&claude_path);
            if let Some(extra_args) = &args {
                for arg in extra_args {
                    cmd.arg(arg);
                }
            }
            if let Some(dir) = &plugin_dir {
                cmd.arg("--plugin-dir");
                cmd.arg(dir);
            }
            log::info!("Executing claude directly: {}", claude_path);
            cmd
        };

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

        log::debug!("Spawning claude: path={}, args={:?}", claude_path, args);

        // 启动进程
        let mut child = match pair.slave.spawn_command(cmd) {
            Ok(c) => c,
            Err(e) => {
                let err_msg = format!("Failed to spawn claude '{}': {}", claude_path, e);
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

        // 启动输出读取线程（startup_buf 旁路收集启动期输出，见 startup_check）
        let pty_id = id.clone();
        let app_handle = self.app_handle.clone();
        let cwd_for_log = cwd.to_string();
        let startup_buf = Arc::new(Mutex::new(StartupBuf {
            active: true,
            data: Vec::new(),
        }));
        let buf_for_thread = startup_buf.clone();

        thread::Builder::new()
            .name(format!("pty-reader-{}", &pty_id[..8.min(pty_id.len())]))
            .spawn(move || {
                log::debug!("[{}] Output reader thread started", pty_id);
                Self::read_output_loop(pty_id, reader, app_handle, &cwd_for_log, buf_for_thread);
            })
            .with_context(|| format!("Failed to spawn reader thread for PTY {}", id))?;

        // 给进程启动一点时间
        thread::sleep(Duration::from_millis(50));

        // 立即退出检测：秒退（EOF 先于本函数返回，事件落在前端消费端
        // 建立之前）转成 spawn 失败并携带 PTY 尾部输出
        self.startup_check(&id, &mut child, &startup_buf, &claude_path)
            .map_err(|e| anyhow!("{}", e))?;

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
        startup_buf: Arc<Mutex<StartupBuf>>,
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
                        // 旁路收集启动期输出（秒退时的死因证据，见 StartupBuf）
                        startup_buf.lock().append(output.as_bytes());
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

    /// 启动期收尾（spawn_claude / spawn_shell 共用）：关闭输出缓存并做
    /// 立即退出检测。进程在启动等待期内退出时，前端终端实例尚未建立，
    /// pty-exit 与输出事件全部丢失（表现为纯白屏，且本模块还报 spawned
    /// successfully——Mac native claude 秒退白屏的教训）——转成 spawn
    /// 失败，错误信息携带退出状态与 PTY 尾部输出（进程自己的报错文本）
    fn startup_check(
        &self,
        id: &str,
        child: &mut Box<dyn Child + Send + Sync>,
        startup_buf: &Mutex<StartupBuf>,
        cmd_hint: &str,
    ) -> std::result::Result<(), String> {
        let tail = {
            let mut buf = startup_buf.lock();
            buf.active = false;
            let start = buf.data.len().saturating_sub(2048);
            String::from_utf8_lossy(&buf.data[start..]).into_owned()
        };
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut err_msg =
                    format!("{} exited immediately during startup ({})", cmd_hint, status);
                let tail = strip_ansi(tail.trim());
                if !tail.is_empty() {
                    err_msg.push_str("\n--- output before exit ---\n");
                    err_msg.push_str(&tail);
                }
                err_msg.push_str(&format!(
                    "\nTry running it in a system terminal first: \"{}\"",
                    cmd_hint
                ));
                log::error!("[{}] {}", id, err_msg);
                self.emit_error(id, &err_msg, "startup");
                Err(err_msg)
            }
            Ok(None) => Ok(()),
            Err(e) => {
                // try_wait 自身失败不阻断启动（保守放行，保持旧行为）
                log::warn!("[{}] try_wait during startup failed: {}", id, e);
                Ok(())
            }
        }
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
        let mut child = match pair.slave.spawn_command(cmd) {
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

        // 启动输出读取线程（startup_buf 旁路收集启动期输出，见 startup_check）
        let pty_id = id.clone();
        let app_handle = self.app_handle.clone();
        let cwd_for_log = cwd.to_string();
        let startup_buf = Arc::new(Mutex::new(StartupBuf {
            active: true,
            data: Vec::new(),
        }));
        let buf_for_thread = startup_buf.clone();

        thread::Builder::new()
            .name(format!("pty-reader-{}", &pty_id[..8.min(pty_id.len())]))
            .spawn(move || {
                log::debug!("[{}] Shell output reader thread started", pty_id);
                Self::read_output_loop(pty_id, reader, app_handle, &cwd_for_log, buf_for_thread);
            })
            .with_context(|| format!("Failed to spawn reader thread for PTY {}", id))?;

        // 给进程启动一点时间
        thread::sleep(Duration::from_millis(50));

        // 立即退出检测（与 spawn_claude 同一语义，秒退可见而非白屏）
        self.startup_check(&id, &mut child, &startup_buf, program)
            .map_err(|e| anyhow!("{}", e))?;

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
            // 幂等语义：kill 的目标状态就是"进程不存在"，重复 kill（双端
            // 关闭竞态：一端 kill 后另一端收到 closed 事件再 kill 同一 id）
            // 不是异常，目标已达成——debug 即可
            log::debug!("[{}] PTY already gone when trying to kill (idempotent)", id);
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

