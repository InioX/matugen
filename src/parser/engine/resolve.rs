use super::Engine;

use crate::{
    color::parse::parse_css_color,
    parser::{engine::format_color, Value},
};

impl Engine {
    pub fn resolve_generic_color<'a>(
        &self,
        color: &Value,
        format: &'a str,
        format_value: bool,
    ) -> Option<Value> {
        let color = match parse_css_color(&color.to_string()) {
            Ok(v) => v,
            Err(_) => return None,
        };
        if format_value {
            Some(Value::Ident(format_color(color, format)?.to_string()))
        } else {
            Some(Value::Color(color))
        }
    }

    pub fn resolve_path<'a, I>(&self, path: I, format_value: bool) -> Option<Value>
    where
        I: IntoIterator<Item = &'a str> + Clone,
    {
        let mut iter = path.clone().into_iter().peekable();
        let first = iter.next()?;

        let mut current = self
            .runtime
            .borrow()
            .resolve_path(std::iter::once(first))
            .or_else(|| self.context.data().get(first).cloned())?;

        while let Some(next_key) = iter.next() {
            match current {
                Value::Map(ref map) => {
                    if map.contains_key("color") {
                        let color = map.get("color").unwrap();
                        current = self
                            .resolve_generic_color(color, next_key, format_value)
                            .unwrap();
                    } else {
                        current = map.get(next_key)?.clone();
                    }
                }
                Value::LazyColor { color, .. } => {
                    current = if format_value {
                        Value::Ident(format_color(color, next_key)?.to_string())
                    } else {
                        Value::Color(color)
                    }
                }
                Value::Color(color) => {
                    current = if format_value {
                        Value::Ident(format_color(color, next_key)?.to_string())
                    } else {
                        Value::Color(color)
                    }
                }
                _ => {
                    return None;
                }
            }
        }

        Some(current)
    }

    pub fn get_format<'a>(&self, keywords: &[&'a str]) -> &'a str {
        keywords
            .last()
            .expect("Could not get format from {keywords}")
    }
}
