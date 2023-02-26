struct VariableDefine {
    name: String,
    variable_type: String,
}

impl VariableDefine {
    fn new(name: String, variable_type: String) -> VariableDefine {
        VariableDefine {
            name,
            variable_type,
        }
    }
}
