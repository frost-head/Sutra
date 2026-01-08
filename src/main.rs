use anyhow::Result;
use clap::Parser as clap_parser;
use std::fs;
use sutra::parser::Parser;
use sutra::resolver::Resolver;
use sutra::resolver::types::{TypeId, TypeKind, TypeTable};
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

    write_file(output_buffer);

    let mut type_table = TypeTable::new();
    let resolver = Resolver::new();

    let ast = parser.ast.clone();

    for item in &ast.items {
        match item {
            Item::Function(func_item) => {
                let mut return_id = None;
                let mut param_ids: Option<Vec<TypeId>> = None;
                if let Some(return_type) = func_item.return_type.clone() {
                    return_id = Some(type_table.type_ref_to_type(return_type.type_ref)?);
                }
                if let Some(param_types) = func_item.params.clone() {
                    param_ids = Some(
                        param_types
                            .into_iter()
                            .map(|param_type| -> Result<TypeId> {
                                Ok(type_table.type_ref_to_type(param_type.type_ref)?)
                            })
                            .collect::<Result<Vec<TypeId>>>()?,
                    );
                }

                let func_id = TypeKind::Function {
                    params: param_ids,
                    ret: return_id,
                };
                let func_id = type_table.intern(func_id);
                println!("Function ID: {:?}", func_id);
            }
            _ => {
                eprintln!("Error occurred while parsing the input");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

#[derive(clap_parser)]
struct Args {
    /// Input file to read
    #[clap(short, long)]
    input_file: String,
    #[clap(short, long)]
    output_file: String,
}

fn read_file() -> String {
    let args = Args::parse();
    let content = fs::read_to_string(&args.input_file).expect("failed to read file");
    content
}

fn write_file(output_buffer: String) {
    let args = Args::parse();
    fs::write(args.output_file, &output_buffer).expect("failed to write file");
}
