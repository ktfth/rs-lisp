use std::env;
use std::process;
use std::io::{self, Write};
use std::io::stdin;

struct Token {}

impl Token {}

struct TokenType {}

impl TokenType {}

struct Scanner {}

impl Scanner {
    fn new(&mut self, source) {
        self.source = source;
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.internal_scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "", None, self.line));

        self.tokens;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn internal_scan_tokens(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, "("),
            ')' => self.add_token(TokenType::RightParen, ")"),
            '+' => self.add_token(TokenType::Plus, "+"),
            ' ' => {},
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else {
                    self.add_token(TokenType::Error, "Unknown token");
                }
            },
        }
    }

    fn advance(&mut self) {
        self.current += 1;
        &self.source[self.current];
    }

    fn add_token(&mut self, token_type: TokenType, literal: &str) {
        self.internal_add_token(token_type, literal);
    }

    fn internal_add_token(&mut self, token_type: TokenType, literal: &str) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, literal, text, self.line));
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }
}

fn run (data: String) {
    let scanner = Scanner::new(&data);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn run_file(file: &str, had_error: bool) {
    let data = std::fs::read_to_string(file).unwrap();
    run(data);
    if had_error {
        process::exit(65);
    }
}

fn run_prompt() {
    loop {
        let mut s = String::new();
        
        print!("> ");
        
        io::stdout().flush().unwrap();

        stdin().read_line(&mut s).unwrap();
        
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        } if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        run(s);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let params = args.iter().skip(1).collect::<Vec<_>>();

    let had_error = false;

    if params.len() > 1 {
        println!("Usage: rs-lisp [script]");
        process::exit(1);
    } else if params.len() == 1 {
        run_file(params[0], had_error);
    } else {
        run_prompt();
    }
}
