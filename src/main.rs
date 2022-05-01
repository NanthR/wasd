mod lexer;
mod parser;
mod utils;

fn main() {
    let filename = "test.asdf";
    let mut lexer = lexer::Lexer::from_file(filename);
    let lexed = lexer.lex();
    // println!("{:?}", lexed);
    let mut parser = parser::Parser::new(lexed);
    match parser.parse() {
        Ok(p) => {
            for i in p.iter() {
                println!("{}", utils::get_sexp(i, 1));
            }
        }
        Err(x) => println!("{}", x),
    }
}
