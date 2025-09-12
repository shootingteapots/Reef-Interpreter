/*
    This is the main file for the reef interpreter. It is a standalone
    executable project that includes the reef-core as an external library.
*/

#![allow(unused)]

use reef_core::{parser::Parser, scanner::Scanner, ReefDebuggable, Token};
use std::{env, fs::read_to_string, path::Path};
use std::collections::HashMap;

const DEBUG_ENV_VAR: &str = "REEF_DEBUG"; // The environment variable for debug mode. If 1, run debug functions

fn main() {
    let args: Vec<String> = env::args().collect();

}

/// Wrapper for `fs::read_to_string` which unwraps the text or panics
fn read_file(file_path: &Path) -> String {
    let res = read_to_string(file_path);
    if res.is_err() {
        panic!("{}", res.unwrap_err());
    }

    res.unwrap()
}
