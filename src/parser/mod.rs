use anyhow::{Context, Result};

use crate::ast::item::Item;
use crate::ast::item::function::FuncItem;
use crate::errors::ParserError;
use crate::lexer::token::KeywordKind;
use crate::{
    ast::Ast,
    lexer::{Lexer, token::Token},
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
            match next {
                Token::EOF => break,
                Token::Keyword(KeywordKind::Func) => {
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

    pub fn peek(&mut self) -> Result<&Token> {
        return Ok(self
            .tokens
            .peek()
            .context("could not peek at the next token")?);
    }

    pub fn consume(&mut self) -> Result<Token> {
        return self
            .tokens
            .next()
            .context("Could not consume the next token");
    }

    pub fn expect(&mut self, expected: Token) -> Result<()> {
        let tok = self.consume()?;
        if tok == expected {
            Ok(())
        } else {
            Err(ParserError::ExpectedTokenGotUnexpected {
                kind: expected,
                got: tok,
            }
            .into())
        }
    }
}
