use std::{cell::RefCell, collections::HashMap};

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{error::Rich, prelude::*, span::SimpleSpan};

use crate::{
    parser::{context::RuntimeContext, filtertype::FilterFn, Error, ErrorCollector, SpannedValue},
    scheme::{Schemes, SchemesEnum},
};

use super::context::Context;

use crate::parser::Value;

mod replace;
pub(crate) use replace::*;

mod resolve;

#[derive(Debug, Clone)]
enum Expression {
    Keyword {
        keywords: Vec<SimpleSpan>,
    },
    Filter {
        name: SimpleSpan,
        args: Vec<Box<SpannedExpr>>,
    },
    KeywordWithFilters {
        keyword: Box<SpannedExpr>,
        filters: Vec<SpannedExpr>,
    },
    ForLoop {
        var: Vec<SpannedValue>,
        iter: Box<SpannedExpr>,
        body: Vec<Box<SpannedExpr>>,
    },
    Raw {
        value: SimpleSpan,
    },
    Include {
        name: SpannedValue,
    },
    If {
        condition: Box<SpannedExpr>,
        then_branch: Vec<Box<SpannedExpr>>,
        else_branch: Option<Vec<Box<SpannedExpr>>>,
    },
    Range {
        start: i64,
        end: i64,
    },
    LiteralValue {
        value: SpannedValue,
    },
}

#[derive(Debug, Clone)]
enum FilterArgEnum {
    Value(SpannedValue),
    Expression(SpannedExpr),
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
    errors: ErrorCollector,
}

pub struct Template {
    pub name: String,
    pub source_id: usize, // Index into `Engine.sources`
    pub ast: Vec<Box<SpannedExpr>>,
}

pub(crate) struct EngineSyntax {
    keyword_left: [char; 2],
    keyword_right: [char; 2],
    block_left: [char; 2],
    block_right: [char; 2],
}

impl Default for EngineSyntax {
    fn default() -> Self {
        Self {
            keyword_left: ['{', '{'],
            keyword_right: ['}', '}'],
            block_left: ['<', '*'],
            block_right: ['*', '>'],
        }
    }
}

impl EngineSyntax {
    pub fn new(
        keyword_left: [char; 2],
        keyword_right: [char; 2],
        block_left: [char; 2],
        block_right: [char; 2],
    ) -> Self {
        Self {
            keyword_left,
            keyword_right,
            block_left,
            block_right,
        }
    }
}

impl Engine {
    pub fn new(schemes: Schemes, default_scheme: SchemesEnum) -> Self {
        let filters: HashMap<&str, FilterFn> = HashMap::new();

        let ctx = Context::new();

        Self {
            filters,
            syntax: EngineSyntax::default(),
            schemes,
            default_scheme,
            context: ctx.clone(),
            runtime: RuntimeContext::new(ctx.clone()).into(),
            templates: HashMap::new(),
            sources: vec![],
            errors: ErrorCollector::new(),
        }
    }

