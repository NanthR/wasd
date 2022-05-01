use crate::parser::ParseNode;

pub fn get_sexp(parse_node: &ParseNode, level: usize) -> String {
    if parse_node.children.is_empty() {
        format!("{:?}", parse_node.token)
    } else {
        format!(
            "({:?}{}\n{}{})",
            parse_node.token,
            if let Some(p) = &parse_node.extra_info {
                format!("\n\t({})", get_sexp(p, level + 1))
            } else {
                " ".to_string()
            },
            "\t".repeat(level),
            parse_node
                .children
                .iter()
                .map(|x| get_sexp(x, level + 1))
                .collect::<Vec<String>>()
                .join(&format!("\n{}", "\t".repeat(level)))
        )
    }
}
