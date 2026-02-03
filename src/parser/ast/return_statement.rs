use crate::errors::span::Span;
use crate::lexer::token::{KeywordKind, PuncuationKind, TokenKind};
use crate::parser::Parser;
use crate::parser::ast::expression::Expression;
use anyhow::Result;
use core::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub value: Expression,
    pub span: Span,
}

impl ReturnStatement {
    pub fn new(value: Expression, span: Span) -> Self {
        ReturnStatement { value, span }
    }

    pub fn parse(parser: &mut Parser) -> Result<ReturnStatement> {
        let expression: Expression;
        let first_tok = parser.expect(TokenKind::Keyword(KeywordKind::Return))?;
        expression = Expression::parse(parser)?;
        let semicolon_tok = parser.expect(TokenKind::Punctuation(PuncuationKind::SemiColon))?;
        let statement =
            ReturnStatement::new(expression, Span::merge(first_tok.span, semicolon_tok.span));
        return Ok(statement);
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Return Statement:\n    Value: {}", self.value)
    }
}
