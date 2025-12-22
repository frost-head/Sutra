use super::Statement;
use super::expression::Expresion;
use core::fmt;
use std::fmt::Display;

pub struct ReturnStatement {
    pub value: Expresion,
}

impl Statement for ReturnStatement {}

impl ReturnStatement {
    pub fn new(value: Expresion) -> Self {
        ReturnStatement { value }
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Return Statement:\n    Value: {:#?}", self.value)
    }
}
