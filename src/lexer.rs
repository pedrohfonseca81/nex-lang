use core::panic;

use crate::errors::Errors;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Operators
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // One character
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Dot,
    Minus,
    Plus,
    Comma,
    Semicolon,
    Slash,
    Star,

    // Literals
    Number,
    Identifier,
    String,

    // Keywords
    And,
    Else,
    False,
    True,
    Fn,
    If,
    Nil,
    Or,
    Print,
    Return,
    Var,
    Match,
    Do,
    End,
    FnTypeAssigner,
    FnTypeImplication,

    // Types
    NumberType,
    FloatType,
    BoolType,
    StringType,
    NilType,
    FunctionType,
    UnknownType,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32,
}

impl Token {
    pub fn new(token: TokenType, lexeme: String, literal: String, line: i32) -> Token {
        Token {
            token,
            lexeme,
            literal,
            line,
        }
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn get_keyword(&mut self, identifier: String) -> Option<TokenType> {
        match identifier.as_str() {
            "and" => Some(TokenType::And),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fn" => Some(TokenType::Fn),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "match" => Some(TokenType::Match),
            "number" => Some(TokenType::NumberType),
            "float" => Some(TokenType::FloatType),
            "bool" => Some(TokenType::BoolType),
            "string" => Some(TokenType::StringType),
            _ => None,
        }
    }

    pub fn get_tokens(&mut self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            let _ = self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            "".to_string(),
            self.line,
        ));
    }

    fn scan_token(&mut self) -> Result<(), Errors> {
        let character = self.advance();

        match character {
            '\n' => self.line += 1,
            ' ' => (),
            '{' => self.add_token(TokenType::LeftBrace)?,
            '}' => self.add_token(TokenType::RightBrace)?,
            '(' => self.add_token(TokenType::LeftParen)?,
            ')' => self.add_token(TokenType::RightParen)?,
            ',' => self.add_token(TokenType::Comma)?,
            '.' => self.add_token(TokenType::Dot)?,
            '-' => {
                let token = if self.is_match('>') {
                    TokenType::FnTypeImplication
                } else {
                    TokenType::Minus
                };

                self.add_token(token)?
            }
            '+' => self.add_token(TokenType::Plus)?,
            ';' => self.add_token(TokenType::Semicolon)?,
            '*' => self.add_token(TokenType::Star)?,
            '!' => {
                let token = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token)?
            }
            '=' => {
                let token = if self.is_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token)?
            }
            '<' => {
                let token = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                self.add_token(token)?
            }
            '>' => {
                let token = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(token)?
            }
            '#' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            '/' => {
                if self.is_match('*') {
                    while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.line += 1;
                        }

                        self.advance();
                    }

                    if self.is_at_end() {
                        panic!("Unterminated comment");
                    }

                    self.advance();
                    self.advance();
                } else {
                    let _ = self.add_token(TokenType::Slash);
                }
            }
            ':' => {
                if self.is_match(':') && !self.is_at_end() {
                    let _ = self.add_token(TokenType::FnTypeAssigner);
                }
            }
            '"' => self.make_string(),
            c => {
                if c.is_digit(10) {
                    return self.make_number();
                } else if c.is_alphanumeric() {
                    return self.make_identifier();
                } else {
                    Errors::unexpected_character(c, self.line);
                }
            }
        };
        Ok(())
    }

    fn advance(&mut self) -> char {
        let current = self.current.clone();
        self.current += 1;

        self.source.chars().nth(current as usize).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
    }

    fn add_token(&mut self, token: TokenType) -> Result<(), Errors> {
        let _ = self.add_token_literal(token, None);

        Ok(())
    }

    fn add_token_literal(
        &mut self,
        token: TokenType,
        literal: Option<String>,
    ) -> Result<(), Errors> {
        let text = self.source[self.start.try_into().unwrap()..self.current.try_into().unwrap()]
            .to_string();

        self.tokens.push(Token::new(
            token,
            text,
            literal.unwrap_or("".to_string()),
            self.line,
        ));

        Ok(())
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len().try_into().unwrap() {
            return '\0';
        }

        self.source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap()
    }

    fn make_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string");
        }

        self.advance();

        let value = self.source[(self.start as usize) + 1..(self.current as usize) - 1].to_string();

        let _ = self.add_token_literal(TokenType::String, Some(value));
    }

    fn make_number(&mut self) -> Result<(), Errors> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
        }

        while self.peek().is_digit(10) {
            self.advance();
        }

        let value = self.source[(self.start as usize)..(self.current as usize)].to_string();

        self.add_token_literal(TokenType::Number, Some(value))
    }

    fn make_identifier(&mut self) -> Result<(), Errors> {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let value = self.source[(self.start as usize)..(self.current as usize)].to_string();

        if let Some(keyword) = self.get_keyword(value.clone()) {
            return self.add_token(keyword);
        }

        return self.add_token_literal(TokenType::Identifier, Some(value));
    }
}
