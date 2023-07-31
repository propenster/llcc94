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
    NumericExpressionSyntax,
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
            SyntaxKind::NumericExpressionSyntax => write!(f, "NumericExpressionSyntax"),

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
            SyntaxKind::NumericExpressionSyntax => write!(f, "NumericExpressionSyntax"),

            SyntaxKind::PipeToken => write!(f, "PipeToken"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SyntaxToken {
    pub kind: SyntaxKind,
    pub position: usize,
    pub text: String,
    pub value: Option<i32>,
}
impl SyntaxToken {
    pub fn new(kind: SyntaxKind, position: usize, text: String, value: Option<i32>) -> SyntaxToken {
        SyntaxToken {
            kind: kind,
            position: 0,
            text: text,
            value: value,
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
                position: self.position,
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
                    position: start,
                    text,
                    value: Some(value),
                });
            } else {
                self.diagnostics
                    .push(format!("The number {} isn't a valid Int32. ", text));
                return Some(SyntaxToken {
                    kind: SyntaxKind::BadToken,
                    position: start,
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
                position: start,
                text,
                value: None,
            });
        }

        match self.current() {
            '+' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::PlusToken,
                    position: self.position - 1,
                    text: "+".to_string(),
                    value: None,
                })
            }
            '-' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::MinusToken,
                    position: self.position - 1,
                    text: "-".to_string(),
                    value: None,
                })
            }
            '*' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::StarToken,
                    position: self.position - 1,
                    text: "*".to_string(),
                    value: None,
                })
            }
            '/' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::ForwardSlashToken,
                    position: self.position - 1,
                    text: "/".to_string(),
                    value: None,
                })
            }
            '(' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::OpenParenthesisToken,
                    position: self.position - 1,
                    text: "(".to_string(),
                    value: None,
                })
            }
            ')' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::CloseParenthesisToken,
                    position: self.position - 1,
                    text: ")".to_string(),
                    value: None,
                })
            }
            '/' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::ForwardSlashToken,
                    position: self.position - 1,
                    text: ")".to_string(),
                    value: None,
                })
            }
            '\\' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::BackwardSlashToken,
                    position: self.position - 1, //fka start..i.e position (index) where this current token starts from
                    text: "\\".to_string(),
                    value: None,
                })
            }
            '|' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::PipeToken,
                    position: self.position - 1,
                    text: "|".to_string(),
                    value: None,
                })
            }
            '=' => {
                self.next();
                Some(SyntaxToken {
                    kind: SyntaxKind::EqualsToken,
                    position: self.position - 1,
                    text: "=".to_string(),
                    value: None,
                })
            }
            _ => {
                self.diagnostics
                    .push(format!("ERROR: bad character input: '{}'", self.current()));
                Some(SyntaxToken {
                    kind: SyntaxKind::BadToken,
                    position: self.position,
                    text: self.text[self.position..self.position + 1].to_string(),
                    value: None,
                })
            }
        } //end match
    }
}

//lexer produces tokens...
//parser produces sentences a.ka. trees
pub struct Parser {
    text: String,
    position: usize,
    tokens: Vec<SyntaxToken>,
}
impl Parser {
    pub fn new(text: &str) -> Parser {
        let mut lexer = Lexer::new(text);
        let mut _tokens: Vec<SyntaxToken> = Vec::new();
        // let mut token: SyntaxToken;
        while let Some(token) = lexer.next_token() {
            if token.kind == SyntaxKind::EOFToken {
                break;
            } else if token.kind != SyntaxKind::WhiteSpaceToken
                && token.kind != SyntaxKind::BadToken
            {
                _tokens.push(token);
            }
        }

        Parser {
            text: text.to_string(),
            tokens: _tokens,
            position: 0,
        }
    }

    fn peek(&self, offset: usize) -> Option<SyntaxToken> {
        let index = self.position + offset;
        if index >= self.tokens.len() {
            return Some(self.tokens[self.tokens.len() - 1].clone());
        } else {
            return Some(self.tokens[index].clone());
        }
    }
    pub fn current(&self) -> SyntaxToken {
        self.peek(0).unwrap()
    }

    pub fn next_token(&mut self) -> SyntaxToken {
        let current = self.current();
        self.position += 1;
        current
    }

    pub fn match_(&mut self, kind: SyntaxKind) -> SyntaxToken {
        if self.current().kind == kind {
            self.next_token().clone()
        } else {
            SyntaxToken::new(kind, self.current().position, String::new(), None)
        }
    }
    fn match_token(&mut self, kind: SyntaxKind) -> SyntaxToken {
        if self.current().kind == kind {
            self.next_token().clone()
        } else {
            SyntaxToken::new(kind, self.current().position, String::new(), None)
        }
    }

    pub fn parse(&mut self) -> ExpressionSyntax {
        let mut left = self.parse_primary_expression();
        while self.current().kind == SyntaxKind::PlusToken
            || self.current().kind == SyntaxKind::MinusToken
        {
            let operator_token = self.next_token();
            let right = self.parse_primary_expression();
            left = ExpressionSyntax::BinaryExpression(BinaryExpressionSyntax {
                left: Box::new(left),
                operator_token,
                right: Box::new(right),
            });
        }

        left
    }

    fn parse_expression(&mut self) -> ExpressionSyntax {
        let mut left = self.parse_primary_expression();

        while self.current().kind == SyntaxKind::PlusToken
            || self.current().kind == SyntaxKind::MinusToken
        {
            let operator_token = self.next_token();
            let right = self.parse_primary_expression();
            left = ExpressionSyntax::BinaryExpression(BinaryExpressionSyntax {
                left: Box::new(left),
                operator_token,
                right: Box::new(right),
            });
        }

        left
    }

    fn parse_primary_expression(&mut self) -> ExpressionSyntax {
        let number_token = self.match_token(SyntaxKind::NumberToken);
        ExpressionSyntax::NumericExpression(NumericExpressionSyntax { number_token })
    }
}

//So to implement a SyntaxNode that we have other EpxressionSyntaxSubNodes extend from...
// this means SyntaxNode abstract class in simpler languages C#, JAVA... it's much harder in Rust, Let me try...

#[derive(Debug, Clone)]
pub enum ExpressionSyntax {
    NumericExpression(NumericExpressionSyntax),
    BinaryExpression(BinaryExpressionSyntax),
}
// struct ExpressionSyntax {
//     number_syntax: Option<NumericExpressionSyntax>,
// }

#[derive(Debug, Clone)]
pub struct NumericExpressionSyntax {
    number_token: SyntaxToken,
    // kind: SyntaxKind,
    // children: Vec<SyntaxToken>,
}
#[derive(Debug, Clone)]
pub struct BinaryExpressionSyntax {
    left: Box<ExpressionSyntax>,
    operator_token: SyntaxToken,
    right: Box<ExpressionSyntax>,
}
