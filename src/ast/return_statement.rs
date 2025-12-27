use super::expression::Expresion;
use crate::errors::LexerError;
use crate::lexer::token::{Token, TokenKind};
use crate::parser::Parser;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;

pub struct ReturnStatement {
    pub value: Expresion,
}

impl ReturnStatement {
    pub fn new(value: Expresion) -> Self {
        ReturnStatement { value }
    }

    pub fn parse_return_statement(parser: &mut Parser) -> Result<ReturnStatement> {
        let mut exepresion: Vec<Token> = Vec::new();
        let peek: &Token = parser.tokens.peek().unwrap();
        if peek.kind == TokenKind::RETURN {
            parser.tokens.next();
        } else {
            return Err(LexerError::ExpectedTokenGotUnexpected {
                kind: TokenKind::RETURN,
                token: peek.clone(),
            }
            .into());
        }

        loop {
            let cur: Token = parser.tokens.next().unwrap();
            if cur.kind == TokenKind::SemiColon {
                break;
            } else if cur.kind == TokenKind::EOF {
                return Err(LexerError::UnexpectedEndOfInput.into());
            } else {
                exepresion.push(cur);
            }
        }

        let statement = ReturnStatement::new(Expresion::new(exepresion));
        return Ok(statement);
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Return Statement:\n    Value: {:#?}", self.value)
    }
}
