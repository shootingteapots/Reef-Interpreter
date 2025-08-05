#![allow(unused)]

use std::collections::HashMap;

use crate::TokenKind;

/// Scanner is responsible for converting text input into a stream of tokens
/// which represent the smallest components of a program.
#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    pub tokens: Vec<TokenKind>,

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
            tokens: vec![],
            ptr: 0,
            line: 1,
            keywords: keyword_map,
        }
    }

    /// Scan the input text and break it down into the smallest components.
    /// Token definitions can be found in ./lib.rs
    pub fn scan(&mut self) -> &Vec<TokenKind> {
        let text_len = self.text.len();

        while self.ptr < text_len {
            let c = self.text[self.ptr].into();

            match c {
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
                    self.ptr += 1;
                }
                ':' => {
                    self.add_token(TokenKind::Colon);
                    self.ptr += 1;
                }
                ';' => {
                    self.add_token(TokenKind::Semicolon);
                    self.ptr += 1;
                }
                '(' => {
                    self.add_token(TokenKind::LParen);
                    self.ptr += 1;
                }
                ')' => {
                    self.add_token(TokenKind::RParen);
                    self.ptr += 1;
                }
                '{' => {
                    self.add_token(TokenKind::LBrace);
                    self.ptr += 1;
                }
                '}' => {
                    self.add_token(TokenKind::RBrace);
                    self.ptr += 1;
                }
                ',' => {
                    self.add_token(TokenKind::Comma);
                    self.ptr += 1;
                }
                '.' => {
                    self.add_token(TokenKind::Dot);
                    self.ptr += 1;
                }
                c if c.is_whitespace() => {
                    self.ptr += 1;
                }
                _ => {
                    panic!("Panic: Unrecognised character {}", c);
                }
            };
        }

        self.add_token(TokenKind::EndOfFile);

        &self.tokens
    }

    /// Scan characters that make up an int/float and convert it into a 64 bit
    /// floating point number. This method can panic if there are multiple "."s
    /// in a number, so PLEASE DONT DO THAT!!!
    fn scan_number(&mut self) -> TokenKind {
        let mut collected: String = String::new();

        // Add the first character of the identifier
        collected.push(self.text[self.ptr].into());
        self.ptr += 1;

        loop {
            let c: char;
            if self.ptr < self.text.len() {
                c = self.text[self.ptr].into();
            } else {
                // Prevent indexing out of bounds
                break;
            }

            if c.is_ascii_digit() || c == '.' {
                self.ptr += 1;
                collected.push(c);
                continue;
            } else if c == '_' {
                self.ptr += 1;
                continue;
            } else {
                break;
            }
        }

        let number = collected.parse::<f64>();
        if number.is_ok() {
            TokenKind::Number(number.unwrap())
        } else {
            panic!("Failed to parse {} to a number.", collected);
        }
    }

    /// Constructs an operator token, starting with one of a select few
    /// characters.
    fn scan_operator(&mut self) -> TokenKind {
        let mut collected = String::new();

        match self.text[self.ptr].into() {
            '=' => {
                collected.push(self.text[self.ptr].into());
                self.ptr += 1;
            }
            '+' | '/' | '*' | '<' | '>' => {
                collected.push(self.text[self.ptr].into());
                self.ptr += 1;
            }
            _ => {}
        };

        TokenKind::Operator(collected)
    }

    /// There are multiple different things a hyphen can be contructed into,
    /// so I put the logic for scanning tokens that start with a hyphen in
    /// its own function.
    fn handle_hyphen(&mut self) {
        self.ptr += 1;

        let next: char = self.text[self.ptr].into();

        if next == '-' {
            self.scan_comment();
        } else if next == '>' {
            self.add_token(TokenKind::Operator(String::from("->")));
            self.ptr += 1;
        } else {
            self.add_token(TokenKind::Operator(String::from("-")));
        }
    }

    /// Discard everything after the comment start until a newline character
    /// is reached.
    fn scan_comment(&mut self) {
        let mut c: char = self.text[self.ptr].into();

        while c != '\n' {
            self.ptr += 1;
            if self.ptr < self.text.len() {
                c = self.text[self.ptr].into();
            } else {
                break;
            }
        }
    }

    /// Scans user defined identifiers, or if the identifier matches the name
    /// of a keyword, return a keyword token instead.
    fn scan_ident(&mut self) -> TokenKind {
        let mut collected = String::new();

        // Add the first character of the identifier
        collected.push(self.text[self.ptr].into());
        self.ptr += 1;

        loop {
            let c: char;
            if self.ptr < self.text.len() {
                c = self.text[self.ptr].into();
            } else {
                break;
            }

            if c.is_ascii_alphanumeric() {
                self.ptr += 1;
                collected.push(c);
                continue;
            } else if c == '_' {
                self.ptr += 1;
                continue;
            } else {
                // Stop because the number has ended
                break;
            }
        }

        if self.is_keyword(&collected) {
            TokenKind::Keyword(collected)
        } else {
            TokenKind::Ident(collected)
        }
    }

    /// Scans a string. A string starts and ends with a double quote, with the
    /// text in between them.
    fn scan_string(&mut self) -> TokenKind {
        // Consume the first double quote
        self.ptr += 1;
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
                self.ptr += 1;
                collected.push(c);
            } else {
                // Consume the ending double quote
                self.ptr += 1;
                break;
            };
        }

        TokenKind::String(collected)
    }

    /// Add a token to the list of tokens stored in the scanner state.
    fn add_token(&mut self, token: TokenKind) {
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
}
