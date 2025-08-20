use std::{cell::RefCell, collections::HashSet};

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::span::SimpleSpan;

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
            Error::TemplateNotFound { template: _ } => false,
            Error::ParseError { kind: _, span } => self.seen_spans.borrow().contains(span),
            Error::ResolveError { span } => self.seen_spans.borrow().contains(span),
            Error::IncludeError { span } => self.seen_spans.borrow().contains(span),
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
    IncludeError {
        span: SimpleSpan,
    },
}

impl Error {
    pub fn get_span(&self) -> Option<SimpleSpan> {
        match self {
            Error::TemplateNotFound { template: _ } => None,
            Error::ParseError { kind: _, span } => Some(*span),
            Error::ResolveError { span } => Some(*span),
            Error::IncludeError { span } => Some(*span),
        }
    }

    pub fn emit(&self, source: &str, file_name: &str) {
        match self {
            Error::ParseError { kind, span } => match kind {
                ParseErrorKind::Filter(filter_error) => {
                    emit_filter_error(source, filter_error, *span, file_name)
                }
                ParseErrorKind::Keyword(keyword_error) => {
                    emit_keyword_error(source, *span, keyword_error, file_name)
                }
            },
            Error::ResolveError { span } => emit_resolve_error(source, *span, file_name),
            Error::TemplateNotFound { template } => {
                eprintln!("{}", format!("Could not find template: {}", template))
            }
            Error::IncludeError { span } => emit_include_error(source, *span, file_name),
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
    InvalidScheme,
    ColorDoesNotExist,
    InvalidColorDefinition,
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
    ColorFilterOnBool,
    FilterNotFound {
        filter: String,
    },
    UnexpectedStringValue {
        expected: String,
        span: SimpleSpan,
    },
}

pub fn emit_include_error(source_code: &str, span: SimpleSpan, file_name: &str) {
    build_report(
        "ResolveError",
        source_code,
        format!(
            "Could not find the '{}' template. Make sure it is in config.toml and named correctly.",
            source_code
                .get(span.start..span.end)
                .unwrap_or("<invalid span>")
        ),
        span,
        file_name,
    );
}

pub fn emit_keyword_error(
    source_code: &str,
    span: SimpleSpan,
    kind: &KeywordError,
    file_name: &str,
) {
    let (name, message) = match kind {
        KeywordError::InvalidFormat => ("InvalidColorFormat", "The format provided is not valid, make sure it is one of:\n\t\t[hex, hex_stripped, rgb, rgba, hsl, hsla, red, green, blue, red, alpha, hue, saturation, lightness]".to_owned()),
        KeywordError::ColorDoesNotExist => ("ColorDoesNotExist", "This color does not exist. Check https://github.com/InioX/matugen/wiki/Configuration#example-of-all-the-color-keywords to get a list of all the colors.".to_owned()),
        KeywordError::InvalidColorDefinition => ("InvalidColorDefinition", "The format for colors is 'colors.<color>.<scheme>.<format>'".to_owned()),
        KeywordError::InvalidScheme => ("InvalidScheme", "Invalid color mode. The color mode can only be one of: [dark, light, default]".to_owned()),
            };

    build_report(
        &format!("KeywordError::{}", name),
        source_code,
        message,
        span,
        file_name,
    );
}

pub fn emit_resolve_error(source_code: &str, span: SimpleSpan, file_name: &str) {
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
        file_name,
    );
}

pub fn emit_filter_error(source_code: &str, kind: &FilterError, span: SimpleSpan, file_name: &str) {
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
        FilterError::ColorFilterOnBool => (
            "Cannot use color filters on a boolean value".to_string(),
            span,
            "ColorFilterOnBool",
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
    build_report(
        &format!("FilterError::{}", name),
        source_code,
        message,
        span,
        file_name,
    );
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
