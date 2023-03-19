#[derive(Debug, PartialEq)]
pub struct StructName {
    names: Vec<String>,
}

impl StructName {
    pub fn new() -> StructName {
        StructName { names: Vec::new() }
    }

    pub fn new_for_str(name: &str) -> StructName {
        let names: Vec<String> = name.split("::").map(|x| x.to_string()).collect();
        StructName { names }
    }

    pub fn name(&self) -> String {
        self.names.join("::")
    }

    pub fn push(&mut self, name: &str) {
        self.names.push(name.to_string());
    }

    pub fn pop(&mut self) -> Option<String> {
        self.names.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_start() {
        let mut c = StructName::new();
        c.push("aaa");
        assert_eq!(c.name(), "aaa");
    }

    #[test]
    fn sinple_new_for_str() {
        let c = StructName::new_for_str("String");
        assert_eq!(c.name(), "String");
    }

    #[test]
    fn multi_new_for_str() {
        let c = StructName::new_for_str("Foo::Bar");
        assert_eq!(c.name(), "Foo::Bar");
    }

    #[test]
    fn test_push_double() {
        let mut c = StructName::new();
        c.push("bbb");
        c.push("ccc");
        assert_eq!(c.name(), "bbb::ccc");
    }
    #[test]
    fn test_pop() {
        let mut c = StructName::new();
        c.push("ddd");
        c.push("eee");
        c.pop().unwrap();
        assert_eq!(c.name(), "ddd");
    }
    #[test]
    fn test_pop_empty() {
        let mut c = StructName::new();
        assert_eq!(c.pop(), None);
    }
}
