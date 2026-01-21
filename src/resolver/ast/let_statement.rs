use crate::resolver::ast::expression::Expression;
use crate::resolver::symbol::SymbolId;
use crate::{errors::span::Span, resolver::ast::types::TypeRes};
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub(crate) identifier: SymbolId,
    pub(crate) value: Expression,
    pub(crate) span: Span,
    pub type_res: TypeRes,
}

impl LetStatement {
    pub fn new(
        identifier: SymbolId,
        value: Expression,
        span: Span,
        type_res: TypeRes,
    ) -> LetStatement {
        LetStatement {
            identifier,
            value,
            span,
            type_res,
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
