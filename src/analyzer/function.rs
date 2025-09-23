//! Parse return type of a function or method
use super::parser_syn::SynStructName;

/// Save the current struct name when parsing
struct StructInfo {
    current_class: Option<String>,
}

impl StructInfo {
    pub fn new() -> StructInfo {
        StructInfo {
            current_class: None,
        }
    }
}

///
#[derive(Debug, PartialEq)]
pub struct FunctionType {
    name: String,
    return_type: String,
}

impl FunctionType {
    pub fn new(name: String, return_type: String) -> FunctionType {
        FunctionType { name, return_type }
    }
}

pub struct AnalyzerFunction {
    project_name: String,
    function_list: Vec<FunctionType>,
    struct_info: StructInfo,
}

impl AnalyzerFunction {
    pub fn new(project_name: String) -> AnalyzerFunction {
        AnalyzerFunction {
            project_name,
            function_list: Vec::new(),
            struct_info: StructInfo::new(),
        }
    }
}

impl<'ast> syn::visit::Visit<'ast> for AnalyzerFunction {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        let class_name = format!("{}::{}", self.project_name, node.sig.ident);
        let output = &node.sig.output;
        let return_type = output_to_return_type(output);
        if let Some(x) = return_type {
            self.function_list.push(FunctionType::new(class_name, x));
        }
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        let class_name = if let Some(x) = &self.struct_info.current_class {
            format!("{}::{}::{}", self.project_name, x, node.sig.ident)
        } else {
            format!("{}::{}", self.project_name, node.sig.ident)
        };
        let return_type = output_to_return_type(&node.sig.output);
        if let Some(x) = return_type {
            self.function_list.push(FunctionType::new(class_name, x));
        }
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if let syn::Type::Path(type_path) = &*node.self_ty {
            let name = SynStructName::new(&type_path.path);
            self.struct_info.current_class = Some(name.to_string());
        }
        syn::visit::visit_item_impl(self, node);
        self.struct_info.current_class = None;
    }
}

fn output_to_return_type(output: &syn::ReturnType) -> Option<String> {
    let mut result = None;
    if let syn::ReturnType::Type(_, ty) = output {
        let tmp = ty.clone();
        if let syn::Type::Path(type_path) = *tmp {
            let name = SynStructName::new(&type_path.path);
            result = Some(name.to_string());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::visit::Visit;

    #[test]
    fn basic() {
        let mut ana = AnalyzerFunction::new("my_project".to_string());
        let src = "fn basic() -> String {}";
        let syntax = syn::parse_file(src).unwrap();
        ana.visit_file(&syntax);

        let class_name = "my_project::basic".to_string();
        let func_type = FunctionType::new(class_name, "String".to_string());
        let expect = vec![func_type];

        assert_eq!(ana.function_list, expect);
    }
    #[test]
    fn class_method() {
        let mut ana = AnalyzerFunction::new("my_project".to_string());
        let src = r#"
        impl ClassMethod {
            fn method() -> String {}
        }"#;

        let syntax = syn::parse_file(src).unwrap();
        ana.visit_file(&syntax);

        let class_name = "my_project::ClassMethod::method".to_string();
        let func_type = FunctionType::new(class_name, "String".to_string());
        let expect = vec![func_type];

        assert_eq!(ana.function_list, expect);
    }
}
