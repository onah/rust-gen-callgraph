use super::dot_writer;
use super::module_tree::ModuleTreeInterface;
use crate::call_data::CallInfo;
use crate::call_data::StructName;
use std::cell::RefCell;
use std::collections::HashSet;

struct CallInfoWithWrited {
    callinfo: CallInfo,
    writed: RefCell<bool>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum ClusterDataType {
    Single(StructName),
    CallInfo(CallInfo),
}

pub struct ClusterData {
    cluster_name: String,
    nodes: HashSet<ClusterDataType>,
}

impl ClusterData {
    pub fn get_cluseter_name(&self) -> &str {
        &self.cluster_name
    }

    pub fn get_nodes(&self) -> &HashSet<ClusterDataType> {
        &self.nodes
    }
}

pub struct CreateDotGraph {
    callinfos: RefCell<Vec<CallInfoWithWrited>>,
    current_classname: RefCell<StructName>,
    result: RefCell<Vec<ClusterData>>,
}

impl CreateDotGraph {
    pub fn new(callinfos: Vec<CallInfo>) -> CreateDotGraph {
        let mut callinfos_with_writed: Vec<CallInfoWithWrited> = Vec::new();

        // remove duplicates
        //let mut callinfos_no_dup: HashSet<CallInfo> = HashSet::new();
        //for c in callinfos {
        //    callinfos_no_dup.insert(c);
        //}

        for c in callinfos {
            // TODO: zantei teki ni class ga nai mono (:: ga nai mono) ha nozoku
            // honrai ha nakutemo yoi hazu
            //if c.callee.contains("::") && c.caller.contains("::") {

            let cww = CallInfoWithWrited {
                callinfo: c,
                writed: RefCell::new(false),
            };
            callinfos_with_writed.push(cww);

            //}
        }
        CreateDotGraph {
            callinfos: RefCell::new(callinfos_with_writed),
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

    pub fn get_all_node(&self) -> HashSet<String> {
        // remove duplicate
        let mut path_names: HashSet<String> = HashSet::new();
        let borrowed_callinfo = self.callinfos.borrow();

        for callinfo in &*borrowed_callinfo {
            path_names.insert(callinfo.callinfo.callee.clone());
            path_names.insert(callinfo.callinfo.caller.clone());
        }
        path_names

        //let mut result = String::new();
        //for path_name in path_names {
        //    result += &dot_writer::node(path_name);
        //}
        //result
    }

    pub fn borrow_result(&self) -> std::cell::Ref<Vec<ClusterData>> {
        self.result.borrow()
    }
    pub fn borrow_mut_result(&self) -> std::cell::RefMut<Vec<ClusterData>> {
        self.result.borrow_mut()
    }
}

impl ModuleTreeInterface for CreateDotGraph {
    fn exec_search_before(&self, fn_name: &str) -> bool {
        self.current_classname.borrow_mut().push(fn_name);

        let mut cluster_data = ClusterData {
            cluster_name: self.current_classname.borrow().name(),
            nodes: HashSet::new(),
        };

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

                    cluster_data
                        .nodes
                        .insert(ClusterDataType::CallInfo(CallInfo {
                            callee: callinfo.callinfo.callee.clone(),
                            caller: callinfo.callinfo.caller.clone(),
                        }));
                } else {
                    cluster_data
                        .nodes
                        .insert(ClusterDataType::Single(StructName::new_for_str(
                            &callinfo.callinfo.callee,
                        )));
                }
            } else if callinfo
                .callinfo
                .caller
                .starts_with(&self.current_classname.borrow().name())
            {
                cluster_data
                    .nodes
                    .insert(ClusterDataType::Single(StructName::new_for_str(
                        &callinfo.callinfo.caller,
                    )));
            }
        }

        let mut result = self.result.borrow_mut();
        result.push(cluster_data);

        true
    }

    fn exec_search_after(&self, _fn_name: &str) -> bool {
        self.current_classname.borrow_mut().pop().unwrap();
        true
    }
}
