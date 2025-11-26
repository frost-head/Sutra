#[derive(Debug, PartialEq)]
pub enum TokenKind {
    /// Represents a single token in the input stream.
    /// The left parenthesis '(' token.
    LeftParen,
    /// The right parenthesis ')' token.
    RightParen,
    /// The left curly brace '{' token.
    LeftCurlyParen,
    /// The right curly brace '}' token.
    RightCurlyParen,
    /// The left square bracket '[' token.
    LeftSquareParen,
    /// The right square bracket ']' token.
    RightSquareParen,
    /// The angle bracket '<' token.
    LeftAngleParen,
    /// The angle bracket '>' token.
    RightAngleParen,
    /// The semicolon ';' token.
    SemiColon,
    /// The colon ':' token.
    Colon,
    /// The comma ',' token.
    Comma,
    /// End of file (EOF) token.
    EOF,
}

/// Represents a single token in the input stream.
#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Token {
        Token { kind, literal }
    }
}
