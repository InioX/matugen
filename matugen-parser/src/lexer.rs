use std::str::Chars;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    String,
    Number,
    Lbracket,
    RBracket,
    Dot,
    Bar,
    Space,
    Colon,
    NewLine,
    Eof,
    Identifier,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
    pub value: TokenValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    None,
    Number(i32),
    String(Atom),
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            source: &input,
            chars: input.chars(),
        }
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    // pub fn tokenize(&mut self) -> Vec<Token> {
    //     let mut tokens = Vec::new();
    //     while let token = self.next_token() {
    //         if token.kind == Kind::Space {
    //             continue;
    //         }
    //         tokens.push(token);
    //     }
    //     tokens.push(Token {
    //         kind: Kind::Eof,
    //         start: self.source.len(),
    //         end: self.source.len(),
    //         value: TokenValue::None,
    //     });
    //     tokens
    // }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn next_token(&mut self) -> Token {
        let start = self.offset();
        let res = self.read_next_kind();
        let end = self.offset();

        Token {
            kind: res.0,
            start,
            end,
            value: res.1,
        }
    }

    fn read_next_kind(&mut self) -> (Kind, TokenValue) {
        let next_char = self.chars.next();

        if next_char.is_none() {
            return (Kind::Eof, TokenValue::None);
        }

        match next_char.unwrap() {
            '{' => (Kind::Lbracket, TokenValue::None),
            '}' => (Kind::RBracket, TokenValue::None),
            '|' => (Kind::Bar, TokenValue::None),
            '.' => (Kind::Dot, TokenValue::None),
            ' ' => (Kind::Space, TokenValue::None),
            ':' => (Kind::Colon, TokenValue::None),
            '0'..='9' => {
                let mut number = next_char.unwrap().to_digit(10).unwrap() as i32;
                while let Some(next_char) = self.peek() {
                    if let Some(digit) = next_char.to_digit(10) {
                        number = number * 10 + digit as i32;
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                (Kind::Number, TokenValue::Number(number))
            }
            _ => {
                if next_char.unwrap() == 0xA as char || next_char.unwrap() == '\n' {
                    (Kind::NewLine, TokenValue::None)
                } else if !next_char.unwrap().is_alphanumeric() || next_char.unwrap() == '_' {
                    (Kind::Identifier, TokenValue::String(String::from(next_char.unwrap()).into()))
                } else {
                    let start = self.offset() - 1;
                    let mut string = String::from(next_char.unwrap());
                    while let Some(next_char) = self.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            string.push(next_char);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    let end = self.offset();
                    (
                        Kind::String,
                        TokenValue::String(self.source[start..end].to_string().into()),
                    )
                }
            }
        }
    }
}
