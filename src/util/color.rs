use material_colors::color::Argb;
use owo_colors::{OwoColorize, Style};

use prettytable::{format, Cell, Row, Table};

use crate::Schemes;

#[cfg(feature = "dump-json")]
use super::arguments::Format;

use colorsys::Rgb;
use matugen::color::format::rgb_from_argb;

pub fn show_color(schemes: &Schemes, source_color: &Argb) {
    let mut table: Table = generate_table_format();

    for ((field, color_light), (_, color_dark)) in std::iter::zip(&schemes.light, &schemes.dark) {
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

#[cfg(feature = "dump-json")]
pub fn dump_json(schemes: &Schemes, source_color: &Argb, format: &Format) {
    use colorsys::{ColorAlpha, Hsl};
    use serde_json::json;
    use std::collections::HashMap;

    type F = Format;
    let fmt = match format {
        F::Rgb => |c: Rgb| format!("rgb({:?}, {:?}, {:?})", c.red(), c.green(), c.blue()),
        F::Rgba => |c: Rgb| {
            format!(
                "rgba({:?}, {:?}, {:?}, {:?})",
                c.red(),
                c.green(),
                c.blue(),
                c.alpha()
            )
        },
        F::Hsl => |c: Rgb| Hsl::from((c.red(), c.green(), c.blue())).to_css_string(),
        F::Hsla => |c: Rgb| Hsl::from((c.red(), c.green(), c.blue(), c.alpha())).to_css_string(),
        F::Hex => |c: Rgb| c.to_hex_string(),
        F::Strip => |c: Rgb| c.to_hex_string().replace('#', ""),
    };

    let mut colors_normal_light: HashMap<&str, String> = HashMap::new();
    let mut colors_normal_dark: HashMap<&str, String> = HashMap::new();

    for (field, _color) in &schemes.dark {
        let color_light: Rgb = rgb_from_argb(schemes.light[field]);
        let color_dark: Rgb = rgb_from_argb(schemes.dark[field]);

        colors_normal_light.insert(field, fmt(color_light));
        colors_normal_dark.insert(field, fmt(color_dark));
    }

    colors_normal_light.insert("source_color", fmt(rgb_from_argb(*source_color)));

    println!(
        "{}",
        json!({
            "colors": {
                "light": colors_normal_light,
                "dark": colors_normal_dark,
            },
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

fn generate_style(color: &Rgb) -> Style {
    let luma = color.red() as u16 + color.blue() as u16 + color.green() as u16;

    let owo_color: owo_colors::Rgb =
        owo_colors::Rgb(color.red() as u8, color.green() as u8, color.blue() as u8);

    if luma > 500 {
        Style::new().black().on_color(owo_color)
    } else {
        Style::new().white().on_color(owo_color)
    }
}
