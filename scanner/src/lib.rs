pub mod errors;
pub mod tokens;
use errors::*;
use tokens::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}
impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: String::from(source),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current > self.source.len()
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::from(TokenType::Eof));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        if self.is_digit(c) {
            self.number();
            return;
        }

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.check_next('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.check_next('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.check_next('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.check_next('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.check_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        let _ = self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if !self.is_digit(c) {
                    let _ = (self.line, "Unexpected character.");
                }
            } ,
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn check_next(&mut self, expected: char) -> bool {
        if self.current >= self.source.len() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }
    fn peek_next(&self) -> char {
        let check_end = {
            self.current + 1 >= self.source.len()
        };
        if check_end {
            return '\0'
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current)
            .collect();

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            error(self.line, "Unterminated String");
        }

        self.advance();

        let value: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 1)
            .collect();
        self.add_token_literal(TokenType::String, Some(value))
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) { self.advance(); }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() {
        let token = Token::new(TokenType::And, "and".to_string(), None, 1);
        assert_eq!(token.into_string(), "And and");
    }

    #[test]
    fn test_scanner() {
        let mut scanner = Scanner::new("and");
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 4);
    }
}
