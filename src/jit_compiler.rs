use memmap2::MmapOptions;

use crate::Token;
use crate::TokenKind;

struct BackPatch {
    jump_ins_arg_offset: usize,
    src_jump_addr: usize,
    dst_jump_adrr: usize
}

pub fn run(tokens: Vec<Token>) -> Result<(), std::io::Error> {
    const MEM_SIZE: usize = 65536;
    let mut memory: [u8; MEM_SIZE] = [0; MEM_SIZE]; // u16 max + 1
    let mut pc: usize = 0;
    let len: usize = tokens.len();
    let mut backpatch_stack: Vec<BackPatch> = vec![];
    let mut address_stack: Vec<usize> = vec![];
    let mut mmap_options = MmapOptions::new();
    let mut asm_ins: Vec<u8> = Vec::new();

    while pc < len {
        let v: &Token = &tokens[pc];
        address_stack.push(asm_ins.len());
        match v.get_token_type() {
            TokenKind::Right => {
                let amount = v.get_successive_count() as u32;
                asm_ins.extend_from_slice(&[0x48, 0x81, 0xc7]);
                // add rdi, amount
                asm_ins.extend_from_slice(&amount.to_le_bytes());
                pc += 1;
            }
            TokenKind::Left => {
                let amount = v.get_successive_count() as u32;
                asm_ins.extend_from_slice(&[0x48, 0x81, 0xef]);
                // sub rdi, amount
                asm_ins.extend_from_slice(&amount.to_le_bytes());
                pc += 1;
            }
            TokenKind::Inc => {
                let amount = v.get_successive_count() as u8;
                asm_ins.extend_from_slice(&[0x80, 0x07, amount]); // add byte [rdi], amount
                pc += 1
            }
            TokenKind::Dec => {
                let amount = v.get_successive_count() as u8;
                asm_ins.extend_from_slice(&[0x80, 0x2f, amount]); // sub byte [rdi], amount
                pc += 1;
            }
            TokenKind::Output => {
                let times = v.get_successive_count();
                for _ in 0..times {
                    asm_ins.extend_from_slice(&[0x57,                                     // push rdi
                                                0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // mov rax, 1
                                                0x48, 0x89, 0xfe,                         // mov rsi, rdi
                                                0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00, // mov rdi, 1
                                                0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, // mov rdx, 1
                                                0x0f, 0x05,                               // syscall
                                                0x5f                                      // pop rdi
                    ]);
                }
                pc += 1;
            }
            TokenKind::Input => {
                println!("Awaiting input...");
                asm_ins.extend_from_slice(&[0x57,                                        // push rdi
                                            0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00,    // mov rax, 0
                                            0x48, 0x89, 0xfe,                            // mov rsi, rdi
                                            0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00,    // mov rdi, 0
                                            0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00,    // mov rdx, 1
                                            0x0f, 0x05,                                  // syscall
                                            0x5f                                         // pop rdi

                ]);
                pc += 1;
            }
            TokenKind::JumpIfZero(address) => {
                asm_ins.extend_from_slice(&[0x8a, 0x07,   // mov al, byte [rdi]
                                            0x84, 0xc0,   // test al, al (bitwise AND)
                                            0x0f, 0x84    // jz
                ]);

                let addr_arg_beginning: usize = asm_ins.len();
                // The 0x90s are NOPs
                asm_ins.extend_from_slice(&[0x90, 0x90, 0x90, 0x90]); // jz argument (will be replaced later)
                let addr_arg_ending: usize = asm_ins.len();
                let jump_address = BackPatch {
                    jump_ins_arg_offset: addr_arg_beginning,
                    src_jump_addr: addr_arg_ending,
                    dst_jump_adrr: *address
                };
                backpatch_stack.push(jump_address);
                pc += 1;
            }
            TokenKind::JumpIfNZero(address) => {
                asm_ins.extend_from_slice(&[0x8a, 0x07,   // mov al, byte [rdi]
                                            0x84, 0xc0,   // test al, al (bitwise AND)
                                            0x0f, 0x85    // jnz
                ]);

                let addr_arg_beginning = asm_ins.len();
                // The 0x90s are NOPs
                asm_ins.extend_from_slice(&[0x90, 0x90, 0x90, 0x90]); // jnz argument (will be replaced later)
                let addr_arg_ending = asm_ins.len();

                let jump_address = BackPatch {
                    jump_ins_arg_offset: addr_arg_beginning,
                    src_jump_addr: addr_arg_ending,
                    dst_jump_adrr: *address
                };
                backpatch_stack.push(jump_address);

                pc += 1;
            }
        }
    }

    address_stack.push(asm_ins.len());
    for b in backpatch_stack {
        let src_addr = b.src_jump_addr as u32;
        let dst_addr = address_stack[b.dst_jump_adrr] as u32;
        let addr_arg = dst_addr.wrapping_sub(src_addr);
        // The 4 0x90s are being replaced here
        asm_ins[b.jump_ins_arg_offset..b.jump_ins_arg_offset + 4].copy_from_slice( &addr_arg.to_le_bytes());
    }


    asm_ins.push(0xc3); // ret
    let mut mmap = mmap_options.len(asm_ins.len())
        .map_anon()?;
    mmap.copy_from_slice(&asm_ins);
    let mmap = mmap.make_exec()?;
    let code_ptr = mmap.as_ptr();
    let code_fn: extern "C" fn(*const u8) -> () = unsafe {
        std::mem::transmute(code_ptr)
    };
    code_fn(memory.as_mut_ptr());
    Ok(())
}

