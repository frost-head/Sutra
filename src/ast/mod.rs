use std::fmt::Display;
pub mod expression;
pub mod let_statement;

pub struct Ast {
    pub statements: Vec<Box<dyn Statement>>,
}

pub trait Statement: Display {
    fn statement(&self) {}
}
