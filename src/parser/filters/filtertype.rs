use colorsys::{Hsl, Rgb};

use crate::parser::{Engine, FilterError, SpannedValue, Value};

#[derive(Debug)]
pub enum FilterReturnType {
    String(String),
    Rgb(Rgb),
    Hsl(Hsl),
    Bool(bool),
}

pub type FilterFn = fn(
    &[&str],
    &[SpannedValue],
    FilterReturnType,
    &Engine,
) -> Result<FilterReturnType, FilterError>;

impl ToString for FilterReturnType {
    fn to_string(&self) -> String {
        match self {
            FilterReturnType::String(value) => value.to_string(),
            FilterReturnType::Rgb(_rgb) => unreachable!(),
            FilterReturnType::Hsl(_hsl) => unreachable!(),
            FilterReturnType::Bool(boolean) => match boolean {
                true => "true".to_owned(),
                false => "false".to_owned(),
            },
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
            true => FilterReturnType::String(String::from("true")),
            false => FilterReturnType::String(String::from("false")),
        }
    }
}

impl From<&bool> for FilterReturnType {
    fn from(value: &bool) -> Self {
        match value {
            true => FilterReturnType::String(String::from("true")),
            false => FilterReturnType::String(String::from("false")),
        }
    }
}

impl From<Rgb> for FilterReturnType {
    fn from(value: Rgb) -> Self {
        FilterReturnType::Rgb(value)
    }
}

impl From<&Rgb> for FilterReturnType {
    fn from(value: &Rgb) -> Self {
        FilterReturnType::Rgb(value.clone())
    }
}

impl From<Value> for FilterReturnType {
    fn from(value: Value) -> Self {
        match value {
            Value::Ident(v) => v.into(),
            Value::Int(v) => v.into(),
            Value::Float(v) => v.into(),
            Value::Color(v) => v.into(),
            Value::Bool(boolean) => Self::Bool(boolean),
            Value::Map(_hash_map) => panic!("Cant convert map to FilterReturnType"),
            Value::Array(_array) => panic!("Cant convert Array to String"),
            Value::Null => todo!(),
            Value::LazyColor { color, scheme } => FilterReturnType::from(Value::Color(color)),
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
            Value::Array(_array) => panic!("Cant convert Array to String"),
            Value::Null => todo!(),
            Value::LazyColor { color, scheme } => todo!(),
        }
    }
}
