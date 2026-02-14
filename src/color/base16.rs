use color_eyre::{eyre::WrapErr, Report};
use colorsys::{Hsl, Rgb};
use image::{ImageReader, RgbImage};
use indexmap::IndexMap;
use material_colors::{
    color::Argb,
    hct::Hct,
    utils::math::{difference_degrees, rotate_direction, sanitize_degrees_double},
};

use crate::{
    color::{
        backend::wal::WalBackend,
        color::{get_source_color_from_color, ColorFormat, Source},
        format::{argb_from_rgb, rgb_from_argb},
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

fn drag_hue(source_hue: f64, target_hue: f64, amount: f64) -> f64 {
    let rot_deg = difference_degrees(source_hue, target_hue);
    let rot_dir = rotate_direction(source_hue, target_hue) * amount;
    sanitize_degrees_double(rot_deg.mul_add(rot_dir, source_hue))
}

pub fn generate_base16_scheme_from_palette(
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

pub fn generate_base16_scheme_from_color(
    color: &Rgb,
    dark: bool,
) -> Result<IndexMap<String, Argb>, Report> {
    let mut scheme = IndexMap::new();

    let hsl: Hsl = color.into();
    let (source_hue, source_sat, source_lit) = (hsl.hue(), hsl.saturation(), hsl.lightness());
    let base00: Rgb = Hsl::new(source_hue, source_sat * 0.3, source_lit * 1.5, None).into();
    let base05: Rgb = Hsl::new(source_hue, source_sat * 0.7, source_lit * 0.2, None).into();

    let gray_ramp = interpolate_grays(&base00, &base05, dark);
    for (i, &name) in GRAY_NAMES.iter().enumerate() {
        scheme.insert(name.to_string(), gray_ramp[i]);
    }

    let hct: Hct = argb_from_rgb(color).into();
    let source_chroma = hct.get_chroma();
    let source_tone = hct.get_tone();
    let pri_hue = hct.get_hue();
    let acc_hue = pri_hue + 60.0;
    let red_hue = drag_hue(pri_hue, 25.0, 0.8);
    let grn_hue = drag_hue(pri_hue, 118.0, 0.8);
    let off_hue = 10.0_f64.mul_add(rotate_direction(red_hue, pri_hue), red_hue);
    let main_chroma = source_chroma.max(80.0);
    let mute_chroma = main_chroma / 2.0;
    let depr_chroma = (source_chroma / 6.0).min(10.0);
    let main_tone = source_tone.mul_add(0.3, 50.0);
    let depr_tone = source_tone.mul_add(0.5, 20.0);
    let accent_parameters = [
        (red_hue, main_chroma, main_tone), // Semantics: Variables, Diff Deleted
        (off_hue, mute_chroma, main_tone), // Semantics: Literals
        (pri_hue, mute_chroma, main_tone), // Semantics: Classes
        (grn_hue, main_chroma, main_tone), // Semantics: Strings, Diff Inserted
        (acc_hue, mute_chroma, main_tone), // Semantics: Escape Characters
        (pri_hue, main_chroma, main_tone), // Semantics: Functions
        (acc_hue, main_chroma, main_tone), // Semantics: Keywords, Diff Changed
        (pri_hue, depr_chroma, depr_tone), // Semantics: Deprecated
    ];

    for (i, &name) in ACCENT_NAMES.iter().enumerate() {
        let (hue, chroma, tone) = accent_parameters[i];
        scheme.insert(name.to_string(), Hct::from(hue, chroma, tone).into());
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

pub fn generate_base16_schemes(source: &Source, backend: Backend) -> Result<Schemes, Report> {
    let schemes = match source {
        Source::Json { path: _ } => unreachable!(),
        Source::Image { path } => {
            let image = ImageReader::open(path)?
                .with_guessed_format()?
                .decode()?
                .to_rgb8();
            generate_base16_schemes_from_image(&image, backend).wrap_err(format!(
                "Could not generate base16 scheme from image: {}",
                path
            ))?
        }
        Source::Color(color) => generate_base16_schemes_from_color(color).wrap_err(format!(
            "Could not generate base16 scheme from color: {}",
            color.get_string()
        ))?,
        #[cfg(feature = "web-image")]
        Source::WebImage { url } => {
            let bytes = reqwest::blocking::get(url)?.bytes()?;
            let image = image::load_from_memory(&bytes)?.to_rgb8();
            generate_base16_schemes_from_image(&image, backend).wrap_err(format!(
                "Could not generate base16 scheme from image: {}",
                url
            ))?
        }
    };
    Ok(schemes)
}

pub fn generate_base16_schemes_from_image(
    image: &RgbImage,
    backend: Backend,
) -> Result<Schemes, Report> {
    let palette = backend.create().extract(&image);

    let dark_scheme = generate_base16_scheme_from_palette(&palette, true)?;
    let light_scheme = generate_base16_scheme_from_palette(&palette, false)?;

    Ok(Schemes {
        dark: dark_scheme,
        light: light_scheme,
    })
}

pub fn generate_base16_schemes_from_color(color: &ColorFormat) -> Result<Schemes, Report> {
    let source_color = rgb_from_argb(get_source_color_from_color(color)?);

    let dark_scheme = generate_base16_scheme_from_color(&source_color, true)?;
    let light_scheme = generate_base16_scheme_from_color(&source_color, false)?;

    Ok(Schemes {
        dark: dark_scheme,
        light: light_scheme,
    })
}
