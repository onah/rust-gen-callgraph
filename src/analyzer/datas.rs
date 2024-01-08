pub struct FullStrcutName {
    name: Vec<String>,
}

impl FullStrcutName {
    pub fn new() -> FullStrcutName {
        FullStrcutName { name: Vec::new() }
    }

    pub fn push(&mut self, input: &str) {
        self.name.push(input.to_string());
    }

    pub fn insert_first(&mut self, input: &str) {
        self.name.insert(0, input.to_string())
    }

    pub fn fullname(&self) -> String {
        self.name.join("::")
    }
}

/*
// for to_string()
use std::fmt;
impl fmt::Display for FullStrcutName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.join("::"))?;
        Ok(())
    }
}
*/
