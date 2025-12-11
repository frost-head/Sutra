use crate::lexer::token::Token;
use std::fmt::{self, Display};

pub struct Ast {
    pub statements: Vec<Box<dyn Statement>> ,
}

pub trait Statement : Display {
    fn statement(&self){}
}

#[derive(Debug)]
pub struct Expresion {
    exp: Vec<Token>,
}

impl Expresion {
    pub fn new(tokens: Vec<Token>) -> Expresion {
        Expresion { exp: tokens }
    }

    fn eval(&self) -> Token {
        todo!("Implement the method");
    }
}

#[derive(Debug)]
pub struct LetStatement {
    identifier: Token,
    value: Expresion,
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
