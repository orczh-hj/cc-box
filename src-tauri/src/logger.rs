//! 日志模块
//! 将日志写入 ~/.cc-box/logs/ 目录，按日期分文件
//! - {date}.log: 全部日志（级别由 CC_BOX_LOG / 编译模式决定）
//! - {date}.error.log: 仅 WARN 及以上
//! 跨零点自动滚动到新日期文件；保留最近 7 天日志，自动清理旧文件
//!
//! 级别开关：环境变量 `CC_BOX_LOG`（error/warn/info/debug/trace，大小写不敏感）
//! 优先于编译默认（debug 编译 = Debug，release = Info）——release 排障时
//! 设系统级 `CC_BOX_LOG=debug` 可临时放开，排障后记得关闭（日志量大）。
//!
//! 记录规范：
//! - ERROR = 失败且影响用户；WARN = 降级/重试/可疑；INFO = 用户可感知的状态变迁；DEBUG = 过程流水
//! - 上下文用 `key=value` 前缀（如 `pty=<id> cmd=<name>`），模块信息已由 target 承载，不写方括号前缀
//! - std Error 错误链用 [`error_chain`] 展开；anyhow 带 context 的错误用 `format!("{e:#}")`

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use chrono::Local;

/// 错误链展开的最大深度（防 source() 环导致死循环）
const ERROR_CHAIN_MAX_DEPTH: usize = 8;

/// 环境变量名：运行时日志级别覆盖
const LOG_LEVEL_ENV: &str = "CC_BOX_LOG";

/// 展开错误链：Display 起头，逐层 source() 以 " -> " 连接，深度上限 8 防环。
/// anyhow 带 context 的错误请直接 `format!("{e:#}")`，不要套本函数。
pub(crate) fn error_chain(e: &dyn std::error::Error) -> String {
    let mut parts = vec![e.to_string()];
    let mut cur = e.source();
    while let Some(src) = cur {
        parts.push(src.to_string());
        if parts.len() >= ERROR_CHAIN_MAX_DEPTH {
            break;
        }
        cur = src.source();
    }
    parts.join(" -> ")
}

/// 解析 CC_BOX_LOG 取值（大小写不敏感）；非法值返回 None（沿用编译默认）
pub(crate) fn parse_level(s: &str) -> Option<log::LevelFilter> {
    match s.trim().to_ascii_lowercase().as_str() {
        "error" => Some(log::LevelFilter::Error),
        "warn" => Some(log::LevelFilter::Warn),
        "info" => Some(log::LevelFilter::Info),
        "debug" => Some(log::LevelFilter::Debug),
        "trace" => Some(log::LevelFilter::Trace),
        _ => None,
    }
}

/// 决定最终日志级别：env 值合法则覆盖，否则按编译模式（debug=Debug / release=Info）
pub(crate) fn resolve_level(env_val: Option<&str>, is_debug_build: bool) -> log::LevelFilter {
    if let Some(level) = env_val.and_then(parse_level) {
        return level;
    }
    if is_debug_build {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    }
}

/// 跨零点判断：当前打开文件的前缀与今天的日期不一致时需要滚动
pub(crate) fn should_rollover(current: &str, today: &str) -> bool {
    current != today
}

/// 日志目录
fn log_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".cc-box").join("logs"))
}

/// 确保日志目录存在
fn ensure_log_dir() -> Option<PathBuf> {
    let dir = log_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir).ok()?;
    }
    Some(dir)
}

/// 当天日期前缀
fn date_prefix() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

/// 打开日志文件（追加）
fn open_log_file(name: &str) -> Option<File> {
    let dir = ensure_log_dir()?;
    let path = dir.join(name);
    OpenOptions::new().create(true).append(true).open(path).ok()
}

/// 格式化日志行
pub(crate) fn format_line(record: &log::Record) -> String {
    let ts = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    format!(
        "[{}][{}] [{}] {}\n",
        ts,
        record.level(),
        record.target(),
        record.args()
    )
}

/// 清理超过 keep_days 天的旧日志
fn cleanup_old_logs(keep_days: u64) {
    let dir = match log_dir() {
        Some(d) if d.exists() => d,
        _ => return,
    };

    let cutoff =
        std::time::SystemTime::now() - std::time::Duration::from_secs(keep_days * 24 * 60 * 60);

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "log").unwrap_or(false) {
                if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        if modified < cutoff {
                            let _ = fs::remove_file(&path);
                        }
                    }
                }
            }
        }
    }
}

pub(crate) struct FileLogger {
    app_log: Mutex<Option<File>>,
    error_log: Mutex<Option<File>>,
    /// 当前打开文件对应的日期前缀，用于跨零点滚动
    current_date: Mutex<String>,
}

