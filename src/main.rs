use anyhow::Result;
use clap::Parser as clap_parser;
use std::fs;
use sutra::parser::Parser;
use sutra::resolver::Resolver;

use sutra::{ast::item::Item, lexer::Lexer};

fn main() -> Result<()> {
    let content = read_file();

    let lexer = Lexer::new(&content);

    let mut parser = Parser::new(lexer);
    let _ = parser.parse()?;

    let mut output_buffer = String::new();

    for item in &parser.ast.items {
        match item {
            Item::Function(func_item) => {
                output_buffer.push_str(&func_item.to_string());
            }
            _ => {
                eprintln!("Error occurred while parsing the input");
                std::process::exit(1);
            }
        }
    }

    write_to_file(output_buffer, "demo/IR/main.parser_ast".to_string());

    let mut resolver = Resolver::new();

    let ast = parser.ast.clone();

    resolver.resolve_global(ast)?;
    println!("resolver : {}", resolver);

    Ok(())
}

#[derive(clap_parser)]
struct Args {
    /// Input file to read
    #[clap(short, long, default_value = "demo/main.su")]
    input_file: String,
    #[clap(short, long, default_value = "demo/main.ir")]
    output_file: String,
}

fn read_file() -> String {
    let args = Args::parse();
    let content = fs::read_to_string(&args.input_file).expect("failed to read file");
    content
}

fn write_to_file(output_buffer: String, file_name: String) {
    fs::write(file_name, &output_buffer).expect("failed to write file");
}
