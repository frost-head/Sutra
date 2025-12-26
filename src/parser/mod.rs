use anyhow::Result;

use crate::{
    ast::{Ast, let_statement::LetStatement, return_statement::ReturnStatement},
    lexer::{Lexer, token::TokenKind},
};
use crate::errors::LexerError;
use std::iter::Peekable;

pub struct Parser<'a> {
    pub tokens: Peekable<Lexer<'a>>,
    pub ast: Ast,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            tokens: lexer.peekable(),
            ast: Ast {
                statements: Vec::new(),
            },
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        while let Some(next) = self.tokens.peek() {
            match next.kind {
                TokenKind::EOF => break,
                TokenKind::LET => {
                    let st = LetStatement::parse_let_statement(self)
                        .expect("Failed to parse let statement : ");
                    self.ast.statements.push(Box::new(st));
                }
                TokenKind::RETURN => {
                    let st = ReturnStatement::parse_return_statement(self)
                        .expect("Failed to parse return statement : ");
                    self.ast.statements.push(Box::new(st));
                }
                _ => {
                    return Err(LexerError::UnexpectedToken {
                        token: next.clone(),
                    }
                    .into());
                }
            }
        }
        Ok(())
    }
}
