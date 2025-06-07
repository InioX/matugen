pub mod context;
pub mod engine;
pub mod filtertype;
pub mod value;

#[derive(Debug, Clone)]
pub enum Value {
    Ident(String),
    Int(i64),
    Float(f64),
    Color(material_colors::color::Argb),
    Bool(bool),
    Map(std::collections::HashMap<String, Value>),
    Object(std::collections::HashMap<String, Value>),
}

pub use engine::Engine;
pub use filtertype::FilterFn;
pub use filtertype::FilterType;
