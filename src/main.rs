mod lexer;
mod parser;
mod utils;
mod transpile;
use std::io::Write;
use std::fs;

fn main() {
    let filename = "test.asdf";
    let mut lexer = lexer::Lexer::from_file(filename);
    let lexed = lexer.lex();
    // println!("{:?}", lexed);
    let mut parser = parser::Parser::new(lexed);
    match parser.parse() {
        Ok(p) => {
            println!("Intermediate code => S Expressions\n");
            for n in p.iter() {
                println!("{}", crate::utils::get_sexp(n, 0));
            }
            println!();
            let transpiler = transpile::Transpiler::new(p);
            println!("Transpiled code to python\n");
            match transpiler.transpile() {
                Ok(t) => {
                    let mut f = fs::File::create("result.py").expect("Couldn't create file");
                    write!(f, "{}", t);
                    println!("Written to result.py");
                },
                Err(x) => println!("{}", x)
            }
        }
        Err(x) => println!("{}", x),
    }
}
