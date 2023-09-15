use material_color_utilities_rs::scheme::scheme_android::SchemeAndroid;
use material_color_utilities_rs::{scheme::scheme::Scheme, util::color::format_argb_as_rgb};
use owo_colors::{OwoColorize, Style};

use prettytable::{format, Cell, Row, Table};

use crate::Schemes;

use super::arguments::{ColorFormat, Format, Source};
use super::image::source_color_from_image;
use color_eyre::{eyre::Result, Report};
use colorsys::{ColorAlpha, Hsl, Rgb};
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;

pub const COLORS: [&str; 30] = [
    "source_color",
    "primary",
    "on_primary",
    "primary_container",
    "on_primary_container",
    "secondary",
    "on_secondary",
    "secondary_container",
    "on_secondary_container",
    "tertiary",
    "on_tertiary",
    "tertiary_container",
    "on_tertiary_container",
    "error",
    "on_error",
    "error_container",
    "on_error_container",
    "background",
    "on_background",
    "surface",
    "on_surface",
    "surface_variant",
    "on_surface_variant",
    "outline",
    "outline_variant",
    "shadow",
    "scrim",
    "inverse_surface",
    "inverse_on_surface",
    "inverse_primary",
];

pub const COLORS_ANDROID: [&str; 25] = [
    "color_accent_primary",
    "color_accent_primary_variant",
    "color_accent_secondary",
    "color_accent_secondary_variant",
    "color_accent_tertiary",
    "color_accent_tertiary_variant",
    "text_color_primary",
    "text_color_secondary",
    "text_color_tertiary",
    "text_color_primary_inverse",
    "text_color_secondary_inverse",
    "text_color_tertiary_inverse",
    "color_background",
    "color_background_floating",
    "color_surface",
    "color_surface_variant",
    "color_surface_highlight",
    "surface_header",
    "under_surface",
    "off_state",
    "accent_surface",
    "text_primary_on_accent",
    "text_secondary_on_accent",
    "volume_background",
    "scrim_android", // Should just be `scrim`, renamed so its not the same as `scrim` in `COLORS`
];

// TODO Fix this monstrosity
pub trait SchemeExt {
    fn get_value<'a>(&'a self, field: &str, source_color: &'a [u8; 4]) -> &[u8; 4];
}
impl SchemeExt for Scheme {
    fn get_value<'a>(&'a self, field: &str, source_color: &'a [u8; 4]) -> &[u8; 4] {
        match field {
            "primary" => &self.primary,
            "on_primary" => &self.on_primary,
            "primary_container" => &self.primary_container,
            "on_primary_container" => &self.on_primary_container,
            "secondary" => &self.secondary,
            "on_secondary" => &self.on_secondary,
            "secondary_container" => &self.secondary_container,
            "on_secondary_container" => &self.on_secondary_container,
            "tertiary" => &self.tertiary,
            "on_tertiary" => &self.on_tertiary,
            "tertiary_container" => &self.tertiary_container,
            "on_tertiary_container" => &self.on_tertiary_container,
            "error" => &self.error,
            "on_error" => &self.on_error,
            "error_container" => &self.error_container,
            "on_error_container" => &self.on_error_container,
            "background" => &self.background,
            "on_background" => &self.on_background,
            "surface" => &self.surface,
            "on_surface" => &self.on_surface,
            "surface_variant" => &self.surface_variant,
            "on_surface_variant" => &self.on_surface_variant,
            "outline" => &self.outline,
            "outline_variant" => &self.outline_variant,
            "shadow" => &self.shadow,
            "scrim" => &self.scrim,
            "inverse_surface" => &self.inverse_surface,
            "inverse_on_surface" => &self.inverse_on_surface,
            "inverse_primary" => &self.inverse_primary,
            "source_color" => &source_color,
            _ => panic!(),
        }
    }
}

