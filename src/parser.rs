use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;

#[macro_export]
macro_rules! match_tokens {
    ( $parser:expr, $( $x:expr ),* ) => {
        {
            let mut ret = false;
            $(
                if $parser.tokens[$parser.current].token_type == $x {
                    $parser.advance();
                    ret = true;
                }
            )*
            ret
        }
    };
}

// assert!(match_tokens!(self, TokenType::Fn));
// assert!(match_tokens!(self, TokenType::Fn, TokenType::Identifier));
// assert!(!match_tokens!(self, TokenType::Identifier));


#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
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
        self.equality();
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

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        todo!()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn advance(&mut self) {
        todo!()
    }

    fn is_at_end(&self) -> bool {
        self.tokens.len() <= self.current
    }
}
