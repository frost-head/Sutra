use super::Statement;
use crate::{ast::expression::Expresion, lexer::token::Token};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct LetStatement {
    pub(crate) identifier: Token,
    pub(crate) value: Expresion,
}

impl Statement for LetStatement {}

impl LetStatement {
    pub fn new(identifier: Token, value: Expresion) -> LetStatement {
        LetStatement {
            identifier: identifier,
            value: value,
        }
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Let Statement:\n  Identifier: {:#?}\n  Value: {:#?}",
            self.identifier, self.value
        )
    }
}
