pub mod tokens;
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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let is_at_end = { self.current > self.source.len() };
        while !is_at_end {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::from(TokenType::Eof));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
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
            _ => todo!(),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
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
        self.current += 1;
        return self.source.chars().nth(self.current).unwrap();
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
