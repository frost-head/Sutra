use crate::errors::ParserError;
use crate::lexer::token::{KeywordKind, OperatorKind, PuncuationKind};
use crate::{ast::expression::Expression, lexer::token::Token, parser::Parser};
use anyhow::Result;
use std::fmt::{self, Display};

#[derive(Debug)]
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
        if parser.tokens.peek().unwrap() == &Token::Keyword(KeywordKind::Let) {
            parser.tokens.next().unwrap();
        } else {
            return Err(ParserError::ExpectedTokenGotUnexpected {
                got: parser.tokens.peek().unwrap().clone(),
                kind: Token::Keyword(KeywordKind::Let),
            }
            .into());
        }

        if let Token::Ident(_id) = parser.tokens.peek().unwrap() {
            identifier = parser.tokens.next().unwrap();
        } else {
            return Err(ParserError::ExpectedTokenGotUnexpected {
                kind: Token::Ident("identifier".to_string()),
                got: parser.tokens.peek().unwrap().clone(),
            }
            .into());
        }

        if parser.tokens.peek().unwrap() == &Token::Operator(OperatorKind::Equal) {
            parser.tokens.next().unwrap();
        } else {
            return Err(ParserError::ExpectedTokenGotUnexpected {
                kind: Token::Operator(OperatorKind::Equal),
                got: parser.tokens.peek().unwrap().clone(),
            }
            .into());
        }

        expression = Expression::parse(parser).unwrap();
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
