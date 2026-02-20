#[cfg(feature = "dump-json")]
use indexmap::IndexMap;
use material_colors::color::Argb;
use owo_colors::Style;

use colorsys::Rgb;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL_CONDENSED, Cell, CellAlignment, Table,
};
use material_colors::{palette::TonalPalette, theme::Palettes};
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

pub fn show_color(
    schemes: Option<&Schemes>,
    source_color: Option<&Argb>,
    base16: Option<&Schemes>,
) {
    let mut table: Table = generate_table_format();

    if let Some(schemes) = schemes {
        for ((field, color_light), (_, color_dark)) in std::iter::zip(&schemes.light, &schemes.dark)
        {
            let color_light: Rgb = rgb_from_argb(*color_light);
            let color_dark: Rgb = rgb_from_argb(*color_dark);

            generate_table_rows(&mut table, field, color_light, color_dark);
        }
    }

    if let Some(base16) = base16 {
        for ((field, color_light), (_, color_dark)) in std::iter::zip(&base16.light, &base16.dark) {
            let color_light: Rgb = rgb_from_argb(*color_light);
            let color_dark: Rgb = rgb_from_argb(*color_dark);

            generate_table_rows(&mut table, field, color_light, color_dark);
        }
    }

    if let Some(source_color) = source_color {
        generate_table_rows(
            &mut table,
            "source_color",
            rgb_from_argb(*source_color),
            rgb_from_argb(*source_color),
        );
    }

    println!["{table}"];
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
pub fn dump_json(json: &mut Value, format: &Format, old_output: Option<bool>) {
    let format_str = format.to_string();

    if old_output.unwrap_or(false) {
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
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_style(comfy_table::TableComponent::VerticalLines, '│');

    table.set_header([
        Cell::new("NAME").set_alignment(CellAlignment::Center),
        Cell::new("LIGHT").set_alignment(CellAlignment::Center),
        Cell::new("").set_alignment(CellAlignment::Center),
        Cell::new("DARK").set_alignment(CellAlignment::Center),
        Cell::new("").set_alignment(CellAlignment::Center),
    ]);

    table.column_mut(2).unwrap().set_padding((1, 0));
    table.column_mut(4).unwrap().set_padding((1, 0));

    table
}

fn generate_table_rows(table: &mut Table, field: &str, color_light: Rgb, color_dark: Rgb) {
    table.add_row([
        // Color names
        Cell::new(field),
        // Light scheme
        Cell::new_owned(color_light.to_hex_string().to_uppercase()),
        Cell::new("").bg(comfy_table::Color::Rgb {
            r: color_light.red().round() as u8,
            g: color_light.green() as u8,
            b: color_light.blue() as u8,
        }),
        // Dark scheme
        Cell::new_owned(color_dark.to_hex_string().to_uppercase()),
        Cell::new("").bg(comfy_table::Color::Rgb {
            r: color_dark.red() as u8,
            g: color_dark.green() as u8,
            b: color_dark.blue() as u8,
        }),
    ]);
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
