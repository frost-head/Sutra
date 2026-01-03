use crate::ast::expression::Expression;
use crate::errors::ParserError;
use crate::lexer::token::{PuncuationKind, Token};
use crate::parser::Parser;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub value: Expression,
}

impl ReturnStatement {
    pub fn new(value: Expression) -> Self {
        ReturnStatement { value }
    }

    pub fn parse(parser: &mut Parser) -> Result<ReturnStatement> {
        let expression: Expression;

        expression = Expression::parse(parser)?;
        if *parser.peek()? == Token::Punctuation(PuncuationKind::SemiColon) {
            parser.consume()?;
        } else {
            return Err(ParserError::ExpectedTokenGotUnexpected {
                kind: Token::Punctuation(PuncuationKind::SemiColon),
                got: parser.peek()?.clone(),
            }
            .into());
        }
        let statement = ReturnStatement::new(expression);
        return Ok(statement);
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Return Statement:\n    Value: {}", self.value)
    }
}
