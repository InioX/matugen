use upon::Value;

use colorsys::{ColorTransform, Hsl, Rgb};
use std::str::FromStr;

use crate::color::{
    format::{format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba},
    parse::{check_string_value, parse_color},
};

pub fn set_lightness(value: &Value, amount: f64) -> Result<String, String> {
    let string = check_string_value(value).unwrap();

    let format = parse_color(string);

    debug!("Setting lightness on string {} by {}", string, amount);

    if format.is_none() {
        return Ok(string.to_string());
    }

    match format.unwrap() {
        "hex" => {
            let mut color = Rgb::from_hex_str(string).unwrap();
            color.lighten(amount);
            Ok(format_hex(&color))
        }
        "hex_stripped" => {
            let mut color = Rgb::from_hex_str(string).unwrap();
            color.lighten(amount);
            Ok(format_hex_stripped(&color))
        }
        "rgb" => {
            let mut color = Rgb::from_str(string).unwrap();
            color.lighten(amount);
            Ok(format_rgb(&color))
        }
        "rgba" => {
            let mut color = Rgb::from_str(string).unwrap();
            color.lighten(amount);
            Ok(format_rgba(&color, true))
        }
        "hsl" => {
            let mut color = Hsl::from_str(string).unwrap();
            color.lighten(amount);
            Ok(format_hsl(&color))
        }
        "hsla" => {
            let mut color = Hsl::from_str(string).unwrap();
            color.lighten(amount);
            Ok(format_hsla(&color, true))
        }
        v => Ok(v.to_string()),
    }
}
