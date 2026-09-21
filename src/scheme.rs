use core::fmt;
use std::collections::HashMap;

use color_eyre::Report;
use indexmap::IndexMap;
use material_colors::color::Argb;
use material_colors::{dynamic_color::Variant as MaterialColorsVariant, scheme::Scheme};
use serde::{Deserialize, Serialize};

use crate::color::color::{
    adjust_color_lightness_dark, adjust_color_lightness_light, generate_dynamic_scheme,
    make_custom_color, OwnCustomColor,
};

#[allow(clippy::enum_variant_names)]
#[derive(
    Clone, clap::ValueEnum, Debug, Copy, Eq, PartialEq, Serialize, Deserialize, Hash, Default,
)]
pub enum SchemeTypes {
    #[default]
    SchemeContent,
    SchemeExpressive,
    SchemeFidelity,
    SchemeFruitSalad,
    SchemeMonochrome,
    SchemeNeutral,
    SchemeRainbow,
    SchemeTonalSpot,
    SchemeVibrant,
    SchemeSmart,
}

impl SchemeTypes {
    #[allow(unreachable_patterns)]
    pub fn as_material_colors_variant(&self) -> Option<MaterialColorsVariant> {
        match self {
            SchemeTypes::SchemeContent => Some(MaterialColorsVariant::Content),
            SchemeTypes::SchemeExpressive => Some(MaterialColorsVariant::Expressive),
            SchemeTypes::SchemeFidelity => Some(MaterialColorsVariant::Fidelity),
            SchemeTypes::SchemeFruitSalad => Some(MaterialColorsVariant::FruitSalad),
            SchemeTypes::SchemeMonochrome => Some(MaterialColorsVariant::Monochrome),
            SchemeTypes::SchemeNeutral => Some(MaterialColorsVariant::Neutral),
            SchemeTypes::SchemeRainbow => Some(MaterialColorsVariant::Rainbow),
            SchemeTypes::SchemeTonalSpot => Some(MaterialColorsVariant::TonalSpot),
            SchemeTypes::SchemeVibrant => Some(MaterialColorsVariant::Vibrant),
            SchemeTypes::SchemeSmart => None,
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Schemes {
    pub light: IndexMap<std::string::String, material_colors::color::Argb>,
    pub dark: IndexMap<std::string::String, material_colors::color::Argb>,
}

impl Schemes {
    pub fn get_all_names(&self) -> Vec<&String> {
        let mut vec = vec![];
        for (name, _key) in &self.dark {
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
    Smart,
}

impl fmt::Display for SchemesEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            SchemesEnum::Light => "light",
            SchemesEnum::Dark => "dark",
            SchemesEnum::Smart => "smart",
        };

        write!(f, "{str}")
    }
}

pub fn get_custom_color_schemes(
    source_color: material_colors::color::Argb,
    scheme_dark: Scheme,
    scheme_light: Scheme,
    custom_colors: &Option<HashMap<String, OwnCustomColor, std::hash::RandomState>>,
    scheme_type: SchemeTypes,
    contrast: &Option<f64>,
    lightness_dark: &Option<f64>,
    lightness_light: &Option<f64>,
) -> Result<Schemes, Report> {
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
    let custom_colors: Vec<_> = custom_colors
        .as_ref()
        .unwrap_or(&empty)
        .iter()
        .filter_map(
            |(name, color)| match color.to_custom_color(name.to_string()) {
                Ok(custom_color) => Some(make_custom_color(
                    custom_color,
                    scheme_type,
                    source_color,
                    *contrast,
                )),
                Err(e) => {
                    error!("Failed to parse custom color: {}, {:?}: {}", name, color, e);
                    None
                }
            },
        )
        .collect();

    let custom_colors_dark = custom_colors.iter().flat_map(|c| from_color!(c, dark));
    let custom_colors_light = custom_colors.iter().flat_map(|c| from_color!(c, light));

    let schemes: Schemes = Schemes {
        dark: IndexMap::from_iter(
            scheme_dark
                .into_iter()
                .chain(custom_colors_dark)
                .map(|(name, color)| (name, adjust_color_lightness_dark(color, lightness_dark))),
        ),
        light: IndexMap::from_iter(
            scheme_light
                .into_iter()
                .chain(custom_colors_light)
                .map(|(name, color)| (name, adjust_color_lightness_light(color, lightness_light))),
        ),
    };
    Ok(schemes)
}

pub fn resolve_achromatic_scheme(source_color: Argb, scheme_type: SchemeTypes) -> SchemeTypes {
    let Argb {
        red, green, blue, ..
    } = source_color;

    if red == green && green == blue && !matches!(scheme_type, SchemeTypes::SchemeMonochrome) {
        // Pure grays (black, white and everything between) carry no hue in
        // HCT, so every chromatic scheme variant picks an essentially
        // arbitrary accent hue for them: #000000 comes out magenta under
        // scheme-tonal-spot and #FFFFFF comes out cyan, because the hue the
        // solver lands on for chroma 0 is undefined. Monochrome is the only
        // variant that stays honest about a hueless source, so use it for
        // this run and leave the configured scheme untouched otherwise.
        warn!("source color is achromatic (r == g == b), using scheme-monochrome for this run");
        return SchemeTypes::SchemeMonochrome;
    }

    scheme_type
}

pub fn get_schemes(
    source_color: material_colors::color::Argb,
    scheme_type: SchemeTypes,
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
            Scheme::from(generate_dynamic_scheme(
                SchemeTypes::default(),
                source_color,
                true,
                None,
            ))
            .primary,
            Argb {
                alpha: 255,
                red: 255,
                green: 180,
                blue: 168,
            }
        );
    }

