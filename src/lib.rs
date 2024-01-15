mod analyzer;
mod call_data;
mod create_graph;
mod filter;

use std::error;
use std::io;
use std::path::PathBuf;

pub fn run(directory: PathBuf, print_data_type: bool) -> Result<(), Box<dyn error::Error>> {
    // Analyze source code
    let callinfo_list = analyzer::analyze(&directory)?;

    // Filterling data
    let filter_options = filter::Options::new(print_data_type);
    let callinfo_list = filter::filterling(callinfo_list, &filter_options);

    // Create graph
    let mut f = io::BufWriter::new(io::stdout());
    create_graph::render_to(callinfo_list, &mut f)?;

    Ok(())
}
