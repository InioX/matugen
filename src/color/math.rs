use color_eyre::Report;
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
