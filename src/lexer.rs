use crate::error;
use std::fmt;

// =========================
//  Tokens
// =========================

#[derive(Debug, Clone, PartialEq)]
pub enum KeywordKind {
    RETURN,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    INT(i32),
    DECL_NAME(String),
    KEYWORD(KeywordKind),
    COMMENT(String),
    PLUS,
    MINUS,
    MUL,
    DIV,
    EQUALS,
    SEMICOLON,
    LPAREN,
    RPAREN,
    CURLY_LPAREN,
    CURLY_RPAREN,
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

const WHITESPACE: [char; 4] = [' ', '\t', '\n', '\0'];

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
                'a'..='z' | 'A'..='Z' => {
                    self.check_keyword();
                }
                '+' => self.push_token(TokenKind::PLUS),
                '-' => self.push_token(TokenKind::MINUS),
                '*' => self.push_token(TokenKind::MUL),
                '/' => self.comment(),
                '=' => self.push_token(TokenKind::EQUALS),
                ';' => self.push_token(TokenKind::SEMICOLON),
                '(' => self.push_token(TokenKind::LPAREN),
                ')' => self.push_token(TokenKind::RPAREN),
                '{' => self.push_token(TokenKind::CURLY_LPAREN),
                '}' => self.push_token(TokenKind::CURLY_RPAREN),
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

    fn comment(&mut self) {
        self.advance();
        if self.current_char.clone() == '/' {
            self.advance();
            let mut comment = String::new();
            while self.current_char.clone() != '\n' && self.current_char.clone() != '\0' {
                comment.push(self.current_char.clone());
                self.advance();
            }
            self.push_token(TokenKind::COMMENT(comment));
        } else {
            self.push_token(TokenKind::DIV);
        }
    }

    fn check_keyword(&mut self) {
        let current_word = self.get_current_word();
        match current_word.as_str() {
            "return" => self.push_token(TokenKind::KEYWORD(KeywordKind::RETURN)),
            _ => self.variable_declaration(current_word),
        }
    }

    fn get_current_word(&mut self) -> String {
        let mut ret = String::new();

        while !WHITESPACE.contains(&self.current_char) {
            ret.push(self.current_char.clone());
            self.advance();
        }
        ret
    }

    fn variable_declaration(&mut self, name: String) {
        self.push_token(TokenKind::DECL_NAME(name));
    }
}
