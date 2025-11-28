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
    
    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_digit(10) {
                number.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        Token::new(TokenKind::INT, number)
    }
    
    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_alphanumeric() || *c == '_' {
                identifier.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        // TODO Check if identifier is a keyword
        Token::new(TokenKind::IDENT, identifier)
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let c = match self.chars.peek() {
            Some(c) => c,
            None => return Some(Token::new(TokenKind::EOF, String::from("EOF"))),
        };
        let tok = match *c {
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
            '=' => Token::new(TokenKind::Equal, c.to_string()),
            '+' => Token::new(TokenKind::Plus, c.to_string()),
            '-' => Token::new(TokenKind::Minus, c.to_string()),
            '*' => Token::new(TokenKind::Star, c.to_string()),
            '/' => Token::new(TokenKind::Slash, c.to_string()),
            '%' => Token::new(TokenKind::Percent, c.to_string()),
            _ => if c.is_alphabetic() {
                self.read_identifier()
            } else if c.is_numeric() {
                self.read_number()
            } else {
                Token::new(TokenKind::Illegal, c.to_string())
            },
        };
        self.chars.next(); 
        return Some(tok);
    }
}
