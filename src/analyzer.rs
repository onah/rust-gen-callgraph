//extern crate syn;
mod callgraph;
mod datas;
mod function;
mod name_resolver;
mod parser_syn;
mod project_infomation;

use self::callgraph::AnalyzerCallGraph;
use self::function::AnalyzerFunction;
use crate::call_info::CallInfo;
use std::error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::visit::Visit;

// TODO: mod.rs wo module mei ni okikaeru

//pub fn analyze(files: &Vec<PathBuf>) -> Result<Vec<CallInfo>, Box<dyn error::Error>> {
pub fn analyze(directory: &PathBuf) -> Result<Vec<CallInfo>, Box<dyn error::Error>> {
    let project_info = project_infomation::ProjectInfomaion::new(directory)?;
    let mut result: Vec<CallInfo> = Vec::new();
    let mut analyzer_funtions = AnalyzerFunction::new(project_info.project_name().to_string());

    let files = project_info.source_files();

    for filename in files {
        let mut file = File::open(filename)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        let syntax = syn::parse_file(&src)?;
        analyzer_funtions.visit_file(&syntax);
    }

    for filename in files {
        //
        let mut funcs = AnalyzerFunction::new(project_info.project_name().to_string());

        let resolver = name_resolver::NameResolver::new(filename)?;
        let mut analyzer = AnalyzerCallGraph::new(resolver);

        let mut file = File::open(filename)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        let syntax = syn::parse_file(&src)?;
        funcs.visit_file(&syntax);
        analyzer.visit_file(&syntax);

        /*
        let tmp1_calls = analyzer.get_callinfo();
        let tmp2_calls;
        for tmp in tmp1_calls {
            let y = None;
            for func in funcs.get_function_list() {
                if tmp.callee = func.func_name() {
                    y = Some(func.name());
                }
            }

            match y {
                Some(x) => tmps2_calls.push(xxx),
                None => xxx,
            }
        }

        */
        let mut calls = analyzer.get_callinfo();
        result.append(&mut calls);
    }

    Ok(result)
}
