use crate::lexer::token::{Token, TokenKind};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token: {}", token.literal)]
    UnexpectedToken { token: Token },

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Expected token: {}, Got : {}", token.kind, token.literal)]
    ExpectedTokenGotUnexpected { kind: TokenKind, token: Token },
}
