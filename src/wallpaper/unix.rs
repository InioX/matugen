use crate::wallpaper::Wallpaper;
use color_eyre::Report;
use execute::Execute;
use std::process::{Command, Stdio};

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub fn set(
    path: &String,
    Wallpaper {
        pre_hook,
        command,
        arguments,
        ..
    }: &Wallpaper,
) -> Result<(), Report> {
    info!("Setting wallpaper...");

    if let Some(hook) = pre_hook {
        spawn_hook(hook.to_string())?
    }

    let mut binding = Command::new(&command);
    let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());

    if let Some(args) = arguments {
        cmd.args(args);
    }
    cmd.arg(path);

    match cmd.spawn() {
        Ok(_) => info!("Successfully set the wallpaper with <blue>{command}</>"),
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                error!(
                    "Failed to set wallpaper, the program <red>{command}</> was not found in PATH!"
                )
            } else {
                error!("Some error(s) occurred while setting wallpaper!");
            }
        }
    };
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
fn spawn_hook(hook: String) -> Result<(), Report> {
    let mut command = execute::shell(&hook);

    command.stdout(Stdio::inherit());

    let output = command.execute_output()?;

    if let Some(exit_code) = output.status.code() {
        if exit_code != 0 {
            error!("Failed executing command: {:?}", hook)
        }
    } else {
        eprintln!("Interrupted!");
    }

    Ok(())
}
