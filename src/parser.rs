use crate::lexer::{Operator, Token};
use std::collections::HashSet;

/*
program := statement
statement := decl statement | decl
decl := var_dec | whileLoop | ifStatement
whileLoop := while (expr) {statement}
ifStatement := if (expr) {statement} | if (expr) {statement} elseBlock
elseBlock := elif (expr) {statement} | elif (expr) {statement} elseBlock | else {statement}
var_dec := let iden equal val
val := expr | string
expr := parsed with pratt parser
*/

pub struct Parser {
    tokens: Vec<Token>,
    cur: usize,
}

#[derive(Debug)]
pub struct ParseNode {
    pub token: Token,
    pub extra_info: Option<Box<ParseNode>>,
    pub children: Vec<ParseNode>,
}

impl ParseNode {
    fn new(token: Token, extra_info: Option<Box<ParseNode>>, children: Vec<ParseNode>) -> Self {
        Self {
            token,
            extra_info,
            children,
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cur: 0 }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.cur).cloned()
    }

    fn peek_n(&self, offset: usize) -> Option<Token> {
        self.tokens.get(self.cur + offset).cloned()
    }

    fn next(&mut self) -> Option<Token> {
        let res = self.tokens.get(self.cur).cloned();
        self.advance();
        res
    }

    fn advance(&mut self) {
        self.cur += 1;
    }

    pub fn parse(&mut self) -> Result<Vec<ParseNode>, String> {
        let mut env = HashSet::new();
        let res = self.parse_statement(&mut env)?;
        if self.peek().is_none() {
            println!();
            Ok(res)
        } else {
            Err("Invalid parse".to_string())
        }
    }

    fn parse_statement(&mut self, env: &mut HashSet<Token>) -> Result<Vec<ParseNode>, String> {
        let mut res: Vec<ParseNode> = vec![];
        while self.peek().is_some() && self.peek() != Some(Token::RightCurly) {
            let node = self.parse_decl(env)?;
            res.push(node)
        }
        Ok(res)
    }

    fn parse_decl(&mut self, env: &mut HashSet<Token>) -> Result<ParseNode, String> {
        match self.next() {
            Some(t) => match t {
                Token::Let => {
                    if let (Some(Token::Identifier(id)), Some(Token::Operator(Operator::Equal))) =
                        (self.peek(), self.peek_n(1))
                    {
                        env.insert(Token::Identifier(id.clone()));
                        self.advance();
                        self.advance();
                        if let Some(Token::StringLiteral(x)) = self.peek() {
                            self.advance();
                            Ok(ParseNode::new(
                                Token::Let,
                                None,
                                vec![
                                    ParseNode::new(Token::Identifier(id), None, vec![]),
                                    ParseNode::new(Token::StringLiteral(x), None, vec![]),
                                ],
                            ))
                        } else {
                            let node = self.parse_expr(0, env)?;
                            Ok(ParseNode::new(
                                Token::Let,
                                None,
                                vec![ParseNode::new(Token::Identifier(id), None, vec![]), node],
                            ))
                        }
                    } else {
                        Err("Invalid variable declaration".to_string())
                    }
                }
                Token::While => {
                    if let Some(Token::LeftParen) = self.next() {
                        let node = self.parse_expr(0, env)?;
                        if let Some(Token::RightParen) = self.next() {
                            if let Some(Token::LeftCurly) = self.next() {
                                let nodes = self.parse_statement(&mut env.clone())?;
                                if let Some(Token::RightCurly) = self.next() {
                                    Ok(ParseNode::new(Token::While, Some(Box::new(node)), nodes))
                                } else {
                                    Err("Missing right curly in while loop".to_string())
                                }
                            } else {
                                Err("Missing left curly in while loop".to_string())
                            }
                        } else {
                            Err("While conditon doesn't have a closing paranthesis".to_string())
                        }
                    } else {
                        Err("Missing open paranthesis in while loop".to_string())
                    }
                }
                Token::If => {
                    if let Some(Token::LeftParen) = self.next() {
                        let if_cond = self.parse_expr(0, env)?;
                        if self.next() != Some(Token::RightParen) {
                            return Err("If condition not closed".to_string());
                        }
                        if self.next() != Some(Token::LeftCurly) {
                            return Err("If condition must start with curly braces".to_string());
                        }
                        let mut nodes = self.parse_statement(&mut env.clone())?;
                        if self.next() != Some(Token::RightCurly) {
                            return Err("If block not closed".to_string());
                        }
                        let mut res = vec![];
                        while let Some(Token::Elif) = self.next() {
                            if self.next() != Some(Token::LeftParen) {
                                return Err(
                                    "Elif condition must start with paranthesis".to_string()
                                );
                            }
                            let elif_cond = self.parse_expr(0, env)?;
                            if self.next() != Some(Token::RightParen) {
                                return Err("Elif condition not closed".to_string());
                            }
                            if self.next() != Some(Token::LeftCurly) {
                                return Err("Elif bloc must start with curly braces".to_string());
                            }
                            let statements = self.parse_statement(&mut env.clone())?;
                            if let Some(Token::RightCurly) = self.next() {
                                res.push(ParseNode::new(
                                    Token::Elif,
                                    Some(Box::new(elif_cond)),
                                    statements,
                                ))
                            } else {
                                return Err("Elif block not closed".to_string());
                            }
                        }
                        if self.next() == Some(Token::Else) {
                            if self.next() != Some(Token::LeftCurly) {
                                return Err("Else bloc must start with curly braces".to_string());
                            }
                            let statements = self.parse_statement(&mut env.clone())?;
                            if self.next() != Some(Token::RightCurly) {
                                return Err("Else block not closed".to_string());
                            }
                            res.push(ParseNode::new(Token::Else, None, statements))
                        }
                        nodes.append(&mut res);
                        Ok(ParseNode::new(Token::If, Some(Box::new(if_cond)), nodes))
                    } else {
                        Err("If condition must start with an open paranthesis".to_string())
                    }
                }
                Token::Print => {
                    if self.next() != Some(Token::LeftParen) {
                        return Err("Left paranthesis missing in function call".to_string());
                    }
                    let print_node = self.parse_expr(0, env)?;
                    if self.next() != Some(Token::RightParen) {
                        return Err("Right paranthesis missing in function call".to_string());
                    }
                    Ok(ParseNode::new(Token::Print, None, vec![print_node]))
                }
                _ => Err(format!("Invalid token {:?}", t)),
            },
            _ => Err("Couldn't be parsed".to_string()),
        }
    }

    fn infix_binding_power(op: &Operator) -> (u8, u8) {
        match op {
            Operator::Equality => (9, 10),
            Operator::LessThan | Operator::LessThanEqual => (11, 12),
            Operator::GreaterThan | Operator::GreaterThanEqual => (13, 14),
            Operator::Plus | Operator::Minus => (15, 16),
            Operator::Multiply | Operator::Divide => (17, 18),
            _ => panic!("Bad"),
        }
    }

    fn prefix_binding_power(op: &Operator) -> u8 {
        match op {
            Operator::Plus | Operator::Minus => 5,
            _ => panic!("Bad operator {:?}", op),
        }
    }

    fn parse_expr(&mut self, cur_bp: u8, env: &HashSet<Token>) -> Result<ParseNode, String> {
        let mut lhs = match self.next() {
            Some(t @ Token::Number(_)) => ParseNode::new(t, None, vec![]),
            Some(t @ Token::Identifier(_)) => {
                if env.get(&t).is_none() {
                    colour::dark_red_ln!("{:?} used before declaration", t);
                }
                ParseNode::new(t, None, vec![])
            }
            Some(Token::Operator(op)) => {
                let r_bp = Parser::prefix_binding_power(&op);
                let rhs = self.parse_expr(r_bp, env)?;
                ParseNode::new(Token::Operator(op), None, vec![rhs])
            }
            _ => panic!("Bad lhs"),
        };
        while let Some(op) = self.peek() {
            let op = match op {
                Token::Operator(x) => x,
                _ => break,
            };
            let (l_bp, r_bp) = Parser::infix_binding_power(&op);
            if l_bp < cur_bp {
                break;
            }
            self.advance();
            let rhs = self.parse_expr(r_bp, env)?;
            lhs = ParseNode::new(Token::Operator(op.clone()), None, vec![lhs, rhs]);
        }
        Ok(lhs)
    }
}
