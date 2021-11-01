use crate::error;
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

    pub filepath: String,
    pub coord: (u32, u32),
}

impl Token {
    pub fn new(kind: TokenKind, filepath: String, coord: (u32, u32)) -> Self {
        Self {
            kind: kind,
            filepath: filepath,
            coord: coord,
        }
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

    // for error handling
    filepath: String,
    coord: (u32, u32),
}

impl Lexer {
    pub fn new(input: String, filepath: String) -> Self {
        Self {
            input: input,
            current_char: '\0',
            pos: 0,
            tokens: Vec::new(),

            filepath: filepath,
            coord: (1, 1),
        }
    }

    pub fn tokenise(&mut self) -> Result<Vec<Token>, error::Error> {
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
                    return Ok(self.tokens.clone());
                }
                _ => {
                    return Err(error::Error::IllegalSyntaxError {
                        found: self.current_char.to_string(),
                        filepath: self.filepath.clone(),
                        coord: self.coord,
                    })
                }
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
        self.coord.1 += 1;
        if self.current_char == '\n' {
            self.coord.0 += 1;
            self.coord.1 = 1;
        }
        self.get_current_char();
    }

    fn push_token(&mut self, kind: TokenKind) {
        self.tokens
            .push(Token::new(kind, self.filepath.clone(), self.coord));
        self.advance();
    }

    fn num_token(&mut self) -> Token {
        let mut num_str = String::new();

        while self.current_char.is_alphanumeric() {
            num_str.push_str(&self.current_char.to_string());
            self.advance();
        }

        Token::new(
            TokenKind::INT(num_str.parse().expect("Cannot parse non int")),
            self.filepath.clone(),
            self.coord,
        )
    }
}
