use core::fmt;

use anyhow::Result;

use crate::{
    ast::block::Block,
    errors::ParserError,
    lexer::token::{KeywordKind, PuncuationKind, Token},
    parser::Parser,
    utils::indent_multiline,
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
        let next = parser.tokens.peek().unwrap();
        match next {
            Token::Keyword(KeywordKind::Func) => parser.tokens.next().unwrap(),
            _ => return Err(ParserError::UnexpectedEndOfInput.into()),
        };
        let next = parser.tokens.peek().unwrap();
        match next {
            Token::Ident(id) => {
                name = id.clone();
                parser.tokens.next().unwrap();
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: next.clone(),
                }
                .into());
            }
        }
        let next = parser.tokens.peek().unwrap();
        match next {
            Token::Punctuation(PuncuationKind::LeftParen) => parser.tokens.next().unwrap(),
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: next.clone(),
                }
                .into());
            }
        };
        let next = parser.tokens.peek().unwrap();
        match next {
            Token::Punctuation(PuncuationKind::RightParen) => parser.tokens.next().unwrap(),
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: next.clone(),
                }
                .into());
            }
        };
        let next = parser.tokens.peek().unwrap();
        match next {
            Token::Punctuation(PuncuationKind::LeftCurlyParen) => parser.tokens.next().unwrap(),
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: next.clone(),
                }
                .into());
            }
        };

        let params: Option<Vec<Token>> = None;

        let body = Block::parse(parser).expect("Error while parsing function body");

        let next = parser.tokens.peek().unwrap();
        match next {
            Token::Punctuation(PuncuationKind::RightCurlyParen) => parser.tokens.next().unwrap(),
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: next.clone(),
                }
                .into());
            }
        };

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
