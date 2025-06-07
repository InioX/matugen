use material_colors::color::Argb;

use crate::engine::{Engine, Value};

pub enum FilterType {
    String(String),
    Color(Argb),
}

pub type FilterFn = fn(&Vec<&str>, Vec<Value>, FilterType, &Engine) -> FilterType;

impl ToString for FilterType {
    fn to_string(&self) -> String {
        match self {
            FilterType::String(value) => format!("{}", value),
            FilterType::Color(argb) => todo!(),
        }
    }
}

impl From<String> for FilterType {
    fn from(value: String) -> Self {
        FilterType::String(value)
    }
}

impl From<&String> for FilterType {
    fn from(value: &String) -> Self {
        FilterType::String(value.to_string())
    }
}

impl From<i64> for FilterType {
    fn from(value: i64) -> Self {
        FilterType::String(value.to_string())
    }
}

impl From<&i64> for FilterType {
    fn from(value: &i64) -> Self {
        FilterType::String(value.to_string())
    }
}

impl From<f64> for FilterType {
    fn from(value: f64) -> Self {
        FilterType::String(value.to_string())
    }
}

impl From<&f64> for FilterType {
    fn from(value: &f64) -> Self {
        FilterType::String(value.to_string())
    }
}

impl From<bool> for FilterType {
    fn from(value: bool) -> Self {
        match value {
            true => return FilterType::String(String::from("true")),
            false => return FilterType::String(String::from("false")),
        }
    }
}

impl From<&bool> for FilterType {
    fn from(value: &bool) -> Self {
        match value {
            true => return FilterType::String(String::from("true")),
            false => return FilterType::String(String::from("false")),
        }
    }
}

impl From<Argb> for FilterType {
    fn from(value: Argb) -> Self {
        FilterType::Color(value)
    }
}

impl From<&Argb> for FilterType {
    fn from(value: &Argb) -> Self {
        FilterType::Color(*value)
    }
}

impl From<Value> for FilterType {
    fn from(value: Value) -> Self {
        match value {
            Value::Ident(v) => v.into(),
            Value::Int(v) => v.into(),
            Value::Float(v) => v.into(),
            Value::Color(v) => v.into(),
            Value::Bool(v) => v.into(),
            Value::Map(_hash_map) => panic!("Cant convert map to FilterType"),
            Value::Object(_hash_map) => panic!("Cant convert Object to FilterType"),
        }
    }
}

impl From<&Value> for FilterType {
    fn from(value: &Value) -> Self {
        match value {
            Value::Ident(v) => v.into(),
            Value::Int(v) => v.into(),
            Value::Float(v) => v.into(),
            Value::Color(v) => v.into(),
            Value::Bool(v) => v.into(),
            Value::Map(_hash_map) => panic!("Cant convert map to FilterType"),
            Value::Object(_hash_map) => panic!("Cant convert Object to FilterType"),
        }
    }
}
