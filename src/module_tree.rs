use std::cell::RefCell;

pub trait ModuleTreeInterface {
    fn exec_search_before(&self, fn_name: &str) -> bool;
    fn exec_search_after(&self, fn_name: &str) -> bool;
}

#[derive(Debug, PartialEq)]
pub struct ModuleTree {
    // If fn_name is "", this instance is root node.
    fn_name: String,
    nodes: RefCell<Vec<ModuleTree>>,
}

impl ModuleTree {
    pub fn new() -> ModuleTree {
        ModuleTree::create_root_node()
    }

    fn create_root_node() -> ModuleTree {
        ModuleTree {
            fn_name: "".to_string(),
            nodes: RefCell::new(Vec::new()),
        }
    }

    fn create_node(fn_name: &str) -> ModuleTree {
        ModuleTree {
            fn_name: fn_name.to_string(),
            nodes: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, function_names: &[&str]) {
        if function_names.is_empty() {
            return;
        }

        // for that new_node can borrow
        // TODO: better coding?
        {
            let nodes = self.nodes.borrow();

            for node in &*nodes {
                if function_names[0] == node.fn_name {
                    node.push(&function_names[1..]);
                    return;
                }
            }
        }

        let new_node = ModuleTree::create_node(function_names[0]);
        new_node.push(&function_names[1..]);
        self.nodes.borrow_mut().push(new_node);
    }

    pub fn search_preorder(&self, interface: &dyn ModuleTreeInterface) -> bool {
        if !self.fn_name.is_empty() && !interface.exec_search_before(&self.fn_name) {
            return false;
        }

        let nodes = self.nodes.borrow();
        for node in &*nodes {
            if !node.search_preorder(interface) {
                return false;
            };
        }

        if !self.fn_name.is_empty() && !interface.exec_search_after(&self.fn_name) {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestInterface {
        test_strings: RefCell<Vec<String>>,
    }
    impl ModuleTreeInterface for TestInterface {
        fn exec_search_before(&self, fn_name: &str) -> bool {
            let s = fn_name.to_string() + "+";
            self.test_strings.borrow_mut().push(s);
            true
        }
        fn exec_search_after(&self, fn_name: &str) -> bool {
            let s = fn_name.to_string() + "-";
            self.test_strings.borrow_mut().push(s);
            true
        }
    }
    #[test]
    fn it_works() {
        let funcs = "A::B::C".to_string();
        let vs: Vec<&str> = funcs.split("::").collect();

        let root = ModuleTree::new();

        root.push(&vs);

        let funcs = "A::B::D".to_string();
        let vs: Vec<&str> = funcs.split("::").collect();

        root.push(&vs);

        // expect
        let c = ModuleTree {
            fn_name: "C".to_string(),
            nodes: RefCell::new(Vec::new()),
        };

        let d = ModuleTree {
            fn_name: "D".to_string(),
            nodes: RefCell::new(Vec::new()),
        };

        let b = ModuleTree {
            fn_name: "B".to_string(),
            nodes: RefCell::new(vec![c, d]),
        };

        let a = ModuleTree {
            fn_name: "A".to_string(),
            nodes: RefCell::new(vec![b]),
        };

        let expect_root = ModuleTree {
            fn_name: "".to_string(),
            nodes: RefCell::new(vec![a]),
        };

        assert_eq!(root, expect_root);

        // Test search_preorder

        let test_interface = TestInterface {
            test_strings: RefCell::new(Vec::new()),
        };
        root.search_preorder(&test_interface);

        let result = test_interface.test_strings.borrow().join("");
        assert_eq!(result, "A+B+C+C-D+D-B-A-")
    }
}
