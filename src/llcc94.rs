use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SyntaxKind {
    NumberToken,
    WhiteSpaceToken,
    PlusToken,
    MinusToken,
    StarToken,
    OpenParenthesisToken,
    CloseParenthesisToken,
    PipeToken,
    ForwardSlashToken,
    BackwardSlashToken,
    OpenCurlyBraceToken,
    CloseCurlyBraceToken,
    EqualsToken,

    BadToken,
    EOFToken,
}
impl Debug for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SyntaxKind::BadToken => write!(f, "BadToken"),
            SyntaxKind::CloseCurlyBraceToken => write!(f, "CloseCurlyBraceToken"),
            SyntaxKind::NumberToken => write!(f, "NumberToken"),
            SyntaxKind::CloseParenthesisToken => write!(f, "CloseParenthesisToken"),
            //SyntaxKind::CommaToken => write!(f, "CommaToken"),
            //SyntaxKind::DivideToken => write!(f, "DivideToken"),
            SyntaxKind::EOFToken => write!(f, "EOFToken"),
            SyntaxKind::EqualsToken => write!(f, "EqualsToken"),
            SyntaxKind::MinusToken => write!(f, "MinusToken"),
            SyntaxKind::StarToken => write!(f, "StarToken"),
            SyntaxKind::OpenCurlyBraceToken => write!(f, "OpenCurlyBraceToken"),
            SyntaxKind::OpenParenthesisToken => write!(f, "OpenParenthesisToken"),
            SyntaxKind::WhiteSpaceToken => write!(f, "WhiteSpaceToken"),
            SyntaxKind::PlusToken => write!(f, "PlusToken"),
            SyntaxKind::ForwardSlashToken => write!(f, "ForwardSlashToken"),
            SyntaxKind::BackwardSlashToken => write!(f, "BackwardSlashToken"),

            SyntaxKind::PipeToken => write!(f, "PipeToken"),
        }
    }
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "({}, {})", self.longitude, self.latitude)

        //write!(f, "SyntaxKind >>> {}", self.to_string())
        match *self {
            SyntaxKind::BadToken => write!(f, "BadToken"),
            SyntaxKind::CloseCurlyBraceToken => write!(f, "CloseCurlyBraceToken"),
            SyntaxKind::NumberToken => write!(f, "NumberToken"),
            SyntaxKind::CloseParenthesisToken => write!(f, "CloseParenthesisToken"),
            //SyntaxKind::CommaToken => write!(f, "CommaToken"),
            //SyntaxKind::DivideToken => write!(f, "DivideToken"),
            SyntaxKind::EOFToken => write!(f, "EOFToken"),
            SyntaxKind::EqualsToken => write!(f, "EqualsToken"),
            SyntaxKind::MinusToken => write!(f, "MinusToken"),
            SyntaxKind::StarToken => write!(f, "StarToken"),
            SyntaxKind::OpenCurlyBraceToken => write!(f, "OpenCurlyBraceToken"),
            SyntaxKind::OpenParenthesisToken => write!(f, "OpenParenthesisToken"),
            SyntaxKind::WhiteSpaceToken => write!(f, "WhiteSpaceToken"),
            SyntaxKind::PlusToken => write!(f, "PlusToken"),
            SyntaxKind::ForwardSlashToken => write!(f, "ForwardSlashToken"),
            SyntaxKind::BackwardSlashToken => write!(f, "BackwardSlashToken"),

            SyntaxKind::PipeToken => write!(f, "PipeToken"),
        }
    }
}

