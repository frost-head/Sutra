use crate::{
    errors::{ParserError, span::Span},
    lexer::token::{KeywordKind, TokenKind},
    parser::Parser,
};
use anyhow::Result;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeRef {
    Named {
        name: String,
        span: Span,
    },
}

impl Display for TypeRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TypeRef::Named { name, .. } => write!(f, "{}", name),
        }
    }
}

impl TypeRef {
    pub fn parse_type_ref(parser: &mut Parser) -> Result<TypeRef> {
        let tok = parser.peek()?;
        let type_ref = match tok.kind {
            TokenKind::Keyword(KeywordKind::Int) => {
                let tok = tok.clone();
                parser.consume()?;
                TypeRef::Named {
                    name: "int".to_string(),
                    span: tok.span,
                }
            }
            TokenKind::Keyword(KeywordKind::Bool) => {
                let tok = tok.clone();
                parser.consume()?;
                TypeRef::Named {
                    name: "bool".to_string(),
                    span: tok.span,
                }
            }
            _ => {
                return Err(ParserError::UnexpectedToken { token: tok.clone() }.into());
            }
        };
        Ok(type_ref)
    }

    pub fn print_span(&self) {
        match self {
            TypeRef::Named { name, span } => {
                println!("Named Type: {} at {}", name, span);
            }
            
        }
    }
}
