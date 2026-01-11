use color_eyre::eyre::Report;
use colorsys::{Hsl, Rgb};
use indexmap::IndexMap;
use material_colors::color::Argb;

use crate::{
    color::format::{argb_from_hsl, argb_from_rgb},
    scheme::Schemes,
};

const GRAY_NAMES: [&str; 8] = [
    "base00", "base01", "base02", "base03", "base04", "base05", "base06", "base07",
];

const ACCENT_NAMES: [&str; 8] = [
    "base08", "base09", "base0a", "base0b", "base0c", "base0d", "base0e", "base0f",
];

pub fn generate_base16_scheme(
    base_color: &Rgb,
    dark: bool,
) -> Result<IndexMap<String, Argb>, Report> {
    let mut scheme = IndexMap::new();
    let accent_lightness = if dark { 60.0 } else { 45.0 };
    let base_color_hsl = Hsl::from(base_color);

    let grays = if dark {
        [8.0, 12.0, 18.0, 35.0, 55.0, 75.0, 85.0, 95.0]
    } else {
        [95.0, 90.0, 82.0, 65.0, 45.0, 25.0, 15.0, 8.0]
    };

    for (i, &name) in GRAY_NAMES.iter().enumerate() {
        scheme.insert(name.to_string(), gray(grays[i]));
    }

    let accent_params = [
        (0.0, 60.0),
        (30.0, 60.0),
        (90.0, 50.0),
        (150.0, 55.0),
        (210.0, 55.0),
        (240.0, 60.0),
        (300.0, 60.0),
        (330.0, 50.0),
    ];

    for (i, &name) in ACCENT_NAMES.iter().enumerate() {
        let (hue_offset, saturation) = accent_params[i];
        scheme.insert(
            name.to_string(),
            accent(&base_color_hsl, hue_offset, accent_lightness, saturation),
        );
    }

    Ok(scheme)
}

pub fn generate_base16_schemes(base_color: Rgb) -> Result<Schemes, Report> {
    let dark_scheme = generate_base16_scheme(&base_color, true)?;
    let light_scheme = generate_base16_scheme(&base_color, false)?;

    Ok(Schemes {
        dark: dark_scheme,
        light: light_scheme,
    })
}

fn gray(lightness: f64) -> Argb {
    let hsl = Hsl::new(0.0, 0.0, lightness, None);
    argb_from_hsl(hsl)
}

fn accent(source: &Hsl, hue_offset: f64, lightness: f64, saturation: f64) -> Argb {
    let mut hue = source.hue() + hue_offset;
    hue = hue.rem_euclid(360.0);

    let hsl = Hsl::new(hue, saturation, lightness, None);
    argb_from_hsl(hsl)
}
