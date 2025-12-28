use core::fmt;
use anyhow::Result;

use crate::{
    ast::{let_statement::LetStatement, return_statement::ReturnStatement},
    errors::ParserError,
    lexer::token::TokenKind,
    parser::Parser,
};

pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    fn new(statements: Vec<Stmt>) -> Self {
        Block { statements }
    }

    pub fn parse(parser: &mut Parser) -> Result<Block> {
        let mut statements = Vec::new();
        while let Some(next) = parser.tokens.peek() {
            match next.kind {
                TokenKind::LET => {
                    let st = Stmt::LetStmt(LetStatement::parse(parser)?);
                    statements.push(st);
                }
                TokenKind::RETURN => {
                    let st = Stmt::ReturnStmt(ReturnStatement::parse(parser)?);
                    statements.push(st);
                }
                TokenKind::RightCurlyParen => {
                    parser.tokens.next();
                    return Ok(Block::new(statements));
                }
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        token: next.clone(),
                    }
                    .into());
                }
            }
        }

        Ok(Block::new(statements))
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.statements {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

pub enum Stmt {
    LetStmt(LetStatement),
    ReturnStmt(ReturnStatement),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::LetStmt(stmt) => write!(f, "{}", stmt),
            Stmt::ReturnStmt(stmt) => write!(f, "{}", stmt),
        }
    }
}
