use core::fmt;

use anyhow::{Context, Result};

use crate::{
    ast::block::Block,
    errors::ParserError,
    lexer::token::{KeywordKind, PuncuationKind, Token, TokenKind},
    parser::Parser,
};

pub struct FuncItem {
    pub name: String,
    pub params: Option<Vec<Token>>,
    pub body: Block,
}

impl FuncItem {
    pub fn new(name: String, params: Option<Vec<Token>>, body: Block) -> Self {
        FuncItem { name, params, body }
    }

    pub fn parse(parser: &mut Parser) -> Result<FuncItem> {
        let name: String;
        let next = parser.peek()?;
        match next.kind {
            TokenKind::Keyword(KeywordKind::Func) => parser.consume()?,
            _ => return Err(ParserError::UnexpectedEndOfInput.into()),
        };
        let next = parser.peek()?;
        match &next.kind {
            TokenKind::Ident(id) => {
                name = id.clone();
                parser.consume()?;
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: next.clone(),
                }
                .into());
            }
        }
        let next = parser.peek()?;
        match next.kind {
            TokenKind::Punctuation(PuncuationKind::LeftParen) => parser.consume()?,
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: next.clone(),
                }
                .into());
            }
        };
        let next = parser.peek()?;
        match next.kind {
            TokenKind::Punctuation(PuncuationKind::RightParen) => parser.consume()?,
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: next.clone(),
                }
                .into());
            }
        };
        let next = parser.peek()?;
        match next.kind {
            TokenKind::Punctuation(PuncuationKind::LeftCurlyParen) => {}
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: next.clone(),
                }
                .into());
            }
        };

        let params: Option<Vec<Token>> = None;

        let body = Block::parse(parser).context("Error while parsing function body")?;

        Ok(FuncItem { name, params, body })
    }
}

impl fmt::Display for FuncItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}(", self.name)?;
        if let Some(params) = &self.params {
            for param in params {
                write!(f, "{}", param)?;
            }
        }
        write!(f, ") \n{}\n", self.body)?;
        Ok(())
    }
}
