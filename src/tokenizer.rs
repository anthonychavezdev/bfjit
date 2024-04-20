use core::fmt;

pub enum TokenKind {
    Right,              // >
    Left,               // <
    Inc,                // +
    Dec,                // -
    Output,             // .
    Input,              // ,
    JumpIfZero(usize),  // [
    JumpIfNZero(usize), // ]
}

pub struct Token {
    kind: TokenKind,
    successive_count: u16,
}

pub struct UnbalancedBracketsError {
    c: char,
    offset: usize
}
type Result<T> = std::result::Result<T, UnbalancedBracketsError>;

impl fmt::Display for UnbalancedBracketsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unbalanced {} at offset: {}", self.c, self.offset)
    }
}


impl Token {
    fn new(kind: TokenKind) -> Self {
        Token {
            kind,
            successive_count: 1,
        }
    }    

    pub fn get_token_type(&self) -> &TokenKind {
        &self.kind
    }

    pub fn get_successive_count(&self) -> u16 {
        self.successive_count
    }

}

#[inline]
fn is_valid_bf_op(c: u8) -> bool {
    match c {
        b'+' | b'-' | b'>' | b'<' | b'[' | b']' | b'.' | b',' => true,
        _ => false
    }
}

pub fn tokenize(file_contents: &Vec<u8>) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::with_capacity(file_contents.len());
    let mut stack: Vec<usize> = Vec::with_capacity(file_contents.len());
    let mut char_iter = file_contents.iter().enumerate().peekable();
    let mut file_offset: usize = 0;
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
                file_offset = idx;
                let current_address = tokens.len();
                stack.push(current_address + 1);
                // The 0 will be replaced when we find a ']'
                TokenKind::JumpIfZero(0)
            },
            b']' => {
                file_offset = idx;
                let jump_to_address: usize = tokens.len();
                match stack.pop() {
                    Some(address) => {
                        tokens[address-1] = Token::new(TokenKind::JumpIfZero(jump_to_address + 1));
                        TokenKind::JumpIfNZero(address)
                    }
                    None => {
                        return Err(UnbalancedBracketsError { c: ']', offset: file_offset});
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
    if let Some(_) = stack.pop() {
        return Err(UnbalancedBracketsError { c: '[', offset: file_offset});
    }
    Ok(tokens)
}
