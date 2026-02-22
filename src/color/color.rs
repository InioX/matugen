use dialoguer::Select;
use material_colors::{
    blend::harmonize,
    color::{Argb, Lab},
    dynamic_color::{DynamicScheme, MaterialDynamicColors},
    hct::Cam16,
    image::{FilterType, ImageReader},
    quantize::{Quantizer, QuantizerCelebi},
    score::Score,
    theme::{ColorGroup, CustomColor, CustomColorGroup},
};

use crate::{
    color::math::{get_color_distance_lab, get_color_distance_lab_from_str, lightness, value},
    scheme::SchemeTypes,
};
use crate::{
    color::{format::rgb_from_argb, math::saturation},
    util::color::generate_style,
};
use crate::{util::arguments::SelectionPreference, FilterType as OwnFilterType};
use color_eyre::{eyre::WrapErr, Report};
use colorsys::{Hsl, Rgb};
use owo_colors::OwoColorize;
use std::{io::IsTerminal as _, str::FromStr};

use material_colors::image::AsPixels;

#[derive(clap::Parser, Debug, Clone)]
pub enum ColorFormat {
    Hex { string: String },
    Rgb { string: String },
    Hsl { string: String },
}

impl ColorFormat {
    pub fn get_string(&self) -> &String {
        match self {
            ColorFormat::Hex { string } => string,
            ColorFormat::Rgb { string } => string,
            ColorFormat::Hsl { string } => string,
        }
    }
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

    /// The json file to use and import for templates
    Json { path: String },
}

impl Source {
    pub fn is_image(&self) -> bool {
        match self {
            Source::Image { path: _ } => true,
            _ => false,
        }
    }

    pub fn is_json(&self) -> bool {
        match self {
            Source::Json { path: _ } => true,
            _ => false,
        }
    }
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

pub fn adjust_color_lightness_dark(color: Argb, lightness_level_dark: &Option<f64>) -> Argb {
    // If lightness values were plotted on a graph, the effect of this function is to rotate the line corresponding to the identity function about x = 255 and y = 255 by setting the value at x = 0 to -lightness_level*255 and then clamping the values to between 0 and 255.
    let pre_lightness_level =
        ((color.red as f64) + (color.green as f64) + (color.blue as f64)) / 3.0;
    let adj = (pre_lightness_level / 255.0 * (1.0 - lightness_level_dark.unwrap_or(0.0))
        + lightness_level_dark.unwrap_or(0.0))
        / pre_lightness_level
        * 255.0;
    Argb::new(
        color.alpha,
        (color.red as f64 * adj).clamp(0.0, 255.0) as u8,
        (color.green as f64 * adj).clamp(0.0, 255.0) as u8,
        (color.blue as f64 * adj).clamp(0.0, 255.0) as u8,
    )
}

pub fn adjust_color_lightness_light(color: Argb, lightness_level_light: &Option<f64>) -> Argb {
    // If lightness values were plotted on a graph, the effect of this function is to rotate the line corresponding to the identity function about x = 0 and y = 0 by setting the value at x = 255 to 255+lightness_level*255 and then clamping the values to between 0 and 255.
    let pre_lightness_level =
        ((color.red as f64) + (color.green as f64) + (color.blue as f64)) / 3.0;
    let adj = pre_lightness_level / 255.0 * (1.0 + lightness_level_light.unwrap_or(0.0))
        / pre_lightness_level
        * 255.0;
    Argb::new(
        color.alpha,
        (color.red as f64 * adj).clamp(0.0, 255.0) as u8,
        (color.green as f64 * adj).clamp(0.0, 255.0) as u8,
        (color.blue as f64 * adj).clamp(0.0, 255.0) as u8,
    )
}

pub fn get_source_color(
    source: &Source,
    resize_filter: &Option<OwnFilterType>,
    fallback_color: Option<Argb>,
    prefer: &Option<SelectionPreference>,
    source_color_index: &Option<i64>,
) -> Result<Argb, Report> {
    use crate::color::color;

    let filter: FilterType = match resize_filter {
        Some(v) => FilterType::from(v),
        None => FilterType::from(&OwnFilterType::Triangle),
    };

    let source_color: Argb = match source {
        Source::Image { path } => {
            info!("Opening image in <d><u>{}</>", path);
            color::get_source_color_from_image(
                path,
                filter,
                fallback_color,
                &prefer,
                source_color_index,
            )
            .wrap_err(format!("Could not get source color from image: {}", path))?
        }
        #[cfg(feature = "web-image")]
        Source::WebImage { url } => {
            info!("Fetching image from <d><u>{}</>", url);
            color::get_source_color_from_web_image(url, filter)
                .expect("Could not get source color from web image")
        }
        Source::Color(color) => color::get_source_color_from_color(color).wrap_err(format!(
            "Could not get source color from color {}",
            color.get_string()
        ))?,
        Source::Json { path: _ } => unreachable!(),
    };
    Ok(source_color)
}

pub fn get_source_color_from_image(
    path: &str,
    filter_type: FilterType,
    fallback_color: Option<Argb>,
    prefer: &Option<SelectionPreference>,
    source_color_index: &Option<i64>,
) -> Result<Argb, Report> {
    let mut original = ImageReader::open(path)?;
    let image = original.resize(112, 112, filter_type);
    let pixels: Vec<Argb> = image
        .as_pixels()
        .iter()
        .copied()
        .filter(|argb| argb.alpha == 255)
        .collect();
    let mut result = QuantizerCelebi::quantize(&pixels, 128);

    result
        .color_to_count
        .retain(|&argb, _| Cam16::from(argb).chroma >= 5.0);

    let ranked = Score::score(&result.color_to_count, None, fallback_color, None);

    let ranked_formatted: Vec<String> = ranked
        .clone()
        .iter()
        .map(|c| {
            format!(
                "{} {}",
                c.to_hex_with_pound(),
                "  ".style(generate_style(&rgb_from_argb(*c)))
            )
        })
        .collect();

    debug!("Ranked colors:");
    for (i, color) in ranked_formatted.iter().enumerate() {
        debug!("{}: {}", i, color);
    }

    if let Some(index) = source_color_index {
        // Should be safe because of the range in the argument definition but just in case...
        if *index < 0 || (*index as usize) >= ranked.len() {
            return Err(Report::msg(format!(
                "Source color index {} is out of bounds (0-{})",
                index,
                ranked.len() - 1
            )));
        }
        return Ok(ranked[*index as usize]);
    }

    let selection = match (prefer, std::io::stdin().is_terminal()) {
        (None, false) => return Err(Report::msg(
            "Multiple source colors found, no preference was inputted, and a terminal was not detected.\nUse --prefer=PREFERENCE to find suitable colors without needing user input.",
        )),
        (None, true) => Select::new()
            .items(&ranked_formatted)
            .with_prompt("Select the color you want to use as source color\nUse arrow keys to navigate and Enter to select")
            .default(0)
            .interact()?,
        (Some(preference), _) => {
            debug!["Multiple source colors found, attempting to pick a color by user preference \"{:?}\"", preference];

            select_source_color_from_ranks(&ranked, fallback_color, preference)?
        },
    };
    debug!["Chose {selection}"];

    Ok(ranked[selection])
}

pub fn select_source_color_from_ranks(
    ranked: &[Argb],
    fallback: Option<Argb>,
    preference: &SelectionPreference,
) -> Result<usize, Report> {
    let sel = match preference {
        SelectionPreference::First => 0,
        SelectionPreference::Last => ranked.len().saturating_sub(1),
        SelectionPreference::ClosestToFallback => {
            let Some(fallback) = fallback else {
                return Err(Report::msg(format![
                    "Preference {:?} chosen but no fallback color was provided",
                    preference
                ]));
            };
            let target = Lab::from(fallback);

            ranked
                .iter()
                .map(|col| Lab::from(*col))
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    get_color_distance_lab(&target, a)
                        .total_cmp(&get_color_distance_lab(&target, b))
                })
                .map(|(idx, _)| idx)
                .unwrap_or(0)
        }

