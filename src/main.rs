use sutra::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new(String::from("()[]{}"));
    lexer.lex();

    println!("{:#?}", lexer.output);
}
