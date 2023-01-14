mod analyzer;
mod class_tree;
mod create_graph;

use std::fs::File;
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

#[derive(Debug)]
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

    println!("digraph G {{");
    println!("}}");
}
