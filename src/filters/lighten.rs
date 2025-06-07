use colorsys::{ColorAlpha, ColorTransform, Rgb};
use material_colors::color::Argb;

use crate::engine::{Engine, FilterReturnType, Value};

pub(crate) fn lighten(
    keywords: &Vec<&str>,
    args: Vec<Value>,
    original: FilterReturnType,
    engine: &Engine,
) -> FilterReturnType {
    match &original {
        FilterReturnType::String(v) => println!("{}", v),
        FilterReturnType::Color(v) => println!("{}", v),
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
        FilterReturnType::String(s) => FilterReturnType::String(s.to_uppercase()),
        FilterReturnType::Color(argb) => {
            let mut color = Rgb::from((argb.red, argb.green, argb.blue));
            color.lighten(amt);
            FilterReturnType::Color(Argb {
                alpha: color.alpha() as u8,
                red: color.red() as u8,
                green: color.green() as u8,
                blue: color.blue() as u8,
            })
        }
    }
}
