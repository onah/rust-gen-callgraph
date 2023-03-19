use std::fmt;
use syn;

struct SynStructName {
    names: syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>,
}

impl fmt::Display for SynStructName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.names.iter();

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
