mod lexer;
mod reporter;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: rawk <filename>");
        return;
    }

    run_file(&args[1]);
}

fn run_file(path: &str) {
    let tokens = lexer::read_from_file(path);

    println!("{:#?}", tokens);

    if let None = tokens {
        std::process::exit(-1);
    }
}
