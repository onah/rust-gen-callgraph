use super::dot_writer;
use super::module_tree::ClassPathTreeInterface;
use crate::call_info::CallInfo;
// StructName removed; use String instead
use std::cell::RefCell;
use std::collections::HashSet;

struct CallInfoWithWrited {
    callinfo: CallInfo,
    writed: RefCell<bool>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum ClusterDataType {
    Single(String),
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

pub struct DotGraphBuilder {
    callinfos: RefCell<Vec<CallInfoWithWrited>>,
    current_classname: RefCell<String>,
    result: RefCell<Vec<ClusterData>>,
}

impl DotGraphBuilder {
    pub fn new(callinfos: Vec<CallInfo>) -> DotGraphBuilder {
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
        DotGraphBuilder {
            callinfos: RefCell::new(callinfos_with_writed),
            current_classname: RefCell::new(String::new()),
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

    pub fn borrow_result(&self) -> std::cell::Ref<'_, Vec<ClusterData>> {
        self.result.borrow()
    }
    pub fn borrow_mut_result(&self) -> std::cell::RefMut<'_, Vec<ClusterData>> {
        self.result.borrow_mut()
    }
}

impl ClassPathTreeInterface for DotGraphBuilder {
    fn exec_search_before(&self, fn_name: &str) -> bool {
        {
            let mut cc = self.current_classname.borrow_mut();
            if !cc.is_empty() {
                cc.push_str("::");
            }
            cc.push_str(fn_name);
        }
        let mut cluster_data = ClusterData {
            cluster_name: self.current_classname.borrow().clone(),
            nodes: HashSet::new(),
        };

        let current_name = self.current_classname.borrow();
        for callinfo in &*self.callinfos.borrow() {
            if callinfo.callinfo.callee.starts_with(&*current_name) {
                if callinfo.callinfo.caller.starts_with(&*current_name) {
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
                        .insert(ClusterDataType::Single(callinfo.callinfo.callee.clone()));
                }
            } else if callinfo.callinfo.caller.starts_with(&*current_name) {
                cluster_data
                    .nodes
                    .insert(ClusterDataType::Single(callinfo.callinfo.caller.clone()));
            }
        }

        let mut result = self.result.borrow_mut();
        result.push(cluster_data);

        true
    }

    fn exec_search_after(&self, _fn_name: &str) -> bool {
        {
            let mut cc = self.current_classname.borrow_mut();
            if let Some(idx) = cc.rfind("::") {
                cc.truncate(idx);
            } else {
                cc.clear();
            }
        }
        true
    }
}
