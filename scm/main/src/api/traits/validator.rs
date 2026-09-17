use crate::api::{FactoryValidator, ValidateRequest, ValidationError};

/// Checks whether a parsed deck is structurally valid before further processing.
pub trait Validator {
    /// Validates `request.deck` and returns structural-validation diagnostics.
    ///
    /// The default implementation stops at the first failure and returns one
    /// message in `ValidationError::violations`.
    fn validate(&self, request: ValidateRequest) -> Result<(), ValidationError>;

    /// Returns the factory used to construct implementations of this trait.
    fn factory() -> FactoryValidator
    where
        Self: Sized,
    {
        FactoryValidator
    }
}
