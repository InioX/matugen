pub mod keywords;
pub mod language;

use colored::Colorize;
use std::cell::RefCell;
use std::fmt;
use std::iter::Filter;
use std::rc::Rc;

use crate::errors::parse::{ParseError, ParseErrorTypes};
use crate::errors::{handle_error, handle_error_panic};

use crate::lexer::{Kind, Lexer, Token, TokenValue};
use crate::node::{FilterDefinition, KeywordDefinition, Node, Program, Statement};

#[derive(Debug)]
pub struct Parser<'a> {
    pub source: &'a str,
    pub filename: &'a str,
    pub lexer: Lexer<'a>,

    /// Current Token consumed from the lexer
    pub cur_token: Token,
    /// The end range of the previous token
    pub prev_token_end: usize,

    pub opened: bool,
    pub closed: bool,
    pub seen_dot: bool,

    pub last_bracket_start: usize,
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
        self.cur_token = token.into();

        println!("self at : {:?}", self.cur_token());
    }
}
