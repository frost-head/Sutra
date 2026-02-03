use crate::{
    errors::span::Span,
    resolver::{ast::types::TypeRes, symbol::SymbolId},
};
use core::fmt;

#[derive(Debug, Clone)]
pub struct Param {
    pub id: SymbolId,
    pub type_res: TypeRes,
    pub span: Span,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.type_res)
    }
}
