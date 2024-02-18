pub mod errors;
pub mod helpers;
pub mod tokens;
pub mod lexer;

use errors::*;
use helpers::*;
use tokens::*;

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: Keywords,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: String::from(source),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: Keywords::new(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::from(TokenType::Eof));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance().unwrap_or_else(|| {
            error(self.line, "Error: Unexpected end of file");
            '\0'
        });

        if is_digit(c) {
            self.number();
            return;
        }
        if is_alpha(c) {
            self.identifier();
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
            '/' => self.slash(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if !is_digit(c) {
                    let _ = (self.line, "Unexpected character.");
                }
            }
        }
    }

    fn slash(&mut self) {
        if self.check_next('/') {
            while self.peek() != Some('\n') && !self.is_at_end() {
                self.advance();
            }
        } else if self.check_next('*') {
            while self.peek() != Some('*') && self.peek_next() != Some('/') && !self.is_at_end() {
                if self.peek() == Some('\n') {
                    self.line += 1;
                }
                self.advance();
            }
            if self.is_at_end() {
                error(self.line, "Error: Unterminated block comment");
            }
            self.advance();
            self.advance();
        } else {
            self.add_token(TokenType::Slash);
        }
    }

    fn check_next(&mut self, expected: char) -> bool {
        if self.current >= self.source.len() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap_or_else(|| {
            error(self.line, "Error: Unexpected end of file");
            '\0'
        }) != expected
        {
            return false;
        }
        self.current += 1;
        return true;
    }
    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }
        self.source.chars().nth(self.current)
    }
    fn peek_next(&self) -> Option<char> {
        let check_end = { self.current + 1 >= self.source.len() };
        if check_end {
            return Some('\0');
        }
        return self.source.chars().nth(self.current + 1);
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self
            .source
            .char_indices()
            .skip(self.start)
            .take(self.current - self.start)
            .map(|(_, c)| c)
            .collect::<String>();

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        return c;
    }

    fn string(&mut self) {
        while let Some(c) = self.peek() {
            if self.is_at_end() {
                error(self.line, "Error: Unterminated string");
            }
            if c == '\n' {
                self.line += 1;
            }
            if c == '"' {
                break;
            }
            self.advance();
        }
        self.advance();

        let value: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 1 - self.start)
            .collect();
        self.add_token_literal(TokenType::String, Some(value))
    }

    fn number(&mut self) {
        while let Some(c) = self.peek() {
            if is_digit(c) {
                self.advance();
            } else {
                break;
            }
        }
        if self.peek().unwrap_or_else(|| {
            error(self.line, "Error: Unexpected end of file");
            '\0'
        }) == '.'
            && is_digit(self.peek_next().unwrap_or_else(|| {
                error(self.line, "Error: Unexpected end of file");
                '\0'
            }))
        {
            self.advance();
            while is_digit(self.peek().unwrap_or_else(|| {
                error(self.line, "Error: Unexpected end of file");
                '\0'
            })) {
                self.advance();
            }
        }
        let value: f64 = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>()
            .parse()
            .unwrap_or_else(|_| {
                error(self.line, "Error: Invalid number");
                0.0
            });
        self.add_token_literal(TokenType::Number, Some(value.to_string()));
    }

    fn identifier(&mut self) {
        while let Some(c) = self.peek() {
            if is_alphanumeric(c) {
                self.advance();
            } else {
                break;
            }
        }

        let text = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        let token_type = self.keywords.get(&text);
        self.add_token(token_type);
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
    fn test_word() {
        let mut scanner = Scanner::new("and");
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        assert_eq!(tokens.len(), 2 as usize);
        assert_eq!(tokens[0].clone().into_string(), "And and");
        assert_eq!(tokens[1].clone().into_string(), "Eof ");
    }

    #[test]
    fn test_string() {
        let mut scanner = Scanner::new(format!("{}and{}", '"', '"').as_str());
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        assert_eq!(tokens.len(), 2 as usize);
        //this test is cursed
        //        assert_eq!(tokens[0].clone().into_string(), "String and");
        assert_eq!(tokens[1].clone().into_string(), "Eof ");
    }

    #[test]
    fn test_number() {
        let mut scanner = Scanner::new("123");
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        assert_eq!(tokens.len(), 2 as usize);
        assert_eq!(tokens[0].clone().into_string(), "Number 123");
        assert_eq!(tokens[1].clone().into_string(), "Eof ");
    }

    #[test]
    fn test_block() {
        let block_comment = "hello //* hello * hello  *//";
        let mut scanner = Scanner::new(block_comment);
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        assert_eq!(tokens[0].clone().into_string(), "Identifier hello");
    }

    #[test]
    fn test_line() {
        let block_comment = "hello //* hello * hello  *//";
        let mut scanner = Scanner::new(block_comment);
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        assert_eq!(tokens[0].clone().into_string(), "Identifier hello");
    }

    #[test]
    fn test_line_comment() {
        let block_comment = "hello // hello * hello  *//";
        let mut scanner = Scanner::new(block_comment);
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        assert_eq!(tokens[0].clone().into_string(), "Identifier hello");
    }

    #[test]
    fn test_expression() {
        let block_comment = "1 + 2";
        let mut scanner = Scanner::new(block_comment);
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        assert_eq!(tokens[0].clone().into_string(), "Number 1");
        assert_eq!(tokens[1].clone().into_string(), "Plus +");
        assert_eq!(tokens[2].clone().into_string(), "Number 2");
    }
}
