#![allow(unused)]

use crate::{ReefDebuggable, Token};
use std::str::Chars;
use std::thread::current;
use std::{collections::HashMap, fs::write, path::Path};

/// Scanner is responsible for converting text input into a stream of tokens
/// which represent the smallest components of a program. It is a struct so
/// it can keep track of its state and so that the state is shared between
/// the methods.
#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    tokens: Vec<Token>,
    text: Vec<u8>,
    current: usize,
    line: i32,
    keywords: HashMap<&'a str, &'a str>,
}

impl<'a> Scanner<'a> {
    /// Construct a new Scanner, taking the text to scan as the only argument.
    pub fn new(text: &'a str) -> Self {
        let mut keyword_map: HashMap<&str, &str> = HashMap::new();

        // Populate the keyword list with the language keywords.
        keyword_map.insert("continue", "continue");
        keyword_map.insert("struct", "struct");
        keyword_map.insert("elseif", "elseif");
        keyword_map.insert("return", "return");
        keyword_map.insert("typeof", "typeof");
        keyword_map.insert("false", "false");
        keyword_map.insert("break", "break");
        keyword_map.insert("true", "true");
        keyword_map.insert("else", "else");
        keyword_map.insert("then", "then");
        keyword_map.insert("type", "type");
        keyword_map.insert("for", "for");
        keyword_map.insert("fun", "fun");
        keyword_map.insert("nil", "nil");
        keyword_map.insert("not", "not");
        keyword_map.insert("and", "and");
        keyword_map.insert("var", "var");
        keyword_map.insert("log", "log");
        keyword_map.insert("do", "do");
        keyword_map.insert("if", "if");
        keyword_map.insert("or", "or");

        Self {
            text: String::from(text).into_bytes(),
            tokens: Vec::new(),
            current: 0,
            line: 1,
            keywords: keyword_map,
        }
    }

    /// Scan the input text and break it down into the smallest components.
    /// Token definitions can be found in ./lib.rs
    pub fn scan(&mut self) {
        let mut current_char: Option<&u8>;
        while !self.is_at_end() {
            current_char = self.text.get(self.current);

            let c = current_char.unwrap();
            match char::from(*c) {
                '\n' => {
                    self.line += 1;
                    self.current += 1;
                }
                '+' | '=' | '<' | '>' | '*' | '/' => {
                    let tk = self.scan_operator();
                    self.add_token(tk);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let tk = self.scan_ident();
                    self.add_token(tk);
                }
                '0'..='9' => {
                    let tk = self.scan_number();
                    self.add_token(tk);
                }
                '-' => self.handle_hyphen(),
                '"' => {
                    let tk = self.scan_string();
                    self.add_token(tk);
                    self.advance();
                }
                ':' => {
                    self.add_token(Token::Colon);
                    self.advance();
                }
                ';' => {
                    self.add_token(Token::Semicolon);
                    self.advance();
                }
                '(' => {
                    self.add_token(Token::LParen);
                    self.advance();
                }
                ')' => {
                    self.add_token(Token::RParen);
                    self.advance();
                }
                '{' => {
                    self.add_token(Token::LBrace);
                    self.advance();
                }
                '}' => {
                    self.add_token(Token::RBrace);
                    self.advance();
                }
                ',' => {
                    self.add_token(Token::Comma);
                    self.advance();
                }
                '.' => {
                    self.add_token(Token::Dot);
                    self.advance();
                }
                c if c.is_whitespace() => {
                    self.advance();
                }
                _ => {
                    panic!("Panic: Unrecognised character {}", c);
                }
            };
        }

        self.add_token(Token::EndOfFile);
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current < self.text.len()
    }

    /// Add a token to the list of tokens stored in the scanner state.
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    /// Check an identifier against the built-in hashmap of keywords, and returns true if it matches a keyword, else returns false.
    fn is_keyword(&self, ident: &str) -> bool {
        self.keywords.contains_key(ident)
    }

    /// Peek one character ahead.
    fn peek(&self) -> Option<&u8> {
        let next_char = self.text.get(self.current + 1);
        next_char
    }

    /// Increment the current char pointer and return the new value.
    fn advance(&mut self) -> usize {
        self.current += 1;
        self.current
    }

