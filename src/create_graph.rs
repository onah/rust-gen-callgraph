use super::CallInfo;
use std::borrow::Cow;
use std::collections::HashSet;
use std::io::Write;

type Nd = String;
type Ed = (String, String);

pub struct GraphData {
    data: HashSet<Ed>,
}

impl GraphData {
    pub fn new(input: Vec<CallInfo>) -> GraphData {
        let mut data: HashSet<Ed> = HashSet::new();
        for callinfo in input.iter() {
            data.insert((callinfo.caller.clone(), callinfo.callee.clone()));
        }

        GraphData { data }
    }
}

pub fn render_to<W: Write>(input: GraphData, output: &mut W) {
    dot::render(&input, output).unwrap();
}

impl<'a> dot::Labeller<'a, Nd, Ed> for GraphData {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("Example").unwrap()
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        let id = n.replace(":", "_");
        dot::Id::new(id).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(format!("{}", *n).into())
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed> for GraphData {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        let mut nodes: Vec<Nd> = Vec::new();
        for (i, j) in self.data.iter() {
            nodes.push(i.clone());
            nodes.push(j.clone());
        }
        nodes.sort();
        nodes.dedup();

        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed> {
        let mut edges: Vec<Ed> = Vec::new();
        for (i, j) in self.data.iter() {
            edges.push((i.clone(), j.clone()));
        }

        Cow::Owned(edges)
    }

    fn source(&self, e: &Ed) -> Nd {
        e.0.clone()
    }

    fn target(&self, e: &Ed) -> Nd {
        e.1.clone()
    }
}
