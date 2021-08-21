extern crate syn;

use super::CallInfo;
use std::error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn analyze(filename: PathBuf) -> Result<Vec<CallInfo>, Box<dyn error::Error>> {
    let mut result: Vec<CallInfo> = Vec::new();
    println!("{:#?}", filename);

    let mut file = File::open(&filename)?;
    let mut src = String::new();
    file.read_to_string(&mut src)?;

    let syntax = syn::parse_file(&src)?;

    for item in syntax.items {
        if let syn::Item::Fn(itemFn) = item {
            println!("{:?}", itemFn.sig.ident.to_string());
            walk_stmt(itemFn.block.stmts);
        }
    }

    Ok(result)
}

fn walk_stmt(items: Vec<syn::Stmt>) {
    for item in items {
        //println!("{:#?}", item);
        match item {
            syn::Stmt::Expr(expr) => walk_expr(expr),
            syn::Stmt::Semi(expr, semi) => walk_expr(expr),
            _ => (),
        }
    }
}

fn walk_expr(item: syn::Expr) {
    match item {
        syn::Expr::Call(exprCall) => {
            walk_expr(*exprCall.func);
        }
        syn::Expr::Path(exprPath) => {
            for i in exprPath.path.segments.iter() {
                println!("  - {:#?}", i.ident.to_string());
            }
        }

        _ => (),
    }
}
