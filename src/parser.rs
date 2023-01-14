use crate::expr::{Binary, Expr, Grouping, Literal, Space};
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

// Implement formatter for Parser
impl std::fmt::Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tokens = String::new();
        for token in &self.tokens {
            tokens.push_str(&format!(" {}", token));
        }
        write!(f, "{}", tokens)
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    pub fn expression(&mut self) -> Expr {
        self.term()
    }

    pub fn term(&mut self) -> Expr {
        self.factor()
    }

    pub fn factor(&mut self) -> Expr {
        self.unary()
    }

    pub fn unary(&mut self) -> Expr {
        self.primary()
    }

    pub fn primary(&mut self) -> Expr {
        if self.r#match(vec![TokenType::Space]) {
            let space = Space::new(self.previous().literal);
            return Expr {
                grouping: None,
                binary: None,
                literal: None,
                space: Some(space),
            };
        }

        if self.r#match(vec![TokenType::Number]) {
            let literal = Literal::new(self.previous().literal.parse::<u32>().unwrap());
            return Expr {
                grouping: None,
                binary: None,
                literal: Some(literal),
                space: None,
            };
        }

        if self.r#match(vec![TokenType::LeftParen, TokenType::RightParen]) {
            let expr = self.expression();
            
            if self.check(TokenType::RightParen) {
                self.advance();
            } else {
                panic!("Expect ')' after expression.");
            }
            
            return Expr {
                grouping: Some(Grouping {
                    expr: Box::new(expr),
                }),
                binary: None,
                literal: None,
                space: None,
            };
        }

        if self.r#match(vec![TokenType::Plus, TokenType::Minus, TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let mut values = vec![];
            self.consume(TokenType::Space, "Expect space after operator.");
            while self.r#match(vec![TokenType::Number]) {
                let expr = Expr {
                    grouping: None,
                    binary: None,
                    literal: Some(Literal::new(
                        self.previous().literal.parse::<u32>().unwrap(),
                    )),
                    space: None,
                };
                values.push(Box::new(expr));
            }
            let space = Expr {
                grouping: None,
                binary: None,
                literal: None,
                space: Some(Space::new(" ".to_string())),
            };
            values.push(Box::new(space));
            if self.r#match(vec![TokenType::Space]) {
                while self.r#match(vec![TokenType::Number]) {
                    let expr = Expr {
                        grouping: None,
                        binary: None,
                        literal: Some(Literal::new(
                            self.previous().literal.parse::<u32>().unwrap(),
                        )),
                        space: None,
                    };
                    values.push(Box::new(expr));
                }
            }
            if self.check(TokenType::LeftParen) {
                let expr = self.expression();
                values.push(Box::new(expr));
            }
            return Expr {
                grouping: None,
                binary: Some(Binary {
                    token: operator,
                    values: values,
                }),
                literal: None,
                space: None,
            };
        }

        panic!("Expect expression.")
    }

    pub fn r#match(&mut self, types: Vec<TokenType>) -> bool {
        for type_ in types {
            if self.check(type_) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check(token_type) {
            self.advance();
        } else {
            panic!("{}", message)
        }
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().r#type == token_type
    }

    pub fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous();
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::EOF
    }

    pub fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    pub fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
