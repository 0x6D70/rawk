#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    Leftparen, // (
    Rightparen,
    Leftcurl, // {
    Rightcurl,
    Leftbrack, // [
    Rightbrack,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Remainder,

    // One or two character tokens.
    Star,
    Power,
    Bang,
    Bangequal,
    Equal,
    Equalequal,
    Greater,
    Greaterequal,
    Less,
    Lessequal,
    And,
    Or,

    // Literals.
    Identifier,
    String,
    Int,
    Double,

    // Keywords.
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Null,
    Return,
    Super,
    This,
    True,
    While,
    In,
}

#[derive(Debug, Clone, Copy)]
pub struct TokenSpan {
    pub start: usize, // inclusive
    pub end: usize,   // exclusive
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub span: TokenSpan,
}

impl Token {}