    #[test]
    fn achromatic_sources_fall_back_to_monochrome() {
        let grays = [
            Argb::new(255, 0, 0, 0),
            Argb::new(255, 255, 255, 255),
            Argb::new(255, 128, 128, 128),
            Argb::new(255, 1, 1, 1),
        ];
        let chromatic_variants = [
            SchemeTypes::SchemeContent,
            SchemeTypes::SchemeExpressive,
            SchemeTypes::SchemeFidelity,
            SchemeTypes::SchemeFruitSalad,
            SchemeTypes::SchemeNeutral,
            SchemeTypes::SchemeRainbow,
            SchemeTypes::SchemeTonalSpot,
            SchemeTypes::SchemeVibrant,
            SchemeTypes::SchemeSmart,
        ];

        for gray in grays {
            for variant in chromatic_variants {
                assert_eq!(
                    resolve_achromatic_scheme(gray, variant),
                    SchemeTypes::SchemeMonochrome,
                    "expected achromatic source to resolve to monochrome for {variant:?}",
                );
            }
            // Already monochrome: no change needed, but the result must hold.
            assert_eq!(
                resolve_achromatic_scheme(gray, SchemeTypes::SchemeMonochrome),
                SchemeTypes::SchemeMonochrome,
            );
        }
    }

    #[test]
    fn chromatic_sources_keep_their_scheme() {
        let purple = Argb::new(255, 0x5B, 0x4A, 0x78);
        for variant in [
            SchemeTypes::SchemeTonalSpot,
            SchemeTypes::SchemeVibrant,
            SchemeTypes::SchemeMonochrome,
        ] {
            assert_eq!(
                resolve_achromatic_scheme(purple, variant),
                variant,
                "expected chromatic source to keep the configured scheme",
            );
        }
    }

    #[test]
    fn monochrome_fallback_black_yields_neutral_primary() {
        // Regression guard for the reported behavior: a #000000 source under
        // scheme-tonal-spot used to produce a magenta primary (#ffb1c8 in
        // dark mode). With the achromatic fallback the same source must
        // produce the neutral monochrome scheme instead.
        let black = Argb::new(255, 0, 0, 0);
        let resolved = resolve_achromatic_scheme(black, SchemeTypes::SchemeTonalSpot);

        let scheme = Scheme::from(generate_dynamic_scheme(resolved, black, true, None));
        assert_eq!(
            scheme.primary,
            Argb {
                alpha: 255,
                red: 255,
                green: 255,
                blue: 255,
            }
        );
    }
}
