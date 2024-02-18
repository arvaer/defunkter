use crate::tokens::{Token, TokenType};
use std::rc::Rc;

pub enum Literal {
    NUMBER(Token),
    STRING(Token),
    KEYWORD(Token),
}

trait Print {
    fn print(&self) -> String;
}

pub enum Expression {
    Unary {
        operator: Token,
        value: Rc<Expression>,
    },
    Binary {
        left: Rc<Expression>,
        operator: Token,
        right: Rc<Expression>,
    },
    Grouping {
        interior: Rc<Expression>,
    },
    Literal(Literal),
}

impl Print for Expression {
    fn print(&self) -> String {
        let print_val: String = match self {
            Expression::Unary { operator, value } => {
                parenthesize(operator.lexeme.as_str(), vec![value.clone()])
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let exprs = vec![left.clone(), right.clone()];
                parenthesize(operator.lexeme.as_str(), exprs)
            }
            Expression::Grouping { interior } => parenthesize("group", vec![interior.clone()]),
            Expression::Literal(c) => match c {
                Literal::NUMBER(t) => t.clone().lexeme,
                Literal::STRING(t) => t.clone().lexeme,
                Literal::KEYWORD(t) => {
                    if t.token_type == TokenType::Nil {
                        return String::from("nil");
                    }
                    t.clone().into_string()
                }
            },
        };
        return print_val;
    }
}

fn parenthesize(name: &str, exprs: Vec<Rc<Expression>>) -> String {
    let mut string: String = String::from("(");
    string.push_str(name);
    for expr in exprs {
        string.push_str(" ");
        string.push_str(expr.print().as_str());
    }
    string.push(')');

    return string;
}

pub struct AST {
    ast: Vec<Rc<Expression>>,
}

impl AST {
    pub fn print(self) {
        for exp in self.ast {
            print!("{}", exp.print());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn print_number_literal() {
        let expr = Expression::Literal(Literal::NUMBER(Token {
            token_type: TokenType::Number,
            lexeme: "42".to_string(),
            literal: None, // Adjust according to your Token struct
            line: 1,
        }));

        assert_eq!(Rc::new(expr).print(), "42");
    }

    #[test]
    fn print_unary_expression() {
        let expr = Expression::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            value: Rc::new(Expression::Literal(Literal::NUMBER(Token {
                token_type: TokenType::Number,
                lexeme: "42".to_string(),
                literal: None,
                line: 1,
            }))),
        };

        assert_eq!(Rc::new(expr).print(), "(- 42)");
    }

    #[test]
    fn print_binary_expression() {
        let left = Rc::new(Expression::Literal(Literal::NUMBER(Token {
            token_type: TokenType::Number,
            lexeme: "1".to_string(),
            literal: None,
            line: 1,
        })));

        let right = Rc::new(Expression::Literal(Literal::NUMBER(Token {
            token_type: TokenType::Number,
            lexeme: "2".to_string(),
            literal: None,
            line: 1,
        })));

        let expr = Expression::Binary {
            left,
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line: 1,
            },
            right,
        };

        assert_eq!(Rc::new(expr).print(), "(+ 1 2)");
    }

    #[test]
    fn print_grouping_expression() {
        let expr = Expression::Grouping {
            interior: Rc::new(Expression::Literal(Literal::NUMBER(Token {
                token_type: TokenType::Number,
                lexeme: "42".to_string(),
                literal: None,
                line: 1,
            })),
            ),
        };

        assert_eq!(Rc::new(expr).print(), "(group 42)");
    }

    #[test]
    fn print_nested_expression() {
        let left = Rc::new(Expression::Literal(Literal::NUMBER(Token {
            token_type: TokenType::Number,
            lexeme: "1".to_string(),
            literal: None,
            line: 1,
        })));

        let right = Rc::new(Expression::Literal(Literal::NUMBER(Token {
            token_type: TokenType::Number,
            lexeme: "2".to_string(),
            literal: None,
            line: 1,
        })));

        let expr = Expression::Binary {
            left,
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line: 1,
            },
            right,
        };

        let outer_expr = Expression::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            value: Rc::new(expr),
        };

        assert_eq!(Rc::new(outer_expr).print(), "(- (+ 1 2))");
    }

    #[test]
    fn print_literal_nil() {
        let expr = Expression::Literal(Literal::KEYWORD(Token {
            token_type: TokenType::Nil,
            lexeme: "nil".to_string(),
            literal: None,
            line: 1,
        }));

        assert_eq!(Rc::new(expr).print(), "nil");
    }
}
