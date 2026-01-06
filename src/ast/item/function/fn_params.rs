use crate::{
    ast::types::TypeRef,
    errors::ParserError,
    lexer::token::{PuncuationKind, TokenKind},
    parser::Parser,
};
use anyhow::Result;
use core::fmt;

pub struct Param {
    pub name: String,
    pub type_ref: TypeRef,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.type_ref)
    }
}
impl Param {
    pub fn parse_params(parser: &mut Parser) -> Result<Vec<Param>> {
        let mut params: Vec<Param> = Vec::new();
        parser.expect(TokenKind::Punctuation(PuncuationKind::LeftParen))?;
        while TokenKind::Punctuation(PuncuationKind::RightParen) != parser.peek()?.kind {
            let name: String;
            let tok = parser.peek()?;
            match &tok.kind {
                TokenKind::Ident(id) => {
                    name = id.clone();
                    parser.consume()?;
                }
                _ => {
                    return Err(ParserError::ExpectedTokenGotUnexpected {
                        kind: TokenKind::Ident("Ident".to_string()),
                        got: tok.clone(),
                    }
                    .into());
                }
            }

            parser.expect(TokenKind::Punctuation(PuncuationKind::Colon))?;

            let type_ref = TypeRef::parse_type_ref(parser)?;

            if parser.peek()?.kind != TokenKind::Punctuation(PuncuationKind::RightParen) {
                parser.expect(TokenKind::Punctuation(PuncuationKind::Comma))?;
            }
            params.push(Param { name, type_ref });
        }
        parser.expect(TokenKind::Punctuation(PuncuationKind::RightParen))?;
        Ok(params)
    }
}
