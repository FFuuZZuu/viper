use crate::ast;
use crate::generator;
use crate::lexer;
use crate::parser;
use std::fs;
use std::process::*;

pub fn compile(filepath: String) {
    let input = fs::read_to_string(filepath.clone())
        .expect("Cannot read file")
        .trim_end()
        .to_string();

    let output = "tests/maths.S".to_string();

    let mut lexer = lexer::Lexer::new(input, filepath.clone());
    let tokens;
    match lexer.tokenise() {
        Ok(x) => tokens = x,
        Err(x) => {
            println!("{}", x);
            exit(1);
        }
    }

    let mut parser = parser::Parser::new(tokens, filepath.clone());
    let ast;
    match parser.parse() {
        Ok(x) => ast = x,
        Err(x) => {
            println!("{}", x);
            exit(1);
        }
    }

    let mut generator = generator::Generator::new(ast, output);
    generator.generate();
}
