use anyhow::{Context, Result};

use crate::ast::block::Block;
use crate::ast::item::Item;
use crate::ast::item::function::FuncItem;
use crate::errors::ParserError;
use crate::{
    ast::Ast,
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
            ast: Ast { items: Vec::new() },
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        while let Some(next) = self.tokens.peek() {
            match next.kind {
                TokenKind::EOF => break,
                TokenKind::FUNC => {
                    let func_item =
                        FuncItem::parse(self).context("Error while parsing function")?;
                    self.ast.items.push(Item::Function(func_item));
                }
                // TokenKind::LET => {
                //     let st = Stmt::LetStmt(LetStatement::parse_let_statement(self)?);
                //     if let Item::Function(func_item) = &mut self.ast.items[0] {
                //         func_item.body.statements.push(st);
                //     }
                // }
                // TokenKind::RETURN => {
                //     let st = Stmt::ReturnStmt(ReturnStatement::parse_return_statement(self)?);
                //     if let Item::Function(func_item) = &mut self.ast.items[0] {
                //         func_item.body.statements.push(st);
                //     }
                // }
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        token: next.clone(),
                    }
                    .into());
                }
            }
        }
        Ok(())
    }
}
