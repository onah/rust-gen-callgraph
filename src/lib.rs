mod analyzer;
mod create_graph;

use std::path::PathBuf;

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

    println!("{:#?}", calls);

    /*
    let graph = create_graph::GraphData {
    }
    */
}
