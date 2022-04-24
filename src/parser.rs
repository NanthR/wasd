use crate::lexer::{Operator, Token};
/*
program := statement
statement := decl statement | decl
decl := var_dec
var_dec := let iden equal val
val := expr | string
expr := summand | summand plus expr | summand minus expr
summand := term / summand | term * summand | term
term := num | (expr)
*/

#[derive(Debug)]
pub struct ParseNode {
    pub current: Token,
    pub children: Vec<ParseNode>,
}

impl ParseNode {
    pub fn new(current: Token, children: Vec<ParseNode>) -> Self {
        ParseNode { children, current }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<Vec<ParseNode>, String> {
        self.parse_statement(0).and_then(|(n, i)| {
            if i == self.tokens.len() {
                Ok(n)
            } else {
                Err("Invalid parse".to_string())
            }
        })
    }

    fn parse_statement(&self, mut pos: usize) -> Result<(Vec<ParseNode>, usize), String> {
        let mut res: Vec<ParseNode> = vec![];
        while self.tokens.get(pos).is_some() {
            let (node, next_pos) = self.parse_decl(pos)?;
            pos = next_pos;
            res.push(node)
        }
        Ok((res, pos))
    }

    fn parse_decl(&self, pos: usize) -> Result<(ParseNode, usize), String> {
        match self.tokens.get(pos) {
            Some(t) => match t {
                Token::Let => {
                    if let (Some(id), Some(Token::Operator(Operator::Equal))) =
                        (self.tokens.get(pos + 1), self.tokens.get(pos + 2))
                    {
                        if let Some(Token::StringLiteral(x)) = self.tokens.get(pos + 3) {
                            Ok((
                                ParseNode::new(
                                    Token::Let,
                                    vec![
                                        ParseNode::new(id.clone(), vec![]),
                                        ParseNode::new(Token::StringLiteral(x.to_string()), vec![]),
                                    ],
                                ),
                                pos + 4,
                            ))
                        } else {
                            let (node, next_pos) = self.parse_expr(pos + 3)?;
                            Ok((
                                ParseNode::new(
                                    Token::Let,
                                    vec![ParseNode::new(id.clone(), vec![]), node],
                                ),
                                next_pos,
                            ))
                        }
                    } else {
                        Err("Invalid decl".to_string())
                    }
                }
                x => {
                    println!("{:?}", x);
                    Err("Not implemented".to_string())
                }
            },
            _ => Err("Couldn't be parsed".to_string()),
        }
    }

    fn parse_expr(&self, pos: usize) -> Result<(ParseNode, usize), String> {
        let (node, next_pos) = self.parse_summand(pos)?;
        let c = self.tokens.get(next_pos);
        match c {
            t
            @ (Some(Token::Operator(Operator::Plus)) | Some(Token::Operator(Operator::Minus))) => {
                let (rhs, i) = self.parse_expr(next_pos + 1)?;
                Ok((ParseNode::new(t.unwrap().clone(), vec![node, rhs]), i))
            }
            _ => Ok((node, next_pos)),
        }
    }

    fn parse_summand(&self, pos: usize) -> Result<(ParseNode, usize), String> {
        let (node, next_pos) = self.parse_term(pos)?;
        let c = self.tokens.get(next_pos);
        match c {
            t @ (Some(Token::Operator(Operator::Multiply))
            | Some(Token::Operator(Operator::Divide))) => {
                let (rhs, i) = self.parse_summand(next_pos + 1)?;
                Ok((ParseNode::new(t.unwrap().clone(), vec![node, rhs]), i))
            }
            _ => Ok((node, next_pos)),
        }
    }

    fn parse_term(&self, pos: usize) -> Result<(ParseNode, usize), String> {
        let c = self.tokens.get(pos);
        if let Some(Token::LeftParen) = c {
            let (node, next_pos) = self.parse_expr(pos + 1)?;
            if let Some(Token::RightParen) = self.tokens.get(next_pos) {
                Ok((node, next_pos + 1))
            } else {
                Err("Parantheses not closed".to_string())
            }
        } else if let Some(n @ Token::Number(_)) = c {
            Ok((ParseNode::new(n.clone(), vec![]), pos + 1))
        } else {
            Err("Couldn't do".to_string())
        }
    }
}
