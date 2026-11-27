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
            // 单字符转义（ESC c 等）：跳过 ESC 后的这一个字符
            Some(c2 @ ('(' | ')')) => {
                // SCS 字符集选择（ESC ( B / ESC ) 0 等）：序列共三字节，
                // 再消费一个字符——否则设计字符 'B' 会残留为孤立字符
                // （实测 uninstall 输出末尾孤立「B」的根因）
                let _ = c2;
                let _ = chars.next();
            }
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
        log::debug!("PTY Manager initialized");
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

        // 3. 安装器默认位置（与启动检查同一实现：解压成功但未写配置时仍可启动）
        #[cfg(target_os = "windows")]
        if let Some(bash_path) = crate::checks::detect_portable_git_default() {
            return Some(bash_path);
        }

        // 4. where git → 推导安装目录下的 bash.exe（与启动检查同一实现；
        //    git.exe 常在 <install>\cmd，bash.exe 在 <install>\bin，不能只看同目录）
        #[cfg(target_os = "windows")]
        if let Some(bash_path) = crate::checks::detect_git_bash_from_git() {
            log::info!("Git Bash found via 'where git': {}", bash_path);
            return Some(bash_path);
        }

        log::warn!("Git Bash not found - Claude CLI may not work properly on Windows");
        None
    }

    /// 解析本次执行的 Claude CLI 目标（与启动检查共用 platform::cli_target）。
    ///
    /// 配置路径必须是可运行文件；失效时返回 PathLookup，由执行层按平台
    /// 查找（Unix 用 find_executable 过滤后 exec，Windows 交给 shell）——
    /// 坏文件不会被选中（Mac 事故：无执行位的旧二进制被直接 exec 报 EACCES）。
    fn claude_target() -> crate::platform::CliTarget {
        let config_path = crate::store::get_app_config()
            .ok()
            .and_then(|config| config.claude_path);
        crate::platform::cli_target(config_path.as_deref())
    }

    /// 启动 Claude CLI（命令解析与终端一致）
    ///
    /// Unix 直接 exec：显式配置用完整路径，否则先按 PATH 解析出可运行文件；
    /// Windows 经 Git Bash/PowerShell 执行（`.cmd` 包装必须由 shell 解析）。
    /// 两条路径都保留“跳过不可执行文件”的终端语义。
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
            return Err(self.spawn_fail(
                &id,
                "validation",
                format!("Working directory does not exist: {}", cwd),
            ));
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

        // 本次执行目标：显式配置（校验通过）或 PATH 查找。
        // 解析复用 platform 的共享实现，执行层不写配置。
        let target = Self::claude_target();
        let mut claude_display = match &target {
            crate::platform::CliTarget::Explicit(path) => path.clone(),
            crate::platform::CliTarget::PathLookup => "claude".to_string(),
        };

        // --plugin-dir（hook 监控 plugin），目录存在才追加
        let plugin_dir = {
            let dir = crate::hook_config::plugin_dir();
            if dir.exists() { Some(dir) } else { None }
        };

        let mut cmd = if cfg!(target_os = "windows") {
            // Windows：npm 安装的 claude 是 .cmd/batch 包装，必须经 shell 中转
            // （Git Bash，未检出回退 PowerShell）。显式配置用完整路径（引号
            // 防空格分词）；PathLookup 用裸命令名，让 shell 自己做 PATHEXT
            // 查找，等价于用户在该终端输入 `claude`。
            let mut claude_cmd = match &target {
                crate::platform::CliTarget::Explicit(path) => format!("\"{}\"", path),
                crate::platform::CliTarget::PathLookup => "claude".to_string(),
            };
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
            log::debug!("Using shell: {} with args: {:?}", program, shell_args);
            let mut cmd = CommandBuilder::new(&program);
            for arg in &shell_args {
                cmd.arg(arg);
            }
            cmd
        } else {
            // Unix（macOS/Linux）：不经 shell 中转——`bash -i -c` 交互模式的
            // job control 在 PTY 下可能把 TUI 子进程停入后台（SIGTTIN/SIGTTOU），
            // 表现为终端白屏（Mac 打开历史会话白屏根因）。
            //
            // PathLookup 必须先用共享解析器按 PATH 顺序取出可运行文件：
            // portable-pty 的 search_path 会优先取 cwd 下同名项且只判 exists()，
            // PATH 候选用 access(X_OK)（目录也能通过）——与终端语义不一致，
            // 可能重蹈「坏路径被 exec → EACCES」的白屏故障。
            let program = match &target {
                crate::platform::CliTarget::Explicit(path) => path.clone(),
                crate::platform::CliTarget::PathLookup => {
                    match crate::platform::find_executable("claude") {
                        Some(path) => {
                            claude_display = path.clone();
                            path
                        }
                        None => {
                            return Err(self.spawn_fail(
                                &id,
                                "detection",
                                "Claude CLI not found in PATH. Install it (npm install -g @anthropic-ai/claude-code) or set a custom path".to_string(),
                            ));
                        }
                    }
                }
            };
            let mut cmd = CommandBuilder::new(&program);
            if let Some(extra_args) = &args {
                for arg in extra_args {
                    cmd.arg(arg);
                }
            }
            if let Some(dir) = &plugin_dir {
                cmd.arg("--plugin-dir");
                cmd.arg(dir);
            }
            log::debug!("Executing claude directly: {}", program);
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

        log::debug!("Spawning claude: target={}, args={:?}", claude_display, args);

        // 启动进程
        let mut child = match pair.slave.spawn_command(cmd) {
            Ok(c) => c,
            Err(e) => {
                let hint = match &target {
                    crate::platform::CliTarget::PathLookup => {
                        " (not found in PATH; install Claude CLI or set a custom path)"
                    }
                    crate::platform::CliTarget::Explicit(_) => "",
                };
                let err_msg = format!("Failed to spawn claude '{}'{}: {}", claude_display, hint, e);
                return Err(self.spawn_fail(&id, "spawn", err_msg));
            }
        };

        // 获取 writer 和 reader
        let writer = match pair.master.take_writer() {
            Ok(w) => w,
            Err(e) => return Err(self.spawn_fail(&id, "writer", format!("Failed to take PTY writer: {}", e))),
        };

        let reader = match pair.master.try_clone_reader() {
            Ok(r) => r,
            Err(e) => return Err(self.spawn_fail(&id, "reader", format!("Failed to clone PTY reader: {}", e))),
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
                log::debug!("pty={} output reader thread started", pty_id);
                Self::read_output_loop(pty_id, reader, app_handle, &cwd_for_log, buf_for_thread);
            })
            .with_context(|| format!("Failed to spawn reader thread for PTY {}", id))?;

        // 给进程启动一点时间
        thread::sleep(Duration::from_millis(50));

        // 立即退出检测：秒退（EOF 先于本函数返回，事件落在前端消费端
        // 建立之前）转成 spawn 失败并携带 PTY 尾部输出
        self.startup_check(&id, &mut child, &startup_buf, &claude_display)
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

        log::info!("pty={} Claude CLI spawned successfully via shell", id);

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
                    log::info!("pty={} EOF reached, process exited normally", pty_id);
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
                        "pty={} read error ({}/{}): {}",
                        pty_id,
                        consecutive_errors,
                        MAX_CONSECUTIVE_ERRORS,
                        e
                    );

                    if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                        log::error!(
                            "pty={} too many consecutive read errors, stopping reader",
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

    /// spawn 失败单点：log 一条 ERROR + emit pty-error + 返回 Err（三步一处收敛）。
    /// stage 是前端展示契约（validation/detection/spawn/writer/reader/startup），勿改。
    fn spawn_fail(&self, id: &str, stage: &str, msg: String) -> anyhow::Error {
        log::error!("pty={} stage={} {}", id, stage, msg);
        self.emit_error(id, &msg, stage);
        anyhow!("{}", msg)
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
                log::error!("pty={} {}", id, err_msg);
                self.emit_error(id, &err_msg, "startup");
                Err(err_msg)
            }
            Ok(None) => Ok(()),
            Err(e) => {
                // try_wait 自身失败不阻断启动（保守放行，保持旧行为）
                log::warn!("pty={} try_wait during startup failed: {}", id, e);
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
            return Err(self.spawn_fail(
                &id,
                "validation",
                format!("Working directory does not exist: {}", cwd),
            ));
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
                return Err(self.spawn_fail(&id, "spawn", format!("Failed to spawn shell '{}': {}", program, e)));
            }
        };

        // 获取 writer 和 reader
        let writer = match pair.master.take_writer() {
            Ok(w) => w,
            Err(e) => return Err(self.spawn_fail(&id, "writer", format!("Failed to take PTY writer: {}", e))),
        };

        let reader = match pair.master.try_clone_reader() {
            Ok(r) => r,
            Err(e) => return Err(self.spawn_fail(&id, "reader", format!("Failed to clone PTY reader: {}", e))),
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
                log::debug!("pty={} shell output reader thread started", pty_id);
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

        log::info!("pty={} Shell spawned successfully", id);

        Ok(PtyInfo {
            id,
            pty_type: "shell".to_string(),
            cwd: cwd.to_string(),
        })
    }

    /// 写入输入到 PTY
    pub fn write(&self, id: &str, data: &str) -> Result<()> {
        log::trace!("pty={} writing {} bytes to PTY", id, data.len());

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

        log::trace!("pty={} write successful", id);
        Ok(())
    }

    /// 调整 PTY 大小
    pub fn resize(&self, id: &str, cols: u16, rows: u16) -> Result<()> {
        log::debug!("pty={} resizing to {}x{}", id, cols, rows);

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

        log::debug!("pty={} resize successful", id);
        Ok(())
    }

    /// 杀掉 PTY 进程
    pub fn kill(&self, id: &str) -> Result<()> {
        log::info!("pty={} killing PTY", id);

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

            log::info!("pty={} PTY killed successfully", id);
        } else {
            // 幂等语义：kill 的目标状态就是"进程不存在"，重复 kill（双端
            // 关闭竞态：一端 kill 后另一端收到 closed 事件再 kill 同一 id）
            // 不是异常，目标已达成——debug 即可
            log::debug!("pty={} PTY already gone when trying to kill (idempotent)", id);
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
                log::debug!("pty={} child killed", id);
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
    log::debug!("PTY manager initialized");
}

