use super::Statement;
use crate::{
    ast::expression::Expresion,
    lexer::token::{Token, TokenKind},
    parser::Parser,
};
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

    pub fn parse_let_statement(parser: &mut Parser) -> Option<LetStatement> {
        let identifier: Token;
        let mut exepresion: Vec<Token> = Vec::new();
        let peek: &Token = parser.tokens.peek()?;
        if peek.kind == TokenKind::LET {
            parser.tokens.next();
        } else {
            panic!("wrong token");
        }

        let peek: &Token = parser.tokens.peek()?;
        if peek.kind == TokenKind::IDENT {
            identifier = parser.tokens.next()?;
        } else {
            panic!("wrong token");
        }

        let peek: &Token = parser.tokens.peek()?;
        if peek.kind == TokenKind::Equal {
            parser.tokens.next();
        } else {
            panic!("wrong token");
        }

        loop {
            let cur: Token = parser.tokens.next()?;
            if cur.kind == TokenKind::SemiColon {
                break;
            } else if cur.kind == TokenKind::EOF {
                panic!("wrong token");
            } else {
                exepresion.push(cur);
            }
        }

        let statement = LetStatement::new(identifier, Expresion::new(exepresion));
        return Some(statement);
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
