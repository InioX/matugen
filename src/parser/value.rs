use std::{fmt::Display, str::FromStr};

use chumsky::span::SimpleSpan;
use material_colors::color::Argb;

#[derive(Debug, Clone)]
pub enum Value {
    Ident(String),
    Int(i64),
    Float(f64),
    Color(material_colors::color::Argb),
    LazyColor {
        color: Argb,
        scheme: Option<String>, // If known, otherwise None
    },
    Bool(bool),
    Map(std::collections::HashMap<String, Value>),
    Array(Vec<Value>),
    Null,
}

#[derive(Debug, Clone)]
pub struct SpannedValue {
    pub value: Value,
    pub span: SimpleSpan,
}

impl SpannedValue {
    pub fn new(value: Value, span: SimpleSpan) -> Self {
        Self {
            value,
            span,
        }
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

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Value::Bool(val)
    }
}

impl From<&Value> for String {
    fn from(value: &Value) -> Self {
        match value {
            Value::Ident(v) => v.to_string(),
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
            Value::Color(_v) => unreachable!(),
            Value::Map(_hash_map) => panic!("Cant convert map to String"),
            Value::Array(_array) => panic!("Cant convert Array to String"),
            Value::Null => String::from("Null"),
            Value::LazyColor { color, scheme } => todo!(),
        }
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::Ident(v) => v.to_string(),
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
            Value::Color(_v) => unreachable!(),
            Value::LazyColor { color, scheme } => unreachable!(),
            Value::Map(_hash_map) => {
                panic!("Cant convert map to String")
            }
            Value::Array(_array) => panic!("Cant convert Array to String"),
            Value::Null => String::from("Null"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
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
            Value::LazyColor { color, scheme } => "Color",
            Value::Map(_) => "Map",
            Value::Null => "Null",
            Value::Array(_) => "Array",
        }
        .to_string()
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
                if let Ok(color) = Argb::from_str(&s) {
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
