use crate::lexer::{Token, TokenValue};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,
}

impl Node {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug)]
pub struct Program {
    pub node: Node,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    KeywordDefinition(Box<KeywordDefinition>),
}

#[derive(Debug)]
pub struct KeywordDefinition {
    pub node: Node,
    pub keywords: Vec<TokenValue>,
    pub filters: Option<Vec<String>>,
}
