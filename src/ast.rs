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
    CompoundExpr {
        nodes: Vec<Node>,
    },
}

// TODO: Better tree printing?
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Node::print_tree(self.clone(), f)
    }
}

static mut indentation: usize = 0;
static mut mask: Vec<bool> = Vec::new();
impl Node {
    pub fn print_tree(node: Node, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            if mask.len() <= indentation {
                mask.resize(32, false);
            }
        }
        match node {
            Node::Primary(x) => {
                return Node::indented_print(format!("{}", x), f);
            }
            Node::UnaryExpr { token, node } => unsafe {
                Node::indented_print(format!("Unary: {}", token), f)?;
                indentation += 1;
                Node::print_tree(*node, f)?;
                indentation -= 1;
                Ok(())
            },
            Node::BinaryExpr { token, left, right } => unsafe {
                Node::indented_print(format!("Binary: {}", token), f)?;
                let binary_indent = indentation;
                indentation += 1;

                mask[binary_indent] = true;
                Node::print_tree(*left, f)?;
                mask[binary_indent] = false;
                Node::print_tree(*right, f)?;

                indentation -= 1;
                Ok(())
            },
            Node::CompoundExpr { nodes } => unsafe {
                Node::indented_print("Compound:".to_string(), f)?;

                let compound_indent = indentation;
                mask[compound_indent] = true;

                let last = nodes.clone().len();
                let mut i = 1;
                for node in nodes {
                    if i == last {
                        mask[compound_indent] = false;
                    }

                    indentation += 1;
                    Node::print_tree(node, f)?;
                    indentation -= 1;
                    i += 1;
                }
                mask[compound_indent] = false;
                return Ok(());
            },
        }
    }

    fn indented_print(data: String, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            if indentation != 0 {
                for i in 0..indentation - 1 {
                    if mask.len() <= i {
                        mask.resize(32, false);
                    }

                    if mask[i] {
                        write!(f, "|    ")?;
                    } else {
                        write!(f, "     ")?;
                    }
                }
                write!(f, "|--> ")?;
            }
            write!(f, "{}\n", data)
        }
    }
}
