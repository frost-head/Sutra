use sutra::lexer::Lexer;
use sutra::parser::Parser;

fn main() {
    let inp: &str = "let x = 5; let y = 6;
    let z=2*4*5;
    return x + y
    ";
    let lexer = Lexer::new(inp);

    let mut parser = Parser::new(lexer);
    parser.parse().unwrap_or_else(|err| {
        eprintln!("Error occured while parsing the input: {}", err);
        std::process::exit(1);
    });

    for s in parser.ast.statements.iter() {
        println!("{}", s);
    }
}
