use std::str::FromStr;

use crate::lexer::Kind;

use super::Parser;

pub type Delimeter = [Kind; 2];

#[derive(Debug)]
pub struct SyntaxSettings {
    pub keyword_opening: Delimeter,
    pub keyword_closing: Delimeter,
    pub function_opening: Delimeter,
    pub function_closing: Delimeter,
}

impl SyntaxSettings {
    pub fn new<'a>(
        keyword_opening: [char; 2],
        keyword_closing: [char; 2],
        function_opening: [char; 2],
        function_closing: [char; 2],
    ) -> SyntaxSettings {
        SyntaxSettings {
            keyword_opening: Kind::from_char_arr(keyword_opening),
            keyword_closing: Kind::from_char_arr(keyword_closing),
            function_opening: Kind::from_char_arr(function_opening),
            function_closing: Kind::from_char_arr(function_closing),
        }
    }
}

impl Default for SyntaxSettings {
    fn default() -> SyntaxSettings {
        SyntaxSettings {
            keyword_opening: Kind::from_char_arr(['{', '{']),
            keyword_closing: Kind::from_char_arr(['}', '}']),
            function_opening: Kind::from_char_arr(['<', '*']),
            function_closing: Kind::from_char_arr(['*', '>']),
        }
    }
}

impl Kind {
    fn from_char(c: &char) -> Kind {
        match c {
            '{' => Kind::LBracket,
            '}' => Kind::RBracket,
            '.' => Kind::Dot,
            '|' => Kind::Bar,
            '<' => Kind::LessThan,
            '>' => Kind::GreaterThan,
            '*' => Kind::Asterisk,
            _ => Kind::String,
        }
    }

    fn from_char_arr(arr: [char; 2]) -> Delimeter {
        arr.map(|c| Kind::from_char(&c))
    }
}
