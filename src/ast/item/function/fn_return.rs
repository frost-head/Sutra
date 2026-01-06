use crate::{ast::types::TypeRef, lexer::token::OperatorKind, parser::Parser};
use anyhow::Result;

pub struct FnReturn {
    pub type_ref: TypeRef,
}

impl std::fmt::Display for FnReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_ref)
    }
}

impl FnReturn {
    pub fn parse_fn_return(parser: &mut Parser) -> Result<FnReturn> {
        parser.expect(crate::lexer::token::TokenKind::Operator(
            OperatorKind::Minus,
        ))?;
        parser.expect(crate::lexer::token::TokenKind::Operator(
            OperatorKind::Greater,
        ))?;
        let type_ref = TypeRef::parse_type_ref(parser)?;

        Ok(FnReturn { type_ref })
    }
}
