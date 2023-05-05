use color_eyre::Report;
use std::process::Command;
use std::process::Stdio;

use super::{
    arguments::{Cli, Commands},
    config::{ConfigFile, WallpaperTool},
    reload::{reload_app}
};

pub fn set_wallaper(config: &ConfigFile, args: &Cli) -> Result<(), Report> {
    let wallpaper_tool = match &config.config.wallpaper_tool {
        Some(wallpaper_tool) => wallpaper_tool,
        None => return Ok(()),
    };

    let path = match &args.source {
        Commands::Image { path } => path,
        Commands::Color { .. } => return Ok(()),
    };

    match wallpaper_tool {
        WallpaperTool::Swaybg => set_wallaper_swaybg(path),
        WallpaperTool::Swww => set_wallaper_swwww(path),
    }
}

fn set_wallaper_swaybg(path: &String) -> Result<(), Report> {
    reload_app("swaybg", "SIGUSR1")?;
    
    let mut binding = Command::new("swaybg");
    let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.arg("-i");
    cmd.arg(path);

    cmd.spawn()?;
    Ok(())
}

fn set_wallaper_swwww(path: &String) -> Result<(), Report> {
    let mut binding = Command::new("swww");
    let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());
    cmd.arg("img");
    cmd.arg(path);

    cmd.spawn()?;
    Ok(())
}
