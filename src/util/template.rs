use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::WrapErr;
use color_eyre::Help;
use color_eyre::{eyre::Result, Report};


use colorsys::{Hsl,Rgb};
use serde::{Deserialize, Serialize};

use std::str;

use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::util::arguments::Source;
use crate::util::color::SchemeExt;
use crate::Scheme;
use crate::util::config::CustomKeyword;

use crate::util::color::SchemeAndroidExt;
use crate::SchemeAndroid;

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
    pub amoled: Colora,
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
    color_accent_primary: ColorVariants,
    color_accent_primary_variant: ColorVariants,
    color_accent_secondary: ColorVariants,
    color_accent_secondary_variant: ColorVariants,
    color_accent_tertiary: ColorVariants,
    color_accent_tertiary_variant: ColorVariants,
    text_color_primary: ColorVariants,
    text_color_secondary: ColorVariants,
    text_color_tertiary: ColorVariants,
    text_color_primary_inverse: ColorVariants,
    text_color_secondary_inverse: ColorVariants,
    text_color_tertiary_inverse: ColorVariants,
    color_background: ColorVariants,
    color_background_floating: ColorVariants,
    color_surface: ColorVariants,
    color_surface_variant: ColorVariants,
    color_surface_highlight: ColorVariants,
    surface_header: ColorVariants,
    under_surface: ColorVariants,
    off_state: ColorVariants,
    accent_surface: ColorVariants,
    text_primary_on_accent: ColorVariants,
    text_secondary_on_accent: ColorVariants,
    volume_background: ColorVariants,
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

        let syntax = Syntax::builder().expr("{{", "}}").block("<[", "]>").build();
        let mut engine = Engine::with_syntax(syntax);

        let image = match &source {
            Source::Image { path } => Some(path),
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

            engine.add_template(name, data)?;

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

            let data = engine.template(name).render(upon::value!{ colors: &colors, image: image, custom: &custom, }).to_string().map_err(|error| {
                let message = format!("{:#}", error);
                Report::new(error).wrap_err(message)
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
    default_scheme:  &SchemesEnum,
) -> Result<Colors, Report> {
    Ok(Colors {
        primary: generate_single_color("primary", &schemes, source_color, default_scheme, false)?,
        primary_fixed: generate_single_color("primary_fixed", &schemes, source_color, default_scheme, false)?,
        primary_fixed_dim: generate_single_color("primary_fixed_dim", &schemes, source_color, default_scheme, false)?,
        on_primary: generate_single_color("on_primary", &schemes, source_color, default_scheme, false)?,
        on_primary_fixed: generate_single_color("on_primary_fixed", &schemes, source_color, default_scheme, false)?,
        on_primary_fixed_variant: generate_single_color("on_primary_fixed_variant", &schemes, source_color, default_scheme, false)?,
        primary_container: generate_single_color("primary_container", &schemes, source_color, default_scheme, false)?,
        on_primary_container: generate_single_color("on_primary_container", &schemes, source_color, default_scheme, false)?,
        secondary: generate_single_color("secondary", &schemes, source_color, default_scheme, false)?,
        secondary_fixed: generate_single_color("secondary_fixed", &schemes, source_color, default_scheme, false)?,
        secondary_fixed_dim: generate_single_color("secondary_fixed_dim", &schemes, source_color, default_scheme, false)?,
        on_secondary: generate_single_color("on_secondary", &schemes, source_color, default_scheme, false)?,
        on_secondary_fixed: generate_single_color("on_secondary_fixed", &schemes, source_color, default_scheme, false)?,
        on_secondary_fixed_variant: generate_single_color("on_secondary_fixed_variant", &schemes, source_color, default_scheme, false)?,
        secondary_container: generate_single_color("secondary_container", &schemes, source_color, default_scheme, false)?,
        on_secondary_container: generate_single_color("on_secondary_container", &schemes, source_color, default_scheme, false)?,
        tertiary: generate_single_color("tertiary", &schemes, source_color, default_scheme, false)?,
        tertiary_fixed: generate_single_color("tertiary_fixed", &schemes, source_color, default_scheme, false)?,
        tertiary_fixed_dim: generate_single_color("tertiary_fixed_dim", &schemes, source_color, default_scheme, false)?,
        on_tertiary: generate_single_color("on_tertiary", &schemes, source_color, default_scheme, false)?,
        on_tertiary_fixed: generate_single_color("on_tertiary_fixed", &schemes, source_color, default_scheme, false)?,
        on_tertiary_fixed_variant: generate_single_color("on_tertiary_fixed_variant", &schemes, source_color, default_scheme, false)?,
        tertiary_container: generate_single_color("tertiary_container", &schemes, source_color, default_scheme, false)?,
        on_tertiary_container: generate_single_color("on_tertiary_container", &schemes, source_color, default_scheme, false)?,
        error: generate_single_color("error", &schemes, source_color, default_scheme, false)?,
        on_error: generate_single_color("on_error", &schemes, source_color, default_scheme, false)?,
        error_container: generate_single_color("error_container", &schemes, source_color, default_scheme, false)?,
        on_error_container: generate_single_color("on_error_container", &schemes, source_color, default_scheme, false)?,
        surface: generate_single_color("surface", &schemes, source_color, default_scheme, false)?,
        on_surface: generate_single_color("on_surface", &schemes, source_color, default_scheme, false)?,
        on_surface_variant: generate_single_color("on_surface_variant", &schemes, source_color, default_scheme, false)?,
        outline: generate_single_color("outline", &schemes, source_color, default_scheme, false)?,
        outline_variant: generate_single_color("outline_variant", &schemes, source_color, default_scheme, false)?,
        shadow: generate_single_color("shadow", &schemes, source_color, default_scheme, false)?,
        scrim: generate_single_color("scrim", &schemes, source_color, default_scheme, false)?,
        inverse_surface: generate_single_color("inverse_surface", &schemes, source_color, default_scheme, false)?,
        inverse_on_surface: generate_single_color("inverse_on_surface", &schemes, source_color, default_scheme, false)?,
        inverse_primary: generate_single_color("inverse_primary", &schemes, source_color, default_scheme, false)?,
        surface_dim: generate_single_color("surface_dim", &schemes, source_color, default_scheme, false)?,
        surface_bright: generate_single_color("surface_bright", &schemes, source_color, default_scheme, false)?,
        surface_container_lowest: generate_single_color("surface_container_lowest", &schemes, source_color, default_scheme, false)?,
        surface_container_low: generate_single_color("surface_container_low", &schemes, source_color, default_scheme, false)?,
        surface_container: generate_single_color("surface_container", &schemes, source_color, default_scheme, false)?,
        surface_container_high: generate_single_color("surface_container_high", &schemes, source_color, default_scheme, false)?,
        surface_container_highest: generate_single_color("surface_container_highest", &schemes, source_color, default_scheme, false)?,
        
        color_accent_primary: generate_single_color("color_accent_primary", &schemes, source_color, default_scheme, true)?,
        color_accent_primary_variant: generate_single_color("color_accent_primary_variant", &schemes, source_color, default_scheme, true)?,
        color_accent_secondary: generate_single_color("color_accent_secondary", &schemes, source_color, default_scheme, true)?,
        color_accent_secondary_variant: generate_single_color("color_accent_secondary_variant", &schemes, source_color, default_scheme, true)?,
        color_accent_tertiary: generate_single_color("color_accent_tertiary", &schemes, source_color, default_scheme, true)?,
        color_accent_tertiary_variant: generate_single_color("color_accent_tertiary_variant", &schemes, source_color, default_scheme, true)?,
        text_color_primary: generate_single_color("text_color_primary", &schemes, source_color, default_scheme, true)?,
        text_color_secondary: generate_single_color("text_color_secondary", &schemes, source_color, default_scheme, true)?,
        text_color_tertiary: generate_single_color("text_color_tertiary", &schemes, source_color, default_scheme, true)?,
        text_color_primary_inverse: generate_single_color("text_color_primary_inverse", &schemes, source_color, default_scheme, true)?,
        text_color_secondary_inverse: generate_single_color("text_color_secondary_inverse", &schemes, source_color, default_scheme, true)?,
        text_color_tertiary_inverse: generate_single_color("text_color_tertiary_inverse", &schemes, source_color, default_scheme, true)?,
        color_background: generate_single_color("color_background", &schemes, source_color, default_scheme, true)?,
        color_background_floating: generate_single_color("color_background_floating", &schemes, source_color, default_scheme, true)?,
        color_surface: generate_single_color("color_surface", &schemes, source_color, default_scheme, true)?,
        color_surface_variant: generate_single_color("color_surface_variant", &schemes, source_color, default_scheme, true)?,
        color_surface_highlight: generate_single_color("color_surface_highlight", &schemes, source_color, default_scheme, true)?,
        surface_header: generate_single_color("surface_header", &schemes, source_color, default_scheme, true)?,
        under_surface: generate_single_color("under_surface", &schemes, source_color, default_scheme, true)?,
        off_state: generate_single_color("off_state", &schemes, source_color, default_scheme, true)?,
        accent_surface: generate_single_color("accent_surface", &schemes, source_color, default_scheme, true)?,
        text_primary_on_accent: generate_single_color("text_primary_on_accent", &schemes, source_color, default_scheme, true)?,
        text_secondary_on_accent: generate_single_color("text_secondary_on_accent", &schemes, source_color, default_scheme, true)?,
        volume_background: generate_single_color("volume_background", &schemes, source_color, default_scheme, true)?,
    })  
}

fn generate_single_color(
    field: &str,
    schemes: &Schemes,
    source_color: &[u8; 4],
    default_scheme:  &SchemesEnum,
    is_android: bool,
) -> Result<ColorVariants, Report> {


    let color_default: Color = match is_android {
        true => {
            let scheme = match default_scheme {
                SchemesEnum::Light => &schemes.light_android,
                SchemesEnum::Dark => &schemes.dark_android,
                SchemesEnum::Amoled => &schemes.amoled_android,
            };
            Color::new(*SchemeAndroid::get_value(scheme, field, source_color))
        },
        false => {
            let scheme = match default_scheme {
                SchemesEnum::Light => &schemes.light,
                SchemesEnum::Dark => &schemes.dark,
                SchemesEnum::Amoled => &schemes.amoled,
            };
            Color::new(*Scheme::get_value(scheme, field, source_color))
        },
    };

    let color_light: Color = match is_android {
        true => Color::new(*SchemeAndroid::get_value(&schemes.light_android, field, source_color)),
        false => Color::new(*Scheme::get_value(&schemes.light, field, source_color)),
    };

    let color_dark: Color = match is_android {
        true => Color::new(*SchemeAndroid::get_value(&schemes.dark_android, field, source_color)),
        false => Color::new(*Scheme::get_value(&schemes.light, field, source_color)),
    };

    let color_amoled: Color = match is_android {
        true => Color::new(*SchemeAndroid::get_value(&schemes.amoled_android, field, source_color)),
        false => Color::new(*Scheme::get_value(&schemes.light, field, source_color)),
    };

    Ok ( ColorVariants {
        default: generate_color_strings(color_default),
        light: generate_color_strings(color_light),
        dark: generate_color_strings(color_dark),
        amoled: generate_color_strings(color_amoled),
    })
}

fn generate_color_strings(color: Color) -> Colora {
    let base_color = Rgb::from((color.red as f64, color.green as f64, color.blue as f64,color.alpha as f64));
    let hsl_color = Hsl::from(&base_color);
    Colora {
        hex: format_argb_as_rgb([
            color.alpha,
            color.red,
            color.green,
            color.blue,
        ]),
        hex_stripped: format_argb_as_rgb([
            color.alpha,
            color.red,
            color.green,
            color.blue,
        ])[1..]
            .to_string(),
        rgb: format!(
            "rgb({:?}, {:?}, {:?})",
            color.red, color.green, color.blue
        ),
        rgba: format!(
            "rgba({:?}, {:?}, {:?}, {:?})",
            color.red, color.green, color.blue, color.alpha
        ),
        hsl: format!(
            "hsl({:?}, {:?}, {:?})",
            hsl_color.hue(), hsl_color.saturation(), hsl_color.lightness(),
        ),
        hsla: hsl_color.to_css_string(),
        red: format!(
            "{:?}",
            color.red
        ),
        green: format!(
            "{:?}",
            color.green
        ),
        blue: format!(
            "{:?}",
            color.blue
        ),
        alpha: format!(
            "{:?}",
            color.alpha
        ),
        hue: format!(
            "{:?}",
            &hsl_color.hue()
        ),
        lightness: format!(
            "{:?}",
            &hsl_color.lightness()
        ),
        saturation: format!(
            "{:?}",
            &hsl_color.saturation()
        ),
    }
}
