



#[derive(Debug)]
pub enum TokenKind {
    Identifier,
    Assign,
    Let,
    String,
    IntType,
    StringType,
    FloatType,
    BoolType,

    Number,
    Plus,
    Multiply,
    Minus,
    Divide
}





#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}
