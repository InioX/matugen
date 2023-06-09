use color_eyre::{eyre::Result, Report};

use regex::Regex;
use serde::{Deserialize, Serialize};

use std::str;

use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::util::arguments::Commands;
use crate::util::color::SchemeExt;
use crate::Scheme;

use super::arguments::Cli;
use super::config::ConfigFile;
use material_color_utilities_rs::util::color::format_argb_as_rgb;
use resolve_path::PathResolveExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
}

struct ColorPattern {
    pattern: Regex,
    replacement: String,
}

struct ImagePattern<'a> {
    pattern: Regex,
    replacement: Option<&'a String>,
}

struct ColorPatterns {
    hex: ColorPattern,
    hex_stripped: ColorPattern,
    rgb: ColorPattern,
    rgba: ColorPattern,
}

struct Patterns<'a> {
    colors: Vec<ColorPatterns>,
    image: ImagePattern<'a>,
}

use colors_transform::{Color as TransformColor, Rgb};

use super::color::Color;

impl Template {
    pub fn generate(
        colors: &Vec<&str>,
        scheme: Scheme,
        config: &ConfigFile,
        args: &Cli,
    ) -> Result<(), Report> {
        let default_prefix = "@".to_string();

        let prefix: &String = match &config.config.prefix {
            Some(prefix) => prefix,
            None => &default_prefix,
        };

        info!("Loaded {} templates.", &config.templates.len());

        let image = match &args.source {
            Commands::Image { path } => Some(path),
            Commands::Color { .. } => None,
        };

        // TODO Use only one regex and use a for loop with matches?
        let regexvec: Patterns = generate_patterns(colors, scheme, prefix, image)?;

        // println!("{}", imageregex.is_match("@{image}"));

        for (name, template) in &config.templates {
            let input_path_absolute = template.input_path.try_resolve()?;
            let output_path_absolute = template.output_path.try_resolve()?;

            if !input_path_absolute.exists() {
                warn!("<d>The <yellow><b>{}</><d> template in <u>{}</><d> doesnt exist, skipping...</>", name, input_path_absolute.display());
                continue;
            }

            let mut data = read_to_string(&input_path_absolute)?;

            replace_matches(&regexvec, &mut data, scheme);

            let mut output_file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&output_path_absolute)?;

            output_file.write_all(data.as_bytes())?;
            success!(
                "Exported the <b><green>{}</> template to <d><u>{}</>",
                name,
                output_path_absolute.display()
            );
        }
        Ok(())
    }
}

fn replace_custom_match(regex: &Regex, data: &mut String, scheme: Scheme) {
    let captures = regex.captures(&data);

    if let Some(caps) = captures {
        let (field, format, hue, saturation) =
            (caps.get(1), caps.get(2), caps.get(3), caps.get(4));
        
        if hue.is_none() | saturation.is_none() == false {
            println!("hue: {:?} saturation: {:?}", hue, saturation);
            println!("regex: {:?}", regex);
            println!("field: {:?}", field);

            let color: [u8; 4] = *Scheme::get_value(&scheme, field.unwrap().into());

            let bleh: Rgb = Rgb::from(color[1] as f32, color[2] as f32, color[3] as f32)
                .set_hue(hue.unwrap().as_str().parse::<f32>().unwrap())
                .set_saturation(saturation.unwrap().as_str().parse::<f32>().unwrap());

            let parsed_format: &str = match format {
                Some(format) => format.as_str(),
                None => ".hex"
            };

            let bleh_color = match parsed_format {
                ".hex" => {
                    bleh.to_css_hex_string()
                },
                ".strip" => {
                    bleh.to_css_hex_string()[..1].to_string()
                },
                ".rgb" => {
                    bleh.to_css_string()
                },
                ".rgba" => {
                    bleh.to_css_string()
                },
                _ => panic!()
            };

            *data = regex
                .replace(&*data, bleh_color)
                .to_string()
        }
    }
}

fn replace_matches(regexvec: &Patterns, data: &mut String, scheme: Scheme) {
    for regex in &regexvec.colors {

        println!("{:?}", regex.hex.pattern);
        
        replace_custom_match(&regex.hex.pattern, data, scheme);
        *data = regex
            .hex
            .pattern
            .replace_all(&*data, regex.hex.replacement.to_string())
            .to_string();

        replace_custom_match(&regex.rgb.pattern, data, scheme);
        *data = regex
            .rgb
            .pattern
            .replace_all(&*data, regex.rgb.replacement.to_string())
            .to_string();

        replace_custom_match(&regex.rgba.pattern, data, scheme);
        *data = regex
            .rgba
            .pattern
            .replace_all(&*data, regex.rgba.replacement.to_string())
            .to_string();

        replace_custom_match(&regex.hex_stripped.pattern, data, scheme);
        *data = regex
            .hex_stripped
            .pattern
            .replace_all(&*data, regex.hex_stripped.replacement.to_string())
            .to_string();
    }
    if let Some(image) = regexvec.image.replacement {
        *data = regexvec
            .image
            .pattern
            .replace_all(&*data, image)
            .to_string();
    }
}

fn generate_patterns<'a>(
    colors: &'a Vec<&'a str>,
    scheme: Scheme,
    prefix: &'a String,
    image: Option<&'a String>,
) -> Result<Patterns<'a>, Report> {
    let mut regexvec: Vec<ColorPatterns> = vec![];
    for field in colors {
        let color: Color = Color::new(*Scheme::get_value(&scheme, field));

        regexvec.push(ColorPatterns {
            hex: ColorPattern {
                pattern: Regex::new(&format!(r#"\{prefix}\{{({field})(\.hex)?(?:\(\s*(-?\d{{1,3}})?\s*[,]\s*(-?\d{{1,3}})?\)?\s*\)?)?\}}"#).to_string())?,
                replacement: format_argb_as_rgb([color.alpha, color.red, color.green, color.blue]),
            },
            hex_stripped: ColorPattern {
                pattern: Regex::new(&format!(r#"\{prefix}\{{{field}\.strip(?:\(\s*(-?\d{{1,3}})?\s*[,]\s*(-?\d{{1,3}})?\)?\s*\)?)?\}}"#).to_string())?,
                replacement: format_argb_as_rgb([color.alpha, color.red, color.green, color.blue])
                    [1..]
                    .to_string(),
            },
            rgb: ColorPattern {
                pattern: Regex::new(&format!(r#"\{prefix}\{{{field}\.rgb(?:\(\s*(-?\d{{1,3}})?\s*[,]\s*(-?\d{{1,3}})?\)?\s*\)?)?\}}"#).to_string())?,
                replacement: format!("rgb({:?}, {:?}, {:?})", color.red, color.green, color.blue),
            },
            rgba: ColorPattern {
                pattern: Regex::new(&format!(r#"\{prefix}\{{{field}\.rgba(?:\(\s*(-?\d{{1,3}})?\s*[,]\s*(-?\d{{1,3}})?\)?\s*\)?)?\}}"#).to_string())?,
                replacement: format!(
                    "rgba({:?}, {:?}, {:?}, {:?})",
                    color.red, color.green, color.blue, color.alpha
                ),
            },
        });
    }
    Ok(Patterns {
        colors: regexvec,
        image: ImagePattern {
            pattern: Regex::new(&format!(r#"\{prefix}\{{image}}"#))?,
            replacement: image,
        },
    })
}
