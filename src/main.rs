use crate::tokenizer::{Token, TokenKind};
use std::{env, fs};

mod tokenizer;
mod interpreter;

#[inline]
fn open_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("missing file operand".to_string());
    }

    let file: Vec<u8> = match open_file(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file\n");
            return Err(e.to_string())
        }
    };
    let tokens: Vec<Token> = match tokenizer::tokenize(&file) {
        Ok(t) => t,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    interpreter::run(tokens);
    Ok(())
}
