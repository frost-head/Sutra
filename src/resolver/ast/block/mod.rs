use core::fmt;
use crate::{errors::span::Span, resolver::ast::statement::Stmt, utils::indent_multiline};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

impl Block {
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        for stmt in &self.statements {
            write!(f, "{}\n", indent_multiline(&stmt.to_string(), "    "))?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
