use super::class_name::ClassName;
use super::CallInfo;
use crate::class_tree::{self, ClassTreeInterface};
use crate::dot_writer;
//use dot_writer;
use std::cell::RefCell;
use std::collections::HashSet;
use std::io;

struct CallInfoWithWrited {
    callinfo: CallInfo,
    writed: RefCell<bool>,
}
struct CreateDotGraph {
    callinfos: RefCell<Vec<CallInfoWithWrited>>,
    cluster_counter: RefCell<usize>,
    current_classname: RefCell<ClassName>,
    result: RefCell<String>,
}

impl CreateDotGraph {
    pub fn new(callinfos: Vec<CallInfo>) -> CreateDotGraph {
        let mut callinfos_with_writed: Vec<CallInfoWithWrited> = Vec::new();

        // remove duplicates
        let mut callinfos_no_dup: HashSet<CallInfo> = HashSet::new();
        for c in callinfos {
            callinfos_no_dup.insert(c);
        }

        for c in callinfos_no_dup {
            let cww = CallInfoWithWrited {
                callinfo: c,
                writed: RefCell::new(false),
            };
            callinfos_with_writed.push(cww);
        }
        CreateDotGraph {
            callinfos: RefCell::new(callinfos_with_writed),
            cluster_counter: RefCell::new(0),
            current_classname: RefCell::new(ClassName::new()),
            result: RefCell::new(String::new()),
        }
    }

    pub fn write_callinfo(&self) -> String {
        let mut result = String::new();

        for callinfo in &*self.callinfos.borrow() {
            if !(*callinfo.writed.borrow()) {
                result += &dot_writer::edge(&callinfo.callinfo.caller, &callinfo.callinfo.callee);
            }
        }
        result
    }

    pub fn write_node_label(&self) -> String {
        let mut result = String::new();

        for callinfo in &*self.callinfos.borrow() {
            result += &dot_writer::node(&callinfo.callinfo.callee);
            result += &dot_writer::node(&callinfo.callinfo.caller);
        }
        result
    }
}

impl ClassTreeInterface for CreateDotGraph {
    fn exec_search_before(&self, fn_name: &str) -> bool {
        self.current_classname.borrow_mut().push(fn_name);

        let mut result = self.result.borrow_mut();

        *result += &format!("subgraph cluster_{} {{\n", self.cluster_counter.borrow());
        *result += &format!("label=\"{}\"\n", self.current_classname.borrow().name());

        for callinfo in &*self.callinfos.borrow() {
            if callinfo
                .callinfo
                .callee
                .starts_with(&self.current_classname.borrow().name())
            {
                if callinfo
                    .callinfo
                    .caller
                    .starts_with(&self.current_classname.borrow().name())
                {
                    *callinfo.writed.borrow_mut() = true;

                    let callee_name = &callinfo.callinfo.callee.replace([':', '-'], "_");
                    let caller_name = &callinfo.callinfo.caller.replace([':', '-'], "_");
                    *result += &format!("{} -> {}\n", caller_name, callee_name);
                } else {
                    let callee_name = &callinfo.callinfo.callee.replace([':', '-'], "_");
                    *result += &format!("{}\n", callee_name);
                }
            } else if callinfo
                .callinfo
                .caller
                .starts_with(&self.current_classname.borrow().name())
            {
                let caller_name = &callinfo.callinfo.caller.replace([':', '-'], "_");
                *result += &format!("{}\n", caller_name);
            }
        }

        *self.cluster_counter.borrow_mut() += 1;
        true
    }

    fn exec_search_after(&self, _fn_name: &str) -> bool {
        self.current_classname.borrow_mut().pop().unwrap();

        let mut result = self.result.borrow_mut();
        *result += "}\n";

        true
    }
}

pub fn render_to<W: io::Write>(callinfos: Vec<CallInfo>, output: &mut W) -> io::Result<()> {
    let class_tree = make_class_tree(&callinfos);
    let create_dot_graph = CreateDotGraph::new(callinfos);

    output.write_all(dot_writer::start().as_bytes())?;
    output.write_all(create_dot_graph.write_node_label().as_bytes())?;

    class_tree.search_preorder(&create_dot_graph);
    output.write_all(create_dot_graph.result.borrow().as_bytes())?;
    create_dot_graph.result.borrow_mut().clear();

    output.write_all(create_dot_graph.write_callinfo().as_bytes())?;
    output.write_all(dot_writer::end().as_bytes())?;

    Ok(())
}

fn make_class_tree(callinfo: &[CallInfo]) -> class_tree::ClassTree {
    let class_tree = class_tree::ClassTree::new();
    for c in callinfo.iter() {
        let mut fn_names_caller: Vec<&str> = c.caller.split("::").collect();
        fn_names_caller.pop().unwrap();
        class_tree.push(&fn_names_caller);

        let mut fn_names_callee: Vec<&str> = c.callee.split("::").collect();
        fn_names_callee.pop().unwrap();
        class_tree.push(&fn_names_callee);
    }
    class_tree
}
