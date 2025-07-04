use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use crate::color::color::Source;
use crate::scheme::Schemes;
use crate::scheme::SchemesEnum;
use crate::util::config::get_proj_path;
use crate::util::config::ProjectDirsTypes;
use chumsky::container::Seq;
use color_eyre::Report;
use image::ImageReader;
use indexmap::IndexMap;
use material_colors::color::Argb;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheFile {
    mode: SchemesEnum,
    image: String,
    colors: SchemesCache,
    custom: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemesCache {
    light: IndexMap<String, ArgbHelper>,
    dark: IndexMap<String, ArgbHelper>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct ArgbHelper {
    alpha: u8,
    red: u8,
    green: u8,
    blue: u8,
}

impl From<ArgbHelper> for Argb {
    fn from(h: ArgbHelper) -> Self {
        Argb {
            alpha: h.alpha,
            red: h.red,
            green: h.green,
            blue: h.blue,
        }
    }
}

fn convert_scheme(scheme: &IndexMap<String, ArgbHelper>) -> IndexMap<String, Argb> {
    scheme
        .iter()
        .map(|(k, v)| (k.clone(), (*v).into()))
        .collect()
}

pub struct ImageCache {
    pub hash: String,
    source: PathBuf,
    cache_folder: PathBuf,
}

impl ImageCache {
    pub fn new(source: &Source) -> Self {
        let pathbuf = match source {
            Source::Image { path } => PathBuf::from(path),
            _ => panic!("Cache only works with image"),
        };

        let cache_folder = get_proj_path(&ProjectDirsTypes::Cache).unwrap();

        Self {
            hash: get_cache(source).unwrap(),
            source: pathbuf,
            cache_folder,
        }
    }

    pub fn save(&self, value: &Value) -> Result<(), Report> {
        let path = self.cache_folder.join(self.get_name());

        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }

        let file = match File::create(&path) {
            Ok(v) => v,
            Err(e) => {
                error!("Could not create the path <b><red>{}</>.", &path.display());
                return Err(e.into());
            }
        };

        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, value)?;

        success!(
            "Saved cache of <b><green>{}</> to <d><u>{}</>",
            self.source.display(),
            path.display()
        );

        Ok(())
    }

    pub fn load(&self) -> Result<(Schemes, SchemesEnum, Value), Report> {
        let path = self.cache_folder.join(self.get_name());

        let string = read_to_string(path)?;

        let json: CacheFile = serde_json::from_str(&string)?;

        let schemes_enum = Schemes {
            dark: convert_scheme(&json.colors.dark),
            light: convert_scheme(&json.colors.light),
        };

        let value = serde_json::json!({
            "image": json.image,
            "custom": json.custom,
            "mode": json.mode,
        });

        Ok((schemes_enum, json.mode, value))
    }

    fn get_name(&self) -> PathBuf {
        let name = format!(
            "{}.{}.json",
            self.source.file_name().unwrap().to_string_lossy(),
            self.hash
        );

        let mut file = PathBuf::new();
        file.push(name);

        file
    }
}

fn get_cache(source: &Source) -> Option<String> {
    match &source {
        Source::Image { path } => match hash_image_from_path(path) {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        },
        _ => None,
    }
}

fn hash_image_from_path(path: &str) -> Result<String, Report> {
    let img = ImageReader::open(path)?.decode()?;
    let bytes = img.into_bytes();

    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash = hasher.finalize();

    Ok(format!("{:x}", hash))
}
