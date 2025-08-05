/*
    This is the main file for the reef interpreter. It is a standalone
    executable project that includes the reef-core as an external library.
*/

#![allow(unused)]

use reef_core::{parser::Parser, scanner::Scanner, TokenKind};
use std::{env, fs, path};

fn main() {
    // This whole function is an absolute mess, rewrite this later for the
    // love of god.

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let text = read_file(path::Path::new(file_path.as_str()));

    let mut scanner = Scanner::new(text.as_str());
    let tokens = scanner.scan();

    let mut parser = Parser::new(tokens.clone());
    parser.parse();

    write_tokens_to_debug_file(&tokens.clone());
}

/// Wrapper for `fs::read_to_string` which unwraps the text or panics because
/// I hate ERROR HANDLING HAHAHA!
fn read_file(file_path: &path::Path) -> String {
    let res = fs::read_to_string(file_path);

    if res.is_ok() {
        res.unwrap()
    } else {
        panic!("{}", res.unwrap_err());
    }
}

/// Takes a string of tokens and writes them to a fixed file path
/// located at `./_scanner_debug.txt`.
/// todo: allow users to input their own debug path, and only call this with a
/// special flag.
fn write_tokens_to_debug_file(tokens: &Vec<TokenKind>) {
    let mut s = String::new();

    for tk in tokens {
        let st = tk.to_string();
        let chars = st.chars();
        for c in chars {
            s.push(c);
        }
        s.push('\n');
    }

    fs::write(path::Path::new("./_scanner_debug.txt"), s);
}
