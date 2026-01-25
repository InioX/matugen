#[cfg(feature = "dump-json")]
use indexmap::IndexMap;
use material_colors::color::Argb;
use owo_colors::{OwoColorize, Style};

use material_colors::{palette::TonalPalette, theme::Palettes};
use prettytable::{format, Cell, Row, Table};

use colorsys::Rgb;
use serde_json::{Map, Value};

use crate::{
    color::parse::parse_css_color, parser::engine::format_color, scheme::SchemesEnum, Schemes,
};

#[cfg(feature = "dump-json")]
use super::arguments::Format;

use crate::color::format::rgb_from_argb;

const DEFAULT_TONES: [i32; 18] = [
    0, 5, 10, 15, 20, 25, 30, 35, 40, 50, 60, 70, 80, 90, 95, 98, 99, 100,
];

pub fn show_color(schemes: &Schemes, source_color: &Argb, base16: &Schemes) {
    let mut table: Table = generate_table_format();

    for ((field, color_light), (_, color_dark)) in std::iter::zip(&schemes.light, &schemes.dark) {
        let color_light: Rgb = rgb_from_argb(*color_light);
        let color_dark: Rgb = rgb_from_argb(*color_dark);

        generate_table_rows(&mut table, field, color_light, color_dark);
    }

    for ((field, color_light), (_, color_dark)) in std::iter::zip(&base16.light, &base16.dark) {
        let color_light: Rgb = rgb_from_argb(*color_light);
        let color_dark: Rgb = rgb_from_argb(*color_dark);

        generate_table_rows(&mut table, field, color_light, color_dark);
    }

    generate_table_rows(
        &mut table,
        "source_color",
        rgb_from_argb(*source_color),
        rgb_from_argb(*source_color),
    );

    table.printstd();
}

pub fn transform_colors(value: &mut Value, format: &str) {
    match value {
        Value::Object(map) => {
            if map.len() == 1 && map.contains_key("color") {
                if let Some(color_str) = map.get("color").and_then(|v| v.as_str()) {
                    if let Ok(parsed) = parse_css_color(color_str) {
                        *value = Value::String(
                            format_color(parsed, &format)
                                .expect("Failed to transform color into json")
                                .to_string(),
                        );
                        return;
                    }
                }
            }

            for val in map.values_mut() {
                transform_colors(val, format);
            }
        }
        Value::Array(arr) => {
            for val in arr.iter_mut() {
                transform_colors(val, format);
            }
        }
        _ => {}
    }
}

#[cfg(feature = "dump-json")]
pub fn dump_json(json: &mut Value, format: &Format, alt_output: Option<bool>) {
    let format_str = format.to_string();

    if alt_output.unwrap_or(false) {
    } else {
        transform_colors(json, &format_str);
    }

    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

pub fn format_schemes(
    schemes: &Schemes,
    default_scheme: SchemesEnum,
    names: Vec<&String>,
) -> IndexMap<String, Value> {
    let mut scheme_map = IndexMap::new();

    for name in names {
        let dark_hex = schemes.dark.get(name).unwrap().to_hex_with_pound();
        let light_hex = schemes.light.get(name).unwrap().to_hex_with_pound();
        let default_hex = match default_scheme {
            SchemesEnum::Dark => dark_hex.clone(),
            SchemesEnum::Light => light_hex.clone(),
        };

        let mut schemes = Map::new();
        schemes.insert("dark".to_string(), color_entry(dark_hex));
        schemes.insert("light".to_string(), color_entry(light_hex));
        schemes.insert("default".to_string(), color_entry(default_hex));
        scheme_map.insert(name.to_string(), Value::Object(schemes));
    }

    scheme_map
}

pub fn color_entry(hex: String) -> Value {
    let mut m = Map::new();
    m.insert("color".to_string(), Value::String(hex));
    Value::Object(m)
}

pub fn format_palettes(palettes: &Palettes, format: &Format) -> serde_json::Value {
    let format = format.to_string();
    let primary = format_single_palette(palettes.primary, &format);
    let secondary = format_single_palette(palettes.secondary, &format);
    let tertiary = format_single_palette(palettes.tertiary, &format);
    let neutral = format_single_palette(palettes.neutral, &format);
    let neutral_variant = format_single_palette(palettes.neutral_variant, &format);
    let error = format_single_palette(palettes.error, &format);
    serde_json::json!({
        "primary": primary,
        "secondary": secondary,
        "tertiary": tertiary,
        "neutral": neutral,
        "neutral_variant": neutral_variant,
        "error": error,
    })
}

#[cfg(feature = "dump-json")]
fn format_single_palette(palette: TonalPalette, format: &str) -> IndexMap<String, Value> {
    let mut map = IndexMap::new();

    for tone in DEFAULT_TONES.into_iter() {
        map.insert(
            tone.to_string(),
            serde_json::json!({
                "color": format_color(rgb_from_argb(palette.tone(tone)), format).unwrap().to_string()
            }),
        );
    }

    map
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
    ]));
    table
}

fn generate_table_rows(table: &mut Table, field: &str, color_light: Rgb, color_dark: Rgb) {
    let formatstr = "  ";

    table.add_row(Row::new(vec![
        // Color names
        Cell::new(field).style_spec(""),
        // Light scheme
        Cell::new(color_light.to_hex_string().to_uppercase().as_str()).style_spec("c"),
        Cell::new(format!("{}", formatstr.style(generate_style(&color_light))).as_str())
            .style_spec("c"),
        // Dark scheme
        Cell::new(color_dark.to_hex_string().to_uppercase().as_str()).style_spec("c"),
        Cell::new(format!("{}", formatstr.style(generate_style(&color_dark))).as_str())
            .style_spec("c"),
    ]));
}

pub fn generate_style(color: &Rgb) -> Style {
    let luma = color.red() as u16 + color.blue() as u16 + color.green() as u16;

    let owo_color: owo_colors::Rgb =
        owo_colors::Rgb(color.red() as u8, color.green() as u8, color.blue() as u8);

    if luma > 500 {
        Style::new().black().on_color(owo_color)
    } else {
        Style::new().white().on_color(owo_color)
    }
}
