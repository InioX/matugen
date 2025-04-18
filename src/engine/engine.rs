use std::cell::RefCell;
use std::collections::HashMap;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use chumsky::prelude::*;
use chumsky::span::SimpleSpan;

use colorsys::{ColorAlpha, ColorTransform, Hsl, Rgb};
use material_colors::color::Argb;
use matugen::{
    color::format::{
        format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba,
        rgb_from_argb,
    },
    scheme::{Schemes, SchemesEnum},
};

use crate::engine::Value;

use super::context::Context;

#[derive(Debug, Clone)]
pub(crate) enum Expression<'src> {
    Keyword {
        keywords: Vec<&'src str>,
    },
    Filter {
        name: &'src str,
        args: Vec<Value>,
    },
    KeywordWithFilters {
        keyword: Box<Expression<'src>>,
        filters: Vec<Expression<'src>>,
    },
}

#[derive(Debug)]
struct SpannedExpr<'src> {
    expr: Expression<'src>,
    span: SimpleSpan,
}

pub(crate) struct Engine {
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

pub(crate) enum FilterType {
    String(String),
    Color(Argb),
}

impl ToString for FilterType {
    fn to_string(&self) -> String {
        match self {
            FilterType::String(value) => format!("{}", value),
            FilterType::Color(argb) => todo!(),
        }
    }
}

impl From<String> for FilterType {
    fn from(value: String) -> Self {
        FilterType::String(value)
    }
}

impl From<&String> for FilterType {
    fn from(value: &String) -> Self {
        FilterType::String(value.to_string())
    }
}

impl From<i64> for FilterType {
    fn from(value: i64) -> Self {
        FilterType::String(value.to_string())
    }
}

impl From<&i64> for FilterType {
    fn from(value: &i64) -> Self {
        FilterType::String(value.to_string())
    }
}

impl From<f64> for FilterType {
    fn from(value: f64) -> Self {
        FilterType::String(value.to_string())
    }
}

impl From<&f64> for FilterType {
    fn from(value: &f64) -> Self {
        FilterType::String(value.to_string())
    }
}

impl From<bool> for FilterType {
    fn from(value: bool) -> Self {
        match value {
            true => return FilterType::String(String::from("true")),
            false => return FilterType::String(String::from("false")),
        }
    }
}

impl From<&bool> for FilterType {
    fn from(value: &bool) -> Self {
        match value {
            true => return FilterType::String(String::from("true")),
            false => return FilterType::String(String::from("false")),
        }
    }
}

impl From<Argb> for FilterType {
    fn from(value: Argb) -> Self {
        FilterType::Color(value)
    }
}

impl From<&Argb> for FilterType {
    fn from(value: &Argb) -> Self {
        FilterType::Color(*value)
    }
}

impl From<Value> for FilterType {
    fn from(value: Value) -> Self {
        match value {
            Value::Ident(v) => v.into(),
            Value::Int(v) => v.into(),
            Value::Float(v) => v.into(),
            Value::Color(v) => v.into(),
            Value::Bool(v) => v.into(),
            Value::Map(_hash_map) => panic!("Cant convert map to FilterType"),
            Value::Object(_hash_map) => panic!("Cant convert Object to FilterType"),
        }
    }
}

impl From<&Value> for FilterType {
    fn from(value: &Value) -> Self {
        match value {
            Value::Ident(v) => v.into(),
            Value::Int(v) => v.into(),
            Value::Float(v) => v.into(),
            Value::Color(v) => v.into(),
            Value::Bool(v) => v.into(),
            Value::Map(_hash_map) => panic!("Cant convert map to FilterType"),
            Value::Object(_hash_map) => panic!("Cant convert Object to FilterType"),
        }
    }
}

pub type FilterFn = fn(&Vec<&str>, Vec<Value>, FilterType, &Engine) -> FilterType;

fn set_red(
    keywords: &Vec<&str>,
    args: Vec<Value>,
    original: FilterType,
    engine: &Engine,
) -> FilterType {
    match &original {
        FilterType::String(v) => println!("{}", v),
        FilterType::Color(v) => println!("{}", v),
    }

    let amt = if args.len() >= 1 {
        match args[0] {
            Value::Int(v) => v as f64,
            Value::Float(v) => v,
            _ => panic!("Invalid argument type"),
        }
    } else {
        panic!("Not enough arguments")
    };

    match original {
        FilterType::String(s) => FilterType::String(s.to_uppercase()),
        FilterType::Color(argb) => {
            let mut color = Rgb::from((argb.red, argb.green, argb.blue));
            color.set_red(amt);
            FilterType::Color(Argb {
                alpha: color.alpha() as u8,
                red: color.red() as u8,
                green: color.green() as u8,
                blue: color.blue() as u8,
            })
        }
    }
}

fn lighten(
    keywords: &Vec<&str>,
    args: Vec<Value>,
    original: FilterType,
    engine: &Engine,
) -> FilterType {
    match &original {
        FilterType::String(v) => println!("{}", v),
        FilterType::Color(v) => println!("{}", v),
    }

    let amt = if args.len() >= 1 {
        match args[0] {
            Value::Int(v) => v as f64,
            Value::Float(v) => v,
            _ => panic!("Invalid argument type"),
        }
    } else {
        panic!("Not enough arguments")
    };

    match original {
        FilterType::String(s) => FilterType::String(s.to_uppercase()),
        FilterType::Color(argb) => {
            let mut color = Rgb::from((argb.red, argb.green, argb.blue));
            color.lighten(amt);
            FilterType::Color(Argb {
                alpha: color.alpha() as u8,
                red: color.red() as u8,
                green: color.green() as u8,
                blue: color.blue() as u8,
            })
        }
    }
}

fn darken(
    keywords: &Vec<&str>,
    args: Vec<Value>,
    original: FilterType,
    engine: &Engine,
) -> FilterType {
    match &original {
        FilterType::String(v) => println!("{}", v),
        FilterType::Color(v) => println!("{}", v),
    }

    let amt = if args.len() >= 1 {
        match args[0] {
            Value::Int(v) => v as f64,
            Value::Float(v) => v,
            _ => panic!("Invalid argument type"),
        }
    } else {
        panic!("Not enough arguments")
    };

    match original {
        FilterType::String(s) => FilterType::String(s.to_uppercase()),
        FilterType::Color(argb) => {
            let mut color = Rgb::from((argb.red, argb.green, argb.blue));
            color.lighten(-amt);
            FilterType::Color(Argb {
                alpha: color.alpha() as u8,
                red: color.red() as u8,
                green: color.green() as u8,
                blue: color.blue() as u8,
            })
        }
    }
}

// fn shuffle_color(keywords: &Vec<&str>, args: Vec<Value>, engine: &Engine) -> FilterType {
//     let
//     let color = engine.get_from_map_check_modified(r#type, name, colorscheme, format, modified_colors)
// }

// fn trim(keyword: Box<Expression<'_>>, args: Vec<Value>) -> String {
//     input.trim().to_string()
// }

impl Engine {
    pub fn new<T: Into<String>>(src: T, schemes: Schemes, default_scheme: SchemesEnum) -> Self {
        let mut filters: HashMap<&str, FilterFn> = HashMap::new();
        filters.insert("lighten", lighten);
        filters.insert("darken", darken);
        filters.insert("set_red", set_red);

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
                let keywords = match *keyword {
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

            self.format_color(color, format).into()
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

    pub fn format_color(
        &self,
        color: &material_colors::color::Argb,
        format: &str,
    ) -> impl Into<String> {
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

    fn get_replacement_filter(
        &self,
        keywords: Vec<&str>,
        filters: Vec<Expression>,
    ) -> impl Into<String> {
        let mut current_value = if keywords[0] == "colors" {
            let (r#type, name, colorscheme, format) = self.get_color_parts(&keywords);
            let modified_colors = self.modified_colors.borrow();
            FilterType::Color(*self.get_from_map_check_modified(
                r#type,
                name,
                colorscheme,
                format,
                &modified_colors,
            ))
        } else {
            // Support string filters too
            FilterType::from(self.resolve_path(keywords.clone()).unwrap())
        };

        for filter in filters {
            if let Expression::Filter {
                name: filtername,
                args,
            } = filter
            {
                let (r#type, name, colorscheme, format) = self.get_color_parts(&keywords);

                current_value = {
                    let modified_colors = self.modified_colors.borrow();

                    self.apply_filter(
                        filtername,
                        args,
                        &keywords,
                        r#type,
                        name,
                        colorscheme,
                        format,
                        &modified_colors,
                    )
                };

                // Update the cache if color
                if let FilterType::Color(argb) = current_value {
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
            FilterType::String(val) => val.into(),
            FilterType::Color(argb) => self.format_color(&argb, keywords[3]).into(),
        }
    }

    fn apply_filter(
        &self,
        filtername: &str,
        args: Vec<Value>,
        keywords: &Vec<&str>,
        r#type: &str,
        name: &str,
        colorscheme: &str,
        format: &str,
        modified_colors: &ColorCache,
    ) -> FilterType {
        let original = if r#type == "colors" {
            let color = self.get_from_map_check_modified(
                r#type,
                name,
                colorscheme,
                format,
                modified_colors,
            );
            FilterType::Color(*color)
        } else {
            FilterType::String(String::from("a"))
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
            .map(|v| Expression::Keyword { keywords: v });

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

        let arg = float.or(int).or(ident);

        let filter = text::ident()
            .then(
                just(':')
                    .padded()
                    .ignore_then(
                        arg.padded()
                            .separated_by(just(',').padded())
                            .collect::<Vec<Value>>(),
                    )
                    .or_not(),
            )
            .map(|(name, args)| Expression::Filter {
                name,
                args: args.unwrap_or_default(),
            });

        let filters = just('|')
            .padded()
            .ignore_then(filter.padded())
            .repeated()
            .collect::<Vec<_>>();

        let full_expr = dotted_ident.then(filters).map(|(expr, filters)| {
            if filters.is_empty() {
                expr
            } else {
                Expression::KeywordWithFilters {
                    keyword: Box::new(expr),
                    filters,
                }
            }
        });

        let keyword_full = just(self.syntax.keyword_left)
            .padded()
            .ignore_then(full_expr)
            .padded()
            .then_ignore(just(self.syntax.keyword_right))
            .map_with(|expr, e| SpannedExpr {
                expr,
                span: e.span(),
            });

        keyword_full.padded().repeated().collect()
    }
}
