use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use color_eyre::{Help, Report};

use serde::{Deserialize, Serialize};

use crate::Template;

#[derive(Serialize, Deserialize, Debug)]
pub enum WallpaperTool {
    Swaybg,
    Hyprwall,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub reload_apps: Option<bool>,
    pub set_wallpaper: Option<bool>,
    pub wallpaper_tool: Option<WallpaperTool>,
    pub prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub config: Config,
    pub templates: HashMap<String, Template>,
}

impl ConfigFile {
    pub fn read() -> Result<ConfigFile, Report> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "InioX", "matugen") {
            let proj_dir = proj_dirs.config_dir();
            let config_file = PathBuf::from(proj_dir).join("config.toml");

            let content: String = match fs::read_to_string(config_file) {
                Ok(content) => content,
                Err(e) => {
                    return Err(Report::new(e)
                        .note("This might have failed due to the config file not being found.")
                        .suggestion(format!(
                            "Try making a config.toml file in {}.",
                            proj_dir.display()
                        )))
                }
            };

            let tomlstr: ConfigFile = toml::from_str(&content)?;

            Ok(tomlstr)
        } else {
            todo!();
        }
    }
}
