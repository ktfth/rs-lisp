use std::process;

use crate::token_type::TokenType;
use crate::token::Token;

pub struct Scanner {
  pub source: String,
  pub line: u32,
  pub current: u32,
  pub start: u32,
  pub tokens: Vec<Token>,
}

impl Scanner {
  pub fn new(source: String) -> Scanner {
      Scanner {
          source: source,
          line: 1,
          current: 0,
          start: 0,
          tokens: Vec::new(),
      }
  }

  pub fn scan_tokens(&mut self) -> Vec<Token> {
      while !self.is_at_end() {
          self.start = self.current;
          self.internal_scan_tokens();
      }

      self.tokens.push(Token::new(TokenType::EOF, "".to_string(), "".to_string(), self.line));

      self.tokens.clone()
  }

  pub fn is_at_end(&self) -> bool {
      self.current >= self.source.len().try_into().unwrap()
  }

  pub fn internal_scan_tokens(&mut self) {
      let c = self.advance();

      match c {
          '(' => self.add_token(TokenType::LeftParen, "("),
          ')' => self.add_token(TokenType::RightParen, ")"),
          '+' => self.add_token(TokenType::Plus, "+"),
          ' ' => self.add_token(TokenType::Space, " "),
          _ => {
              if self.is_digit(c) {
                  self.number();
              } else if !self.is_at_end() {
                  self.error(format!("Unexpected character {}", c));
              }
          },
      }
  }

  pub fn advance(&mut self) -> char {
      self.current += 1;
      self.source.chars().collect::<Vec<char>>()[self.current as usize - 1]
  }

  pub fn add_token(&mut self, token_type: TokenType, literal: &str) {
      self.internal_add_token(token_type, literal);
  }

  pub fn internal_add_token(&mut self, token_type: TokenType, literal: &str) {
      let text = &self.source.chars().skip(self.start as usize).take((self.current - self.start).try_into().unwrap()).collect::<String>();
      self.tokens.push(Token::new(token_type, literal.to_string(), text.to_string(), self.line));
  }

  pub fn is_digit(&self, c: char) -> bool {
      c >= '0' && c <= '9'
  }

  pub fn number(&mut self) {
      let number = &self.source.chars().skip(self.start as usize).take((self.current - self.start).try_into().unwrap()).collect::<String>();
      self.add_token(TokenType::Number, number);
  }

  pub fn error(&self, message: String) {
      println!("[line {}] Error: {}", self.line, message);
      process::exit(1);
  }
}