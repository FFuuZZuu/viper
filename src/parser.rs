use crate::ast::*;
use crate::error::*;
use crate::lexer::*;

pub struct Parser {
    tokens: Vec<Token>,
    tok_idx: usize,
    current_tok: Token,

    // DEBUG
    filepath: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, filepath: String) -> Self {
        Parser {
            tokens: tokens.clone(),
            tok_idx: 0,
            current_tok: tokens[0].clone(),
            filepath: filepath,
        }
    }

    pub fn parse(&mut self) -> Result<Node, Error> {
        self.expr()
    }

    fn expr(&mut self) -> Result<Node, Error> {
        let mut left = self.term()?;

        while vec![TokenKind::PLUS, TokenKind::MINUS].contains(&self.current_tok.kind) {
            let op_tok = self.current_tok.clone();
            self.advance();
            let right = self.term()?;
            left = Node::BinaryExpr {
                token: op_tok,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Node, Error> {
        let mut left = self.factor()?;

        while vec![TokenKind::MUL, TokenKind::DIV].contains(&self.current_tok.kind) {
            let op_tok = self.current_tok.clone();
            self.advance();
            let right = self.factor()?;
            left = Node::BinaryExpr {
                token: op_tok,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // TODO: Error on multiple numbers in a row
    fn factor(&mut self) -> Result<Node, Error> {
        let tok = self.current_tok.clone();
        match self.current_tok.kind {
            TokenKind::INT(..) => {
                self.advance();
                Ok(Node::Primary(tok))
            }
            TokenKind::PLUS | TokenKind::MINUS => {
                self.advance();
                let factor = self.factor();
                Ok(Node::UnaryExpr {
                    token: tok,
                    node: Box::new(factor?),
                })
            }
            TokenKind::LPAREN => {
                self.advance();
                let expr = self.expr();
                if self.current_tok.kind == TokenKind::RPAREN {
                    self.advance();
                    expr
                } else {
                    Err(Error::FoundExpectedError {
                        found: format!("{:?}", self.current_tok.kind),
                        expected: ")".to_string(),
                        filepath: self.current_tok.filepath.clone(),
                        coord: self.current_tok.coord,
                    })
                }
            }
            _ => Err(Error::IllegalSyntaxError {
                found: format!("{:?}", self.current_tok.kind),
                filepath: self.current_tok.filepath.clone(),
                coord: self.current_tok.coord,
            }),
        }
    }

    fn advance(&mut self) {
        self.tok_idx += 1;
        if self.tok_idx < self.tokens.len() {
            self.current_tok = self.tokens[self.tok_idx as usize].clone();
        }
    }
}
