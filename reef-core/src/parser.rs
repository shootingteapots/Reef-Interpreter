#![allow(unused)]

use std::thread::current;

use crate::{ParseNode, ParseNodeKind, Token};

/// The parser is responsible for reading a stream of tokens and outputting
/// an abstract syntax tree representation of the program.
pub struct Parser {
    pub ast_nodes: Vec<ParseNode>,

    current_token: usize,
    token_stream: Vec<Token>,
}

impl Parser {
    pub fn new(token_stream: Vec<Token>) -> Self {
        Self {
            token_stream,
            ast_nodes: Vec::new(),
            current_token: 0,
        }
    }

    pub fn parse_tokens(&mut self) -> ParseNode {
        let base_node = ParseNode::new(ParseNodeKind::Program);

        while self.token_stream.len() > 0 {
            let current_token = self.token_stream.get(self.current_token).unwrap();

            self.parse_expr();
        }

        base_node
    }

    fn parse_number_expr(&mut self, number: &f64) -> ParseNode {
        let node = ParseNode::new(ParseNodeKind::Number(*number));
        self.get_next_token();

        node
    }

    fn parse_paren_expr(&mut self) -> ParseNode {
        self.get_next_token(); // Consume '('

        let v = self.parse_expr();
        let current_token = self.token_stream.get(self.current_token).unwrap();
        if *current_token != Token::RParen {
            panic!("FUCK ITS NOT A R PAREN NOOOOOOO");
        }

        self.get_next_token(); // Consume ')'

        v
    }

    fn parse_ident_expr(&mut self) -> ParseNode {
        ParseNode::new(ParseNodeKind::Product)
    }

    /// Base method for parsing any type of expression. Gets the next token from the
    /// stream and matches it, calling the relevant function and appending the returned
    /// node to the return node.
    fn parse_expr(&mut self) -> ParseNode {
        let next = self.get_next_token();

        match next {
            _ => {}
        }

        ParseNode::new(ParseNodeKind::Product)
    }

    fn get_next_token(&mut self) -> &Token {
        self.current_token += 1;

        &self.token_stream.get(self.current_token).unwrap()
    }
}
