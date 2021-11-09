use crate::ast::*;
use crate::lexer::*;
use std::fs;
use std::io::Write;

pub struct Generator {
    ast: Node,
    file: fs::File,
}

// TODO: Generator error handling
impl Generator {
    pub fn new(ast: Node, output: String) -> Self {
        Self {
            ast: ast,
            file: fs::File::create(output).expect("fail"),
        }
    }

    pub fn generate_code_block(&mut self) {
        self.emit(".text");
        self.emit(".globl main");
        self.emit("main:");
        //self.emit("    push %rbp");
        //self.emit("    mov %rbp, %rsp");

        self.generate_node(self.ast.clone());
    }

    fn generate_node(&mut self, node: Node) {
        match node {
            Node::Primary(x) => self.generate_primary_expression(x),
            Node::UnaryExpr { .. } => self.generate_unary_expression(node),
            Node::BinaryExpr { .. } => self.generate_binary_expression(node),
            Node::CompoundExpr { nodes } => self.generate_compound_expression(nodes),
        }
    }

    fn generate_compound_expression(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            self.generate_node(node);
        }
    }

    fn generate_primary_expression(&mut self, token: Token) {
        match token.kind {
            TokenKind::INT(x) => self.emit(format!("    mov ${}, %rax", x).as_str()),
            TokenKind::COMMENT(x) => self.emit(format!("    #{}", x).as_str()),
            _ => panic!("unimplemented primary expression"),
        }
    }

    fn generate_unary_expression(&mut self, node: Node) {
        match node {
            Node::UnaryExpr { token, node } => {
                self.generate_node(*node);

                match token.kind {
                    TokenKind::KEYWORD(KeywordKind::RETURN) => self.emit("    ret"),
                    TokenKind::MINUS => {
                        self.emit("    mov %rax, %rbx");
                        self.emit("    mov $0, %rax");
                        self.emit("    sub %rbx, %rax");
                    }
                    TokenKind::PLUS => return,
                    _ => panic!("unimplemented unary operator"),
                }
            }
            _ => panic!("non unary expr passed to gen unary expr"),
        }
    }

    fn generate_binary_expression(&mut self, node: Node) {
        match node {
            Node::BinaryExpr { token, left, right } => match token.kind {
                TokenKind::PLUS | TokenKind::MINUS | TokenKind::MUL | TokenKind::DIV => {
                    self.generate_maths_expr(token, *left, *right)
                }
                TokenKind::EQUALS => self.generate_declaration(token, *left, *right),
                _ => panic!("unimplemented binary operator"),
            },
            _ => panic!("non binary node passed to generate_binary_expression"),
        }
    }

    fn generate_maths_expr(&mut self, token: Token, left: Node, right: Node) {
        self.generate_node(right);
        self.emit("    push %rax");
        self.generate_node(left);
        self.emit("    pop %rdi");

        match token.kind {
            TokenKind::PLUS => self.emit("    add %rdi, %rax"),
            TokenKind::MINUS => self.emit("    sub %rdi, %rax"),
            TokenKind::MUL => self.emit("    imul %rdi, %rax"),
            TokenKind::DIV => {
                self.emit("    cdq");
                self.emit("    idiv %rdi");
            }
            _ => panic!("unimplemented binary operator"),
        }
    }

    // TODO: A large portion of the generator and parser need to be refactored for greater
    // expandability, such as proper typing of declarations
    fn generate_declaration(&mut self, token: Token, left: Node, right: Node) {
        self.generate_node(right);
        self.emit("    lea -4(%rbp), %rax");
    }

    // TODO: replace expect with "?"
    fn emit(&mut self, text: &str) {
        self.file
            .write_all(format!("{}\n", text).as_bytes())
            .expect("could not write to file");
    }
}
