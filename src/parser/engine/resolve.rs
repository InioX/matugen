use chumsky::span::SimpleSpan;
use indexmap::IndexMap;
use material_colors::color::Argb;

use super::Engine;

use crate::{
    color::format::rgb_from_argb,
    parser::{engine::format_color_all, Error, KeywordError, ParseErrorKind, Value},
};

use crate::scheme::SchemesEnum;

impl Engine {
    pub fn resolve_path<'a, I>(&self, path: I) -> Option<Value>
    where
        I: IntoIterator<Item = &'a str> + Clone,
    {
        let mut iter = path.clone().into_iter().peekable();

        if let Some(&first) = iter.peek() {
            let mut color_map: IndexMap<String, Value> = IndexMap::new();

            if first == "colors" {
                iter.next()?;

                if let Some(color_name) = iter.next() {
                    if let Some(scheme_name) = iter.next() {
                        if let Some(format_name) = iter.next() {
                            let scheme = match scheme_name {
                                "light" => &self.schemes.light,
                                "dark" => &self.schemes.dark,
                                "default" => match self.default_scheme {
                                    SchemesEnum::Light => &self.schemes.light,
                                    SchemesEnum::Dark => &self.schemes.dark,
                                },
                                _ => return None,
                            };

                            let color = rgb_from_argb(*scheme.get(color_name)?);
                            let formats = format_color_all(color);
                            return formats.get(format_name).cloned();
                        }

                        let scheme = match scheme_name {
                            "light" => &self.schemes.light,
                            "dark" => &self.schemes.dark,
                            "default" => match self.default_scheme {
                                SchemesEnum::Light => &self.schemes.light,
                                SchemesEnum::Dark => &self.schemes.dark,
                            },
                            _ => return None,
                        };

                        let color = scheme.get(color_name)?;
                        return Some(Value::LazyColor {
                            color: rgb_from_argb(*color),
                            scheme: Some(scheme_name.to_string()),
                        });
                    }

                    let mut scheme_map = IndexMap::new();
                    for (scheme_name, scheme) in [
                        ("light", &self.schemes.light),
                        ("dark", &self.schemes.dark),
                        (
                            "default",
                            match self.default_scheme {
                                SchemesEnum::Light => &self.schemes.light,
                                SchemesEnum::Dark => &self.schemes.dark,
                            },
                        ),
                    ] {
                        if let Some(color) = scheme.get(color_name) {
                            scheme_map.insert(
                                scheme_name.to_string(),
                                Value::LazyColor {
                                    color: rgb_from_argb(*color),
                                    scheme: Some(scheme_name.to_string()),
                                },
                            );
                        }
                    }
                    return Some(Value::Map(scheme_map));
                }

                for name in self.schemes.get_all_names() {
                    let mut scheme_map = IndexMap::new();

                    let default_scheme = match self.default_scheme {
                        SchemesEnum::Light => self.schemes.light.clone(),
                        SchemesEnum::Dark => self.schemes.dark.clone(),
                    };

                    for (scheme_name, scheme) in [
                        ("light", self.schemes.light.clone()),
                        ("dark", self.schemes.dark.clone()),
                        ("default", default_scheme),
                    ] {
                        if let Some(color) = scheme.get(name) {
                            scheme_map.insert(
                                scheme_name.to_string(),
                                Value::LazyColor {
                                    color: rgb_from_argb(*color),
                                    scheme: Some(scheme_name.to_string()),
                                },
                            );
                        }
                    }
                    color_map.insert(name.clone(), Value::Map(scheme_map));
                }

                return Some(Value::Map(color_map));
            }
        }

        let first = iter.next()?;

        let mut current = self
            .runtime
            .borrow()
            .resolve_path(std::iter::once(first))
            .or_else(|| self.context.data().get(first).cloned())?;

        for next_key in iter {
            match current {
                Value::Map(ref map) => {
                    current = map.get(next_key)?.clone();
                }
                Value::LazyColor { color, .. } => {
                    let color_map = format_color_all(color);
                    current = Value::Ident(color_map.get(next_key)?.into());
                }
                Value::Color(argb) => {
                    // convert to map and keep walking
                    let color_map = format_color_all(argb);
                    current = Value::Ident(color_map.get(next_key)?.clone().into());
                }
                _ => {
                    return None;
                }
            }
        }

        Some(current)
    }

