use core::panic;

use crate::ast::{Expression, Statement, BinaryOp};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }
    //parser our entire program out before we can read it into an AST... Abstract Syntax Tree
    pub fn parse(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        //noew parse shi into statements...
        while let Some(token) = self.lexer.next() {
            match token.kind {
                TokenKind::Let => {
                    let identifier = if let Some(identifier) = self.lexer.next() {
                        identifier
                    } else {
                        panic!("Expected an identifier.");
                    };

                    if !matches!(
                        self.lexer.peek(),
                        Some(Token {
                            kind: TokenKind::Assign,
                            ..
                        })
                    ) {
                        panic!("Expected an '=' for assignment");
                    }

                    self.lexer.next();

                    //now parse out the expressions...
                    let expression = self.parse_expression(0);

                    //println!("{:?}", expression);
                    statements.push(Statement::Let {
                        name: identifier.literal,
                        initial: expression,
                    })
                }
                _ => unimplemented!(),
            }
        }

        statements //prog is type alias for Vec<Statement> so chill d f out...
    }

    fn parse_expression(&mut self, bp: u8) -> Expression {
        let mut lhs = match self.lexer.next() {
            Some(Token {
                kind: TokenKind::Number,
                literal,
            }) => Expression::Number(literal.parse().unwrap()),
            _ => unimplemented!(),
        };


        loop {
            let infix =  if let Some(infix) = self.lexer.peek(){
                infix
            }else{
                break;
            };

            
            if let Some((lbp, rbp)) = infix_binding_power(infix.kind){
                if lbp < bp {
                    break;
                }
                let op = self.lexer.next().unwrap().kind;
                let rhs = self.parse_expression(rbp);

                lhs = make_infix_expression(lhs, op, rhs);

                continue;


            }
            break;

        }

        lhs
    }
}

fn make_infix_expression(lhs: Expression, operator: TokenKind, rhs: Expression) -> Expression{
    
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);
    match operator{
        TokenKind::Plus => Expression::Binary(lhs, BinaryOp::Plus, rhs),
        TokenKind::Multiply => Expression::Binary(lhs, BinaryOp::Multiply, rhs),
        TokenKind::Minus => Expression::Binary(lhs, BinaryOp::Minus, rhs),
        TokenKind::Divide => Expression::Binary(lhs, BinaryOp::Divide, rhs),

        _ => unimplemented!()
    }
}
fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)>{
    let bp = match kind {
        TokenKind::Multiply | TokenKind::Divide => (8,9),
        TokenKind::Minus | TokenKind::Plus => (6,7),
        _ => return None,
    };

    Some(bp)
}
//this struct holds our AST
pub type Program = Vec<Statement>;
