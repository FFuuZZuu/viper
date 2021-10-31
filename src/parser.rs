use crate::ast::*;
use crate::lexer::*;

pub struct Parser {
    tokens: Vec<Token>,
    tok_idx: usize,
    current_tok: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.clone(),
            tok_idx: 0,
            current_tok: tokens[0].clone(),
        }
    }

    pub fn parse(&mut self) -> Node {
        self.expr()
    }

    fn expr(&mut self) -> Node {
        let mut left = self.term();

        while vec![TokenKind::PLUS, TokenKind::MINUS].contains(&self.current_tok.kind) {
            let op_tok = self.current_tok.clone();
            self.advance();
            let right = self.term();
            left = Node::BinaryExpr {
                token: op_tok,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn term(&mut self) -> Node {
        let mut left = self.factor();

        while vec![TokenKind::MUL, TokenKind::DIV].contains(&self.current_tok.kind) {
            let op_tok = self.current_tok.clone();
            self.advance();
            let right = self.factor();
            left = Node::BinaryExpr {
                token: op_tok,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn factor(&mut self) -> Node {
        let tok = self.current_tok.clone();
        match self.current_tok.kind {
            TokenKind::INT(..) => {
                self.advance();
                Node::Primary(tok)
            }
            TokenKind::PLUS | TokenKind::MINUS => {
                self.advance();
                let factor = self.factor();
                Node::UnaryExpr {
                    token: tok,
                    node: Box::new(factor),
                }
            }
            TokenKind::LPAREN => {
                self.advance();
                let expr = self.expr();
                if self.current_tok.kind == TokenKind::RPAREN {
                    self.advance();
                    expr
                } else {
                    panic!("expected )");
                }
            }
            _ => {
                panic!("invalid syntax -> cannot make char into factor");
            }
        }
    }

    fn advance(&mut self) {
        self.tok_idx += 1;
        if self.tok_idx < self.tokens.len() {
            self.current_tok = self.tokens[self.tok_idx as usize].clone();
        }
    }
}
