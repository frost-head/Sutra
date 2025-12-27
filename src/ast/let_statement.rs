use crate::errors::LexerError;
use crate::{
    ast::expression::Expresion,
    lexer::token::{Token, TokenKind},
    parser::Parser,
};
use anyhow::Result;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct LetStatement {
    pub(crate) identifier: Token,
    pub(crate) value: Expresion,
}


impl LetStatement {
    pub fn new(identifier: Token, value: Expresion) -> LetStatement {
        LetStatement {
            identifier: identifier,
            value: value,
        }
    }

    pub fn parse_let_statement(parser: &mut Parser) -> Result<LetStatement> {
        let identifier: Token;
        let mut expression: Vec<Token> = Vec::new();
        let peek: &Token = parser.tokens.peek().unwrap();
        if peek.kind == TokenKind::LET {
            parser.tokens.next();
        } else {
            return Err(LexerError::ExpectedTokenGotUnexpected {
                kind: TokenKind::LET,
                token: peek.clone(),
            }
            .into());
        }

        let peek: &Token = parser.tokens.peek().unwrap();
        if peek.kind == TokenKind::IDENT {
            identifier = parser.tokens.next().unwrap();
        } else {
            return Err(LexerError::ExpectedTokenGotUnexpected {
                kind: TokenKind::IDENT,
                token: peek.clone(),
            }
            .into());
        }

        let peek: &Token = parser.tokens.peek().unwrap();
        if peek.kind == TokenKind::Equal {
            parser.tokens.next();
        } else {
            return Err(LexerError::ExpectedTokenGotUnexpected {
                kind: TokenKind::Equal,
                token: peek.clone(),
            }
            .into());
        }

        loop {
            let cur: Token = parser.tokens.next().unwrap();
            if cur.kind == TokenKind::SemiColon {
                break;
            } else if cur.kind == TokenKind::EOF {
                return Err(LexerError::UnexpectedToken { token: cur.clone() }.into());
            } else {
                expression.push(cur);
            }
        }

        let statement = LetStatement::new(identifier, Expresion::new(expression));
        return Ok(statement);
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
