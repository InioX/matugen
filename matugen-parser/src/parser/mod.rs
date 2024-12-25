pub mod keywords;
pub mod language;
pub mod settings;

use settings::SyntaxSettings;

use crate::lexer::{Kind, Lexer, Token, TokenValue};
use crate::node::{FilterDefinition, KeywordDefinition, Node, Program, Statement};

#[derive(Debug)]
pub struct Parser<'a> {
    pub source: &'a str,
    pub filename: &'a str,
    pub lexer_state: LexerState<'a>,

    pub opened: bool,
    pub closed: bool,
    pub seen_dot: bool,

    pub last_bracket_start: usize,
    pub syntax: &'a SyntaxSettings,
}

#[derive(Debug)]
pub struct LexerState<'a> {
    pub lexer: Lexer<'a>,
    pub cur_token: Token,
    pub prev_token_end: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, filename: &'a str, syntax: &'a SyntaxSettings) -> Parser<'a> {
        let mut lexer = Lexer::new(&source);
        Parser {
            source,
            filename,
            lexer_state: LexerState {
                cur_token: lexer.start(),
                prev_token_end: 0,
                lexer,
            },
            opened: false,
            closed: false,
            seen_dot: false,
            last_bracket_start: 0,
            syntax: &syntax,
        }
    }

    pub fn parse(&mut self) -> Program {
        let end = self.source.len();
        let keyword_statements = self.get_keywords();
        Program {
            node: Node { start: 0, end },
            body: keyword_statements,
        }
    }

    fn start_node(&mut self) -> Node {
        let token = self.lexer_state.cur_token();
        Node::new(token.start, 0)
    }

    fn finish_node(&self, node: Node) -> Node {
        Node::new(node.start, self.lexer_state.prev_token_end)
    }
}

impl LexerState<'_> {
    fn cur_token(&self) -> &Token {
        &self.cur_token
    }

    fn cur_kind(&self) -> &Kind {
        &self.cur_token.kind
    }

    fn cur_val(&self) -> &TokenValue {
        &self.cur_token.value
    }

    fn at(&self, kind: &Kind) -> bool {
        self.cur_kind() == kind
    }

    fn bump(&mut self, kind: &Kind) {
        if self.at(kind) {
            self.advance();
        }
    }

    fn bump_any(&mut self) {
        self.advance();
    }

    fn bump_until_not_at(&mut self, kind: &Kind) {
        while self.cur_kind() == kind && !self.at(&Kind::Eof) {
            self.bump_any()
        }
    }

    fn bump_while_not(&mut self, kind: &Kind) {
        while self.cur_kind() != kind && !self.at(&Kind::Eof) {
            self.advance();
        }
    }

    fn eat(&mut self, kind: &Kind) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }
        false
    }

    fn eat_ignore_spaces(&mut self, kind: &Kind) -> bool {
        self.bump_until_not_at(&Kind::Space);

        if self.at(kind) {
            self.advance();
            return true;
        }
        false
    }

    fn advance(&mut self) {
        let token = self.lexer.next_token();
        self.prev_token_end = self.cur_token.end;
        self.cur_token = token.into();

        println!("self at : {:?}", self.cur_token());
    }
}
