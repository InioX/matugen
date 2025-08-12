use std::{
    fmt::{self},
    str::FromStr,
};

use chumsky::span::SimpleSpan;
use colorsys::{Hsl, Rgb};
use indexmap::IndexMap;

use crate::parser::{engine::format_color_all, FilterReturnType};

#[derive(Debug, Clone)]
pub enum Value {
    Ident(String),
    Int(i64),
    Float(f64),
    Color(Rgb),
    HslColor(Hsl),
    LazyColor {
        color: Rgb,
        scheme: Option<String>, // If known, otherwise None
    },
    Bool(bool),
    Map(IndexMap<String, Value>),
    Array(Vec<Value>),
    Null,
}

pub enum ColorValue {
    Rgb(Rgb),
    Hsl(Hsl),
}

#[derive(Debug, Clone)]
pub struct SpannedValue {
    pub value: Value,
    pub span: SimpleSpan,
}

impl SpannedValue {
    pub fn new(value: Value, span: SimpleSpan) -> Self {
        Self { value, span }
    }
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Value::Ident(val.to_string())
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        Value::Ident(val)
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Int(val as i64)
    }
}

impl From<i32> for Value {
    fn from(val: i32) -> Self {
        Value::Int(val as i64)
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Value::Int(val)
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Value::Bool(val)
    }
}

impl From<FilterReturnType> for Value {
    fn from(value: FilterReturnType) -> Self {
        match value {
            FilterReturnType::String(s) => Value::Ident(s),
            FilterReturnType::Rgb(rgb) => Value::Color(rgb),
            FilterReturnType::Hsl(hsl) => Value::HslColor(hsl),
            FilterReturnType::Bool(b) => Value::Bool(b),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Ident(v) => write!(f, "{}", v),
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
            Value::Color(color) | Value::LazyColor { color, scheme: _ } => {
                let formats = format_color_all(color.clone());
                write!(f, "{:?}", formats)
            }
            Value::HslColor(color) => {
                let formats = format_color_all(color.clone().into());
                write!(f, "{:?}", formats)
            }
            Value::Map(v) => write!(f, "{:?}", v),
            Value::Array(v) => write!(f, "{:?}", v),
            Value::Null => write!(f, "Null"),
        }
    }
}

impl Value {
    pub fn variant_name(&self) -> String {
        match self {
            Value::Ident(_) => "String",
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::Bool(_) => "Bool",
            Value::Color(_) => "Color",
            Value::HslColor(_) => "Hsl Color",
            Value::LazyColor {
                color: _,
                scheme: _,
            } => "Color",
            Value::Map(_) => "Map",
            Value::Null => "Null",
            Value::Array(_) => "Array",
        }
        .to_string()
    }

    pub fn get_int(&self) -> Option<i64> {
        match self {
            Value::Int(v) => Some(*v),
            // Value::Float(_) => todo!(),
            _ => None,
        }
    }

    pub fn is_color(&self) -> bool {
        match self {
            Value::Color(_) => true,
            Value::LazyColor {
                color: _,
                scheme: _,
            } => true,
            Value::HslColor(_) => true,
            _ => false,
        }
    }

    pub fn get_color(self) -> Option<ColorValue> {
        match self {
            Value::Color(color) => Some(ColorValue::Rgb(color)),
            Value::LazyColor { color, scheme: _ } => Some(ColorValue::Rgb(color)),
            Value::HslColor(color) => Some(ColorValue::Hsl(color)),
            _ => None,
        }
    }
}

impl From<serde_json::Value> for Value {
    fn from(v: serde_json::Value) -> Self {
        match v {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Int(i)
                } else if let Some(f) = n.as_f64() {
                    Value::Float(f)
                } else {
                    panic!("Invalid number format");
                }
            }
            serde_json::Value::String(s) => {
                if let Ok(color) = Rgb::from_str(&s) {
                    Value::Color(color)
                } else {
                    Value::Ident(s)
                }
            }
            serde_json::Value::Array(arr) => {
                Value::Array(arr.into_iter().map(Value::from).collect())
            }
            serde_json::Value::Object(map) => {
                Value::Map(map.into_iter().map(|(k, v)| (k, Value::from(v))).collect())
            }
        }
    }
}
