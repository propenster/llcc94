use std::{env, fs};

//use lexer::Lexer;

//use ringo::lexer::{ Lexer};

mod lexer;
mod token;
mod parser;
mod ast;

fn main(){
    let file = env::args()
        .nth(1)
        .unwrap();

    let contents = fs::read_to_string(file).unwrap();

    let lexer = lexer::Lexer::new(contents);
    // while let Some(token) = lexer.next(){
    //     println!("{:?}", token);
    // }
    let mut p = parser::Parser::new(lexer);

    let program = p.parse();
    println!("{:?}", program);

}