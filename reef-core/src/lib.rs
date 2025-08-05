pub mod parser;
pub mod scanner;

/// Different types of tokens which can be returned by the scanner.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Operator(String),
    Keyword(String),
    Ident(String),
    String(String),
    Number(f64),
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
pub enum ASTNodeKind {}

pub struct ASTNode {
    pub node_kind: ASTNodeKind,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for ASTNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ASTNode {
    pub fn new(node_kind: ASTNodeKind) -> Self {
        Self { node_kind }
    }
}
