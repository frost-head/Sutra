use crate::ast::expression::Expression;
use crate::errors::ParserError;
use crate::lexer::token::{KeywordKind, Token};
use crate::parser::Parser;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Expression,
}

impl ReturnStatement {
    pub fn new(value: Expression) -> Self {
        ReturnStatement { value }
    }

    pub fn parse(parser: &mut Parser) -> Result<ReturnStatement> {
        let expression: Expression;
        let peek: &Token = parser.peek()?;
        if *peek == Token::Keyword(KeywordKind::Return) {
            parser.consume()?;
        } else {
            return Err(ParserError::ExpectedTokenGotUnexpected {
                kind: Token::Keyword(KeywordKind::Return),
                got: peek.clone(),
            }
            .into());
        }

        expression = Expression::parse(parser)?;
        let statement = ReturnStatement::new(expression);
        return Ok(statement);
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Return Statement:\n    Value: {}", self.value)
    }
}
