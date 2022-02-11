mod lexer;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: rawk <filename>");
        return;
    }

    let path = &args[1];

    let tokens = lexer::read_from_file(path);

    println!("{:#?}", tokens);
}
