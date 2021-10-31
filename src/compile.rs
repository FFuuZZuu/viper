use crate::ast;
use crate::generator;
use crate::lexer;
use crate::parser;
use std::fs;

pub fn compile(filepath: String) {
    let input = fs::read_to_string(filepath)
        .expect("Cannot read file")
        .trim_end()
        .to_string();

    let output = "tests/maths.S".to_string();

    let mut lexer = lexer::Lexer::new(input);
    lexer.tokenise();
    for token in lexer.tokens.clone() {
        println!("{}", token);
    }

    let mut parser = parser::Parser::new(lexer.tokens);
    let ast = parser.parse();
    println!("{}", ast);

    let mut generator = generator::Generator::new(ast, output);
    generator.generate();
}