/// 获取 PTY 管理器
pub fn get_pty_manager() -> Option<Arc<PtyManager>> {
    PTY_MANAGER.lock().clone()
}

// ==================== 一次性 PTY 命令（插件市场 CLI 流式输出） ====================
//
// 市场命令（install/uninstall/update/marketplace add…）耗时可达 120s 且唯一
// 实时信息源是 CLI 的 TTY 渲染（spinner/状态行）——管道下 Node stdout 块缓冲，
// 输出到进程退出才一帧到达（实测）。本执行器把 CLI 跑在隐藏 PTY 里，输出经
// 字符矩阵渲染（PtyScreen）还原为可读文本，再经回调上抛。
//
// 为什么必须渲染矩阵而不是线性清洗：CLI 的 TUI 层（Ink 类）在全屏模式下用
// 「光标绝对定位（ESC[r;cH）+ 光标右移代替空格（ESC[nC）」重绘，实测连词间
// 空格都是 ESC[1C 画的——任何线性 strip/merge 都还原不了，只有带光标状态的
// 字符矩阵能正确呈现（CI=1 试图让 CLI 退化为静态输出，实测无效）。

/// 输出事件：Line = 追加一行；Screen = 整屏重置（TUI 重绘帧，前端清空重建）
pub(crate) enum PtyOut {
    Line(String),
    Screen(String),
}

