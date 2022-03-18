use std::env;
use std::process;
use std::io::{self, Write};
use std::io::stdin;
use std::fmt::{Display, Formatter, Result};

// struct Parser {
//     tokens: Vec<Token>,
//     current: u32,
// }

// impl Parser {
//     fn new(tokens: Vec<Token>) -> Parser {
//         Parser {
//             tokens: tokens,
//             current: 0,
//         }
//     }
// }

#[derive(Clone)]
enum TokenType {
    LeftParen,
    RightParen,
    Plus,
    Number,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
       match *self {
           TokenType::LeftParen => write!(f, "{}", "("),
           TokenType::RightParen => write!(f, "{}", ")"),
           TokenType::Plus => write!(f, "{}", "+"),
           TokenType::Number => write!(f, "{}", "<number>"),
           TokenType::EOF => write!(f, "{}", "<EOF>"),
       }
    }
}

#[derive(Clone)]
struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: String,
    line: u32,
}

impl Token {
    fn new(type_: TokenType, lexeme: String, literal: String, line: u32) -> Token {
        Token {
            r#type: type_,
            lexeme: lexeme,
            literal: literal,
            line: line,
        }
    }

    fn to_string(&self) -> String {
        format!("type: {} | lexeme: {} | literal: {} | line: {}", self.r#type, self.lexeme, self.literal, self.line)
    }
}

struct Expr {
    binary: Option<Binary>,
    literal: Option<Literal>,
}

impl Expr {
    fn new(binary: Option<Binary>, literal: Option<Literal>) -> Expr {
        Expr {
            binary: binary,
            literal: literal,
        }
    }

    fn to_string(&self) -> String {
        match &self.binary {
            Some(binary) => format!("binary: {}", binary.to_string()),
            None => match &self.literal {
                Some(literal) => format!("literal: {}", literal.to_string()),
                None => format!(""),
            },
        }
    }
}

struct Binary {
    token: Token,
    left: Box<Expr>,
    right: Box<Expr>,
}

impl Binary {
    fn new(token: Token, left: Expr, right: Expr) -> Binary {
        Binary {
            token: token,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn to_string(&self) -> String {
        format!("{}\n{}\n{}", self.token.to_string(), self.left.to_string(), self.right.to_string())
    }
}

struct Literal {
    value: u32,
}

impl Literal {
    fn new(value: u32) -> Literal {
        Literal {
            value: value,
        }
    }

    fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}

struct AstPrinter {}

impl AstPrinter {
    fn print(expression: Expr) -> String {
        format!("{}", expression.to_string())
    }
}

struct Scanner {
    source: String,
    line: u32,
    current: u32,
    start: u32,
    tokens: Vec<Token>,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            line: 1,
            current: 0,
            start: 0,
            tokens: Vec::new(),
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.internal_scan_tokens();
        }

        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), "".to_string(), self.line));

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
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
                    self.error(format!("Unexpected character {}", c));
                }
            },
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().collect::<Vec<char>>()[self.current as usize - 1]
    }

    fn add_token(&mut self, token_type: TokenType, literal: &str) {
        self.internal_add_token(token_type, literal);
    }

    fn internal_add_token(&mut self, token_type: TokenType, literal: &str) {
        let text = &self.source.chars().skip(self.start as usize).take((self.current - self.start).try_into().unwrap()).collect::<String>();
        self.tokens.push(Token::new(token_type, literal.to_string(), text.to_string(), self.line));
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        self.add_token(TokenType::Number, &self.source.chars().skip(self.start as usize).take((self.current - self.start).try_into().unwrap()).collect::<String>());
    }

    // fn peek(&self) -> char {
    //     if self.is_at_end() {
    //         '\0'
    //     } else {
    //         self.source.chars().nth(self.current as usize).unwrap()
    //     }
    // }

    fn error(&self, message: String) {
        println!("[line {}] Error: {}", self.line, message);
        process::exit(1);
    }
}

fn run (data: String) {
    let mut scanner = Scanner::new(data);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token.to_string());
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
    } else if params.len() == 1 && params[0].ends_with(".lisp") {
        run_file(params[0], had_error);
    } else if params.len() == 1 && params[0] == "ast-printer" {
        let a = Literal::new(5);
        let b = Literal::new(5);
        let binary = Binary::new(
            Token::new(TokenType::Plus, "+".to_string(), "+".to_string(), 1),
            Expr::new(None, Some(a)),
            Expr::new(None, Some(b)),
        );
        
        let expression = Expr::new(Some(binary), None);
        
        println!("{}", AstPrinter::print(expression));
    } else {
        run_prompt();
    }
}
