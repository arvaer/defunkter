use crate::errors::token_error;
use crate::lexer::Expression;
use crate::{
    lexer::Literal,
    tokens::{Token, TokenType},
};
use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

//expression     → equality ;
//equality       → comparison ( ( "!=" | "==" ) comparison )* ;
//comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
//term           → factor ( ( "-" | "+" ) factor )* ;
//factor         → unary ( ( "/" | "*" ) unary )* ;
//unary          → ( "!" | "-" ) unary
//               | primary ;
//primary        → NUMBER | STRING | "true" | "false" | "nil"
//               | "(" expression ")" ;
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Option<Expression> {
        return self.equality();
    }
    fn equality(&mut self) -> Option<Expression> {
        let mut base_expr = self.comparison()?;
        while let Some(token_type) = self.peek().map(|token| token.token_type.clone()) {
            match token_type {
                TokenType::BangEqual | TokenType::EqualEqual => {
                    let operator = self.advance()?.clone();
                    let right = self.comparison()?;

                    base_expr = Expression::Binary {
                        left: Rc::new(base_expr),
                        operator,
                        right: Rc::new(right),
                    };
                }
                _ => break,
            }
        }
        return Some(base_expr);
    }

    fn comparison(&mut self) -> Option<Expression> {
        let mut base_expr = self.term()?;
        while let Some(token_type) = self.peek().map(|token| token.token_type.clone()) {
            match token_type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    let operator = self.advance()?.clone();
                    let right = self.term()?;

                    base_expr = Expression::Binary {
                        left: Rc::new(base_expr),
                        operator,
                        right: Rc::new(right),
                    };
                }
                _ => break,
            }
        }
        return Some(base_expr);
    }

    fn term(&mut self) -> Option<Expression> {
        let mut base_expr = self.factor()?;
        while let Some(token_type) = self.peek().map(|token| token.token_type.clone()) {
            match token_type {
                TokenType::Minus | TokenType::Plus => {
                    let operator = self.advance()?.clone();
                    let right = self.factor()?;

                    base_expr = Expression::Binary {
                        left: Rc::new(base_expr),
                        operator,
                        right: Rc::new(right),
                    };
                }
                _ => break,
            }
        }
        return Some(base_expr);
    }

    fn factor(&mut self) -> Option<Expression> {
        let mut base_expr = self.unary()?;
        while let Some(token_type) = self.peek().map(|token| token.token_type.clone()) {
            match token_type {
                TokenType::Slash | TokenType::Star => {
                    let operator = self.advance()?.clone();
                    let right = self.unary()?;

                    base_expr = Expression::Binary {
                        left: Rc::new(base_expr),
                        operator,
                        right: Rc::new(right),
                    };
                }
                _ => break,
            }
        }
        return Some(base_expr);
    }

    fn unary(&mut self) -> Option<Expression> {
        let token_type = self.peek().map(|token| token.token_type.clone());
        match token_type {
            Some(TokenType::Bang) | Some(TokenType::Minus) => {
                let operator = self.advance()?.clone();
                let unary = self.unary()?;
                return Some(Expression::Unary {
                    operator,
                    value: Rc::new(unary),
                });
            }
            _ => {
                return self.primary();
            }
        }
    }

    fn primary(&mut self) -> Option<Expression> {
        let token_type = self.peek().map(|token| token.token_type.clone());
        match token_type {
            Some(TokenType::False) | Some(TokenType::True) | Some(TokenType::Nil) => {
                let new_literal = Literal::KEYWORD(self.advance().unwrap().clone());
                return Some(Expression::Literal(new_literal));
            }
            Some(TokenType::String) => {
                let string = self.previous().unwrap().clone();
                let new_literal = Literal::STRING(string);
                return Some(Expression::Literal(new_literal));
            }
            Some(TokenType::Number) => {
                let string = self.previous().unwrap().clone();
                let new_literal = Literal::NUMBER(string);
                return Some(Expression::Literal(new_literal));
            }
            Some(TokenType::LeftParen) => {
                let base_expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ) after expression");
                return Some(Expression::Grouping {
                    interior: Rc::new(base_expr),
                });
            }
            Some(TokenType::Eof) | None | _ => {
                return None;
            }
        }
    }

    fn consume(&mut self, check_on: TokenType, message: &str) {
        if  self.check(check_on) {
            let _ = self.advance();
        } else {
            token_error(self.peek().unwrap().clone(), message)
        }
    }

    fn check(&self, check_on: TokenType) -> bool {
        if !self.is_at_end() {
            let token = self.peek();
            match token {
                Some(token) => {
                    if token.token_type == check_on {
                        return true;
                    } else {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        let token = self.peek();
        match token {
            Some(token) => {
                if token.token_type == TokenType::Eof {
                    return true;
                }
            }
            None => {
                return false;
            }
        }
        return false;
    }

    fn peek(&self) -> Option<&Token> {
        let current_token = self.tokens.get(self.current);
        return current_token;
    }
    fn previous(&self) -> Option<&Token> {
        let prev_token = self.tokens.get(self.current - 1);
        return prev_token;
    }
}
