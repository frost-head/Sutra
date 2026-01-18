use crate::errors::ParserError;
use crate::errors::span::Span;
use crate::lexer::token::{KeywordKind, OperatorKind, PuncuationKind, TokenKind};
use crate::resolver::ast::expression::Expression;
use crate::resolver::ast::types::TypeRef;
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
