//extern crate syn;
mod callgraph;
mod function;
mod name_resolver;
mod parser_syn;

use self::callgraph::AnalyzerCallGraph;
use self::function::AnalyzerFunction;
use super::CallInfo;
use std::error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::visit::Visit;

// TODO: mod.rs wo module mei ni okikaeru

pub fn analyze(files: &Vec<PathBuf>) -> Result<Vec<CallInfo>, Box<dyn error::Error>> {
    let mut result: Vec<CallInfo> = Vec::new();
    let mut analyzer_funtions = AnalyzerFunction::new();

    for filename in files {
        let mut file = File::open(filename)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        let syntax = syn::parse_file(&src)?;
        analyzer_funtions.visit_file(&syntax);
    }

    for filename in files {
        let module_name = get_module_name(filename)?;
        let mut analyzer = AnalyzerCallGraph::new(module_name);

        let mut file = File::open(filename)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        let syntax = syn::parse_file(&src)?;
        analyzer.visit_file(&syntax);

        let mut calls = analyzer.get_callinfo();

        result.append(&mut calls);
    }

    Ok(result)
}

fn get_module_name(file_name: &PathBuf) -> Result<String, Box<dyn error::Error>> {
    let result;

    if file_name == OsStr::new("./src/lib.rs") {
        let mut f = File::open("Cargo.toml")?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        let values = contents.parse::<toml::Value>()?;
        let project_name = values["package"]["name"].as_str().unwrap_or("");
        result = String::from(project_name);
    } else if file_name.file_name().unwrap_or_else(|| OsStr::new("")) == OsStr::new("lib.rs") {
        let mut filename2 = file_name.clone();
        filename2.pop();
        if filename2.file_name().unwrap() == OsStr::new("src") {
            filename2.pop();
            {
                // TODO Copy Code Refactoring
                let mut f = File::open(format!("{}/Cargo.toml", filename2.to_str().unwrap()))?;
                let mut contents = String::new();
                f.read_to_string(&mut contents)?;

                let values = contents.parse::<toml::Value>()?;
                let project_name = values["package"]["name"].as_str().unwrap();

                result = String::from(project_name);
            }
        } else {
            result = filename2.file_stem().unwrap().to_str().unwrap().to_string();
        }
    } else {
        result = file_name.file_stem().unwrap().to_str().unwrap().to_string();
    }

    Ok(result)
}
