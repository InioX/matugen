use std::str::FromStr;

use colorsys::Rgb;
use csscolorparser::{Color, ParseColorError};

pub fn parse_color(string: &str) -> Option<&str> {
    if let Some(_s) = string.strip_prefix('#') {
        return Some("hex");
    }

    if let (Some(i), Some(s)) = (string.find('('), string.strip_suffix(')')) {
        let fname = s[..i].trim_end();
        Some(fname)
    } else if string.len() == 6 {
        // Does not matter if it is actually a stripped hex or not, we handle it somewhere else.
        return Some("hex_stripped");
    } else {
        None
    }
}

pub fn parse_css_color(string: &str) -> Result<Rgb, ParseColorError> {
    match Color::from_str(string) {
        Ok(v) => {
            let alpha = if v.a.is_nan() { 1.0 } else { v.a };
            Ok(Rgb::new(
                v.r as f64 * 255.0,
                v.g as f64 * 255.0,
                v.b as f64 * 255.0,
                Some(alpha.into()),
            ))
        }
        Err(e) => Err(e),
    }
}
