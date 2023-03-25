mod analyzer;
mod create_graph;
mod module_tree;
mod struct_name;

use std::error;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct CallInfo {
    callee: String,
    caller: String,
}

pub fn run(directory: PathBuf, print_data_type: bool) -> Result<(), Box<dyn error::Error>> {
    let sourcefiles = get_sourcefile(directory)?;
    let calls = analyzer::analyze(&sourcefiles)?;

    let mut f = io::BufWriter::new(io::stdout());
    create_graph::render_to(calls, &mut f, print_data_type)?;

    Ok(())
}

/// create a file list from the specified directory.
/// Find files with extension rs recursively.
fn get_sourcefile(path: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut result: Vec<PathBuf> = Vec::new();

    let dirfiles = fs::read_dir(path)?;
    for item in dirfiles {
        let dirfile = item?;

        // recursive to directory
        if dirfile.metadata()?.is_dir() {
            result.append(&mut get_sourcefile(dirfile.path())?);
        }

        if let Some(v) = dirfile.path().extension() {
            if v == "rs" {
                result.push(dirfile.path());
            }
        }
    }

    Ok(result)
}
