use colorsys::{Hsl, Rgb};

use crate::Value;

#[cfg(feature = "filter-docs")]
#[derive(serde::Serialize, Debug, Clone)]
pub struct FilterDoc {
    pub name: &'static str,
    pub category: &'static str,
    pub description: String,
    pub example: Option<String>,
    pub arguments: Vec<String>,
}

#[cfg(feature = "filter-docs")]
use std::sync::{Mutex, OnceLock};

#[cfg(feature = "filter-docs")]
pub static FILTER_DOCS: OnceLock<Mutex<Vec<FilterDoc>>> = OnceLock::new();

#[cfg(feature = "filter-docs")]
pub fn filter_docs() -> Vec<FilterDoc> {
    FILTER_DOCS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap()
        .clone()
}

#[cfg(feature = "filter-docs")]
#[macro_export]
macro_rules! __register_filter_doc {
    ($name:expr, $category:expr, $doc:expr) => {{
        let (description, example, arguments) = $crate::helpers::parse_doc_string($doc);

        $crate::helpers::FILTER_DOCS
            .get_or_init(|| std::sync::Mutex::new(Vec::new()))
            .lock()
            .unwrap()
            .push($crate::helpers::FilterDoc {
                name: $name,
                category: $category,
                description,
                example,
                arguments,
            });
    }};
}

#[cfg(not(feature = "filter-docs"))]
#[macro_export]
macro_rules! __register_filter_doc {
    ($name:expr, $category:expr, $doc:expr) => {};
}

#[macro_export]
macro_rules! register_filters {
    (($engine:expr) {
        $(
            $category:literal => {
                $(
                    $(#[doc = $doc:literal])*
                    $name:literal => $func:path
                ),* $(,)?
            }
        ),* $(,)?
    }) => {{
        $(
            $(
                $engine.add_filter($name, $func);

                $crate::__register_filter_doc!(
                    $name,
                    $category,
                    concat!($($doc, "\n"),*)
                );
            )*
        )*
    }};
}

// #[cfg(feature = "filter-docs")]
// pub fn filters_to_html() -> String {
//     let mut out = String::new();

//     for doc in filter_docs() {
//         out.push_str(&format!(
//             "<div class='filter-doc' data-type={}>
//     <h2>{}</h2>
//     {}
// </div>\n",
//             doc.category, doc.name, doc.description
//         ));
//     }

//     out
// }

#[cfg(feature = "filter-docs")]
pub fn parse_doc_string(raw: &str) -> (String, Option<String>, Vec<String>) {
    let blocks: Vec<&str> = raw
        .split("\n\n")
        .map(|b| b.trim())
        .filter(|b| !b.is_empty())
        .collect();

    let description = blocks.first().unwrap_or(&"").to_string();
    let example = blocks.get(1).map(|&s| s.to_string());

    let mut arguments = Vec::new();
    if let Some(arg_block) = blocks.get(2) {
        for line in arg_block.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                arguments.push(trimmed.to_string());
            }
        }
    }

    (description, example, arguments)
}

#[cfg(feature = "filter-docs")]
pub fn filters_to_json() -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&filter_docs())
}

#[macro_export]
macro_rules! expect_args {
    ($args:expr, $( $ty:ty ),* $(,)?) => {{
        let expected_len = [$(
            stringify!($ty)
        ),*].len();
        if $args.len() < expected_len {
            return Err(
                $crate::FilterError::NotEnoughArguments,
            );
        }

        let mut _i = 0;
        (
            $(
                {
                    let spanned = &$args[_i];
                    _i += 1;
                    match <$ty as $crate::helpers::ExpectFromValue>::expect_from(&spanned.value) {
                        Ok(v) => v,
                        Err(actual) => {
                            return Err(
                                $crate::FilterError::InvalidArgumentType {
                                    span: spanned.span,
                                    expected: stringify!($ty).to_string(),
                                    actual,
                                }
                            )
                    }}
                }
            ),*
        )
    }};
}

pub trait ExpectFromValue: Sized {
    fn expect_from(value: &Value) -> Result<Self, String>;
}

impl ExpectFromValue for String {
    fn expect_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::Ident(s) => Ok(s.clone()),
            other => Err(other.variant_name()),
        }
    }
}

impl ExpectFromValue for i64 {
    fn expect_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::Int(i) => Ok(*i),
            other => Err(other.variant_name()),
        }
    }
}

impl ExpectFromValue for f64 {
    fn expect_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::Float(f) => Ok(*f),
            Value::Int(i) => Ok(*i as f64),
            other => Err(other.variant_name()),
        }
    }
}

impl ExpectFromValue for Rgb {
    fn expect_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::Color(color) => Ok(color.clone()),
            Value::LazyColor { color, scheme: _ } => Ok(color.clone()),
            other => Err(other.variant_name()),
        }
    }
}

impl ExpectFromValue for Hsl {
    fn expect_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::HslColor(color) => Ok(color.clone()),
            other => Err(other.variant_name()),
        }
    }
}
