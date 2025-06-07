use material_colors::color::Argb;

use crate::engine::{Engine, Value};

pub enum FilterReturnType {
    String(String),
    Color(Argb),
}

pub type FilterFn = fn(&Vec<&str>, Vec<Value>, FilterReturnType, &Engine) -> FilterReturnType;

impl ToString for FilterReturnType {
    fn to_string(&self) -> String {
        match self {
            FilterReturnType::String(value) => format!("{}", value),
            FilterReturnType::Color(argb) => todo!(),
        }
    }
}

impl From<String> for FilterReturnType {
    fn from(value: String) -> Self {
        FilterReturnType::String(value)
    }
}

impl From<&String> for FilterReturnType {
    fn from(value: &String) -> Self {
        FilterReturnType::String(value.to_string())
    }
}

impl From<i64> for FilterReturnType {
    fn from(value: i64) -> Self {
        FilterReturnType::String(value.to_string())
    }
}

impl From<&i64> for FilterReturnType {
    fn from(value: &i64) -> Self {
        FilterReturnType::String(value.to_string())
    }
}

impl From<f64> for FilterReturnType {
    fn from(value: f64) -> Self {
        FilterReturnType::String(value.to_string())
    }
}

impl From<&f64> for FilterReturnType {
    fn from(value: &f64) -> Self {
        FilterReturnType::String(value.to_string())
    }
}

impl From<bool> for FilterReturnType {
    fn from(value: bool) -> Self {
        match value {
            true => return FilterReturnType::String(String::from("true")),
            false => return FilterReturnType::String(String::from("false")),
        }
    }
}

impl From<&bool> for FilterReturnType {
    fn from(value: &bool) -> Self {
        match value {
            true => return FilterReturnType::String(String::from("true")),
            false => return FilterReturnType::String(String::from("false")),
        }
    }
}

impl From<Argb> for FilterReturnType {
    fn from(value: Argb) -> Self {
        FilterReturnType::Color(value)
    }
}

impl From<&Argb> for FilterReturnType {
    fn from(value: &Argb) -> Self {
        FilterReturnType::Color(*value)
    }
}

impl From<Value> for FilterReturnType {
    fn from(value: Value) -> Self {
        match value {
            Value::Ident(v) => v.into(),
            Value::Int(v) => v.into(),
            Value::Float(v) => v.into(),
            Value::Color(v) => v.into(),
            Value::Bool(v) => v.into(),
            Value::Map(_hash_map) => panic!("Cant convert map to FilterReturnType"),
            Value::Object(_hash_map) => panic!("Cant convert Object to FilterReturnType"),
        }
    }
}

impl From<&Value> for FilterReturnType {
    fn from(value: &Value) -> Self {
        match value {
            Value::Ident(v) => v.into(),
            Value::Int(v) => v.into(),
            Value::Float(v) => v.into(),
            Value::Color(v) => v.into(),
            Value::Bool(v) => v.into(),
            Value::Map(_hash_map) => panic!("Cant convert map to FilterReturnType"),
            Value::Object(_hash_map) => panic!("Cant convert Object to FilterReturnType"),
        }
    }
}
