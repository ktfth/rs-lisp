use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    pub r#type: TokenType,
    // lexeme: String,
    pub literal: String,
    // line: u32,
}

// Implement formatter for Token
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.literal)
    }
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
