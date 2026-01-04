use std::fmt::{self};

use crate::errors::span::Span;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Operator(OperatorKind),
    Keyword(KeywordKind),
    Punctuation(PuncuationKind),
    Illegal,
    Ident(String),
    Number(i64),
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PuncuationKind {
    LeftParen,
    RightParen,
    LeftCurlyParen,
    RightCurlyParen,
    LeftSquareParen,
    RightSquareParen,
    SemiColon,
    Colon,
    Comma,
}
#[derive(Debug, PartialEq, Clone)]
pub enum OperatorKind {
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Less,
    Greater,
    EqualEqual,
    Bang,
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordKind {
    Let,
    If,
    Else,
    While,
    For,
    Return,
    Func,
    Struct,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token : {}, {}", self.kind, self.span)
    }
}
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Operator(operator_kind) => write!(f, "{}", operator_kind),
            TokenKind::Keyword(keyword_kind) => write!(f, "{}", keyword_kind),
            TokenKind::Punctuation(punctuation_kind) => write!(f, "{}", punctuation_kind),
            TokenKind::Illegal => write!(f, "Illegal token"),
            TokenKind::Ident(ident) => write!(f, "Ident('{}')", ident),
            TokenKind::Number(num) => write!(f, "Num({})", num),
            TokenKind::EOF => write!(f, "End Of File"),
        }
    }
}

impl fmt::Display for OperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperatorKind::Equal => write!(f, "="),
            OperatorKind::Plus => write!(f, "+"),
            OperatorKind::Minus => write!(f, "-"),
            OperatorKind::Star => write!(f, "*"),
            OperatorKind::Slash => write!(f, "/"),
            OperatorKind::Percent => write!(f, "%"),
            OperatorKind::Less => write!(f, "<"),
            OperatorKind::Greater => write!(f, ">"),
            OperatorKind::EqualEqual => write!(f, "=="),
            OperatorKind::Bang => write!(f, "!"),
        }
    }
}

impl fmt::Display for KeywordKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeywordKind::Let => write!(f, "Let"),
            KeywordKind::If => write!(f, "If"),
            KeywordKind::Else => write!(f, "Else"),
            KeywordKind::While => write!(f, "While"),
            KeywordKind::For => write!(f, "For"),
            KeywordKind::Return => write!(f, "Return"),
            KeywordKind::Func => write!(f, "Function"),
            KeywordKind::Struct => write!(f, "Struct"),
        }
    }
}

impl fmt::Display for PuncuationKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PuncuationKind::LeftParen => write!(f, "Punctuation : ("),
            PuncuationKind::RightParen => write!(f, "Punctuation : )"),
            PuncuationKind::LeftCurlyParen => write!(f, "Punctuation : {{"),
            PuncuationKind::RightCurlyParen => write!(f, "Punctuation : }}"),
            PuncuationKind::LeftSquareParen => write!(f, "Punctuation : ["),
            PuncuationKind::RightSquareParen => write!(f, "Punctuation : ]"),
            PuncuationKind::SemiColon => write!(f, "Punctuation : ;"),
            PuncuationKind::Colon => write!(f, "Punctuation : :"),
            PuncuationKind::Comma => write!(f, "Punctuation : ,"),
        }
    }
}
