use crate::{errors::span::Span, resolver::ast::types::TypeRef};

#[derive(Debug, Clone)]
pub struct FnReturn {
    pub type_ref: TypeRef,
    pub span: Span,
}

impl std::fmt::Display for FnReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_ref)
    }
}
