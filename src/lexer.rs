use std::fs;
use std::iter::Peekable;
use std::vec::IntoIter;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug)]
pub struct Lexer {
    raw_data: Peekable<IntoIter<char>>,
}

impl Lexer {
    pub fn from_file(filename: &str) -> Self {
        Lexer {
            raw_data: (fs::read_to_string(filename).expect("Something went wrong"))
                .chars()
                .collect::<Vec<_>>()
                .into_iter()
                .peekable(),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut res = vec![];
        while let Some(c) = self.raw_data.next() {
            match c {
                '\n' | ' ' | '\t' => {}
                '+' => res.push(Token::Operator(Operator::Plus)),
                '-' => res.push(Token::Operator(Operator::Minus)),
                '(' => res.push(Token::LeftParen),
                ')' => res.push(Token::RightParen),
                '/' => {
                    if let Some(x) = self.raw_data.peek() {
                        if *x == '/' {
                            self.raw_data.next();
                            while let Some(x) = self.raw_data.next() {
                                if x == '\n' {
                                    break;
                                }
                            }
                        } else {
                            println!("Invalid comment string");
                            break;
                        }
                    }
                }
                '*' => res.push(Token::Operator(Operator::Multiply)),
                // ';' => {},
                '>' => {
                    let is_done = if let Some(x) = self.raw_data.peek() {
                        if *x == '=' {
                            res.push(Token::Operator(Operator::GreaterThanEqual));
                            self.raw_data.next();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                    if !is_done {
                        res.push(Token::Operator(Operator::GreaterThan))
                    }
                }
                '<' => {
                    let is_done = if let Some(x) = self.raw_data.peek() {
                        if *x == '=' {
                            res.push(Token::Operator(Operator::LessThanEqual));
                            self.raw_data.next();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                    if !is_done {
                        res.push(Token::Operator(Operator::LessThan))
                    }
                }
                ';' => res.push(Token::SemiColon),
                '=' => {
                    let is_done = if let Some(x) = self.raw_data.peek() {
                        if *x == '=' {
                            res.push(Token::Operator(Operator::Equality));
                            self.raw_data.next();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                    if !is_done {
                        res.push(Token::Operator(Operator::Equal));
                    }
                }
                n @ '0'..='9' => {
                    let mut num = String::from(n);
                    while let Some(c) = self.raw_data.peek() {
                        if !c.is_digit(10) {
                            break;
                        }
                        num.push(*c);
                        self.raw_data.next();
                    }
                    res.push(Token::Number(num.parse::<f32>().unwrap()))
                }
                '"' => {
                    let mut done = false;
                    let mut str = String::new();
                    let mut cur = 'a';
                    while let Some(c) = self.raw_data.next() {
                        match c {
                            '"' => {
                                if cur == '\\' {
                                    str.push('"');
                                    cur = 'a';
                                } else {
                                    done = true;
                                    break;
                                }
                            }
                            '\\' => {
                                if cur == '\\' {
                                    str.push('\\');
                                    cur = 'a';
                                } else {
                                    cur = '\\';
                                }
                            }
                            x => {
                                if cur == '\\' {
                                    str.push('\\');
                                }
                                str.push(x);
                                cur = x;
                            }
                        }
                    }
                    if !done {
                        println!("Error");
                        break;
                    }
                    res.push(Token::StringLiteral(str));
                }
                x => {
                    if !x.is_alphanumeric() {
                        println!("Invalid token: {}", x);
                        continue;
                    }
                    let mut str = String::from(x);
                    while let Some(c) = self.raw_data.peek() {
                        if !(c.is_alphanumeric() || *c == '_') {
                            break;
                        }
                        str.push(*c);
                        self.raw_data.next();
                    }
                    res.push(match str.as_str() {
                        "print" => Token::Print,
                        "let" => Token::Let,
                        _ => Token::Identifier(str),
                    });
                }
            }
        }
        res
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Equal,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equality,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Operator(Operator),
    StringLiteral(String),
    Print,
    Let,
    SemiColon,
    Number(f32),
    Identifier(String),
    LeftParen,
    RightParen,
}
