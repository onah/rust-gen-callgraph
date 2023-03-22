use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;

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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path for the analyze directory.
    dirname: PathBuf,
    /// print for data type. (default ignore) ex. Vec, String.
    #[arg(long)]
    print_data_type: bool,
}

fn main() {
    let args = Args::parse();

    let sourcefiles = get_sourcefile(PathBuf::from(args.dirname)).unwrap();
    rust_gen_callgraph::run(sourcefiles, args.print_data_type);
}
