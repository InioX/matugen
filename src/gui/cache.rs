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
    if let Some(path) = get_proj_path(&ProjectDirsTypes::Cache) {
        // dbg!(&path);
        // std::fs::create_dir_all(&path).expect("Failed to crate cache folder");
        fs::write(path.join("config.toml"), toml).expect("Failed saving cache")
    }
}
