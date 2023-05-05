use super::datas::FullStrcutName;
use super::name_resolver::VariableDefine;
use super::parser_syn::SynStructName;
use super::CallInfo;

enum KindCaller {
    Function(Vec<String>),
    Method(Vec<String>),
}

struct FnInfo {
    current_function: Option<KindCaller>,
    current_impl: Option<String>,
}

impl FnInfo {
    pub fn new() -> FnInfo {
        FnInfo {
            current_function: None,
            current_impl: None,
        }
    }

    pub fn get_caller_name(&self, module_name: &str) -> String {
        let mut caller = String::new();

        match &self.current_function {
            Some(func) => match func {
                KindCaller::Function(name) => {
                    caller.push_str(module_name);
                    caller.push_str("::");
                    caller.push_str(&name.join("::"));
                }
                KindCaller::Method(name) => {
                    caller.push_str(&self.current_impl.clone().unwrap());
                    caller.push_str("::");
                    caller.push_str(&name.join("::"));
                }
            },
            None => caller.push_str("NoData"),
        }
        caller
    }
}

pub struct AnalyzerCallGraph {
    calls: Vec<CallInfo>,
    status: FnInfo,
    local_variables: Vec<VariableDefine>,
    module_name: String,
}

impl AnalyzerCallGraph {
    pub fn new(module_name: String) -> AnalyzerCallGraph {
        let calls: Vec<CallInfo> = Vec::new();
        let status = FnInfo::new();
        let local_variables: Vec<VariableDefine> = Vec::new();
        AnalyzerCallGraph {
            calls,
            status,
            local_variables,
            module_name,
        }
    }

    fn push_callinfo(&mut self, callee: String) {
        let callinfo = CallInfo {
            callee,
            caller: self.status.get_caller_name(&self.module_name),
        };
        self.calls.push(callinfo);
    }

    pub fn get_callinfo(&self) -> Vec<CallInfo> {
        self.calls.clone()
    }
}

impl<'ast> syn::visit::Visit<'ast> for AnalyzerCallGraph {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        // don't analyze test code
        for attr in &node.attrs {
            let path_name = match attr.path.get_ident() {
                Some(n) => n.to_string(),
                None => "".to_string(),
            };
            if path_name == "cfg" && attr.tokens.to_string() == "(test)" {
                return;
            }
        }
        syn::visit::visit_item_mod(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.status.current_function = Some(KindCaller::Function(vec![node.sig.ident.to_string()]));
        syn::visit::visit_item_fn(self, node);
        self.status.current_function = None;
        self.local_variables.clear();
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if let syn::Type::Path(type_path) = &*node.self_ty {
            let impl_name = SynStructName::new(&type_path.path);
            self.status.current_impl = Some(impl_name.to_string());
        }

        syn::visit::visit_item_impl(self, node);

        self.status.current_impl = None;
    }

    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        self.status.current_function = Some(KindCaller::Method(vec![node.sig.ident.to_string()]));

        syn::visit::visit_impl_item_method(self, node);
        self.status.current_function = None;
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(expr_path) = &*node.func {
            let callee = SynStructName::new(&expr_path.path);
            let mut callee_name = callee.name();
            check_callee_path(&mut callee_name, &self.module_name);
            self.push_callinfo(callee_name.fullname());
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let mut method_name = String::new();
        if let syn::Expr::Path(expr_path) = &*node.receiver {
            let path_name = SynStructName::new(&expr_path.path);
            let receiver_name = path_name.to_string();
            if "self" == receiver_name {
                method_name.push_str(
                    &(self
                        .status
                        .current_impl
                        .clone()
                        .unwrap_or_else(|| String::from("NonImpl"))),
                );
                method_name.push_str("::");
            } else {
                for v in &self.local_variables {
                    if v.same_name(&receiver_name) {
                        let ty = v.variable_type();
                        if let Some(name) = ty {
                            method_name.push_str(&name);
                            method_name.push_str("::");
                        }
                    }
                }
            }
        }

        method_name.push_str(&(node.method.to_string()));
        self.push_callinfo(method_name);

        syn::visit::visit_expr_method_call(self, node);
    }

    // syn::Local
    //   (enum)pat - ident -> name
    fn visit_local(&mut self, node: &'ast syn::Local) {
        // for explicit_declaration
        // ex. let var: Vec<String> = Vec::new()
        if let syn::Pat::Type(pat_type) = &node.pat {
            if let syn::Pat::Ident(ident) = &*pat_type.pat {
                let name = ident.ident.to_string();
                let mut variable_type = None;

                if let syn::Type::Path(ty) = &*pat_type.ty {
                    let struct_name = SynStructName::new(&ty.path);
                    variable_type = Some(struct_name.to_string());
                }
                let var = VariableDefine::new(name, variable_type);
                self.local_variables.push(var);
            }
        } else if let syn::Pat::Ident(pat_ident) = &node.pat {
            let name = pat_ident.ident.to_string();
            let mut variable_type = None;
            if let Some(val) = &node.init {
                if let syn::Expr::Path(expr_path) = &*val.1 {
                    let struct_name = SynStructName::new(&expr_path.path);
                    variable_type = Some(struct_name.to_string());
                }
                let var = VariableDefine::new(name, variable_type);
                self.local_variables.push(var);
            }
        }

        syn::visit::visit_local(self, node);
    }
}

fn check_callee_path(callee: &mut FullStrcutName, module: &str) {
    // use wo check site onaji kansu ga aruka
    // file wo kakunin site onaji kansu mei ga aruka
    // onaji dattara sentou ni module mei wo huyo

    // zantei
    //callee.insert_first(module);
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::visit::Visit;

    #[test]
    fn method_call() {
        let src = r#"
            fn func() {
                let a = A::new();
            }
        "#;

        let mut ana = AnalyzerCallGraph::new("module".to_string());
        let syntax = syn::parse_file(&src).unwrap();
        ana.visit_file(&syntax);

        let expect_info = CallInfo {
            callee: "module::A::new".to_string(),
            caller: "module::func".to_string(),
        };
        let expect = vec![expect_info];

        assert_eq!(ana.get_callinfo(), expect);
    }
}
