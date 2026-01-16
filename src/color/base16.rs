use color_eyre::eyre::Report;
use colorsys::Rgb;
use image::RgbImage;
use indexmap::IndexMap;
use material_colors::color::Argb;

use crate::{
    color::{
        backend::wal::WalBackend,
        format::argb_from_rgb,
        math::{luminance, saturation},
    },
    scheme::Schemes,
};

const GRAY_NAMES: [&str; 8] = [
    "base00", "base01", "base02", "base03", "base04", "base05", "base06", "base07",
];

const ACCENT_NAMES: [&str; 8] = [
    "base08", "base09", "base0a", "base0b", "base0c", "base0d", "base0e", "base0f",
];

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Backend {
    Wal,
}

impl Backend {
    pub fn create(&self) -> Box<dyn PaletteBackend> {
        match self {
            Backend::Wal => Box::new(WalBackend::default()),
        }
    }
}

pub trait PaletteBackend {
    fn extract(&self, image: &RgbImage) -> Vec<Rgb>;
}

pub fn generate_base16_scheme(
    palette: &[Rgb],
    dark: bool,
) -> Result<IndexMap<String, Argb>, Report> {
    let mut scheme = IndexMap::new();

    let mut sorted = palette.to_vec();
    sorted.sort_by(|a, b| luminance(b).partial_cmp(&luminance(a)).unwrap());

    let base00 = sorted.first().unwrap();
    let base05 = sorted.last().unwrap();

    scheme.insert("base00".to_string(), argb_from_rgb(base00));
    scheme.insert("base05".to_string(), argb_from_rgb(base05));

    let gray_ramp = interpolate_grays(base00, base05, dark);
    for (i, &name) in GRAY_NAMES.iter().enumerate() {
        scheme.insert(name.to_string(), gray_ramp[i]);
    }

    let mut accents: Vec<&Rgb> = sorted.iter().collect();
    accents.sort_by(|a, b| saturation(b).partial_cmp(&saturation(a)).unwrap());

    for (i, &name) in ACCENT_NAMES.iter().enumerate() {
        scheme.insert(name.to_string(), argb_from_rgb(accents[i % accents.len()]));
    }

    Ok(scheme)
}

fn interpolate_grays(base00: &Rgb, base05: &Rgb, dark: bool) -> Vec<Argb> {
    let mut grays = Vec::new();
    let n = GRAY_NAMES.len();

    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let r = base00.red() as f32 + t * (base05.red() as f32 - base00.red() as f32);
        let g = base00.green() as f32 + t * (base05.green() as f32 - base00.green() as f32);
        let b = base00.blue() as f32 + t * (base05.blue() as f32 - base00.blue() as f32);
        grays.push(Argb::new(
            255,
            r.round() as u8,
            g.round() as u8,
            b.round() as u8,
        ));
    }

    if dark {
        grays.reverse();
    }

    grays
}

pub fn generate_base16_schemes(path: &String, backend: Backend) -> Result<Schemes, Report> {
    let image = image::open(path)?.to_rgb8();
    let palette = backend.create().extract(&image);

    let dark_scheme = generate_base16_scheme(&palette, true)?;
    let light_scheme = generate_base16_scheme(&palette, false)?;

    Ok(Schemes {
        dark: dark_scheme,
        light: light_scheme,
    })
}
