mod analyzer;
mod create_dot_graph;
mod dot_writer;
mod struct_name;
mod struct_tree;

use std::io;
use std::path::PathBuf;

/*
#[derive(Debug)]
pub struct FunctionInfo {
    path: Vec<String>,
    name: String,
}

#[derive(Debug)]
pub struct CallInfo {
    callee: FunctionInfo,
    caller: FunctionInfo,
}
*/

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct CallInfo {
    callee: String,
    caller: String,
}

pub fn run(input: Vec<PathBuf>) {
    let mut calls: Vec<CallInfo> = Vec::new();
    for path in input {
        calls.append(&mut analyzer::analyze(path).unwrap());
    }

    //println!("{:#?}", calls);

    // let graph = create_graph::GraphData::new(calls);
    // println!("{:#?}", graph);

    // let mut f = File::create("graph.dot").unwrap();
    // create_graph::render_to(graph, &mut f);

    //let mut f = File::create("graph.dot").unwrap();
    let mut f = io::BufWriter::new(io::stdout());

    create_dot_graph::render_to(calls, &mut f).unwrap();
}
