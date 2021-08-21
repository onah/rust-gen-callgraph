extern crate rust_gen_callgraph;

use std::fs;
use std::io;
use std::path::PathBuf;

fn get_sourcefile(path: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut result: Vec<PathBuf> = Vec::new();

    let dirfiles = fs::read_dir(path)?;
    for item in dirfiles.into_iter() {
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

fn main() {
    let sourcefiles = get_sourcefile(PathBuf::from(".")).unwrap();
    rust_gen_callgraph::run(sourcefiles);
}
