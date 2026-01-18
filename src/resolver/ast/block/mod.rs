use anyhow::Result;
use core::fmt;

use crate::{
    errors::span::Span,
    lexer::token::{KeywordKind, OperatorKind, PuncuationKind, TokenKind},
    resolver::ast::{
        expression::{Expression, ExpressionKind, assign::Identifier},
        let_statement::LetStatement,
        return_statement::ReturnStatement,
        statement::Stmt,
    },
    utils::indent_multiline,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

impl Block {
    fn new(statements: Vec<Stmt>, span: Span) -> Self {
        Block { statements, span }
    }
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
