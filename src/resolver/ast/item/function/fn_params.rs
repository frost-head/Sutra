use crate::{
    errors::span::Span,
    parser::ast::types::TypeRef,
    resolver::symbol::{SymbolId},
};
use core::fmt;

#[derive(Debug, Clone)]
pub struct Param {
    pub id: SymbolId,
    pub type_ref: TypeRef,
    pub span: Span,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.type_ref)
    }
}
