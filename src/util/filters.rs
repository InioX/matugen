use colorsys::ColorAlpha;
use colorsys::ColorTransform;
use colorsys::Hsl;
use colorsys::Rgb;
use std::str::FromStr;
use upon::Value;

use crate::util::template::{check_string_value, parse_color};

use crate::util::color::{
    format_hex, format_hex_stripped, format_hsl, format_hsla, format_hsla_float, format_rgb,
    format_rgba, format_rgba_float,
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

pub fn set_alpha(value: &Value, amount: f64) -> Result<String, String> {
    let string = check_string_value(value).unwrap();

    let format = parse_color(string);

    debug!("Setting alpha on string {} by {}", string, amount);

    if format.is_none() {
        return Ok(string.to_string());
    }

    if !(0.0..=1.0).contains(&amount) {
        return Err("alpha must be in range [0.0 to 1.0]".to_string());
    }

    match format.unwrap() {
        "hex" => Err("cannot set alpha on hex color".to_string()),
        "hex_stripped" => Err("cannot set alpha on hex color".to_string()),
        "rgb" => Err("cannot set alpha on rgb color, use rgba".to_string()),
        "rgba" => {
            let mut color = Rgb::from_str(string).unwrap();
            color.set_alpha(amount);
            Ok(format_rgba(&color, false))
        }
        "hsl" => Err("cannot set alpha on hsl color, use hsla".to_string()),
        "hsla" => {
            let mut color = Hsl::from_str(string).unwrap();
            color.set_alpha(amount);
            Ok(format_hsla(&color, false))
        }
        v => Ok(v.to_string()),
    }
}

pub fn set_alpha(value: &Value, amount: f64) -> Result<String, String> {
    let string = check_string_value(value).unwrap();

    let format = parse_color(string);

    debug!("Setting alpha on string {} by {}", string, amount);

    if format.is_none() {
        return Ok(string.to_string());
    }

    if !(0.0..=1.0).contains(&amount) {
        return Err("alpha must be in range [0.0 to 1.0]".to_string());
    }

    match format.unwrap() {
        "hex" => Err("cannot set alpha on hex color".to_string()),
        "hex_stripped" => Err("cannot set alpha on hex color".to_string()),
        "rgb" => Err("cannot set alpha on rgb color, use rgba".to_string()),
        "rgba" => {
            let mut color = Rgb::from_str(string).unwrap();
            color.set_alpha(amount);
            Ok(format_rgba_float(&color))
        }
        "hsl" => Err("cannot set alpha on hsl color, use hsla".to_string()),
        "hsla" => {
            let mut color = Hsl::from_str(string).unwrap();
            color.set_alpha(amount);
            Ok(format_hsla_float(&color))
        }
        v => Ok(v.to_string()),
    }
}
