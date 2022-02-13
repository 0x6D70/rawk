mod token;

use token::Token;

pub fn read_from_file(path: &str) -> Option<Vec<Token>> {
    let source = std::fs::read_to_string(path).unwrap();
    read(source)
}

pub fn read(source: String) -> Option<Vec<Token>> {
    return Some(vec![Token{}])
}
