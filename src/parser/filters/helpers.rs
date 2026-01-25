use colorsys::{Hsl, Rgb};

use crate::parser::Value;

#[cfg(feature = "filter-docs")]
#[derive(Debug, Clone)]
pub struct FilterDoc {
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
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
        $crate::parser::helpers::FILTER_DOCS
            .get_or_init(|| std::sync::Mutex::new(Vec::new()))
            .lock()
            .unwrap()
            .push($crate::parser::helpers::FilterDoc {
                name: $name,
                category: $category,
                description: $doc,
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
pub fn filters_to_html() -> String {
    use std::collections::BTreeMap;

    let mut grouped: BTreeMap<&str, Vec<&FilterDoc>> = BTreeMap::new();
    let docs = filter_docs();

    for doc in docs.iter() {
        grouped.entry(doc.category).or_default().push(doc);
    }

    let mut out = String::new();

    for (category, docs) in grouped {
        out.push_str(&format!("<h2>{}</h2><md-divider></md-divider>\n", category));

        for doc in docs {
            out.push_str(&format!(
                "<div class='filter-doc'>
    <h3>{}</h3>
    {}
</div>\n",
                doc.name, doc.description
            ));
        }
    }

    out
}

#[macro_export]
macro_rules! expect_args {
    ($args:expr, $( $ty:ty ),* $(,)?) => {{
        let expected_len = [$(
            stringify!($ty)
        ),*].len();
        if $args.len() < expected_len {
            return Err(
                $crate::parser::FilterError::NotEnoughArguments,
            );
        }

        let mut _i = 0;
        (
            $(
                {
                    let spanned = &$args[_i];
                    _i += 1;
                    match <$ty as $crate::parser::helpers::ExpectFromValue>::expect_from(&spanned.value) {
                        Ok(v) => v,
                        Err(actual) => {
                            return Err(
                                $crate::parser::FilterError::InvalidArgumentType {
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
