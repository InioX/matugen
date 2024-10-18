use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Wallpaper {
    pub set: bool,
    /// Useful for, for example, killing the wallpaper daemon
    pub pre_hook: Option<String>,
    pub command: String,
    /// The last argument will be the image path
    pub arguments: Option<Vec<String>>,
}

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub mod unix;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;
