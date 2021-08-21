mod analyzer;
use std::path::PathBuf;

pub struct CallInfo {
    caller: String,
    callee: String,
}

pub fn run(input: Vec<PathBuf>) {
    let calls: Vec<CallInfo> = Vec::new();
    for path in input {
        analyzer::analyze(path).unwrap();
    }
}