#[derive(Debug)]
pub struct SyntaxToken {
    pub kind: SyntaxKind,
    start: usize,
    pub text: String,
    pub value: Option<i32>,
}
impl SyntaxToken {
    pub fn new() -> SyntaxToken {
        SyntaxToken {
            kind: SyntaxKind::WhiteSpaceToken,
            start: 0,
            text: "".to_string(),
            value: None,
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    pub text: String,
    pub position: usize,

    diagnostics: Vec<String>,
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer {
            text: text.replace("\n", "").to_string(),
            position: 0,
            diagnostics: Vec::new(),
        }
    }

    pub fn diagnostics(&self) -> &Vec<String> {
        &self.diagnostics
    }

    fn current(&self) -> char {
        if self.position >= self.text.len() {
            '\0'
        } else {
            self.text.chars().nth(self.position).unwrap()
        }
    }

    fn next(&mut self) {
        self.position += 1;
    }

    pub fn next_token(&mut self) -> Option<SyntaxToken> {
        if self.position >= self.text.len() {
            return Some(SyntaxToken {
                kind: SyntaxKind::EOFToken,
                start: self.position,
                text: "\0".to_string(),
                value: None,
            });
        }

        if self.current().is_digit(10) {
            let start = self.position;
            while self.current().is_digit(10) {
                self.next();
            }
            let length = self.position - start;
            let text = self.text[start..self.position].to_string();
            if let Ok(value) = text.parse::<i32>() {
                return Some(SyntaxToken {
                    kind: SyntaxKind::NumberToken,
                    start,
                    text,
                    value: Some(value),
                });
            } else {
                self.diagnostics
                    .push(format!("The number {} isn't a valid Int32. ", text));
                return Some(SyntaxToken {
                    kind: SyntaxKind::BadToken,
                    start,
                    text,
                    value: None,
                });
            }
        }

        if self.current().is_whitespace() {
            let start = self.position;
            while self.current().is_whitespace() {
                self.next();
            }
            let length = self.position - start;
            let text = self.text[start..self.position].to_string();
            return Some(SyntaxToken {
                kind: SyntaxKind::WhiteSpaceToken,
                start,
                text,
                value: None,
            });
        }

        match self.current() {
            '+' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::PlusToken,
                    start: self.position - 1,
                    text: "+".to_string(),
                    value: None,
                })
            }
            '-' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::MinusToken,
                    start: self.position - 1,
                    text: "-".to_string(),
                    value: None,
                })
            }
            '*' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::StarToken,
                    start: self.position - 1,
                    text: "*".to_string(),
                    value: None,
                })
            }
            '/' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::ForwardSlashToken,
                    start: self.position - 1,
                    text: "/".to_string(),
                    value: None,
                })
            }
            '(' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::OpenParenthesisToken,
                    start: self.position - 1,
                    text: "(".to_string(),
                    value: None,
                })
            }
            ')' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::CloseParenthesisToken,
                    start: self.position - 1,
                    text: ")".to_string(),
                    value: None,
                })
            }
            '/' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::ForwardSlashToken,
                    start: self.position - 1,
                    text: ")".to_string(),
                    value: None,
                })
            }
            '\\' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::BackwardSlashToken,
                    start: self.position - 1,
                    text: "\\".to_string(),
                    value: None,
                })
            }
            '|' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::PipeToken,
                    start: self.position - 1,
                    text: "|".to_string(),
                    value: None,
                })
            }
            '=' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::EqualsToken,
                    start: self.position - 1,
                    text: "=".to_string(),
                    value: None,
                })
            }
            _ => {
                self.diagnostics
                    .push(format!("ERROR: bad character input: '{}'", self.current()));
                Some(SyntaxToken {
                    kind: SyntaxKind::BadToken,
                    start: self.position,
                    text: self.text[self.position..self.position + 1].to_string(),
                    value: None,
                })
            }
        } //end match
    }
}

//lexer produces tokens...
//parser produces sentences a.ka. trees
struct Parser {
    text: String,
    tokens: Vec<SyntaxToken>,
}
impl Parser {
    pub fn new(text: &str) -> Parser {
        let mut lexer = Lexer::new(text);
        let mut _tokens: Vec<SyntaxToken> = Vec::new();
        let mut token: SyntaxToken;
        //let mut token = lexer.next_token();
        // loop {
        //     token = lexer.next_token();
        //     if token.kind != SyntaxKind::WhiteSpaceToken && token.kind != SyntaxKind::BadToken {
        //         _tokens.push(token);
        //     } else if token.kind == SyntaxKind::EOFToken {
        //         break;
        //     }
        // }
        while let Some(token) = lexer.next_token() {
            if token.kind != SyntaxKind::WhiteSpaceToken && token.kind != SyntaxKind::BadToken {
                _tokens.push(token);
            } else if token.kind == SyntaxKind::EOFToken {
                break;
            }
        }

        Parser {
            text: text.to_string(),
            tokens: _tokens,
        }
    }
}
