//! GUI ↔ Broker 本机 IPC
//!
//! - Windows：Named Pipe（每用户管道名 + 显式 DACL）
//! - Unix：`~/.cc-box/run/broker.sock`（目录 0700 / socket 0600 + peer UID）
//! - 帧格式：4 字节大端长度 + UTF-8 JSON，上限 16 MiB。
//!
//! 连接首帧：
//! - `member.hello`（GwUp v2）→ 注册为成员并保持长连；
//! - `admin`（BrokerAdminRequest）→ 执行一次管理操作后关闭。

use std::path::PathBuf;
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::{mpsc, watch};

use crate::gateway_proto::{
    BrokerAdminRequest, BrokerAdminResponse, GwDown, GwUp, GW_PROTO_VER,
};

const MAX_FRAME: usize = 16 * 1024 * 1024;
const MEMBER_QUEUE_CAPACITY: usize = 4096;

pub trait AsyncStream: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncStream for T {}
pub type BoxStream = Box<dyn AsyncStream>;

pub async fn read_frame<R: AsyncRead + Unpin>(r: &mut R) -> std::io::Result<Option<Vec<u8>>> {
    let mut len = [0u8; 4];
    match r.read_exact(&mut len).await {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    }
    let n = u32::from_be_bytes(len) as usize;
    if n == 0 || n > MAX_FRAME {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid frame length",
        ));
    }
    let mut buf = vec![0u8; n];
    r.read_exact(&mut buf).await?;
    Ok(Some(buf))
}

pub async fn write_frame<W: AsyncWrite + Unpin>(
    w: &mut W,
    data: &[u8],
) -> std::io::Result<()> {
    if data.len() > MAX_FRAME {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "frame too large",
        ));
    }
    w.write_all(&(data.len() as u32).to_be_bytes()).await?;
    w.write_all(data).await?;
    w.flush().await
}

fn config_dir() -> Result<PathBuf, String> {
    crate::store::get_gui_config_dir().map_err(|e| e.to_string())
}

/// 稳定的每用户端点标识（FNV-1a，跨进程/版本不变）
fn user_tag() -> String {
    let dir = config_dir().unwrap_or_else(|_| PathBuf::from("cc-box"));
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in dir.to_string_lossy().to_lowercase().as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}

#[cfg(windows)]
pub(crate) fn pipe_name() -> String {
    format!(r"\\.\pipe\cc-box-broker-{}", user_tag())
}

#[cfg(unix)]
pub(crate) fn socket_path() -> Result<PathBuf, String> {
    let dir = config_dir()?.join("run");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
    }
    Ok(dir.join("broker.sock"))
}

/// 启动 IPC 监听（后台任务）。重复启动会失败（单实例锁已保证唯一）。
pub async fn spawn_server() -> Result<(), String> {
    #[cfg(windows)]
    {
        let name = pipe_name();
        tokio::spawn(async move {
            let mut first = true;
            loop {
                let server = match create_pipe_server(&name, first) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("[broker-ipc] create pipe failed: {}", e);
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        continue;
                    }
                };
                first = false;
                if let Err(e) = server.connect().await {
                    log::debug!("[broker-ipc] pipe connect failed: {}", e);
                    continue;
                }
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(server).await {
                        log::debug!("[broker-ipc] connection ended: {}", e);
                    }
                });
            }
        });
        Ok(())
    }
    #[cfg(unix)]
    {
        use tokio::net::{UnixListener, UnixStream};
        let path = socket_path()?;
        if path.exists() {
            if UnixStream::connect(&path).await.is_ok() {
                return Err("broker IPC already listening".to_string());
            }
            let _ = std::fs::remove_file(&path);
        }
        let listener = UnixListener::bind(&path).map_err(|e| e.to_string())?;
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        }
        let our_uid = std::fs::metadata(&path)
            .ok()
            .map(|m| {
                use std::os::unix::fs::MetadataExt;
                m.uid()
            });
        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        if let (Some(expected), Ok(cred)) = (our_uid, stream.peer_cred()) {
                            if cred.uid() != expected {
                                log::warn!("[broker-ipc] rejected peer with different uid");
                                continue;
                            }
                        }
                        tokio::spawn(async move {
                            if let Err(e) = handle_connection(stream).await {
                                log::debug!("[broker-ipc] connection ended: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        log::warn!("[broker-ipc] accept failed: {}", e);
                        tokio::time::sleep(Duration::from_millis(200)).await;
                    }
                }
            }
        });
        Ok(())
    }
}

/// GUI 侧连接 Broker（Windows 遇 pipe busy 短退避重试）
pub async fn connect_client() -> std::io::Result<BoxStream> {
    #[cfg(windows)]
    {
        use tokio::net::windows::named_pipe::ClientOptions;
        let name = pipe_name();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(3);
        loop {
            match ClientOptions::new().open(&name) {
                Ok(client) => return Ok(Box::new(client)),
                Err(e) => {
                    let busy = e.raw_os_error() == Some(231);
                    if busy && tokio::time::Instant::now() < deadline {
                        tokio::time::sleep(Duration::from_millis(30)).await;
                        continue;
                    }
                    return Err(e);
                }
            }
        }
    }
    #[cfg(unix)]
    {
        let path = socket_path().map_err(std::io::Error::other)?;
        let stream = tokio::net::UnixStream::connect(path).await?;
        Ok(Box::new(stream))
    }
}

