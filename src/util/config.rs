use directories::ProjectDirs;
use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
};

use color_eyre::{Help, Report};

use serde::{Deserialize, Serialize};

use super::arguments::Cli;
use crate::{wallpaper::Wallpaper, Template};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub version_check: Option<bool>,
    pub caching: Option<bool>,
    pub wallpaper: Option<Wallpaper>,
    // TODO: Add a `Command` struct
    pub prefix: Option<String>,
    pub custom_colors: Option<HashMap<String, crate::color::color::OwnCustomColor>>,
    pub expr_prefix: Option<String>,
    pub expr_postfix: Option<String>,
    pub block_prefix: Option<String>,
    pub block_postfix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

pub enum ProjectDirsTypes {
    Config,
    Cache,
}

pub fn get_proj_path(dir_type: &ProjectDirsTypes) -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "InioX", "matugen") {
        let file = match dir_type {
            ProjectDirsTypes::Config => PathBuf::from(proj_dirs.config_dir()),
            ProjectDirsTypes::Cache => PathBuf::from(proj_dirs.cache_dir()),
        };

        Some(file)
    } else {
        None
    }
}

impl ConfigFile {
    pub fn read(args: &Cli) -> Result<(ConfigFile, Option<PathBuf>), Report> {
        match &args.config {
            Some(config_file) => Ok((
                Self::read_from_custom_path(config_file)?,
                Some(config_file.to_path_buf()),
            )),
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
        if let Some(mut path) = get_proj_path(&ProjectDirsTypes::Config) {
            path = path.join("config.toml");
            if path.exists() {
                let content: String = fs::read_to_string(&path).unwrap();
                match toml::from_str(&content) {
                    Ok(res) => Ok((res, Some(path))),
                    Err(e) => Err(Report::new(e).suggestion(ERROR_TEXT)),
                }
            } else {
                Ok((Self::read_from_fallback_path()?, None))
            }
        } else {
            Ok((Self::read_from_fallback_path()?, None))
        }
    }

    fn read_from_fallback_path() -> Result<ConfigFile, Report> {
        Ok(toml::from_str(DEFAULT_CONFIG)?)
    }
}
