use std::{cell::RefCell, collections::HashMap};

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{error::Rich, prelude::*, span::SimpleSpan};

use crate::parser::{
    context::RuntimeContext, filtertype::FilterFn, Error, ErrorCollector, SpannedValue,
};

use super::context::Context;

mod replace;
pub(crate) use replace::*;
mod parser;

mod resolve;

#[derive(Debug, Clone)]
enum Expression {
    Keyword {
        keywords: Box<SpannedExpr>,
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
        negated: bool,
    },
    Range {
        start: i64,
        end: i64,
    },
    LiteralValue {
        value: SpannedValue,
    },
    BinaryOp {
        lhs: Box<SpannedExpr>,
        op: SpannedBinaryOperator,
        rhs: Box<SpannedExpr>,
    },
    Access {
        keywords: Vec<SimpleSpan>,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct SpannedBinaryOperator {
    op: BinaryOperator,
    span: SimpleSpan,
}

#[derive(Debug, Clone, Copy)]
enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
        }
    }
}

impl Expression {
    pub fn as_keywords<'a>(&self, source: &'a str) -> Option<Vec<&'a str>> {
        match self {
            Expression::Keyword { keywords } => keywords.expr.as_keywords(source),
            Expression::Access { keywords } => Some(get_str_vec(source, keywords)),
            _ => None,
        }
    }
    pub fn as_spans<'a>(&self) -> Option<&Vec<SimpleSpan>> {
        if let Expression::Access { keywords } = self {
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
    context: Context,
    runtime: RefCell<RuntimeContext>,
    templates: HashMap<String, Template>,
    sources: Vec<String>,
    errors: ErrorCollector,
}

pub struct Template {
    pub name: String,
    pub source_id: usize,
    pub ast: Vec<Box<SpannedExpr>>,
}

pub struct EngineSyntax {
    pub keyword_left: String,
    pub keyword_right: String,
    pub block_left: String,
    pub block_right: String,
}

impl Default for EngineSyntax {
    fn default() -> Self {
        Self {
            keyword_left: String::from("{{"),
            keyword_right: String::from("}}"),
            block_left: String::from("<*"),
            block_right: String::from("*>"),
        }
    }
}

impl EngineSyntax {
    pub fn new(
        keyword_left: String,
        keyword_right: String,
        block_left: String,
        block_right: String,
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
    pub fn new() -> Self {
        let filters: HashMap<&str, FilterFn> = HashMap::new();

        let ctx = Context::new();

        Self {
            filters,
            syntax: EngineSyntax::default(),
            context: ctx.clone(),
            runtime: RuntimeContext::new(ctx.clone()).into(),
            templates: HashMap::new(),
            sources: vec![],
            errors: ErrorCollector::new(),
        }
    }

    pub fn set_syntax(&mut self, syntax: EngineSyntax) -> EngineSyntax {
        std::mem::replace(&mut self.syntax, syntax)
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

        let (ast, errs) = parser.parse(source_ref).into_output_errors();

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
                let res = self.generate_template(template, name.to_string());
                if !self.errors.is_empty() {
                    return Err(self.errors.take());
                }
                Ok(res)
            }
            None => {
                self.errors.add(Error::TemplateNotFound {
                    template: name.to_owned(),
                    name: "none".to_string(),
                });
                Err(self.errors.take())
            }
        }
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
