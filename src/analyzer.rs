extern crate syn;

use super::CallInfo;
use std::error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
//use syn::spanned::Spanned;

pub fn analyze(filename: PathBuf) -> Result<Vec<CallInfo>, Box<dyn error::Error>> {
    let mut ana = Analyzer::new();
    ana.run(filename)?;

    let mut result: Vec<CallInfo> = Vec::new();
    for call in ana.calls.iter() {
        let callinfo = CallInfo {
            callee: call.0.clone(),
            caller: call.1.clone(),
        };
        result.push(callinfo);
    }

    Ok(result)
}

struct Analyzer {
    calls: Vec<(String, String)>,
    current_function: Option<String>,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        let calls: Vec<(String, String)> = Vec::new();
        let current_function = None;

        Analyzer {
            calls,
            current_function,
        }
    }

    pub fn run(&mut self, filename: PathBuf) -> Result<(), Box<dyn error::Error>> {
        let mut file = File::open(&filename)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        let syntax = syn::parse_file(&src)?;

        for item in syntax.items {
            match item {
                syn::Item::Fn(item_fn) => self.walk_item_fn(item_fn),
                syn::Item::Impl(item_impl) => self.walk_item_impl(item_impl),
                _ => (),
            }
        }

        Ok(())
    }

    fn walk_item_fn(&mut self, item_fn: syn::ItemFn) {
        self.current_function = Some(item_fn.sig.ident.to_string());
        self.walk_stmt(item_fn.block.stmts);
    }

    fn walk_item_impl(&mut self, item_impl: syn::ItemImpl) {
        match *(item_impl.self_ty) {
            syn::Type::Path(type_path) => {
                println!("debug");
                println!("{}", punctuated_to_string(type_path.path.segments));
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

    fn walk_impl_item_method(&mut self, method: syn::ImplItemMethod) {
        self.walk_stmt(method.block.stmts);
    }

    fn walk_stmt(&mut self, items: Vec<syn::Stmt>) {
        for item in items {
            //println!("{:#?}", item);
            match item {
                syn::Stmt::Expr(expr) => self.walk_expr(expr),
                syn::Stmt::Semi(expr, _semi) => self.walk_expr(expr),
                _ => (),
            }
        }
    }

    fn walk_expr(&mut self, item: syn::Expr) {
        match item {
            syn::Expr::Call(expr_call) => {
                self.walk_expr(*expr_call.func);
            }
            syn::Expr::Path(expr_path) => {
                //println!("{}", punctuated_to_string(expr_path.path.segments));
                self.calls.push((
                    punctuated_to_string(expr_path.path.segments),
                    self.current_function.clone().unwrap(),
                ));
            }
            syn::Expr::MethodCall(expr_methodcall) => {
                //println!("{:?}", expr_methodcall.span().source_file());
                println!("{:?}", expr_methodcall.method);
            }

            _ => (),
        }
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
