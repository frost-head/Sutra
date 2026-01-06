use anyhow::{Context, Result};
use core::fmt;

use crate::{
    ast::{
        block::Block,
        item::function::{fn_params::Param, fn_return::FnReturn},
    },
    errors::ParserError,
    lexer::token::{KeywordKind, OperatorKind, PuncuationKind, TokenKind},
    parser::Parser,
};

pub mod fn_params;
pub mod fn_return;

pub struct FuncItem {
    pub name: String,
    pub params: Option<Vec<Param>>,
    pub return_type: Option<FnReturn>,
    pub body: Block,
}

impl FuncItem {
    pub fn new(
        name: String,
        params: Option<Vec<Param>>,
        return_type: Option<FnReturn>,
        body: Block,
    ) -> Self {
        FuncItem {
            name,
            params,
            return_type,
            body,
        }
    }

    pub fn parse(parser: &mut Parser) -> Result<FuncItem> {
        let name: String;

        parser.expect(TokenKind::Keyword(KeywordKind::Func))?;

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
        let params = Param::parse_params(parser)?;

        let mut return_type: Option<FnReturn> = None;

        let next = parser.peek()?;
        match next.kind {
            TokenKind::Operator(OperatorKind::Minus) => {
                return_type = Some(FnReturn::parse_fn_return(parser)?);
            }
            TokenKind::Punctuation(PuncuationKind::LeftCurlyParen) => {}
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

        let body = Block::parse(parser).context("Error while parsing function body")?;

        if params.len() == 0 {
            return Ok(FuncItem {
                name,
                body,
                params: None,
                return_type,
            });
        }

        Ok(FuncItem {
            name,
            params: Some(params),
            body,
            return_type,
        })
    }
}

impl fmt::Display for FuncItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}(", self.name)?;
        if let Some(params) = &self.params {
            for (i, param) in params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", param)?;
            }
        }
        if let Some(return_type) = &self.return_type {
            write!(f, ") -> {} \n{}\n", return_type, self.body)?;
        } else {
            write!(f, ") \n{}\n", self.body)?;
        }
        Ok(())
    }
}