/// mini 终端屏幕：80×24 字符矩阵 + 光标，渲染覆盖 CLI 实测用到的序列子集
/// （CUP 定位 / CUF·CUB·CUU·CUD 移动 / ED·EL 清除 / \r \n / 可见字符 / OSC /
/// 私有序列忽略）。 unwritten 格子即空格——ESC[nC「画空格」由提取时自然成立。
/// feed 是跨 chunk 持久的状态机：转义序列拆在两次 feed 之间也能正确续解析
/// （实测 chunk 边界切在 CSI 中间的情况真实存在）。
pub(crate) struct PtyScreen {
    cols: usize,
    rows: usize,
    grid: Vec<char>,
    row: usize,
    col: usize,
    /// 解析状态（跨 feed 持久）
    st: ParseSt,
}

enum ParseSt {
    Plain,
    /// 已见 ESC
    Esc,
    /// 已见 ESC [，累积参数直到终结字节
    Csi(String),
    /// OSC（ESC ]）直到 BEL 或 ESC \
    Osc,
    /// OSC 中已见 ESC（等待 \）
    OscEsc,
    /// SCS（ESC ( / ESC )）：再消费一个字符
    Scs,
}

impl Default for PtyScreen {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

impl PtyScreen {
    pub(crate) fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            grid: vec![' '; cols * rows],
            row: 0,
            col: 0,
            st: ParseSt::Plain,
        }
    }

    fn clear_all(&mut self) {
        self.grid.fill(' ');
    }

    fn clear_row_tail(&mut self) {
        // EL：从光标清到本行行尾
        for i in (self.row * self.cols + self.col)..((self.row + 1) * self.cols) {
            self.grid[i] = ' ';
        }
    }

    fn put(&mut self, c: char) {
        if self.col >= self.cols {
            self.col = 0;
            self.newline();
        }
        self.grid[self.row * self.cols + self.col] = c;
        self.col += 1;
    }

    fn newline(&mut self) {
        self.row += 1;
        if self.row >= self.rows {
            // 底部滚动：丢首行，底部开新行
            self.grid.drain(..self.cols);
            self.grid.resize(self.cols * self.rows, ' ');
            self.row = self.rows - 1;
        }
    }

    /// 渲染一段 PTY 文本（已解码，可能含 ANSI 序列）
    pub(crate) fn feed(&mut self, s: &str) {
        for c in s.chars() {
            match std::mem::replace(&mut self.st, ParseSt::Plain) {
                ParseSt::Plain => match c {
                    '\u{1b}' => self.st = ParseSt::Esc,
                    '\r' => self.col = 0,
                    '\n' => self.newline(),
                    '\u{8}' => self.col = self.col.saturating_sub(1), // BS
                    '\t' => {
                        let next = (self.col / 8 + 1) * 8;
                        self.col = next.min(self.cols - 1);
                    }
                    c if (c as u32) < 32 => {} // 其余控制字符忽略
                    c => self.put(c),
                },
                ParseSt::Esc => match c {
                    '[' => self.st = ParseSt::Csi(String::new()),
                    ']' => self.st = ParseSt::Osc,
                    '(' | ')' => self.st = ParseSt::Scs,
                    _ => {} // 其它单字符转义（ESC c 等）忽略，回 Plain
                },
                ParseSt::Csi(mut params) => {
                    if ('@'..='~').contains(&c) {
                        self.apply_csi(&params, c);
                    } else {
                        params.push(c);
                        self.st = ParseSt::Csi(params);
                    }
                }
                ParseSt::Osc => match c {
                    '\u{7}' => {} // BEL 终结，回 Plain（replace 已发生）
                    '\u{1b}' => self.st = ParseSt::OscEsc,
                    _ => self.st = ParseSt::Osc,
                },
                ParseSt::OscEsc => {
                    // ESC \（ST）终结；其它字符回 Plain（容错）
                }
                ParseSt::Scs => {} // 设计符已消费，回 Plain
            }
        }
    }

    fn apply_csi(&mut self, params: &str, final_byte: char) {
        // 跳过私有标记（?25l/h 等）
        let nums: Vec<usize> = params
            .trim_start_matches(['?', '>', '<', '='])
            .split(';')
            .map(|p| p.parse::<usize>().unwrap_or(1))
            .collect();
        match final_byte {
            'H' | 'f' => {
                let r = nums.first().copied().unwrap_or(1).max(1);
                let c = nums.get(1).copied().unwrap_or(1).max(1);
                self.row = (r - 1).min(self.rows - 1);
                self.col = (c - 1).min(self.cols - 1);
            }
            'A' => self.row = self.row.saturating_sub(nums.first().copied().unwrap_or(1)),
            'B' => {
                self.row = self
                    .row
                    .saturating_add(nums.first().copied().unwrap_or(1))
                    .min(self.rows - 1)
            }
            'C' => {
                self.col = self
                    .col
                    .saturating_add(nums.first().copied().unwrap_or(1))
                    .min(self.cols - 1)
            }
            'D' => self.col = self.col.saturating_sub(nums.first().copied().unwrap_or(1)),
            'G' => self.col = (nums.first().copied().unwrap_or(1).max(1) - 1).min(self.cols - 1),
            'd' => self.row = (nums.first().copied().unwrap_or(1).max(1) - 1).min(self.rows - 1),
            'J' => {
                // ED：2（或空）= 全屏；0/1（局部清除）按全屏近似——TUI 清屏帧
                let mode = params.trim_start_matches('?');
                if mode.is_empty() || mode == "0" || mode == "2" {
                    self.clear_all();
                }
            }
            'K' => self.clear_row_tail(),
            _ => {} // m/l/h/s/u 等忽略
        }
    }

    /// 当前屏幕文本（行尾空格去除；全空行跳过）
    pub(crate) fn dump_lines(&self) -> Vec<String> {
        (0..self.rows)
            .map(|r| {
                self.grid[r * self.cols..(r + 1) * self.cols]
                    .iter()
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .filter(|l| !l.is_empty())
            .collect()
    }
}

/// bash / PowerShell 共用的单引号包裹：单引号串内无任何展开（$、反引号等
/// 字面处理），内部单引号以 `''` 转义。两平台语义一致，注入面收敛到一处。
fn shell_quote(arg: &str) -> String {
    format!("'{}'", arg.replace('\'', "''"))
}

/// 取文本尾部 N 行（错误 context 用）
fn tail_lines(text: &str, n: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(n);
    lines[start..].join("\n")
}

/// 一次性 PTY 执行 claude CLI。
///
/// 输出渲染进字符矩阵（PtyScreen）后按帧 diff 经 `on_out` 上抛（调用方线程）：
/// 新增行 → `PtyOut::Line`；整屏重绘 → `PtyOut::Screen`（全文重置）。
/// 返回值 full 为最终屏幕帧文本。spawn 失败/超时返回 Err；非零退出返回
/// `Ok((false, full))`（由调用方按现有语义判失败）。
/// 不注册 instances/writers、不发 pty-* 事件——与交互终端完全隔离。
pub(crate) fn run_claude_pty_command(
    args: &[&str],
    cwd: Option<&Path>,
    timeout: Duration,
    mut on_line: impl FnMut(PtyOut),
) -> anyhow::Result<(bool, String)> {
    // 目标解析与 spawn_claude 同源（platform::cli_target 单点）：显式配置用
    // 完整路径；PathLookup Windows 用裸名让 shell 做 PATHEXT 查找，Unix 解析
    // 成可运行文件（与 spawn_claude 同语义）
    let configured = crate::store::get_app_config().ok().and_then(|c| c.claude_path);
    let target = crate::platform::cli_target(configured.as_deref());

    let pair = native_pty_system()
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .context("openpty failed")?;

    #[cfg(target_os = "windows")]
    let claude_cmd = match &target {
        crate::platform::CliTarget::Explicit(path) => format!("\"{}\"", path),
        crate::platform::CliTarget::PathLookup => "claude".to_string(),
    };

    let mut cmd = {
        #[cfg(target_os = "windows")]
        {
            let git_bash = crate::store::detect_git_bash_path();
            // bash 与 PowerShell 回退统一用单引号包裹（shell_quote，内部 '
            // 以 '' 转义）——无插值、无注入面。PowerShell 侧 & 调用操作符
            // 必须存在，否则裸字符串字面量后跟参数会被解析为「意外标记」
            // （实测 ParserError）。
            let mut invocation = claude_cmd.clone();
            for a in args {
                invocation.push(' ');
                invocation.push_str(&shell_quote(a));
            }
            let invocation = match &git_bash {
                Some(_) => invocation,
                None => format!("& {}", invocation),
            };
            let (program, shell_args) =
                crate::platform::get_claude_shell(&invocation, git_bash.as_deref());
            let mut c = CommandBuilder::new(&program);
            for a in &shell_args {
                c.arg(a);
            }
            if let Some(gb) = &git_bash {
                c.env("CLAUDE_CODE_GIT_BASH_PATH", gb);
            }
            c
        }
        #[cfg(not(target_os = "windows"))]
        {
            let program = match &target {
                crate::platform::CliTarget::Explicit(path) => path.clone(),
                crate::platform::CliTarget::PathLookup => crate::platform::find_executable("claude")
                    .unwrap_or_else(|| "claude".to_string()),
            };
            let mut c = CommandBuilder::new(&program);
            for a in args {
                c.arg(a);
            }
            c
        }
    };

    for (k, v) in env::vars() {
        cmd.env(k, v);
    }
    cmd.env("TERM", "xterm-256color");
    cmd.env("COLORTERM", "truecolor");
    // CI=1 本意是让 Ink 类 TUI 退化为静态输出；实测 claude CLI 无视它仍全屏
    // 渲染（光标右移画空格 + 绝对定位重绘）——这正是需要 PtyScreen 矩阵渲染
    // 的原因。保留无害，删掉亦无行为差异。
    cmd.env("CI", "1");
    if let Ok(config) = crate::store::get_app_config() {
        if let Some(vars) = config.claude_env_vars {
            for (k, v) in &vars {
                cmd.env(k, v);
            }
        }
    }
    if let Some(dir) = cwd {
        cmd.cwd(dir);
    }

    let mut child = pair.slave.spawn_command(cmd).context("PTY spawn failed")?;
    let mut reader = pair
        .master
        .try_clone_reader()
        .context("clone reader failed")?;
    // master 必须保活到子进程退出：Windows conpty 下关闭 master 句柄会立即
    // 终止伪控制台（进程秒退、输出全丢——实测 uninstall「exit non-zero 空输出」
    // 根因）。slave 不 drop 会向 PTY 回传输入，但一次性命令无需写 stdin；
    // 保留 slave 不读也无害，pair 整体留到函数末尾自然 drop 最简单。
    let _pair_alive = pair;

    // 读线程：只把解码后的原始文本累积进共享缓冲（on_line 回调持有方在
    // 主线程，规避 FnMut 跨线程 Send 约束）
    let raw: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let done = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let raw_for_reader = raw.clone();
    let done_for_reader = done.clone();
    thread::Builder::new()
        .name("pty-once-reader".to_string())
        .spawn(move || {
            let mut buf = [0u8; 4096];
            let mut decoder = crate::pty_decoder::PtyDecoder::new();
            loop {
                use std::io::Read;
                match reader.read(&mut buf) {
                    Ok(0) => {
                        let tail = decoder.flush();
                        if !tail.is_empty() {
                            raw_for_reader.lock().push_str(&tail);
                        }
                        break;
                    }
                    Ok(n) => {
                        let decoded = decoder.decode(&buf[..n]);
                        if !decoded.is_empty() {
                            raw_for_reader.lock().push_str(&decoded);
                        }
                    }
                    Err(_) => break,
                }
            }
            done_for_reader.store(true, std::sync::atomic::Ordering::SeqCst);
        })
        .context("spawn reader thread failed")?;

    // 主循环：轮询共享缓冲 → 渲染进字符矩阵 → 屏幕帧 diff 回调；同时等待子进程
    let debug_raw = std::env::var("CCBOX_DEBUG_PTY_RAW").is_ok();
    let mut screen = PtyScreen::default();
    let mut prev_lines: Vec<String> = Vec::new();
    let mut consumed = 0usize;
    let mut full = String::new();
    let start = std::time::Instant::now();

    // 渲染并 diff：新帧以上一帧为前缀 → 只发新增行；否则发整屏重置帧
    // （TUI 每帧整屏重绘/绝对定位改写，线性行 diff 只能表达这两种形态）
    let mut render_diff = |screen: &mut PtyScreen,
                           text: &str,
                           prev: &mut Vec<String>,
                           full: &mut String,
                           on_line: &mut dyn FnMut(PtyOut)| {
        screen.feed(text);
        let lines = screen.dump_lines();
        let is_prefix = lines.len() >= prev.len()
            && lines[..prev.len()] == prev[..]
            && !(prev.is_empty() && lines.is_empty());
        if is_prefix {
            for line in &lines[prev.len()..] {
                on_line(PtyOut::Line(line.clone()));
            }
        } else {
            on_line(PtyOut::Screen(lines.join("\n")));
        }
        // full 恒为最新屏幕帧（错误文案/超时「最近输出」取自它——必须是
        // 最终帧而非首帧，否则 install 失败时 toast 里只有开头的 spinner 行）
        if !lines.is_empty() {
            *full = lines.join("\n");
            full.push('\n');
        }
        *prev = lines;
    };

    let status = loop {
        {
            let mut r = raw.lock();
            if r.len() > consumed {
                let new_text = r[consumed..].to_string();
                consumed = r.len();
                drop(r);
                if debug_raw {
                    eprintln!("[pty-once RAW] {:?}", new_text);
                }
                render_diff(&mut screen, &new_text, &mut prev_lines, &mut full, &mut on_line);
            }
        }
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                if start.elapsed() >= timeout {
                    if let Some(pid) = child.process_id() {
                        crate::store::kill_process_tree(pid);
                    }
                    let _ = child.wait();
                    let tail = tail_lines(&full, 10);
                    anyhow::bail!(
                        "claude {} timed out after {}s\n--- 最近输出 ---\n{}",
                        args.join(" "),
                        timeout.as_secs(),
                        tail
                    );
                }
                thread::sleep(Duration::from_millis(50));
            }
            Err(e) => anyhow::bail!("wait failed: {}", e),
        }
    };

    // EOF 收尾：等读线程结束（通常已随子进程退出到达），渲染残留后终帧 diff
    for _ in 0..20 {
        if done.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    {
        let r = raw.lock();
        if r.len() > consumed {
            let new_text = r[consumed..].to_string();
            if debug_raw {
                eprintln!("[pty-once RAW] {:?}", new_text);
            }
            render_diff(&mut screen, &new_text, &mut prev_lines, &mut full, &mut on_line);
        }
    }

    Ok((status.success(), full))
}

