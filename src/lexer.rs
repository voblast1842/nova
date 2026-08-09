#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    EqualEqual,
    OpenParen,
    ClosedParen,
    Number(f64),
    StringLiteral(String),
    Identifier(String),
    VarKeyword,
    ConstKeyword,
    IfKeyword,
    ElseKeyword,
    OpenBrace,
    ClosedBrace,
    Comma,
    FuncKeyword,
    ReturnKeyword,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    NotEqual,
    WhileKeyword,
    ForKeyword,
    Dot,
    Caret,
    And,
    Or,
    Not,
    OpenBracket,
    ClosedBracket,
    Log,
}

pub fn lex(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source_code.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }
            '#' => {
                chars.next();
                while let Some(&comment_ch) = chars.peek() {
                    if comment_ch == '\n' {
                        break;
                    }
                    chars.next();
                }
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::EqualEqual);
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '&' => {
                chars.next();
                if let Some(&'&') = chars.peek() {
                    chars.next();
                    tokens.push(Token::And);
                }
            }
            '~' => {
                chars.next();
                if let Some(&'~') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Not);
                } else if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::NotEqual);
                }
            }
            '|' => {
                chars.next();
                if let Some(&'|') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Or);
                }
            }
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::ClosedParen);
                chars.next();
            }
            '{' => {
                tokens.push(Token::OpenBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::ClosedBrace);
                chars.next();
            }
            '[' => {
                tokens.push(Token::OpenBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::ClosedBracket);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '<' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::LessEqual);
                } else {
                    tokens.push(Token::Less);
                }
            }
            '>' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::GreaterEqual);
                } else {
                    tokens.push(Token::Greater);
                }
            }
            '.' => {
                tokens.push(Token::Dot);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Caret);
                chars.next();
            }
            '0'..='9' => {
                let mut num_str = String::new();

                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() || next_ch == '.' {
                        num_str.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Ok(number_val) = num_str.parse::<f64>() {
                    tokens.push(Token::Number(number_val));
                }
            }
            '"' => {
                chars.next();
                let mut string_val = String::new();

                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '"' {
                        chars.next();
                        break;
                    } else {
                        string_val.push(next_ch);
                        chars.next();
                    }
                }

                tokens.push(Token::StringLiteral(string_val));
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident_str = String::new();

                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        ident_str.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match ident_str.as_str() {
                    "let" => tokens.push(Token::VarKeyword),
                    "letc" => tokens.push(Token::ConstKeyword),
                    "fn" => tokens.push(Token::FuncKeyword),
                    "wh" => tokens.push(Token::WhileKeyword),
                    "log" => tokens.push(Token::Log),
                    "for" => tokens.push(Token::ForKeyword),
                    "ret" => tokens.push(Token::ReturnKeyword),
                    "if" => tokens.push(Token::IfKeyword),
                    "else" => tokens.push(Token::ElseKeyword),

                    _ => tokens.push(Token::Identifier(ident_str)),
                }
            }

            _ => {
                chars.next();
            }
        }
    }

    tokens
}
