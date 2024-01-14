use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::WrapErr;
use color_eyre::Help;
use color_eyre::{eyre::Result, Report};

use colorsys::{Hsl, Rgb};
use serde::{Deserialize, Serialize};

use std::str;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::util::arguments::Source;
use crate::util::config::CustomKeyword;

use material_color_utilities_rs::util::color::format_argb_as_rgb;
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

#[derive(Serialize, Deserialize, Debug)]
struct Colors {
    primary: ColorVariants,
    primary_fixed: ColorVariants,
    primary_fixed_dim: ColorVariants,
    on_primary: ColorVariants,
    on_primary_fixed: ColorVariants,
    on_primary_fixed_variant: ColorVariants,
    primary_container: ColorVariants,
    on_primary_container: ColorVariants,
    secondary: ColorVariants,
    secondary_fixed: ColorVariants,
    secondary_fixed_dim: ColorVariants,
    on_secondary: ColorVariants,
    on_secondary_fixed: ColorVariants,
    on_secondary_fixed_variant: ColorVariants,
    secondary_container: ColorVariants,
    on_secondary_container: ColorVariants,
    tertiary: ColorVariants,
    tertiary_fixed: ColorVariants,
    tertiary_fixed_dim: ColorVariants,
    on_tertiary: ColorVariants,
    on_tertiary_fixed: ColorVariants,
    on_tertiary_fixed_variant: ColorVariants,
    tertiary_container: ColorVariants,
    on_tertiary_container: ColorVariants,
    error: ColorVariants,
    on_error: ColorVariants,
    error_container: ColorVariants,
    on_error_container: ColorVariants,
    surface: ColorVariants,
    on_surface: ColorVariants,
    on_surface_variant: ColorVariants,
    outline: ColorVariants,
    outline_variant: ColorVariants,
    shadow: ColorVariants,
    scrim: ColorVariants,
    inverse_surface: ColorVariants,
    inverse_on_surface: ColorVariants,
    inverse_primary: ColorVariants,
    surface_dim: ColorVariants,
    surface_bright: ColorVariants,
    surface_container_lowest: ColorVariants,
    surface_container_low: ColorVariants,
    surface_container: ColorVariants,
    surface_container_high: ColorVariants,
    surface_container_highest: ColorVariants,
    source_color: ColorVariants,
}

use super::color::Color;

