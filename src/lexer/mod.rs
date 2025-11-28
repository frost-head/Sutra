use std::{iter::Peekable, str::Chars};

use crate::lexer::token::{Token, TokenKind};

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
            println!("fn num out {}", c);
            if c.is_digit(10) {
                println!("fn num in {}", c);
                number.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        Token::new(TokenKind::INT, number)
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

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(c) = self.chars.peek() {
            println!("fn ind out {}", c);
            if c.is_alphanumeric() || *c == '_' {
                println!("fn ind in {}", c);
                identifier.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        // TODO Check if identifier is a keyword
        self.match_keyword(&identifier)
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            println!("white out {}", c);
            if c.is_whitespace() {
                print!("white in {}", c);
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
        println!("next out {}", c);
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
            _ => {
                if c.is_alphabetic() {
                    do_next = false; // read_identifier performs self.chars.next() to stop doing it twice we need this flag
                    self.read_identifier()
                } else if c.is_numeric() {
                    do_next = false; // read_identifier performs self.chars.next() to stop doing it twice we need this flag
                    self.read_number()
                } else {
                    Token::new(TokenKind::Illegal, c.to_string())
                }
            }
        };
        if do_next {
            self.chars.next();
        }
        return Some(tok);
    }
}
