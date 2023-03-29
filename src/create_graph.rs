mod create_dot_graph;
mod dot_writer;
mod module_tree;

use crate::call_data::CallInfo;
use create_dot_graph::ClusterDataType;
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

    let mut cluster_counter = 1;
    for cluster_data in &*create_dot_graph.borrow_result() {
        // *result += &format!("subgraph cluster_{} {{\n", self.cluster_counter.borrow());
        // *result += &format!("label=\"{}\"\n", self.current_classname.borrow().name());
        output.write_all(format!("subgraph cluster_{} {{\n", cluster_counter).as_bytes())?;
        output.write_all(format!("label=\"{}\"\n", cluster_data.get_cluseter_name()).as_bytes())?;

        for c in cluster_data.get_nodes() {
            match c {
                ClusterDataType::Single(single) => {
                    //let caller_name = &callinfo.callinfo.caller.replace([':', '-'], "_");
                    //*result += &format!("{}\n", caller_name);

                    let name = single.name().replace([':', '-'], "_");
                    output.write_all(format!("{}\n", name).as_bytes())?;
                }
                ClusterDataType::CallInfo(callinfo) => {
                    //let callee_name = &callinfo.callinfo.callee.replace([':', '-'], "_");
                    //let caller_name = &callinfo.callinfo.caller.replace([':', '-'], "_");
                    //*result += &format!("{} -> {}\n", caller_name, callee_name);
                    let callee_name = &callinfo.callee.replace([':', '-'], "_");
                    let caller_name = &callinfo.caller.replace([':', '-'], "_");
                    output.write_all(format!("{} -> {}\n", caller_name, callee_name).as_bytes())?;
                }
            }
        }
        //let mut result = self.result.borrow_mut();
        //*result += "}\n";
        output.write_all(format!("}}\n").as_bytes())?;
        cluster_counter += 1;
    }

    //output.write_all(create_dot_graph.borrow_result().as_bytes())?;
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
