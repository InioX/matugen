use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::WrapErr;
use color_eyre::Help;
use color_eyre::{eyre::Result, Report};

use colorsys::{ColorAlpha, Hsl};
use serde::{Deserialize, Serialize};

use std::str;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::util::arguments::Source;
use resolve_path::PathResolveExt;

use crate::{Schemes, SchemesEnum};

use upon::{Engine, Syntax};

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub mode: Option<SchemesEnum>,
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

impl Template {
    pub fn generate(
        schemes: &Schemes,
        templates: &HashMap<String, Template>,
        source: &Source,
        prefix: &Option<String>,
        source_color: &[u8; 4],
        default_scheme: &SchemesEnum,
        custom_keywords: &Option<HashMap<String, String>>,
    ) -> Result<(), Report> {
        let default_prefix = "@".to_string();

        let _prefix: &String = match &prefix {
            Some(prefix) => prefix,
            None => &default_prefix,
        };

        info!("Loaded <b><cyan>{}</> templates.", &templates.len());

        let syntax = Syntax::builder().expr("{{", "}}").block("<*", "*>").build();
        let mut engine = Engine::with_syntax(syntax);

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

        for (i, (name, template)) in templates.iter().enumerate() {
            let input_path_absolute = template.input_path.try_resolve()?;
            let output_path_absolute = template.output_path.try_resolve()?;

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
                Report::new(error)
                    .wrap_err(message)
                    .suggestion("Make sure you closed the {{ opening  properly.")
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
                .render(upon::value! { colors: &colors, image: image, custom: &custom, })
                .to_string()
                .map_err(|error| {
                    let message = format!(
                        "[{} - {}]\n{:#}",
                        name,
                        input_path_absolute.display(),
                        error
                    );
                    Report::new(error).wrap_err(message).note(
                        r#"The following colors have been removed:
    - color_accent_primary
    - color_accent_primary_variant
    - color_accent_secondary
    - color_accent_secondary_variant
    - color_accent_tertiary
    - color_accent_tertiary_variant
    - text_color_primary
    - text_color_secondary:
    - text_color_tertiary
    - text_color_primary_inverse
    - text_color_secondary_inverse
    - text_color_tertiary_inverse
    - color_background
    - color_background_floating
    - color_surface
    - color_surface_variant
    - color_surface_highlight
    - surface_header
    - under_surface
    - off_state
    - accent_surface
    - text_primary_on_accent
    - text_secondary_on_accent
    - volume_background
                    "#,
                    )
                })?;

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
    source_color: &[u8; 4],
    default_scheme: &SchemesEnum,
) -> Result<HashMap<String, ColorVariants>, Report> {
    let mut hashmap: HashMap<String, ColorVariants> = Default::default();
    for (field, _color) in &schemes.dark {
        hashmap.insert(
            field.to_string(),
            generate_single_color(field, &schemes, source_color, default_scheme)?,
        );
    }
    hashmap.insert(
        String::from("source_color"),
        generate_single_color("source_color", &schemes, source_color, default_scheme)?,
    );
    Ok(hashmap)
}

fn generate_single_color(
    field: &str,
    schemes: &Schemes,
    source_color: &[u8; 4],
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

fn generate_color_strings(color: [u8; 4]) -> Colora {
    let base_color = rgb_from_argb(color);
    let hsl_color = Hsl::from(&base_color);
    Colora {
        hex: base_color.to_hex_string(),
        hex_stripped: base_color.to_hex_string()[1..].to_string(),
        rgb: format!(
            "rgb({:?}, {:?}, {:?})",
            base_color.red(),
            base_color.green(),
            base_color.blue()
        ),
        rgba: format!(
            "rgba({:?}, {:?}, {:?}, {:?})",
            base_color.red(),
            base_color.green(),
            base_color.blue(),
            base_color.alpha()
        ),
        hsl: format!(
            "hsl({:?}, {:?}, {:?})",
            hsl_color.hue(),
            hsl_color.saturation(),
            hsl_color.lightness(),
        ),
        hsla: hsl_color.to_css_string(),
        red: format!("{:?}", base_color.red()),
        green: format!("{:?}", base_color.green()),
        blue: format!("{:?}", base_color.blue()),
        alpha: format!("{:?}", base_color.alpha()),
        hue: format!("{:?}", &hsl_color.hue()),
        lightness: format!("{:?}", &hsl_color.lightness()),
        saturation: format!("{:?}", &hsl_color.saturation()),
    }
}
