use crate::lexer::token::Token;

pub struct Root {}

impl Root {
    pub fn new() -> Root {
        Root {}
    }
}

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

pub struct LetStatement {
    statement: Vec<Token>,
    indetifier: Token,
    value: Expresion,
}

impl LetStatement {
    pub fn new() -> LetStatement {
        todo!("Implement method");
    }
}
