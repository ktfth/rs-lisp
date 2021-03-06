use std::env;
use std::process;
use std::io::{self, Write};
use std::io::stdin;

#[derive(Clone, PartialEq)]
enum TokenType {
    LeftParen,
    RightParen,
    Plus,
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
           TokenType::Number => write!(f, "{}", "<number>"),
           TokenType::Space => write!(f, "{}", "<space>"),
           TokenType::EOF => write!(f, "{}", "<EOF>"),
       }
    }
}

#[derive(Clone)]
struct Token {
    r#type: TokenType,
    // lexeme: String,
    literal: String,
    // line: u32,
}

impl Token {
    fn new(type_: TokenType, _lexeme: String, literal: String, _line: u32) -> Token {
        Token {
            r#type: type_,
            // lexeme: lexeme,
            literal: literal,
            // line: line,
        }
    }
}

#[derive(Clone)]
struct Binary {
    token: Token,
    values: Vec<Box<Expr>>,
}

impl Binary {
    fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_binary_expr(&Binary {
            token: self.token.clone(),
            values: self.values.clone(),
        })
    }
}

#[derive(Clone)]
struct Grouping {
    expr: Box<Expr>,
}

impl Grouping {
    fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_grouping_expr(&Grouping {
            expr: self.expr.clone(),
        })
    }
}

#[derive(Clone)]
struct Literal {
    value: u32,
}

impl Literal {
    fn new(value: u32) -> Literal {
        Literal {
            value: value,
        }
    }

    fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_literal_expr(&Literal {
            value: self.value.clone(),
        })
    }
}

#[derive(Clone)]
struct Space {
    value: String,
}

impl Space {
    fn new(value: String) -> Space {
        Space {
            value: value,
        }
    }

    fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_space_expr(&Space {
            value: self.value.clone(),
        })
    }
}

#[derive(Clone)]
struct Expr {
    grouping: Option<Grouping>,
    binary: Option<Binary>,
    literal: Option<Literal>,
    space: Option<Space>,
}

impl Expr {
    fn accept(&self, interpreter: Interpreter) -> String {
        match &self.grouping {
            Some(grouping) => grouping.accept(interpreter),
            None => match &self.binary {
                Some(binary) => binary.accept(interpreter),
                None => match &self.literal {
                    Some(literal) => literal.accept(interpreter),
                    None => match &self.space {
                        Some(space) => space.accept(interpreter),
                        None => "nil".to_string(),
                    },
                },
            }
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        self.factor()
    }

    fn factor(&mut self) -> Expr {
        self.unary()
    }

    fn unary(&mut self) -> Expr {
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.r#match(vec![TokenType::Space]) {
            let space = Space::new(self.previous().literal);
            return Expr {
                grouping: None,
                binary: None,
                literal: None,
                space: Some(space),
            }
        }

        if self.r#match(vec![TokenType::Number]) {
            let literal = Literal::new(self.previous().literal.parse::<u32>().unwrap());
            return Expr {
                grouping: None,
                binary: None,
                literal: Some(literal),
                space: None,
            }
        }

        if self.r#match(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            return Expr {
                grouping: Some(Grouping {
                    expr: Box::new(expr),
                }),
                binary: None,
                literal: None,
                space: None,
            }
        }

        if self.r#match(vec![TokenType::Plus]) {
            let operator = self.previous();
            let mut values = vec![];
            self.consume(TokenType::Space, "Expect space after '+'.");
            while self.r#match(vec![TokenType::Number]) {
                let expr = Expr {
                    grouping: None,
                    binary: None,
                    literal: Some(Literal::new(self.previous().literal.parse::<u32>().unwrap())),
                    space: None,
                };
                values.push(Box::new(expr));
            }
            let space = Expr {
                grouping: None,
                binary: None,
                literal: None,
                space: Some(Space::new(" ".to_string())),
            };
            values.push(Box::new(space));
            if self.r#match(vec![TokenType::Space]) {
                while self.r#match(vec![TokenType::Number]) {
                    let expr = Expr {
                        grouping: None,
                        binary: None,
                        literal: Some(Literal::new(self.previous().literal.parse::<u32>().unwrap())),
                        space: None,
                    };
                    values.push(Box::new(expr));
                }
            }
            if self.check(TokenType::LeftParen) {
                let expr = self.expression();
                values.push(Box::new(expr));
            }
            return Expr {
                grouping: None,
                binary: Some(Binary {
                    token: operator,
                    values: values,
                }),
                literal: None,
                space: None,
            }
        }

        panic!("Expect expression.")
    }

    fn r#match(&mut self, types: Vec<TokenType>) -> bool {
        for type_ in types {
            if self.check(type_) {
                self.advance();
                return true
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check(token_type) {
            self.advance();
        } else {
            panic!("{}", message)
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false
        }
        self.peek().r#type == token_type
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

struct Interpreter {}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {}
    }

    fn interpret(&self, expression: Expr) {
        let value = self.evaluate(expression);
        println!("{}", self.stringify(value));
    }

    fn stringify(&self, value: String) -> String {
        value
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        expr.value.to_string()
    }

    fn visit_space_expr(&self, expr: &Space) -> String {
        expr.value.to_string()
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        self.evaluate(*expr.expr.clone())
    }

    fn evaluate(&self, expr: Expr) -> String {
        expr.accept(Interpreter {})
    }

    fn visit_binary_expr(&self, expr: &Binary) -> String {
        let expr_values = &expr.values;
        let mut values = String::new();
        let token = &expr.token;

        for value in expr_values {
            values.push_str(&self.evaluate(*value.clone()));
        }

        match token.r#type {
            TokenType::Plus => {
                let mut sum = 0;
                let candidates = values.split(" ").collect::<Vec<&str>>();
                for candidate in candidates {
                    sum += candidate.parse::<u32>().unwrap();
                }
                sum.to_string()
            }
            _ => panic!("Unknown operation."),
        }
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
        let number = &self.source.chars().skip(self.start as usize).take((self.current - self.start).try_into().unwrap()).collect::<String>();
        self.add_token(TokenType::Number, number);
    }

    fn error(&self, message: String) {
        println!("[line {}] Error: {}", self.line, message);
        process::exit(1);
    }
}

fn run (data: String) {
    let mut scanner = Scanner::new(data);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    let interpreter = Interpreter::new();

    interpreter.interpret(expression);
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
    } else {
        run_prompt();
    }
}
