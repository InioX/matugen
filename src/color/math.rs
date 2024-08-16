use material_colors::color::{Argb, Lab};
use std::str::FromStr;
use colorsys::Rgb;

pub fn get_color_distance_lab(c1: &str, c2: &str) -> f64 {
    let c1 = Lab::from(Argb::from_str(c1).unwrap());
    let c2 = Lab::from(Argb::from_str(c2).unwrap());

    let l: f64 = c1.l - c2.l;
    let a: f64 = c1.a - c2.a;
    let b: f64 = c1.b - c2.b;

    f64::sqrt((l * l) + (a * a) + (b * b))
}

// for rgb - useless but ill keep it here

#[allow(dead_code)]
pub fn get_color_distance(c1: &Rgb, c2: &Rgb) -> f64 {
    let (r1, g1, b1) = (c1.red() as i64, c1.blue() as i64, c1.green() as i64);
    let (r2, g2, b2) = (c2.red() as i64, c2.green() as i64, c2.blue() as i64);

    let rmean: f64 = ((r1 + r2) / 2) as f64;
    let weight_r: f64 = 2.0 + rmean / 256.0;
    let weight_g: f64 = 4.0;
    let weight_b: f64 = 2.0 + (255.0 - rmean) / 256.0;

    return f64::sqrt(weight_r * i64::pow(r1-r2, 2) as f64 + weight_g * i64::pow(g1-g2, 2) as f64 + weight_b * i64::pow(b1-b2, 2) as f64)
}