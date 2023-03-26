mod create_dot_graph;
mod dot_writer;
mod module_tree;

use crate::call_data::CallInfo;
use create_dot_graph::CreateDotGraph;
use module_tree::ModuleTree;
use std::io;

pub fn render_to<W: io::Write>(
    callinfos: Vec<CallInfo>,
    output: &mut W,
    print_data_type: bool,
) -> io::Result<()> {
    let class_tree = make_module_tree(&callinfos);
    let create_dot_graph = CreateDotGraph::new(callinfos, print_data_type);

    output.write_all(dot_writer::start().as_bytes())?;
    output.write_all(create_dot_graph.write_node_label().as_bytes())?;

    class_tree.search_preorder(&create_dot_graph);
    output.write_all(create_dot_graph.borrow_result().as_bytes())?;
    create_dot_graph.borrow_mut_result().clear();
    //    output.write_all(create_dot_graph.result.borrow().as_bytes())?;
    //    create_dot_graph.result.borrow_mut().clear();

    output.write_all(create_dot_graph.write_callinfo().as_bytes())?;
    output.write_all(dot_writer::end().as_bytes())?;

    Ok(())
}

fn make_module_tree(callinfo: &[CallInfo]) -> ModuleTree {
    let module_tree = ModuleTree::new();
    for c in callinfo.iter() {
        let mut fn_names_caller: Vec<&str> = c.caller.split("::").collect();
        fn_names_caller.pop().unwrap();
        module_tree.push(&fn_names_caller);

        let mut fn_names_callee: Vec<&str> = c.callee.split("::").collect();
        fn_names_callee.pop().unwrap();
        module_tree.push(&fn_names_callee);
    }
    module_tree
}
