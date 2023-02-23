use super::class_name::ClassName;
use super::CallInfo;
use crate::class_tree::{self, ClassTreeInterface};
use crate::dot_creater;
//use dot_creater;
use std::cell::RefCell;
use std::collections::HashSet;
use std::io;

struct CallInfoWithWrited {
    callinfo: CallInfo,
    writed: RefCell<bool>,
}
struct CreateDotGraph<W> {
    output: RefCell<W>,
    callinfos: RefCell<Vec<CallInfoWithWrited>>,
    cluster_counter: RefCell<usize>,
    current_classname: RefCell<ClassName>,
}

impl<W: io::Write> CreateDotGraph<W> {
    pub fn new(output: W, callinfos: Vec<CallInfo>) -> CreateDotGraph<W> {
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
            output: RefCell::new(output),
            callinfos: RefCell::new(callinfos_with_writed),
            cluster_counter: RefCell::new(0),
            current_classname: RefCell::new(ClassName::new()),
        }
    }

    pub fn write_callinfo(&self) {
        for callinfo in &*self.callinfos.borrow() {
            if *callinfo.writed.borrow() == false {
                let callee_name = &callinfo.callinfo.callee.replace(":", "_").replace("-", "_");
                let caller_name = &callinfo.callinfo.caller.replace(":", "_").replace("-", "_");

                self.output
                    .borrow_mut()
                    .write(format!("{} -> {}\n", caller_name, callee_name).as_bytes())
                    .unwrap();
            }
        }
    }

    pub fn write_node_label(&self) {
        for callinfo in &*self.callinfos.borrow() {
            let mut output = self.output.borrow_mut();
            output
                .write(dot_creater::node(&callinfo.callinfo.callee).as_bytes())
                .unwrap();
            output
                .write(dot_creater::node(&callinfo.callinfo.caller).as_bytes())
                .unwrap();
        }
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<()> {
        self.output.borrow_mut().write(buf)?;
        Ok(())
    }
}

impl<W: io::Write> ClassTreeInterface for CreateDotGraph<W> {
    fn exec_search_before(&self, fn_name: &str) {
        self.current_classname.borrow_mut().push(fn_name);

        let mut w = self.output.borrow_mut();
        w.write(format!("subgraph cluster_{} {{\n", self.cluster_counter.borrow()).as_bytes())
            .unwrap();
        w.write(format!("label=\"{}\"\n", self.current_classname.borrow().name()).as_bytes())
            .unwrap();

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

                    let callee_name = &callinfo.callinfo.callee.replace(":", "_").replace("-", "_");
                    let caller_name = &callinfo.callinfo.caller.replace(":", "_").replace("-", "_");
                    w.write(format!("{} -> {}\n", caller_name, callee_name).as_bytes())
                        .unwrap();
                } else {
                    let callee_name = &callinfo.callinfo.callee.replace(":", "_").replace("-", "_");
                    w.write(format!("{}\n", callee_name).as_bytes()).unwrap();
                }
            } else {
                if callinfo
                    .callinfo
                    .caller
                    .starts_with(&self.current_classname.borrow().name())
                {
                    let caller_name = &callinfo.callinfo.caller.replace(":", "_").replace("-", "_");
                    w.write(format!("{}\n", caller_name).as_bytes()).unwrap();
                }
            }
        }

        *self.cluster_counter.borrow_mut() += 1;
    }

    fn exec_search_after(&self, _fn_name: &str) {
        self.current_classname.borrow_mut().pop().unwrap();
        self.output
            .borrow_mut()
            .write(format!("}}\n").as_bytes())
            .unwrap();
    }
}

pub fn render_to<W: io::Write>(callinfos: Vec<CallInfo>, output: &mut W) -> io::Result<()> {
    let class_tree = make_class_tree(&callinfos);
    let create_dot_graph = CreateDotGraph::new(output, callinfos);

    create_dot_graph.write(dot_creater::start().as_bytes())?;
    create_dot_graph.write_node_label();
    class_tree.search_preorder(&create_dot_graph);
    create_dot_graph.write_callinfo();
    create_dot_graph.write(dot_creater::end().as_bytes())?;

    Ok(())
}

fn make_class_tree(callinfo: &Vec<CallInfo>) -> class_tree::ClassTree {
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
