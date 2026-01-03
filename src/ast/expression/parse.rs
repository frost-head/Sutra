use crate::{
    ast::expression::Expression,
    errors::ParserError,
    lexer::token::{PuncuationKind, Token},
    parser::Parser,
};
use anyhow::{Ok, Result};

impl Expression {
    pub fn parse(parser: &mut Parser) -> Result<Expression> {
        let new = parser.peek()?;
        println!("{}", new);
        match new {
            // Token::Operator(operator_kind) => todo!(),
            // Token::Punctuation(puncuation_kind) => todo!(),
            // Token::Ident(_) => todo!(),
            Token::Number(num) => {
                let val = num.clone();
                parser.consume()?;
                let peek = parser.peek()?;
                println!("{}", peek);
                match peek {
                    Token::Punctuation(punctuation_kind) => match punctuation_kind {
                        PuncuationKind::SemiColon => {
                            parser.consume()?;
                            return Ok(Expression::Literal { literal: val });
                        }
                        _ => {
                            return Err(ParserError::UnexpectedToken {
                                token: peek.clone(),
                            }
                            .into());
                        }
                    },
                    _ => {
                        return Err(ParserError::UnexpectedToken {
                            token: peek.clone(),
                        }
                        .into());
                    }
                }
            }
            _ => {
                return Err(ParserError::UnexpectedToken { token: new.clone() }.into());
            }
        }
    }
}
