use crate::lexer::{Operator, Token};
/*
program := statement
statement := decl statement | decl
decl := var_dec
var_dec := let iden equal expr
expr := summand | summand plus expr | summand minus expr
summand := term * summand | term
term := num
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

pub fn parse(tokens: Vec<Token>) -> Result<ParseNode, String> {
    parse_statement(&tokens, 0).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err("Invalid parse".to_string())
        }
    })
}

fn parse_statement(tokens: &Vec<Token>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node, next_pos) = parse_decl(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        _ => Ok((node, next_pos)),
    }
}

fn parse_decl(tokens: &Vec<Token>, pos: usize) -> Result<(ParseNode, usize), String> {
    match tokens.get(pos) {
        Some(t) => match t {
            Token::Let => {
                if let (Some(id), Some(Token::Operator(Operator::Equal))) =
                    (tokens.get(pos + 1), tokens.get(pos + 2))
                {
                    let (node, next_pos) = parse_expr(tokens, pos + 3)?;
                    Ok((
                        ParseNode::new(Token::Let, vec![ParseNode::new(id.clone(), vec![]), node]),
                        next_pos,
                    ))
                } else {
                    Err("Invalid decl".to_string())
                }
            }
            _ => Err("Not implemented".to_string()),
        },
        _ => Err("Couldn't be parsed".to_string()),
    }
}

fn parse_expr(tokens: &Vec<Token>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node, next_pos) = parse_summand(tokens, pos)?;
    let c = tokens.get(next_pos);
    if let Some(&Token::Operator(Operator::Plus)) = c {
        let (rhs, i) = parse_expr(tokens, next_pos + 1)?;
        Ok((
            ParseNode::new(Token::Operator(Operator::Plus), vec![node, rhs]),
            i,
        ))
    } else {
        Ok((node, next_pos))
    }
}

fn parse_summand(tokens: &Vec<Token>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node, next_pos) = parse_term(tokens, pos)?;
    let c = tokens.get(next_pos);
    if let Some(&Token::Operator(Operator::Multiply)) = c {
        let (rhs, i) = parse_summand(tokens, next_pos + 1)?;
        Ok((
            ParseNode::new(Token::Operator(Operator::Multiply), vec![node, rhs]),
            i,
        ))
    } else {
        Ok((node, next_pos))
    }
}

fn parse_term(tokens: &Vec<Token>, pos: usize) -> Result<(ParseNode, usize), String> {
    if let Some(n @ Token::Number(_)) = tokens.get(pos) {
        Ok((ParseNode::new(n.clone(), vec![]), pos + 1))
    } else {
        Err("Couldn't do".to_string())
    }
}
