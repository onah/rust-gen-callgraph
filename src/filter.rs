use crate::call_info::CallInfo;

pub fn filterling(callinfo_list: Vec<CallInfo>, options: &Options) -> Vec<CallInfo> {
    let mut callinfo_list = callinfo_list;

    // delete duplicate data
    //let mut callinfo_list_no_dup: HashSet<CallInfo> = HashSet::new();
    //for c in callinfo_list {
    //    callinfo_list_no_dup.insert(c);
    //}
    callinfo_list.sort_by(|a, b| a.cmp(&b));
    callinfo_list.dedup_by(|a, b| a == b);

    // delete data type
    if !options.print_data_type {
        callinfo_list = callinfo_list
            .into_iter()
            .filter(|x| !is_data_type(&x.callee) && !is_data_type(&x.caller))
            .collect();
    }

    callinfo_list
}

fn is_data_type(name: &str) -> bool {
    let data_types = vec!["String", "Vec"];
    for ty in data_types {
        if name.starts_with(ty) {
            return true;
        };
    }
    false
}

// print_data_type is that printing standard type ex) String, Vec, etc
// TODO: std library no seigyo wo sitahouga yosasou
pub struct Options {
    pub print_data_type: bool,
}

impl Options {
    pub fn new(print_data_type: bool) -> Options {
        Options { print_data_type }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filterling_print_data_type1() {
        let callinfo_list = vec![
            CallInfo {
                caller: "String".to_string(),
                callee: "String".to_string(),
            },
            CallInfo {
                caller: "String".to_string(),
                callee: "Vec".to_string(),
            },
            CallInfo {
                caller: "Vec".to_string(),
                callee: "String".to_string(),
            },
            CallInfo {
                caller: "Vec".to_string(),
                callee: "Vec".to_string(),
            },
        ];

        let options = Options::new(false);
        let callinfo_list = filterling(callinfo_list, &options);

        assert_eq!(callinfo_list.len(), 0);
    }

    #[test]
    fn test_filterling_print_data_type2() {
        let callinfo_list = vec![
            CallInfo {
                caller: "String".to_string(),
                callee: "String".to_string(),
            },
            CallInfo {
                caller: "String".to_string(),
                callee: "Vec".to_string(),
            },
            CallInfo {
                caller: "Vec".to_string(),
                callee: "String".to_string(),
            },
            CallInfo {
                caller: "Vec".to_string(),
                callee: "Vec".to_string(),
            },
        ];

        let options = Options::new(true);
        let callinfo_list = filterling(callinfo_list, &options);

        assert_eq!(callinfo_list.len(), 4);
    }

    #[test]
    fn test_filterling_duplicate() {
        let callinfo_list = vec![
            CallInfo {
                caller: "String".to_string(),
                callee: "String".to_string(),
            },
            CallInfo {
                caller: "String".to_string(),
                callee: "Vec".to_string(),
            },
            CallInfo {
                caller: "Vec".to_string(),
                callee: "String".to_string(),
            },
            CallInfo {
                caller: "Vec".to_string(),
                callee: "Vec".to_string(),
            },
        ];

        let options = Options::new(true);
        let callinfo_list = filterling(callinfo_list, &options);

        assert_eq!(callinfo_list.len(), 4);

        let callinfo_list = vec![
            CallInfo {
                caller: "MethodA".to_string(),
                callee: "MethodB".to_string(),
            },
            CallInfo {
                caller: "MethodA".to_string(),
                callee: "MethodC".to_string(),
            },
            CallInfo {
                caller: "MethodB".to_string(),
                callee: "MethodC".to_string(),
            },
            CallInfo {
                caller: "MethodA".to_string(),
                callee: "MethodC".to_string(),
            },
            CallInfo {
                caller: "Vec".to_string(),
                callee: "Vec".to_string(),
            },
        ];

        let options = Options::new(true);
        let callinfo_list = filterling(callinfo_list, &options);

        assert_eq!(callinfo_list.len(), 4);
    }
}
