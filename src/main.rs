extern crate rust_gen_callgraph;

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

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
    let mut args = env::args();
    let _ = args.next();

    let dirname = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: callgraph path/to/dirname");
            process::exit(1);
        }
    };

    let sourcefiles = get_sourcefile(PathBuf::from(dirname)).unwrap();
    rust_gen_callgraph::run(sourcefiles);
}
