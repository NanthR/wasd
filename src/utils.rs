use crate::parser::ParseNode;

pub fn get_sexp(parse_node: &ParseNode) -> String {
    if parse_node.children.is_empty() {
        format!("{:?}", parse_node.current)
    } else {
        format!(
            "({:?} {})",
            parse_node.current,
            parse_node.children.iter().map(|x| get_sexp(&x)).collect::<Vec<String>>().join(" ")
        )
    }
}
