use std::cell::RefCell;
use std::collections::HashMap;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use chumsky::prelude::*;
use chumsky::span::SimpleSpan;

use crate::{
    color::format::{
        format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba,
        rgb_from_argb,
    },
    engine::{
        filtertype::{emit_filter_error, FilterFn, FilterReturnType},
        FilterError, SpannedValue,
    },
    scheme::{Schemes, SchemesEnum},
};
use colorsys::{ColorAlpha, ColorTransform, Hsl, Rgb};
use material_colors::color::Argb;

use crate::engine::Value;

use super::context::Context;

#[derive(Debug, Clone)]
pub(crate) enum Expression<'src> {
    Keyword {
        keywords: Vec<&'src str>,
    },
    Filter {
        name: &'src str,
        args: Vec<SpannedValue>,
    },
    KeywordWithFilters {
        keyword: Box<SpannedExpr<'src>>,
        filters: Vec<SpannedExpr<'src>>,
    },
}

#[derive(Debug, Clone)]
struct SpannedExpr<'src> {
    expr: Expression<'src>,
    span: SimpleSpan,
}

pub struct Engine {
    src: String,
    filters: HashMap<&'static str, FilterFn>,
    syntax: EngineSyntax,
    schemes: Schemes,
    default_scheme: SchemesEnum,
    modified_colors: RefCell<ColorCache>,
    context: Context,
}

pub(crate) struct ColorCache {
    pub dark: HashMap<String, Argb>,
    pub light: HashMap<String, Argb>,
}

pub(crate) struct EngineSyntax {
    keyword_left: [char; 2],
    keyword_right: [char; 2],
    block_left: [char; 2],
    block_right: [char; 2],
}

pub fn format_color(color: &material_colors::color::Argb, format: &str) -> impl Into<String> {
    let base_color = rgb_from_argb(*color);
    let hsl_color = Hsl::from(&base_color);

    match format {
        "hex" => format_hex(&base_color),
        "hex_stripped" => format_hex_stripped(&base_color),
        "rgb" => format_rgb(&base_color),
        "rgba" => format_rgba(&base_color, true),
        "hsl" => format_hsl(&hsl_color),
        "hsla" => format_hsla(&hsl_color, true),
        "red" => format!("{:?}", base_color.red() as u8),
        "green" => format!("{:?}", base_color.green() as u8),
        "blue" => format!("{:?}", base_color.blue() as u8),
        "alpha" => format!("{:?}", base_color.alpha() as u8),
        "hue" => format!("{:?}", &hsl_color.hue()),
        "saturation" => format!("{:?}", &hsl_color.lightness()),
        "lightness" => format!("{:?}", &hsl_color.saturation()),
        _ => panic!("Invalid format"),
    }
}

impl Engine {
    pub fn new<T: Into<String>>(src: T, schemes: Schemes, default_scheme: SchemesEnum) -> Self {
        let mut filters: HashMap<&str, FilterFn> = HashMap::new();

        // Setting individual values
        filters.insert("lighten", crate::filters::lighten);
        filters.insert("darken", crate::filters::darken);

        filters.insert("set_red", crate::filters::set_red);
        filters.insert("set_green", crate::filters::set_green);
        filters.insert("set_blue", crate::filters::set_blue);

        let mut ctx = Context::new();

        let mut inner_map = HashMap::new();
        inner_map.insert("name".to_string(), Value::Ident("test".to_string()));

        let mut outer_map = HashMap::new();
        outer_map.insert("user".to_string(), Value::Object(inner_map));

        ctx.merge(&outer_map);

        // filters.insert("trim", trim);

        Self {
            src: src.into(),
            filters,
            syntax: EngineSyntax {
                keyword_left: ['{', '{'],
                keyword_right: ['}', '}'],
                block_left: ['<', '*'],
                block_right: ['*', '>'],
            },
            schemes,
            default_scheme,
            modified_colors: ColorCache {
                dark: HashMap::new(),
                light: HashMap::new(),
            }
            .into(),
            context: ctx,
        }
    }

