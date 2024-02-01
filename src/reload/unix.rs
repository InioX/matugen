use crate::util::{arguments::Cli, config::ConfigFile};
use color_eyre::{eyre::Result, Report};

use std::process::Command;

use crate::SchemesEnum;

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub fn reload(args: &Cli, config: &ConfigFile) -> Result<(), Report> {
    if config.config.reload_apps_list.is_none() {
        warn!("<d>The option <yellow><u>config.reload_apps<b></><d> is set to <u><green>TRUE</>, but <yellow><u>config.reload_apps_list</><d> is <u><red>EMPTY</>. Not restarting any apps...</>");
        return Ok(());
    }

    let reload_apps_list = &config.config.reload_apps_list.as_ref().unwrap();

    if reload_apps_list.waybar == Some(true) || reload_apps_list.waybar.is_none() {
        reload_app("waybar", "SIGUSR2")?;
    }

    if reload_apps_list.kitty == Some(true) || reload_apps_list.waybar.is_none() {
        reload_app("kitty", "SIGUSR1")?;
    }

    if reload_apps_list.dunst == Some(true) || reload_apps_list.waybar.is_none() {
        reload_app("dunst", "SIGUSR2")?;
    }

    if reload_apps_list.gtk_theme == Some(true) || reload_apps_list.waybar.is_none() {
        reload_gtk_theme(args)?;
    }

    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub fn reload_app(name: &str, signal: &str) -> Result<(), Report> {
    info!("Restarting {}", name);
    let mut kill = Command::new("pkill");
    kill.arg(format!("-{}", signal));
    kill.arg(name);

    kill.spawn()?;
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
fn reload_gtk_theme(args: &Cli) -> Result<(), Report> {
    let mode = match args.mode {
        Some(SchemesEnum::Light) => "light",
        Some(SchemesEnum::Dark) => "dark",
        None => "dark",
    };

    info!("Setting gtk theme to adw-gtk3-{}", mode);

    set_theme("")?;
    set_theme(format!("adw-gtk3-{}", mode).as_str())?;
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
fn set_theme(theme: &str) -> Result<(), Report> {
    Command::new("gsettings")
        .args(["set", "org.gnome.desktop.interface", "gtk-theme", theme])
        .spawn()?;

    Ok(())
}
