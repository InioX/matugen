use color_eyre::{eyre::Result, Report};

use material_color_utilities_rs::scheme;
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

use crate::{Schemes, SchemesEnum};

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub mode: Option<SchemesEnum>,
}

struct ColorPattern {
    pattern: Regex,
    replacements: ColorReplacements,
}

struct ImagePattern<'a> {
    pattern: Regex,
    replacement: Option<&'a String>,
}

pub struct ColorReplacement {
    hex: String,
    hex_stripped: String,
    rgb: String,
    rgba: String,
}

struct Patterns<'a> {
    colors: Vec<ColorPattern>,
    image: ImagePattern<'a>,
}
pub struct ColorReplacements {
    pub light: ColorReplacement,
    pub dark: ColorReplacement,
    pub amoled: ColorReplacement,
}

use super::color::Color;

impl Template {
    pub fn generate(
        colors: &Vec<&str>,
        schemes: &Schemes,
        config: &ConfigFile,
        args: &Cli,
        source_color: &[u8; 4],
        default_scheme: &SchemesEnum,
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

        let regexvec: Patterns = generate_patterns(colors, &schemes, prefix, image, source_color)?;

        for (name, template) in &config.templates {
            println!("{}", name);

            let input_path_absolute = template.input_path.try_resolve()?;
            let output_path_absolute = template.output_path.try_resolve()?;

            if !input_path_absolute.exists() {
                warn!("<d>The <yellow><b>{}</><d> template in <u>{}</><d> doesnt exist, skipping...</>", name, input_path_absolute.display());
                continue;
            }

            let mut data = read_to_string(&input_path_absolute)?;

            replace_matches(
                &regexvec,
                &mut data,
                &template.mode,
                &schemes,
                &default_scheme,
            );

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

fn replace_matches(
    regexvec: &Patterns,
    data: &mut String,
    scheme: &Option<SchemesEnum>,
    schemes: &Schemes,
    default_scheme: &SchemesEnum,
) {
    for regex in &regexvec.colors {
        let captures = regex.pattern.captures(data);

        let format = if let Some(caps) = captures {
            caps.get(1)
        } else {
            continue;
        };

        let replacement = if let Some(scheme) = &scheme {
            match scheme {
                SchemesEnum::Light => &regex.replacements.light,
                SchemesEnum::Dark => &regex.replacements.dark,
                SchemesEnum::Amoled => &regex.replacements.amoled,
            }
        } else {
            match default_scheme {
                SchemesEnum::Light => &regex.replacements.light,
                SchemesEnum::Dark => &regex.replacements.dark,
                SchemesEnum::Amoled => &regex.replacements.amoled,
            }
        };

        dbg!(scheme);

        if format.is_some() {
            match format.unwrap().as_str() {
                ".hex" => {
                    *data = regex
                        .pattern
                        .replace_all(data, &replacement.hex)
                        .to_string()
                }
                ".strip" => {
                    *data = regex
                        .pattern
                        .replace_all(data, &replacement.hex_stripped)
                        .to_string()
                }
                ".rgb" => {
                    *data = regex
                        .pattern
                        .replace_all(data, &replacement.rgb)
                        .to_string()
                }
                ".rgba" => {
                    *data = regex
                        .pattern
                        .replace_all(data, &replacement.rgba)
                        .to_string()
                }
                _ => continue,
            }
        } else {
            *data = regex
                .pattern
                .replace_all(data, &replacement.hex)
                .to_string()
        }
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
    schemes: &Schemes,
    prefix: &'a String,
    image: Option<&'a String>,
    source_color: &[u8; 4],
) -> Result<Patterns<'a>, Report> {
    let mut regexvec: Vec<ColorPattern> = vec![];
    for field in colors {
        let color_light: Color =
            Color::new(*Scheme::get_value(&schemes.light, field, source_color));
        let color_dark: Color = Color::new(*Scheme::get_value(&schemes.dark, field, source_color));
        let color_amoled: Color =
            Color::new(*Scheme::get_value(&schemes.amoled, field, source_color));

        regexvec.push(ColorPattern {
            pattern: Regex::new(
                &format!(r#"\{prefix}\{{{field}(\.hex|\.rgb|\.rgba|\.strip)?}}"#).to_string(),
            )?,
            replacements: ColorReplacements {
                light: ColorReplacement {
                    hex: format_argb_as_rgb([
                        color_light.alpha,
                        color_light.red,
                        color_light.green,
                        color_light.blue,
                    ]),
                    hex_stripped: format_argb_as_rgb([
                        color_light.alpha,
                        color_light.red,
                        color_light.green,
                        color_light.blue,
                    ])[1..]
                        .to_string(),
                    rgb: format!(
                        "rgb({:?}, {:?}, {:?})",
                        color_light.red, color_light.green, color_light.blue
                    ),
                    rgba: format!(
                        "rgba({:?}, {:?}, {:?}, {:?})",
                        color_light.red, color_light.green, color_light.blue, color_light.alpha
                    ),
                },
                dark: ColorReplacement {
                    hex: format_argb_as_rgb([
                        color_dark.alpha,
                        color_dark.red,
                        color_dark.green,
                        color_dark.blue,
                    ]),
                    hex_stripped: format_argb_as_rgb([
                        color_dark.alpha,
                        color_dark.red,
                        color_dark.green,
                        color_dark.blue,
                    ])[1..]
                        .to_string(),
                    rgb: format!(
                        "rgb({:?}, {:?}, {:?})",
                        color_dark.red, color_dark.green, color_dark.blue
                    ),
                    rgba: format!(
                        "rgba({:?}, {:?}, {:?}, {:?})",
                        color_dark.red, color_dark.green, color_dark.blue, color_dark.alpha
                    ),
                },
                amoled: ColorReplacement {
                    hex: format_argb_as_rgb([
                        color_amoled.alpha,
                        color_amoled.red,
                        color_amoled.green,
                        color_amoled.blue,
                    ]),
                    hex_stripped: format_argb_as_rgb([
                        color_amoled.alpha,
                        color_amoled.red,
                        color_amoled.green,
                        color_amoled.blue,
                    ])[1..]
                        .to_string(),
                    rgb: format!(
                        "rgb({:?}, {:?}, {:?})",
                        color_amoled.red, color_amoled.green, color_amoled.blue
                    ),
                    rgba: format!(
                        "rgba({:?}, {:?}, {:?}, {:?})",
                        color_amoled.red, color_amoled.green, color_amoled.blue, color_amoled.alpha
                    ),
                },
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
