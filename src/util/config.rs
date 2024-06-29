use directories::ProjectDirs;
use material_colors::color::Argb;
use std::fs;
use std::path::PathBuf;
use std::{collections::HashMap, str::FromStr};

use color_eyre::{Help, Report};

use serde::{Deserialize, Serialize};

use super::arguments::Cli;
use crate::Template;

#[derive(Serialize, Deserialize, Debug)]
pub enum WallpaperTool {
    Swaybg,
    Swww,
    Nitrogen,
    Feh,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CustomColor {
    Color(String),
    Options { color: String, blend: bool },
}

impl CustomColor {
    pub fn to_custom_color(
        &self,
        name: String,
    ) -> Result<material_colors::theme::CustomColor, material_colors::error::Error> {
        Ok(match self {
            CustomColor::Color(color) => material_colors::theme::CustomColor {
                value: Argb::from_str(color)?,
                blend: true,
                name,
            },
            CustomColor::Options { color, blend } => material_colors::theme::CustomColor {
                value: Argb::from_str(color)?,
                blend: *blend,
                name,
            },
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub reload_apps: Option<bool>,
    pub version_check: Option<bool>,
    pub reload_apps_list: Option<Apps>,
    pub set_wallpaper: Option<bool>,
    pub wallpaper_tool: Option<WallpaperTool>,
    // TODO: Add a `Command` struct
    pub swww_options: Option<Vec<String>>,
    pub feh_options: Option<Vec<String>>,
    pub prefix: Option<String>,
    pub custom_keywords: Option<HashMap<String, String>>,
    pub custom_colors: Option<HashMap<String, CustomColor>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Apps {
    pub kitty: Option<bool>,
    pub waybar: Option<bool>,
    pub gtk_theme: Option<bool>,
    pub dunst: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub config: Config,
    pub templates: HashMap<String, Template>,
}

const ERROR_TEXT: &str =
    "Error reading config file, check https://github.com/InioX/Matugen#configuration for help";

const DEFAULT_CONFIG: &str = r#"
[config]
[templates]
"#;

impl ConfigFile {
    pub fn read(args: &Cli) -> Result<(ConfigFile, Option<PathBuf>), Report> {
        match &args.config {
            Some(config_file) => Ok((Self::read_from_custom_path(config_file)?, Some(config_file.to_path_buf()))),
            None => Ok(Self::read_from_proj_path()?),
        }
    }

    fn read_from_custom_path(config_file: &PathBuf) -> Result<ConfigFile, Report> {
        let content: String = match fs::read_to_string(config_file) {
            Ok(content) => content,
            Err(e) => {
                return Err(Report::new(e).wrap_err("Could not find the provided config file."))
            }
        };

        match toml::from_str(&content) {
            Ok(res) => Ok(res),
            Err(e) => Err(Report::new(e).suggestion(ERROR_TEXT)),
        }
    }

    fn read_from_proj_path() -> Result<(ConfigFile, Option<PathBuf>), Report> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "InioX", "matugen") {
            let proj_dir = proj_dirs.config_dir();
            let config_file = PathBuf::from(proj_dir).join("config.toml");

            if !config_file.exists() {
                return Ok((Self::read_from_fallback_path()?, None));
            }

            let content: String = fs::read_to_string(&config_file).unwrap();
            match toml::from_str(&content) {
                Ok(res) => Ok((res, Some(config_file))),
                Err(e) => Err(Report::new(e).suggestion(ERROR_TEXT)),
            }
        } else {
            Ok((Self::read_from_fallback_path()?, None))
        }
    }

    fn read_from_fallback_path() -> Result<ConfigFile, Report> {
        Ok(toml::from_str(DEFAULT_CONFIG)?)
    }
}
