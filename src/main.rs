use sutra::lexer::Lexer;
use sutra::parser::Parser;

fn main() {
    let inp: &str = "let x = 5; let y = 6;
    let z=2*4*5;
    ";
    let lexer = Lexer::new(inp);

    let mut parser = Parser::new(lexer);

    // let out = parser.parseLetStatement();
    // match out {
    //     Some(_) => println!("{:#?}", out),
    //     None => println!("Not read"),
    // };

    parser.parse();

    for s in parser.ast.statements.iter() {
        println!("{}", s);
    }
}
