use std::cell::RefCell;

pub trait ClassTreeInterface {
    fn exec_search_before(&self, fn_name: &str);
    fn exec_search_after(&self, fn_name: &str);
}

#[derive(Debug, PartialEq)]
pub struct ClassTree {
    // If fn_name is "", this instance is root node.
    fn_name: String,
    nodes: RefCell<Vec<ClassTree>>,
}

impl ClassTree {
    pub fn new() -> ClassTree {
        ClassTree::create_root_node()
    }

    fn create_root_node() -> ClassTree {
        ClassTree {
            fn_name: "".to_string(),
            nodes: RefCell::new(Vec::new()),
        }
    }

    fn create_node(fn_name: &str) -> ClassTree {
        ClassTree {
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

        let new_node = ClassTree::create_node(function_names[0]);
        new_node.push(&function_names[1..]);
        self.nodes.borrow_mut().push(new_node);
    }

    pub fn search_preorder(&self, interface: &dyn ClassTreeInterface) {
        if self.fn_name != "" {
            interface.exec_search_before(&self.fn_name);
        }

        let nodes = self.nodes.borrow();
        for node in &*nodes {
            node.search_preorder(interface);
        }

        if self.fn_name != "" {
            interface.exec_search_after(&self.fn_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestInterface {
        testStrings: RefCell<Vec<String>>,
    }
    impl ClassTreeInterface for TestInterface {
        fn exec_search_before(&self, fn_name: &str) {
            let s = fn_name.to_string() + "+";
            self.testStrings.borrow_mut().push(s);
        }
        fn exec_search_after(&self, fn_name: &str) {
            let s = fn_name.to_string() + "-";
            self.testStrings.borrow_mut().push(s);
        }
    }
    #[test]
    fn it_works() {
        let funcs = "A::B::C".to_string();
        let vs: Vec<&str> = funcs.split("::").collect();

        let root = ClassTree::new();

        root.push(&vs);

        let funcs = "A::B::D".to_string();
        let vs: Vec<&str> = funcs.split("::").collect();

        root.push(&vs);

        // expect
        let c = ClassTree {
            fn_name: "C".to_string(),
            nodes: RefCell::new(Vec::new()),
        };

        let d = ClassTree {
            fn_name: "D".to_string(),
            nodes: RefCell::new(Vec::new()),
        };

        let b = ClassTree {
            fn_name: "B".to_string(),
            nodes: RefCell::new(vec![c, d]),
        };

        let a = ClassTree {
            fn_name: "A".to_string(),
            nodes: RefCell::new(vec![b]),
        };

        let expect_root = ClassTree {
            fn_name: "".to_string(),
            nodes: RefCell::new(vec![a]),
        };

        assert_eq!(root, expect_root);

        // search_preorder

        let test_interface = TestInterface {
            testStrings: RefCell::new(Vec::new()),
        };
        root.search_preorder(&test_interface);

        let result = test_interface.testStrings.borrow().join("");
        assert_eq!(result, "A+B+C+C-D+D-B-A-")
    }
}
