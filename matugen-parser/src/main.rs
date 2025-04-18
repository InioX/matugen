use std::collections::HashMap;
use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use chumsky::prelude::*;
use chumsky::span::SimpleSpan;

#[derive(Debug, Clone)]
pub enum Arg {
    Ident(String),
    Int(i64),
    Float(f64),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum Expression<'src> {
    Keyword {
        keywords: Vec<&'src str>,
    },
    Filter {
        name: &'src str,
        args: Vec<Arg>,
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

pub struct Engine {
    src: String,
    filters: HashMap<&'static str, FilterFn>,
    syntax: EngineSyntax,
}

pub struct EngineSyntax {
    keyword_left: [char; 2],
    keyword_right: [char; 2],
    block_left: [char; 2],
    block_right: [char; 2],
}

pub type FilterFn = fn(&Box<Expression<'_>>, Vec<Arg>) -> String;

fn uppercase(keyword: &Box<Expression>, args: Vec<Arg>) -> String {
    "A".to_string()
}

// fn trim(keyword: Box<Expression<'_>>, args: Vec<Arg>) -> String {
//     input.trim().to_string()
// }

impl Engine {
    pub fn new<T: Into<String>>(src: T) -> Self {
        let mut filters: HashMap<&str, FilterFn> = HashMap::new();
        filters.insert("uppercase", uppercase);
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
        }
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

        println!("{:?}", changed_src);
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
                src.replace_range(range, &self.get_replacement(keywords).into());
            }
            Expression::KeywordWithFilters { keyword, filters } => {
                src.replace_range(range, &self.get_replacement_filter(keyword, filters).into());
            }
            _ => {
                println!("other")
            }
        }
    }

    fn get_replacement(&self, keywords: Vec<&str>) -> impl Into<String> {
        // if keywords.len() == 0 {
        return String::from("");
        // }

        // if keywords[0] == "colors" {}
    }

    fn get_replacement_filter(
        &self,
        keyword: Box<Expression<'_>>,
        filters: Vec<Expression>,
    ) -> impl Into<String> {
        let replacement: &mut String = &mut String::from("");

        for filter in filters {
            match filter {
                Expression::Filter { name, args } => {
                    *replacement = self.apply_filter(name, args, &keyword).into()
                }
                _ => {}
            };
        }

        replacement.to_string()
    }

    fn apply_filter(
        &self,
        name: &str,
        args: Vec<Arg>,
        keyword: &Box<Expression<'_>>,
    ) -> impl Into<String> {
        match self.filters.get(name) {
            Some(f) => return f(keyword, args),
            None => panic!("{}", format!("Could not find filter {:?}", name)),
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
                Arg::Float(parsed)
            });

        let int = text::int(10).map(|s: &str| Arg::Int(s.parse::<i64>().unwrap()));

        let ident = text::ident().map(|s: &str| Arg::Ident(s.to_string()));

        let arg = float.or(int).or(ident);

        let filter = text::ident()
            .then(
                just(':')
                    .padded()
                    .ignore_then(
                        arg.padded()
                            .separated_by(just(',').padded())
                            .collect::<Vec<Arg>>(),
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

        keyword_full.repeated().collect()
    }
}

fn main() {
    let src = std::fs::read_to_string("matugen-parser/test.test").unwrap();

    let mut engine = Engine::new(src);

    engine.generate_templates();
}
