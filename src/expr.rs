use crate::token::Token;
use crate::interpreter::Interpreter;

#[derive(Clone)]
pub struct Binary {
    pub token: Token,
    pub values: Vec<Box<Expr>>,
}

impl Binary {
    pub fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_binary_expr(&Binary {
            token: self.token.clone(),
            values: self.values.clone(),
        })
    }
}

#[derive(Clone)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

impl Grouping {
    pub fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_grouping_expr(&Grouping {
            expr: self.expr.clone(),
        })
    }
}

#[derive(Clone)]
pub struct Literal {
    pub value: u32,
}

impl Literal {
    pub fn new(value: u32) -> Literal {
        Literal {
            value: value,
        }
    }

    pub fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_literal_expr(&Literal {
            value: self.value.clone(),
        })
    }
}

#[derive(Clone)]
pub struct Space {
    pub value: String,
}

impl Space {
    pub fn new(value: String) -> Space {
        Space {
            value: value,
        }
    }

    pub fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_space_expr(&Space {
            value: self.value.clone(),
        })
    }
}

#[derive(Clone)]
pub struct Expr {
    pub grouping: Option<Grouping>,
    pub binary: Option<Binary>,
    pub literal: Option<Literal>,
    pub space: Option<Space>,
}

impl Expr {
    pub fn accept(&self, interpreter: Interpreter) -> String {
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