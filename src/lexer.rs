use std::fmt;

// =========================
//  Tokens
// =========================

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    INT(i32),
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    EOF,
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind: kind }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

// =========================
//  Lexer
// =========================

pub struct Lexer {
    input: String,
    current_char: char,
    pos: usize,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input,
            current_char: '\0',
            pos: 0,
            tokens: Vec::new(),
        }
    }

    pub fn tokenise(&mut self) {
        self.get_current_char();

        loop {
            match self.current_char {
                ' ' | '\t' | '\n' => self.advance(),
                '0'..='9' => {
                    let token = self.num_token();
                    self.tokens.push(token);
                }
                '+' => self.push_token(TokenKind::PLUS),
                '-' => self.push_token(TokenKind::MINUS),
                '*' => self.push_token(TokenKind::MUL),
                '/' => self.push_token(TokenKind::DIV),
                '(' => self.push_token(TokenKind::LPAREN),
                ')' => self.push_token(TokenKind::RPAREN),
                '\0' => {
                    self.push_token(TokenKind::EOF);
                    break;
                }
                _ => panic!("Illegal Char"), // TODO: Proper error handling
            }
        }
    }

    fn get_current_char(&mut self) {
        self.current_char = if self.pos < self.input.len() {
            self.input.as_bytes()[self.pos] as char
        } else {
            '\0'
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.get_current_char();
    }

    fn push_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token::new(kind));
        self.advance();
    }

    fn num_token(&mut self) -> Token {
        let mut num_str = String::new();

        while self.current_char.is_alphanumeric() {
            num_str.push_str(&self.current_char.to_string());
            self.advance();
        }

        Token::new(TokenKind::INT(
            num_str.parse().expect("Cannot parse non int"),
        ))
    }
}
