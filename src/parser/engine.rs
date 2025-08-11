use std::{cell::RefCell, collections::HashMap};

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{error::Rich, prelude::*, span::SimpleSpan};

use crate::{
    parser::{context::RuntimeContext, filtertype::FilterFn, Error, ErrorCollector, SpannedValue},
    scheme::{Schemes, SchemesEnum},
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

impl Expression {
    pub fn as_keywords<'a>(&self, source: &'a str) -> Option<Vec<&'a str>> {
        if let Expression::Access { keywords } = self {
            Some(get_str_vec(source, keywords))
        } else {
            None
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
    pub source_id: usize,
    pub ast: Vec<Box<SpannedExpr>>,
}

pub struct EngineSyntax {
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
