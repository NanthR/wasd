use crate::parser::ParseNode;

pub fn get_sexp(parse_node: &ParseNode) -> String {
    if parse_node.children.is_empty() {
        format!("{:?}", parse_node.token)
    } else {
        format!(
            "({:?}{}{})",
            parse_node.token,
            if let Some(p) = &parse_node.extra_info {
                format!("({})", get_sexp(p))
            } else {
                " ".to_string()
            },
            parse_node
                .children
                .iter()
                .map(get_sexp)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
