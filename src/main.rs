use sutra::lexer::Lexer;
use sutra::parser::Parser;

fn main() {
    let inp: &str = "let x = 5; let y = 6;
    let z=2*4*5;
    return x + y;
    ";
    let lexer = Lexer::new(inp);

    let mut parser = Parser::new(lexer);
    parser.parse();

    for s in parser.ast.statements.iter() {
        println!("{}", s);
    }
}
