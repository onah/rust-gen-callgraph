use std::cell::RefCell;

pub trait ClassPathTreeInterface {
    fn exec_search_before(&self, edge: &str) -> bool;
    fn exec_search_after(&self, edge: &str) -> bool;
}

/// ClassPathTree is tree data structure for class path.
/// This is used to recursively traverse tree-structured data.
/// Use push() to register data, and use search_preorder() to access (traverse) the data.
#[derive(Debug, PartialEq)]
pub struct ClassPathTree {
    // If edge is "", this instance is root node.
    node: String,
    edges: RefCell<Vec<ClassPathTree>>,
}

impl ClassPathTree {
    pub fn new() -> ClassPathTree {
        ClassPathTree::create_root_node()
    }

    fn create_root_node() -> ClassPathTree {
        ClassPathTree {
            node: "".to_string(),
            edges: RefCell::new(Vec::new()),
        }
    }

    fn create_node(name: &str) -> ClassPathTree {
        ClassPathTree {
            node: name.to_string(),
            edges: RefCell::new(Vec::new()),
        }
    }

    /// Recursively registers a path (such as module or function hierarchy) into the tree structure.
    /// By inputting the full path name of a function (e.g., Package::Module::ClassA), you can construct the class tree structure.
    /// e.g. PackageName -> ModuleName -> ClassA
    ///                                -> ClassB
    pub fn push(&self, function_names: &[&str]) {
        if function_names.is_empty() {
            return;
        }

        // for that new_node can borrow
        // TODO: better coding?
        {
            let edges = self.edges.borrow();

            for edge in &*edges {
                if function_names[0] == edge.node {
                    edge.push(&function_names[1..]);
                    return;
                }
            }
        }

        let new_node = ClassPathTree::create_node(function_names[0]);
        new_node.push(&function_names[1..]);
        self.edges.borrow_mut().push(new_node);
    }

    /// Traverses the tree in preorder (root → children) and calls the provided interface methods.
    /// `exec_search_before` is called before visiting children, and `exec_search_after` is called after.
    /// Returns false if any interface method returns false, otherwise true.
    /// This is useful for recursive processing or output of tree-structured data.
    pub fn search_preorder(&self, interface: &dyn ClassPathTreeInterface) -> bool {
        if !self.node.is_empty() && !interface.exec_search_before(&self.node) {
            return false;
        }

        let nodes = self.edges.borrow();
        for node in &*nodes {
            if !node.search_preorder(interface) {
                return false;
            };
        }

        if !self.node.is_empty() && !interface.exec_search_after(&self.node) {
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
    impl ClassPathTreeInterface for TestInterface {
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
        let root = ClassPathTree::new();

        let funcs = "A::B::C".to_string();
        let vs: Vec<&str> = funcs.split("::").collect();
        root.push(&vs);

        let funcs = "A::B::D".to_string();
        let vs: Vec<&str> = funcs.split("::").collect();
        root.push(&vs);

        let funcs = "A::E::F".to_string();
        let vs: Vec<&str> = funcs.split("::").collect();
        root.push(&vs);

        // expect
        let f = ClassPathTree {
            node: "F".to_string(),
            edges: RefCell::new(Vec::new()),
        };

        let e = ClassPathTree {
            node: "E".to_string(),
            edges: RefCell::new(vec![f]),
        };

        let c = ClassPathTree {
            node: "C".to_string(),
            edges: RefCell::new(Vec::new()),
        };

        let d = ClassPathTree {
            node: "D".to_string(),
            edges: RefCell::new(Vec::new()),
        };

        let b = ClassPathTree {
            node: "B".to_string(),
            edges: RefCell::new(vec![c, d]),
        };

        let a = ClassPathTree {
            node: "A".to_string(),
            edges: RefCell::new(vec![b, e]),
        };

        let expect_root = ClassPathTree {
            node: "".to_string(),
            edges: RefCell::new(vec![a]),
        };

        assert_eq!(root, expect_root);

        // Test search_preorder

        let test_interface = TestInterface {
            test_strings: RefCell::new(Vec::new()),
        };
        root.search_preorder(&test_interface);

        let result = test_interface.test_strings.borrow().join("");
        assert_eq!(result, "A+B+C+C-D+D-B-E+F+F-E-A-")
    }
}
