use material_colors::color::Lab;
use material_colors::dynamic_color::dynamic_scheme::DynamicScheme;
use material_colors::dynamic_color::material_dynamic_colors::MaterialDynamicColors;
use material_colors::theme::{ColorGroup, CustomColor, CustomColorGroup};
use material_colors::{
    color::Argb,
    hct::Hct,
    image::FilterType,
    image::ImageReader,
    scheme::variant::{
        SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome,
        SchemeNeutral, SchemeRainbow, SchemeTonalSpot,
    },
};
use owo_colors::{OwoColorize, Style};

use prettytable::{format, Cell, Row, Table};

use crate::Schemes;

use super::arguments::{ColorFormat, Format, SchemeTypes, Source};
use super::template::ColorDefinition;
use color_eyre::{eyre::Result, Report};
use colorsys::{ColorAlpha, Hsl, Rgb};
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;

use material_colors::blend::harmonize;

pub fn rgb_from_argb(color: Argb) -> Rgb {
    Rgb::from([
        color.red as f64,
        color.green as f64,
        color.blue as f64,
        color.alpha as f64,
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

pub fn get_color_distance_lab(c1: &str, c2: &str) -> f64 {
    let c1 = Lab::from(Argb::from_str(c1).unwrap());
    let c2 = Lab::from(Argb::from_str(c2).unwrap());

    let l: f64 = c1.l - c2.l;
    let a: f64 = c1.a - c2.a;
    let b: f64 = c1.b - c2.b;

    return f64::sqrt((l * l) + (a * a) + (b * b));
}

// for rgb - useless but ill keep it here

// pub fn get_color_distance(c1: &Rgb, c2: &Rgb) -> f64 {
//     let (r1, g1, b1) = (c1.red() as i64, c1.blue() as i64, c1.green() as i64);
//     let (r2, g2, b2) = (c2.red() as i64, c2.green() as i64, c2.blue() as i64);

//     let rmean: f64 = ((r1 + r2) / 2) as f64;
//     let weightR: f64 = 2.0 + rmean / 256.0;
//     let weightG: f64 = 4.0;
//     let weightB: f64 = 2.0 + (255.0 - rmean) / 256.0;

//     return f64::sqrt(weightR * i64::pow(r1-r2, 2) as f64 + weightG * i64::pow(g1-g2, 2) as f64 + weightB * i64::pow(b1-b2, 2) as f64)
// }

pub fn color_to_string(colors_to_compare: &Vec<ColorDefinition>, compare_to: &String) -> String {
    let mut closest_distance: Option<f64> = None;
    let mut closest_color: &str = "";

    for c in colors_to_compare {
        let distance = get_color_distance_lab(&c.color, &compare_to);
        if closest_distance.is_none() || closest_distance.unwrap() > distance {
            closest_distance = Some(distance);
            closest_color = &c.name;
        }
        debug!("distance: {}, name: {}", distance, c.name)
    }
    debug!(
        "closest distance: {:?}, closest color: {}",
        closest_distance, closest_color
    );
    return closest_color.to_string();
}

pub fn generate_dynamic_scheme(
    scheme_type: &Option<SchemeTypes>,
    source_color: Argb,
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
    source_color: Argb,
    contrast_level: Option<f64>,
) -> CustomColorGroup {
    // debug!("make_custom_color: {:#?}", &color);

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

    // debug!("custom_color: {:#?}", &custom_color);
    custom_color
}

pub fn show_color(schemes: &Schemes, source_color: &Argb) {
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

pub fn dump_json(schemes: &Schemes, source_color: &Argb, format: &Format) {
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

pub fn get_source_color(source: &Source) -> Result<Argb, Report> {
    let source_color: Argb = match &source {
        Source::Image { path } => {
            // test
            info!("Opening image in <d><u>{}</>", path);
            ImageReader::extract_color(ImageReader::open(path)?.resize(
                128,
                128,
                FilterType::Lanczos3,
            ))
        }
        Source::WebImage { url } => {
            // test
            info!("Fetching image from <d><u>{}</>", url);

            let bytes = reqwest::blocking::get(url)?.bytes()?;
            ImageReader::extract_color(ImageReader::read(&bytes)?.resize(
                128,
                128,
                FilterType::Lanczos3,
            ))
        }
        Source::Color(color) => match color {
            ColorFormat::Hex { string } => {
                Argb::from_str(string).expect("Invalid hex color string provided")
            }
            ColorFormat::Rgb { string } => {
                string.parse().expect("Invalid rgb color string provided")
            }
            ColorFormat::Hsl { string } => {
                let rgb: Rgb = Hsl::from_str(string)
                    .expect("Invalid hsl color string provided")
                    .into();
                Argb {
                    red: rgb.red() as u8,
                    green: rgb.green() as u8,
                    blue: rgb.blue() as u8,
                    alpha: 255,
                }
            }
        },
    };
    Ok(source_color)
}