        _ => ranked
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let (val_a, val_b) = get_comparison_values(a, b, preference);
                val_a.total_cmp(&val_b)
            })
            .map(|(idx, _)| idx)
            .unwrap_or(0),
    };
    Ok(sel)
}

fn get_comparison_values(a: &Argb, b: &Argb, pref: &SelectionPreference) -> (f32, f32) {
    let rgb_a = rgb_from_argb(*a);
    let rgb_b = rgb_from_argb(*b);

    match pref {
        SelectionPreference::Darkness => (lightness(&rgb_a), lightness(&rgb_b)),
        SelectionPreference::Lightness => (lightness(&rgb_b), lightness(&rgb_a)),
        SelectionPreference::Saturation => (saturation(&rgb_b), saturation(&rgb_a)),
        SelectionPreference::LessSaturation => (saturation(&rgb_a), saturation(&rgb_b)),
        SelectionPreference::Value => (value(&rgb_a), value(&rgb_b)),
        _ => (0.0, 0.0),
    }
}

#[cfg(feature = "web-image")]
pub fn get_source_color_from_web_image(url: &str, filter_type: FilterType) -> Result<Argb, Report> {
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    Ok(ImageReader::extract_color(
        ImageReader::read(&bytes)?.resize(128, 128, filter_type),
    ))
}

pub fn get_source_color_from_color(color: &ColorFormat) -> Result<Argb, Report> {
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
    let scheme_type: SchemeTypes = scheme_type.unwrap_or(SchemeTypes::SchemeContent);
    if let Some(var) = scheme_type.as_material_colors_variant() {
        DynamicScheme::by_variant(source_color, &var, is_dark, contrast_level)
    } else {
        unreachable!()
    }
}

pub fn make_custom_color(
    color: CustomColor,
    scheme_type: &Option<SchemeTypes>,
    source_color: Argb,
    contrast_level: Option<f64>,
) -> CustomColorGroup {
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

pub fn get_closest_color(
    colors_to_compare: &Vec<ColorDefinition>,
    compare_to: &str,
) -> Result<String, Report> {
    let mut closest_distance: Option<f64> = None;
    let mut closest_color: &str = "";

    for c in colors_to_compare {
        let distance = match get_color_distance_lab_from_str(&c.color, compare_to) {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "Failed to get color distance between {} and {}",
                    c.color, compare_to
                );
                return Err(Report::msg(format!("Could not get closest color: {}", e)));
            }
        };
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
    Ok(closest_color.to_string())
}
