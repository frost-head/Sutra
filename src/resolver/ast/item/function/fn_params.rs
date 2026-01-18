use crate::{
    parser::ast::types::TypeRef,
    errors::{ParserError, TypeRefError, span::Span},
    lexer::token::{PuncuationKind, TokenKind},
};
use anyhow::Result;
use core::fmt;

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_ref: TypeRef,
    pub span: Span,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.type_ref)
    }
}
