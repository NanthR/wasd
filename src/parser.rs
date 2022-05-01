use crate::lexer::{Operator, Token};

pub struct Parser {
    tokens: Vec<Token>,
    cur: usize,
}

#[derive(Debug)]
pub struct ParseNode {
    pub token: Token,
    pub children: Vec<ParseNode>,
}

impl ParseNode {
    fn new(token: Token, children: Vec<ParseNode>) -> Self {
        Self { token, children }
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
        while self.peek().is_some() {
            let node = self.parse_decl()?;
            res.push(node)
        }
        Ok(res)
    }

    fn parse_decl(&mut self) -> Result<ParseNode, String> {
        match self.peek() {
            Some(t) => match t {
                Token::Let => {
                    if let (Some(id), Some(Token::Operator(Operator::Equal))) =
                        (self.peek_n(1), self.peek_n(2))
                    {
                        self.advance();
                        self.advance();
                        self.advance();
                        println!("{:?}", self.peek());
                        if let Some(Token::StringLiteral(x)) = self.peek() {
                            self.advance();
                            Ok(ParseNode::new(
                                Token::Let,
                                vec![
                                    ParseNode::new(id.clone(), vec![]),
                                    ParseNode::new(Token::StringLiteral(x.to_string()), vec![]),
                                ],
                            ))
                        } else {
                            let node = self.parse_expr(0)?;
                            Ok(ParseNode::new(
                                Token::Let,
                                vec![ParseNode::new(id.clone(), vec![]), node],
                            ))
                        }
                    } else {
                        Err("Invalid decl".to_string())
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
            Some(t) => ParseNode::new(t.clone(), vec![]),
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
            lhs = ParseNode::new(Token::Operator(op.clone()), vec![lhs, rhs]);
        }
        Ok(lhs)
    }
}
