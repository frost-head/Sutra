use crate::{errors::span::Span, resolver::ast::types::TypeRes};

#[derive(Debug, Clone)]
pub struct FnReturn {
    pub type_res: TypeRes,
    pub span: Span,
}

impl std::fmt::Display for FnReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_res)
    }
}
