/// Itemized structural-validation diagnostics.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValidationError {
    /// One human-readable message per violation found.
    pub violations: Vec<String>,
}
