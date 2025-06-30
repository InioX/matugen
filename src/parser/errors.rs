use std::{cell::RefCell, collections::HashSet};

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{container::Seq, span::SimpleSpan};

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
            Error::TemplateNotFound { template } => false,
            Error::ParseError { kind, span } => self.seen_spans.borrow().contains(span),
            Error::ResolveError { span } => self.seen_spans.borrow().contains(span),
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

#[derive(Debug)]
pub enum Error {
    TemplateNotFound {
        template: String,
    },
    ParseError {
        kind: ParseErrorKind,
        span: SimpleSpan,
    },
    ResolveError {
        span: SimpleSpan,
    },
}

impl Error {
    pub fn get_span(&self) -> Option<SimpleSpan> {
        match self {
            Error::TemplateNotFound { template } => None,
            Error::ParseError { kind, span } => Some(*span),
            Error::ResolveError { span } => Some(*span),
        }
    }

    pub fn emit(&self, source: &str) {
        match self {
            Error::ParseError { kind, span } => match kind {
                ParseErrorKind::Filter(filter_error) => {
                    emit_filter_error(source, filter_error, *span)
                }
                ParseErrorKind::Keyword(keyword_error) => {
                    emit_keyword_error(source, *span, keyword_error)
                }
            },
            Error::ResolveError { span } => emit_resolve_error(source, *span),
            Error::TemplateNotFound { template } => {
                eprintln!("{}", format!("Could not find template: {}", template))
            }
        }
    }
}

#[derive(Debug)]
pub enum ParseErrorKind {
    Filter(FilterError),
    Keyword(KeywordError),
}

#[derive(Debug)]
pub enum KeywordError {
    InvalidFormat,
}

#[derive(Debug)]
pub enum FilterError {
    NotEnoughArguments,
    InvalidArgumentType {
        span: SimpleSpan,
        expected: String,
        actual: String,
    },
    ColorFilterOnString,
    FilterNotFound {
        filter: String,
    },
    UnexpectedStringValue {
        expected: String,
        span: SimpleSpan,
    },
}

pub fn emit_keyword_error(source_code: &str, span: SimpleSpan, kind: &KeywordError) {
    build_report(
        "KeywordError",
        source_code,
        match kind {
            KeywordError::InvalidFormat => "The format provided is not valid, make sure it is one of:\n\t\t[hex, hex_stripped, rgb, rgba, hsl, hsla, red, green, blue, red, alpha, hue, saturation, lightness]".to_owned(),
        },
        span,
    );
}

pub fn emit_resolve_error(source_code: &str, span: SimpleSpan) {
    build_report(
        "ResolveError",
        source_code,
        format!(
            "The value '{}' does not exist in the context",
            source_code
                .get(span.start..span.end)
                .unwrap_or("<invalid span>")
        ),
        span,
    );
}

pub fn emit_filter_error(source_code: &str, kind: &FilterError, span: SimpleSpan) {
    let (message, span, name) = match kind {
        FilterError::NotEnoughArguments => (
            "Not enough arguments provided for filter".to_string(),
            span,
            "NotEnoughArguments",
        ),
        FilterError::InvalidArgumentType {
            span,
            expected,
            actual,
        } => (
            format!("Found '{}' expected '{}'", actual, expected),
            *span,
            "InvalidArgumentType",
        ),
        FilterError::ColorFilterOnString => (
            "Cannot use color filters on a string filter, consider using the 'to_color' filter"
                .to_string(),
            span,
            "ColorFilterOnString",
        ),
        FilterError::FilterNotFound { filter } => (
            format!("Could not fild filter {filter}").to_string(),
            span,
            "FilterNotFound",
        ),
        FilterError::UnexpectedStringValue { expected, span } => (
            format!("Invalid String, expected one of: [{expected}]").to_string(),
            *span,
            "UnexpectedStringValue",
        ),
    };
    build_report(name, source_code, message, span);
}

fn build_report(name: &str, source_code: &str, message: String, span: SimpleSpan) {
    Report::build(ReportKind::Error, ((), span.into_range()))
        .with_config(ariadne::Config::default().with_index_type(ariadne::IndexType::Byte))
        .with_message(name)
        .with_label(
            Label::new(((), span.into_range()))
                .with_message(message)
                .with_color(Color::Red),
        )
        .finish()
        .print(Source::from(&source_code))
        .unwrap();
}
