#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub mod unix;

#[cfg(target_os = "windows")]
pub mod windows;
