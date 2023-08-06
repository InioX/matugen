use super::{arguments::Cli, config::ConfigFile};
use color_eyre::{eyre::Result, Report};
use std::process::Command;

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub fn reload_apps_linux(args: &Cli, config: &ConfigFile) -> Result<(), Report> {
    reload_app("kitty", "SIGUSR1")?;
    reload_app("waybar", "SIGUSR2")?;

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

    set_theme("")?;
    set_theme(format!("adw-gtk3-{}", mode).as_str())?;
    Ok(())
}

fn set_theme(theme: &str) -> Result<(), Report> {
    Command::new("gsettings")
        .args(["set", "org.gnome.desktop.interface", "gtk-theme", theme])
        .spawn()?;

    Ok(())
}
