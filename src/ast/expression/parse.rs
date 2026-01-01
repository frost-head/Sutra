use crate::{ast::expression::Expression, errors::ParserError, parser::Parser};
use anyhow::Result;

impl Expression {
    pub fn parse(parser: &mut Parser) -> Result<Expression> {
        let new = parser.peek()?;
        println!("{}", new);
        match new {
            // Token::Operator(operator_kind) => todo!(),
            // Token::Punctuation(puncuation_kind) => todo!(),
            // Token::Ident(_) => todo!(),
            // Token::Number(_) => todo!(),
            _ => return Err(ParserError::UnexpectedToken { token: new.clone() }.into()),
        }
    }
}
