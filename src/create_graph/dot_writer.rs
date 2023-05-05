pub struct DotWriter {
    cluster_counter: usize,
}

impl DotWriter {
    pub fn new() -> DotWriter {
        DotWriter { cluster_counter: 0 }
    }

    pub fn start_cluster(&mut self, cluster_name: &str) -> String {
        let mut result = "".to_string();
        result += &format!("subgraph cluster_{} {{\n", self.cluster_counter);
        result += &format!("label=\"{}\"\n", cluster_name);

        self.cluster_counter += 1;

        result
    }

    pub fn end_cluster(&self) -> String {
        "}\n".to_string()
    }
}

pub fn start() -> String {
    String::from("digraph G {\n rankdir=LR;\n")
}

pub fn end() -> String {
    String::from("}\n")
}

pub fn node(name: &str) -> String {
    let id = escape_for_id(name);
    let binding: Vec<&str> = name.split("::").collect();
    let label = binding.last().unwrap_or(&"");
    format!("{} [label=\"{}\"]\n", id, label)
}

pub fn edge(source: &str, dest: &str) -> String {
    format!("{} -> {}\n", escape_for_id(source), escape_for_id(dest))
}

pub fn single_edge(source: &str) -> String {
    format!("{}\n", escape_for_id(source))
}

fn escape_for_id(name: &str) -> String {
    name.replace([':', '-'], "_")
}
