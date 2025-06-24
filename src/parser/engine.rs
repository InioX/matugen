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
enum Expression {
    Keyword {
        keywords: Vec<SimpleSpan>,
    },
    Filter {
        name: SimpleSpan,
        args: Vec<SpannedValue>,
    },
    KeywordWithFilters {
        keyword: Box<SpannedExpr>,
        filters: Vec<SpannedExpr>,
    },
    ForLoop {
        var: Vec<SpannedValue>,
        list: Box<SpannedExpr>,
        body: Vec<Box<SpannedExpr>>,
    },
    Raw {
        value: SimpleSpan,
    },
}

impl Expression {
    pub fn as_keywords<'a>(&self, source: &'a str) -> Option<Vec<&'a str>> {
        if let Expression::Keyword { keywords } = self {
            Some(get_str_vec(source, keywords))
        } else {
            None
        }
    }
    pub fn as_spans<'a>(&self) -> Option<&Vec<SimpleSpan>> {
        if let Expression::Keyword { keywords } = self {
            Some(keywords)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpannedExpr {
    expr: Expression,
    span: SimpleSpan,
}

pub struct Engine {
    filters: HashMap<&'static str, FilterFn>,
    syntax: EngineSyntax,
    schemes: Schemes,
    default_scheme: SchemesEnum,
    context: Context,
    runtime: RefCell<RuntimeContext>,
    templates: HashMap<String, Template>,
    sources: Vec<String>,
}

pub struct Template {
    pub name: String,
    pub source_id: usize, // Index into `Engine.sources`
    pub ast: Vec<Box<SpannedExpr>>,
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

impl Engine {
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
            sources: vec![],
        }
    }

    pub fn add_filter(&mut self, name: &'static str, function: FilterFn) -> Option<FilterFn> {
        self.filters.insert(name, function)
    }
    pub fn remove_filter(&mut self, name: &'static str) -> Option<FilterFn> {
        self.filters.remove(name)
    }

    pub fn add_template(&mut self, name: String, source: String) {
        self.sources.push(source);
        let source_id = self.sources.len() - 1;
        let source_ref = &self.sources[source_id];

        let parser = Self::parser(&self.syntax);

        let (ast, errs) = parser.parse(source_ref.trim()).into_output_errors();

        self.templates.insert(
            name.clone(),
            Template {
                name,
                source_id,
                ast: ast.unwrap_or_else(|| {
                    self.show_errors(errs, source_ref);
                    std::process::exit(1);
                }),
            },
        );
    }

    pub fn remove_template(&mut self, name: String) -> bool {
        match self.templates.remove(&name) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn add_context(&mut self, context: serde_json::Value) {
        self.context.merge_json(context);
    }

    pub fn render(&self, name: &str) -> String {
        self.generate_template(self.templates.get(name).expect("Failed to get template"))
    }

    pub fn parser<'src>(
        syntax: &'src EngineSyntax,
    ) -> impl Parser<'src, &'src str, Vec<Box<SpannedExpr>>, extra::Err<Rich<'src, char>>> {
        recursive(|expr| {
            // Dotted identifier as a sequence of spans
            let dotted_ident = text::ident()
                .map_with(|_, e| e.span())
                .separated_by(just('.').padded())
                .at_least(1)
                .collect::<Vec<SimpleSpan>>()
                .map_with(|spans, e| SpannedExpr {
                    expr: Expression::Keyword { keywords: spans },
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

            // Filter: name is span, args are spanned values
            let filter = text::ident()
                .map_with(|_, e| e.span())
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

            let keyword_full = just(syntax.keyword_left)
                .padded()
                .ignore_then(full_expr)
                .padded()
                .then_ignore(just(syntax.keyword_right))
                .map_with(|expr, e| {
                    Box::new(SpannedExpr {
                        expr: expr.expr,
                        span: e.span(),
                    })
                });

            let for_end = just("endfor")
                .padded()
                .delimited_by(just(syntax.block_left), just(syntax.block_right));

            let raw = any()
                .and_is(
                    just(syntax.keyword_left[0])
                        .or(just(syntax.block_left[0]))
                        .not(),
                )
                .repeated()
                .at_least(1)
                .collect::<String>()
                .map_with(|_, span| {
                    Box::new(SpannedExpr {
                        expr: Expression::Raw { value: span.span() },
                        span: span.span(),
                    })
                });

            let for_loop = just(syntax.block_left)
                .padded()
                .ignore_then(just("for"))
                .padded()
                .ignore_then(
                    spanned_ident
                        .separated_by(just(',').padded())
                        .at_least(1)
                        .collect::<Vec<SpannedValue>>(),
                )
                .padded()
                .then_ignore(just("in"))
                .padded()
                .then(dotted_ident)
                .padded()
                .then_ignore(just(syntax.block_right))
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
        .collect::<Vec<Box<SpannedExpr>>>()
    }

    fn show_errors(&self, errs: Vec<Rich<'_, char>>, source: &str) {
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
