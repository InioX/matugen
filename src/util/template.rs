use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::WrapErr;
use color_eyre::{eyre::Result, Report};

use colorsys::Hsl;
use regex::Regex;
use serde::{Deserialize, Serialize};

use std::str;

use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::util::arguments::Source;
use crate::util::color::SchemeExt;
use crate::Scheme;

use crate::util::color::SchemeAndroidExt;
use crate::SchemeAndroid;

use super::arguments::Cli;
use super::config::ConfigFile;
use material_color_utilities_rs::util::color::format_argb_as_rgb;
use resolve_path::PathResolveExt;

use super::color::{COLORS, COLORS_ANDROID};

use crate::{Schemes, SchemesEnum};

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub mode: Option<SchemesEnum>,
}

struct ColorPattern {
    patterns: RegexPatterns,
    replacements: ColorReplacements,
}

struct RegexPatterns {
    light: Regex,
    dark: Regex,
    amoled: Regex,
    default: Regex,
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
    hsl: String,
    hsla: String,
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
            Source::Image { path } => Some(path),
            Source::Color { .. } => None,
        };

        let regexvec: Patterns = generate_patterns(schemes, prefix, image, source_color)?;

        for (name, template) in &config.templates {
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
                schemes,
                default_scheme,
            );

            debug!(
                "Trying to write the {} template to {}",
                name,
                output_path_absolute.display()
            );

            if !output_path_absolute.exists() {
                error!(
                    "The <b><yellow>{}</> folder doesnt exist, trying to create...",
                    &output_path_absolute.display()
                );
                let parent_folder = &output_path_absolute
                    .parent()
                    .wrap_err("Could not get the parent of the output path.")?;
                debug!("{}", parent_folder.display());
                let _ = create_dir_all(&parent_folder).wrap_err(format!(
                    "Failed to create the {} folders.",
                    &output_path_absolute.display()
                ));
            }

            let mut output_file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&output_path_absolute)?;

            if output_file.metadata()?.permissions().readonly() {
                error!(
                    "The <b><red>{}</> file is Read-Only",
                    &output_path_absolute.display()
                );
            }

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
    _schemes: &Schemes,
    default_scheme: &SchemesEnum,
) {
    for regex in &regexvec.colors {
        let default_replacement = if let Some(scheme) = &scheme {
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

        let _ = replace_single_match(&regex.patterns.light, data, &regex.replacements.light);
        let _ = replace_single_match(&regex.patterns.dark, data, &regex.replacements.dark);
        let _ = replace_single_match(&regex.patterns.amoled, data, &regex.replacements.amoled);

        if let Err(()) = replace_single_match(&regex.patterns.default, data, default_replacement) {
            continue;
        }
    }

    replace_image_keyword(regexvec, data);
}

fn replace_single_match(
    pattern: &Regex,
    data: &mut String,
    replacement: &ColorReplacement,
) -> Result<(), ()> {
    let captures = pattern.captures(data);

    if captures.is_none() {
        return Err(());
    }

    let format = captures.unwrap().get(1);

    if format.is_some() {
        match format.unwrap().as_str() {
            ".hex" => *data = pattern.replace_all(data, &replacement.hex).to_string(),
            ".strip" => {
                *data = pattern
                    .replace_all(data, &replacement.hex_stripped)
                    .to_string()
            }
            ".rgb" => *data = pattern.replace_all(data, &replacement.rgb).to_string(),
            ".rgba" => *data = pattern.replace_all(data, &replacement.rgba).to_string(),
            ".hsl" => *data = pattern.replace_all(data, &replacement.hsl).to_string(),
            ".hsla" => *data = pattern.replace_all(data, &replacement.hsla).to_string(),
            _ => return Err(()),
        }
    } else {
        *data = pattern.replace_all(data, &replacement.hex).to_string()
    }

    Ok(())
}

fn replace_image_keyword(regexvec: &Patterns<'_>, data: &mut String) {
    if let Some(image) = regexvec.image.replacement {
        *data = regexvec
            .image
            .pattern
            .replace_all(&*data, image)
            .to_string();
    }
}

fn generate_patterns<'a>(
    schemes: &Schemes,
    prefix: &'a String,
    image: Option<&'a String>,
    source_color: &[u8; 4],
) -> Result<Patterns<'a>, Report> {
    let mut regexvec: Vec<ColorPattern> = vec![];

    for field in COLORS {
        let color_light: Color =
            Color::new(*Scheme::get_value(&schemes.light, field, source_color));
        let color_dark: Color = Color::new(*Scheme::get_value(&schemes.dark, field, source_color));
        let color_amoled: Color =
            Color::new(*Scheme::get_value(&schemes.amoled, field, source_color));

        generate_single_pattern(
            &mut regexvec,
            prefix,
            field,
            color_light,
            color_dark,
            color_amoled,
        )?;
    }

    for field in COLORS_ANDROID {
        let color_light: Color = Color::new(*SchemeAndroid::get_value(
            &schemes.light_android,
            field,
            source_color,
        ));
        let color_dark: Color = Color::new(*SchemeAndroid::get_value(
            &schemes.dark_android,
            field,
            source_color,
        ));
        let color_amoled: Color = Color::new(*SchemeAndroid::get_value(
            &schemes.amoled_android,
            field,
            source_color,
        ));

        generate_single_pattern(
            &mut regexvec,
            prefix,
            field,
            color_light,
            color_dark,
            color_amoled,
        )?;
    }

    Ok(Patterns {
        colors: regexvec,
        image: ImagePattern {
            pattern: Regex::new(&format!(r#"\{prefix}\{{image}}"#))?,
            replacement: image,
        },
    })
}

fn generate_single_pattern<'a>(
    regexvec: &'a mut Vec<ColorPattern>,
    prefix: &'a str,
    field: &'a str,
    color_light: Color,
    color_dark: Color,
    color_amoled: Color,
) -> Result<&'a mut Vec<ColorPattern>, Report> {
    regexvec.push(ColorPattern {
        patterns: RegexPatterns {
            default: Regex::new(&format!(
                r#"\{prefix}\{{{field}(\.hex|\.rgb|\.rgba|\.strip|\.hsla|\.hsl)?}}"#
            ))?,
            light: Regex::new(&format!(
                r#"\{prefix}\{{{field}\.light(\.hex|\.rgb|\.rgba|\.strip|\.hsla|\.hsl)?}}"#
            ))?,
            dark: Regex::new(&format!(
                r#"\{prefix}\{{{field}\.dark(\.hex|\.rgb|\.rgba|\.strip|\.hsla|\.hsl)?}}"#
            ))?,
            amoled: Regex::new(&format!(
                r#"\{prefix}\{{{field}\.amoled(\.hex|\.rgb|\.rgba|\.strip|\.hsla|\.hsl)?}}"#
            ))?,
        },
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
                hsl: Hsl::new(
                    color_light.red as f64,
                    color_light.green as f64,
                    color_light.blue as f64,
                    Some(color_light.alpha as f64),
                )
                .to_css_string(),
                hsla: Hsl::new(
                    color_light.red as f64,
                    color_light.green as f64,
                    color_light.blue as f64,
                    None,
                )
                .to_css_string(),
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
                hsl: Hsl::new(
                    color_dark.red as f64,
                    color_dark.green as f64,
                    color_dark.blue as f64,
                    Some(color_dark.alpha as f64),
                )
                .to_css_string(),
                hsla: Hsl::new(
                    color_dark.red as f64,
                    color_dark.green as f64,
                    color_dark.blue as f64,
                    None,
                )
                .to_css_string(),
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
                hsl: Hsl::new(
                    color_amoled.red as f64,
                    color_amoled.green as f64,
                    color_amoled.blue as f64,
                    Some(color_amoled.alpha as f64),
                )
                .to_css_string(),
                hsla: Hsl::new(
                    color_amoled.red as f64,
                    color_amoled.green as f64,
                    color_amoled.blue as f64,
                    None,
                )
                .to_css_string(),
            },
        },
    });
    Ok(regexvec)
}
