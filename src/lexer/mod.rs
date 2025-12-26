use anyhow::{Ok, Result};

use crate::lexer::{
    token::{Token, TokenKind},
};
use crate::errors::LexerError;
use std::{iter::Peekable, str::Chars};

#[cfg(test)]
mod tests;
pub mod token;

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
        while let Some(tok) = self.next() {
            if tok.kind == TokenKind::EOF {
                self.output
                    .push(Token::new(TokenKind::EOF, String::from("EOF")));
                break;
            } else {
                self.output.push(tok);
            }
        }
    }

    fn read_number(&mut self) -> Result<Token> {
        let mut number = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_digit(10) {
                number.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        Ok(Token::new(TokenKind::INT, number))
    }

    fn match_keyword(&mut self, indent: &str) -> Token {
        match indent {
            "if" => Token::new(TokenKind::IF, "if".to_string()),
            "else" => Token::new(TokenKind::ELSE, "else".to_string()),
            "while" => Token::new(TokenKind::WHILE, "while".to_string()),
            "for" => Token::new(TokenKind::FOR, "for".to_string()),
            "return" => Token::new(TokenKind::RETURN, "return".to_string()),
            "func" => Token::new(TokenKind::FUNC, "func".to_string()),
            "struct" => Token::new(TokenKind::STRUCT, "struct".to_string()),
            "let" => Token::new(TokenKind::LET, "let".to_string()),
            _ => Token::new(TokenKind::IDENT, indent.to_string()),
        }
    }

    fn read_identifier(&mut self) -> Result<Token> {
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
        Ok(self.match_keyword(&identifier))
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
        let mut do_next: bool = true;
        let c = match self.chars.peek() {
            Some(c) => c,
            None => return Some(Token::new(TokenKind::EOF, String::from("EOF"))),
        };
        let tok: Result<Token> = match *c {
            '(' => Ok(Token::new(TokenKind::LeftParen, c.to_string())),
            ')' => Ok(Token::new(TokenKind::RightParen, c.to_string())),
            '{' => Ok(Token::new(TokenKind::LeftCurlyParen, c.to_string())),
            '}' => Ok(Token::new(TokenKind::RightCurlyParen, c.to_string())),
            '[' => Ok(Token::new(TokenKind::LeftSquareParen, c.to_string())),
            ']' => Ok(Token::new(TokenKind::RightSquareParen, c.to_string())),
            '<' => Ok(Token::new(TokenKind::LeftAngleParen, c.to_string())),
            '>' => Ok(Token::new(TokenKind::RightAngleParen, c.to_string())),
            ':' => Ok(Token::new(TokenKind::Colon, c.to_string())),
            ';' => Ok(Token::new(TokenKind::SemiColon, c.to_string())),
            ',' => Ok(Token::new(TokenKind::Comma, c.to_string())),
            '=' => Ok(Token::new(TokenKind::Equal, c.to_string())),
            '+' => Ok(Token::new(TokenKind::Plus, c.to_string())),
            '-' => Ok(Token::new(TokenKind::Minus, c.to_string())),
            '*' => Ok(Token::new(TokenKind::Star, c.to_string())),
            '/' => Ok(Token::new(TokenKind::Slash, c.to_string())),
            '%' => Ok(Token::new(TokenKind::Percent, c.to_string())),
            _ => {
                if c.is_alphabetic() {
                    do_next = false; // read_identifier performs self.chars.next() to stop doing it twice we need this flag
                    Ok(self.read_identifier().expect("failed to read indentifier"))
                } else if c.is_numeric() {
                    do_next = false; // read_identifier performs self.chars.next() to stop doing it twice we need this flag
                    Ok(self.read_number().expect("failed to read number"))
                } else {
                    Err(LexerError::UnexpectedCharacter(*c).into())
                }
            }
        };

        if do_next {
            self.chars.next();
        }
        return Some(tok.unwrap());
    }
}
