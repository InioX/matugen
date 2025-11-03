use std::{
    fmt,
    fs::{create_dir_all, read_to_string, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::{
    color::color::Source,
    scheme::{Schemes, SchemesEnum},
    util::config::{get_proj_path, ProjectDirsTypes},
};
use color_eyre::Report;
use image::ImageReader;
use indexmap::IndexMap;
use material_colors::color::Argb;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheFile {
    mode: SchemesEnum,
    colors: SchemesCache,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemesCache {
    light: IndexMap<String, ArgbHelper>,
    dark: IndexMap<String, ArgbHelper>,
}

#[derive(Debug, Copy, Clone)]
pub struct ArgbHelper {
    alpha: u8,
    red: u8,
    green: u8,
    blue: u8,
}

impl Serialize for ArgbHelper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_string = format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.alpha
        );
        serializer.serialize_str(&hex_string)
    }
}

impl<'de> Deserialize<'de> for ArgbHelper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArgbVisitor;

        impl<'de> Visitor<'de> for ArgbVisitor {
            type Value = ArgbHelper;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a hex string in the format #RRGGBBAA")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let v = v
                    .strip_prefix('#')
                    .ok_or_else(|| E::custom("missing '#' prefix"))?;
                if v.len() != 8 {
                    return Err(E::custom("hex string must be 8 characters (AARRGGBB)"));
                }
                let red = u8::from_str_radix(&v[0..2], 16).map_err(E::custom)?;
                let green = u8::from_str_radix(&v[2..4], 16).map_err(E::custom)?;
                let blue = u8::from_str_radix(&v[4..6], 16).map_err(E::custom)?;
                let alpha = u8::from_str_radix(&v[6..8], 16).map_err(E::custom)?;

                Ok(ArgbHelper {
                    alpha,
                    red,
                    green,
                    blue,
                })
            }
        }

        deserializer.deserialize_str(ArgbVisitor)
    }
}

impl From<ArgbHelper> for Argb {
    fn from(value: ArgbHelper) -> Self {
        Argb {
            alpha: value.alpha,
            red: value.red,
            green: value.green,
            blue: value.blue,
        }
    }
}

impl From<Argb> for ArgbHelper {
    fn from(value: Argb) -> Self {
        ArgbHelper {
            alpha: value.alpha,
            red: value.red,
            green: value.green,
            blue: value.blue,
        }
    }
}

fn convert_helper_scheme(scheme: &IndexMap<String, ArgbHelper>) -> IndexMap<String, Argb> {
    scheme
        .iter()
        .map(|(k, v)| (k.clone(), (*v).into()))
        .collect()
}

pub fn convert_argb_scheme(scheme: &IndexMap<String, Argb>) -> IndexMap<String, ArgbHelper> {
    scheme
        .iter()
        .map(|(k, v)| (k.clone(), (*v).into()))
        .collect()
}

pub struct ImageCache {
    pub hash: Option<String>,
    source: Option<PathBuf>,
    cache_folder: PathBuf,
}

impl ImageCache {
    pub fn new(source: &Source) -> Self {
        let pathbuf = match source {
            Source::Image { path } => Some(PathBuf::from(path)),
            _ => None,
        };

        let cache_folder = get_proj_path(&ProjectDirsTypes::Cache)
            .unwrap()
            .join("images");

        Self {
            hash: get_cache(source),
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

        let json = serde_json::to_string_pretty(value)?;

        let mut writer = BufWriter::new(file);
        writer.write_all(json.as_bytes())?;

        success!(
            "Saved cache of <b><green>{}</> to <d><u>{}</>",
            self.source.as_ref().unwrap().display(),
            path.display()
        );

        Ok(())
    }

    pub fn load(&self) -> Result<(Schemes, SchemesEnum), Report> {
        let path = self.cache_folder.join(self.get_name());

        let string = read_to_string(&path)?;

        let json: CacheFile = serde_json::from_str(&string)?;

        let schemes_enum = Schemes {
            dark: convert_helper_scheme(&json.colors.dark),
            light: convert_helper_scheme(&json.colors.light),
        };

        success!("Loaded cache from <<d><u>{}</>", path.display());

        Ok((schemes_enum, json.mode))
    }

    fn get_name(&self) -> PathBuf {
        let name = format!(
            "{}.{}.json",
            self.source
                .as_ref()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy(),
            self.hash.as_ref().unwrap()
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
