use crate::{
    ast::{
        Ast, expression::Expresion, let_statement::LetStatement, return_statement::ReturnStatement,
    },
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
};
use std::iter::Peekable;

pub struct Parser<'a> {
    pub tokens: Peekable<Lexer<'a>>,
    pub ast: Ast,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            tokens: lexer.peekable(),
            ast: Ast {
                statements: Vec::new(),
            },
        }
    }

    pub fn parse(&mut self) {
        loop {
            let next = self.tokens.peek();
            let next = match next {
                Some(n) => n,
                None => panic!("Unkown Token"),
            };
            match next.kind {
                TokenKind::EOF => break,
                TokenKind::LET => {
                    let st = LetStatement::parse_let_statement(self);
                    let st = match st {
                        Some(s) => s,
                        None => panic!("Unknown Error"),
                    };
                    self.ast.statements.push(Box::new(st));
                }
                TokenKind::RETURN => {
                    let st = ReturnStatement::parse_return_statement(self);
                    let st = match st {
                        Some(s) => s,
                        None => panic!("Unknown Error"),
                    };
                    self.ast.statements.push(Box::new(st));
                }
                _ => {
                    panic!("Wrong Token")
                }
            }
        }
    }
}
