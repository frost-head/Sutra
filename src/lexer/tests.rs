use crate::{errors::span::Span, lexer::token::TokenKind};

use super::*;

#[test]
fn first_token() {
    let mut lexer = Lexer::new("(");
    let tok = lexer.next();
    let tok = match tok {
        Some(token) => token,
        None => panic!("Unexpected end of input"),
    };
    println!("Printing token :: {:?}", tok);
    let perfect = Token {
        kind: TokenKind::Punctuation(PuncuationKind::LeftParen),
        span: Span { start: 0, end: 1 },
    };

    println!("Printing perfect token :: {:?}", perfect);
    assert_eq!(tok, perfect);
}

#[test]
fn second_token() {
    let mut lexer = Lexer::new("()");
    let _ = lexer.next();
    let tok = lexer.next();
    let tok = match tok {
        Some(token) => token,
        None => panic!("Unexpected end of input"),
    };
    println!("Printing token :: {:?}", tok);

    let perfect = Token {
        kind: TokenKind::Punctuation(PuncuationKind::RightParen),
        span: Span { start: 1, end: 2 },
    };

    println!("Printing perfect token :: {:?}", perfect);
    assert_eq!(tok, perfect);
}

#[test]
fn all_tokens() {
    let mut lexer = Lexer::new("()[]{}");
    lexer.lex();

    let out = vec![
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::LeftParen),
            span: Span { start: 0, end: 1 },
        },
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::RightParen),
            span: Span { start: 1, end: 2 },
        },
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::LeftSquareParen),
            span: Span { start: 2, end: 3 },
        },
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::RightSquareParen),
            span: Span { start: 3, end: 4 },
        },
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::LeftCurlyParen),
            span: Span { start: 4, end: 5 },
        },
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::RightCurlyParen),
            span: Span { start: 5, end: 6 },
        },
        Token {
            kind: TokenKind::EOF,
            span: Span { start: 6, end: 6 },
        },
    ];

    assert_eq!(lexer.output, out);
}

#[test]
fn func() {
    let mut lexer = Lexer::new(
        "func main() { let x = 5;}",
    );
    lexer.lex();

    let out = vec![
        Token {
            kind: TokenKind::Keyword(KeywordKind::Func),
            span: Span { start: 0, end: 4 },
        },
        Token {
            kind: TokenKind::Ident("main".to_string()),
            span: Span { start: 5, end: 9 },
        },
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::LeftParen),
            span: Span { start: 9, end: 10 },
        },
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::RightParen),
            span: Span { start: 10, end: 11 },
        },
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::LeftCurlyParen),
            span: Span { start: 12, end: 13 },
        },
        Token {
            kind: TokenKind::Keyword(KeywordKind::Let),
            span: Span { start: 14, end: 17 },
        },
        Token {
            kind: TokenKind::Ident("x".to_string()),
            span: Span { start: 18, end: 19 },
        },
        Token {
            kind: TokenKind::Operator(OperatorKind::Equal),
            span: Span { start: 20, end: 21 },
        },
        Token {
            kind: TokenKind::Number(5),
            span: Span { start: 22, end: 23 },
        },
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::SemiColon),
            span: Span { start: 23, end: 24 },
        },
        Token {
            kind: TokenKind::Punctuation(PuncuationKind::RightCurlyParen),
            span: Span { start: 24, end: 25 },
        },
        Token {
            kind: TokenKind::EOF,
            span: Span { start: 25, end: 25 },
        },
    ];

    assert_eq!(lexer.output, out);
}
