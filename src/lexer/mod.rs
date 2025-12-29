use anyhow::{Ok, Result};

use crate::lexer::token::{OperatorKind, Token};
use crate::{
    errors::LexerError,
    lexer::token::{KeywordKind, PuncuationKind},
};
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
            if tok == Token::EOF {
                self.output.push(Token::EOF);
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
        Ok(Token::Number(number.parse().unwrap()))
    }

    fn match_keyword(&mut self, ident: &str) -> Token {
        match ident {
            "if" => Token::Keyword(KeywordKind::If),
            "else" => Token::Keyword(KeywordKind::Else),
            "while" => Token::Keyword(KeywordKind::While),
            "for" => Token::Keyword(KeywordKind::For),
            "return" => Token::Keyword(KeywordKind::Return),
            "func" => Token::Keyword(KeywordKind::Func),
            "struct" => Token::Keyword(KeywordKind::Struct),
            "let" => Token::Keyword(KeywordKind::Let),
            _ => Token::Ident(ident.to_string()),
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
            None => return Some(Token::EOF),
        };
        let tok: Result<Token> = match *c {
            '(' => Ok(Token::Punctuation(PuncuationKind::LeftParen)),
            ')' => Ok(Token::Punctuation(PuncuationKind::RightParen)),
            '{' => Ok(Token::Punctuation(PuncuationKind::LeftCurlyParen)),
            '}' => Ok(Token::Punctuation(PuncuationKind::RightCurlyParen)),
            '[' => Ok(Token::Punctuation(PuncuationKind::LeftSquareParen)),
            ']' => Ok(Token::Punctuation(PuncuationKind::RightSquareParen)),
            '<' => Ok(Token::Punctuation(PuncuationKind::LeftAngleParen)),
            '>' => Ok(Token::Punctuation(PuncuationKind::RightAngleParen)),
            ':' => Ok(Token::Punctuation(PuncuationKind::Colon)),
            ';' => Ok(Token::Punctuation(PuncuationKind::SemiColon)),
            ',' => Ok(Token::Punctuation(PuncuationKind::Comma)),
            '=' => Ok(Token::Operator(OperatorKind::Equal)),
            '+' => Ok(Token::Operator(OperatorKind::Plus)),
            '-' => Ok(Token::Operator(OperatorKind::Minus)),
            '*' => Ok(Token::Operator(OperatorKind::Star)),
            '/' => Ok(Token::Operator(OperatorKind::Slash)),
            '%' => Ok(Token::Operator(OperatorKind::Percent)),
            _ => {
                if c.is_alphabetic() {
                    do_next = false; // read_identifier performs self.chars.next() to stop doing it twice we need this flag
                    Ok(self.read_identifier().expect("failed to read identifier"))
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
