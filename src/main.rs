use clap::Parser;
use std::path::PathBuf;

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
    if let Err(err) = rust_gen_callgraph::run(args.dirname, args.print_data_type) {
        eprintln!("{}", err);
        std::process::exit(2);
    }
    std::process::exit(0);
}
