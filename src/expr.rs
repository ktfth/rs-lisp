use crate::interpreter::Interpreter;
use crate::token::Token;

#[derive(Clone, Debug)]
pub struct Binary {
    pub token: Token,
    pub values: Vec<Box<Expr>>,
}

// Implement formatter for Binary
impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut values = String::new();
        for value in &self.values {
            values.push_str(&format!(" {}", value));
        }
        write!(f, "{}", values)
    }
}

impl Binary {
    pub fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_binary_expr(&Binary {
            token: self.token.clone(),
            values: self.values.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

// Implement formatter for Grouping
impl std::fmt::Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self.expr)
    }
}

impl Grouping {
    pub fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_grouping_expr(&Grouping {
            expr: self.expr.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub value: u32,
}

// Implement formatter for Literal
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Literal {
    pub fn new(value: u32) -> Literal {
        Literal { value: value }
    }

    pub fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_literal_expr(&Literal {
            value: self.value.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Space {
    pub value: String,
}

// Implement formatter for Space
impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Space {
    pub fn new(value: String) -> Space {
        Space { value: value }
    }

    pub fn accept(&self, interpreter: Interpreter) -> String {
        interpreter.visit_space_expr(&Space {
            value: self.value.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub grouping: Option<Grouping>,
    pub binary: Option<Binary>,
    pub literal: Option<Literal>,
    pub space: Option<Space>,
}

// Implement formatter for Expr
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.grouping {
            Some(grouping) => write!(f, "{}", grouping),
            None => match &self.binary {
                Some(binary) => write!(f, "{}", binary),
                None => match &self.literal {
                    Some(literal) => write!(f, "{}", literal),
                    None => match &self.space {
                        Some(space) => write!(f, "{}", space),
                        None => write!(f, "nil"),
                    },
                },
            },
        }
    }
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
            },
        }
    }
}
