mod lex;
mod token;

use lex::Lexer;
use std::env;
use token::TokenType;

fn main() {
    println!("Teeny Tiny Compiler");

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected the teeny file");

    let source = "IF+-123 foo*THEN/";
    let mut lexer = Lexer::new(source);

    let mut token = match lexer.get_token() {
        None => panic!("Token not found"),
        Some(t) => t,
    };

    while token.kind != TokenType::Eof {
        println!("{:?}", token.kind);
        token = match lexer.get_token() {
            None => panic!("Token not found"),
            Some(t) => t,
        };
    }
}

fn get_nth_arg(n: usize) -> String {
    env::args().nth(n).unwrap()
}
