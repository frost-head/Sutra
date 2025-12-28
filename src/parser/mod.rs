use anyhow::{Context, Result};

use crate::ast::item::Item;
use crate::ast::item::function::FuncItem;
use crate::errors::ParserError;
use crate::{
    ast::Ast,
    lexer::{Lexer, token::TokenKind},
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
