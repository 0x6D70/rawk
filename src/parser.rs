use crate::lexer::token::{Token, TokenType};
use crate::reporter;

#[macro_export]
macro_rules! match_tokens {
    ( $parser:expr, $( $x:expr ),* ) => {
        {
            let mut ret = false;
            $(
                if $parser.check($x) {
                    $parser.advance();
                    ret = true;
                }
            )*
            ret
        }
    };
}

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),

    // TODO: find better way of doing this (maybe seperate enum???)
    LiteralTrue,
    LiteralFalse,
    LiteralNull,
    LiteralInt(i64),
    LiteralDouble(f64),
    LiteralString(String),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) {
        let expr = self.expression();

        println!("{:#?}", expr);
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while match_tokens!(self, TokenType::Bangequal, TokenType::Equalequal) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while match_tokens!(
            self,
            TokenType::Greater,
            TokenType::Greaterequal,
            TokenType::Less,
            TokenType::Lessequal
        ) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while match_tokens!(self, TokenType::Plus, TokenType::Minus) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while match_tokens!(self, TokenType::Slash, TokenType::Star) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if match_tokens!(self, TokenType::Bang, TokenType::Minus) {
            let op = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator: op,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        // TODO: refactor this (probably use match???)
        if match_tokens!(self, TokenType::False) {
            return Expr::LiteralFalse;
        }
        if match_tokens!(self, TokenType::True) {
            return Expr::LiteralTrue;
        }
        if match_tokens!(self, TokenType::Null) {
            return Expr::LiteralNull;
        }
        if match_tokens!(self, TokenType::Int) {
            return Expr::LiteralInt(self.previous().lexeme.parse().unwrap());
        }
        if match_tokens!(self, TokenType::Double) {
            return Expr::LiteralDouble(self.previous().lexeme.parse().unwrap());
        }
        if match_tokens!(self, TokenType::String) {
            return Expr::LiteralString(self.previous().lexeme);
        }

        if match_tokens!(self, TokenType::Leftparen) {
            let expr = self.expression();
            self.consume_token(TokenType::Rightparen, "Expect ')' after expression.");
            return Expr::Grouping(Box::new(expr));
        }

        Self::error(self.peek(), "Expect expression.");
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn consume_token(&mut self, token_type: TokenType, msg: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

        Self::error(self.peek(), msg);
    }

    fn error(token: Token, msg: &str) -> ! {
        // TODO: properly report file and line numbe here
        reporter::report_error(format!("{:#?} {}", token, msg).as_str(), "", token.line);
        panic!("parsing error");
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}

#[test]
fn test_pasic_parsing() {
    let mut parser = Parser::new(vec![
        Token {
            token_type: TokenType::Int,
            lexeme: String::from("3"),
            line: 1,
        },
        Token {
            token_type: TokenType::Plus,
            lexeme: String::from("+"),
            line: 1,
        },
        Token {
            token_type: TokenType::Int,
            lexeme: String::from("2"),
            line: 1,
        },
        Token {
            token_type: TokenType::Eof,
            lexeme: String::from(""),
            line: 1,
        },
    ]);

    let expr = parser.expression();

    assert!(matches!(
        expr,
        Expr::Binary {
            left: _,
            operator: _,
            right: _
        }
    ));

    if let Expr::Binary {
        left,
        operator,
        right,
    } = expr
    {
        assert!(operator.token_type == TokenType::Plus);

        assert!(matches!(*left, Expr::LiteralInt(3)));
        assert!(matches!(*right, Expr::LiteralInt(2)));
    }
}

#[test]
fn test_match_tokens() {
    let mut parser = Parser::new(vec![
        Token {
            token_type: TokenType::Int,
            lexeme: String::from("3"),
            line: 1,
        },
        Token {
            token_type: TokenType::Plus,
            lexeme: String::from("+"),
            line: 1,
        },
        Token {
            token_type: TokenType::Int,
            lexeme: String::from("2"),
            line: 1,
        },
        Token {
            token_type: TokenType::Eof,
            lexeme: String::from(""),
            line: 1,
        },
    ]);

    assert!(match_tokens!(parser, TokenType::Int));
    assert!(match_tokens!(parser, TokenType::Int, TokenType::Plus));
}
