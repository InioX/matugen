use std::collections::HashMap;

use color_eyre::{eyre::Result, Report};
use colorsys::{ColorAlpha, Hsl, Rgb};
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
use crate::filters::lightness::{auto_lightness, set_lightness};
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

#[derive(Debug)]
pub struct ColorBase {
    rgba: Rgb,
    hsla: Hsl,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ColorVariants {
    pub light: Color,
    pub dark: Color,
    pub default: Color,
}

#[derive(Debug)]
pub struct ColorVariantss {
    pub light: ColorBase,
    pub dark: ColorBase,
}

pub fn add_engine_filters(engine: &mut Engine) {
    // Color manipulation
    engine.add_filter("set_lightness", set_lightness);
    engine.add_filter("auto_lightness", auto_lightness);
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

fn renderer(
    a: &[upon::ValueMember<'_>],
    color_map: &HashMap<String, ColorVariantss>,
) -> Result<Value, String> {
    let first_word = &a[0].access;

    match first_word {
        upon::ValueAccess::Index(i) => {
            println!("index")
        }
        upon::ValueAccess::Key(k) => match *k {
            "colors" => renderer_colors(a, color_map, k),
            "custom" => {}
            _ => {}
        },
    }

    println!("\n--------\n");
    Ok(Value::from("a"))
}

// TODO: Maybe change to indexmap
fn find_color_in_hasmap(
    a: &upon::ValueAccess,
    hashmap: &HashMap<String, ColorVariantss>,
) {
    // TODO: add searchup by index and by key
    match a {
        upon::ValueAccess::Index(i) => {},
        upon::ValueAccess::Key(k) => match *k {
            _ => println!("a"),
        },
    }
}

fn renderer_colors(
    a: &[upon::ValueMember<'_>],
    color_map: &HashMap<String, ColorVariantss>,
    key: &&str,
) {
    // TODO: Check if array isnt out of bound
    let color_name = a[1].access;
    let color_scheme = a[2].access;
    let color_format = a[3].access;
    let bleh = find_color_in_hasmap(&color_name, &color_map);
    // let color = color_map.get(&key.to_string());
    // println!("color: {:?}", color);

    for (i, key) in a.into_iter().skip(1).enumerate() {
        println!("[{}]: {:#?}", i, key);
    }
}

pub fn render_template(
    engine: &Engine,
    name: &String,
    render_data: &Value,
    path: Option<&str>,
    schemes: &Schemes,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
) -> Result<String, Report> {
    let color_map = generate_colors_2(schemes, source_color, default_scheme)?;
    let data1 = engine
        .template(name)
        .render_from_fn(|a: &[upon::ValueMember<'_>]| (renderer(a, &color_map)))
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
        colors: &colors, image: image, custom: &custom, mode: default_scheme,
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

pub fn generate_colors_2(
    schemes: &Schemes,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
) -> Result<HashMap<String, ColorVariantss>, Report> {
    let mut hashmap: HashMap<String, ColorVariantss> = Default::default();
    for ((field, color_light), (_, color_dark)) in std::iter::zip(&schemes.light, &schemes.dark) {
        hashmap.insert(
            field.to_string(),
            generate_single_color2(
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
        generate_single_color2(
            "source_color",
            source_color,
            default_scheme,
            *source_color,
            *source_color,
        )?,
    );
    Ok(hashmap)
}

pub fn generate_single_color2(
    field: &str,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
    color_light: Argb,
    color_dark: Argb,
) -> Result<ColorVariantss, Report> {
    if field == "source_color" {
        return Ok(ColorVariantss {
            light: generate_color_bases(*source_color),
            dark: generate_color_bases(*source_color),
        });
    }

    Ok(ColorVariantss {
        light: generate_color_bases(color_light),
        dark: generate_color_bases(color_dark),
    })
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

fn generate_color_bases(color: Argb) -> ColorBase {
    let rgb_color = rgb_from_argb(color);
    let hsl_color = Hsl::from(&rgb_color);
    ColorBase {
        rgba: rgb_from_argb(color),
        hsla: hsl_color,
    }
}
