mod lexer;
mod parser;
mod reporter;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: rawk <filename>");
        return;
    }

    run_file(&args[1]);
}

fn run_file(path: &str) {
    let mut lexer = Lexer::from_file(path);

    let tokens = lexer.lex_tokens();

    if tokens.is_none() {
        std::process::exit(-1);
    }

    let tokens = tokens.unwrap();

    let mut parser = Parser::new(tokens);

    println!("{:#?}", &parser);

    parser.parse();
}
