#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN, // (
    RIGHTPAREN,
    LEFTCURL, // {
    RIGHTCURL,
    LEFTBRACK, // [
    RIGHTBRACK,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,

    // One or two character tokens.
    STAR,
    POWER,
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,
    AND,
    OR,

    // Literals.
    IDENTIFIER,
    STRING,
    INT,
    DOUBLE,

    // Keywords.
    CLASS,
    ELSE,
    FALSE,
    FN,
    FOR,
    IF,
    NULL,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    WHILE,
    IN,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {}
