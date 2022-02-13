mod lexer;
mod reporter;

use lexer::Lexer;

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

    println!("{:#?}", tokens);

    if tokens.is_none() {
        std::process::exit(-1);
    }
}
