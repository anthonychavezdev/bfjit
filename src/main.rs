use clap::Parser;

use crate::tokenizer::{Token, TokenKind};
use std::fs;

mod interpreter;
mod jit_compiler;
mod tokenizer;


#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// path to a bf program
    #[arg(short, long)]
    file_path: String,
    /// Use interpeter
    #[arg(short, long, default_value_t = false)]
    interpreter: bool,
    /// Use JIT compilation (default)
    #[arg(short, long, default_value_t = true)]
    jit_compiler: bool
}

#[inline]
fn open_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path)
}

fn main() -> Result<(), String> {
    let args: Args = Args::parse();

    let file: Vec<u8> = match open_file(&args.file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file\n");
            return Err(e.to_string());
        }
    };

    let tokens: Vec<Token> = match tokenizer::tokenize(&file) {
        Ok(t) => t,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    if args.interpreter {
        if let Err(e) = interpreter::run(tokens) {
            eprintln!("error during program execution");
            return Err(e.to_string());
        };
    } else if args.jit_compiler {
        if let Err(e) = jit_compiler::run(tokens) {
            eprintln!("error during program execution");
            return Err(e.to_string());
        }
    }

    Ok(())
}
