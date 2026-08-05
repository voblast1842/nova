// src/evaluator.rs
use crate::parser::{BinaryOp, Expr, Stmt};
use std::collections::HashMap;

pub struct Evaluator {
    environment: HashMap<String, f64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            environment: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.execute_statement(stmt)
        }
    }

    fn execute_statement(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::VarDeclaration { name, initializer }
            | Stmt::ConstDeclaration { name, initializer } => {
                let value = self.eval_expr(&initializer);
                self.environment.insert(name, value);
            }
            Stmt::Expression(expr) => {
                let result = self.eval_expr(&expr);
                println!("=> {}", result);
            }
        }
    }

    fn eval_expr(&self, expr: &Expr) -> f64 {
        match expr {
            Expr::Number(val) => *val,
            Expr::Variable(name) => *self.environment.get(name).unwrap_or_else(|| {
                panic!("Runtime Error: Undefined Variable '{}'", name);
            }),
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expr(left);
                let right_val = self.eval_expr(right);

                match op {
                    BinaryOp::Add => left_val + right_val,
                    BinaryOp::Subtract => left_val - right_val,
                    BinaryOp::Multiply => left_val * right_val,
                    BinaryOp::Divide => {
                        if right_val == 0.0 {
                            panic!("Runtime Error: Division by zero. (undefined)")
                        }
                        left_val / right_val
                    }
                }
            }
        }
    }
}
