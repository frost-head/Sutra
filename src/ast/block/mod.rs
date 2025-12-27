use crate::ast::{let_statement::LetStatement, return_statement::ReturnStatement};

pub struct Block {
    pub statements: Vec<Stmt>,
}

pub enum Stmt {
    LetStmt(LetStatement),
    ReturnStmt(ReturnStatement),
}
