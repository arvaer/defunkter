use std::collections::HashMap;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.
    Identifier,
    String,
    Number,

    // keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub struct Keywords {
    pub keywords: HashMap<String, TokenType>,
}
impl Keywords {
    pub fn new() -> Self {
     let keywords: HashMap<String, TokenType> =  HashMap::from([
         ("and".to_string(), TokenType::And),
         ("class".to_string(), TokenType::Class),
         ("else".to_string(), TokenType::Else),
         ("false".to_string(), TokenType::False),
         ("for".to_string(), TokenType::For),
         ("fun".to_string(), TokenType::Fun),
         ("if".to_string(), TokenType::If),
         ("nil".to_string(), TokenType::Nil),
         ("or".to_string(), TokenType::Or),
         ("print".to_string(), TokenType::Print),
         ("return".to_string(), TokenType::Return),
         ("super".to_string(), TokenType::Super),
         ("this".to_string(), TokenType::This),
         ("true".to_string(), TokenType::True),
         ("var".to_string(), TokenType::Var),
         ("while".to_string(), TokenType::While),
     ]);
     return Self { keywords };
    }

    pub fn get(&self, key: &String) -> TokenType {
        match self.keywords.get(key) {
            Some(token_type) => token_type.clone(),
            None => TokenType::Identifier,
        }
    }
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
    pub fn into_string(self) -> String {
        format!("{:?} {}", self.token_type, self.lexeme)
    }
}

impl From<TokenType> for Token {
    fn from(token_type: TokenType) -> Self {
        Self {
            token_type,
            lexeme: String::new(),
            literal: None,
            line: 0,
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            token_type: self.token_type.clone(),
            lexeme: self.lexeme.clone(),
            literal: self.literal.clone(),
            line: self.line,
        }
    }
}
