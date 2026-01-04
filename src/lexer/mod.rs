use anyhow::{Ok, Result};

use crate::errors::span::Span;
use crate::lexer::token::{OperatorKind, Token, TokenKind};
use crate::{
    errors::LexerError,
    lexer::token::{KeywordKind, PuncuationKind},
};
use std::{iter::Peekable, str::Chars};

#[cfg(test)]
mod tests;
pub mod token;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pub output: Vec<Token>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            chars: input.chars().peekable(),
            output: Vec::new(),
            cursor: 0,
        }
    }

    pub fn lex(&mut self) {
        while let Some(tok) = self.next() {
            if tok.kind == TokenKind::EOF {
                self.output.push(Token {
                    kind: TokenKind::EOF,
                    span: Span {
                        start: self.cursor,
                        end: self.cursor,
                    },
                });
                break;
            } else {
                self.output.push(tok);
            }
        }
    }

    fn read_number(&mut self) -> Result<TokenKind> {
        let mut number = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_digit(10) {
                number.push(*c);
                self.bump();
            } else {
                break;
            }
        }
        Ok(TokenKind::Number(number.parse().unwrap()))
    }

    fn match_keyword(&mut self, ident: &str) -> TokenKind {
        match ident {
            "if" => TokenKind::Keyword(KeywordKind::If),
            "else" => TokenKind::Keyword(KeywordKind::Else),
            "while" => TokenKind::Keyword(KeywordKind::While),
            "for" => TokenKind::Keyword(KeywordKind::For),
            "return" => TokenKind::Keyword(KeywordKind::Return),
            "func" => TokenKind::Keyword(KeywordKind::Func),
            "struct" => TokenKind::Keyword(KeywordKind::Struct),
            "let" => TokenKind::Keyword(KeywordKind::Let),
            _ => TokenKind::Ident(ident.to_string()),
        }
    }

    fn read_identifier(&mut self) -> Result<TokenKind> {
        let mut identifier = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_alphanumeric() || *c == '_' {
                identifier.push(*c);
                self.bump();
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
                self.bump();
            } else {
                break;
            }
        }
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.cursor += c.len_utf8();
        Some(c)
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let start = self.cursor;
        let mut do_next: bool = true;
        let c = match self.chars.peek() {
            Some(c) => c,
            None => {
                return Some(Token {
                    kind: TokenKind::EOF,
                    span: Span {
                        start: self.cursor,
                        end: self.cursor + 1,
                    },
                });
            }
        };
        let tok: Result<TokenKind> = match *c {
            '(' => Ok(TokenKind::Punctuation(PuncuationKind::LeftParen)),
            ')' => Ok(TokenKind::Punctuation(PuncuationKind::RightParen)),
            '{' => Ok(TokenKind::Punctuation(PuncuationKind::LeftCurlyParen)),
            '}' => Ok(TokenKind::Punctuation(PuncuationKind::RightCurlyParen)),
            '[' => Ok(TokenKind::Punctuation(PuncuationKind::LeftSquareParen)),
            ']' => Ok(TokenKind::Punctuation(PuncuationKind::RightSquareParen)),
            ':' => Ok(TokenKind::Punctuation(PuncuationKind::Colon)),
            ';' => Ok(TokenKind::Punctuation(PuncuationKind::SemiColon)),
            ',' => Ok(TokenKind::Punctuation(PuncuationKind::Comma)),
            '=' => {
                self.bump();
                do_next = false;
                let n = match self.chars.peek() {
                    Some(n) => n,
                    None => {
                        return Some(Token {
                            kind: TokenKind::EOF,
                            span: Span {
                                start: self.cursor,
                                end: self.cursor,
                            },
                        });
                    }
                };
                match *n {
                    '=' => Ok(TokenKind::Operator(OperatorKind::EqualEqual)),
                    _ => Ok(TokenKind::Operator(OperatorKind::Equal)),
                }
            }
            '+' => Ok(TokenKind::Operator(OperatorKind::Plus)),
            '-' => Ok(TokenKind::Operator(OperatorKind::Minus)),
            '*' => Ok(TokenKind::Operator(OperatorKind::Star)),
            '/' => Ok(TokenKind::Operator(OperatorKind::Slash)),
            '%' => Ok(TokenKind::Operator(OperatorKind::Percent)),
            '<' => Ok(TokenKind::Operator(OperatorKind::Less)),
            '>' => Ok(TokenKind::Operator(OperatorKind::Greater)),
            '!' => Ok(TokenKind::Operator(OperatorKind::Bang)),
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
            self.bump();
        }
        let end = self.cursor;
        return Some(Token {
            kind: tok.unwrap(),
            span: Span { start, end },
        });
    }
}
