use ringo::llcc94::{Lexer, SyntaxKind, SyntaxToken};
use std::io;

fn main() {
    println!("Welcome ringo compiler v0.0.1");
    println!();

    //my repl
    loop {
        println!("> ");
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            return;
        }

        //init lexer for line grabbed from stdin buff
        let mut lexer = Lexer::new(line.as_str());
        println!("{:?}", lexer);

        loop {
            let token = lexer.next_token().unwrap();
            if token.kind == SyntaxKind::EOFToken {
                break;
            }

            //println!("{}: '{}'", );
            //if there is a value...

            if let Some(tok) = token.value {
                println!("{}: '{}' {}", token.kind, token.text, tok);
            } else {
                println!("{}: '{}'", token.kind, token.text);
            }
        } //end inner while...
    } //end infinite loop..
}
