use crate::lexer::{Operator, Token};

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

    fn advance(&mut self) {
        self.cur += 1;
    }

    pub fn parse(&mut self) -> Result<Vec<ParseNode>, String> {
        let res = self.parse_statement()?;
        if self.peek().is_none() {
            Ok(res)
        } else {
            Err("Invalid parse".to_string())
        }
    }

    fn parse_statement(&mut self) -> Result<Vec<ParseNode>, String> {
        let mut res: Vec<ParseNode> = vec![];
        while self.peek().is_some() && self.peek() != Some(Token::RightCurly) {
            let node = self.parse_decl()?;
            res.push(node)
        }
        Ok(res)
    }

    fn parse_decl(&mut self) -> Result<ParseNode, String> {
        match self.peek() {
            Some(t) => match t {
                Token::Let => {
                    if let (Some(Token::Identifier(id)), Some(Token::Operator(Operator::Equal))) =
                        (self.peek_n(1), self.peek_n(2))
                    {
                        self.advance();
                        self.advance();
                        self.advance();
                        if let Some(Token::StringLiteral(x)) = self.peek() {
                            self.advance();
                            Ok(ParseNode::new(
                                Token::Let,
                                None,
                                vec![
                                    ParseNode::new(Token::Identifier(id), None, vec![]),
                                    ParseNode::new(
                                        Token::StringLiteral(x),
                                        None,
                                        vec![],
                                    ),
                                ],
                            ))
                        } else {
                            let node = self.parse_expr(0)?;
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
                    self.advance();
                    if let Some(Token::LeftParen) = self.peek() {
                        self.advance();
                        let node = self.parse_expr(0)?;
                        if let Some(Token::RightParen) = self.peek() {
                            self.advance();
                            if let Some(Token::LeftCurly) = self.peek() {
                                self.advance();
                                let nodes = self.parse_statement()?;
                                if let Some(Token::RightCurly) = self.peek() {
                                    self.advance();
                                    Ok(ParseNode::new(Token::While, Some(Box::new(node)), nodes))
                                } else {
                                    Err("Missing right curly in while loop".to_string())
                                }
                            } else {
                                Err("Missing left curly in while loop".to_string())
                            }
                        } else {
                            Err(
                                "While conditon doesn't have a closing paranthesis".to_string()
                            )
                        }
                    } else {
                        Err("Missing open paranthesis in while loop".to_string())
                    }
                }
                Token::If => {
                    self.advance();
                    if let Some(Token::LeftParen) = self.peek() {
                        self.advance();
                        let if_cond = self.parse_expr(0)?;
                        if self.peek() != Some(Token::RightParen) {
                            return Err("If condition not closed".to_string());
                        }
                        self.advance();
                        if self.peek() != Some(Token::LeftCurly) {
                            return Err("If condition must start with curly braces".to_string());
                        }
                        self.advance();
                        let mut nodes = self.parse_statement()?;
                        if self.peek() != Some(Token::RightCurly) {
                            return Err("If block not closed".to_string());
                        }
                        self.advance();
                        let mut res = vec![];
                        while let Some(Token::Elif) = self.peek() {
                            self.advance();
                            if self.peek() != Some(Token::LeftParen) {
                                return Err(
                                    "Elif condition must start with paranthesis".to_string()
                                );
                            }
                            self.advance();
                            let elif_cond = self.parse_expr(0)?;
                            if self.peek() != Some(Token::RightParen) {
                                return Err("Elif condition not closed".to_string());
                            }
                            self.advance();
                            if self.peek() != Some(Token::LeftCurly) {
                                return Err("Elif bloc must start with curly braces".to_string());
                            }
                            self.advance();
                            let statements = self.parse_statement()?;
                            if let Some(Token::RightCurly) = self.peek() {
                                self.advance();
                                res.push(ParseNode::new(
                                    Token::Elif,
                                    Some(Box::new(elif_cond)),
                                    statements,
                                ))
                            } else {
                                return Err("Elif block not closed".to_string());
                            }
                        }
                        if self.peek() == Some(Token::Else) {
                            self.advance();
                            if self.peek() != Some(Token::LeftCurly) {
                                return Err("Else bloc must start with curly braces".to_string());
                            }
                            self.advance();
                            let statements = self.parse_statement()?;
                            if self.peek() != Some(Token::RightCurly) {
                                return Err("Else block not closed".to_string());
                            }
                            self.advance();
                            res.push(ParseNode::new(Token::Else, None, statements))
                        }
                        nodes.append(&mut res);
                        Ok(ParseNode::new(Token::If, Some(Box::new(if_cond)), nodes))
                    } else {
                        Err("If condition must start with an open paranthesis".to_string())
                    }
                }
                _ => Err("Not implemented".to_string()),
            },
            _ => Err("Couldn't be parsed".to_string()),
        }
    }

    fn infix_binding_power(op: &Operator) -> (u8, u8) {
        match op {
            Operator::Plus | Operator::Minus => (1, 2),
            Operator::Multiply | Operator::Divide => (3, 4),
            _ => panic!("Bad"),
        }
    }

    fn parse_expr(&mut self, cur_bp: u8) -> Result<ParseNode, String> {
        let mut lhs = match self.peek() {
            Some(t) => ParseNode::new(t, None, vec![]),
            _ => panic!("Bad lhs"),
        };
        self.advance();
        while let Some(op) = self.peek() {
            if op == Token::SemiColon {
                break;
            }
            let op = match op {
                Token::Operator(x) => x,
                _ => break,
            };
            let (l_bp, r_bp) = Parser::infix_binding_power(&op);
            if l_bp < cur_bp {
                break;
            }
            self.advance();
            let rhs = self.parse_expr(r_bp)?;
            lhs = ParseNode::new(Token::Operator(op.clone()), None, vec![lhs, rhs]);
        }
        Ok(lhs)
    }
}
