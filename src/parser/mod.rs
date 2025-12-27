use anyhow::Result;

use crate::ast::block::{Block, Stmt};
use crate::ast::item::{FuncItem, Item};
use crate::errors::LexerError;
use crate::{
    ast::{Ast, let_statement::LetStatement, return_statement::ReturnStatement},
    lexer::{Lexer, token::TokenKind},
};
use std::iter::Peekable;
use std::vec;

pub struct Parser<'a> {
    pub tokens: Peekable<Lexer<'a>>,
    pub ast: Ast,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            tokens: lexer.peekable(),
            ast: Ast {
                items: vec![Item::Function(FuncItem {
                    name: String::from("main"),
                    params: None,
                    body: Block {
                        statements: Vec::new(),
                    },
                })],
            },
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        while let Some(next) = self.tokens.peek() {
            match next.kind {
                TokenKind::EOF => break,
                TokenKind::LET => {
                    let st = Stmt::LetStmt(LetStatement::parse_let_statement(self)?);
                    if let Item::Function(func_item) = &mut self.ast.items[0] {
                        func_item.body.statements.push(st);
                    }
                }
                TokenKind::RETURN => {
                    let st = Stmt::ReturnStmt(ReturnStatement::parse_return_statement(self)?);
                    if let Item::Function(func_item) = &mut self.ast.items[0] {
                        func_item.body.statements.push(st);
                    }
                }
                _ => {
                    return Err(LexerError::UnexpectedToken {
                        token: next.clone(),
                    }
                    .into());
                }
            }
        }
        Ok(())
    }
}
