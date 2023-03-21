use super::name_resolver::VariableDefine;
use super::CallInfo;
use std::error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

enum KindCaller {
    Function(Vec<String>),
    Method(Vec<String>),
}

struct FnInfo {
    current_function: Option<KindCaller>,
    current_impl: Option<String>,
    // filename: PathBuf,
}

impl FnInfo {
    pub fn new() -> FnInfo {
        FnInfo {
            current_function: None,
            current_impl: None,
            // filename: filename.clone(),
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

struct FileInfo {
    //file_name: PathBuf,
    module_name: String,
}

impl FileInfo {
    pub fn new(file_name: PathBuf) -> Result<FileInfo, Box<dyn error::Error>> {
        let module_name = FileInfo::get_module_name(&file_name)?;

        let file_info = FileInfo {
            //file_name,
            module_name,
        };
        Ok(file_info)
    }

    fn get_module_name(file_name: &PathBuf) -> Result<String, Box<dyn error::Error>> {
        let result;

        if file_name == OsStr::new("./src/lib.rs") {
            let mut f = File::open("Cargo.toml")?;
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;

            let values = contents.parse::<toml::Value>()?;
            let project_name = values["package"]["name"].as_str().unwrap_or_else(|| "");
            result = String::from(project_name);
        } else if file_name.file_name().unwrap_or_else(|| OsStr::new("")) == OsStr::new("lib.rs") {
            let mut filename2 = file_name.clone();
            filename2.pop();
            if filename2.file_name().unwrap() == OsStr::new("src") {
                filename2.pop();
                {
                    // TODO Copy Code Refactoring
                    let mut f = File::open(format!("{}/Cargo.toml", filename2.to_str().unwrap()))?;
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)?;

                    let values = contents.parse::<toml::Value>()?;
                    let project_name = values["package"]["name"].as_str().unwrap();

                    result = String::from(project_name);
                }
            } else {
                result = filename2.file_stem().unwrap().to_str().unwrap().to_string();
            }
        } else {
            result = file_name.file_stem().unwrap().to_str().unwrap().to_string();
        }

        Ok(result)
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }
}
pub struct Analyzer {
    calls: Vec<CallInfo>,
    status: FnInfo,
    local_variables: Vec<VariableDefine>,
    file_info: FileInfo,
}

impl Analyzer {
    pub fn new(file_name: PathBuf) -> Result<Analyzer, Box<dyn error::Error>> {
        let calls: Vec<CallInfo> = Vec::new();
        let status = FnInfo::new();
        let local_variables: Vec<VariableDefine> = Vec::new();
        let file_info: FileInfo = FileInfo::new(file_name)?;
        Ok(Analyzer {
            calls,
            status,
            local_variables,
            file_info,
        })
    }

    fn push_callinfo(&mut self, callee: String) {
        let callinfo = CallInfo {
            callee,
            caller: self.status.get_caller_name(self.file_info.module_name()),
        };
        self.calls.push(callinfo);
    }

    pub fn get_callinfo(&self) -> Vec<CallInfo> {
        self.calls.clone()
    }
}

fn punctuated_to_string(
    punctuated: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>,
) -> String {
    let mut iter = punctuated.iter();

    let first = match iter.next() {
        Some(first) => first,
        None => return "".to_string(),
    };

    let mut result = first.ident.to_string();
    for i in iter {
        result += "::";
        result += &i.ident.to_string();
    }

    result
}

impl<'ast> syn::visit::Visit<'ast> for Analyzer {
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
            self.status.current_impl = Some(punctuated_to_string(&type_path.path.segments));
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
            /*
                let tmp = match expr_path.path.get_ident() {
                    Some(x) => x.to_string(),
                    None => "".to_string(),
                };
                println!("{}", tmp);
            */

            self.push_callinfo(punctuated_to_string(&expr_path.path.segments));
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let mut method_name = String::new();
        if let syn::Expr::Path(expr_path) = &*node.receiver {
            let receiver_name = punctuated_to_string(&expr_path.path.segments);
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
                let name = (&ident).ident.to_string();
                let mut variable_type = None;

                if let syn::Type::Path(ty) = &*pat_type.ty {
                    variable_type = Some(punctuated_to_string(&ty.path.segments));
                }
                let var = VariableDefine::new(name, variable_type);
                self.local_variables.push(var);
            }
        } else if let syn::Pat::Ident(pat_ident) = &node.pat {
            let name = pat_ident.ident.to_string();
            let mut variable_type = None;
            if let Some(val) = &node.init {
                if let syn::Expr::Path(expr_path) = &*val.1 {
                    variable_type = Some(punctuated_to_string(&expr_path.path.segments));
                }
                let var = VariableDefine::new(name, variable_type);
                self.local_variables.push(var);
            }
        }
    }
}
