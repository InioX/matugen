use material_colors::color::Argb;

use super::Engine;

use crate::{
    parser::{
        engine::{format_color, format_color_all},
        Value,
    },
    scheme::SchemesEnum,
};
use std::collections::HashMap;

impl Engine {
    pub fn resolve_path<'a, I>(&self, path: I) -> Option<Value>
    where
        I: IntoIterator<Item = &'a str> + Clone,
    {
        let mut iter = path.clone().into_iter().peekable();

        if let Some(&first) = iter.peek() {
            let mut color_map: HashMap<String, Value> = HashMap::new();

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

                            let color = scheme.get(color_name)?;
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
                            color: *color,
                            scheme: Some(scheme_name.to_string()),
                        });
                    }

                    let mut scheme_map = HashMap::new();
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
                                    color: *color,
                                    scheme: Some(scheme_name.to_string()),
                                },
                            );
                        }
                    }
                    return Some(Value::Map(scheme_map));
                }

                for name in self.schemes.get_all_names() {
                    let mut scheme_map = HashMap::new();

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
                                    color: *color,
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
                    let color_map = format_color_all(&color);
                    current = Value::Ident(color_map.get(next_key)?.into());
                }
                Value::Color(argb) => {
                    // convert to map and keep walking
                    let color_map = format_color_all(&argb);
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

        if let Some(&"colors") = iter.peek() {
            iter.next()?;

            let color_name = iter.next()?;
            let scheme_name = iter.next()?;

            let scheme = match scheme_name {
                "light" => &self.schemes.light,
                "dark" => &self.schemes.dark,
                "default" => match self.default_scheme {
                    SchemesEnum::Light => &self.schemes.light,
                    SchemesEnum::Dark => &self.schemes.dark,
                },
                _ => return None,
            };

            return scheme.get(color_name).copied().map(Value::Color);
        }

        self.resolve_path(path)
    }

    pub(crate) fn get_replacement(&self, keywords: Vec<&str>) -> String {
        if keywords[0] == "colors" {
            let (r#type, name, colorscheme, format) = self.get_color_parts(&keywords);
            let color = self.get_from_map(r#type, name, colorscheme);
            let format = &keywords[3];

            format_color(color, format).into()
        } else {
            String::from(self.resolve_path(keywords).unwrap())
        }
    }

    fn validate_color_parts(&self, keywords: &[&str]) -> bool {
        !(keywords.is_empty() || keywords.len() > 4 || keywords.len() < 4)
    }

    fn get_color_parts<'a>(&self, keywords: &Vec<&'a str>) -> (&'a str, &'a str, &'a str, &'a str) {
        if !self.validate_color_parts(keywords) {
            panic!(
                "{}",
                format!("Keyword length invalid: {:?}", keywords.len())
            );
        }

        (keywords[0], keywords[1], keywords[2], keywords[3])
    }

    pub fn get_format<'a>(&self, keywords: &[&'a str]) -> &'a str {
        keywords[3]
    }

    pub fn get_from_map(&self, r#type: &str, name: &str, colorscheme: &str) -> &Argb {
        if r#type == "colors" {
            // Just to check if the color exists, we get the color later
            let mut scheme = &self.schemes.dark;

            if !scheme.contains_key(name) {
                panic!("{}", format!("The color {:?} does not exist.", name));
            }

            scheme = match colorscheme {
                "light" => &self.schemes.light,
                "dark" => &self.schemes.dark,
                "default" => match self.default_scheme {
                    SchemesEnum::Light => &self.schemes.light,
                    SchemesEnum::Dark => &self.schemes.dark,
                },
                _ => panic!("{}", format!("Invalid color mode {:?}. The color mode can only be one of: [dark, light, default]", colorscheme))
            };

            scheme.get(name).unwrap()
        } else {
            todo!()
        }
    }
}
