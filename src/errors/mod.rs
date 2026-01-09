use crate::{
    ast::types::TypeRef,
    lexer::token::{Token, TokenKind},
};
use thiserror::Error;

pub mod span;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token: {},\nError: {}", token.kind, token.span)]
    UnexpectedToken { token: Token },

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Expected token: {}, Got : {},\nError : {}", kind, got.kind, got.span)]
    ExpectedTokenGotUnexpected { kind: TokenKind, got: Token },
}

#[derive(Error, Debug)]
pub enum TypeRefError {
    #[error("Invalid type reference, \nError: {:?}", type_ref.print_span())]
    InvalidTypeReference { type_ref: TypeRef },
}

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("Symbol not found: {}", symbol)]
    SymbolNotFound { symbol: String },

    #[error("Symbol already declared: {}", symbol)]
    SymbolAlreadyDeclared { symbol: String },
}
