use sutra::lexer::Lexer;

fn main() {
    let inp : &str = "x=5;let";
    let mut lexer = Lexer::new(inp);
    lexer.lex();

    println!("{:#?}", lexer.output);
}