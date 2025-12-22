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
    let perfect = Token::new(TokenKind::LeftParen, String::from("("));

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

    let perfect = Token::new(TokenKind::RightParen, String::from(")"));

    println!("Printing perfect token :: {:?}", perfect);
    assert_eq!(tok, perfect);
}

#[test]
fn all_tokens() {
    let mut lexer = Lexer::new("()[]{}");
    lexer.lex();

    let out = vec![
        Token::new(TokenKind::LeftParen, String::from("(")),
        Token::new(TokenKind::RightParen, String::from(")")),
        Token::new(TokenKind::LeftSquareParen, String::from("[")),
        Token::new(TokenKind::RightSquareParen, String::from("]")),
        Token::new(TokenKind::LeftCurlyParen, String::from("{")),
        Token::new(TokenKind::RightCurlyParen, String::from("}")),
        Token::new(TokenKind::EOF, String::from("EOF")),
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
        Token::new(TokenKind::FUNC, String::from("func")),
        Token::new(TokenKind::IDENT, String::from("main")),
        Token::new(TokenKind::LeftParen, String::from("(")),
        Token::new(TokenKind::RightParen, String::from(")")),
        Token::new(TokenKind::LeftCurlyParen, String::from("{")),
        Token::new(TokenKind::LET, String::from("let")),
        Token::new(TokenKind::IDENT, String::from("x")),
        Token::new(TokenKind::Equal, String::from("=")),
        Token::new(TokenKind::INT, String::from("5")),
        Token::new(TokenKind::SemiColon, String::from(";")),
        Token::new(TokenKind::RightCurlyParen, String::from("}")),
        Token::new(TokenKind::EOF, String::from("EOF")),
    ];

    assert_eq!(lexer.output, out);
}
