use color_eyre::Report;
use colorsys::Rgb;
use material_colors::color::{Argb, Lab};
use std::str::FromStr;

pub fn get_color_distance_lab(c1: &Lab, c2: &Lab) -> f64 {
    let l: f64 = c1.l - c2.l;
    let a: f64 = c1.a - c2.a;
    let b: f64 = c1.b - c2.b;

    f64::sqrt((l * l) + (a * a) + (b * b))
}

pub fn get_color_distance_lab_from_str(c1: &str, c2: &str) -> Result<f64, Report> {
    let c1 = Lab::from(Argb::from_str(c1)?);
    let c2 = Lab::from(Argb::from_str(c2)?);

    Ok(get_color_distance_lab(&c1, &c2))
}

pub fn luminance(c: &Rgb) -> f32 {
    0.2126 * c.red() as f32 + 0.7152 * c.green() as f32 + 0.0722 * c.blue() as f32
}

pub fn lightness(c: &Rgb) -> f32 {
    let y = luminance(c) / 255.0;
    if y <= 0.008856 {
        y * 903.3
    } else {
        y.powf(1.0 / 3.0) * 116.0 - 16.0
    }
}

pub fn saturation(c: &Rgb) -> f32 {
    let max = c.red().max(c.green()).max(c.blue()) as f32;
    let min = c.red().min(c.green()).min(c.blue()) as f32;
    max - min
}

pub fn value(c: &Rgb) -> f32 {
    let max = c.red().max(c.green()).max(c.blue()) as f32;
    max * (100.0 / 255.0)
}
