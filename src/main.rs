mod lex;
mod token;

use lex::Lexer;
use std::env;
use token::TokenType;

fn main() {
    println!("Teeny Tiny Compiler");

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected the teeny file");

    let source = "+- \"This is a string\" # This is a comment!\n */";
    let mut lexer = Lexer::new(source);
    let mut token = lexer.get_token().unwrap();

    while token.kind != TokenType::Eof {
        println!("{:?}", token.kind);
        token = lexer.get_token().unwrap();
    }
}

fn get_nth_arg(n: usize) -> String {
    env::args().nth(n).unwrap()
}