    pub fn set_syntax(&mut self, syntax: EngineSyntax) {
        self.syntax = syntax;
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

    pub fn remove_template(&mut self, name: &String) -> bool {
        self.templates.remove(name).is_some()
    }

    pub fn add_context(&mut self, context: serde_json::Value) {
        self.context.merge_json(context);
    }

    pub fn get_source(&self, name: &str) -> &String {
        let template = self
            .templates
            .get(name)
            .unwrap_or_else(|| panic!("Failed to get template: {}", name));
        self.sources
            .get(template.source_id)
            .unwrap_or_else(|| panic!("Failed to get source of template: {}", name))
    }

    pub fn compile(&mut self, source: String) -> Result<String, Vec<Error>> {
        self.add_template(String::from("temporary"), source.clone());
        let res = self.render("temporary");
        self.remove_template(&String::from("temporary"));
        res
    }

    pub fn render(&self, name: &str) -> Result<String, Vec<Error>> {
        match self.templates.get(name) {
            Some(template) => {
                let res = self.generate_template(template);
                if !self.errors.is_empty() {
                    return Err(self.errors.take());
                }
                Ok(res)
            }
            None => {
                self.errors.add(Error::TemplateNotFound {
                    template: name.to_owned(),
                });
                Err(self.errors.take())
            }
        }
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

            let plain_ident = text::ident().map(|s: &str| Value::Ident(s.to_string()));

            let escape = just('\\').ignore_then(just('"').or(just('\\')));

            let inner = escape
                .or(none_of("\"\\"))
                .repeated()
                .collect::<String>()
                .map(Value::Ident);

            let quoted_ident = inner.delimited_by(just('"'), just('"'));

            let ident = quoted_ident.or(plain_ident);

            let sign = just('-').or(just('+')).or_not();

            let int = sign
                .then(text::int(10))
                .map(|(sign, digits): (Option<char>, &str)| {
                    let number = format!("{}{}", sign.unwrap_or('+'), digits);
                    Value::Int(number.parse::<i64>().unwrap())
                });

            let float = sign
                .then(text::int(10)) // int part
                .then_ignore(just('.'))
                .then(text::int(10)) // frac part
                .map(
                    |((sign, int_part), frac_part): ((Option<char>, &str), &str)| {
                        let number = format!("{}{}.{}", sign.unwrap_or('+'), int_part, frac_part);
                        Value::Float(number.parse::<f64>().unwrap())
                    },
                );

            let range = int
                .then_ignore(just(".."))
                .then(int)
                .map_with(|(start, end), e| SpannedExpr {
                    expr: Expression::Range {
                        start: start.get_int().expect("Failed to get int from range"),
                        end: end.get_int().expect("Failed to get int from range"),
                    },
                    span: e.span(),
                });

            let boolean = just("true")
                .to(Value::Bool(true))
                .or(just("false").to(Value::Bool(false)));

            let spanned_ident = ident.map_with(|value, e| SpannedValue::new(value, e.span()));

            let arg = float.or(int).or(ident).or(boolean).map_with(|value, e| {
                Box::new(SpannedExpr {
                    expr: Expression::LiteralValue {
                        value: SpannedValue {
                            value,
                            span: e.span(),
                        },
                    },
                    span: e.span(),
                })
            });

            // Filter: name is span, args are spanned values
            let filter = text::ident()
                .map_with(|_, e| e.span())
                .then(
                    just(':')
                        .padded()
                        .ignore_then(
                            arg.or(expr.clone())
                                .padded()
                                .separated_by(just(',').padded())
                                .collect::<Vec<Box<SpannedExpr>>>(),
                        )
                        .or_not(),
                )
                .map_with(
                    |(name, args), e: &mut chumsky::input::MapExtra<'_, '_, _, _>| SpannedExpr {
                        expr: Expression::Filter {
                            name,
                            args: args.unwrap_or_default(),
                        },
                        span: e.span(),
                    },
                );

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

            let raw = any()
                .and_is(just(syntax.keyword_left).or(just(syntax.block_left)).not())
                .repeated()
                .at_least(1)
                .collect::<String>()
                .map_with(|_, span| {
                    Box::new(SpannedExpr {
                        expr: Expression::Raw { value: span.span() },
                        span: span.span(),
                    })
                });

            let include = just("include")
                .padded()
                .ignore_then(spanned_ident.padded())
                .delimited_by(just(syntax.block_left), just(syntax.block_right))
                .map_with(|name, e| {
                    Box::new(SpannedExpr {
                        expr: Expression::Include { name },
                        span: e.span(),
                    })
                });

            let if_statement = just("if")
                .padded()
                .ignore_then(keyword_full.clone().padded())
                .then_ignore(just(syntax.block_right).padded())
                .then(raw.or(expr.clone()).repeated().collect())
                .then(
                    just(syntax.block_left)
                        .padded()
                        .ignore_then(just("else").padded())
                        .ignore_then(just(syntax.block_right).padded())
                        .ignore_then(raw.or(expr.clone()).repeated().collect())
                        .or_not(),
                )
                .delimited_by(
                    just(syntax.block_left),
                    just("endif")
                        .padded()
                        .delimited_by(just(syntax.block_left), just(syntax.block_right)),
                )
                .map_with(|((condition, then_branch), else_branch), e| {
                    Box::new(SpannedExpr {
                        expr: Expression::If {
                            condition: condition,
                            then_branch: then_branch,
                            else_branch: else_branch,
                        },
                        span: e.span(),
                    })
                });

            let for_loop = just("for")
                .padded()
                .ignore_then(
                    spanned_ident
                        .separated_by(just(',').padded())
                        .at_least(1)
                        .collect::<Vec<SpannedValue>>(),
                )
                .padded()
                .then_ignore(just("in").padded())
                .then(dotted_ident.or(range).padded())
                .then_ignore(just(syntax.block_right))
                .then(raw.or(expr).repeated().collect())
                .delimited_by(
                    just(syntax.block_left),
                    just("endfor")
                        .padded()
                        .delimited_by(just(syntax.block_left), just(syntax.block_right)),
                )
                .map_with(|((var, list), body), e| {
                    Box::new(SpannedExpr {
                        expr: Expression::ForLoop {
                            var,
                            iter: Box::new(list),
                            body,
                        },
                        span: e.span(),
                    })
                });

            choice((raw, keyword_full, for_loop, if_statement, include))
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
