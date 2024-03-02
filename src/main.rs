use crate::tokenizer::{Token, TokenKind};
use std::{env, fs};

mod tokenizer;
mod interpreter;

fn open_file(path: &str) -> Vec<u8> {
    match fs::read(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(2);
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("missing file operand".to_string());
    }

    let file: Vec<u8> = open_file(&args[1]);
    let tokens: Vec<Token> = tokenizer::tokenize(&file);
    interpreter::run(tokens);
    Ok(())
}
