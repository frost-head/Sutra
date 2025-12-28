use core::fmt;

use anyhow::Result;

use crate::{
    ast::block::Block,
    errors::ParserError,
    lexer::token::{Token, TokenKind},
    parser::Parser, utils::indent_multiline,
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
        let next = parser.tokens.next().unwrap();
        if next.kind != TokenKind::FUNC {
            return Err(ParserError::UnexpectedEndOfInput.into());
        }
        let name = parser.tokens.next().unwrap();
        if name.kind != TokenKind::IDENT {
            return Err(ParserError::UnexpectedToken {
                token: name.clone(),
            }
            .into());
        }
        let name = name.literal.clone();

        let next = parser.tokens.next().unwrap();
        if next.kind != TokenKind::LeftParen {
            return Err(ParserError::UnexpectedToken {
                token: next.clone(),
            }
            .into());
        }

        let next = parser.tokens.next().unwrap();
        if next.kind != TokenKind::RightParen {
            return Err(ParserError::UnexpectedToken {
                token: next.clone(),
            }
            .into());
        }

        let params: Option<Vec<Token>> = None;

        let next = parser.tokens.next().unwrap();
        if next.kind != TokenKind::LeftCurlyParen {
            return Err(ParserError::UnexpectedToken {
                token: next.clone(),
            }
            .into());
        }

        let body = Block::parse(parser).expect("Error while parsing function body");
        println!("Function body parsed successfully");

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
        write!(
            f,
            ") {{\n{}\n}}",
            indent_multiline(&self.body.to_string(), "    ")
        )?;
        Ok(())
    }
}
