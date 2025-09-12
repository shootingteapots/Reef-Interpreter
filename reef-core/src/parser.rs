#![allow(unused)]

use std::thread::current;

use crate::{ParseNode, ParseNodeKind, ReefDebuggable, Token};

/// The parser is responsible for reading a stream of tokens and outputting
/// an abstract syntax tree representation of the program.
pub struct Parser {
    ast: ParseNode,
    current: usize,
    token_stream: Vec<Token>,
}

impl Parser {
    pub fn new(token_stream: Vec<Token>) -> Self {
        Self {
            token_stream,
            ast: ParseNode::new(ParseNodeKind::Program),
            current: 0,
        }
    }

    pub fn parse_tokens(&mut self) -> ParseNode {
        let mut base_node = ParseNode::new(ParseNodeKind::Program);

        while !self.is_at_end() {
            let next_node = ParseNode::new(ParseNodeKind::Product);

            base_node.add_child(next_node);
        }

        base_node
    }

    pub fn get_program_node(&self) -> &ParseNode {
        &self.ast
    }

    fn is_at_end(&self) -> bool {
        *self.peek().unwrap() == Token::EndOfFile
    }

    fn match_types(t: &Vec<Token>) {
        for token in t {

        }
    }

    fn check(&self, t: Token) -> bool {
        if self.is_at_end() {
            return false
        }

        let peeked = self.peek();
        if peeked.is_none() {
            return false
        }

        *peeked.unwrap() == t
    }

    fn peek(&self) -> Option<&Token> {
        self.token_stream.get(self.current)
    }

    /// Base method for parsing any type of expression. Gets the next token from the
    /// stream and matches it, calling the relevant function and appending the returned
    /// node to the return node.
    fn expr(&mut self) -> ParseNode {
        let next = self.get_next_token();

        match *next {
            Token::Number(number) => {
            },
            _ => {}
        }

        ParseNode::new(ParseNodeKind::Product)
    }

    // fn paren_expr(&mut self) -> ParseNode {
    //     self.get_next_token(); // Consume '('
    //
    //     let current_token = self.token_stream.get(self.current_token).unwrap();
    //     if *current_token != Token::RParen {
    //         panic!("FUCK ITS NOT A R PAREN NOOOOOOO");
    //     }
    //
    //     self.get_next_token(); // Consume ')'
    // }

    fn parse_ident_expr(&mut self) -> ParseNode {
        ParseNode::new(ParseNodeKind::Product)
    }

    fn get_next_token(&mut self) -> &Token {
        self.current += 1;

        let next_token = self.token_stream.get(self.current);
        next_token.unwrap()
    }
}

impl ReefDebuggable for Parser {
    fn debug_write_to_file(&self, file_path: &str) {
        todo!()
    }

    fn debug(&self) {
        todo!();
    }
}
