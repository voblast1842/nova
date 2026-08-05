// main.rs

use std::fs;

mod evaluator;
mod lexer;
mod parser;

fn main() {
    let source_code = fs::read_to_string("tests/basic.mpds").expect("Failed to read test script!");

    let tokens = lexer::lex(&source_code);
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    let mut evaluator = evaluator::Evaluator::new();
    evaluator.evaluate(ast);
}
