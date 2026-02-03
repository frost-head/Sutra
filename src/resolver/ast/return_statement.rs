use crate::errors::span::Span;
use crate::resolver::ast::expression::Expression;
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
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Return Statement:\n    Value: {}", self.value)
    }
}
