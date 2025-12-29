use crate::lexer::token::Token;
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
    #[error("Unexpected token: {}", token)]
    UnexpectedToken { token: Token },

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Expected token: {}, Got : {}", kind, got)]
    ExpectedTokenGotUnexpected { kind: Token, got: Token },
}
