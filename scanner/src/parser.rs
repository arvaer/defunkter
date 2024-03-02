use crate::errors::token_error;
use crate::lexer::Expression;
use crate::{
    lexer::Literal,
    tokens::{Token, TokenType},
};
use std::rc::Rc;

pub enum Equality {
    Left(Comparison),
    Right(Vec<EqualitySubexp>),
}
pub enum EqualitySubexp {
    Operator(Token),
    Operand(Rc<Comparison>),
}

pub enum Comparison {
    Left(Term),
    Right(Vec<ComparisonSubexp>),
}

pub enum ComparisonSubexp {
    Operator(Token),
    Operands(Rc<Term>),
}

pub enum Term {
    Left(Factor),
    Right(Vec<TermSubexp>),
}

pub enum TermSubexp {
    Operator(Token),
    Operands(Rc<Factor>),
}

pub enum Factor {
    Left(Rc<Unary>),
    Right(Vec<FactorSubexp>),
}

pub enum FactorSubexp {
    Operator(Token),
    Operands(Rc<Unary>),
}

pub enum Unary {
    Left { operation: Token, unary: Rc<Unary> },
    Right(Primary),
}

pub enum Primary {
    NUMBER(Token),
    STRING(Token),
    KEYWORD(Token),
    GROUPING(Rc<Expression>),
}

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

    fn primary(&mut self) -> Option<Expression> {
        let token_type = self.peek().map(|token| token.token_type.clone());
        match token_type {
            Some(TokenType::False) | Some(TokenType::True) | Some(TokenType::Nil) => {
                self.advance();
                let new_literal = Literal::KEYWORD(self.previous().unwrap().clone());
                return Some(Expression::Literal(new_literal));
            }
            Some(TokenType::String) => {
                self.advance();
                let string = self.previous().unwrap().clone();
                let new_literal = Literal::STRING(string);
                return Some(Expression::Literal(new_literal));
            }
            Some(TokenType::Number) => {
                self.advance();
                let string = self.previous().unwrap().clone();
                let new_literal = Literal::NUMBER(string);
                return Some(Expression::Literal(new_literal));
            }
            Some(TokenType::LeftParen) => {
                todo!()
            }
            Some(TokenType::Eof) | None | _ => {
                return None;
            }
        }
    }

    fn consume(&self, token: Token, message: &str) {
        let token_type = self.peek().map(|token| token.token_type.clone());
        match self.check(token_type) {
            true => {
                let _ = self.advance();
            },
            false => token_error(token, message)
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
