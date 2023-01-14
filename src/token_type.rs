#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Star,
    Slash,
    Number,
    Space,
    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TokenType::LeftParen => write!(f, "{}", "("),
            TokenType::RightParen => write!(f, "{}", ")"),
            TokenType::Plus => write!(f, "{}", "+"),
            TokenType::Minus => write!(f, "{}", "-"),
            TokenType::Star => write!(f, "{}", "*"),
            TokenType::Slash => write!(f, "{}", "/"),
            TokenType::Number => write!(f, "{}", "<number>"),
            TokenType::Space => write!(f, "{}", "<space>"),
            TokenType::EOF => write!(f, "{}", "<EOF>"),
        }
    }
}
