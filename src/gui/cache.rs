use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::util::config::get_proj_path;
use crate::util::config::ProjectDirsTypes;

use super::init::Tabs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub image_folder: PathBuf,
    pub selected_tab: Tabs,
}

pub fn save_cache(image_folder: PathBuf, selected_tab: Tabs) {
    let config = Config {
        image_folder,
        selected_tab,
    };
    let toml = toml::to_string(&config).unwrap();
    if let Some(mut path) = get_proj_path(&ProjectDirsTypes::Cache) {
        path = path.join("ui_cache.toml");

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create cache folder");
        }

        fs::write(path, toml).expect("Failed saving cache")
    }
}

pub fn read_cache() -> Option<Config> {
    if let Some(mut path) = get_proj_path(&ProjectDirsTypes::Cache) {
        path = path.join("ui_cache.toml");

        let str = match fs::read_to_string(path) {
            Ok(v) => v,
            Err(_) => return None,
        };
        Some(toml::from_str(&str).expect("Couldn't parse cache file"))
    } else {
        None
    }
}