pub trait SchemeAndroidExt {
    fn get_value<'a>(&'a self, field: &str, source_color: &'a [u8; 4]) -> &[u8; 4];
}
impl SchemeAndroidExt for SchemeAndroid {
    fn get_value<'a>(&'a self, field: &str, source_color: &'a [u8; 4]) -> &[u8; 4] {
        match field {
            "source_color" => &source_color,
            "color_accent_primary" => &self.color_accent_primary,
            "color_accent_primary_variant" => &self.color_accent_primary_variant,
            "color_accent_secondary" => &self.color_accent_secondary,
            "color_accent_secondary_variant" => &self.color_accent_secondary_variant,
            "color_accent_tertiary" => &self.color_accent_tertiary,
            "color_accent_tertiary_variant" => &self.color_accent_tertiary_variant,
            "text_color_primary" => &self.text_color_primary,
            "text_color_secondary" => &self.text_color_secondary,
            "text_color_tertiary" => &self.text_color_tertiary,
            "text_color_primary_inverse" => &self.text_color_primary_inverse,
            "text_color_secondary_inverse" => &self.text_color_secondary_inverse,
            "text_color_tertiary_inverse" => &self.text_color_tertiary_inverse,
            "color_background" => &self.color_background,
            "color_background_floating" => &self.color_background_floating,
            "color_surface" => &self.color_surface,
            "color_surface_variant" => &self.color_surface_variant,
            "color_surface_highlight" => &self.color_surface_highlight,
            "surface_header" => &self.surface_header,
            "under_surface" => &self.under_surface,
            "off_state" => &self.off_state,
            "accent_surface" => &self.accent_surface,
            "text_primary_on_accent" => &self.text_primary_on_accent,
            "text_secondary_on_accent" => &self.text_secondary_on_accent,
            "volume_background" => &self.volume_background,
            "scrim_android" => &self.scrim,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(colors: [u8; 4]) -> Color {
        Color {
            red: colors[1],
            green: colors[2],
            blue: colors[3],
            alpha: colors[0],
        }
    }
}

pub fn show_color(schemes: &Schemes, source_color: &[u8; 4]) {
    let mut table: Table = generate_table_format();

    for field in COLORS {
        let color_light: Color =
            Color::new(*Scheme::get_value(&schemes.light, field, source_color));
        let color_dark: Color = Color::new(*Scheme::get_value(&schemes.dark, field, source_color));
        let color_amoled: Color =
            Color::new(*Scheme::get_value(&schemes.amoled, field, source_color));

        generate_table_rows(&mut table, &field, color_light, color_dark, color_amoled);
    }

    let mut table_android: Table = generate_table_format();

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

        generate_table_rows(
            &mut table_android,
            &field,
            color_light,
            color_dark,
            color_amoled,
        );
    }

    table.printstd();
    table_android.printstd();
}

fn hex(color: Color, prefix: bool) -> String {
    format!(
        "{}{:02x}{:02x}{:02x}",
        if prefix { "#" } else { "" },
        color.red,
        color.green,
        color.blue
    )
}

pub fn dump_json(schemes: &Schemes, source_color: &[u8; 4], format: Format) {
    type F = Format;
    let fmt = match format {
        F::Rgb => |c: Color| format!("rgb({:?}, {:?}, {:?})", c.red, c.green, c.blue),
        F::Rgba => |c: Color| {
            format!(
                "rgba({:?}, {:?}, {:?}, {:?})",
                c.red, c.green, c.blue, c.alpha
            )
        },
        F::Hsl => {
            |c: Color| Hsl::new(c.red as f64, c.green as f64, c.blue as f64, None).to_css_string()
        }
        F::Hsla => |c: Color| {
            Hsl::new(
                c.red as f64,
                c.green as f64,
                c.blue as f64,
                Some(c.alpha as f64),
            )
            .to_css_string()
        },
        F::Hex => |c: Color| hex(c, true),
        F::Strip => |c: Color| hex(c, false),
    };

    let mut colors_normal_light: HashMap<&str, String> = HashMap::new();
    let mut colors_normal_dark: HashMap<&str, String> = HashMap::new();
    let mut colors_normal_amoled: HashMap<&str, String> = HashMap::new();

    for field in COLORS {
        let color_light: Color =
            Color::new(*Scheme::get_value(&schemes.light, field, source_color));
        let color_dark: Color = Color::new(*Scheme::get_value(&schemes.dark, field, source_color));
        let color_amoled: Color =
            Color::new(*Scheme::get_value(&schemes.amoled, field, source_color));

        colors_normal_light.insert(field, fmt(color_light));
        colors_normal_dark.insert(field, fmt(color_dark));
        colors_normal_amoled.insert(field, fmt(color_amoled));
    }

    let mut colors_android_light: HashMap<&str, String> = HashMap::new();
    let mut colors_android_dark: HashMap<&str, String> = HashMap::new();
    let mut colors_android_amoled: HashMap<&str, String> = HashMap::new();

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

        colors_android_light.insert(field, fmt(color_light));
        colors_android_dark.insert(field, fmt(color_dark));
        colors_android_amoled.insert(field, fmt(color_amoled));
    }

    println!(
        "{}",
        json!({
            "colors": {
                "light": colors_normal_light,
                "dark": colors_normal_dark,
                "amoled": colors_normal_amoled,
            },
            "colors_android": {
                "light": colors_android_light,
                "dark": colors_android_dark,
                "amoled": colors_android_amoled,
            }
        })
    );
}

