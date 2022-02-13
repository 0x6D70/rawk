mod token;

use token::Token;
use crate::reporter;
use self::token::TokenType;

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
            line: 0,
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
            token_type: token::TokenType::EOF,
            lexeme: String::from(""),
            line: self.line,
        });       

        Some(tokens)
    }

    fn get_next_token(&mut self) -> Option<Token> {
        let c = self.advance();

        let token = match c {
            '(' => Some(TokenType::LEFTPAREN),
            ')' => Some(TokenType::RIGHTPAREN),
            '{' => Some(TokenType::LEFTCURL),
            '}' => Some(TokenType::RIGHTCURL),
            '[' => Some(TokenType::LEFTBRACK),
            ']' => Some(TokenType::RIGHTBRACK),
            ',' => Some(TokenType::COMMA),
            '.' => Some(TokenType::DOT),
            '+' => Some(TokenType::PLUS),
            '-' => Some(TokenType::MINUS),
            ';' => Some(TokenType::SEMICOLON),
            '\r' => None,
            '\t' => None,
            ' ' => None,
            '\n' => {
                self.line += 1;
                None
            },
            '"' => self.string_token(),

            // TODO: use macro to create two character tokens
            '&' => {
                if self.match_next('&') {
                    Some(TokenType::AND)
                } else {
                    None
                }
            },
            '|' => {
                if self.match_next('|') {
                    Some(TokenType::OR)
                } else {
                    None
                }
            },
            '*' => {
                if self.match_next('*') {
                    Some(TokenType::POWER)
                } else {
                    Some(TokenType::STAR)
                }
            },
            '!' => {
                if self.match_next('=') {
                    Some(TokenType::BANGEQUAL)
                } else {
                    Some(TokenType::BANG)
                }
            },
            '=' => {
                if self.match_next('=') {
                    Some(TokenType::EQUALEQUAL)
                } else {
                    Some(TokenType::EQUAL)
                }
            },
            '>' => {
                if self.match_next('=') {
                    Some(TokenType::GREATEREQUAL)
                } else {
                    Some(TokenType::GREATER)
                }
            },
            '<' => {
                if self.match_next('=') {
                    Some(TokenType::LESSEQUAL)
                } else {
                    Some(TokenType::LESS)
                }
            },
            '/' => {
                if self.match_next('/') {

                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    
                    None
                } else {
                    Some(TokenType::SLASH)
                }
            },
            '0'..='9' => {
                self.number_token()
            }
            _   => {
                if Lexer::is_alpha(c) {
                    self.identifier_token()
                } else {
                    reporter::report_error(&format!("unexpected character '{}' found", c), &self.file_path, self.line);
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
            "class" => Some(TokenType::CLASS),
            "else" => Some(TokenType::ELSE),
            "false" => Some(TokenType::FALSE),
            "fn" => Some(TokenType::FN),
            "for" => Some(TokenType::FOR),
            "if" => Some(TokenType::IF),
            "null" => Some(TokenType::NULL),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "while" => Some(TokenType::WHILE),
            "in" => Some(TokenType::IN),
            _ => Some(TokenType::IDENTIFIER),
        }
    }

    fn number_token(&mut self) -> Option<TokenType> {
        let mut is_float = false;

        while self.advance().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            is_float = true;

            self.advance();

            while self.advance().is_digit(10) {
                self.advance();
            }
        }

        if is_float {
            Some(TokenType::DOUBLE)
        } else {
            Some(TokenType::INT)
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

        Some(TokenType::STRING)
    }

    fn get_lexem_string(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    fn is_alpha(c: char) -> bool {
        ('a'..='z').contains(&c) ||
        ('A'..='Z').contains(&c) ||
            c == '_'
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
