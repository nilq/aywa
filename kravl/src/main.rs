extern crate kravl_parser;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use kravl_parser::syntax;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {

        let mut lexer = syntax::lexer::Lexer::new();

        let path = &args[1];
        let mut source = match File::open(path) {
            Ok(f)  => f,
            Err(_) => panic!("failed to open: {}", path),
        };

        let mut buffer = String::new();
        source.read_to_string(&mut buffer).unwrap();

        lexer.tokenize(buffer);
        
        let mut parser = syntax::ast::Parser::from(lexer);      

        let stack = parser.parse_full();

        for n in stack {
            for j in n {
                println!("{:?}", j)
            }
        }

        std::process::exit(0)

    } else {
        println!("the kravl language");

        loop {
            print!(">> ");
            io::stdout().flush();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    let mut lexer  = syntax::lexer::Lexer::new();

                    lexer.tokenize(input);
                    
                    let mut parser = syntax::ast::Parser::from(lexer);      

                    let stack = parser.parse_full();

                    for n in stack {
                        for j in n {
                            println!("{:?}", j)
                        }
                    }
                },

                Err(e) => panic!(e)
            }
        }
    }
}