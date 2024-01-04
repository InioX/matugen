use color_eyre::Report;
extern crate enquote;
use std::process::Command;

#[cfg(target_os = "macos")]
pub fn set(path: &str) -> Result<(), Report> {
    // Generate the Applescript string
    let cmd = &format!(
        r#"tell app "finder" to set desktop picture to POSIX file {}"#,
        enquote::enquote('"', path),
    );
    // Run it using osascript
    Command::new("osascript").args(&["-e", cmd]).output()?;

    Ok(())
}