impl Template {
    pub fn generate(
        schemes: &Schemes,
        templates: &HashMap<String, Template>,
        source: &Source,
        prefix: &Option<String>,
        source_color: &[u8; 4],
        default_scheme: &SchemesEnum,
        custom_keywords: &Option<HashMap<String, CustomKeyword>>,
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

        let colors: Colors = generate_colors(schemes, source_color, default_scheme)?;

        let mut custom: HashMap<String, String> = Default::default();

        for entry in custom_keywords.iter() {
            for (_name, values) in entry {
                custom.insert(values.find.to_string(), values.replace.to_string());
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
) -> Result<Colors, Report> {
    Ok(Colors {
        primary: generate_single_color("primary", &schemes, source_color, default_scheme)?,
        primary_fixed: generate_single_color(
            "primary_fixed",
            &schemes,
            source_color,
            default_scheme,
        )?,
        primary_fixed_dim: generate_single_color(
            "primary_fixed_dim",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_primary: generate_single_color("on_primary", &schemes, source_color, default_scheme)?,
        on_primary_fixed: generate_single_color(
            "on_primary_fixed",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_primary_fixed_variant: generate_single_color(
            "on_primary_fixed_variant",
            &schemes,
            source_color,
            default_scheme,
        )?,
        primary_container: generate_single_color(
            "primary_container",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_primary_container: generate_single_color(
            "on_primary_container",
            &schemes,
            source_color,
            default_scheme,
        )?,
        secondary: generate_single_color("secondary", &schemes, source_color, default_scheme)?,
        secondary_fixed: generate_single_color(
            "secondary_fixed",
            &schemes,
            source_color,
            default_scheme,
        )?,
        secondary_fixed_dim: generate_single_color(
            "secondary_fixed_dim",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_secondary: generate_single_color(
            "on_secondary",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_secondary_fixed: generate_single_color(
            "on_secondary_fixed",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_secondary_fixed_variant: generate_single_color(
            "on_secondary_fixed_variant",
            &schemes,
            source_color,
            default_scheme,
        )?,
        secondary_container: generate_single_color(
            "secondary_container",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_secondary_container: generate_single_color(
            "on_secondary_container",
            &schemes,
            source_color,
            default_scheme,
        )?,
        tertiary: generate_single_color("tertiary", &schemes, source_color, default_scheme)?,
        tertiary_fixed: generate_single_color(
            "tertiary_fixed",
            &schemes,
            source_color,
            default_scheme,
        )?,
        tertiary_fixed_dim: generate_single_color(
            "tertiary_fixed_dim",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_tertiary: generate_single_color("on_tertiary", &schemes, source_color, default_scheme)?,
        on_tertiary_fixed: generate_single_color(
            "on_tertiary_fixed",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_tertiary_fixed_variant: generate_single_color(
            "on_tertiary_fixed_variant",
            &schemes,
            source_color,
            default_scheme,
        )?,
        tertiary_container: generate_single_color(
            "tertiary_container",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_tertiary_container: generate_single_color(
            "on_tertiary_container",
            &schemes,
            source_color,
            default_scheme,
        )?,
        error: generate_single_color("error", &schemes, source_color, default_scheme)?,
        on_error: generate_single_color("on_error", &schemes, source_color, default_scheme)?,
        error_container: generate_single_color(
            "error_container",
            &schemes,
            source_color,
            default_scheme,
        )?,
        on_error_container: generate_single_color(
            "on_error_container",
            &schemes,
            source_color,
            default_scheme,
        )?,
        surface: generate_single_color("surface", &schemes, source_color, default_scheme)?,
        on_surface: generate_single_color("on_surface", &schemes, source_color, default_scheme)?,
        on_surface_variant: generate_single_color(
            "on_surface_variant",
            &schemes,
            source_color,
            default_scheme,
        )?,
        outline: generate_single_color("outline", &schemes, source_color, default_scheme)?,
        outline_variant: generate_single_color(
            "outline_variant",
            &schemes,
            source_color,
            default_scheme,
        )?,
        shadow: generate_single_color("shadow", &schemes, source_color, default_scheme)?,
        scrim: generate_single_color("scrim", &schemes, source_color, default_scheme)?,
        inverse_surface: generate_single_color(
            "inverse_surface",
            &schemes,
            source_color,
            default_scheme,
        )?,
        inverse_on_surface: generate_single_color(
            "inverse_on_surface",
            &schemes,
            source_color,
            default_scheme,
        )?,
        inverse_primary: generate_single_color(
            "inverse_primary",
            &schemes,
            source_color,
            default_scheme,
        )?,
        surface_dim: generate_single_color("surface_dim", &schemes, source_color, default_scheme)?,
        surface_bright: generate_single_color(
            "surface_bright",
            &schemes,
            source_color,
            default_scheme,
        )?,
        surface_container_lowest: generate_single_color(
            "surface_container_lowest",
            &schemes,
            source_color,
            default_scheme,
        )?,
        surface_container_low: generate_single_color(
            "surface_container_low",
            &schemes,
            source_color,
            default_scheme,
        )?,
        surface_container: generate_single_color(
            "surface_container",
            &schemes,
            source_color,
            default_scheme,
        )?,
        surface_container_high: generate_single_color(
            "surface_container_high",
            &schemes,
            source_color,
            default_scheme,
        )?,
        surface_container_highest: generate_single_color(
            "surface_container_highest",
            &schemes,
            source_color,
            default_scheme,
        )?,
        source_color: generate_single_color(
            "source_color",
            &schemes,
            source_color,
            default_scheme,
        )?,
    })
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
            default: generate_color_strings(Color::new(*source_color)),
            light: generate_color_strings(Color::new(*source_color)),
            dark: generate_color_strings(Color::new(*source_color)),
        });
    }

    let color_default: Color = Color::new(scheme[field]);

    let color_light: Color = Color::new(schemes.light[field]);

    let color_dark: Color = Color::new(schemes.dark[field]);

    Ok(ColorVariants {
        default: generate_color_strings(color_default),
        light: generate_color_strings(color_light),
        dark: generate_color_strings(color_dark),
    })
}

fn generate_color_strings(color: Color) -> Colora {
    let base_color = Rgb::from((
        color.red as f64,
        color.green as f64,
        color.blue as f64,
        color.alpha as f64,
    ));
    let hsl_color = Hsl::from(&base_color);
    Colora {
        hex: format_argb_as_rgb([color.alpha, color.red, color.green, color.blue]),
        hex_stripped: format_argb_as_rgb([color.alpha, color.red, color.green, color.blue])[1..]
            .to_string(),
        rgb: format!("rgb({:?}, {:?}, {:?})", color.red, color.green, color.blue),
        rgba: format!(
            "rgba({:?}, {:?}, {:?}, {:?})",
            color.red, color.green, color.blue, color.alpha
        ),
        hsl: format!(
            "hsl({:?}, {:?}, {:?})",
            hsl_color.hue(),
            hsl_color.saturation(),
            hsl_color.lightness(),
        ),
        hsla: hsl_color.to_css_string(),
        red: format!("{:?}", color.red),
        green: format!("{:?}", color.green),
        blue: format!("{:?}", color.blue),
        alpha: format!("{:?}", color.alpha),
        hue: format!("{:?}", &hsl_color.hue()),
        lightness: format!("{:?}", &hsl_color.lightness()),
        saturation: format!("{:?}", &hsl_color.saturation()),
    }
}
