use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    current: usize,
    next: usize,
    char: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut s = Self {
            source: input.chars().collect(),
            current: 0,
            next: 1,
            char: '\0',
        };
        s.char = s.source[s.current];

        s
    }

    //shifts; advances current char pointer in the feed by 1...or null-terminate if we've reached end of feed.
    fn read(&mut self) {
        if self.next >= self.source.len() {
            self.char = '\0'
        } else {
            self.char = self.source[self.next]
        }

        self.current = self.next;
        self.next = self.current + 1;
    }
    //skips whitespace...
    fn skip_whitespace(&mut self) {
        let char = self.char;
        while self.char.is_whitespace() {
            self.read();
        }
    }
    fn match_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.char {
            '=' => {
                self.read();
                Token::new(TokenKind::Assign, "=".to_owned())
            },
            '+' => {
                self.read();
                Token::new(TokenKind::Plus, "+".to_owned())
            },
            '-' => {
                self.read();
                Token::new(TokenKind::Minus, "-".to_owned())
            },
            '*' => {
                self.read();
                Token::new(TokenKind::Multiply, "*".to_owned())
            },
            '/' => {
                self.read();
                Token::new(TokenKind::Divide, "/".to_owned())
            },
            _ if self.char.is_alphabetic() => {
                let mut buffer = String::new();
                buffer.push(self.char);

                self.read();

                while self.current < self.source.len() && self.char.is_alphabetic() {
                    buffer.push(self.char);
                    self.read();
                }
                let kind = match buffer.as_str() {
                    "let" => TokenKind::Let,
                    _ => TokenKind::Identifier,
                };

                Token::new(kind, buffer)
            }

            _ if self.char.is_numeric() => {
                let mut buffer = String::new();
                buffer.push(self.char);

                self.read();

                loop {
                    if self.current >= self.source.len() {
                        break;
                    }

                    //support 1_000_000
                    if self.char == '_' {
                        self.read();
                    }

                    //support decimals 1_000_000.00
                    if !self.char.is_numeric() && self.char != '.' {
                        break;
                    }

                    buffer.push(self.char);
                    self.read();
                }
                Token::new(TokenKind::Number, buffer)
            }
            _ => unimplemented!(),
        }
    }
    pub fn peek(&mut self) -> Option<Token> {
        if self.next >= self.source.len() {
            return None;
        }
        let old_current = self.current;
        let old_next = self.next;

        let old_char = self.char;
        self.char = self.source[self.next];

        let token = self.match_token();
        self.current = old_current;
        self.next = old_next;
        self.char = old_char;

        Some(token)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.next >= self.source.len() {
            return None;
        }
        let token = self.match_token();

        Some(token)
    }
}