async fn handle_connection<S>(stream: S) -> std::io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let (mut rd, mut wr) = tokio::io::split(stream);
    let first = match tokio::time::timeout(Duration::from_secs(10), read_frame(&mut rd)).await {
        Ok(Ok(Some(frame))) => frame,
        _ => return Ok(()),
    };
    let value: serde_json::Value = match serde_json::from_slice(&first) {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };
    let ty = value.get("type").and_then(|v| v.as_str()).unwrap_or_default();

    if ty == "admin" {
        let req: BrokerAdminRequest = match serde_json::from_value(value) {
            Ok(v) => v,
            Err(_) => return Ok(()),
        };
        let BrokerAdminRequest::Admin {
            id,
            secret,
            op,
        } = req;
        let should_exit = matches!(
            op,
            crate::gateway_proto::BrokerOp::Quit
                | crate::gateway_proto::BrokerOp::PrepareForUpdate { .. }
        );
        let resp = if super::ownership::read_broker_secret().as_deref() != Some(secret.as_str()) {
            BrokerAdminResponse::Result {
                id,
                ok: false,
                status: None,
                error: Some("invalid secret".to_string()),
            }
        } else {
            match super::control(op).await {
                Ok(status) => BrokerAdminResponse::Result {
                    id,
                    ok: true,
                    status,
                    error: None,
                },
                Err(e) => BrokerAdminResponse::Result {
                    id,
                    ok: false,
                    status: None,
                    error: Some(e),
                },
            }
        };
        let bytes = serde_json::to_vec(&resp).map_err(std::io::Error::other)?;
        let _ = write_frame(&mut wr, &bytes).await;
        // 响应已写出，再触发主循环退出（否则 --quit-broker/更新调用
        // 可能因进程先退出而误报失败）。
        if should_exit {
            super::request_exit();
        }
        return Ok(());
    }

    let hello: GwUp = match serde_json::from_value(value) {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };
    let (ver, id, secret, label, pid, window_title, sessions) = match hello {
        GwUp::MemberHello {
            ver,
            instance_id,
            secret,
            label,
            pid,
            window_title,
            sessions,
        } => (ver, instance_id, secret, label, pid, window_title, sessions),
        _ => return Ok(()),
    };
    let secret_ok =
        super::ownership::read_broker_secret().as_deref() == Some(secret.as_str());
    if ver != GW_PROTO_VER || !secret_ok {
        let reject = GwDown::Reject {
            reason: "invalid secret or protocol version".to_string(),
        };
        let _ = write_frame(
            &mut wr,
            &serde_json::to_vec(&reject).unwrap_or_default(),
        )
        .await;
        return Ok(());
    }

    let (down_tx, mut down_rx) = mpsc::channel::<GwDown>(MEMBER_QUEUE_CAPACITY);
    let (close_tx, mut close_rx) = watch::channel(false);
    let Some(name) = crate::gateway::register_member(
        id.clone(),
        label,
        ver,
        pid,
        window_title,
        sessions,
        down_tx.clone(),
        close_tx,
    ) else {
        let reject = GwDown::Reject {
            reason: "duplicate instance id".to_string(),
        };
        let _ = write_frame(
            &mut wr,
            &serde_json::to_vec(&reject).unwrap_or_default(),
        )
        .await;
        return Ok(());
    };
    log::info!("[broker-ipc] member registered: {} ({})", name, &id[..8.min(id.len())]);

    let _ = down_tx
        .send(GwDown::RegisterOk {
            instance_name: name.clone(),
            gateway_id: crate::gateway_proto::instance_id().to_string(),
        })
        .await;
    let (enabled, connected, url, pair_code) = crate::remote_client::status_snapshot();
    let _ = down_tx
        .send(GwDown::RelayStatus {
            enabled,
            connected,
            url,
            pair_code,
        })
        .await;
    if let Some(status) = super::status_snapshot() {
        let _ = down_tx.send(GwDown::BrokerStatus { status }).await;
    }
    crate::gateway::notify_sessions_changed();
    // 新实例上线：其余成员的状态镜像也要刷新在线数量
    crate::gateway::broadcast_broker_status();

    let writer = tokio::spawn(async move {
        while let Some(msg) = down_rx.recv().await {
            let Ok(bytes) = serde_json::to_vec(&msg) else {
                continue;
            };
            if write_frame(&mut wr, &bytes).await.is_err() {
                break;
            }
        }
    });

    loop {
        tokio::select! {
            _ = close_rx.changed() => {
                if *close_rx.borrow() { break; }
            }
            frame = read_frame(&mut rd) => match frame {
                Ok(Some(bytes)) => match serde_json::from_slice::<GwUp>(&bytes) {
                    Ok(up) => {
                        if !crate::gateway::handle_member_up(&id, up).await { break; }
                    }
                    Err(e) => log::debug!(
                        "[broker-ipc] member {} invalid frame: {}",
                        &id[..8.min(id.len())],
                        e
                    ),
                },
                Ok(None) => break,
                Err(_) => break,
            },
        }
    }

    writer.abort();
    let sessions = crate::gateway::unregister_member(&id);
    crate::gateway::notify_sessions_changed();
    log::info!(
        "[broker-ipc] member {} left ({} sessions)",
        name,
        sessions.len()
    );
    Ok(())
}

