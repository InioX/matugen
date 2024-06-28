use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::WrapErr;
use color_eyre::Help;
use color_eyre::{eyre::Result, Report};

use colorsys::{ColorAlpha, Hsl};
use material_colors::color::Argb;
use proper_path_tools::path::rebase;
use serde::{Deserialize, Serialize};

use upon::Value;

use crate::util::color;
use crate::util::color::color_to_string;
use crate::util::filters::set_lightness;
use crate::util::variables::replace_hook_keywords;

use std::str;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::util::color::{
    format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba,
};

use crate::util::arguments::Source;
use resolve_path::PathResolveExt;

use crate::{Schemes, SchemesEnum};

use upon::{Engine, Syntax};

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorDefinition {
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub mode: Option<SchemesEnum>,
    pub colors_to_compare: Option<Vec<ColorDefinition>>,
    pub compare_to: Option<String>,
    pub hook: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Colora {
    hex: String,
    hex_stripped: String,
    rgb: String,
    rgba: String,
    hsl: String,
    hsla: String,
    red: String,
    green: String,
    blue: String,
    alpha: String,
    hue: String,
    saturation: String,
    lightness: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ColorVariants {
    pub light: Colora,
    pub dark: Colora,
    pub default: Colora,
}

use super::color::rgb_from_argb;

pub fn check_string_value(value: &Value) -> Option<&String> {
    match value {
        Value::String(v) => Some(v),
        _v => None,
    }
}

pub fn parse_color(string: &String) -> Option<&str> {
    if let Some(_s) = string.strip_prefix('#') {
        return Some("hex");
    }

    if let (Some(i), Some(s)) = (string.find('('), string.strip_suffix(')')) {
        let fname = s[..i].trim_end();
        Some(fname)
    } else if string.len() == 6 {
        // Does not matter if it is actually a stripped hex or not, we handle it somewhere else.
        return Some("hex_stripped");
    } else {
        None
    }
}

impl Template {
    pub fn generate(
        schemes: &Schemes,
        templates: &HashMap<String, Template>,
        source: &Source,
        prefix: &Option<String>,
        source_color: &Argb,
        default_scheme: &SchemesEnum,
        custom_keywords: &Option<HashMap<String, String>>,
        path_prefix: &Option<PathBuf>,
        default_fill_value: Option<String>
    ) -> Result<(), Report> {
        let default_prefix = "@".to_string();

        let _prefix: &String = match &prefix {
            Some(prefix) => prefix,
            None => &default_prefix,
        };

        info!("Loaded <b><cyan>{}</> templates.", &templates.len());

        let syntax = Syntax::builder().expr("{{", "}}").block("<*", "*>").build();
        let mut engine = Engine::with_syntax(syntax);

        engine.add_filter("set_lightness", set_lightness);
        engine.add_filter("to_upper", str::to_uppercase);
        engine.add_filter("to_lower", str::to_lowercase);
        engine.add_filter("replace", |s: String, from: String, to: String| {
            s.replace(&from, &to)
        });

        let image = match &source {
            Source::Image { path } => Some(path),
            Source::WebImage { .. } => None,
            Source::Color { .. } => None,
        };

        let colors = generate_colors(schemes, source_color, default_scheme)?;

        let mut custom: HashMap<String, String> = Default::default();

        for entry in custom_keywords.iter() {
            for (name, value) in entry {
                custom.insert(name.to_string(), value.to_string());
            }
        }

        let render_data = upon::value! {
            colors: &colors, image: image, custom: &custom,
        };

        let default_fill_value = default_fill_value.unwrap_or(String::from("-"));

        // debug!("render_data: {:#?}", &render_data);

        for (i, (name, template)) in templates.iter().enumerate() {
            let input_path_absolute = template.input_path.try_resolve()?;
            let output_path_absolute = template.output_path.try_resolve()?;

            if template.hook.is_some() {
                let compared_color: Option<String> = if template.colors_to_compare.is_some() && template.compare_to.is_some() {
                    Some(color_to_string(&template.colors_to_compare.as_ref().unwrap(), &template.compare_to.as_ref().unwrap()))
                } else {
                    None
                };

                let parsed = replace_hook_keywords(&template.hook.as_ref().unwrap(), &default_fill_value, image, compared_color.as_ref(), source_color);
                println!("{}", parsed);
            }

            if template.colors_to_compare.is_some() && template.compare_to.is_some() {
                color::color_to_string(&template.colors_to_compare.as_ref().unwrap(), &template.compare_to.as_ref().unwrap());
            }

            if !input_path_absolute.exists() {
                warn!("<d>The <yellow><b>{}</><d> template in <u>{}</><d> doesnt exist, skipping...</>", name, input_path_absolute.display());
                continue;
            }

            let data = read_to_string(&input_path_absolute)
                .wrap_err(format!("Could not read the {} template.", name))
                .suggestion("Try converting the file to use UTF-8 encoding.")?;

            engine.add_template(name, data).map_err(|error| {
                let message = format!(
                    "[{} - {}]\n{:#}",
                    name,
                    input_path_absolute.display(),
                    error
                );
                Report::new(error).wrap_err(message)
            })?;

            debug!(
                "Trying to write the {} template to {}",
                name,
                output_path_absolute.display()
            );

            let parent_folder = &output_path_absolute
                .parent()
                .wrap_err("Could not get the parent of the output path.")?;

            if !parent_folder.exists() {
                error!(
                    "The <b><yellow>{}</> folder doesnt exist, trying to create...",
                    &parent_folder.display()
                );
                debug!("{}", parent_folder.display());
                let _ = create_dir_all(parent_folder).wrap_err(format!(
                    "Failed to create the {} folders.",
                    &output_path_absolute.display()
                ));
            }

            let data = engine
                .template(name)
                .render(&render_data)
                .to_string()
                .map_err(|error| {
                    let message = format!(
                        "[{} - {}]\n{:#}",
                        name,
                        input_path_absolute.display(),
                        &error
                    );

                    Report::new(error).wrap_err(message)
                })?;

            let out = if path_prefix.is_some() && !cfg!(windows) {
                let prefix_path = PathBuf::from(path_prefix.as_ref().unwrap());
                rebase(output_path_absolute.as_ref(), &prefix_path, None).expect("failed to rebase output path")
            } else {
                output_path_absolute.to_path_buf()
            };

            debug!("out: {:?}", out);

            let mut output_file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true).open(out)?;

            if output_file.metadata()?.permissions().readonly() {
                error!(
                    "The <b><red>{}</> file is Read-Only",
                    &output_path_absolute.display()
                );
            }

            output_file.write_all(data.as_bytes())?;
            success!(
                "[{}/{}] Exported the <b><green>{}</> template to <d><u>{}</>",
                i + 1,
                &templates.len(),
                name,
                output_path_absolute.display()
            );
        }
        Ok(())
    }
}

fn generate_colors(
    schemes: &Schemes,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
) -> Result<HashMap<String, ColorVariants>, Report> {
    let mut hashmap: HashMap<String, ColorVariants> = Default::default();
    for (field, _color) in &schemes.dark {
        hashmap.insert(
            field.to_string(),
            generate_single_color(field, schemes, source_color, default_scheme)?,
        );
    }
    hashmap.insert(
        String::from("source_color"),
        generate_single_color("source_color", schemes, source_color, default_scheme)?,
    );
    Ok(hashmap)
}

fn generate_single_color(
    field: &str,
    schemes: &Schemes,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
) -> Result<ColorVariants, Report> {
    let scheme = match default_scheme {
        SchemesEnum::Light => &schemes.light,
        SchemesEnum::Dark => &schemes.dark,
    };

    if field == "source_color" {
        return Ok(ColorVariants {
            default: generate_color_strings(*source_color),
            light: generate_color_strings(*source_color),
            dark: generate_color_strings(*source_color),
        });
    }

    Ok(ColorVariants {
        default: generate_color_strings(scheme[field]),
        light: generate_color_strings(schemes.light[field]),
        dark: generate_color_strings(schemes.dark[field]),
    })
}

fn generate_color_strings(color: Argb) -> Colora {
    let base_color = rgb_from_argb(color);
    let hsl_color = Hsl::from(&base_color);
    Colora {
        hex: format_hex(&base_color),
        hex_stripped: format_hex_stripped(&base_color),
        rgb: format_rgb(&base_color),
        rgba: format_rgba(&base_color),
        hsl: format_hsl(&hsl_color),
        hsla: format_hsla(&hsl_color),
        red: format!("{:?}", base_color.red() as u8),
        green: format!("{:?}", base_color.green() as u8),
        blue: format!("{:?}", base_color.blue() as u8),
        alpha: format!("{:?}", base_color.alpha() as u8),
        hue: format!("{:?}", &hsl_color.hue()),
        lightness: format!("{:?}", &hsl_color.lightness()),
        saturation: format!("{:?}", &hsl_color.saturation()),
    }
}
