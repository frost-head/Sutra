use crate::lexer::token::Token;
use core::fmt;

#[derive(Debug)]
pub struct Expresion {
    pub(crate) exp: Vec<Token>,
}

impl Expresion {
    pub fn new(tokens: Vec<Token>) -> Expresion {
        Expresion { exp: tokens }
    }
}

impl fmt::Display for Expresion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in &self.exp {
            write!(f, "{} ", token)?;
        }
        Ok(())
    }
}