    pub fn resolve_path_filter<'a, I>(&self, path: I) -> Option<Value>
    where
        I: IntoIterator<Item = &'a str> + Clone,
    {
        let mut iter = path.clone().into_iter().peekable();

        if let Some(&first) = iter.peek() {
            let local_val = self
                .runtime
                .borrow()
                .resolve_path(std::iter::once(first))
                .or_else(|| self.context.data().get(first).cloned());

            if let Some(mut current) = local_val {
                for next_key in iter.skip(1) {
                    match current {
                        Value::Map(ref map) => {
                            current = map.get(next_key)?.clone();
                        }
                        Value::LazyColor { color, .. } => {
                            current = Value::Color(color);
                        }
                        Value::Color(color) => {
                            current = Value::Color(color);
                        }
                        _ => return None,
                    }
                }

                return match current {
                    Value::Color(c) => Some(Value::Color(c)),
                    Value::LazyColor { color, .. } => Some(Value::Color(color)),
                    _ => Some(current),
                };
            }
        }

        let mut iter = path.into_iter().peekable();
        if let Some(&"colors") = iter.peek() {
            iter.next()?; // consume "colors"
            let color_name = iter.next()?;
            let scheme_name = iter.next()?;
            let _format = iter.next(); // ignore

            let scheme = match scheme_name {
                "light" => &self.schemes.light,
                "dark" => &self.schemes.dark,
                "default" => match self.default_scheme {
                    SchemesEnum::Light => &self.schemes.light,
                    SchemesEnum::Dark => &self.schemes.dark,
                },
                _ => return None,
            };

            let color = scheme.get(color_name)?;
            return Some(Value::Color(rgb_from_argb(*color)));
        }

        None
    }

    fn validate_color_parts(&self, keywords: &[&str]) -> bool {
        !(keywords.is_empty() || keywords.len() > 4 || keywords.len() < 4)
    }

    pub(crate) fn get_color_parts<'a>(
        &self,
        keywords: &[&'a str],
        span: SimpleSpan,
    ) -> (&'a str, &'a str, &'a str, &'a str) {
        if !self.validate_color_parts(keywords) {
            self.errors.add(Error::ParseError {
                kind: crate::parser::ParseErrorKind::Keyword(KeywordError::InvalidColorDefinition),
                span,
            });
            return ("colors", "source_color", "default", "hex");
        }

        (keywords[0], keywords[1], keywords[2], keywords[3])
    }

    pub fn get_format<'a>(&self, keywords: &[&'a str]) -> &'a str {
        keywords
            .last()
            .expect("Could not get format from {keywords}")
    }

    pub fn get_from_map(
        &self,
        r#type: &str,
        name: &str,
        colorscheme: &str,
        span: SimpleSpan,
    ) -> &Argb {
        if r#type == "colors" {
            let mut scheme = &self.schemes.dark;

            scheme = match colorscheme {
                "light" => &self.schemes.light,
                "dark" => &self.schemes.dark,
                "default" => match self.default_scheme {
                    SchemesEnum::Light => &self.schemes.light,
                    SchemesEnum::Dark => &self.schemes.dark,
                },
                _ => {
                    self.errors.add(Error::ParseError {
                        kind: ParseErrorKind::Keyword(KeywordError::InvalidScheme),
                        span,
                    });
                    &self.schemes.dark
                }
            };

            match scheme.get(name) {
                Some(v) => v,
                None => {
                    self.errors.add(Error::ParseError {
                        kind: crate::parser::ParseErrorKind::Keyword(
                            crate::parser::KeywordError::ColorDoesNotExist,
                        ),
                        span,
                    });
                    &Argb {
                        alpha: 0,
                        red: 0,
                        green: 0,
                        blue: 0,
                    }
                }
            }
        } else {
            unreachable!()
        }
    }
}
