use crate::errors::ParserError;
use crate::errors::span::Span;
use crate::lexer::token::{KeywordKind, OperatorKind, PuncuationKind, TokenKind};
use crate::{ast::expression::Expression, parser::Parser};
use anyhow::Result;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub(crate) identifier: String,
    pub(crate) value: Expression,
    pub(crate) span: Span,
}

impl LetStatement {
    pub fn new(identifier: String, value: Expression, span: Span) -> LetStatement {
        LetStatement {
            identifier,
            value,
            span,
        }
    }

    pub fn parse(parser: &mut Parser) -> Result<LetStatement> {
        let identifier: String;
        let expression: Expression;

        let first_tok = parser.expect(TokenKind::Keyword(KeywordKind::Let))?;
        if let TokenKind::Ident(id) = &parser.consume()?.kind {
            identifier = id.clone();
        } else {
            return Err(ParserError::ExpectedTokenGotUnexpected {
                kind: TokenKind::Ident("identifier".to_string()),
                got: parser.peek()?.clone(),
            }
            .into());
        }

        parser.expect(TokenKind::Operator(OperatorKind::Equal))?;
        expression = Expression::parse(parser)?;

        let semicolon_tok = parser.expect(TokenKind::Punctuation(PuncuationKind::SemiColon))?;

        let statement = LetStatement::new(
            identifier,
            expression,
            Span::merge(first_tok.span, semicolon_tok.span),
        );
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
