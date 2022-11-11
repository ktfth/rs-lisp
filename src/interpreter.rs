use crate::token_type::TokenType;
use crate::expr::{Expr, Grouping, Literal, Space, Binary};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, expression: Expr) {
        let value = self.evaluate(expression);
        println!("{}", self.stringify(value));
    }

    pub fn stringify(&self, value: String) -> String {
        value
    }

    pub fn visit_literal_expr(&self, expr: &Literal) -> String {
        expr.value.to_string()
    }

    pub fn visit_space_expr(&self, expr: &Space) -> String {
        expr.value.to_string()
    }

    pub fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        self.evaluate(*expr.expr.clone())
    }

    pub fn evaluate(&self, expr: Expr) -> String {
        expr.accept(Interpreter {})
    }

    pub fn visit_binary_expr(&self, expr: &Binary) -> String {
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
            TokenType::Minus => {
              let candidates = values.split(" ").collect::<Vec<&str>>();
              let mut minus = candidates[0].parse::<u32>().unwrap();
              for candidate in candidates.iter().skip(1) {
                  minus -= candidate.parse::<u32>().unwrap();
              }
              minus.to_string()
            }
            _ => panic!("Unknown operation."),
        }
    }
}