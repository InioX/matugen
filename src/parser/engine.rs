use std::cell::RefCell;
use std::collections::HashMap;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use chumsky::prelude::*;
use chumsky::span::SimpleSpan;
use serde_json::json;

use crate::{
    parser::{context::RuntimeContext, filtertype::FilterFn, SpannedValue},
    scheme::{Schemes, SchemesEnum},
};

use material_colors::color::Argb;

use super::context::Context;

use crate::parser::Value;

mod replace;
pub(crate) use replace::*;

mod resolve;
pub(crate) use resolve::*;

#[derive(Debug, Clone)]
enum Expression<'src> {
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
pub struct SpannedExpr<'src> {
    expr: Expression<'src>,
    span: SimpleSpan,
}

pub struct Engine<'src> {
    filters: HashMap<&'static str, FilterFn>,
    syntax: EngineSyntax,
    schemes: Schemes,
    default_scheme: SchemesEnum,
    context: Context,
    runtime: RefCell<RuntimeContext>,
    templates: HashMap<String, Template<'src>>,
}

#[derive(Debug)]
pub struct Template<'src> {
    pub name: String,
    pub source: String,
    pub ast: Vec<Box<SpannedExpr<'src>>>,
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

impl<'src> Engine<'src> {
    pub fn new(schemes: Schemes, default_scheme: SchemesEnum) -> Self {
        let mut filters: HashMap<&str, FilterFn> = HashMap::new();

        let ctx = Context::new();

        Self {
            filters,
            syntax: EngineSyntax {
                keyword_left: ['{', '{'],
                keyword_right: ['}', '}'],
                block_left: ['<', '*'],
                block_right: ['*', '>'],
            },
            schemes,
            default_scheme,
            context: ctx.clone(),
            runtime: RuntimeContext::new(ctx.clone()).into(),
            templates: HashMap::new(),
        }
    }

    pub fn add_filter(&mut self, name: &'static str, function: FilterFn) -> Option<FilterFn> {
        self.filters.insert(name, function)
    }
    pub fn remove_filter(&mut self, name: &'static str) -> Option<FilterFn> {
        self.filters.remove(name)
    }

    pub fn add_template(&mut self, name: &'src str, source: &'src str) {
        let (ast, errs) = self.parser().parse(source.trim()).into_output_errors();

        self.templates.insert(
            name.to_string(),
            Template {
                name: name.to_string(),
                ast: {
                    match ast {
                        Some(v) => v,
                        None => {
                            self.show_errors(errs, source);
                            std::process::exit(1)
                        }
                    }
                },
                source: source.to_owned(),
            },
        );
    }

    pub fn remove_template(&mut self, name: &'src str) -> bool {
        match self.templates.remove(name) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn add_context(&mut self, context: serde_json::Value) {
        self.context.merge_json(context);
    }

    pub fn render(&self, name: &'src str) -> String {
        self.generate_template(self.templates.get(name).expect("Failed to get template"))
    }

    fn parser(
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

    fn show_errors(&self, errs: Vec<Rich<'_, char>>, source: &'src str) {
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
                .print(Source::from(source))
                .unwrap();
        });
    }
}
