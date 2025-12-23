use crate::lexer::token::{Token, TokenKind};
use crate::parser::Parser;

use super::Statement;
use super::expression::Expresion;
use core::fmt;
use std::fmt::Display;

pub struct ReturnStatement {
    pub value: Expresion,
}

impl Statement for ReturnStatement {}

impl ReturnStatement {
    pub fn new(value: Expresion) -> Self {
        ReturnStatement { value }
    }

    pub fn parse_return_statement(parser: &mut Parser) -> Option<ReturnStatement> {
        let mut exepresion: Vec<Token> = Vec::new();
        let peek: &Token = parser.tokens.peek()?;
        if peek.kind == TokenKind::RETURN {
            parser.tokens.next();
        } else {
            panic!("wrong token");
        }

        loop {
            let cur: Token = parser.tokens.next()?;
            if cur.kind == TokenKind::SemiColon {
                break;
            } else if cur.kind == TokenKind::EOF {
                panic!("wrong token");
            } else {
                exepresion.push(cur);
            }
        }

        let statement = ReturnStatement::new(Expresion::new(exepresion));
        return Some(statement);
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Return Statement:\n    Value: {:#?}", self.value)
    }
}
