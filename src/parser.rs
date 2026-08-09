use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum UnaryOp {
	Negate,
	Not,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum BinaryOp {
	Add,
	Subtract,
	Multiply,
	Divide,
	EqualEqual,
	NotEqual,
	Less,
	Greater,
	LessEqual,
	GreaterEqual,
	And,
	Or,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Expr {
	Number(f64),
	StringLiteral(String),
	Variable(String),
	Array(Vec<Expr>),
	Unary {
		op: UnaryOp,
		right: Box<Expr>,
	},
	Binary {
		left: Box<Expr>,
		op: BinaryOp,
		right: Box<Expr>,
	},
	Call {
		callee: String,
		arguments: Vec<Expr>,
	},
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Stmt {
	VarDeclaration {
		name: String,
		initializer: Expr,
	},
	ConstDeclaration {
		name: String,
		initializer: Expr,
	},
	If {
		condition: Expr,
		then_branch: Vec<Stmt>,
		else_branch: Option<Vec<Stmt>>,
	},
	While {
		condition: Expr,
		body: Vec<Stmt>,
	},
	Function {
		name: String,
		params: Vec<String>,
		body: Vec<Stmt>,
	},
	Return(Option<Expr>),
	Expression(Expr),
}

pub struct Parser {
	tokens: Vec<Token>,
	current: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser { tokens, current: 0 }
	}

	fn peek(&mut self) -> Option<&Token> {
		let token = self.tokens.get(self.current);
		token
	}

	fn advance(&mut self) -> Option<&Token> {
		let token = self.tokens.get(self.current);
		self.current += 1;
		token
	}

	pub fn parse(&mut self) -> Vec<Stmt> {
		let mut statements = Vec::new();
		while self.current < self.tokens.len() {
			if let Some(stmt) = self.parse_statement() {
				statements.push(stmt);
			}
		}
		statements
	}

	fn parse_statement(&mut self) -> Option<Stmt> {
		match self.peek()? {
			Token::VarKeyword => {
				self.advance();

				let name = match self.advance()? {
					Token::Identifier(n) => n.clone(),
					_ => panic!("Expected variable name after 'var'."),
				};

				match self.advance()? {
					Token::Equal => {}
					_ => panic!("Expected '=' after variable name."),
				};

				let initializer = self.parse_expression()?;
				Some(Stmt::VarDeclaration { name, initializer })
			}
			Token::ConstKeyword => {
				self.advance();

				let name = match self.advance()? {
					Token::Identifier(n) => n.clone(),
					_ => panic!("Expected constant name after 'con'."),
				};

				match self.advance()? {
					Token::Equal => {}
					_ => panic!("Expected '=' after constant name."),
				};

				let initializer = self.parse_expression()?;
				Some(Stmt::ConstDeclaration { name, initializer })
			}
			Token::IfKeyword => {
				self.advance();

				let condition = self.parse_expression()?;
				let then_branch = self.parse_block()?;
				let else_branch = if matches!(self.peek(), Some(Token::ElseKeyword)) {
					self.advance();
					Some(self.parse_block()?)
				} else {
					None
				};
				Some(Stmt::If {
					condition,
					then_branch,
					else_branch,
				})
			}
			Token::WhileKeyword => {
				self.advance();
				let condition = self.parse_expression()?;
				let body = self.parse_block()?;
				Some(Stmt::While {condition, body})
			}
			Token::FuncKeyword => {
				self.advance();
				let name = match self.advance()? {
					Token::Identifier(n) => n.clone(),
					_ => panic!("Expected function name after 'fn'."),
				};

				match self.advance()? {
					Token::OpenParen => {}
					_ => panic!("Expected '(' after the function name.")
				};

				let mut params = Vec::new();
				if !matches!(self.peek(), Some(Token::ClosedParen)) {
					loop {
						if let Token::Identifier(p) = self.advance()? {
							params.push(p.clone());
						}
						if matches!(self.peek(), Some(Token::Comma)) {
							self.advance();
						} else {
							break;
						}
					}
				}

				match self.advance()? {
					Token::ClosedParen => {}
					_ => panic!("Expected ')' after parameters."),
				};

				let body = self.parse_block()?;
				Some(Stmt::Function { name, params, body })
			}
			Token::ReturnKeyword => {
				self.advance();
				let value = if matches!(self.peek(), Some(Token::ClosedBrace) | None) {
					None
				} else {
					Some(self.parse_expression()?)
				};
				Some(Stmt::Return(value))
			}
			_ => {
				let expr = self.parse_expression()?;
				Some(Stmt::Expression(expr))
			}
		}
	}

	pub fn parse_expression(&mut self) -> Option<Expr> {
		self.parse_logical_or()
	}

	fn parse_logical_or(&mut self) -> Option<Expr> {
		let mut left = self.parse_logical_and()?;
		while matches!(self.peek(), Some(Token::Or)) {
			self.advance();
			let right = self.parse_logical_and()?;
			left = Expr::Binary { left: Box::new(left), op: BinaryOp::Or, right: Box::new(right) };
		}
		Some(left)
	}

	fn parse_logical_and(&mut self) -> Option<Expr> {
		let mut left = self.parse_equality()?;
		while matches!(self.peek(), Some(Token::And)) {
			self.advance();
			let right = self.parse_equality()?;
			left = Expr::Binary { left: Box:: new(left), op: BinaryOp::And, right: Box::new(right) };
		}
		Some(left)
	}

	fn parse_equality(&mut self) -> Option<Expr> {
		let mut left = self.parse_comparison()?;
		while let Some(token) = self.peek() {
			let op = match token {
				Token::EqualEqual => BinaryOp::EqualEqual,
				Token::NotEqual => BinaryOp::NotEqual,
				_ => break,
			};
			self.advance();
			let right = self.parse_comparison()?;
			left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) };
		}
		Some(left)
	}

	fn parse_comparison(&mut self) -> Option<Expr> {
		let mut left = self.parse_additive()?;
		while let Some(token) = self.peek() {
			let op = match token {
				Token::Less => BinaryOp::Less,
				Token::Greater => BinaryOp::Greater,
				Token::LessEqual => BinaryOp::LessEqual,
				Token::GreaterEqual => BinaryOp::GreaterEqual,
				_ => break,
			};
			self.advance();
			let right = self.parse_additive()?;
			left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) };
		}
		Some(left)
	}

	fn parse_block(&mut self) -> Option<Vec<Stmt>> {
		match self.advance()? {
			Token::OpenBrace => {}
			_ => panic!("Expected '{{' to start block.")
		};

		let mut statements = Vec::new();
		while !matches!(self.peek(), Some(Token::ClosedBrace) | None) {
			if let Some(stmt) = self.parse_statement() {
				statements.push(stmt);
			}
		}

		match self.advance()? {
			Token::ClosedBrace => {}
			_ => panic!("Expected '}}' after block."),
		};

		Some(statements)
	}

	fn parse_additive(&mut self) -> Option<Expr> {
		let mut left = self.parse_multiplicative()?;

		while let Some(token) = self.peek() {
			match token {
				Token::Plus => {
					self.advance();
					let right = self.parse_multiplicative()?;
					left = Expr::Binary {
						left: Box::new(left),
						op: BinaryOp::Add,
						right: Box::new(right),
					};
				}
				Token::Minus => {
					self.advance();
					let right = self.parse_multiplicative()?;
					left = Expr::Binary {
						left: Box::new(left),
						op: BinaryOp::Subtract,
						right: Box::new(right),
					};
				}
				_ => break,
			}
		}

		Some(left)
	}

	fn parse_multiplicative(&mut self) -> Option<Expr> {
		let mut left = self.parse_primary()?;

		while let Some(token) = self.peek() {
			match token {
				Token::Multiply => {
					self.advance();
					let right = self.parse_primary()?;
					left = Expr::Binary {
						left: Box::new(left),
						op: BinaryOp::Multiply,
						right: Box::new(right),
					};
				}
				Token::Divide => {
					self.advance();
					let right = self.parse_primary()?;
					left = Expr::Binary {
						left: Box::new(left),
						op: BinaryOp::Divide,
						right: Box::new(right),
					};
				}
				_ => break,
			}
		}

		Some(left)
	}

	fn parse_primary(&mut self) -> Option<Expr> {
		match self.advance()? {
			Token::Number(val) => Some(Expr::Number(*val)),
			Token::Identifier(name) => Some(Expr::Variable(name.clone())),
			Token::OpenParen => {
				let expr = self.parse_expression()?;
				match self.advance() {
					Some(Token::ClosedParen) => Some(expr),
					_ => panic!("Expected closing parenthesis ')'"),
				}
			}
			token => panic!("Unexpected token in expression: {:?}", token),
		}
	}
}
