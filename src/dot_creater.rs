pub fn start() -> String {
    String::from("digraph G {{\n")
}

pub fn end() -> String {
    String::from("}}\n")
}

pub fn node(name: &str) -> String {
    let id = name.replace(":", "_").replace("-", "_");
    format!("{} [label=\"{}\"]\n", id, name)
}
