use std::cell::RefCell;
use std::collections::HashMap;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use chumsky::prelude::*;
use chumsky::span::SimpleSpan;
use serde_json::json;

use crate::{
    color::format::{
        format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba,
        rgb_from_argb,
    },
    engine::{
        context::RuntimeContext,
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
    ForLoop {
        var: Vec<SpannedValue>,
        list: Box<SpannedExpr<'src>>,
        body: Vec<Box<SpannedExpr<'src>>>,
    },
    Raw {
        value: String,
    },
}

impl<'src> Expression<'src> {
    pub fn as_keywords(&self) -> Option<&Vec<&'src str>> {
        if let Expression::Keyword { keywords } = self {
            Some(keywords)
        } else {
            None
        }
    }
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
    runtime: RefCell<RuntimeContext>,
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

pub fn format_color_all(color: &material_colors::color::Argb) -> HashMap<String, Value> {
    let base_color = rgb_from_argb(*color);
    let hsl_color = Hsl::from(&base_color);

    let mut map = HashMap::new();

    map.insert("hex".to_string(), Value::Ident(format_hex(&base_color)));
    map.insert(
        "hex_stripped".to_string(),
        Value::Ident(format_hex_stripped(&base_color)),
    );
    map.insert("rgb".to_string(), Value::Ident(format_rgb(&base_color)));
    map.insert(
        "rgba".to_string(),
        Value::Ident(format_rgba(&base_color, true)),
    );
    map.insert("hsl".to_string(), Value::Ident(format_hsl(&hsl_color)));
    map.insert(
        "hsla".to_string(),
        Value::Ident(format_hsla(&hsl_color, true)),
    );
    map.insert(
        "red".to_string(),
        Value::Ident(format!("{:?}", base_color.red() as u8)),
    );
    map.insert(
        "green".to_string(),
        Value::Ident(format!("{:?}", base_color.green() as u8)),
    );
    map.insert(
        "blue".to_string(),
        Value::Ident(format!("{:?}", base_color.blue() as u8)),
    );
    map.insert(
        "alpha".to_string(),
        Value::Ident(format!("{:?}", base_color.alpha() as u8)),
    );
    map.insert(
        "hue".to_string(),
        Value::Ident(format!("{:?}", &hsl_color.hue())),
    );
    map.insert(
        "saturation".to_string(),
        Value::Ident(format!("{:?}", &hsl_color.lightness())),
    );
    map.insert(
        "lightness".to_string(),
        Value::Ident(format!("{:?}", &hsl_color.saturation())),
    );

    map
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

        ctx.merge_json(json!({
            "user": {
                "name": "test",
                "pets": {
                    "dog": {
                        "name": "Paw"
                    },
                    "cat": {
                        "name": "Spotty"
                    },
                }
            },
        }));

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
            context: ctx.clone(),
            runtime: RuntimeContext::new(ctx.clone()).into(),
        }
    }

    pub fn resolve_path<'a, I>(&self, path: I) -> Option<Value>
    where
        I: IntoIterator<Item = &'a str> + Clone,
    {
        let mut iter = path.clone().into_iter().peekable();

        if let Some(&first) = iter.peek() {
            let mut color_map: HashMap<String, Value> = HashMap::new();

            if first == "colors" {
                let subkeys: Vec<&str> = iter.collect();

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

        while let Some(next_key) = iter.next() {
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

    pub fn generate_templates(&self) {
        let (res, errs) = self.parser().parse(self.src.trim()).into_output_errors();

        let mut changed_src: String = String::new();

        match res {
            Some(exprs) => {
                self.build_string(&mut changed_src, exprs);
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

    fn build_string(&self, src: &mut String, exprs: Vec<Box<SpannedExpr>>) {
        for expr in exprs.into_iter() {
            let _range = expr.span.into_range();

            self.eval(src, expr);
        }
    }

    fn eval(&self, src: &mut String, expr: Box<SpannedExpr>) {
        match expr.expr {
            Expression::Keyword { keywords } => {
                src.push_str(&self.get_replacement(keywords));
            }
            Expression::KeywordWithFilters { keyword, filters } => {
                let keywords = match keyword.expr {
                    Expression::Keyword { keywords } => keywords,
                    _ => panic!(""),
                };

                src.push_str(&self.get_replacement_filter(keywords, filters).into());
            }
            Expression::Raw { value } => {
                src.push_str(&value);
            }
            Expression::ForLoop { var, list, body } => {
                let values = match list.expr.as_keywords() {
                    Some(v) => self.resolve_path(v.iter().copied()),
                    None => unreachable!(),
                }
                .unwrap();

                match values {
                    Value::Map(map) => {
                        for (key, value) in map {
                            self.runtime.borrow_mut().push_scope();

                            if var.len() == 1 {
                                self.runtime
                                    .borrow_mut()
                                    .insert(var[0].value.clone(), Value::Ident(key.clone()));
                            } else if var.len() == 2 {
                                self.runtime
                                    .borrow_mut()
                                    .insert(var[0].value.clone(), Value::Ident(key.clone()));
                                self.runtime
                                    .borrow_mut()
                                    .insert(var[1].value.clone(), value.clone());
                            } else {
                                panic!("for-loop supports only one or two variables");
                            }

                            // Evaluate the body with these bindings
                            src.push_str(&self.eval_loop_body(body.clone()));

                            self.runtime.borrow_mut().pop_scope();
                        }
                    }
                    Value::Array(arr) => {
                        for item in arr {
                            self.runtime.borrow_mut().push_scope();

                            if var.len() == 1 {
                                self.runtime
                                    .borrow_mut()
                                    .insert(var[0].value.clone(), item.clone());
                            } else {
                                panic!("for-loop over list supports only one variable");
                            }

                            src.push_str(&self.eval_loop_body(body.clone()));
                            self.runtime.borrow_mut().pop_scope();
                        }
                    }
                    _ => {
                        panic!("Cannot loop over non-iterable value");
                    }
                }
            }
            _ => {}
        }
    }

    fn eval_loop_body(&self, exprs: Vec<Box<SpannedExpr>>) -> String {
        let mut output = String::from("");

        for expr in exprs.into_iter() {
            let _range = expr.span.into_range();
            self.eval(&mut output, expr);
        }

        output
    }

    fn get_replacement(&self, keywords: Vec<&str>) -> String {
        if keywords[0] == "colors" {
            let (r#type, name, colorscheme, format) = self.get_color_parts(&keywords);
            let color = self.get_from_map(r#type, name, colorscheme);
            let format = &keywords[3];

            format_color(color, format).into()
        } else {
            String::from(self.resolve_path(keywords).unwrap())
        }
    }

    fn validate_color_parts(&self, keywords: &Vec<&str>) -> bool {
        if keywords.len() == 0 || keywords.len() > 4 || keywords.len() < 4 {
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

    fn get_color_parts_partial<'a>(
        &self,
        keywords: &Vec<&'a str>,
    ) -> (
        Option<&'a str>,
        Option<&'a str>,
        Option<&'a str>,
        Option<&'a str>,
    ) {
        (
            keywords.get(0).copied(),
            keywords.get(1).copied(),
            keywords.get(2).copied(),
            keywords.get(3).copied(),
        )
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

        self.get_from_map(r#type, name, colorscheme)
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
    ) -> impl Parser<'src, &'src str, Vec<Box<SpannedExpr<'src>>>, extra::Err<Rich<'src, char>>>
    {
        recursive(|expr| {
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

            let spanned_ident = ident.map_with(|value, e| SpannedValue::new(value, e.span()));

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
                .map_with(|expr, e| {
                    Box::new(SpannedExpr {
                        expr: expr.expr,
                        span: e.span(),
                    })
                });

            let for_end = just("endfor")
                .padded()
                .delimited_by(just(self.syntax.block_left), just(self.syntax.block_right));

            let raw = any()
                .and_is(
                    just(self.syntax.keyword_left[0])
                        .or(just(self.syntax.block_left[0]))
                        .not(),
                )
                .repeated()
                .at_least(1)
                .collect::<String>()
                .map_with(|text, span| {
                    Box::new(SpannedExpr {
                        expr: Expression::Raw {
                            value: text, // or use Box::leak(text.into_boxed_str())
                        },
                        span: span.span(),
                    })
                });

            let for_loop = just(self.syntax.block_left)
                .map_with(|expr, e| (expr, e.span()))
                .padded()
                .ignore_then(just("for"))
                .padded()
                .ignore_then(
                    spanned_ident
                        .padded()
                        .separated_by(just(","))
                        .at_least(1)
                        .collect::<Vec<SpannedValue>>(),
                )
                .padded()
                .then_ignore(just("in"))
                .padded()
                .then(dotted_ident)
                .padded()
                .then_ignore(just(self.syntax.block_right))
                .padded()
                .then(raw.or(expr).repeated().collect())
                .then_ignore(for_end)
                .map_with(|((var, list), body), e| {
                    Box::new(SpannedExpr {
                        expr: Expression::ForLoop {
                            var,
                            list: Box::new(list),
                            body,
                        },
                        span: e.span(),
                    })
                });

            raw.or(keyword_full).or(for_loop)
        })
        .repeated()
        .collect::<Vec<Box<SpannedExpr<'src>>>>()
    }
}
