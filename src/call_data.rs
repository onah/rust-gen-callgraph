#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct CallInfo {
    pub callee: String,
    pub caller: String,
}
