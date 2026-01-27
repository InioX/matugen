use std::{cell::RefCell, collections::HashSet};

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::span::SimpleSpan;

use thiserror::Error as ThisError;

use crate::parser::Engine;

#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: RefCell<Vec<Error>>,
    seen_spans: RefCell<HashSet<SimpleSpan>>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self {
            errors: RefCell::new(Vec::new()),
            seen_spans: RefCell::new(HashSet::new()),
        }
    }

    pub fn add(&self, error: Error) {
        let seen = match &error {
            Error::TemplateNotFound { template: _, .. } => false,
            Error::ParseError { kind: _, span, .. } => self.seen_spans.borrow().contains(span),
            Error::ResolveError { span, .. } => self.seen_spans.borrow().contains(span),
            Error::IncludeError { span, .. } => self.seen_spans.borrow().contains(span),
        };
        if !seen {
            let span = error.get_span();
            if span.is_some() {
                self.seen_spans.borrow_mut().insert(span.unwrap());
            }

            self.errors.borrow_mut().push(error);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.borrow().is_empty()
    }

    pub fn into_inner(self) -> Vec<Error> {
        self.errors.into_inner()
    }

    pub fn take(&self) -> Vec<Error> {
        let mut errors = self.errors.borrow_mut();
        let mut taken = Vec::new();
        std::mem::swap(&mut *errors, &mut taken);
        self.seen_spans.borrow_mut().clear();
        taken
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Could not find template: {template}")]
    TemplateNotFound { template: String, name: String },
    #[error("Parse Error: {kind}")]
    ParseError {
        kind: ParseErrorKind,
        span: SimpleSpan,
        name: String,
    },
    #[error("Value does not exist in the context")]
    ResolveError { span: SimpleSpan, name: String },
    #[error("Failed to include file")]
    IncludeError { span: SimpleSpan, name: String },
}

#[derive(Debug, ThisError)]
pub enum ParseErrorKind {
    #[error(transparent)]
    Filter(#[from] FilterError),

    #[error(transparent)]
    Keyword(#[from] KeywordError),

    #[error(transparent)]
    Loop(#[from] LoopError),

    #[error(transparent)]
    BinOp(#[from] BinaryOperatorError),

    #[error(transparent)]
    If(#[from] IfError),
}

#[derive(Debug, ThisError)]
pub enum IfError {
    #[error("You can only use if conditions with Booleans")]
    InvalidIfCondition,
}

#[derive(Debug, ThisError)]
pub enum BinaryOperatorError {
    #[error("Cannot apply '{op}' operator between {lhs} and {rhs}")]
    InvalidBinaryOperatorType {
        lhs: String,
        op: String,
        rhs: String,
    },
}

#[derive(Debug, ThisError)]
pub enum LoopError {
    #[error("You can only loop over Arrays, Maps and Colors")]
    LoopOverNonIterableValue,
    #[error("For loop over an Array supports only one variable. Key and value iteration (`<* for key, value in map *>`) is only valid for Maps.")]
    TooManyLoopVariablesArray,
    #[error("For loop supports only one or two variables")]
    TooManyLoopVariables,
}

#[derive(Debug, ThisError)]
pub enum KeywordError {
    #[error("The format provided is not valid. Available formats are: {formats:?}")]
    InvalidFormat { formats: &'static [&'static str] },
    #[error("Invalid color mode. The color mode can only be one of: [dark, light, default]")]
    ColorDoesNotExist,
    #[error("The format for colors is 'colors.<color>.<scheme>.<format>'")]
    InvalidColorDefinition,
}

#[derive(Debug, ThisError)]
pub enum FilterError {
    #[error("Not enough arguments provided for filter")]
    NotEnoughArguments,
    #[error("Found '{actual}' expected '{expected}'")]
    InvalidArgumentType {
        span: SimpleSpan,
        expected: String,
        actual: String,
    },
    #[error("Cannot use color filters on a string filter, consider using the 'to_color' filter")]
    ColorFilterOnString,
    #[error("Cannot use color filters on a boolean value")]
    ColorFilterOnBool,
    #[error("Could not find the filter: {filter}")]
    FilterNotFound { filter: String },
    #[error("Invalid String, expected one of: [{expected}]")]
    UnexpectedStringValue { expected: String, span: SimpleSpan },
}

impl Error {
    pub fn get_span(&self) -> Option<SimpleSpan> {
        match self {
            Error::TemplateNotFound { template: _, .. } => None,
            Error::ParseError { kind: _, span, .. } => Some(*span),
            Error::ResolveError { span, .. } => Some(*span),
            Error::IncludeError { span, .. } => Some(*span),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Error::TemplateNotFound { .. } => "TemplateNotFound".to_owned(),
            Error::ParseError { kind, .. } => match kind {
                ParseErrorKind::Filter(e) => format!("ParseError::{}", e.name()),
                ParseErrorKind::Keyword(e) => format!("ParseError::{}", e.name()),
                ParseErrorKind::Loop(e) => format!("ParseError::{}", e.name()),
                ParseErrorKind::BinOp(e) => format!("ParseError::{}", e.name()),
                ParseErrorKind::If(e) => format!("ParseError::{}", e.name()),
            },
            Error::ResolveError { .. } => "ResolveError".to_owned(),
            Error::IncludeError { .. } => "IncludeError".to_owned(),
        }
    }

    pub fn get_file_name(&self) -> &String {
        match self {
            Error::TemplateNotFound { name, .. } => name,
            Error::ParseError { name, .. } => name,
            Error::ResolveError { name, .. } => name,
            Error::IncludeError { name, .. } => name,
        }
    }

    pub fn emit(&self, engine: &Engine) -> Result<(), color_eyre::Report> {
        let name = self.get_name();
        let message = self.to_string();
        let span = self.get_span();
        let file_name = self.get_file_name();
        let source_code = engine.get_source(&file_name)?;

        if let Some(span) = span {
            Ok(build_report(&name, source_code, message, span, file_name))
        } else {
            Ok(eprintln!("{}", message))
        }
    }
}

impl FilterError {
    pub fn name(&self) -> &str {
        match self {
            FilterError::NotEnoughArguments => "NotEnoughArguments",
            FilterError::InvalidArgumentType { .. } => "InvalidArgumentType",
            FilterError::ColorFilterOnString => "ColorFilterOnString",
            FilterError::ColorFilterOnBool => "ColorFilterOnBool",
            FilterError::FilterNotFound { .. } => "FilterNotFound",
            FilterError::UnexpectedStringValue { .. } => "UnexpectedStringValue",
        }
    }
}

impl KeywordError {
    pub fn name(&self) -> &str {
        match self {
            KeywordError::InvalidFormat { .. } => "InvalidFormat",
            KeywordError::ColorDoesNotExist => "ColorDoesNotExist",
            KeywordError::InvalidColorDefinition => "InvalidColorDefinition",
        }
    }
}

impl LoopError {
    pub fn name(&self) -> &str {
        match self {
            LoopError::LoopOverNonIterableValue => "LoopOverNonIterableValue",
            LoopError::TooManyLoopVariablesArray => "TooManyLoopVariables",
            LoopError::TooManyLoopVariables => "TooManyLoopVariables",
        }
    }
}

impl BinaryOperatorError {
    pub fn name(&self) -> &str {
        match self {
            BinaryOperatorError::InvalidBinaryOperatorType { .. } => "InvalidBinaryOperatorType",
        }
    }
}

impl IfError {
    pub fn name(&self) -> &str {
        match self {
            IfError::InvalidIfCondition => "InvalidIfCondition",
        }
    }
}

fn build_report(name: &str, source_code: &str, message: String, span: SimpleSpan, file_name: &str) {
    Report::build(ReportKind::Error, (file_name, span.into_range()))
        .with_config(ariadne::Config::default().with_index_type(ariadne::IndexType::Byte))
        .with_message(name)
        .with_label(
            Label::new((file_name, span.into_range()))
                .with_message(message)
                .with_color(Color::Red),
        )
        .finish()
        .print((file_name, Source::from(&source_code)))
        .unwrap();
}
