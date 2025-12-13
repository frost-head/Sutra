use crate::lexer::token::Token;

#[derive(Debug)]
pub struct Expresion {
    pub(crate) exp: Vec<Token>,
}

impl Expresion {
    pub fn new(tokens: Vec<Token>) -> Expresion {
        Expresion { exp: tokens }
    }

    pub(crate) fn eval(&self) -> Token {
        todo!("Implement the method");
    }
}
