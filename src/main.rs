use sutra::token::{Token, TokenKind};
use sutra::lexer::Lexer;

fn main() {
    let firsttoken = Token {
        literal: String::from("2"),
        kind: TokenKind::LeftParen,
    };
    let mut lexer = Lexer::new(String::from("()[]{}"));
    let tok =  lexer.next();
    println!("{:?}", tok);
    let tok = match tok {
        Some(token) => token,
        None => panic!("Unexpected end of input"),
    };
    println!("Printing token :: {:?}", tok);
    let perfect = Token::new(TokenKind::LeftParen, String::from("("));
    
    println!("Printing perfect token :: {:?}", perfect);
    println!("Frist token {:#?}", firsttoken);
}
