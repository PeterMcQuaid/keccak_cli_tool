//! # Keccak CLI Tool
//!
//! `keccak_cli_tool` binary crate performs a keccak256 hash on any standard input
//! arguments, either as a string or a file if it exists

use std::env;
use keccak_cli_tool::hash_logic;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);     // Remove the first argument, which is the name of the program
    hash_logic(args);
}