    pub fn resolve_path<'a, I>(&'a self, path: I) -> Option<&'a Value>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut iter = path.into_iter();

        let next = iter.next();

        let mut current = self.context.data().get(next?);

        for key in iter {
            current = match current {
                Some(Value::Object(map)) => map.get(key),
                _ => {
                    eprintln!("Could not find {}", key);
                    return None;
                }
            };
        }

        current
    }

    pub fn generate_templates(&self) {
        let (res, errs) = self.parser().parse(self.src.trim()).into_output_errors();

        let mut changed_src: &mut String = &mut self.src.clone();

        match res {
            Some(exprs) => {
                for e in exprs.into_iter().rev() {
                    self.get_keyword(&mut changed_src, e);
                }
            }
            None => {}
        }

        println!("==================\n{}", changed_src);
        self.show_errors(errs);
    }

    fn show_errors(&self, errs: Vec<Rich<'_, char>>) {
        errs.into_iter().for_each(|e| {
            Report::build(ReportKind::Error, ((), e.span().into_range()))
                .with_config(ariadne::Config::default().with_index_type(ariadne::IndexType::Byte))
                .with_message(e.to_string())
                .with_label(
                    Label::new(((), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .print(Source::from(&self.src))
                .unwrap();
        });
    }

    fn get_keyword(&self, src: &mut String, expr: SpannedExpr) {
        let range = expr.span.into_range();

        match expr.expr {
            Expression::Keyword { keywords } => {
                src.replace_range(range, &self.get_replacement(keywords));
            }
            Expression::KeywordWithFilters { keyword, filters } => {
                let keywords = match keyword.expr {
                    Expression::Keyword { keywords } => keywords,
                    _ => panic!(""),
                };

                src.replace_range(
                    range,
                    &self.get_replacement_filter(keywords, filters).into(),
                );
            }
            _ => {
                println!("other")
            }
        }
    }

    fn get_replacement(&self, keywords: Vec<&str>) -> String {
        if keywords[0] == "colors" {
            let (r#type, name, colorscheme, format) = self.get_color_parts(&keywords);
            let color = self.get_from_map(r#type, name, colorscheme, format);
            let format = &keywords[3];

            format_color(color, format).into()
        } else {
            String::from(self.resolve_path(keywords).unwrap())
        }
    }

    fn validate_color_parts(&self, keywords: &Vec<&str>) -> bool {
        if keywords.len() == 0 || keywords.len() > 4 {
            false
        } else {
            true
        }
    }

    // fn get_from_context<'a>(&self, keywords: &Vec<&'a str>) -> Option<Value> {
    //     let mut current = self.context.get(keywords[0])?;

    //     for key in &keywords[1..] {
    //         current = match current {
    //             Value::Object(map) => map.get(*key)?,
    //             _ => return None,
    //         };
    //     }

    //     Some(Value::Str(String::from("a")))
    // }

    fn get_color_parts<'a>(&self, keywords: &Vec<&'a str>) -> (&'a str, &'a str, &'a str, &'a str) {
        if self.validate_color_parts(keywords) == false {
            panic!(
                "{}",
                format!("Keyword length invalid: {:?}", keywords.len())
            );
        }

        (keywords[0], keywords[1], keywords[2], keywords[3])
    }

    pub fn get_from_map_check_modified<'a>(
        &'a self,
        r#type: &str,
        name: &str,
        colorscheme: &str,
        format: &str,
        modified_colors: &'a ColorCache,
    ) -> &'a Argb {
        match colorscheme {
            "light" => match modified_colors.light.get(name) {
                Some(v) => return v,
                None => {},
            },
            "dark" => match modified_colors.dark.get(name) {
                Some(v) => return v,
                None => {},
            },
            "default" => match self.default_scheme {
                SchemesEnum::Light => match modified_colors.light.get(name) {
                    Some(v) => return v,
                    None => {},
                },
                SchemesEnum::Dark => match modified_colors.dark.get(name) {
                    Some(v) => return v,
                    None => {},
                }
            },
            _ => panic!("{}", format!("Invalid color mode {:?}. The color mode can only be one of: [dark, light, default]", colorscheme))
        };

        self.get_from_map(r#type, name, colorscheme, format)
    }

    pub fn get_from_map(&self, r#type: &str, name: &str, colorscheme: &str, format: &str) -> &Argb {
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

            return scheme.get(name).unwrap();
        } else {
            todo!()
        }
    }

    fn get_replacement_filter(
        &self,
        keywords: Vec<&str>,
        filters: Vec<SpannedExpr>,
    ) -> impl Into<String> {
        let mut current_value = if keywords[0] == "colors" {
            let (r#type, name, colorscheme, format) = self.get_color_parts(&keywords);
            let modified_colors = self.modified_colors.borrow();
            FilterReturnType::Color(*self.get_from_map_check_modified(
                r#type,
                name,
                colorscheme,
                format,
                &modified_colors,
            ))
        } else {
            // Support string filters too
            FilterReturnType::from(self.resolve_path(keywords.clone()).unwrap())
        };

        for filter in filters {
            if let Expression::Filter {
                name: filtername,
                args,
            } = filter.expr
            {
                let (r#type, name, colorscheme, format) = self.get_color_parts(&keywords);

                current_value = {
                    let modified_colors = self.modified_colors.borrow();

                    match self.apply_filter(
                        filtername,
                        args,
                        &keywords,
                        r#type,
                        name,
                        colorscheme,
                        format,
                        &modified_colors,
                    ) {
                        Ok(v) => v,
                        Err(e) => {
                            emit_filter_error("test", &self.src, &e.kind, filter.span);
                            std::process::exit(1);
                        }
                    }
                };

                // Update the cache if color
                if let FilterReturnType::Color(argb) = current_value {
                    match colorscheme {
                        "dark" => {
                            self.modified_colors
                                .borrow_mut()
                                .dark
                                .insert(name.to_owned(), argb);
                        }
                        "light" => {
                            self.modified_colors
                                .borrow_mut()
                                .light
                                .insert(name.to_owned(), argb);
                        }
                        "default" => match self.default_scheme {
                            SchemesEnum::Dark => {
                                self.modified_colors
                                    .borrow_mut()
                                    .dark
                                    .insert(name.to_owned(), argb);
                            }
                            SchemesEnum::Light => {
                                self.modified_colors
                                    .borrow_mut()
                                    .light
                                    .insert(name.to_owned(), argb);
                            }
                        },
                        _ => panic!("Invalid color scheme"),
                    };
                }
            }
        }

        match current_value {
            FilterReturnType::String(val) => val.into(),
            FilterReturnType::Color(argb) => format_color(&argb, keywords[3]).into(),
        }
    }

    fn apply_filter(
        &self,
        filtername: &str,
        args: Vec<SpannedValue>,
        keywords: &Vec<&str>,
        r#type: &str,
        name: &str,
        colorscheme: &str,
        format: &str,
        modified_colors: &ColorCache,
    ) -> Result<FilterReturnType, FilterError> {
        let original = if r#type == "colors" {
            let color = self.get_from_map_check_modified(
                r#type,
                name,
                colorscheme,
                format,
                modified_colors,
            );
            FilterReturnType::Color(*color)
        } else {
            FilterReturnType::String(String::from("a"))
        };

        match self.filters.get(filtername) {
            Some(f) => return f(keywords, args, original, &self),
            None => panic!("{}", format!("Could not find filter {:?}", filtername)),
        };
    }

    fn add_filter(&mut self, name: &'static str, function: FilterFn) {}
    fn remove_filter(&mut self) {}

    fn parser<'src>(
        &self,
    ) -> impl Parser<'src, &'src str, Vec<SpannedExpr<'src>>, extra::Err<Rich<'src, char>>> {
        let dotted_ident = text::ident()
            .separated_by(just('.'))
            .at_least(1)
            .collect::<Vec<&'src str>>()
            .map_with(|v, e| SpannedExpr {
                expr: Expression::Keyword { keywords: v },
                span: e.span(),
            });

        let float = text::int(10)
            .then_ignore(just('.'))
            .then(text::int(10))
            .map(|(int_part, frac_part)| {
                let parsed = format!("{}.{}", int_part, frac_part)
                    .parse::<f64>()
                    .unwrap();
                Value::Float(parsed)
            });

        let int = text::int(10).map(|s: &str| Value::Int(s.parse::<i64>().unwrap()));

        let ident = text::ident().map(|s: &str| Value::Ident(s.to_string()));

        let arg = float
            .or(int)
            .or(ident)
            .map_with(|value, e| SpannedValue::new(value, e.span()));

        let filter = text::ident()
            .then(
                just(':')
                    .padded()
                    .ignore_then(
                        arg.padded()
                            .separated_by(just(',').padded())
                            .collect::<Vec<SpannedValue>>(),
                    )
                    .or_not(),
            )
            .map_with(|(name, args), e| SpannedExpr {
                expr: Expression::Filter {
                    name,
                    args: args.unwrap_or_default(),
                },
                span: e.span(),
            });

        let filters = just('|')
            .padded()
            .ignore_then(filter.padded())
            .repeated()
            .collect::<Vec<_>>();

        let full_expr = dotted_ident.then(filters).map(|(keyword, filters)| {
            if filters.is_empty() {
                keyword
            } else {
                let span = SimpleSpan::new(
                    (),
                    keyword.span.start
                        ..filters
                            .last()
                            .map(|f| f.span.end)
                            .unwrap_or(keyword.span.end),
                );
                dbg!(&span);
                SpannedExpr {
                    expr: Expression::KeywordWithFilters {
                        keyword: Box::new(keyword),
                        filters,
                    },
                    span,
                }
            }
        });

        let keyword_full = just(self.syntax.keyword_left)
            .padded()
            .ignore_then(full_expr)
            .padded()
            .then_ignore(just(self.syntax.keyword_right))
            .map_with(|expr, e| SpannedExpr {
                expr: expr.expr,
                span: e.span(),
            });

        keyword_full.padded().repeated().collect()
    }
}
