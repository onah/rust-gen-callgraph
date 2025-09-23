/// call_data is a module for storing call information.

/// Represents a simple function call relationship in the call graph.
///
/// This struct stores the fully qualified names of the caller and callee functions or methods,
/// such as `Project::Class::Func`. All call relationships in the project are collected as a
/// collection of `CallInfo` instances (e.g., `Vec<CallInfo>`).
///
/// # Fields
/// - `caller`: The fully qualified name of the calling function or method.
/// - `callee`: The fully qualified name of the called function or method.
///
/// # Example
/// ```
/// use rust_gen_callgraph::call_data::CallInfo;
/// let call = CallInfo {
///     caller: "my_project::foo::bar".to_string(),
///     callee: "my_project::baz::qux".to_string(),
/// };
/// assert_eq!(call.caller, "my_project::foo::bar");
/// assert_eq!(call.callee, "my_project::baz::qux");
/// ```
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
pub struct CallInfo {
    pub callee: String,
    pub caller: String,
}

#[cfg(test)]
mod tests {
    // use super::*; // No longer needed

    // All StructName tests removed. All names are now handled as String.
}
