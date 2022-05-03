use crate::lexer::{Operator, TokenType};
use crate::parser::ParseNode;

pub struct Transpiler {
    nodes: Vec<ParseNode>,
}

impl Transpiler {
    pub fn new(nodes: Vec<ParseNode>) -> Self {
        Self { nodes }
    }

    pub fn transpile(&self) -> Result<String, String> {
        let mut main_func = vec![];
        for node in self.nodes.iter() {
            main_func.push(Transpiler::convert_to_python(node, 0)?);
        }
        Ok(main_func.join("\n"))
    }

    fn convert_to_python(node: &ParseNode, level: usize) -> Result<String, String> {
        match &node.token {
            TokenType::Identifier(x) => Ok(x.clone()),
            TokenType::Number(x) => Ok(x.to_string()),
            TokenType::StringLiteral(x) => Ok(x.clone()),
            TokenType::Let => {
                if let TokenType::Identifier(id) =
                    &node.children.get(0).ok_or("Invalid state")?.token
                {
                    let value = node.children.get(1).ok_or("Invalid state")?;
                    let res = Transpiler::convert_to_python(value, level)?;
                    Ok(format!("{}{} = {}", " ".repeat(level), id, res))
                } else {
                    Err("Invalid error".to_string())
                }
            }
            TokenType::Operator(op) => {
                let lhs_parsed = node.children.get(0).ok_or("Invalid state")?;
                let lhs = Transpiler::convert_to_python(lhs_parsed, level)?;

                let rhs_parsed = node.children.get(1).ok_or("Invalid state")?;
                let rhs = Transpiler::convert_to_python(rhs_parsed, level)?;

                Ok(format!("{} {} {}", lhs, op, rhs))
            }
            TokenType::While => {
                let cond =
                    Transpiler::convert_to_python(&node.extra_info.as_ref().unwrap(), level)?;
                let mut res = vec![];
                for statement in node.children.iter() {
                    res.push(format!(
                        "{}{}",
                        " ".repeat(level),
                        Transpiler::convert_to_python(&statement, level + 1)?
                    ));
                }
                let statements = res.join("\n");
                Ok(format!(
                    "{}while {}:\n{} ",
                    " ".repeat(level),
                    cond,
                    statements
                ))
            }
            TokenType::If => {
                let cond =
                    Transpiler::convert_to_python(&node.extra_info.as_ref().unwrap(), level)?;
                let mut res = vec![];
                for children in node.children.iter() {
                    let f = match children.token {
                        TokenType::Elif => {
                            let mut elif_res = vec![];
                            for statement in children.children.iter() {
                                elif_res.push(format!(
                                    "{}",
                                    Transpiler::convert_to_python(&statement, level + 1)?
                                ))
                            }
                            let elif_cond = Transpiler::convert_to_python(
                                &children.extra_info.as_ref().unwrap(),
                                level,
                            )?;
                            format!(
                                "{}elif {}:\n{}",
                                " ".repeat(level),
                                elif_cond,
                                elif_res.join("\n")
                            )
                        }
                        TokenType::Else => {
                            let mut else_rus = vec![];
                            for statement in children.children.iter() {
                                else_rus.push(format!(
                                    "{}",
                                    Transpiler::convert_to_python(&statement, level + 1)?
                                ));
                            }
                            format!("{}else:\n{}", " ".repeat(level), else_rus.join("\n"))
                        }
                        _ => Transpiler::convert_to_python(children, level + 1)?,
                    };
                    res.push(f);
                }
                Ok(format!(
                    "{}if {}:\n{}",
                    " ".repeat(level),
                    cond,
                    res.join("\n")
                ))
            }
            TokenType::Print => Ok(format!(
                "{}print({})",
                " ".repeat(level),
                Transpiler::convert_to_python(node.children.get(0).unwrap(), level)?
            )),
            _ => Ok("".to_string()),
        }
    }
}
