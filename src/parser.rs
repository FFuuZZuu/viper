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
        self.code_block()
    }

    fn code_block(&mut self) -> Result<Node, Error> {
        self.scope()
    }

    fn scope(&mut self) -> Result<Node, Error> {
        let mut nodes: Vec<Node> = Vec::new();
        if self.current_tok.kind == TokenKind::CURLY_LPAREN {
            self.advance();
            while self.current_tok.kind != TokenKind::CURLY_RPAREN {
                nodes.push(match self.current_tok.kind {
                    TokenKind::CURLY_LPAREN => self.scope()?,
                    TokenKind::INT(..) => self.expr()?,
                    _ => self.stmt()?,
                });
                self.advance();
            }
            Ok(Node::CompoundExpr { nodes: nodes })
        } else {
            return Err(Error::FoundExpectedError {
                found: format!("{:?}", self.current_tok.kind),
                expected: format!("{:?}", TokenKind::CURLY_LPAREN),
                coord: self.current_tok.coord,
                filepath: self.filepath.clone(),
            });
        }
    }

    fn stmt(&mut self) -> Result<Node, Error> {
        match self.current_tok.kind.clone() {
            TokenKind::DECL_NAME(..) => return self.decl(),
            TokenKind::KEYWORD(KeywordKind::RETURN) => return self.ret(),
            _ => {
                return Err(Error::IllegalSyntaxError {
                    found: format!("{:?}", self.current_tok.kind),
                    coord: self.current_tok.coord,
                    filepath: self.filepath.clone(),
                })
            }
        }
    }

    fn decl(&mut self) -> Result<Node, Error> {
        let left = Node::Primary(self.current_tok.clone());
        match self.current_tok.kind.clone() {
            TokenKind::DECL_NAME(x) => {
                println!("{}", x);
                self.advance();
                if self.current_tok.kind == TokenKind::EQUALS {
                    let op_tok = self.current_tok.clone();
                    self.advance();
                    let right = self.expr()?;
                    return Ok(Node::BinaryExpr {
                        token: op_tok,
                        left: Box::new(left),
                        right: Box::new(right),
                    });
                } else {
                    return Err(Error::FoundExpectedError {
                        found: format!("{:?}", self.current_tok.kind.clone()),
                        expected: format!("{:?}", TokenKind::EQUALS),
                        coord: self.current_tok.coord,
                        filepath: self.filepath.clone(),
                    });
                }
            }
            _ => panic!("decl() called incorrectly"),
        }
    }

    fn ret(&mut self) -> Result<Node, Error> {
        let ret = self.current_tok.clone();
        self.advance();
        let expr = self.expr()?;
        Ok(Node::UnaryExpr {
            token: ret,
            node: Box::new(expr),
        })
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
                return Ok(Node::Primary(tok));
            }
            TokenKind::PLUS | TokenKind::MINUS => {
                self.advance();
                let factor = self.factor();
                return Ok(Node::UnaryExpr {
                    token: tok,
                    node: Box::new(factor?),
                });
            }
            TokenKind::LPAREN => {
                self.advance();
                let expr = self.expr();
                if self.current_tok.kind == TokenKind::RPAREN {
                    self.advance();
                    return expr;
                } else {
                    return Err(Error::FoundExpectedError {
                        found: format!("{:?}", self.current_tok.kind),
                        expected: ")".to_string(),
                        filepath: self.current_tok.filepath.clone(),
                        coord: self.current_tok.coord,
                    });
                }
            }
            _ => {
                return Err(Error::IllegalSyntaxError {
                    found: format!("{:?}", self.current_tok.kind),
                    filepath: self.current_tok.filepath.clone(),
                    coord: self.current_tok.coord,
                })
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
