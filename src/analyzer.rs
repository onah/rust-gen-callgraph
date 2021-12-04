extern crate syn;

use super::CallInfo;
use std::error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::visit::Visit;
use toml;

pub fn analyze(filename: PathBuf) -> Result<Vec<CallInfo>, Box<dyn error::Error>> {
    let mut analyzer = Analyzer::new();

    let mut file = File::open(&filename)?;
    let mut src = String::new();
    file.read_to_string(&mut src)?;

    let syntax = syn::parse_file(&src)?;

    analyzer.status.filename = Some(filename);
    analyzer.visit_file(&syntax);
    analyzer.status.filename = None;

    Ok(analyzer.calls)
}

enum FunctionOrMethod {
    Function(String),
    Method(String),
}

struct FnInfo {
    current_function: Option<FunctionOrMethod>,
    current_impl: Option<String>,
    filename: Option<PathBuf>,
}

impl FnInfo {
    pub fn new() -> FnInfo {
        FnInfo {
            current_function: None,
            current_impl: None,
            filename: None,
        }
    }

    pub fn get_caller_name(&self) -> String {
        let mut caller = String::new();

        match &self.current_function {
            Some(func) => match func {
                FunctionOrMethod::Function(name) => {
                    caller.push_str(&FnInfo::filename_to_modulename(
                        &self.filename.clone().unwrap(),
                    ));
                    caller.push_str("::");
                    caller.push_str(&name)
                }
                FunctionOrMethod::Method(name) => {
                    caller.push_str(&self.current_impl.clone().unwrap());
                    caller.push_str("::");
                    caller.push_str(&name);
                }
            },
            None => caller.push_str("NoData"),
        }
        caller
    }

    fn filename_to_modulename(filename: &PathBuf) -> String {
        let result;

        if filename == OsStr::new("./src/lib.rs") {
            let mut f = File::open("Cargo.toml").unwrap();
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();

            let values = contents.parse::<toml::Value>().unwrap();
            let project_name = values["package"]["name"].as_str().unwrap();

            result = String::from(project_name);
        } else if filename.file_name().unwrap() == OsStr::new("lib.rs") {
            let mut filename2 = filename.clone();
            filename2.pop();
            if filename2.file_name().unwrap() == OsStr::new("src") {
                filename2.pop();
                {
                    // TODO Copy Code Refactoring
                    let mut f =
                        File::open(format!("{}/Cargo.toml", filename2.to_str().unwrap())).unwrap();
                    let mut contents = String::new();
                    f.read_to_string(&mut contents).unwrap();

                    let values = contents.parse::<toml::Value>().unwrap();
                    let project_name = values["package"]["name"].as_str().unwrap();

                    result = String::from(project_name);
                }
            } else {
                result = filename2.file_stem().unwrap().to_str().unwrap().to_string();
            }
        } else {
            result = filename.file_stem().unwrap().to_str().unwrap().to_string();
        }

        result
    }
}

struct Analyzer {
    calls: Vec<CallInfo>,
    status: FnInfo,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        let calls: Vec<CallInfo> = Vec::new();
        let status = FnInfo::new();

        Analyzer { calls, status }
    }

    fn push_callinfo(&mut self, callee: String) {
        let callinfo = CallInfo {
            callee,
            caller: self.status.get_caller_name(),
        };
        self.calls.push(callinfo);
    }

    fn punctuated_to_string(
        &self,
        punctuated: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>,
    ) -> String {
        let mut result = String::new();

        // TODO: other implementation ?

        for i in punctuated.iter() {
            result = result + &i.ident.to_string() + "::";
        }

        // Delete last "::"
        result.pop();
        result.pop();

        result
    }
}

impl<'ast> syn::visit::Visit<'ast> for Analyzer {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.status.current_function = Some(FunctionOrMethod::Function(node.sig.ident.to_string()));
        syn::visit::visit_item_fn(self, node);
        self.status.current_function = None;
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if let syn::Type::Path(type_path) = &*node.self_ty {
            self.status.current_impl = Some(self.punctuated_to_string(&type_path.path.segments));
        }

        syn::visit::visit_item_impl(self, node);

        self.status.current_impl = None;
    }

    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        self.status.current_function = Some(FunctionOrMethod::Method(node.sig.ident.to_string()));

        syn::visit::visit_impl_item_method(self, node);
        self.status.current_function = None;
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(expr_path) = &*node.func {
            self.push_callinfo(self.punctuated_to_string(&expr_path.path.segments));
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let mut method_name = String::new();
        if let syn::Expr::Path(expr_path) = &*node.receiver {
            if "self" == self.punctuated_to_string(&expr_path.path.segments) {
                method_name.push_str(
                    &(self
                        .status
                        .current_impl
                        .clone()
                        .unwrap_or(String::from("NonImpl"))),
                );
                method_name.push_str("::");
            }
        }

        method_name.push_str(&(node.method.to_string()));
        self.push_callinfo(method_name);

        syn::visit::visit_expr_method_call(self, node);
    }
}
