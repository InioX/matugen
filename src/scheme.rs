use core::fmt;
use std::collections::HashMap;

use indexmap::IndexMap;
use material_colors::scheme::Scheme;

use crate::color::color::{generate_dynamic_scheme, make_custom_color, OwnCustomColor};

#[allow(clippy::enum_variant_names)]
#[derive(Clone, clap::ValueEnum, Debug, Copy, PartialEq)]
pub enum SchemeTypes {
    SchemeContent,
    SchemeExpressive,
    SchemeFidelity,
    SchemeFruitSalad,
    SchemeMonochrome,
    SchemeNeutral,
    SchemeRainbow,
    SchemeTonalSpot,
    SchemeVibrant,
}
#[derive(Debug, Clone)]
pub struct Schemes {
    pub light: IndexMap<std::string::String, material_colors::color::Argb>,
    pub dark: IndexMap<std::string::String, material_colors::color::Argb>,
}

impl Schemes {
    pub fn get_all_schemes(&self) -> [&str; 2] {
        ["light", "dark"]
    }
    pub fn get_all_names(&self) -> Vec<&String> {
        let mut vec = vec![];
        for (name, key) in &self.dark {
            vec.push(name);
        }

        vec
    }
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    clap::ValueEnum,
)]
pub enum SchemesEnum {
    Light,
    Dark,
}

impl fmt::Display for SchemesEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            SchemesEnum::Light => "light",
            SchemesEnum::Dark => "dark",
        };

        write!(f, "{str}")
    }
}

pub fn get_custom_color_schemes(
    source_color: material_colors::color::Argb,
    scheme_dark: Scheme,
    scheme_light: Scheme,
    custom_colors: &Option<HashMap<String, OwnCustomColor, std::hash::RandomState>>,
    scheme_type: &Option<SchemeTypes>,
    contrast: &Option<f64>,
) -> Schemes {
    macro_rules! from_color {
        ($color: expr, $variant: ident) => {
            [
                (format!("{}_source", $color.color.name), $color.color.value),
                (format!("{}_value", $color.color.name), $color.color.value),
                (format!("{}", $color.color.name), $color.$variant.color),
                (
                    format!("on_{}", $color.color.name),
                    $color.$variant.on_color,
                ),
                (
                    format!("{}_container", $color.color.name),
                    $color.$variant.color_container,
                ),
                (
                    format!("on_{}_container", $color.color.name),
                    $color.$variant.on_color_container,
                ),
            ]
        };
    }

    let empty = HashMap::new();
    let custom_colors = custom_colors
        .as_ref()
        .unwrap_or(&empty)
        .iter()
        .map(|(name, color)| {
            make_custom_color(
                color.to_custom_color(name.to_string()).unwrap_or_else(|_| {
                    panic!("Failed to parse custom color: {}, {:?}", name, color)
                }),
                scheme_type,
                source_color,
                *contrast,
            )
        });

    let custom_colors_dark = custom_colors.clone().flat_map(|c| from_color!(c, dark));
    let custom_colors_light = custom_colors.flat_map(|c| from_color!(c, light));

    let schemes: Schemes = Schemes {
        dark: IndexMap::from_iter(scheme_dark.into_iter().chain(custom_colors_dark)),
        light: IndexMap::from_iter(scheme_light.into_iter().chain(custom_colors_light)),
    };
    schemes
}

pub fn get_schemes(
    source_color: material_colors::color::Argb,
    scheme_type: &Option<SchemeTypes>,
    contrast: &Option<f64>,
) -> (Scheme, Scheme) {
    let scheme_dark = Scheme::from(generate_dynamic_scheme(
        scheme_type,
        source_color,
        true,
        *contrast,
    ));
    let scheme_light = Scheme::from(generate_dynamic_scheme(
        scheme_type,
        source_color,
        false,
        *contrast,
    ));
    (scheme_dark, scheme_light)
}

#[cfg(test)]
mod tests {
    use super::*;
    use material_colors::color::Argb;

    #[test]
    fn schemes_eq() {
        let source_color = material_colors::color::Argb::new(255, 255, 0, 0);
        assert_eq!(
            Scheme::from(generate_dynamic_scheme(&None, source_color, true, None,)).primary,
            Argb {
                alpha: 255,
                red: 255,
                green: 180,
                blue: 168,
            }
        );
    }
}
