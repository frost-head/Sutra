use crate::ast::types::TypeRef;
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
    pub type_ref: TypeRef,
}

impl LetStatement {
    pub fn new(
        identifier: String,
        value: Expression,
        span: Span,
        type_ref: TypeRef,
    ) -> LetStatement {
        LetStatement {
            identifier,
            value,
            span,
            type_ref,
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

        parser.expect(TokenKind::Punctuation(PuncuationKind::Colon))?;

        let type_ref = TypeRef::parse_type_ref(parser)?;

        parser.expect(TokenKind::Operator(OperatorKind::Equal))?;
        expression = Expression::parse(parser)?;

        let semicolon_tok = parser.expect(TokenKind::Punctuation(PuncuationKind::SemiColon))?;

        let statement = LetStatement::new(
            identifier,
            expression,
            Span::merge(first_tok.span, semicolon_tok.span),
            type_ref,
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
