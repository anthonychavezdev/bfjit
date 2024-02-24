use std::{env, fs};

#[derive(Debug)]
enum TokenKind {
    Right,
    Left,
    Inc,
    Dec,
    Output,
    Input,
    JumpIfZero,
    JumpIfNZero
}

#[derive(Debug)]
struct Token {
    token_kind: TokenKind,
}

fn read_file(path: &str) -> Vec<u8> {
    let contents: Result<Vec<u8>, std::io::Error> = fs::read(path);
    match contents {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e.to_string());
            std::process::exit(2);
        },
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("missing file operand".to_string());
    }
    let file_contents: Vec<u8> = read_file(&args[1]);
    let mut tokens: Vec<Token> = vec![];
    for c in file_contents {
        match c as char {
            '+' => {
                let token: Token = Token { token_kind: TokenKind::Inc };
                tokens.push(token);
            },
            '-' => {
                let token: Token = Token { token_kind: TokenKind::Dec };
                tokens.push(token);
            },
            '>' => {
                let token: Token = Token { token_kind: TokenKind::Right };
                tokens.push(token);
            },
            '<' => {
                let token: Token = Token { token_kind: TokenKind::Left };
                tokens.push(token);
            },
            '[' => {
                let token: Token = Token { token_kind: TokenKind::JumpIfZero };
                tokens.push(token);
            },
            ']' => {
                let token: Token = Token { token_kind: TokenKind::JumpIfNZero };
                tokens.push(token);
            },
            '.' => {
                let token: Token = Token { token_kind: TokenKind::Output };
                tokens.push(token);
            }
            ',' => {
                let token: Token = Token { token_kind: TokenKind::Input };
                tokens.push(token);
            }
            _ => {}
        }
    }
    dbg!(tokens);
    Ok(())
}
