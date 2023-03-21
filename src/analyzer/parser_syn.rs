use std::fmt;

pub struct SynStructName {
    path: syn::Path,
}

impl SynStructName {
    pub fn new(path: &syn::Path) -> SynStructName {
        SynStructName { path: path.clone() }
    }
}

impl fmt::Display for SynStructName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.path.segments.iter();

        let first = match iter.next() {
            Some(first) => first,
            None => return write!(f, ""),
        };

        let mut result = first.ident.to_string();
        for i in iter {
            result += "::";
            result += &i.ident.to_string();
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;
    use quote;

    #[test]
    fn basic() {
        let tokens = quote::quote! {
            StrcutName::method_name();
        };
        let path: syn::Result<syn::Path> = syn::parse2(tokens).unwrap();
        let strcut_name = SynStructName::new(&path);
        assert_eq!(
            strcut_name.to_string(),
            "StrcutName::method_name".to_string()
        );
    }
    */
}
