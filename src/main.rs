use std::fs;
use std::io::{self, Write};

mod evaluator;
mod lexer;
mod parser;

fn main() {
    let filename = get_test_filename();

    let target_path = format!("tests/{}", filename);
    println!("Loading file: {}...", target_path);

    let source_code = fs::read_to_string(&target_path)
        .unwrap_or_else(|_| panic!("Failed to read test script at: {}!", target_path));

    let tokens = lexer::lex(&source_code);
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    let mut evaluator = evaluator::Evaluator::new();
    evaluator.evaluate(ast);
}

fn get_test_filename() -> String {
    print!("Enter the test file name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
