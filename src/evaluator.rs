use crate::parser::{BinaryOp, Expr, Stmt::{self}, UnaryOp};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Nil,
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
				let elems: Vec<String> =
					arr.iter().map(|v| v.to_string()).collect();

				write!(f, "[{}]", elems.join(", "))
            }
            Value::Function { .. } => write!(f, "<fn>"),
            Value::Nil => write!(f, "nil"),
		}
	}
}

#[derive(Debug)]
struct ReturnControl(Value);

pub struct Evaluator {
    environment: HashMap<String, Value>,
}

impl Evaluator {
	pub fn new() -> Self {
		Evaluator {
			environment: HashMap::new(),
		}
	}

    pub fn evaluate(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            if let Err(ReturnControl(_)) = self.execute_statement(&stmt) {
                panic!("Runtime Error: 'return' statement outside of function scope.");
            }
        }
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> Result<(), ReturnControl> {
        match stmt {
            Stmt::VarDeclaration { name, initializer }
            | Stmt::ConstDeclaration { name, initializer } => {
                let value = self.eval_expr(&initializer);
                self.environment.insert(name.clone(), value);
            }
			Stmt::If {
				condition,
				then_branch,
				else_branch,
			} => {
				let cond_val = self.eval_expr(condition);
				if self.is_truthy(&cond_val) {
					self.execute_block(then_branch)?;
				} else if let Some(else_stmts) = else_branch {
					self.execute_block(else_stmts)?;
				}
			}
			Stmt::While {
				condition,
				body
			 } => {
				let cond_val = self.eval_expr(condition);
				while self.is_truthy(&cond_val) {
					self.execute_block(body)?;
				}
			}
			Stmt::Function {
				name,
				params,
				body
			} => {
				let func = Value::Function {
					params: params.clone(),
					body: body.clone()
				};
				self.environment.insert(name.clone(), func);
			}
			Stmt::Return(value_expr) => {
                let return_val = match value_expr {
                    Some(expr) => self.eval_expr(expr),
                    None => Value::Nil,
                };
                return Err(ReturnControl(return_val));
            }
			Stmt::Expression(expr) => {
                let result = self.eval_expr(expr);
                if result != Value::Nil {
                    println!("=> {}", result);
                }
            }
        }
		Ok(())
    }

	fn execute_block(&mut self, stmts: &[Stmt]) -> Result<(), ReturnControl> {
        for stmt in stmts {
            self.execute_statement(stmt)?;
        }
        Ok(())
    }
fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(val) => Value::Number(*val),
            Expr::StringLiteral(val) => Value::String(val.clone()),
            Expr::Variable(name) => self
                .environment
                .get(name)
                .cloned()
                .unwrap_or_else(|| panic!("Runtime Error: Undefined Variable '{}'", name)),
            Expr::Array(elements) => {
                let evaled_elements = elements.iter().map(|e| self.eval_expr(e)).collect();
                Value::Array(evaled_elements)
            }
            Expr::Unary { op, right } => {
                let right_val = self.eval_expr(right);
                match op {
                    UnaryOp::Negate => match right_val {
                        Value::Number(n) => Value::Number(-n),
                        _ => panic!("Runtime Error: '-' operand must be a number."),
                    },
                    UnaryOp::Not => Value::Number(if self.is_truthy(&right_val) { 0.0 } else { 1.0 }),
                }
            }
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expr(left);
                let right_val = self.eval_expr(right);

                match op {
                    BinaryOp::Add => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                        (Value::String(a), Value::String(b)) => Value::String(a + &b),
                        (Value::String(a), Value::Number(b)) => Value::String(format!("{}{}", a, b)),
                        (Value::Number(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
						(Value::Array(mut a), Value::Array(b)) => {
							a.extend(b);
							Value::Array(a)
						}
                        _ => panic!("Runtime Error: Operands must be numbers or strings for '+'."),
                    },
                    BinaryOp::Subtract => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
                        _ => panic!("Runtime Error: Operands must be numbers for '-'."),
                    },
                    BinaryOp::Multiply => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                        _ => panic!("Runtime Error: Operands must be numbers for '*'."),
                    },
                    BinaryOp::Divide => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => {
                            if b == 0.0 {
                                panic!("Runtime Error: Division by zero.");
                            }
                            Value::Number(a / b)
                        }
                        _ => panic!("Runtime Error: Operands must be numbers for '/'."),
                    },
                    BinaryOp::EqualEqual => Value::Number(if left_val == right_val { 1.0 } else { 0.0 }),
                    BinaryOp::NotEqual => Value::Number(if left_val != right_val { 1.0 } else { 0.0 }),
                    BinaryOp::Less => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(if a < b { 1.0 } else { 0.0 }),
                        _ => panic!("Runtime Error: Operands must be numbers for '<'."),
                    },
                    BinaryOp::Greater => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(if a > b { 1.0 } else { 0.0 }),
                        _ => panic!("Runtime Error: Operands must be numbers for '>'."),
                    },
                    BinaryOp::LessEqual => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(if a <= b { 1.0 } else { 0.0 }),
                        _ => panic!("Runtime Error: Operands must be numbers for '<='."),
                    },
                    BinaryOp::GreaterEqual => match (left_val, right_val) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(if a >= b { 1.0 } else { 0.0 }),
                        _ => panic!("Runtime Error: Operands must be numbers for '>='."),
                    },
                    BinaryOp::And => {
                        let is_true = self.is_truthy(&left_val) && self.is_truthy(&right_val);
                        Value::Number(if is_true { 1.0 } else { 0.0 })
                    }
                    BinaryOp::Or => {
                        let is_true = self.is_truthy(&left_val) || self.is_truthy(&right_val);
                        Value::Number(if is_true { 1.0 } else { 0.0 })
                    }
                }
            }
            Expr::Call { callee, arguments } => {
                let func_val = self
                    .environment
                    .get(callee)
                    .cloned()
                    .unwrap_or_else(|| panic!("Runtime Error: Undefined Function '{}'", callee));

                match func_val {
                    Value::Function { params, body } => {
                        if arguments.len() != params.len() {
                            panic!(
                                "Runtime Error: Function '{}' expected {} arguments, got {}",
                                callee,
                                params.len(),
                                arguments.len()
                            );
                        }

                        // Evaluate argument expressions first
                        let evaled_args: Vec<Value> = arguments.iter().map(|arg| self.eval_expr(arg)).collect();

                        // Save current environment state to restore after invocation
                        let previous_env = self.environment.clone();

                        // Bind parameters to argument values
                        for (param, val) in params.iter().zip(evaled_args) {
                            self.environment.insert(param.clone(), val);
                        }

                        // Execute body and capture return values
                        let mut return_value = Value::Nil;
                        if let Err(ReturnControl(val)) = self.execute_block(&body) {
                            return_value = val;
                        }

                        // Restore pre-invocation environment scope
                        self.environment = previous_env;

                        return_value
                    }
                    _ => panic!("Runtime Error: '{}' is not a callable function.", callee),
                }
            }
        }
    }

	fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Function { .. } => true,
            Value::Nil => false,
        }
    }
}
