use material_colors::dynamic_color::dynamic_scheme::DynamicScheme;
use material_colors::dynamic_color::material_dynamic_colors::MaterialDynamicColors;
use material_colors::utils::theme::{ColorGroup, CustomColor, CustomColorGroup};
use material_colors::{
    Hct, SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome,
    SchemeNeutral, SchemeRainbow, SchemeTonalSpot,
};
use owo_colors::{OwoColorize, Style};

use prettytable::{format, Cell, Row, Table};

use crate::Schemes;

use crate::util::image::fetch_image;

use image::imageops::{resize, FilterType};
use image::io::Reader as ImageReader;

use super::arguments::{ColorFormat, Format, SchemeTypes, Source};
use super::image::source_color_from_image;
use color_eyre::{eyre::Result, Report};
use colorsys::{ColorAlpha, Hsl, Rgb};
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;

use material_colors::blend::harmonize;

pub fn rgb_from_argb(color: [u8; 4]) -> Rgb {
    Rgb::from([
        color[1] as f64,
        color[2] as f64,
        color[3] as f64,
        color[0] as f64,
    ])
}

pub fn format_hex(color: &Rgb) -> String {
    color.to_hex_string()
}

pub fn format_hex_stripped(color: &Rgb) -> String {
    color.to_hex_string()[1..].to_string()
}

pub fn format_rgb(color: &Rgb) -> String {
    format!(
        "rgb({:?}, {:?}, {:?})",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8,
    )
}

pub fn format_rgba(color: &Rgb) -> String {
    format!(
        "rgba({:?}, {:?}, {:?}, {:?})",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8,
        color.alpha() as u8
    )
}

pub fn format_hsl(color: &Hsl) -> String {
    format!(
        "hsl({:?}, {:?}%, {:?}%)",
        color.hue() as u8,
        color.saturation() as u8,
        color.lightness() as u8,
    )
}

pub fn format_hsla(color: &Hsl) -> String {
    format!(
        "hsla({:?}, {:?}%, {:?}%, {:?})",
        color.hue() as u8,
        color.saturation() as u8,
        color.lightness() as u8,
        color.alpha() as u8
    )
}

pub fn generate_dynamic_scheme(
    scheme_type: &Option<SchemeTypes>,
    source_color: [u8; 4],
    is_dark: bool,
    contrast_level: Option<f64>,
) -> DynamicScheme {
    match scheme_type.unwrap() {
        SchemeTypes::SchemeContent => {
            SchemeContent::new(Hct::new(source_color), is_dark, contrast_level).scheme
        }
        SchemeTypes::SchemeExpressive => {
            SchemeExpressive::new(Hct::new(source_color), is_dark, contrast_level).scheme
        }
        SchemeTypes::SchemeFidelity => {
            SchemeFidelity::new(Hct::new(source_color), is_dark, contrast_level).scheme
        }
        SchemeTypes::SchemeFruitSalad => {
            SchemeFruitSalad::new(Hct::new(source_color), is_dark, contrast_level).scheme
        }
        SchemeTypes::SchemeMonochrome => {
            SchemeMonochrome::new(Hct::new(source_color), is_dark, contrast_level).scheme
        }
        SchemeTypes::SchemeNeutral => {
            SchemeNeutral::new(Hct::new(source_color), is_dark, contrast_level).scheme
        }
        SchemeTypes::SchemeRainbow => {
            SchemeRainbow::new(Hct::new(source_color), is_dark, contrast_level).scheme
        }
        SchemeTypes::SchemeTonalSpot => {
            SchemeTonalSpot::new(Hct::new(source_color), is_dark, contrast_level).scheme
        }
    }
}

pub fn make_custom_color(
    color: CustomColor,
    scheme_type: &Option<SchemeTypes>,
    source_color: [u8; 4],
    contrast_level: Option<f64>,
) -> CustomColorGroup {
    debug!("make_custom_color: {:#?}", &color);

    let value = if color.blend {
        harmonize(color.value, source_color)
    } else {
        color.value
    };

    let light = generate_dynamic_scheme(scheme_type, value, false, contrast_level);
    let dark = generate_dynamic_scheme(scheme_type, value, true, contrast_level);

    let custom_color = CustomColorGroup {
        color,
        value,
        light: ColorGroup {
            color: MaterialDynamicColors::primary().get_argb(&light),
            on_color: MaterialDynamicColors::on_primary().get_argb(&light),
            color_container: MaterialDynamicColors::primary_container().get_argb(&light),
            on_color_container: MaterialDynamicColors::on_primary_container().get_argb(&light),
        },
        dark: ColorGroup {
            color: MaterialDynamicColors::primary().get_argb(&dark),
            on_color: MaterialDynamicColors::on_primary().get_argb(&dark),
            color_container: MaterialDynamicColors::primary_container().get_argb(&dark),
            on_color_container: MaterialDynamicColors::on_primary_container().get_argb(&dark),
        },
    };

    debug!("custom_color: {:#?}", &custom_color);
    custom_color
}

pub fn show_color(schemes: &Schemes, source_color: &[u8; 4]) {
    let mut table: Table = generate_table_format();

    for (field, _color) in &schemes.dark {
        let color_light: Rgb = rgb_from_argb(schemes.light[field]);
        let color_dark: Rgb = rgb_from_argb(schemes.dark[field]);

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

pub fn dump_json(schemes: &Schemes, source_color: &[u8; 4], format: &Format) {
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

pub fn get_source_color(source: &Source) -> Result<[u8; 4], Report> {
    let source_color: [u8; 4] = match &source {
        Source::Image { path } => {
            // test
            info!("Opening image in <d><u>{}</>", path);
            let img = ImageReader::open(path)
                .expect("failed to open image")
                .with_guessed_format()
                .expect("failed to guess format")
                .decode()
                .expect("failed to decode image")
                .into_rgba8();
            let img = resize(&img, 128, 128, FilterType::Gaussian);

            source_color_from_image(img)?
        }
        Source::WebImage { url } => {
            // test
            info!("Fetching image from <d><u>{}</>", url);

            let img = fetch_image(url)?.into_rgba8();
            let img = resize(&img, 128, 128, FilterType::Gaussian);

            source_color_from_image(img)?
        }
        Source::Color(color) => {
            let src: Rgb = match color {
                ColorFormat::Hex { string } => {
                    Rgb::from_hex_str(string).expect("Invalid hex color string provided")
                }
                ColorFormat::Rgb { string } => {
                    string.parse().expect("Invalid rgb color string provided")
                }
                ColorFormat::Hsl { string } => Hsl::from_str(string)
                    .expect("Invalid hsl color string provided")
                    .into(),
            };
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
