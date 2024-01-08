mod analyzer;
mod call_data;
mod create_graph;
mod struct_name;

use std::error;
use std::io;
use std::path::PathBuf;

pub fn run(directory: PathBuf, print_data_type: bool) -> Result<(), Box<dyn error::Error>> {
    // Analyze source code
    let calls = analyzer::analyze(&directory)?;

    // Create graph
    let mut f = io::BufWriter::new(io::stdout());
    create_graph::render_to(calls, &mut f, print_data_type)?;

    Ok(())
}