fn generate_table_format() -> Table {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separators(
            &[format::LinePosition::Title],
            format::LineSeparator::new('─', '┼', '├', '┤'),
        )
        .separators(
            &[format::LinePosition::Top],
            format::LineSeparator::new('─', '┬', '╭', '╮'),
        )
        .separators(
            &[format::LinePosition::Bottom],
            format::LineSeparator::new('─', '┴', '╰', '╯'),
        )
        .padding(1, 1)
        .build();

    table.set_format(format);

    table.set_titles(Row::new(vec![
        Cell::new("NAME").style_spec("c"),
        Cell::new("LIGHT").style_spec("c"),
        Cell::new("LIGHT").style_spec("c"),
        Cell::new("DARK").style_spec("c"),
        Cell::new("DARK").style_spec("c"),
        Cell::new("AMOLED").style_spec("c"),
        Cell::new("AMOLED").style_spec("c"),
    ]));
    table
}

fn generate_table_rows(
    table: &mut Table,
    field: &str,
    color_light: Color,
    color_dark: Color,
    color_amoled: Color,
) {
    let formatstr = "  ";

    table.add_row(Row::new(vec![
        // Color names
        Cell::new(field).style_spec(""),
        // Light scheme
        Cell::new(
            format!(
                "{}",
                format_argb_as_rgb([
                    color_light.alpha,
                    color_light.red,
                    color_light.green,
                    color_light.blue
                ])
            )
            .to_uppercase()
            .as_str(),
        )
        .style_spec("c"),
        Cell::new(format!("{}", formatstr.style(generate_style(&color_light))).as_str())
            .style_spec("c"),
        // Dark scheme
        Cell::new(
            format!(
                "{}",
                format_argb_as_rgb([
                    color_dark.alpha,
                    color_dark.red,
                    color_dark.green,
                    color_dark.blue
                ])
            )
            .to_uppercase()
            .as_str(),
        )
        .style_spec("c"),
        Cell::new(format!("{}", formatstr.style(generate_style(&color_dark))).as_str())
            .style_spec("c"),
        // Amoled theme
        Cell::new(
            format!(
                "{}",
                format_argb_as_rgb([
                    color_amoled.alpha,
                    color_amoled.red,
                    color_amoled.green,
                    color_amoled.blue
                ])
            )
            .to_uppercase()
            .as_str(),
        )
        .style_spec("c"),
        Cell::new(format!("{}", formatstr.style(generate_style(&color_amoled))).as_str())
            .style_spec("c"),
    ]));
}

fn generate_style(color: &Color) -> Style {
    let luma = color.red as u16 + color.blue as u16 + color.green as u16;

    let owo_color: owo_colors::Rgb = owo_colors::Rgb(color.red, color.green, color.blue);

    if luma > 500 {
        Style::new().black().on_color(owo_color)
    } else {
        Style::new().white().on_color(owo_color)
    }
}

pub fn get_source_color(source: &Source) -> Result<[u8; 4], Report> {
    let source_color: [u8; 4] = match &source {
        Source::Image { path } => source_color_from_image(path)?[0],
        Source::Color(color) => {
            let src: Rgb;

            match color {
                ColorFormat::Hex { string } => {
                    src = Rgb::from_hex_str(string).expect("Invalid hex color string provided")
                }
                ColorFormat::Rgb { string } => {
                    src = string.parse().expect("Invalid rgb color string provided")
                }
                ColorFormat::Hsl { string } => {
                    src = Hsl::from_str(string)
                        .expect("Invalid hsl color string provided")
                        .into()
                }
            }
            [
                src.alpha() as u8,
                src.red() as u8,
                src.green() as u8,
                src.blue() as u8,
            ]
        }
    };
    Ok(source_color)
}