#[cfg(test)]
mod once_command_tests {
    use super::{PtyOut, PtyScreen, shell_quote};

    // bash / PowerShell 单引号包裹：内部单引号以 '' 转义（POSIX 与 PowerShell
    // 同语义），包裹后 $、反引号等一律字面处理——注入面收敛点
    #[test]
    fn ShellQuote_单引号转义_001() {
        assert_eq!(shell_quote("abc"), "'abc'");
        assert_eq!(shell_quote("it's"), "'it''s'");
        assert_eq!(shell_quote(""), "''");
        // 含 shell 元字符的来源被完整字面包裹
        assert_eq!(shell_quote("a$(rm -rf)b`c`d e"), "'a$(rm -rf)b`c`d e'");
    }

    // 手动实测（需本机 claude CLI）：cargo test PtyOnce_live_plugin_list -- --ignored --nocapture
    // CCBOX_DEBUG_PTY_RAW=1 时额外打印原始 PTY 字节（排查 spinner 序列残留）
    #[test]
    #[ignore]
    fn PtyOnce_live_plugin_list() {
        // 任意命令可用环境变量注入：CCBOX_PTY_TEST_ARGS="plugin,list"
        let args: Vec<String> = std::env::var("CCBOX_PTY_TEST_ARGS")
            .unwrap_or_else(|_| "plugin,list".into())
            .split(',')
            .map(str::to_string)
            .collect();
        let argrefs: Vec<&str> = args.iter().map(String::as_str).collect();
        let (ok, full) = super::run_claude_pty_command(
            &argrefs,
            None,
            std::time::Duration::from_secs(120),
            |out| match out {
                PtyOut::Line(l) => println!("LINE: {l:?}"),
                PtyOut::Screen(s) => println!("SCREEN-RESET:\n{s}"),
            },
        )
        .expect("pty run failed");
        println!("success={ok}\n---FULL---\n{}", full);
    }

