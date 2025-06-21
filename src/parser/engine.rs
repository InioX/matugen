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

mod resolve;
pub(crate) use resolve::*;

mod replace;
pub(crate) use replace::*;

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
            context: ctx.clone(),
            runtime: RuntimeContext::new(ctx.clone()).into(),
        }
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
