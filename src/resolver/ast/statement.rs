use core::fmt;

use crate::parser::ast::{
    expression::Expression, let_statement::LetStatement, return_statement::ReturnStatement,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    LetStmt(LetStatement),
    ReturnStmt(ReturnStatement),
    Expr(Expression),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::LetStmt(stmt) => write!(f, "{} \n", stmt),
            Stmt::ReturnStmt(stmt) => write!(f, "{} \n", stmt),
            Stmt::Expr(expression) => write!(f, "{} \n", expression),
        }
    }
}
