use colorsys::{ColorAlpha, ColorTransform, Rgb};
use material_colors::color::Argb;

use crate::engine::{Engine, FilterType, Value};

pub(crate) fn darken(
    keywords: &Vec<&str>,
    args: Vec<Value>,
    original: FilterType,
    engine: &Engine,
) -> FilterType {
    match &original {
        FilterType::String(v) => println!("{}", v),
        FilterType::Color(v) => println!("{}", v),
    }

    let amt = if args.len() >= 1 {
        match args[0] {
            Value::Int(v) => v as f64,
            Value::Float(v) => v,
            _ => panic!("Invalid argument type"),
        }
    } else {
        panic!("Not enough arguments")
    };

    match original {
        FilterType::String(s) => FilterType::String(s.to_uppercase()),
        FilterType::Color(argb) => {
            let mut color = Rgb::from((argb.red, argb.green, argb.blue));
            color.lighten(-amt);
            FilterType::Color(Argb {
                alpha: color.alpha() as u8,
                red: color.red() as u8,
                green: color.green() as u8,
                blue: color.blue() as u8,
            })
        }
    }
}
