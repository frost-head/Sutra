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
    let perfect = Token::Punctuation(PuncuationKind::LeftParen);

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

    let perfect = Token::Punctuation(PuncuationKind::RightParen);

    println!("Printing perfect token :: {:?}", perfect);
    assert_eq!(tok, perfect);
}

#[test]
fn all_tokens() {
    let mut lexer = Lexer::new("()[]{}");
    lexer.lex();

    let out = vec![
        Token::Punctuation(PuncuationKind::LeftParen),
        Token::Punctuation(PuncuationKind::RightParen),
        Token::Punctuation(PuncuationKind::LeftSquareParen),
        Token::Punctuation(PuncuationKind::RightSquareParen),
        Token::Punctuation(PuncuationKind::LeftCurlyParen),
        Token::Punctuation(PuncuationKind::RightCurlyParen),
        Token::EOF,
    ];

    assert_eq!(lexer.output, out);
}

#[test]
fn func() {
    let mut lexer = Lexer::new(
        "
        func main() {
            let x = 5;
        }
        ",
    );
    lexer.lex();

    let out = vec![
        Token::Keyword(KeywordKind::Func),
        Token::Ident("main".to_string()),
        Token::Punctuation(PuncuationKind::LeftParen),
        Token::Punctuation(PuncuationKind::RightParen),
        Token::Punctuation(PuncuationKind::LeftCurlyParen),
        Token::Keyword(KeywordKind::Let),
        Token::Ident("x".to_string()),
        Token::Operator(OperatorKind::Equal),
        Token::Number(5),
        Token::Punctuation(PuncuationKind::SemiColon),
        Token::Punctuation(PuncuationKind::RightCurlyParen),
        Token::EOF,
    ];

    assert_eq!(lexer.output, out);
}
