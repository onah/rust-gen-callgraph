mod dot_writer;
mod graph_creator;
mod module_tree;

use crate::call_info::CallInfo;
use graph_creator::ClusterDataType;
use graph_creator::DotGraphBuilder;
use module_tree::ClassPathTree;
use std::io;

pub fn render_to<W: io::Write>(callinfos: Vec<CallInfo>, output: &mut W) -> io::Result<()> {
    let classpath_tree = make_classpath_tree(&callinfos);
    //println!("{:?}", classpath_tree);

    let create_dot_graph = DotGraphBuilder::new(callinfos);

    let mut dot_writer = dot_writer::DotWriter::new();

    output.write_all(dot_writer::start().as_bytes())?;
    //output.write_all(create_dot_graph.write_node_label().as_bytes())?;
    for pathname in create_dot_graph.get_all_node() {
        let node = dot_writer::node(&pathname);
        output.write_all(node.as_bytes())?;
    }

    classpath_tree.search_preorder(&create_dot_graph);

    for cluster_data in &*create_dot_graph.borrow_result() {
        let cluster = dot_writer.start_cluster(cluster_data.get_cluseter_name());
        output.write_all(cluster.as_bytes())?;

        for c in cluster_data.get_nodes() {
            match c {
                ClusterDataType::Single(single) => {
                    let name = dot_writer::single_edge(single);
                    output.write_all(name.as_bytes())?;
                }
                ClusterDataType::CallInfo(callinfo) => {
                    let edge = dot_writer::edge(&callinfo.callee, &callinfo.caller);
                    output.write_all(edge.as_bytes())?;
                }
            }
        }
        output.write_all(dot_writer.end_cluster().as_bytes())?;
    }

    create_dot_graph.borrow_mut_result().clear();

    output.write_all(create_dot_graph.write_callinfo().as_bytes())?;
    output.write_all(dot_writer::end().as_bytes())?;

    Ok(())
}

fn make_classpath_tree(callinfo: &[CallInfo]) -> ClassPathTree {
    let module_tree = ClassPathTree::new();
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
