use std::{env, fs};

#[derive(Debug)]
enum TokenKind {
    Right,       // >
    Left,        // <
    Inc,         // +
    Dec,         // -
    Output,      // .
    Input,       // ,
    JumpIfZero,  // [
    JumpIfNZero, // ]
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    successive_count: usize,
    op: char,
    jumpAddr: usize
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Token {
            kind,
            successive_count: 1,
            op: ' ',
            jumpAddr: 0
        }
    }
}

fn read_file(path: &str) -> Vec<u8> {
    match fs::read(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(2);
        }
    }
}

fn is_valid_bf_op(c: u8) -> bool {
    match c {
        b'+' | b'-' | b'>' | b'<' | b'[' | b']' | b'.' | b',' => true,
        _ => false
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("missing file operand".to_string());
    }

    let file_contents = read_file(&args[1]);
    let mut tokens: Vec<Token> = vec![];
    let mut stack: Vec<usize> = vec![];
    let mut char_iter = file_contents.iter().enumerate().peekable();

    while let Some((_, &c)) = char_iter.next() {
        if !is_valid_bf_op(c) {
            continue;
        }

        let mut token = Token::new(match c {
            b'+' => TokenKind::Inc,
            b'-' => TokenKind::Dec,
            b'>' => TokenKind::Right,
            b'<' => TokenKind::Left,
            b'[' => TokenKind::JumpIfZero,
            b']' => TokenKind::JumpIfNZero,
            b'.' => TokenKind::Output,
            b',' => TokenKind::Input,
            _ => continue, // Ignore unrecognized characters
        });

        if c == b'[' {
            let len: usize = tokens.len();
            stack.push(len + 1);
        } else if c == b']' {
            if stack.is_empty() {
                eprintln!("unbalanced brackets!");
                std::process::exit(3);
            }
            let len: usize = tokens.len();
            let pop = stack.pop();
            token.jumpAddr = pop.unwrap();
            if let Some(v) = tokens.get_mut(token.jumpAddr - 1) {
                v.jumpAddr = len + 1;
            }
        }

        // Count successive occurrences of the same character
        while let Some((_, &next_c)) = char_iter.peek() {
            if next_c == c {
                token.successive_count += 1;
                char_iter.next();
            } else {
                break;
            }
        }

        token.op = c as char;
        tokens.push(token);
    }

    dbg!(tokens);
    Ok(())
}
