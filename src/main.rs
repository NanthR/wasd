mod lexer;
mod parser;
mod utils;

fn main() {
    let filename = "test.asdf";
    println!("In file {}", filename);

    let mut lexer = lexer::Lexer::from_file(filename);
    match parser::parse(lexer.lex()) {
        Ok(p) => println!("{}", utils::get_sexp(&p)),
        Err(x) => println!("{}", x)
    }
}
