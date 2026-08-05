// src/lexer.rs

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
}

pub fn lex(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source_code.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
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
                chars.next();
                if let Some(&'/') = chars.peek() {
                    chars.next();
                    while let Some(&comment_ch) = chars.peek() {
                        if comment_ch == '\n' {
                            break;
                        }
                        chars.next();
                    }
                } else {
                    tokens.push(Token::Divide);
                }
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
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::ClosedParen);
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
                    "var" => tokens.push(Token::VarKeyword),
                    "con" => tokens.push(Token::ConstKeyword),
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
