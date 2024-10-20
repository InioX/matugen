use std::collections::HashMap;

use color_eyre::{eyre::Result, Report};
use colorsys::{ColorAlpha, Hsl};
use material_colors::color::Argb;
use upon::{Engine, Value};

use crate::color::format::{
    format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba,
    rgb_from_argb,
};
use crate::filters::alpha::set_alpha;
use crate::filters::camel::camel_case;
use crate::filters::grayscale::grayscale;
use crate::filters::hue::set_hue;
use crate::filters::invert::invert;
use crate::filters::lightness::set_lightness;
use crate::scheme::{Schemes, SchemesEnum};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Color {
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ColorVariants {
    pub light: Color,
    pub dark: Color,
    pub default: Color,
}

pub fn add_engine_filters(engine: &mut Engine) {
    // Color manipulation
    engine.add_filter("set_lightness", set_lightness);
    engine.add_filter("set_alpha", set_alpha);
    engine.add_filter("set_hue", set_hue);
    engine.add_filter("grayscale", grayscale);
    engine.add_filter("invert", invert);

    // String manipulation
    engine.add_filter("to_upper", str::to_uppercase);
    engine.add_filter("to_lower", str::to_lowercase);
    engine.add_filter("replace", |s: String, from: String, to: String| {
        s.replace(&from, &to)
    });
    engine.add_filter("camel_case", camel_case);
}

pub fn render_template(
    engine: &Engine,
    name: &String,
    render_data: &Value,
    path: Option<&str>,
) -> Result<String, Report> {
    let data = engine
        .template(name)
        .render(render_data)
        .to_string()
        .map_err(|error| {
            let message = format!(
                "[{} - {}]\n{:#}",
                name,
                path.unwrap_or(&"".to_string()),
                &error
            );

            Report::new(error).wrap_err(message)
        })?;
    Ok(data)
}

pub fn get_render_data(
    schemes: &Schemes,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
    custom_keywords: &Option<HashMap<String, String>>,
    image: Option<&String>,
) -> Result<Value, Report> {
    let colors = generate_colors(schemes, source_color, default_scheme)?;
    let mut custom: HashMap<String, String> = Default::default();
    for entry in custom_keywords.iter() {
        for (name, value) in entry {
            custom.insert(name.to_string(), value.to_string());
        }
    }
    Ok(upon::value! {
        colors: &colors, image: image, custom: &custom,
    })
}

pub fn generate_colors(
    schemes: &Schemes,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
) -> Result<HashMap<String, ColorVariants>, Report> {
    let mut hashmap: HashMap<String, ColorVariants> = Default::default();
    for ((field, color_light), (_, color_dark)) in std::iter::zip(&schemes.light, &schemes.dark) {
        hashmap.insert(
            field.to_string(),
            generate_single_color(
                field,
                source_color,
                default_scheme,
                *color_light,
                *color_dark,
            )?,
        );
    }
    hashmap.insert(
        String::from("source_color"),
        generate_single_color(
            "source_color",
            source_color,
            default_scheme,
            *source_color,
            *source_color,
        )?,
    );
    Ok(hashmap)
}

pub fn generate_single_color(
    field: &str,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
    color_light: Argb,
    color_dark: Argb,
) -> Result<ColorVariants, Report> {
    let default_scheme_color = match default_scheme {
        SchemesEnum::Light => color_light,
        SchemesEnum::Dark => color_dark,
    };

    if field == "source_color" {
        return Ok(ColorVariants {
            default: generate_color_strings(*source_color),
            light: generate_color_strings(*source_color),
            dark: generate_color_strings(*source_color),
        });
    }

    Ok(ColorVariants {
        default: generate_color_strings(default_scheme_color),
        light: generate_color_strings(color_light),
        dark: generate_color_strings(color_dark),
    })
}

fn generate_color_strings(color: Argb) -> Color {
    let base_color = rgb_from_argb(color);
    let hsl_color = Hsl::from(&base_color);
    Color {
        hex: format_hex(&base_color),
        hex_stripped: format_hex_stripped(&base_color),
        rgb: format_rgb(&base_color),
        rgba: format_rgba(&base_color, true),
        hsl: format_hsl(&hsl_color),
        hsla: format_hsla(&hsl_color, true),
        red: format!("{:?}", base_color.red() as u8),
        green: format!("{:?}", base_color.green() as u8),
        blue: format!("{:?}", base_color.blue() as u8),
        alpha: format!("{:?}", base_color.alpha() as u8),
        hue: format!("{:?}", &hsl_color.hue()),
        lightness: format!("{:?}", &hsl_color.lightness()),
        saturation: format!("{:?}", &hsl_color.saturation()),
    }
}
