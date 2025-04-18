use std::fmt::Display;

use crate::engine::Value;

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
            Value::Object(_hash_map) => panic!("Cant convert Object to String"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
