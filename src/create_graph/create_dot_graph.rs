use super::dot_writer;
use super::module_tree::ModuleTreeInterface;
use crate::call_data::CallInfo;
use crate::struct_name::StructName;
use std::cell::RefCell;
use std::collections::HashSet;

struct CallInfoWithWrited {
    callinfo: CallInfo,
    writed: RefCell<bool>,
}

pub enum ClusterDataType {
    Single(StructName),
    CallInfo(CallInfo),
}

pub struct ClusterData {
    cluster_name: String,
    nodes: Vec<ClusterDataType>,
}

impl ClusterData {
    pub fn get_cluseter_name(&self) -> &str {
        &self.cluster_name
    }

    pub fn get_nodes(&self) -> &Vec<ClusterDataType> {
        &self.nodes
    }
}

pub struct CreateDotGraph {
    callinfos: RefCell<Vec<CallInfoWithWrited>>,
    cluster_counter: RefCell<usize>,
    current_classname: RefCell<StructName>,
    //result: RefCell<String>,
    result: RefCell<Vec<ClusterData>>,
}

impl CreateDotGraph {
    pub fn new(callinfos: Vec<CallInfo>, print_data_type: bool) -> CreateDotGraph {
        let mut callinfos_with_writed: Vec<CallInfoWithWrited> = Vec::new();

        // remove duplicates
        let mut callinfos_no_dup: HashSet<CallInfo> = HashSet::new();
        for c in callinfos {
            callinfos_no_dup.insert(c);
        }

        for c in callinfos_no_dup {
            // TODO: zantei teki ni class ga nai mono (:: ga nai mono) ha nozoku
            // honrai ha nakutemo yoi hazu
            //if c.callee.contains("::") && c.caller.contains("::") {
            if !print_data_type
                && !CreateDotGraph::is_data_type(&c.callee)
                && !CreateDotGraph::is_data_type(&c.caller)
            {
                let cww = CallInfoWithWrited {
                    callinfo: c,
                    writed: RefCell::new(false),
                };
                callinfos_with_writed.push(cww);
            }
            //}
        }
        CreateDotGraph {
            callinfos: RefCell::new(callinfos_with_writed),
            cluster_counter: RefCell::new(0),
            current_classname: RefCell::new(StructName::new()),
            result: RefCell::new(Vec::new()),
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
        // remove duplicate
        let mut path_names: HashSet<&str> = HashSet::new();
        let borrowed_callinfo = self.callinfos.borrow();

        for callinfo in &*borrowed_callinfo {
            path_names.insert(&callinfo.callinfo.callee);
            path_names.insert(&callinfo.callinfo.caller);
        }

        let mut result = String::new();
        for path_name in path_names {
            result += &dot_writer::node(path_name);
        }
        result
    }

    pub fn borrow_result(&self) -> std::cell::Ref<Vec<ClusterData>> {
        self.result.borrow()
    }
    pub fn borrow_mut_result(&self) -> std::cell::RefMut<Vec<ClusterData>> {
        self.result.borrow_mut()
    }

    fn is_data_type(name: &str) -> bool {
        let data_types = vec!["String", "Vec"];
        for ty in data_types {
            if name.starts_with(ty) {
                return true;
            };
        }
        false
    }
}

impl ModuleTreeInterface for CreateDotGraph {
    fn exec_search_before(&self, fn_name: &str) -> bool {
        self.current_classname.borrow_mut().push(fn_name);

        //let mut result = self.result.borrow_mut();
        // *result += &format!("subgraph cluster_{} {{\n", self.cluster_counter.borrow());
        // *result += &format!("label=\"{}\"\n", self.current_classname.borrow().name());
        let mut cluster_data = ClusterData {
            cluster_name: self.current_classname.borrow().name(),
            nodes: Vec::new(),
        };

        // TODO duplicate*
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

                    //let callee_name = &callinfo.callinfo.callee.replace([':', '-'], "_");
                    //let caller_name = &callinfo.callinfo.caller.replace([':', '-'], "_");
                    //*result += &format!("{} -> {}\n", caller_name, callee_name);
                    cluster_data.nodes.push(ClusterDataType::CallInfo(CallInfo {
                        callee: callinfo.callinfo.callee.clone(),
                        caller: callinfo.callinfo.caller.clone(),
                    }));
                } else {
                    //let callee_name = &callinfo.callinfo.callee.replace([':', '-'], "_");
                    //*result += &format!("{}\n", callee_name);
                    cluster_data
                        .nodes
                        .push(ClusterDataType::Single(StructName::new_for_str(
                            &callinfo.callinfo.callee,
                        )));
                }
            } else if callinfo
                .callinfo
                .caller
                .starts_with(&self.current_classname.borrow().name())
            {
                //let caller_name = &callinfo.callinfo.caller.replace([':', '-'], "_");
                //*result += &format!("{}\n", caller_name);
                cluster_data
                    .nodes
                    .push(ClusterDataType::Single(StructName::new_for_str(
                        &callinfo.callinfo.caller,
                    )));
            }
        }

        let mut result = self.result.borrow_mut();
        result.push(cluster_data);

        *self.cluster_counter.borrow_mut() += 1;
        true
    }

    fn exec_search_after(&self, _fn_name: &str) -> bool {
        self.current_classname.borrow_mut().pop().unwrap();

        //let mut result = self.result.borrow_mut();
        //*result += "}\n";

        true
    }
}
