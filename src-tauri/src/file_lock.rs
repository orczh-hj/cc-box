//! 跨平台 OS 排他文件锁（进程崩溃自动释放）
//!
//! 用于：
//! - `broker.lock`：本机唯一 Broker 实例；
//! - `config.lock`：Broker/GUI 跨进程 read-merge-write 配置。

use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub struct FileLock {
    file: File,
}

impl FileLock {
    /// 非阻塞尝试排他锁；已被占用返回 `WouldBlock`
    pub fn try_exclusive(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)?;
        lock_file(&file)?;
        Ok(Self { file })
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        unlock_file(&self.file);
    }
}

#[cfg(windows)]
fn lock_file(file: &File) -> io::Result<()> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        LockFileEx, LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY,
    };
    use windows_sys::Win32::System::IO::OVERLAPPED;
    let mut ov: OVERLAPPED = unsafe { std::mem::zeroed() };
    let ok = unsafe {
        LockFileEx(
            file.as_raw_handle() as _,
            LOCKFILE_EXCLUSIVE_LOCK | LOCKFILE_FAIL_IMMEDIATELY,
            0,
            u32::MAX,
            u32::MAX,
            &mut ov,
        )
    };
    if ok == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(windows)]
fn unlock_file(file: &File) {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::UnlockFileEx;
    use windows_sys::Win32::System::IO::OVERLAPPED;
    let mut ov: OVERLAPPED = unsafe { std::mem::zeroed() };
    unsafe {
        let _ = UnlockFileEx(
            file.as_raw_handle() as _,
            0,
            u32::MAX,
            u32::MAX,
            &mut ov,
        );
    }
}

#[cfg(unix)]
fn lock_file(file: &File) -> io::Result<()> {
    use std::os::unix::io::AsRawFd;
    let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if rc != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(unix)]
fn unlock_file(file: &File) {
    use std::os::unix::io::AsRawFd;
    unsafe {
        let _ = libc::flock(file.as_raw_fd(), libc::LOCK_UN);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn FileLock_SecondAcquireFails_1() {
        let dir = std::env::temp_dir().join(format!(
            "cc-box-lock-test-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test.lock");
        let first = FileLock::try_exclusive(&path).expect("first lock");
        assert!(FileLock::try_exclusive(&path).is_err(), "second must fail");
        drop(first);
        assert!(FileLock::try_exclusive(&path).is_ok(), "released lock reusable");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
