use anyhow::Result;
use core::fmt;

use crate::{
    ast::{let_statement::LetStatement, return_statement::ReturnStatement},
    errors::ParserError,
    lexer::token::{KeywordKind, PuncuationKind, Token},
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
        loop {
            let next = parser.peek()?;
            match next {
                Token::Keyword(KeywordKind::Let) => {
                    let st = Stmt::LetStmt(LetStatement::parse(parser)?);
                    statements.push(st);
                }
                Token::Keyword(KeywordKind::Return) => {
                    let st = Stmt::ReturnStmt(ReturnStatement::parse(parser)?);
                    statements.push(st);
                }
                Token::Punctuation(PuncuationKind::RightCurlyParen) => {
                    return Ok(Block::new(statements));
                },
                Token::EOF => {
                    break;
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
