use std::{iter::Peekable, str::Chars};

use super::token::{Token, TokenKind};

#[cfg(test)]
mod tests;

pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    pub output: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input,
            chars: input.chars().peekable(),
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

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = match self.chars.next() {
            Some(c) => c,
            None => return Some(Token::new(TokenKind::EOF, String::from("EOF"))),
        };
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
}
