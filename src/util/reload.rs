use super::{arguments::Cli,config::ConfigFile};
use color_eyre::{eyre::Result, Report};
use std::process::Command;

#[cfg(target_os = "linux")]
pub fn reload_apps_linux(args: &Cli, config: &ConfigFile) -> Result<(), Report> {

    reload_app("kitty", "SIGUSR1")?;
    
    if config.config.reload_gtk_theme == Some(true) {
        reload_gtk_theme(args)?;
    }

    Ok(())
}

pub fn reload_app(name: &str, signal: &str) -> Result<(), Report> {
    info!("Restarting {}", name);
    let mut kill = Command::new("pkill");
    kill.arg(format!("-{}", signal));
    kill.arg(name);

    kill.spawn()?;
    Ok(())
}

fn reload_gtk_theme(args: &Cli) -> Result<(), Report> {
    let mode = if args.lightmode == Some(true) {
        "light"
    } else {
        "dark"
    };

    info!("Setting gtk theme to adw-gtk3-{}", mode);

    let mut cmd = Command::new("gsettings");
    cmd.arg("set");
    cmd.arg("org.gnome.desktop.interface");
    cmd.arg("gtk-theme");
    cmd.arg(format!("adw-gtk3-{}", mode));

    cmd.spawn()?;
    Ok(())
}