    /// Since a hyphen can be the start of multiple things, this function figures out
    /// which type of token it is supposed to be and calls the correct function to scan
    /// it fully.
    fn handle_hyphen(&mut self) {
        self.advance();

        let next_char = self.text.get(self.current);
        if next_char.is_none() {
            return;
        }

        let c = char::from(*next_char.unwrap());

        match c {
            '-' => {
                let comment_token = self.scan_comment();
                self.add_token(comment_token);
            }
            '>' => {
                self.add_token(Token::Operator(String::from("->")));
                self.advance();
            }
            _ => self.add_token(Token::Operator(String::from(c))),
        }
    }

    /// Scan characters that make up an int/float and convert it into a 64 bit
    /// floating point number. This method can panic if there are multiple "."
    fn scan_number(&mut self) -> Token {
        let mut collected = String::new();
        let first_char = self.text.get(self.current);
        if first_char.is_some() {
            // Add the first character of the identifier
            collected.push(char::from(*first_char.unwrap()));
            self.advance();

            loop {
                let mut current_char = self.text.get(self.current);
                if current_char.is_none() {
                    break;
                }
                let c = char::from(*current_char.unwrap());

                if c.is_ascii_digit() || c == '.' {
                    self.advance();
                    collected.push(c);
                    continue;
                } else if c == '_' {
                    self.advance();
                    continue;
                } else {
                    break;
                }
            }
        }

        let parsed_number = collected.parse::<f64>();
        if parsed_number.is_err() {
            panic!("Failed to parse {} to a number.", collected);
        }

        Token::Number(parsed_number.unwrap())
    }

    /// Constructs an operator token, starting with one of a select few
    /// characters.
    fn scan_operator(&mut self) -> Token {
        let mut collected = String::new();
        let current_char = char::from(*self.text.get(self.current).unwrap());
        dbg!(current_char);

        match current_char {
            '=' => {
                collected.push(current_char);
                self.advance();
            }
            '+' | '/' | '*' | '<' | '>' => {
                collected.push(current_char);
                self.advance();
            }
            _ => {}
        };

        Token::Operator(collected)
    }

    /// Save the contents of a comment as a string for potential use in the parser.
    fn scan_comment(&mut self) -> Token {
        let mut buf = String::new();
        let mut current_char = self.text.get(self.current);
        if current_char.is_some() {
            buf.push(char::from(*current_char.unwrap()));

            loop {
                current_char = self.text.get(self.current);
                if current_char.is_none() {
                    break;
                }

                buf.push(char::from(*current_char.unwrap()));
                self.advance();
            }
        }

        Token::Comment(buf)
    }

    /// Scans user defined identifiers, or if the identifier matches the name
    /// of a keyword, return a keyword token instead.
    fn scan_ident(&mut self) -> Token {
        let mut buf = String::new();
        let mut current_char = self.text.get(self.current);
        if current_char.is_some() {
            // Add the first character of the identifier and skip past it
            buf.push(char::from(*current_char.unwrap()));
            self.advance();

            loop {
                current_char = self.text.get(self.current);
                if current_char.is_none() {
                    break;
                }

                let c = char::from(*current_char.unwrap());
                if c.is_ascii_alphanumeric() {
                    self.advance();
                    buf.push(c);
                    continue;
                } else if c == '_' {
                    self.advance();
                    continue;
                } else {
                    // Stop because the number has ended
                    break;
                }
            }
        }

        if self.is_keyword(&buf) {
            Token::Keyword(buf)
        } else {
            Token::Ident(buf)
        }
    }

    /// Scans a string. A string starts and ends with a double quote, with the
    /// text in between them.
    fn scan_string(&mut self) -> Token {
        // Consume the first double quote
        self.advance();

        let mut buf = String::new();
        loop {
            // Get the character at index ptr
            let current_char = self.text.get(self.current);
            if current_char.is_none() {
                break;
            }

            let c = char::from(*current_char.unwrap());
            match c {
                '\"' => {
                    self.advance();
                    buf.push(c);
                }
                _ => {
                    self.advance();
                    break;
                }
            }
        }

        Token::String(buf)
    }
}

impl ReefDebuggable for Scanner<'_> {
    fn debug_write_to_file(&self, file_path: &str) {
        let mut buf = String::new();
        for token in &self.tokens {
            let token_as_str = token.to_string();
            let token_as_chars = token_as_str.chars();

            for char in token_as_chars {
                buf.push(char);
            }

            buf.push('\n');
        }

        write(Path::new(file_path), buf);
    }

    fn debug(&self) {}
}
