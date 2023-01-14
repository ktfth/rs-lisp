use std::env;
use std::io::stdin;
use std::io::{self, Write};
use std::process;

mod expr;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod token_type;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

fn run(data: String) {
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

fn run_evaluation(data: &str, had_error: bool) {
    run(data.to_string());
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
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        run(s);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let params = args.iter().skip(1).collect::<Vec<_>>();

    let had_error = false;

    if params.len() == 1 && params[0] == "help" {
        println!("Usage: rs-lisp [script]");
        process::exit(1);
    } else if params.len() == 1 && params[0].ends_with(".lisp") {
        run_file(params[0], had_error);
    } else if params.len() == 2 && params[0] == "evaluate" {
        run_evaluation(params[1], had_error);
    } else {
        run_prompt();
    }
}
