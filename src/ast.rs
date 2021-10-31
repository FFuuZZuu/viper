use crate::lexer;
use std::fmt;

#[derive(Clone)]
pub enum Node {
    Primary(lexer::Token),
    UnaryExpr {
        token: lexer::Token,
        node: Box<Node>,
    },
    BinaryExpr {
        token: lexer::Token,
        left: Box<Node>,
        right: Box<Node>,
    },
}

// TODO: Better tree printing?
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Node::Primary(x) => write!(f, "{}", x),
            Node::UnaryExpr { token, node } => {
                write!(f, "{}, {}", token, *node)
            }
            Node::BinaryExpr { token, left, right } => {
                write!(f, "({}, {}, {})", *left, token, *right)
            }
        }
    }
}
