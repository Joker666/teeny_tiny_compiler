mod lex;
mod token;

use lex::Lexer;
use std::env;

fn main() {
    println!("Teeny Tiny Compiler");

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected the teeny file");

    let source = "LET foobar = 123";
    let mut lexer = Lexer::new(source);

    while lexer.peek() != '\0' {
        print!("{}", lexer.cur_char);
        lexer.next_char();
    }
}

fn get_nth_arg(n: usize) -> String {
    env::args().nth(n).unwrap()
}
