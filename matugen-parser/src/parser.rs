#![allow(unused_variables)]

#[derive(Debug)]
pub struct ParseError<'a> {
    pub err_type: ParseErrorTypes,
    pub start: usize,
    pub end: usize,
    pub source: &'a str,
    pub filename: &'a str,
    pub line_number: u64,
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
            "\n\u{1b}[2;30;41m ERROR \u{1b}[0m\u{1b}[2;30;47m {} \u{1b}[0m\n\x1b[94m-->\x1b[0m {}:{}..{}:\n{}\n",
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

use std::fmt;
use std::iter::Filter;

use crate::lexer::{Kind, Lexer, Token, TokenValue};
use crate::node::{FilterDefinition, KeywordDefinition, Node, Program, Statement};
/// A parser for turning a stream of tokens into a Abstract Syntax Tree.
#[derive(Debug)]
pub struct Parser<'a> {
    source: &'a str,
    filename: &'a str,
    lexer: Lexer<'a>,

    /// Current Token consumed from the lexer
    cur_token: Token,
    /// The end range of the previous token
    prev_token_end: usize,

    opened: bool,
    closed: bool,
    seen_dot: bool,

    last_bracket_start: usize,
}

impl<'a> Parser<'a> {
    /// Create a new parser.
    pub fn new(source: &'a str, filename: &'a str) -> Parser<'a> {
        let mut lexer = Lexer::new(&source);
        Parser {
            source,
            filename,
            cur_token: lexer.start(),
            lexer,
            prev_token_end: 0,
            opened: false,
            closed: false,
            seen_dot: false,
            last_bracket_start: 0,
        }
    }

    pub fn parse(&mut self) -> Program {
        let end = self.source.len();
        let statments = self.get_keywords();
        Program {
            node: Node { start: 0, end },
            body: statments,
        }
    }

    // pub fn parse(&mut self) -> Template {
    //     Template {
    //         node: Node {
    //             start: 0,
    //             end: self.source.len(),
    //         },
    //         body: vec![],
    //     }
    // }

    // fn parse_keyword_statement(&mut self) -> Statement {
    //     let node = self.start_node();
    //     // NOTE: the token returned from the lexer is `Kind::Debugger`, we'll fix this later.
    //     self.bump_any();
    //     Statement::KeywordDeclarationStatement {
    //         0: KeywordDeclaration {
    //         node: self.finish_node(node),
    //         },
    //     }
    // }

    fn start_node(&mut self) -> Node {
        let token = self.cur_token();
        Node::new(token.start, 0)
    }

    fn finish_node(&self, node: Node) -> Node {
        Node::new(node.start, self.prev_token_end)
    }

    fn cur_token(&self) -> &Token {
        &self.cur_token
    }

    fn cur_kind(&self) -> &Kind {
        &self.cur_token.kind
    }

    fn cur_val(&self) -> &TokenValue {
        &self.cur_token.value
    }

    /// Checks if the current index has token `Kind`
    fn at(&self, kind: Kind) -> bool {
        self.cur_kind() == &kind
    }

    /// Advance if we are at `Kind`
    fn bump(&mut self, kind: Kind) {
        if self.at(kind) {
            self.advance();
        }
    }

    /// Advance any token
    fn bump_any(&mut self) {
        self.advance();
    }

    fn bump_until_not_at(&mut self, kind: Kind) {
        while self.cur_kind() == &kind && !self.at(Kind::Eof) {
            self.bump_any()
        }
    }

    /// Advance any token
    fn bump_while_not(&mut self, kind: Kind) {
        while self.cur_kind() != &kind && !self.at(Kind::Eof) {
            self.advance();
        }
    }

    /// Advance and return true if we are at `Kind`, return false otherwise
    fn eat(&mut self, kind: Kind) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }
        false
    }

    /// Advance and return true if we are at `Kind`, return false otherwise
    fn eat_ignore_spaces(&mut self, kind: Kind) -> bool {
        self.bump_until_not_at(Kind::Space);

        if self.at(kind) {
            self.advance();
            return true;
        }
        false
    }

    /// Move to the next token
    fn advance(&mut self) {
        let token = self.lexer.next_token();
        self.prev_token_end = self.cur_token.end;
        self.cur_token = token;

        println!("self at : {:?}", self.cur_token());
    }

    pub fn get_closing(&mut self) -> Result<(), ParseError> {
        self.bump_any();
        if self.eat(Kind::RBracket) {
            self.closed = true;
            self.opened = false;
            println!(
                "{}..{}: closed fine without filter",
                self.last_bracket_start, self.prev_token_end
            );
            Ok(())
        } else {
            Err(ParseError {
                err_type: ParseErrorTypes::UnclosedBracket,
                start: self.last_bracket_start,
                end: self.prev_token_end,
                source: self.source,
                filename: &self.filename,
                line_number: self.lexer.cur_line,
            })
        }
    }

    pub fn get_keywords(&mut self) -> Vec<Statement> {
        let mut vec: Vec<Statement> = vec![];

        while !self.at(Kind::Eof) {
            if !self.at(Kind::Lbracket) {
                self.bump_until_not_at(Kind::Lbracket);
            }

            // We would only get the second bracket at the start without the -1,
            // the opening will ALWAYS have two brackets unlike the closing, which
            // might have an error inside of it (so we dont look ahead for the closing).
            self.last_bracket_start = self.get_opening().unwrap() - 1;
            let start = self.start_node();

            let mut strings: Vec<TokenValue> = vec![];

            let res = self.collect_strings(&mut strings);

            if let Err(e) = res {
                panic!("{}", format!("{}", e));
            }

            vec.push(Statement::KeywordDefinition(Box::new(KeywordDefinition {
                node: self.finish_node(start),
                keywords: strings,
                filters: None,
            })));
        }
        vec
    }

    fn get_filter(&mut self) -> Result<Option<FilterDefinition>, ParseError> {
        if self.eat(Kind::Bar) {
            println!("ok");
        } else {
            // return Err(ParseError {
            //     err_type: ParseErrorTypes::UnclosedBracket,
            //     start: self.last_bracket_start,
            //     end: self.prev_token_end,
            //     source: self.source,
            // });
        }
        let start = self.start_node();

        // FilterDefinition {
        //     node: self.finish_node(start),
        //     filter_name: todo!(),
        //     arguments: todo!(),
        // };
        self.bump_while_not(Kind::String);
        let name = self.cur_token().clone().value;

        self.advance();

        if self.eat(Kind::RBracket) {
            println!("no filter args");
            self.get_closing();
            return Ok(None);
        }

        let node = self.finish_node(start);

        let res = self.collect_filter_args();
        if let Err(ref e) = res {
            eprintln!("{}", e);
        }

        Ok(Some(FilterDefinition {
            node,
            filter_name: name,
            arguments: res.unwrap(),
        }))
        // self.bump_while_not(Kind::RBracket);
    }

    fn collect_filter_args(&mut self) -> Result<Vec<TokenValue>, ParseError> {
        let mut arguments: Vec<TokenValue> = vec![];

        if !self.eat_ignore_spaces(Kind::Colon) {
            println!("not: {:?}", self.cur_token());
            self.bump_while_not(Kind::RBracket)
        } else {
            // while !self.at(Kind::RBracket) {
            //     match self.cur_kind() {
            //         Kind::String => arguments.push(&self.cur_token.value),
            //         Kind::Number => todo!(),
            //         _ => {}
            //     }
            // }
            loop {
                match self.cur_kind() {
                    Kind::Space => {
                        self.bump_until_not_at(Kind::Space);
                    }
                    Kind::String => {
                        arguments.push(self.cur_token.value.clone());
                        self.bump(Kind::String)
                    }
                    Kind::Number => {
                        arguments.push(self.cur_token.value.clone());
                        self.bump(Kind::Number)
                    }
                    Kind::RBracket => {
                        println!("herer");
                        break;
                    }
                    _ => {
                        return Err(ParseError {
                            err_type: ParseErrorTypes::UnexpectedFilterArgumentToken,
                            start: self.last_bracket_start,
                            end: self.prev_token_end,
                            source: self.source,
                            filename: &self.filename,
                            line_number: self.lexer.cur_line,
                        })
                    }
                }
            }
            // return Err(ParseError {
            //     err_type: ParseErrorTypes::MissingFilterColon,
            //     start: self.last_bracket_start,
            //     end: self.prev_token_end,
            //     source: &self.source,
            // });
        }
        println!("arguments: {:?}", arguments);
        Ok(arguments)
    }

    // Returns true if filter is used
    fn collect_strings(&mut self, strings: &mut Vec<TokenValue>) -> Result<(), ParseError> {
        // Always first string, what comes after we cant know
        self.bump_while_not(Kind::String);
        strings.push(self.cur_token.clone().value);

        self.bump_any();

        while !&self.closed && !self.at(Kind::Eof) {
            match &self.cur_kind() {
                Kind::Dot => {
                    if self.seen_dot && self.eat(Kind::Dot) {
                        self.seen_dot = false;
                        return Err(ParseError {
                            err_type: ParseErrorTypes::DoubleDot,
                            start: self.last_bracket_start,
                            end: self.prev_token_end + 1,
                            source: self.source,
                            filename: &self.filename,
                            line_number: self.lexer.cur_line,
                        });
                    } else {
                        self.seen_dot = true;
                        self.bump(Kind::Dot);
                    }
                }
                Kind::String => {
                    if self.seen_dot {
                        strings.push(self.cur_token.clone().value);
                        self.bump(Kind::String);
                        self.seen_dot = false;
                    } else {
                        self.bump_while_not(Kind::RBracket);
                        return Err(ParseError {
                            err_type: ParseErrorTypes::DoubleString,
                            start: self.last_bracket_start,
                            end: self.prev_token_end + 1,
                            source: self.source,
                            filename: &self.filename,
                            line_number: self.lexer.cur_line,
                        });
                    }
                }
                Kind::Bar => {
                    let res = self.get_filter();
                    if let Err(e) = res {
                        eprintln!("{}", e)
                    }
                    if self.eat_ignore_spaces(Kind::RBracket) {
                        return self.get_closing();
                    } else {
                        self.bump_until_not_at(Kind::RBracket);
                        return self.get_closing();
                    }
                }
                Kind::RBracket => {
                    return self.get_closing();
                    // if self.eat(Kind::RBracket) {
                    //     self.closed = true;
                    //     self.opened = false;
                    //     println!("closed without filter")
                    // } else {
                    //     println!("fucked the closing");
                    //     break;
                    // }
                }
                Kind::Space => self.bump(Kind::Space),
                Kind::NewLine => self.bump(Kind::NewLine),
                Kind::Identifier => self.bump(Kind::Identifier),
                _ => {
                    println!("{:?}", self.cur_token());
                }
            }
        }
        Ok(())
    }

    fn get_opening(&mut self) -> Option<usize> {
        let mut start = self.cur_token().start;

        self.bump_any();

        while !self.opened {
            if self.eat(Kind::Lbracket) {
                self.opened = true;
                self.closed = false;
            } else if self.eat(Kind::Eof) {
                return None;
            }
            self.bump_while_not(Kind::Lbracket);
            start = self.cur_token().start;
        }
        Some(start)
    }
}
