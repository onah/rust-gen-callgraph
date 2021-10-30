extern crate syn;

use super::CallInfo;
use std::error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
//use syn::spanned::Spanned;

pub fn analyze(filename: PathBuf) -> Result<Vec<CallInfo>, Box<dyn error::Error>> {
    let mut analyzer = Analyzer::new();
    analyzer.run(filename)?;
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
                    caller.push_str(
                        self.filename
                            .clone()
                            .unwrap()
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    );
                    caller.push_str("::");
                    caller.push_str(&name)
                }
                FunctionOrMethod::Method(name) => {
                    caller.push_str(&self.current_impl.clone().unwrap());
                    caller.push_str("::");
                    caller.push_str(&name);
                }
            },
            None => caller.push_str("NonData"),
        }
        caller
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

    pub fn run(&mut self, filename: PathBuf) -> Result<(), Box<dyn error::Error>> {
        let mut file = File::open(&filename)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        self.status.filename = Some(filename);

        let syntax = syn::parse_file(&src)?;
        self.walk_file(syntax);

        self.status.filename = None;

        Ok(())
    }

    fn walk_file(&mut self, file: syn::File) {
        for item in file.items {
            match item {
                syn::Item::Fn(item_fn) => self.walk_item_fn(item_fn),
                syn::Item::Impl(item_impl) => self.walk_item_impl(item_impl),
                _ => (),
            }
        }
    }

    fn walk_item_fn(&mut self, item_fn: syn::ItemFn) {
        /*
        let mut caller = String::new();
        caller.push_str(
            self.status
                .filename
                .clone()
                .unwrap()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap(),
        );
        caller.push_str("::");
        caller.push_str(&item_fn.sig.ident.to_string());
        */

        //let fn_name = item_fn.sig.ident.to_string();
        //self.status.current_function = Some(fn_name);

        self.status.current_function =
            Some(FunctionOrMethod::Function(item_fn.sig.ident.to_string()));
        self.walk_block(*item_fn.block);
        self.status.current_function = None;
    }

    fn walk_item_impl(&mut self, item_impl: syn::ItemImpl) {
        if let syn::Type::Path(type_path) = *(item_impl.self_ty) {
            self.status.current_impl = Some(punctuated_to_string(type_path.path.segments));
        }

        for item in item_impl.items {
            match item {
                syn::ImplItem::Method(impl_item_method) => {
                    self.walk_impl_item_method(impl_item_method);
                }
                _ => (),
            }
        }
        self.status.current_impl = None;
    }

    fn walk_block(&mut self, block: syn::Block) {
        for stmt in block.stmts {
            self.walk_stmt(stmt);
        }
    }

    fn walk_impl_item_method(&mut self, method: syn::ImplItemMethod) {
        /*
        let mut caller = String::new();
        caller.push_str(&self.status.current_impl.clone().unwrap());
        caller.push_str("::");
        caller.push_str(&(method.sig.ident.to_string()));
        */

        self.status.current_function = Some(FunctionOrMethod::Method(method.sig.ident.to_string()));
        self.walk_block(method.block);
        self.status.current_function = None;
    }

    fn walk_stmt(&mut self, stmt: syn::Stmt) {
        match stmt {
            syn::Stmt::Expr(expr) => self.walk_expr(expr),
            syn::Stmt::Semi(expr, _semi) => self.walk_expr(expr),
            _ => (),
        }
    }

    fn walk_expr(&mut self, item: syn::Expr) {
        match item {
            syn::Expr::Call(expr_call) => {
                self.walk_expr(*expr_call.func);
            }
            syn::Expr::Path(expr_path) => {
                self.push_callinfo(punctuated_to_string(expr_path.path.segments));
            }

            syn::Expr::MethodCall(expr_methodcall) => {
                let mut method_name = String::new();

                match *(expr_methodcall.receiver) {
                    syn::Expr::Path(expr_path) => {
                        if "self" == punctuated_to_string(expr_path.path.segments) {
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
                    _ => (),
                }

                method_name.push_str(&(expr_methodcall.method.to_string()));
                self.push_callinfo(method_name);
            }
            syn::Expr::If(expr_if) => self.walk_block(expr_if.then_branch),

            syn::Expr::ForLoop(expr_forloop) => self.walk_block(expr_forloop.body),

            syn::Expr::Block(expr_block) => self.walk_block(expr_block.block),

            syn::Expr::Match(expr_match) => {
                for arm in expr_match.arms {
                    self.walk_expr(*arm.body);
                }
            }

            _ => (),
        }
    }

    fn push_callinfo(&mut self, callee: String) {
        //let caller = self
        //    .current_function
        //    .clone()
        //    .unwrap_or(String::from("NonData"));

        let callinfo = CallInfo {
            callee: callee,
            caller: self.status.get_caller_name(),
        };
        self.calls.push(callinfo);
    }
}

fn punctuated_to_string(
    punctuated: syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>,
) -> String {
    let mut result = String::new();
    for i in punctuated.iter() {
        result = result + &i.ident.to_string() + "::";
    }
    result.pop();
    result.pop();
    result
}
