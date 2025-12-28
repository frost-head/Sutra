use std::fs;

use sutra::lexer::Lexer;
use sutra::parser::Parser;

use clap::{Arg, Command};

fn main() {
    let content = read_file();

    let lexer = Lexer::new(&content);

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

fn read_file() -> String {
    let matches = Command::new("app")
        .arg(Arg::new("file").required(true))
        .get_matches();

    let file = matches.get_one::<String>("file").unwrap();
    println!("{}", file);
    let content = fs::read_to_string(file).expect("failed to read file");
    content
}
