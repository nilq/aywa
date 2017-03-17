extern crate kravl_parser;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use kravl_parser::syntax;

fn main() {
    let mut lexer = syntax::lexer::Lexer::new();    
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let path = &args[1];
        let mut source = match File::open(path) {
            Ok(f)  => f,
            Err(_) => panic!("failed to open: {}", path),
        };

        let mut buffer = String::new();
        source.read_to_string(&mut buffer).unwrap();

        lexer.tokenize(buffer);

        for n in lexer.get_tokens() {
            println!("found: {:?}({:?})", n.token_type, n.content)
        }

        println!("\n");

        let mut parser = syntax::ast::Parser::from(lexer);

        let stack = parser.parse_full();

        println!("{:?}", stack);

        std::process::exit(0)
    }
}