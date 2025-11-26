use super::token::{Token, TokenKind};

#[cfg(test)]
mod tests;

pub struct Lexer {
    input: String,
    pos: usize,
    pub output: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            pos: 0,
            output: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        loop {
            let tok = self.next();
            let tok = match tok {
                Some(tok) => tok,
                None => panic!("Unknown toknen"), // TODO remove panic
            };
            if tok.kind == TokenKind::EOF {
                self.output.push(tok);
                break;
            } else {
                self.output.push(tok);
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return Some(Token::new(TokenKind::EOF, String::from("EOF")));
        }

        if let Some((_, c)) = self.input[self.pos..].char_indices().next() {
            let char_len_bytes = c.len_utf8();
            self.pos += char_len_bytes;
            return Some(match c {
                '(' => Token::new(TokenKind::LeftParen, c.to_string()),
                ')' => Token::new(TokenKind::RightParen, c.to_string()),
                '{' => Token::new(TokenKind::LeftCurlyParen, c.to_string()),
                '}' => Token::new(TokenKind::RightCurlyParen, c.to_string()),
                '[' => Token::new(TokenKind::LeftSquareParen, c.to_string()),
                ']' => Token::new(TokenKind::RightSquareParen, c.to_string()),
                '<' => Token::new(TokenKind::LeftAngleParen, c.to_string()),
                '>' => Token::new(TokenKind::RightAngleParen, c.to_string()),
                ':' => Token::new(TokenKind::Colon, c.to_string()),
                ';' => Token::new(TokenKind::SemiColon, c.to_string()),
                ',' => Token::new(TokenKind::Comma, c.to_string()),
                _ => panic!("Unexpected character: {}", c),
            });
        }

        return None;
    }
}
