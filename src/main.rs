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
    successive_count: u16,
    op: char,
    jump_addr: usize
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Token {
            kind,
            successive_count: 1,
            op: ' ',
            jump_addr: 0
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

    while let Some((idx, &c)) = char_iter.next() {
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
                eprintln!("unbalanced brackets! pos: {}", idx);
                std::process::exit(3);
            }
            let len: usize = tokens.len();
            let pop = stack.pop();
            token.jump_addr = pop.unwrap();
            if let Some(v) = tokens.get_mut(token.jump_addr - 1) {
                v.jump_addr = len + 1;
            }
        }

        if c != b'[' && c != b']' {
            // Count successive occurrences of the same character
            while let Some((_, &next_c)) = char_iter.peek() {
                if next_c == c {
                    token.successive_count += 1;
                    char_iter.next();
                } else {
                    break;
                }
            }
        }
        token.op = c as char;
        tokens.push(token);
    }

    for (idx, v) in tokens.iter().enumerate() {
        println!("{}: {} ({}) Jump: {}", idx, v.op as char, v.successive_count, v.jump_addr);
    }

    let mut memory: [u8; 65536] = [0; 65536]; // u16 max + 1
    let mut pc: usize = 0;
    let mut idx: u16 = 0;

    let len: usize = tokens.len();
    while pc < len {
        let v: &Token = &tokens[pc];
        match v.kind {
            TokenKind::Right => {
                idx = idx.wrapping_add(v.successive_count as u16);
                pc += 1;
            }
            TokenKind::Left => {
                idx = idx.wrapping_sub(v.successive_count as u16);
                pc += 1;
            }
            TokenKind::Inc => {
                memory[idx as usize] = memory[idx as usize].wrapping_add(v.successive_count as u8);
                pc += 1;
            }
            TokenKind::Dec => {
                memory[idx as usize] = memory[idx as usize].wrapping_sub(v.successive_count as u8);
                pc += 1;
            }
            TokenKind::Output => {
                for _ in 0..v.successive_count {
                    print!("{}", memory[idx as usize] as char);
                }
                pc += 1;
            }
            TokenKind::Input => {
                todo!();
            }
            TokenKind::JumpIfZero => {
                if memory[idx as usize] == 0 {
                    pc = v.jump_addr;
                } else {
                    pc += 1;
                }
            }
            TokenKind::JumpIfNZero => {
                if memory[idx as usize] != 0 {
                    pc = v.jump_addr;
                } else {
                    pc += 1;
                }
            }
        }
    }

    Ok(())
}