// ==================== Windows Named Pipe + 显式 DACL ====================

#[cfg(windows)]
fn create_pipe_server(
    name: &str,
    first: bool,
) -> std::io::Result<tokio::net::windows::named_pipe::NamedPipeServer> {
    use tokio::net::windows::named_pipe::ServerOptions;
    let mut opts = ServerOptions::new();
    opts.first_pipe_instance(first)
        .reject_remote_clients(true)
        .max_instances(16);
    match win_security_attributes() {
        Some((mut sa, sd)) => {
            let result = unsafe {
                opts.create_with_security_attributes_raw(
                    name,
                    &mut sa as *mut _ as *mut std::ffi::c_void,
                )
            };
            unsafe {
                let _ = windows_sys::Win32::Foundation::LocalFree(sd);
            }
            result
        }
        None => opts.create(name),
    }
}

/// 构造仅当前用户 + SYSTEM/Administrators 可访问的 SECURITY_ATTRIBUTES。
/// 失败时返回 None，调用方回退默认 DACL（仍受每用户管道名 + secret 保护）。
#[cfg(windows)]
fn win_security_attributes() -> Option<(
    windows_sys::Win32::Security::SECURITY_ATTRIBUTES,
    *mut std::ffi::c_void,
)> {
    use std::ffi::c_void;
    use windows_sys::Win32::Foundation::{CloseHandle, LocalFree};
    use windows_sys::Win32::Security::Authorization::{
        ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW,
    };
    use windows_sys::Win32::Security::{
        GetTokenInformation, TokenUser, SECURITY_ATTRIBUTES, TOKEN_QUERY, TOKEN_USER,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return None;
        }
        let mut len = 0u32;
        GetTokenInformation(token, TokenUser, std::ptr::null_mut(), 0, &mut len);
        if len == 0 {
            let _ = CloseHandle(token);
            return None;
        }
        let mut buf = vec![0u8; len as usize];
        let ok = GetTokenInformation(
            token,
            TokenUser,
            buf.as_mut_ptr() as *mut c_void,
            len,
            &mut len,
        );
        let mut sid_str: *mut u16 = std::ptr::null_mut();
        let sid_ok = ok != 0
            && ConvertSidToStringSidW(
                (*(buf.as_ptr() as *const TOKEN_USER)).User.Sid,
                &mut sid_str,
            ) != 0;
        let _ = CloseHandle(token);
        if !sid_ok || sid_str.is_null() {
            return None;
        }
        let sid_len = (0..).take_while(|&i| *sid_str.add(i) != 0).count();
        let sid = String::from_utf16_lossy(std::slice::from_raw_parts(sid_str, sid_len));
        let _ = LocalFree(sid_str as *mut c_void);

        let sddl = format!(
            "D:(A;;GA;;;{})(A;;GA;;;SY)(A;;GA;;;BA)",
            sid
        );
        let wide: Vec<u16> = sddl.encode_utf16().chain(std::iter::once(0)).collect();
        let mut sd: *mut c_void = std::ptr::null_mut();
        if ConvertStringSecurityDescriptorToSecurityDescriptorW(
            wide.as_ptr(),
            1,
            &mut sd,
            std::ptr::null_mut(),
        ) == 0
            || sd.is_null()
        {
            return None;
        }
        let sa = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: sd,
            bInheritHandle: 0,
        };
        Some((sa, sd))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::duplex;

    #[tokio::test]
    async fn Ipc_FrameRoundTrip_1() {
        let (mut a, mut b) = duplex(4096);
        let payload = br#"{"type":"ping"}"#;
        write_frame(&mut a, payload).await.unwrap();
        let got = read_frame(&mut b).await.unwrap().unwrap();
        assert_eq!(got, payload);
    }

    #[tokio::test]
    async fn Ipc_OversizedFrameRejected_2() {
        let (mut a, mut b) = duplex(64);
        let len = (MAX_FRAME as u32 + 1).to_be_bytes();
        a.write_all(&len).await.unwrap();
        let err = read_frame(&mut b).await.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    }

    #[tokio::test]
    async fn Ipc_EmptyFrameRejected_3() {
        let (mut a, mut b) = duplex(64);
        a.write_all(&0u32.to_be_bytes()).await.unwrap();
        let err = read_frame(&mut b).await.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    }
}
