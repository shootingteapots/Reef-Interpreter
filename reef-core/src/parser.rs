#![allow(unused)]

use crate::{ASTNode, ASTNodeKind, TokenKind};

/// The parser is responsible for reading a stream of tokens and outputting
/// an abstract syntax tree representation of the program.
pub struct Parser {
    token_stream: Vec<TokenKind>,
    ast_nodes: Vec<ASTNode>,
}

impl<'a> Parser {
    /// Construct a new parser, taking the tokens to parse as the input.
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self {
            token_stream: tokens,
            ast_nodes: vec![],
        }
    }

    pub fn parse(&mut self) {
        for i in &self.token_stream {
            match i {
                TokenKind::Ident(x) => {
                    println!("{}", x);
                }
                _ => {}
            }
        }
    }
}
