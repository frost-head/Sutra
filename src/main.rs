fn main() {
    let firsttoken = sutra::Token {
        literal: String::from("2"),
        kind: sutra::TokenKind::LeftParen,
    };
    println!("Frist token {:#?}", firsttoken);
}
