use material_colors::color::{Argb, Lab};
use std::str::FromStr;

pub fn get_color_distance_lab(c1: &str, c2: &str) -> f64 {
    let c1 = Lab::from(Argb::from_str(c1).unwrap());
    let c2 = Lab::from(Argb::from_str(c2).unwrap());

    let l: f64 = c1.l - c2.l;
    let a: f64 = c1.a - c2.a;
    let b: f64 = c1.b - c2.b;

    f64::sqrt((l * l) + (a * a) + (b * b))
}
