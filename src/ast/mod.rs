use crate::lexer::token::Token;

pub struct Ast {
    pub statements: Vec<dyn Statement>,
}

pub trait Statement {
    fn statement()
    where
        Self: Sized,
    {
    }
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
