mod token;

use self::token::TokenType;
use crate::reporter;
use token::Token;

#[derive(Debug)]
pub struct Lexer {
    file_path: String,
    source: Vec<char>,
    line: usize,
    start: usize,
    current: usize,
    error: bool,
}

impl Lexer {
    pub fn from_file(path: &str) -> Self {
        let source = std::fs::read_to_string(path).unwrap();

        let mut ret = Lexer::from_string(source);
        ret.file_path = String::from(path);

        ret
    }

    pub fn from_string(source: String) -> Self {
        Lexer {
            file_path: String::new(),
            source: source.chars().collect(),
            line: 1,
            start: 0,
            current: 0,
            error: false,
        }
    }

    pub fn lex_tokens(&mut self) -> Option<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            // start of new lexem
            self.start = self.current;

            let token = self.get_next_token();

            if let Some(t) = token {
                tokens.push(t);
            }
        }

        if self.error {
            return None;
        }

        tokens.push(Token {
            token_type: token::TokenType::Eof,
            lexeme: String::from(""),
            line: self.line,
        });

        Some(tokens)
    }

    fn get_next_token(&mut self) -> Option<Token> {
        let c = self.advance();

        let token = match c {
            '(' => Some(TokenType::Lefparen),
            ')' => Some(TokenType::Rightparen),
            '{' => Some(TokenType::Leftcurl),
            '}' => Some(TokenType::Rightcurl),
            '[' => Some(TokenType::Leftbrack),
            ']' => Some(TokenType::Rightbrack),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            ';' => Some(TokenType::Semicolon),
            '\r' => None,
            '\t' => None,
            ' ' => None,
            '\n' => {
                self.line += 1;
                None
            }
            '"' => self.string_token(),

            // TODO: use macro to create two character tokens
            '&' => {
                if self.match_next('&') {
                    Some(TokenType::And)
                } else {
                    None
                }
            }
            '|' => {
                if self.match_next('|') {
                    Some(TokenType::Or)
                } else {
                    None
                }
            }
            '*' => {
                if self.match_next('*') {
                    Some(TokenType::Power)
                } else {
                    Some(TokenType::Star)
                }
            }
            '!' => {
                if self.match_next('=') {
                    Some(TokenType::Bangequal)
                } else {
                    Some(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    Some(TokenType::Equalequal)
                } else {
                    Some(TokenType::Equal)
                }
            }
            '>' => {
                if self.match_next('=') {
                    Some(TokenType::Greaterequal)
                } else {
                    Some(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_next('=') {
                    Some(TokenType::Lessequal)
                } else {
                    Some(TokenType::Less)
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }

                    None
                } else {
                    Some(TokenType::Slash)
                }
            }
            '0'..='9' => self.number_token(),
            _ => {
                if Lexer::is_alpha(c) {
                    self.identifier_token()
                } else {
                    reporter::report_error(
                        &format!("unexpected character '{}' found", c),
                        &self.file_path,
                        self.line,
                    );
                    self.error = true;
                    None
                }
            }
        };

        if let Some(t) = token {
            Some(Token {
                token_type: t,
                line: self.line,
                lexeme: self.get_lexem_string(),
            })
        } else {
            None
        }
    }

    fn identifier_token(&mut self) -> Option<TokenType> {
        while self.peek().is_digit(10) || Lexer::is_alpha(self.peek()) {
            self.advance();
        }

        match self.get_lexem_string().as_str() {
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fn" => Some(TokenType::Fn),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "null" => Some(TokenType::Null),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "while" => Some(TokenType::While),
            "in" => Some(TokenType::In),
            _ => Some(TokenType::Identifier),
        }
    }

    fn number_token(&mut self) -> Option<TokenType> {
        let mut is_float = false;

        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            is_float = true;

            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        if is_float {
            Some(TokenType::Double)
        } else {
            Some(TokenType::Int)
        }
    }

    fn string_token(&mut self) -> Option<TokenType> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            reporter::report_error("underminated string", &self.file_path, self.line);

            return None;
        }

        // closing "
        self.advance();

        Some(TokenType::String)
    }

    fn get_lexem_string(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    fn is_alpha(c: char) -> bool {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;

        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}