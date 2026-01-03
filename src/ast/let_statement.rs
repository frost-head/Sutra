use crate::errors::ParserError;
use crate::lexer::token::{OperatorKind, PuncuationKind};
use crate::{ast::expression::Expression, lexer::token::Token, parser::Parser};
use anyhow::Result;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub(crate) identifier: Token,
    pub(crate) value: Expression,
}

impl LetStatement {
    pub fn new(identifier: Token, value: Expression) -> LetStatement {
        LetStatement { identifier, value }
    }

    pub fn parse(parser: &mut Parser) -> Result<LetStatement> {
        let identifier: Token;
        let expression: Expression;

        if let Token::Ident(_id) = parser.peek()? {
            identifier = parser.consume()?;
        } else {
            return Err(ParserError::ExpectedTokenGotUnexpected {
                kind: Token::Ident("identifier".to_string()),
                got: parser.peek()?.clone(),
            }
            .into());
        }

        parser.expect(Token::Operator(OperatorKind::Equal))?;
        expression = Expression::parse(parser)?;

        parser.expect(Token::Punctuation(PuncuationKind::SemiColon))?;

        let statement = LetStatement::new(identifier, expression);
        Ok(statement)
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Let Statement:\n  Identifier: {}\n  Value: {}",
            self.identifier, self.value
        )
    }
}
