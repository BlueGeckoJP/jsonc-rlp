#[derive(Debug)]
pub enum TokenType {
    // Special characters
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,

    // Value types
    String,
    Number,
    Boolean,
    Null,

    // EOF
    Eof,

    // Error
    Error,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}
