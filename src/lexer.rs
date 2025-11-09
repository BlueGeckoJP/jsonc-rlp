use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    // 1-based indexing for line and column
    line: usize,
    pub tokens: Vec<Token>,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let current_char = input.chars().next();
        Lexer {
            input,
            position: 0,
            line: 1,
            tokens: Vec::new(),
            current_char,
        }
    }

    pub fn advance(&mut self) {
        if let Some(c) = self.current_char
            && c == '\n'
        {
            self.line += 1;
        }

        self.position += 1;
        self.current_char = if self.position < self.input.len() {
            Some(self.input.chars().nth(self.position).unwrap())
        } else {
            None
        };
    }

    pub fn peek(&self, offset: usize) -> Option<char> {
        if self.position + offset < self.input.len() {
            Some(self.input.chars().nth(self.position + offset).unwrap())
        } else {
            None
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn skip_single_comment(&mut self) {
        while let Some(c) = self.current_char {
            if c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn skip_multi_comment(&mut self) {
        while let Some(c) = self.current_char {
            if c == '*' && self.peek(1) == Some('/') {
                self.advance(); // consume '*'
                self.advance(); // consume '/'
                break;
            } else {
                self.advance();
            }
        }
    }

    pub fn number(&mut self) {
        let mut num = String::new();

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() || c == '.' {
                num.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if num.is_empty() {
            return;
        }

        if num.parse::<f64>().is_err() {
            self.tokens.push(Token {
                token_type: TokenType::Error,
                lexeme: num.clone(),
                line: self.line,
            });
        }

        self.tokens.push(Token {
            token_type: TokenType::Number,
            lexeme: num,
            line: self.line,
        });
    }

    pub fn string(&mut self) {
        let mut string = String::new();
        self.advance(); // consume opening quote

        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance(); // consume closing quote
                break;
            } else {
                string.push(c);
                self.advance();
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::String,
            lexeme: string,
            line: self.line,
        });
    }

    pub fn boolean(&mut self) {
        if let Some(c) = self.current_char {
            if c == 't'
                && self.peek(1) == Some('r')
                && self.peek(2) == Some('u')
                && self.peek(3) == Some('e')
            {
                self.tokens.push(Token {
                    token_type: TokenType::Boolean,
                    lexeme: "true".to_string(),
                    line: self.line,
                });
                for _ in 0..4 {
                    self.advance();
                }
            } else if c == 'f'
                && self.peek(1) == Some('a')
                && self.peek(2) == Some('l')
                && self.peek(3) == Some('s')
                && self.peek(4) == Some('e')
            {
                self.tokens.push(Token {
                    token_type: TokenType::Boolean,
                    lexeme: "false".to_string(),
                    line: self.line,
                });
                for _ in 0..5 {
                    self.advance();
                }
            }
        }
    }

    pub fn null(&mut self) {
        if let Some(c) = self.current_char
            && c == 'n'
            && self.peek(1) == Some('u')
            && self.peek(2) == Some('l')
            && self.peek(3) == Some('l')
        {
            self.tokens.push(Token {
                token_type: TokenType::Null,
                lexeme: "null".to_string(),
                line: self.line,
            });
            for _ in 0..4 {
                self.advance();
            }
        }
    }

    pub fn special_char(&mut self) {
        let token_type = match self.current_char {
            Some('{') => TokenType::LBrace,
            Some('}') => TokenType::RBrace,
            Some('[') => TokenType::LBracket,
            Some(']') => TokenType::RBracket,
            Some(':') => TokenType::Colon,
            Some(',') => TokenType::Comma,
            _ => return,
        };

        self.tokens.push(Token {
            token_type,
            lexeme: self.current_char.unwrap().to_string(),
            line: self.line,
        });

        self.advance();
    }

    pub fn tokenize(&mut self) {
        while let Some(c) = self.current_char {
            match c {
                c if c.is_whitespace() => {
                    self.skip_whitespace();
                }
                c if c == '/' && self.peek(1) == Some('/') => {
                    self.skip_single_comment();
                }
                c if c == '/' && self.peek(1) == Some('*') => {
                    self.skip_multi_comment();
                }
                c if c.is_ascii_digit() => {
                    self.number();
                }
                c if c == 't' || c == 'f' => {
                    self.boolean();
                }
                'n' => {
                    self.null();
                }
                '"' => {
                    self.string();
                }
                _ => {
                    self.special_char();
                }
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            line: self.line,
        });
    }
}
