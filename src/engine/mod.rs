pub mod context;
pub mod engine;
pub mod value;

#[derive(Debug, Clone)]
pub enum Value {
    Ident(String),
    Int(i64),
    Float(f64),
    Color(crate::Argb),
    Bool(bool),
    Map(std::collections::HashMap<String, Value>),
    Object(std::collections::HashMap<String, Value>),
}
