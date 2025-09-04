/*
    This is the main file for the reef interpreter. It is a standalone
    executable project that includes the reef-core as an external library.
*/

#![allow(unused)]

use reef_core::{parser::Parser, scanner::Scanner, ReefDebuggable, Token};
use std::{env::args, fs::read_to_string, path::Path};

fn main() {
    // This whole function is an absolute mess, rewrite this later for the
    // love of god.

    let args: Vec<String> = args().collect();
    let file_path = &args[1];
    let text = read_file(Path::new(file_path.as_str()));

    let mut scanner = Scanner::new(text.as_str());
    let tokens = scanner.scan();

    // let mut parser = Parser::new(tokens.clone());
    // let parse_node = parser.parse_tokens();

    scanner.debug_write_to_file("./_scanner_debug.txt");
}

/// Wrapper for `fs::read_to_string` which unwraps the text or panics because
/// I hate ERROR HANDLING HAHAHA!
fn read_file(file_path: &Path) -> String {
    let res = read_to_string(file_path);
    if res.is_ok() {
        res.unwrap()
    } else {
        panic!("{}", res.unwrap_err());
    }
}
