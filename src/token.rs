use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    pub r#type: TokenType,
    // lexeme: String,
    pub literal: String,
    // line: u32,
}

impl Token {
    pub fn new(type_: TokenType, _lexeme: String, literal: String, _line: u32) -> Token {
        Token {
            r#type: type_,
            // lexeme: lexeme,
            literal: literal,
            // line: line,
        }
    }
}