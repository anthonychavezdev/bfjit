use crate::Token;
use crate::TokenKind;
use std::io::stdin;
use std::io::Error;
use std::io::Read;

#[inline(always)]
pub fn run(tokens: Vec<Token>) -> Result<(), Error> {
    const MEM_SIZE: usize = 65536;
    let mut memory: [u8; MEM_SIZE] = [0; MEM_SIZE]; // u16 max + 1
    let mut pc: usize = 0;
    let mut idx: u16 = 0;

    let len: usize = tokens.len();
    while pc < len {
        let v: &Token = &tokens[pc];
        match v.get_token_type() {
            TokenKind::Right => {
                idx = idx.wrapping_add(v.get_successive_count());
                pc += 1;
            }
            TokenKind::Left => {
                idx = idx.wrapping_sub(v.get_successive_count());
                pc += 1;
            }
            TokenKind::Inc => {
                memory[idx as usize] =
                    memory[idx as usize].wrapping_add(v.get_successive_count() as u8);
                pc += 1;
            }
            TokenKind::Dec => {
                memory[idx as usize] =
                    memory[idx as usize].wrapping_sub(v.get_successive_count() as u8);
                pc += 1;
            }
            TokenKind::Output => {
                let times = v.get_successive_count();
                for _ in 0..times {
                    print!("{}", memory[idx as usize] as char);
                }
                pc += 1;
            }
            TokenKind::Input => {
                println!("Awaiting input...");
                let mut input: [u8; 1] = [0; 1];
                match stdin().read_exact(&mut input) {
                    Ok(_) => memory[idx as usize] = input[0],
                    Err(e) => return Err(e),
                }
                pc += 1;
            }
            TokenKind::JumpIfZero(address) => {
                if memory[idx as usize] == 0 {
                    pc = *address;
                } else {
                    pc += 1;
                }
            }
            TokenKind::JumpIfNZero(address) => {
                if memory[idx as usize] != 0 {
                    pc = *address;
                } else {
                    pc += 1;
                }
            }
        }
    }
    Ok(())
}
