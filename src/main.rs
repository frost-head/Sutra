use sutra::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("()[]{}");
    lexer.lex();

    println!("{:#?}", lexer.output);
}