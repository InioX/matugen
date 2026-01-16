use color_eyre::Report;
use colorsys::Rgb;
use material_colors::color::{Argb, Lab};
use std::str::FromStr;

pub fn get_color_distance_lab(c1: &str, c2: &str) -> Result<f64, Report> {
    let c1 = Lab::from(Argb::from_str(c1)?);
    let c2 = Lab::from(Argb::from_str(c2)?);

    let l: f64 = c1.l - c2.l;
    let a: f64 = c1.a - c2.a;
    let b: f64 = c1.b - c2.b;

    Ok(f64::sqrt((l * l) + (a * a) + (b * b)))
}

pub fn luminance(c: &Rgb) -> f32 {
    0.2126 * c.red() as f32 + 0.7152 * c.green() as f32 + 0.0722 * c.blue() as f32
}

pub fn saturation(c: &Rgb) -> f32 {
    let max = c.red().max(c.green()).max(c.blue()) as f32;
    let min = c.red().min(c.green()).min(c.blue()) as f32;
    max - min
}
