use std::fmt;

use crate::parser::Parser;

#[derive(Debug)]
pub struct ParseError<'a> {
    pub err_type: ParseErrorTypes,
    pub start: usize,
    pub end: usize,
    pub source: &'a str,
    pub filename: &'a str,
    pub line_number: u64,
}

impl ParseError<'_> {
    pub fn new<'a>(
        err_type: ParseErrorTypes,
        start: usize,
        end: usize,
        source: &'a str,
        filename: &'a str,
        line_number: u64,
    ) -> ParseError<'a> {
        ParseError {
            err_type,
            start,
            end,
            source,
            filename,
            line_number,
        }
    }
    pub fn new_from_parser<'a>(err_type: ParseErrorTypes, parser: &Parser<'a>) -> ParseError<'a> {
        ParseError {
            err_type,
            start: parser.last_bracket_start,
            end: parser.prev_token_end,
            source: parser.source,
            filename: &parser.filename,
            line_number: parser.lexer.cur_line,
        }
    }
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_msg = match self.err_type {
            ParseErrorTypes::UnexpectedFilterArgumentToken => {
                "Unexpected character in filter argument"
            }
            ParseErrorTypes::UnclosedBracket => "Unclosed bracket",
            ParseErrorTypes::DoubleDot => "Double dot",
            ParseErrorTypes::DoubleString => "Double string",
        };
        let mut str = "".to_string();

        let span = self.source.get(self.start..self.end).unwrap_or("");

        for line in span.lines() {
            str.push_str(&format!("{} \x1b[94m|\x1b[0m {}\n", self.line_number, line))
        }

        write!(
            f,
            "\n\u{1b}[2;30;41m ERROR \u{1b}[0m\u{1b}[2;30;47m {} \u{1b}[0m\n\x1b[94m-->\x1b[0m {}:{}:{}\n{}\n",
            err_msg, self.filename, self.start, self.end, str,
        )

        // write!(
        // f,
        // "\n\u{1b}[1;31m[ERROR] {} {}..{}: {}:\u{1b}[0m\n{}\n",
        // self.filename, self.start, self.end, err_msg, span,
        // )
    }
}

#[derive(Debug)]
pub enum ParseErrorTypes {
    UnclosedBracket,
    DoubleDot,
    DoubleString,
    UnexpectedFilterArgumentToken,
}
