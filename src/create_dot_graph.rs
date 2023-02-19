use super::class_name::ClassName;
use super::CallInfo;
use crate::class_tree::{self, ClassTreeInterface};
use std::cell::RefCell;

//use std::intrinsics::caller_location;
use std::io;

struct CreateDotGraph<W> {
    output: RefCell<W>,
    callinfos: RefCell<Vec<CallInfo>>,
    cluster_counter: RefCell<usize>,
    current_classname: RefCell<ClassName>,
}

impl<W: io::Write> CreateDotGraph<W> {
    pub fn new(output: W, callinfos: Vec<CallInfo>) -> CreateDotGraph<W> {
        CreateDotGraph {
            output: RefCell::new(output),
            callinfos: RefCell::new(callinfos),
            cluster_counter: RefCell::new(0),
            current_classname: RefCell::new(ClassName::new()),
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

        // kokode graph wo kaku
        for callinfo in &*self.callinfos.borrow() {
            if callinfo
                .callee
                .starts_with(&self.current_classname.borrow().name())
            {
                if callinfo
                    .caller
                    .starts_with(&self.current_classname.borrow().name())
                {
                    let callee_name = &callinfo.callee.replace("::", "__");
                    let caller_name = &callinfo.caller.replace("::", "__");
                    w.write(format!("{} -> {}\n", callee_name, caller_name).as_bytes())
                        .unwrap();
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

    create_dot_graph.write(format!("digraph G {{\n").as_bytes())?;

    class_tree.search_preorder(&create_dot_graph);

    // koko ni nokori no guraph wo kaku

    create_dot_graph.write(format!("}}\n").as_bytes())?;

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
