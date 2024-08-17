use upon::Value;

use colorsys::{ColorTransform, Hsl, Rgb};
use std::str::FromStr;

use crate::color::{
    format::{format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba},
    parse::{check_string_value, parse_color},
};

pub fn grayscale(value: &Value) -> Result<String, String> {
    let string = check_string_value(value).unwrap();

    let format = parse_color(string);

    if format.is_none() {
        return Ok(string.to_string());
    }

    match format.unwrap() {
        "hex" => {
            let mut color = Rgb::from_hex_str(string).unwrap();
            color.grayscale_simple();
            Ok(format_hex(&color))
        }
        "hex_stripped" => {
            let mut color = Rgb::from_hex_str(string).unwrap();
            color.grayscale_simple();
            Ok(format_hex_stripped(&color))
        }
        "rgb" => {
            let mut color = Rgb::from_str(string).unwrap();
            color.grayscale_simple();
            Ok(format_rgb(&color))
        }
        "rgba" => {
            let mut color = Rgb::from_str(string).unwrap();
            color.grayscale_simple();
            Ok(format_rgba(&color, false))
        }
        "hsl" => {
            let mut color = Hsl::from_str(string).unwrap();
            color.grayscale_simple();
            Ok(format_hsl(&color))
        }
        "hsla" => {
            let mut color = Hsl::from_str(string).unwrap();
            color.grayscale_simple();
            Ok(format_hsla(&color, false))
        }
        v => Ok(v.to_string()),
    }
}
