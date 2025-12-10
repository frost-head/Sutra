use sutra::lexer::Lexer;
use sutra::parser::Parser;

fn main() {
    let inp: &str = "let x = 5;";
    let lexer = Lexer::new(inp);

    let mut parser = Parser::new(lexer, Vec<dyn Statement>::new());

    let out = parser.parseLetStatement()?;
    println!("{:#?}", out);
}
