mod lexer;
mod parser;
mod utils;

fn main() {
    let filename = "test.asdf";
    println!("In file {}", filename);

    let mut lexer = lexer::Lexer::from_file(filename);
    let parser = parser::Parser::new(lexer.lex());
    match parser.parse() {
        Ok(p) => println!("{}", utils::get_sexp(&p)),
        Err(x) => println!("{}", x)
    }
}
