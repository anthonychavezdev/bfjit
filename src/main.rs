use std::{env, fs, io::{stdin, Read}};

#[derive(Debug)]
enum TokenKind {
    Right,              // >
    Left,               // <
    Inc,                // +
    Dec,                // -
    Output,             // .
    Input,              // ,
    JumpIfZero(usize),  // [
    JumpIfNZero(usize), // ]
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    successive_count: u16,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Token {
            kind,
            successive_count: 1,
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
            b'[' => {
                let current_address = tokens.len();
                    stack.push(current_address + 1);
                // The 0 will be replaced when we find a ']'
                TokenKind::JumpIfZero(0)
            },
            b']' => {
                let jump_to_address: usize = tokens.len();
                match stack.pop() {
                    Some(address) => {
                        tokens[address-1] = Token::new(TokenKind::JumpIfZero(jump_to_address + 1));
                        TokenKind::JumpIfNZero(address)
                    }
                    None => {
                        eprintln!("unbalanced brackets! pos: {}", idx);
                        std::process::exit(3);
                    }
                }
            },
            b'.' => TokenKind::Output,
            b',' => TokenKind::Input,
            _ => continue, // Ignore unrecognized characters
        });

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
        tokens.push(token);
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
                println!("Awaiting input...");
                let mut input: [u8; 1] = [0; 1];
                stdin().read_exact(&mut input).expect("ERROR: reading input");
                memory[idx as usize] = input[0];
                pc += 1;
            }
            TokenKind::JumpIfZero(address) => {
                if memory[idx as usize] == 0 {
                    pc = address;
                } else {
                    pc += 1;
                }
            }
            TokenKind::JumpIfNZero(address) => {
                if memory[idx as usize] != 0 {
                    pc = address;
                } else {
                    pc += 1;
                }
            }
        }
    }

    Ok(())
}
