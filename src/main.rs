use sutra::lexer::Lexer;
use sutra::parser::Parser;

fn main() {
    let inp: &str = "let x = 5; let y = 6;
    let z=2*4*5;
    return x + y;
    ";
    let lexer = Lexer::new(inp);

    let mut parser = Parser::new(lexer);
    if let Err(err) = parser.parse() {
        eprintln!("Error occured while parsing the input: {}", err);
        std::process::exit(1);
    }

    match &parser.ast.items[0] {
        sutra::ast::item::Item::Function(func_item) => {
            for s in func_item.body.statements.iter() {
                match s {
                    sutra::ast::block::Stmt::LetStmt(let_statement) => {
                        println!("{}", let_statement)
                    }
                    sutra::ast::block::Stmt::ReturnStmt(return_statement) => {
                        println!("{}", return_statement)
                    }
                };
            }
        }
        sutra::ast::item::Item::Struct() => {
            eprintln!("Error occured while parsing the input");
            std::process::exit(1);
        }
    }
}
