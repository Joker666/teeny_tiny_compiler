mod emitter;
mod lex;
mod parse;
mod token;

use crate::emitter::Emitter;
use lex::Lexer;
use parse::Parser;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Teeny Tiny Compiler");

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Expected the teeny file");

    let path_arg = get_nth_arg(1);
    let path = Path::new(&path_arg);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut source = String::new();
    if let Err(why) = file.read_to_string(&mut source) {
        panic!("couldn't read {}: {}", display, why);
    }

    let lexer = Lexer::new(&source);
    let mut emitter = Emitter::new("out.c");
    let mut parser = Parser::new(lexer, &mut emitter);

    parser.program();
    emitter.write_file();
    println!("Compiling completed")
}

fn get_nth_arg(n: usize) -> String {
    env::args().nth(n).unwrap()
}
