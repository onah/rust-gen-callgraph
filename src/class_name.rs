pub struct ClassName {
    names: Vec<String>,
}

impl ClassName {
    pub fn new() -> ClassName {
        ClassName { names: Vec::new() }
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
        let mut c = ClassName::new();
        c.push("aaa");
        assert_eq!(c.name(), "aaa");
    }
    #[test]
    fn test_push_double() {
        let mut c = ClassName::new();
        c.push("bbb");
        c.push("ccc");
        assert_eq!(c.name(), "bbb::ccc");
    }
    #[test]
    fn test_pop() {
        let mut c = ClassName::new();
        c.push("ddd");
        c.push("eee");
        c.pop().unwrap();
        assert_eq!(c.name(), "ddd");
    }
    #[test]
    fn test_pop_empty() {
        let mut c = ClassName::new();
        assert_eq!(c.pop(), None);
    }
}
