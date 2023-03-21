extern crate syn;
mod callgraph;
mod function;
mod name_resolver;
mod parser_syn;

use self::callgraph::Analyzer;
use self::function::AnalyzerFunction;
use super::CallInfo;
use std::error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::visit::Visit;

// TODO: mod.rs wo module mei ni okikaeru

// pub fn analyze(filename: PathBuf) -> Result<Vec<CallInfo>, Box<dyn error::Error>> {
pub fn analyze(files: &Vec<PathBuf>) -> Result<Vec<CallInfo>, Box<dyn error::Error>> {
    let mut result: Vec<CallInfo> = Vec::new();
    let mut analyzer_funtions = AnalyzerFunction::new();

    for filename in files {
        let mut file = File::open(&filename)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        let syntax = syn::parse_file(&src)?;
        analyzer_funtions.visit_file(&syntax);
    }

    for filename in files {
        let mut analyzer = Analyzer::new(filename.clone())?;

        let mut file = File::open(&filename)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        let syntax = syn::parse_file(&src)?;
        //analyzer.status.filename = Some(filename.clone());
        analyzer.visit_file(&syntax);
        //analyzer.status.filename = None;

        let mut calls = analyzer.get_callinfo();

        result.append(&mut calls);
    }

    Ok(result)
}
