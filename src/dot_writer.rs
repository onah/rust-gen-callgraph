pub fn start() -> String {
    String::from("digraph G {\n rankdir=LR;\n")
}

pub fn end() -> String {
    String::from("}\n")
}

pub fn node(name: &str) -> String {
    let id = escape_for_id(name);
    format!("{} [label=\"{}\"]\n", id, name)
}

pub fn edge(source: &str, dest: &str) -> String {
    format!("{} -> {}\n", escape_for_id(source), escape_for_id(dest))
}

fn escape_for_id(name: &str) -> String {
    name.replace([':', '-'], "_")
}
