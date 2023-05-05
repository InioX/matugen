use std::process::Command;
use super::arguments::Cli;
use color_eyre::{eyre::Result, Report};

#[cfg(target_os = "linux")]
pub fn reload_apps_linux(args: &Cli) -> Result<(), Report> {

    restart("kitty", "SIGUSR1")?;
    restart("waybar", "SIGUSR2")?;

    Ok(set_gtk_theme(&args)?)
}

fn restart(name: &str, signal: &str) -> Result<(), Report> {
    info!("Restarting {}", name);
    let mut kill = Command::new("pkill");
    kill.arg(format!("-{}", signal));
    kill.arg(name);

    kill.spawn()?;
    Ok(())
}

fn set_gtk_theme(args: &Cli) -> Result<(), Report> {
    info!("Setting gtk theme to {}", "test");
    let mode = if &args.lightmode == &Some(true){
        "light"
    } else {
        "dark"
    };
    
    let mut cmd = Command::new("gsettings");
    cmd.arg("set");
    cmd.arg("org.gnome.desktop.interface");
    cmd.arg("gtk-theme");
    cmd.arg(format!("adw-gtk3-{}", mode));

    cmd.spawn()?;
    Ok(())
}