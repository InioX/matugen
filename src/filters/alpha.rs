use upon::Value;

use colorsys::{ColorAlpha, Hsl, Rgb};
use std::str::FromStr;

use crate::color::{
    parse::{parse_color, check_string_value},
    format::{format_rgba, format_hsla},
};

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
