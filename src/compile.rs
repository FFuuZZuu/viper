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

    let output = "tests/test.S".to_string();
    println!("Input: {}", input.clone());
    println!("Output: {}", output.clone());

    println!("Running Lexer:");
    let mut lexer = lexer::Lexer::new(input, filepath.clone());
    let tokens;
    match lexer.tokenise() {
        Ok(x) => tokens = x,
        Err(x) => {
            println!("{}", x);
            exit(1);
        }
    }
    for token in tokens.clone() {
        println!("{}", token);
    }

    println!("Running Parser:");
    let mut parser = parser::Parser::new(tokens, filepath.clone());
    let ast;
    match parser.parse() {
        Ok(x) => ast = x,
        Err(x) => {
            println!("{}", x);
            exit(1);
        }
    }
    println!("{}", ast.clone());

    println!("Running generator:");
    let mut generator = generator::Generator::new(ast, output);
    generator.generate_code_block();
    println!("Done!");
}
