use std::fmt::{Display, Formatter, Result};

pub mod parser;
pub mod scanner;

pub trait ReefDebuggable {
    fn debug_write_to_file(&self, file_path: &str);
}

/// Different types of tokens which can be returned by the scanner.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(String),
    Keyword(String),
    Ident(String),
    String(String),
    Number(f64),
    Comment(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Colon,
    Comma,
    Dot,
    EndOfFile,
}

/// The different types that an AST node can have.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseNodeKind {
    Program,
    Number(f64),
    Sum,
    Product,
}

pub struct ParseNode {
    pub node_kind: ParseNodeKind,
    pub children: Vec<ParseNode>,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ParseNodeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl ParseNode {
    pub fn new(node_kind: ParseNodeKind) -> Self {
        Self {
            node_kind,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, parse_node: ParseNode) {
        self.children.push(parse_node);
    }
}
