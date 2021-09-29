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

struct Analyzer {
    calls: Vec<CallInfo>,
    current_function: Option<String>,
    item_impl: Option<String>,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        let calls: Vec<CallInfo> = Vec::new();
        Analyzer {
            calls,
            current_function: None,
            item_impl: None,
        }
    }

    pub fn run(&mut self, filename: PathBuf) -> Result<(), Box<dyn error::Error>> {
        let mut file = File::open(&filename)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        let syntax = syn::parse_file(&src)?;
        self.walk_file(syntax);

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
        self.current_function = Some(item_fn.sig.ident.to_string());
        self.walk_block(*item_fn.block);
    }

    fn walk_item_impl(&mut self, item_impl: syn::ItemImpl) {
        match *(item_impl.self_ty) {
            syn::Type::Path(type_path) => {
                self.item_impl = Some(punctuated_to_string(type_path.path.segments));
            }
            _ => (),
        }

        for item in item_impl.items {
            match item {
                syn::ImplItem::Method(impl_item_method) => {
                    self.walk_impl_item_method(impl_item_method)
                }
                _ => (),
            }
        }
    }

    fn walk_block(&mut self, block: syn::Block) {
        for stmt in block.stmts {
            self.walk_stmt(stmt);
        }
    }

    fn walk_impl_item_method(&mut self, method: syn::ImplItemMethod) {
        let mut curfun = String::new();
        curfun.push_str(&(self.item_impl.clone().unwrap()));
        curfun.push_str("::");
        curfun.push_str(&(method.sig.ident.to_string()));

        self.current_function = Some(curfun);
        self.walk_block(method.block);
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
                            method_name.push_str(&(self.item_impl.clone().unwrap()));
                            method_name.push_str("::");
                        }
                    }
                    _ => (),
                }

                method_name.push_str(&(expr_methodcall.method.to_string()));
                self.push_callinfo(method_name);
            }
            syn::Expr::If(expr_if) => {
                self.walk_block(expr_if.then_branch);
            }
            syn::Expr::ForLoop(expr_forloop) => {
                self.walk_block(expr_forloop.body);
            }

            _ => (),
        }
    }

    fn push_callinfo(&mut self, callee: String) {
        let caller = self
            .current_function
            .clone()
            .unwrap_or(String::from("NonData"));

        let callinfo = CallInfo { callee, caller };
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
