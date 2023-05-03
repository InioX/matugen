use color_eyre::{eyre::Result, Report};

use regex::Regex;
use serde::{Deserialize, Serialize};

use std::str;

use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::util::color::SchemeExt;
use crate::Scheme;

use super::config::ConfigFile;
use material_color_utilities_rs::util::color::format_argb_as_rgb;
use resolve_path::PathResolveExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
}

#[derive(Debug)]
struct ColorType {
    pattern: Regex,
    replacement: String,
}

#[derive(Debug)]
struct Pattern {
    hex: ColorType,
    hex_stripped: ColorType,
    rgb: ColorType,
    rgba: ColorType,
}

use super::color::Color;

impl Template {
    pub fn new(colors: &Vec<&str>, scheme: Scheme, config: ConfigFile) -> Result<(), Report> {
        let prefix: &String = &config.config.prefix.unwrap_or("@".to_string());

        info!("Loaded {} templates.", &config.templates.len());

        let regexvec: Vec<Pattern> = generate_patterns(colors, scheme, &prefix)?;

        for (name, template) in &config.templates {
            let input_path_absolute = template.input_path.try_resolve()?;
            let output_path_absolute = template.output_path.try_resolve()?;

            if !input_path_absolute.exists() {
                warn!("<d>The <yellow><b>{}</><d> template in <u>{}</><d> doesnt exist, skipping...</>", name, input_path_absolute.display());
                continue;
            }

            let mut data = read_to_string(&input_path_absolute)?;

            replace_matches(&regexvec, &mut data);

            let mut output_file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&output_path_absolute)?;

            success!(
                "Exported the <b><green>{}</> template to <d><u>{}</>",
                name,
                output_path_absolute.display()
            );
            output_file.write_all(&data.as_bytes())?;
        }
        Ok(())
    }
}

fn replace_matches(regexvec: &Vec<Pattern>, data: &mut String) {
    for regex in regexvec {
        *data = regex
            .hex
            .pattern
            .replace_all(&*data, regex.hex.replacement.to_string())
            .to_string();

        *data = regex
            .rgb
            .pattern
            .replace_all(&*data, regex.rgb.replacement.to_string())
            .to_string();

        *data = regex
            .rgba
            .pattern
            .replace_all(&*data, regex.rgba.replacement.to_string())
            .to_string();

        *data = regex
            .hex_stripped
            .pattern
            .replace_all(&*data, regex.hex_stripped.replacement.to_string())
            .to_string();
    }
}

fn generate_patterns(
    colors: &Vec<&str>,
    scheme: Scheme,
    prefix: &String,
) -> Result<Vec<Pattern>, Report> {
    let mut regexvec: Vec<Pattern> = vec![];
    for field in colors {
        let color: Color = Color::new(*Scheme::get_value(&scheme, field));

        regexvec.push(Pattern {
            hex: ColorType {
                pattern: Regex::new(&format!(r#"\{prefix}\{{{field}}}"#).to_string())?,
                replacement: format_argb_as_rgb([color.alpha, color.red, color.blue, color.green]),
            },
            hex_stripped: ColorType {
                pattern: Regex::new(&format!(r#"\{prefix}\{{{field}.strip}}"#).to_string())?,
                replacement: format_argb_as_rgb([color.alpha, color.red, color.blue, color.green])
                    [1..]
                    .to_string(),
            },
            rgb: ColorType {
                pattern: Regex::new(&format!(r#"\{prefix}\{{{field}.rgb}}"#).to_string())?,
                replacement: format!("rgb({:?}, {:?}, {:?})", color.red, color.blue, color.green),
            },
            rgba: ColorType {
                pattern: Regex::new(&format!(r#"\{prefix}\{{{field}.rgba}}"#).to_string())?,
                replacement: format!(
                    "rgba({:?}, {:?}, {:?}, {:?})",
                    color.red, color.green, color.blue, color.alpha
                ),
            },
        });
    }
    Ok(regexvec)
}
