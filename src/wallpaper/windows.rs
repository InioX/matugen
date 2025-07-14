use color_eyre::Report;

#[cfg(target_os = "windows")]
use std::ffi::OsStr;
use std::{io, iter, os::windows::ffi::OsStrExt};
use winapi::{
    ctypes::c_void,
    um::winuser::{
        SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
    },
};

#[cfg(target_os = "windows")]
pub fn set(path: &String) -> Result<(), Report> {
    unsafe {
        let path = OsStr::new(path)
            .encode_wide()
            // append null byte
            .chain(iter::once(0))
            .collect::<Vec<u16>>();
        let successful = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) == 1;

        if successful {
            Ok(())
        } else {
            Err(io::Error::last_os_error().into())
        }
    }
}
