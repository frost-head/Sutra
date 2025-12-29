use std::fmt::{self};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
    LeftAngleParen,
    RightAngleParen,
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
        match self {
            Token::Operator(operator_kind) => write!(f, "{}", operator_kind),
            Token::Keyword(keyword_kind) => write!(f, "{}", keyword_kind),
            Token::Punctuation(punctuation_kind) => write!(f, "{}", punctuation_kind),
            Token::Illegal => write!(f, "Illegal token"),
            Token::Ident(ident) => write!(f, "Ident('{}')", ident),
            Token::Number(num) => write!(f, "Num({})", num),
            Token::EOF => write!(f, "End Of File"),
        }
    }
}

impl fmt::Display for OperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperatorKind::Equal => write!(f, "Equal(=)"),
            OperatorKind::Plus => write!(f, "Plus(+)"),
            OperatorKind::Minus => write!(f, "Minus(-)"),
            OperatorKind::Star => write!(f, "Star(*)"),
            OperatorKind::Slash => write!(f, "Slash(/)"),
            OperatorKind::Percent => write!(f, "Percentage(%)"),
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
            PuncuationKind::LeftAngleParen => write!(f, "Punctuation : <"),
            PuncuationKind::RightAngleParen => write!(f, "Punctuation : >"),
            PuncuationKind::SemiColon => write!(f, "Punctuation : ;"),
            PuncuationKind::Colon => write!(f, "Punctuation : :"),
            PuncuationKind::Comma => write!(f, "Punctuation : ,"),
        }
    }
}
