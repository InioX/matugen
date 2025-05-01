use material_colors::{
    blend::harmonize,
    color::Argb,
    dynamic_color::{DynamicScheme, MaterialDynamicColors},
    hct::Hct,
    image::{FilterType, ImageReader},
    scheme::variant::{
        SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome,
        SchemeNeutral, SchemeRainbow, SchemeTonalSpot, SchemeVibrant,
    },
    theme::{ColorGroup, CustomColor, CustomColorGroup},
};

use colorsys::{Hsl, Rgb};
use std::str::FromStr;

use crate::{color::math::get_color_distance_lab, scheme::SchemeTypes};

#[derive(clap::Parser, Debug, Clone)]
pub enum ColorFormat {
    Hex { string: String },
    Rgb { string: String },
    Hsl { string: String },
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Source {
    /// The image to use for generating a color scheme
    Image { path: String },

    #[cfg(feature = "web-image")]
    /// The image to fetch from web and use for generating a color scheme
    WebImage { url: String },

    /// The source color to use for generating a color scheme
    #[clap(subcommand)]
    Color(crate::color::color::ColorFormat),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ColorDefinition {
    pub name: String,
    pub color: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OwnCustomColor {
    Color(String),
    Options { color: String, blend: bool },
}

impl OwnCustomColor {
    pub fn to_custom_color(
        &self,
        name: String,
    ) -> Result<material_colors::theme::CustomColor, material_colors::error::Error> {
        Ok(match self {
            OwnCustomColor::Color(color) => material_colors::theme::CustomColor {
                value: Argb::from_str(color)?,
                blend: true,
                name,
            },
            OwnCustomColor::Options { color, blend } => material_colors::theme::CustomColor {
                value: Argb::from_str(color)?,
                blend: *blend,
                name,
            },
        })
    }
}

pub fn get_source_color(source: &Source) -> Result<Argb, Box<dyn std::error::Error>> {
    use crate::color::color;

    let source_color: Argb = match source {
        Source::Image { path } => {
            // test
            info!("Opening image in <d><u>{}</>", path);
            color::get_source_color_from_image(path).expect(&format!(
                "Could not get source color from image {:#?}",
                path
            ))
        }
        #[cfg(feature = "web-image")]
        Source::WebImage { url } => {
            info!("Fetching image from <d><u>{}</>", url);
            color::get_source_color_from_web_image(url)
                .expect("Could not get source color from web image")
        }
        Source::Color(color) => color::get_source_color_from_color(color)
            .expect("Could not get source color from color"),
    };
    Ok(source_color)
}

pub fn get_source_color_from_image(path: &str) -> Result<Argb, Box<dyn std::error::Error>> {
    Ok(ImageReader::extract_color(ImageReader::open(path)?.resize(
        128,
        128,
        FilterType::Lanczos3,
    )))
}

#[cfg(feature = "web-image")]
pub fn get_source_color_from_web_image(url: &str) -> Result<Argb, Box<dyn std::error::Error>> {
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    Ok(ImageReader::extract_color(
        ImageReader::read(&bytes)?.resize(128, 128, FilterType::Lanczos3),
    ))
}

pub fn get_source_color_from_color(
    color: &ColorFormat,
) -> Result<Argb, Box<dyn std::error::Error>> {
    match color {
        ColorFormat::Hex { string } => {
            Ok(Argb::from_str(string).expect("Invalid hex color string provided"))
        }
        ColorFormat::Rgb { string } => {
            let rgb = Rgb::from_str(string).expect("Invalid rgb color string provided");
            Ok(Argb {
                red: rgb.red() as u8,
                green: rgb.green() as u8,
                blue: rgb.blue() as u8,
                alpha: 255,
            })
        }
        ColorFormat::Hsl { string } => {
            let rgb: Rgb = Hsl::from_str(string)
                .expect("Invalid hsl color string provided")
                .into();
            Ok(Argb {
                red: rgb.red() as u8,
                green: rgb.green() as u8,
                blue: rgb.blue() as u8,
                alpha: 255,
            })
        }
    }
}

pub fn generate_dynamic_scheme(
    scheme_type: &Option<SchemeTypes>,
    source_color: Argb,
    is_dark: bool,
    contrast_level: Option<f64>,
) -> DynamicScheme {
    match scheme_type.unwrap_or(SchemeTypes::SchemeContent) {
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
        SchemeTypes::SchemeVibrant => {
            SchemeVibrant::new(Hct::new(source_color), is_dark, contrast_level).scheme
        }
    }
}

pub fn adjust_color_lightness_dark(
    color: Argb,
    lightness_level: &Option<f64>,
) -> Argb {
    // If lightness values were plotted on a graph, the effect of this function is to rotate the line corresponding to the identity function about x = 255 and y = 255 by setting the value at x = 0 to -lightness_level*255 and then clamping the values to between 0 and 255.
    let pre_lightness_level = ((color.red as f64) + (color.green as f64) + (color.blue as f64))/ 3.0;
    let adj = (pre_lightness_level / 255.0 * (1.0 - lightness_level.unwrap_or(0.0)) + lightness_level.unwrap_or(0.0)) / pre_lightness_level* 255.0;
    Argb::new(color.alpha, (color.red as f64 * adj).clamp(0.0, 255.0) as u8, (color.green as f64 * adj).clamp(0.0, 255.0) as u8, (color.blue as f64 * adj).clamp(0.0, 255.0) as u8)
}

pub fn adjust_color_lightness_light(
    color: Argb,
    lightness_level: &Option<f64>,
) -> Argb {
    // If lightness values were plotted on a graph, the effect of this function is to rotate the line corresponding to the identity function about x = 0 and y = 0 by setting the value at x = 255 to 255+lightness_level*255 and then clamping the values to between 0 and 255.
    let pre_lightness_level = ((color.red as f64) + (color.green as f64) + (color.blue as f64))/ 3.0;
    let adj = pre_lightness_level / 255.0 * (1.0 + lightness_level.unwrap_or(0.0)) / pre_lightness_level* 255.0;
    Argb::new(color.alpha, (color.red as f64 * adj).clamp(0.0, 255.0) as u8, (color.green as f64 * adj).clamp(0.0, 255.0) as u8, (color.blue as f64 * adj).clamp(0.0, 255.0) as u8)
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

    // debug!("custom_color: {:#?}", &custom_color);
    CustomColorGroup {
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
    }
}

pub fn color_to_string(colors_to_compare: &Vec<ColorDefinition>, compare_to: &str) -> String {
    let mut closest_distance: Option<f64> = None;
    let mut closest_color: &str = "";

    for c in colors_to_compare {
        let distance = get_color_distance_lab(&c.color, compare_to);
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
    closest_color.to_string()
}
