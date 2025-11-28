use sutra::lexer::Lexer;

fn main() {
    let inp : &str = "
                let x = 5;
                let y = 10;
                let z = x + y;q
                println!(x = {}, y = {}, z = {}, x, y, z);
                println!(Hello, world!);
        ";
    let mut lexer = Lexer::new(inp);
    lexer.lex();

    println!("{:#?}", lexer.output);
}