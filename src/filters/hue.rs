use upon::Value;

use colorsys::{ColorTransform, Hsl, Rgb};
use std::str::FromStr;

use crate::color::{
    parse::{parse_color, check_string_value},
    format::{format_hex, format_hex_stripped, format_rgb, format_rgba, format_hsl, format_hsla},
};

pub fn set_hue(value: &Value, amount: f64) -> Result<String, String> {
    let string = check_string_value(value).unwrap();

    let format = parse_color(string);

    debug!("Setting alpha on string {} by {}", string, amount);

    if format.is_none() {
        error!("Could not detect the format for string {:?}", string);
        return Ok(string.to_string());
    }

    if !(-360.0..=360.0).contains(&amount) {
        return Err("alpha must be in range [-360.0 to 360.0]".to_string());
    }

    match format.unwrap() {
        "hex" => {
            let mut color = Rgb::from_hex_str(string).unwrap();
            color.adjust_hue(amount);
            Ok(format_hex(&color))
        }
        "hex_stripped" => {
            let mut color = Rgb::from_hex_str(string).unwrap();
            color.adjust_hue(amount);
            Ok(format_hex_stripped(&color))
        }
        "rgb" => {
            let mut color = Rgb::from_str(string).unwrap();
            color.adjust_hue(amount);
            Ok(format_rgb(&color))
        }
        "rgba" => {
            let mut color = Rgb::from_str(string).unwrap();
            color.adjust_hue(amount);
            Ok(format_rgba(&color, false))
        }
        "hsl" => {
            let mut color = Hsl::from_str(string).unwrap();
            color.adjust_hue(amount);
            Ok(format_hsl(&color))
        }
        "hsla" => {
            let mut color = Hsl::from_str(string).unwrap();
            color.adjust_hue(amount);
            Ok(format_hsla(&color, false))
        }
        v => Ok(v.to_string()),
    }
}
