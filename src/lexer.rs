use std::fs;
use std::iter::Peekable;
use std::vec::IntoIter;

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
        let mut line_no = 1;
        while let Some(c) = self.raw_data.next() {
            match c {
                '\n' => {
                    line_no += 1;
                }
                ' ' | '\t' => {}
                '+' => res.push(Token::new(TokenType::Operator(Operator::Plus), line_no)),
                '-' => res.push(Token::new(TokenType::Operator(Operator::Minus), line_no)),
                '(' => res.push(Token::new(TokenType::LeftParen, line_no)),
                ')' => res.push(Token::new(TokenType::RightParen, line_no)),
                '{' => res.push(Token::new(TokenType::LeftCurly, line_no)),
                '}' => res.push(Token::new(TokenType::RightCurly, line_no)),
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
                            res.push(Token::new(TokenType::Operator(Operator::Divide), line_no))
                        }
                    }
                }
                '*' => res.push(Token::new(TokenType::Operator(Operator::Multiply), line_no)),
                '>' => {
                    let is_done = if let Some(x) = self.raw_data.peek() {
                        if *x == '=' {
                            res.push(Token::new(
                                TokenType::Operator(Operator::GreaterThanEqual),
                                line_no,
                            ));
                            self.raw_data.next();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                    if !is_done {
                        res.push(Token::new(
                            TokenType::Operator(Operator::GreaterThan),
                            line_no,
                        ));
                    }
                }
                '<' => {
                    let is_done = if let Some(x) = self.raw_data.peek() {
                        if *x == '=' {
                            res.push(Token::new(
                                TokenType::Operator(Operator::LessThanEqual),
                                line_no,
                            ));
                            self.raw_data.next();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                    if !is_done {
                        res.push(Token::new(TokenType::Operator(Operator::LessThan), line_no));
                    }
                }
                ';' => res.push(Token::new(TokenType::SemiColon, line_no)),
                '=' => {
                    let is_done = if let Some(x) = self.raw_data.peek() {
                        if *x == '=' {
                            res.push(Token::new(TokenType::Operator(Operator::Equality), line_no));
                            self.raw_data.next();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                    if !is_done {
                        res.push(Token::new(TokenType::Operator(Operator::Equal), line_no));
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
                    res.push(Token::new(
                        TokenType::Number(ordered_float::OrderedFloat(num.parse::<f32>().unwrap())),
                        line_no,
                    ))
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
                    res.push(Token::new(TokenType::StringLiteral(str), line_no))
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
                        "print" => Token::new(TokenType::Print, line_no),
                        "let" => Token::new(TokenType::Let, line_no),
                        "if" => Token::new(TokenType::If, line_no),
                        "elif" => Token::new(TokenType::Elif, line_no),
                        "while" => Token::new(TokenType::While, line_no),
                        "else" => Token::new(TokenType::Else, line_no),
                        _ => Token::new(TokenType::Identifier(str), line_no),
                    });
                }
            }
        }
        res
    }
}

#[derive(Eq, Hash, Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equality,
}

#[derive(Eq, Hash, Debug, PartialEq, Clone)]
pub struct Token {
    pub token: TokenType,
    pub line_no: usize,
}

impl Token {
    fn new(token: TokenType, line_no: usize) -> Self {
        Self { token, line_no }
    }
}

#[derive(Eq, Hash, Debug, PartialEq, Clone)]
pub enum TokenType {
    Operator(Operator),
    StringLiteral(String),
    Print,
    Let,
    If,
    Elif,
    Else,
    While,
    SemiColon,
    Number(ordered_float::OrderedFloat<f32>),
    Identifier(String),
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
}
