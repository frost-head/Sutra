use super::*;

#[test]
fn first_token() {
    let mut lexer = Lexer::new(String::from("("));
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
    let mut lexer = Lexer::new(String::from("()"));
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
