use crate::lexer::Token;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Stmt {
    VarDeclaration { name: String, initializer: Expr },
    ConstDeclaration { name: String, initializer: Expr },
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
            _ => {
                let expr = self.parse_expression()?;
                Some(Stmt::Expression(expr))
            }
        }
    }

    pub fn parse_expression(&mut self) -> Option<Expr> {
        self.parse_additive()
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
