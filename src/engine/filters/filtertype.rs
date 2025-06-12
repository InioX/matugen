use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::span::SimpleSpan;
use material_colors::color::Argb;
use upon::filters::Filter;

use crate::engine::{Engine, SpannedValue, Value};

#[derive(Debug)]
pub struct FilterError {
    pub kind: FilterErrorKind,
}

#[derive(Debug)]
pub enum FilterErrorKind {
    NotEnoughArguments,
    InvalidArgumentType {
        span: SimpleSpan,
        expected: String,
        actual: String,
    },
    ColorFilterOnString,
}

impl FilterError {
    pub fn new(kind: FilterErrorKind) -> Self {
        Self { kind }
    }
}

pub fn emit_filter_error(
    source_id: &str,
    source_code: &str,
    kind: &FilterErrorKind,
    span: SimpleSpan,
) {
    let (message, span, name) = match kind {
        FilterErrorKind::NotEnoughArguments => (
            "Not enough arguments provided for filter",
            span,
            "NotEnoughArguments",
        ),
        FilterErrorKind::InvalidArgumentType {
            span,
            expected,
            actual,
        } => ("a", *span, "InvalidArgumentType"),
        FilterErrorKind::ColorFilterOnString => (
            "Cannot use color filters on a string filter, consider using the 'to_color' filter",
            span,
            "ColorFilterOnString",
        ),
    };
    Report::build(ReportKind::Error, ((), span.into_range()))
        .with_config(ariadne::Config::default().with_index_type(ariadne::IndexType::Byte))
        .with_message(name)
        .with_label(
            Label::new(((), span.into_range()))
                .with_message(message)
                .with_color(Color::Red),
        )
        .finish()
        .print(Source::from(&source_code))
        .unwrap();
}

pub enum FilterReturnType {
    String(String),
    Color(Argb),
}
pub struct SpannedFilterReturnType {
    value: FilterReturnType,
    span: SimpleSpan,
}

pub type FilterFn = fn(
    &Vec<&str>,
    Vec<SpannedValue>,
    FilterReturnType,
    &Engine,
) -> Result<FilterReturnType, FilterError>;

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
