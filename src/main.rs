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

trait Visitor {
    fn visit_binary(&mut self);
    fn visit_literal(&mut self);
}

trait Expr {
    fn new(token: Token, left: Option<&dyn Expr>, right: Option<&dyn Expr>) -> Self where Self: Sized;
    fn accept(&self, visitor: &mut AstPrinter) -> String;
    left: Option<&dyn Expr>;
    rigth: Option<&dyn Expr>;
    value: Option<u32>;
}

impl dyn Expr {}

struct Binary {
    token: Token,
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn new(token: Token, left: Option<&dyn Expr>, right: Option<&dyn Expr>) -> Self {
        Binary {
            token: token,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn accept(&self, visitor: &mut AstPrinter) -> String {
        match visitor.visit_binary(self) {
            Some(s) => s,
            None => String::new(),
        }
    }
}

struct Grouping {
    expr: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn new(expr: &dyn Expr) -> Self {
        Grouping {
            expr: Box::new(expr),
        }
    }
}

struct Literal {
    value: u32,
}

impl Expr for Literal {
    fn new(value: u32) -> Self {
        Literal {
            value: value,
        }
    }

    fn accept(&self, visitor: &mut AstPrinter) -> String {
        visitor.visit_literal(self)
    }
}

struct AstPrinter {}

impl AstPrinter {
    fn print(&self, expr: &dyn Expr) -> String {
        format!("{}", expr.accept(&mut self))
    }

    fn visit_binary(&mut self, expr: &dyn Expr) {
        let mut exprs = vec![&expr.left, &expr.right];
        self.parenthesized(&expr.operator.lexeme, exprs);
    }

    fn visit_group(&mut self, expr: &dyn Expr) {
        self.parenthesized("group", &expr.expr);
    }

    fn visit_literal(&mut self, expr: &dyn Expr) -> String {
        if expr.value == None {
            return "nil".to_string();
        }
        return expr.value.to_string();
    }

    fn visit_unary(&mut self, expr: &dyn Expr) {
        self.parenthesized(&expr.operator.lexeme, &expr.right);
    }

    fn parenthesized(&mut self, name: &str, exprs: Vec<&dyn Expr>) -> String {
        let mut builder: Vec<String> = Vec::new();

        builder.push("(".to_string());
        builder.push(name.to_string());
        for expr in exprs {
            builder.push(" ".to_string());
            builder.push(expr.accept(self));
        }
        builder.push(")".to_string());

        format!("{}", builder.join(""))
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
    // let mut scanner = Scanner::new(data);
    // let tokens = scanner.scan_tokens();
    // let parser = Parser::new(tokens);
    let expression = Binary::new(
        Token::new(TokenType::Plus, "+".to_string(), "+".to_string(), 1),
        Some(&Literal { value: 5 }),
        Some(&Literal { value: 5 }),
    );
    let ast_printer = AstPrinter {};

    println!("{}", ast_printer.print(&expression));
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
