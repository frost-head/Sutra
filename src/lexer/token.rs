use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// Represents a single token in the input stream.
    /// The left parenthesis '(' token.
    LeftParen,
    /// The right parenthesis ')' token.
    RightParen,
    /// The lefe curly brace '{' token.
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
    /// The equal '=' token.
    Equal,
    /// The plus '+' token.
    Plus,
    /// The minus '-' token.
    Minus,
    /// The star '*' token.
    Star,
    /// The slash '/' token.
    Slash,
    /// The percent '%' token.
    Percent,
    /// Illegal token.
    Illegal,
    /// Integer token.
    INT,
    /// Identifier token.
    IDENT,
    /// End of file (EOF) token.
    EOF,

    LET,
    IF,
    ELSE,
    WHILE,
    FOR,
    RETURN,
    FUNC,
    STRUCT,
}

impl fmt::Display for TokenKind{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftCurlyParen => write!(f, "{{"),
            TokenKind::RightCurlyParen => write!(f, "}}"),
            TokenKind::LeftSquareParen => write!(f, "["),
            TokenKind::RightSquareParen => write!(f, "]"),
            TokenKind::LeftAngleParen => write!(f, "<"),
            TokenKind::RightAngleParen => write!(f, ">"),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Equal => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Illegal => write!(f, "ILLEGAL"),
            TokenKind::INT => write!(f, "INT"),
            TokenKind::IDENT => write!(f, "IDENT"),
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::LET => write!(f, "LET"),
            TokenKind::IF => write!(f, "IF"),
            TokenKind::ELSE => write!(f, "ELSE"),
            TokenKind::WHILE => write!(f, "WHILE"),
            TokenKind::FOR => write!(f, "FOR"),
            TokenKind::RETURN => write!(f, "RETURN"),
            TokenKind::FUNC => write!(f, "FUNC"),
            TokenKind::STRUCT => write!(f, "STRUCT"),
        }
    }  
}

/// Represents a single token in the input stream.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Token {
        Token { kind, literal }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            TokenKind::RightAngleParen => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::LeftAngleParen => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::Illegal => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::INT => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::IDENT => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::EOF => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::LET => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::IF => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::ELSE => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::WHILE => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::FOR => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::RETURN => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::FUNC => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::STRUCT => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::LeftParen => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::RightParen => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::LeftCurlyParen => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::RightCurlyParen => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::LeftSquareParen => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::RightSquareParen => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::SemiColon => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::Colon => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::Comma => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::Equal => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::Plus => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::Minus => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::Star => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::Slash => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
            TokenKind::Percent => write!(f, "Token {{ kind: {}, literal: \"{}\" }}", self.kind, self.literal),
        }
    }
}