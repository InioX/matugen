use std::str::Chars;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    String,
    Number,
    Float,
    Space,
    Colon,
    NewLine,
    Eof,
    Sof,
    Identifier,

    // SPECIAL TOKENS
    LBracket,
    RBracket,
    LessThan,
    GreaterThan,
    Asterisk,
    Bar,
    Dot,
}

use std::str::FromStr;

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
    Float(f32),
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    pub cur_line: u64,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            source: &input,
            chars: input.chars(),
            cur_line: 0,
        }
    }

    // pub fn init_tokens(&mut self) {
    //     for (i, line) in self.source.lines().enumerate() {
    //         if i < self.source.lines().count() - 1 {
    //             self.tokens.push((Kind::NewLine, TokenValue::None));
    //         }
    //     }
    // }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    pub fn start(&self) -> Token {
        println!("{:#?}\n\n", self.source);
        Token {
            kind: Kind::Sof,
            start: 0,
            end: 0,
            value: TokenValue::None,
        }
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
        let (kind, value) = self.read_next_kind();
        let end = self.offset();

        Token {
            kind,
            start,
            end,
            value,
        }
    }

    fn read_next_kind(&mut self) -> (Kind, TokenValue) {
        let next_char = self.chars.next();

        if next_char.is_none() {
            return (Kind::Eof, TokenValue::None);
        }

        match next_char.unwrap() {
            '{' => (Kind::LBracket, TokenValue::None),
            '}' => (Kind::RBracket, TokenValue::None),
            ':' => (Kind::Colon, TokenValue::None),
            '<' => (Kind::LessThan, TokenValue::None),
            '>' => (Kind::GreaterThan, TokenValue::None),
            '*' => (Kind::Asterisk, TokenValue::None),
            '|' => (Kind::Bar, TokenValue::None),
            '.' => (Kind::Dot, TokenValue::None),
            ' ' => (Kind::Space, TokenValue::None),
            '0'..='9' => {
                let mut str = String::from(next_char.unwrap());
                let mut is_float = false;

                while let Some(next_char) = self.peek() {
                    if next_char == '.' {
                        is_float = true;
                        str.push(next_char);
                        self.chars.next();
                        continue;
                    }
                    if next_char.is_ascii_digit() {
                        str.push(next_char);
                        self.chars.next();
                    } else {
                        println!("{}", str);
                        break;
                    }
                }
                if is_float {
                    (
                        Kind::Float,
                        TokenValue::Float(
                            f32::from_str(&str)
                                .expect(&format!("Couldn't make f32 from {:?}", str)),
                        ),
                    )
                } else {
                    (
                        Kind::Number,
                        TokenValue::Number(
                            i32::from_str(&str)
                                .expect(&format!("Couldn't make i32 from {:?}", str)),
                        ),
                    )
                }

                // let mut number = next_char.unwrap().to_digit(10).unwrap() as i32;
                // let mut number_float = next_char.unwrap().to_digit(10).unwrap() as f32;
                // let mut is_float = false;
                // while let Some(next_char) = self.peek() {
                //     if next_char == '.' {
                //         is_float = true;
                //     }
                //     if let Some(digit) = next_char.to_digit(10) {
                //         if is_float {
                //             number_float = number_float as f32 * 10.0 + digit as f32;
                //         } else {
                //             number = number * 10 + digit as i32;
                //         }
                //         self.chars.next();
                //     } else {
                //         break;
                //     }
                // }
                // if is_float {
                //     (Kind::Float, TokenValue::Float(number_float))
                // } else {
                //     (Kind::Number, TokenValue::Number(number))
                // }
            }
            _ => {
                if (next_char.unwrap() == '\r' && self.peek() == Some('\n'))
                    || next_char.unwrap() == '\n'
                    || next_char.unwrap() == 0xA as char
                {
                    self.cur_line += 1;
                    (Kind::NewLine, TokenValue::None)
                } else if !next_char.unwrap().is_alphanumeric() || next_char.unwrap() == '_' {
                    println!("{:#?}", next_char.unwrap());
                    (
                        Kind::Identifier,
                        TokenValue::String(String::from(next_char.unwrap()).into()),
                    )
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