pub(crate) static LOGGER: FileLogger = FileLogger {
    app_log: Mutex::new(None),
    error_log: Mutex::new(None),
    current_date: Mutex::new(String::new()),
};

/// 运行时落盘级别（init 时由 resolve_level 结果写入）。
/// `log::Log::enabled` 必须以此为准——宏侧只查 max_level 不查 enabled，
/// 硬编码 `<= Info` 会把 CC_BOX_LOG=debug 放开的 Debug/Trace 全部丢弃
/// （log crate 中 Level 与 LevelFilter 的判别值对齐：Error=1..Trace=5，可直接比较）
pub(crate) static RUNTIME_MAX_LEVEL: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(log::LevelFilter::Info as usize);

impl FileLogger {
    /// 跨零点滚动：在锁内重开两个文件；重开失败保留旧句柄（不丢日志），
    /// 每条日志都会重试（目录持续不可写时 debug 构建下会在 stderr 刷屏——
    /// 预期行为：不可写本身就该被看见）
    fn rollover_if_needed(&self, today: &str) {
        let mut date_guard = match self.current_date.lock() {
            Ok(g) => g,
            Err(_) => return,
        };
        if !should_rollover(&date_guard, today) {
            return;
        }
        let app_file = open_log_file(&format!("{}.log", today));
        let err_file = open_log_file(&format!("{}.error.log", today));
        if app_file.is_none() || err_file.is_none() {
            // 保留旧句柄继续写，下一条日志会再尝试（open 每次重试成本可接受）
            eprintln!("[Logger] Warning: log rollover to {} failed, keeping old handles", today);
            return;
        }
        if let Ok(mut guard) = self.app_log.lock() {
            *guard = app_file;
        }
        if let Ok(mut guard) = self.error_log.lock() {
            *guard = err_file;
        }
        *date_guard = today.to_string();
    }
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        (metadata.level() as usize) <= RUNTIME_MAX_LEVEL.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // 跨零点滚动（date_prefix 开销为一次本地时间格式化，可接受）
        self.rollover_if_needed(&date_prefix());

        let line = format_line(record);

        // 写入全部日志文件
        if let Ok(mut guard) = self.app_log.lock() {
            if let Some(ref mut f) = *guard {
                let _ = f.write_all(line.as_bytes());
                let _ = f.flush();
            }
        }

        // WARN 及以上写入错误日志文件
        if record.level() <= log::Level::Warn {
            if let Ok(mut guard) = self.error_log.lock() {
                if let Some(ref mut f) = *guard {
                    let _ = f.write_all(line.as_bytes());
                    let _ = f.flush();
                }
            }
        }

        // debug 模式同时输出到 stderr
        #[cfg(debug_assertions)]
        {
            eprint!("{}", line);
        }
    }

    fn flush(&self) {
        if let Ok(mut guard) = self.app_log.lock() {
            if let Some(ref mut f) = *guard {
                let _ = f.flush();
            }
        }
        if let Ok(mut guard) = self.error_log.lock() {
            if let Some(ref mut f) = *guard {
                let _ = f.flush();
            }
        }
    }
}

/// 初始化日志系统，在 Tauri setup 阶段调用
pub fn init() {
    let prefix = date_prefix();

    // 打开日志文件
    let app_file = open_log_file(&format!("{}.log", prefix));
    let err_file = open_log_file(&format!("{}.error.log", prefix));

    if let Ok(mut guard) = LOGGER.app_log.lock() {
        *guard = app_file;
    }
    if let Ok(mut guard) = LOGGER.error_log.lock() {
        *guard = err_file;
    }
    if let Ok(mut guard) = LOGGER.current_date.lock() {
        *guard = prefix;
    }

    // CC_BOX_LOG 环境变量覆盖编译默认（debug=Debug / release=Info）
    let level = resolve_level(std::env::var(LOG_LEVEL_ENV).ok().as_deref(), cfg!(debug_assertions));
    log::set_max_level(level);
    // enabled() 与宏侧 max_level 必须同步（enabled 是落盘的真正闸门，见 RUNTIME_MAX_LEVEL）
    RUNTIME_MAX_LEVEL.store(level as usize, std::sync::atomic::Ordering::Relaxed);

    // 设置全局日志器
    if log::set_logger(&LOGGER).is_err() {
        // 已有其他 logger（理论上不会发生）
        eprintln!("[Logger] Warning: another logger already set");
    }

    log::info!("=== CC-Box started ===");
    if let Some(dir) = log_dir() {
        log::info!("Log directory: {}", dir.display());
    }

    // 后台清理旧日志
    std::thread::spawn(|| {
        cleanup_old_logs(7);
    });
}
