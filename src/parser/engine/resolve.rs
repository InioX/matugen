use chumsky::span::SimpleSpan;

use super::Engine;

use crate::{
    color::parse::parse_css_color,
    parser::{
        engine::{format_color, FORMATS},
        Error, ParseErrorKind, Value,
    },
};

impl Engine {
    pub fn resolve_generic_color<'a>(
        &self,
        color: &Value,
        format: &'a str,
        format_value: bool,
        span: SimpleSpan,
        name: &str,
    ) -> Result<Value, Error> {
        let color = match parse_css_color(&color.to_string()) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::ResolveError {
                    span,
                    name: name.to_string(),
                })
            }
        };
        if format_value {
            let res = match format_color(color, format) {
                Some(v) => v,
                None => {
                    return Err(Error::ParseError {
                        kind: ParseErrorKind::Keyword(crate::parser::KeywordError::InvalidFormat {
                            formats: FORMATS,
                        }),
                        span,
                        name: name.to_string(),
                    })
                }
            };

            Ok(Value::Ident(res.to_string()))
        } else {
            Ok(Value::Color(color))
        }
    }

    pub fn resolve_path<'a, I>(
        &self,
        path: I,
        format_value: bool,
        span: SimpleSpan,
        name: &str,
    ) -> Result<Value, Error>
    where
        I: IntoIterator<Item = &'a str> + Clone,
    {
        let mut iter = path.clone().into_iter().peekable();
        let first = iter.next().ok_or(Error::ResolveError {
            span,
            name: name.to_string(),
        })?;

        let mut current = self
            .runtime
            .borrow()
            .resolve_path(std::iter::once(first))
            .or_else(|| self.context.data().get(first).cloned())
            .ok_or(Error::ResolveError {
                span,
                name: name.to_string(),
            })?;

        while let Some(next_key) = iter.next() {
            let next_key = if next_key.starts_with("_") {
                next_key.strip_prefix("_").unwrap()
            } else {
                next_key
            };

            match current {
                Value::Map(ref map) => {
                    if map.contains_key("color") {
                        let color = map.get("color").unwrap();
                        current =
                            self.resolve_generic_color(color, next_key, format_value, span, name)?;
                    } else {
                        current = map
                            .get(next_key)
                            .ok_or(Error::ResolveError {
                                span,
                                name: name.to_string(),
                            })?
                            .clone();
                    }
                }
                Value::LazyColor { color, .. } => {
                    current = if format_value {
                        Value::Ident(
                            format_color(color, next_key)
                                .ok_or(Error::ResolveError {
                                    span,
                                    name: name.to_string(),
                                })?
                                .to_string(),
                        )
                    } else {
                        Value::Color(color)
                    }
                }
                Value::Color(color) => {
                    current = if format_value {
                        Value::Ident(
                            format_color(color, next_key)
                                .ok_or(Error::ResolveError {
                                    span,
                                    name: name.to_string(),
                                })?
                                .to_string(),
                        )
                    } else {
                        Value::Color(color)
                    }
                }
                _ => {
                    return Err(Error::ResolveError {
                        span,
                        name: name.to_string(),
                    });
                }
            }
        }

        Ok(current)
    }

    pub fn get_format<'a>(&self, keywords: &[&'a str]) -> &'a str {
        keywords
            .last()
            .expect("Could not get format from {keywords}")
    }
}
