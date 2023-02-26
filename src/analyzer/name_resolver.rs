pub struct VariableDefine {
    name: String,
    variable_type: Option<String>,
}

impl VariableDefine {
    pub fn new(name: String, variable_type: Option<String>) -> VariableDefine {
        VariableDefine {
            name,
            variable_type,
        }
    }

    pub fn same_name(&self, other: &str) -> bool {
        self.name == other
    }

    pub fn variable_type(&self) -> Option<String> {
        self.variable_type.clone()
    }
}
