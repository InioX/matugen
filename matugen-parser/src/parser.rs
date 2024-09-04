#![allow(unused_variables)]

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

use std::{error::Error, fmt};

use crate::lexer::{Kind, Lexer, Token, TokenValue};
use crate::node::{KeywordDefinition, Node, Program, Statement};
/// A parser for turning a stream of tokens into a Abstract Syntax Tree.
#[derive(Debug)]
pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    /// Current Token consumed from the lexer
    cur_token: Token,

    /// The end range of the previous token
    prev_token_end: usize,
}

impl<'a> Parser<'a> {
    /// Create a new parser.
    pub fn new(source: &'a str) -> Parser<'a> {
        let mut lexer = Lexer::new(&source);
        Parser {
            source,
            cur_token: lexer.next_token(), // There should always be at least one token, Eof
            lexer,
            prev_token_end: 0,
        }
    }

    pub fn parse(&mut self) -> Program {
        let statments = self.get_keywords();
        Program {
            node: Node {
                start: 0,
                end: self.source.len(),
            },
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

    fn cur_token(&mut self) -> &Token {
        &self.cur_token
    }

    fn cur_kind(&mut self) -> &Kind {
        &self.cur_token.kind
    }

    /// Checks if the current index has token `Kind`
    fn at(&mut self, kind: Kind) -> bool {
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

    /// Advance any token
    fn bump_while_not(&mut self, kind: Kind) {
        while self.cur_kind() != &kind && !self.at(Kind::Eof) {
            self.advance()
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

    /// Move to the next token
    fn advance(&mut self) {
        let token = self.lexer.next_token();
        self.prev_token_end = self.cur_token.end;
        self.cur_token = token;
    }

    pub fn get_keywords(&mut self) -> Vec<Statement> {
        let mut opened = false;
        let mut closed = false;
        let mut seen_dot = false;
        let mut vec: Vec<Statement> = vec![];

        while !self.at(Kind::Eof) {
            let start = self.start_node();
            println!("getting opening, {:?}", self.cur_kind());
            println!("is_closed: {}, is_open: {}", closed, opened);
            self.get_opening(&mut opened, &mut closed);

            let mut strings: Vec<TokenValue> = vec![];

            self.collect_strings(&mut closed, &mut opened, &mut seen_dot, &mut strings);

            println!("is_closed: {}, is_open: {}", closed, opened);

            for string in &strings {
                println!("{:?}", string);
            }
            vec.push(Statement::KeywordDefinition(Box::new(KeywordDefinition {
                node: self.finish_node(start),
                keywords: strings,
                filters: None,
            })));
        }
        vec
    }

    fn collect_strings(
        &mut self,
        closed: &mut bool,
        opened: &mut bool,
        seen_dot: &mut bool,
        strings: &mut Vec<TokenValue>,
    ) {
        // Always first string, what comes after we cant know
        self.bump_while_not(Kind::String);
        strings.push(self.cur_token.clone().value);

        self.bump_any();

        while !*closed {
            match self.cur_kind() {
                Kind::Dot => {
                    if *seen_dot {
                        println!("double dot");
                        break;
                    } else {
                        *seen_dot = true;
                        self.bump(Kind::Dot);
                    }
                }
                Kind::String => {
                    if *seen_dot {
                        strings.push(self.cur_token.clone().value);
                        self.bump(Kind::String);
                        *seen_dot = false;
                    } else {
                        println!("double string");
                        break;
                    }
                }
                Kind::RBracket => {
                    if self.eat(Kind::RBracket) {
                        *closed = true;
                        *opened = false;
                    } else {
                        println!("fucked the closing");
                        break;
                    }
                }
                Kind::Space => self.bump(Kind::Space),
                // Kind::Bar => todo!(),
                _ => {
                    println!("{:?}", self.cur_token());
                }
            }
        }
    }

    fn get_opening(&mut self, opened: &mut bool, closed: &mut bool) {
        self.bump_while_not(Kind::Lbracket);

        while !*opened {
            if self.eat(Kind::Lbracket) {
                *opened = true;
                *closed = false;
            } else if self.eat(Kind::Eof) {
                break;
            }
        }
    }
}