    // ESC[nC（光标右移）在提取时表现为空格——Ink TUI 用它画词间空格（实测根因）
    #[test]
    fn PtyScreen_CUF画空格_001() {
        let mut s = PtyScreen::default();
        s.feed("Installed\u{1b}[1Cplugins:\u{1b}[1C2 items");
        assert_eq!(s.dump_lines(), vec!["Installed plugins: 2 items"]);
    }

    // CUP 绝对定位写两处，dump 按行还原（跳过中间全空行）
    #[test]
    fn PtyScreen_CUP定位还原_002() {
        let mut s = PtyScreen::default();
        s.feed("\u{1b}[3;3H❯\u{1b}[1Cplugin-a\u{1b}[6;5HVersion:\u{1b}[1C1.2.3");
        assert_eq!(s.dump_lines(), vec!["  ❯ plugin-a", "    Version: 1.2.3"]);
    }

    // 清屏后重写：旧内容消失，只剩新帧（spinner 重绘的矩阵语义）
    #[test]
    fn PtyScreen_清屏重绘_003() {
        let mut s = PtyScreen::default();
        s.feed("\u{1b}[2J\u{1b}[H⠙ Installing\u{1b}[2J\u{1b}[H✔ Done");
        assert_eq!(s.dump_lines(), vec!["✔ Done"]);
    }

    // 底部滚动：超出屏高的行顶出首行；结尾 \r\n 在底部同样滚动（真实终端语义）
    #[test]
    fn PtyScreen_底部滚动_004() {
        let mut s = PtyScreen::new(40, 3);
        s.feed("row1\r\nrow2\r\nrow3\r\nrow4\r\n");
        assert_eq!(s.dump_lines(), vec!["row3", "row4"]);
    }

    // OSC 标题与 SCS 字符集序列不残留字符（ESC]0;...BEL / ESC(B）
    #[test]
    fn PtyScreen_OSC与SCS不残留_005() {
        let mut s = PtyScreen::default();
        s.feed("\u{1b}]0;window title\u{7}\u{1b}(Bplain");
        assert_eq!(s.dump_lines(), vec!["plain"]);
    }

    // 跨 chunk 半序列：CSI 拆在两个 feed 里仍正确解析
    #[test]
    fn PtyScreen_跨chunk序列拆分_006() {
        let mut s = PtyScreen::default();
        s.feed("A\u{1b}[1");
        s.feed("CB");
        assert_eq!(s.dump_lines(), vec!["A B"]);
    }
}

