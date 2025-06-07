use colorsys::{ColorAlpha, Rgb};
use material_colors::color::Argb;

use crate::engine::{Engine, FilterType, Value};

pub(crate) fn set_red(
    keywords: &Vec<&str>,
    args: Vec<Value>,
    original: FilterType,
    engine: &Engine,
) -> FilterType {
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
        FilterType::String(s) => panic!("Cannot use color filters on strings ({})", s),
        FilterType::Color(argb) => {
            let mut color = Rgb::from((argb.red, argb.green, argb.blue));
            color.set_red(amt);
            FilterType::Color(Argb {
                alpha: color.alpha() as u8,
                red: color.red() as u8,
                green: color.green() as u8,
                blue: color.blue() as u8,
            })
        }
    }
}

pub(crate) fn set_green(
    keywords: &Vec<&str>,
    args: Vec<Value>,
    original: FilterType,
    engine: &Engine,
) -> FilterType {
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
        FilterType::String(s) => panic!("Cannot use color filters on strings ({})", s),
        FilterType::Color(argb) => {
            let mut color = Rgb::from((argb.red, argb.green, argb.blue));
            color.set_green(amt);
            FilterType::Color(Argb {
                alpha: color.alpha() as u8,
                red: color.red() as u8,
                green: color.green() as u8,
                blue: color.blue() as u8,
            })
        }
    }
}

pub(crate) fn set_blue(
    keywords: &Vec<&str>,
    args: Vec<Value>,
    original: FilterType,
    engine: &Engine,
) -> FilterType {
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
        FilterType::String(s) => panic!("Cannot use color filters on strings ({})", s),
        FilterType::Color(argb) => {
            let mut color = Rgb::from((argb.red, argb.green, argb.blue));
            color.set_blue(amt);
            FilterType::Color(Argb {
                alpha: color.alpha() as u8,
                red: color.red() as u8,
                green: color.green() as u8,
                blue: color.blue() as u8,
            })
        }
    }
}
