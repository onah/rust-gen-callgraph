use crate::class_name::ClassName;
use syn::{self, token::Return};

pub struct ClassInfo {
    current_class: Option<String>,
}

impl ClassInfo {
    pub fn new() -> ClassInfo {
        ClassInfo {
            current_class: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionType {
    name: ClassName,
    return_type: String,
}

impl FunctionType {
    pub fn new(name: ClassName, return_type: String) -> FunctionType {
        FunctionType { name, return_type }
    }
}

pub struct AnalyzerFunction {
    function_list: Vec<FunctionType>,
    class_info: ClassInfo,
}

impl AnalyzerFunction {
    pub fn new() -> AnalyzerFunction {
        AnalyzerFunction {
            function_list: Vec::new(),
            class_info: ClassInfo::new(),
        }
    }
}

impl<'ast> syn::visit::Visit<'ast> for AnalyzerFunction {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        // TODO class name??
        let mut class_name = ClassName::new();
        class_name.push(&node.sig.ident.to_string());

        let output = &node.sig.output;
        let return_type = output_to_return_type(output);

        if let Some(x) = return_type {
            self.function_list.push(FunctionType::new(class_name, x));
        }
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        let mut class_name = ClassName::new();
        if let Some(x) = &self.class_info.current_class {
            class_name.push(x);
        }
        class_name.push(&node.sig.ident.to_string());
        let return_type = output_to_return_type(&node.sig.output);

        if let Some(x) = return_type {
            self.function_list.push(FunctionType::new(class_name, x));
        }
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if let syn::Type::Path(type_path) = &*node.self_ty {
            self.class_info.current_class = match &type_path.path.get_ident() {
                Some(v) => Some(v.to_string()),
                None => None,
            };
        }
        syn::visit::visit_item_impl(self, node);
        self.class_info.current_class = None;
    }
}

fn output_to_return_type(output: &syn::ReturnType) -> Option<String> {
    let mut result = None;
    if let syn::ReturnType::Type(_, ty) = output {
        let tmp = ty.clone();
        if let syn::Type::Path(type_path) = *tmp {
            result = type_path.path.get_ident().map(|x| x.to_string());
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
        let mut ana = AnalyzerFunction::new();
        let src = "fn basic() -> String {}";
        let syntax = syn::parse_file(src).unwrap();
        ana.visit_file(&syntax);

        let class_name = ClassName::new_for_str("basic");
        let func_type = FunctionType::new(class_name, "String".to_string());
        let expect = vec![func_type];

        assert_eq!(ana.function_list, expect);
    }
    #[test]
    fn class_method() {
        let mut ana = AnalyzerFunction::new();
        let src = r#"
        impl ClassMethod {
            fn method() -> String {}
        }"#;

        let syntax = syn::parse_file(src).unwrap();
        ana.visit_file(&syntax);

        let class_name = ClassName::new_for_str("ClassMethod::method");
        let func_type = FunctionType::new(class_name, "String".to_string());
        let expect = vec![func_type];

        assert_eq!(ana.function_list, expect);
    }
}
