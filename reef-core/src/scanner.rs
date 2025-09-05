#![allow(unused)]

use std::{collections::HashMap, fs::write, path::Path};

use crate::{ReefDebuggable, Token};

/// Scanner is responsible for converting text input into a stream of tokens
/// which represent the smallest components of a program.
#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    pub tokens: Vec<Token>,

    text: Vec<u8>,
    ptr: usize,
    line: i32,
    keywords: HashMap<&'a str, &'a str>,
}

impl<'a> Scanner<'a> {
    /// Construct a new Scanner, taking the text to scan as the only argument.
    pub fn new(text: &'a str) -> Self {
        let mut keyword_map: HashMap<&str, &str> = HashMap::new();

        // Populate the keyword list with the language keywords.
        // todo: maybe change this to be a static dictionary instead?
        keyword_map.insert("continue", "continue");
        keyword_map.insert("struct", "struct");
        keyword_map.insert("elseif", "elseif");
        keyword_map.insert("return", "return");
        keyword_map.insert("typeof", "typeof");
        keyword_map.insert("false", "false");
        keyword_map.insert("while", "while");
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
            ptr: 0,
            line: 1,
            keywords: keyword_map,
        }
    }

    /// Scan the input text and break it down into the smallest components.
    /// Token definitions can be found in ./lib.rs
    pub fn scan(&mut self) -> &Vec<Token> {
        let text_len = self.text.len();

        while self.ptr < text_len {
            let current_char = self.text[self.ptr].into();

            match current_char {
                '\n' => {
                    self.line += 1;
                    self.ptr += 1;
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
                    panic!("Panic: Unrecognised character {}", current_char);
                }
            };
        }

        self.add_token(Token::EndOfFile);

        &self.tokens
    }

    /// Add a token to the list of tokens stored in the scanner state.
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    /// Check an identifier against the built-in hashmap of keywords, and
    /// returns true if it matches a keyword, else returns false.
    fn is_keyword(&self, ident: &str) -> bool {
        if self.keywords.contains_key(ident) {
            true
        } else {
            false
        }
    }

    /// Peek one character ahead. if the next index is out of bounds, return
    /// None.
    fn peek(&self) -> Option<char> {
        let new_ptr = self.ptr + 1;

        if new_ptr < self.text.len() {
            Some(self.text[new_ptr].into())
        } else {
            None
        }
    }

    /// Increment the current char pointer and return the new value.
    fn advance(&mut self) -> usize {
        self.ptr += 1;

        self.ptr
    }

    /// There are multiple different things a hyphen can be contructed into,
    /// so I put the logic for scanning tokens that start with a hyphen in
    /// its own function.
    fn handle_hyphen(&mut self) {
        self.advance();

        let next_char: char = self.text[self.ptr].into();

        if next_char == '-' {
            let comment_token = self.scan_comment();
            self.add_token(comment_token);
        } else if next_char == '>' {
            self.add_token(Token::Operator(String::from("->")));
            self.advance();
        } else {
            self.add_token(Token::Operator(String::from("-")));
        }
    }

    /// Scan characters that make up an int/float and convert it into a 64 bit
    /// floating point number. This method can panic if there are multiple "."s
    /// in a number, so PLEASE DONT DO THAT!!!
    fn scan_number(&mut self) -> Token {
        let mut collected = String::new();

        // Add the first character of the identifier
        collected.push(self.text[self.ptr].into());
        self.advance();

        loop {
            let current_char: char;
            if self.ptr < self.text.len() {
                current_char = self.text[self.ptr].into();
            } else {
                // Prevent indexing out of bounds
                break;
            }

            if current_char.is_ascii_digit() || current_char == '.' {
                self.advance();
                collected.push(current_char);
                continue;
            } else if current_char == '_' {
                self.advance();
                continue;
            } else {
                break;
            }
        }

        let parsed_number = collected.parse::<f64>();
        if parsed_number.is_ok() {
            Token::Number(parsed_number.unwrap())
        } else {
            panic!("Failed to parse {} to a number.", collected);
        }
    }

    /// Constructs an operator token, starting with one of a select few
    /// characters.
    fn scan_operator(&mut self) -> Token {
        let mut collected = String::new();
        let current_char = self.text[self.ptr].into();

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
        let mut collected = String::new();
        let mut current_char: char = self.text[self.ptr].into();

        collected.push(current_char);

        while current_char != '\n' {
            if self.ptr < self.text.len() {
                current_char = self.text[self.ptr].into();
                collected.push(current_char)
            } else {
                break;
            }

            self.advance();
        }

        Token::Comment(collected)
    }

    /// Scans user defined identifiers, or if the identifier matches the name
    /// of a keyword, return a keyword token instead.
    fn scan_ident(&mut self) -> Token {
        let mut collected = String::new();

        // Add the first character of the identifier
        collected.push(self.text[self.ptr].into());
        self.advance();

        loop {
            let current_char: char;
            if self.ptr < self.text.len() {
                current_char = self.text[self.ptr].into();
            } else {
                break;
            }

            if current_char.is_ascii_alphanumeric() {
                self.advance();
                collected.push(current_char);
                continue;
            } else if current_char == '_' {
                self.advance();
                continue;
            } else {
                // Stop because the number has ended
                break;
            }
        }

        if self.is_keyword(&collected) {
            Token::Keyword(collected)
        } else {
            Token::Ident(collected)
        }
    }

    /// Scans a string. A string starts and ends with a double quote, with the
    /// text in between them.
    fn scan_string(&mut self) -> Token {
        // Consume the first double quote
        self.advance();
        let mut collected = String::new();

        loop {
            // Get the character at index ptr
            let c: char;
            if self.ptr < self.text.len() {
                c = self.text[self.ptr].into();
            } else {
                panic!("Reached end of file while scanning a string!");
            }

            if c != '\"' {
                self.advance();
                collected.push(c);
            } else {
                // Consume the ending double quote
                self.advance();
                break;
            };
        }

        Token::String(collected)
    }
}

impl<'a> ReefDebuggable for Scanner<'a> {
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
}
