use color_eyre::Report;
use std::process::Command;
use std::process::Stdio;

use super::{
    arguments::{Cli, Source},
    config::{ConfigFile, WallpaperTool},
    reload::reload_app,
};

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub fn set_wallaper(config: &ConfigFile, args: &Cli) -> Result<(), Report> {
    let wallpaper_tool = match &config.config.wallpaper_tool {
        Some(wallpaper_tool) => wallpaper_tool,
        None => {
            return Ok(warn!(
                "<d>Wallpaper tool not set, not setting wallpaper...</>"
            ))
        }
    };

    let path = match &args.source {
        Source::Image { path } => path,
        Source::Color { .. } => return Ok(()),
    };

    match wallpaper_tool {
        WallpaperTool::Swaybg => set_wallaper_swaybg(path),
        WallpaperTool::Swww => set_wallaper_swwww(config, path),
        WallpaperTool::Nitrogen => set_wallaper_nitrogen(path),
        WallpaperTool::Feh => set_wallaper_feh(config, path),
    }
}

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
fn set_wallaper_swaybg(path: &String) -> Result<(), Report> {
    reload_app("swaybg", "SIGUSR1")?;

    let mut binding = Command::new("swaybg");
    let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.arg("-i");
    cmd.arg(path);

    cmd.spawn()?;
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
fn set_wallaper_swwww(config: &ConfigFile, path: &String) -> Result<(), Report> {
    let mut binding = Command::new("swww");
    let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.arg("img");
    cmd.arg(path);

    if let Some(options) = &config.config.swww_options {
        if !options[0].is_empty() {
            cmd.args(options);
        }
    }

    cmd.spawn()?;
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
fn set_wallaper_nitrogen(path: &String) -> Result<(), Report> {
    let mut binding = Command::new("nitrogen");
    let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.arg(path);

    cmd.spawn()?;
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
fn set_wallaper_feh(config: &ConfigFile, path: &String) -> Result<(), Report> {
    let mut binding = Command::new("feh");
    let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());

    if let Some(options) = &config.config.feh_options {
        if !options[0].is_empty() {
            cmd.args(options);
        } else {
            cmd.arg("--bg-scale");
        }
    }

    cmd.arg(path);

    cmd.spawn()?;
    Ok(())
}
