use crate::api::{FactoryValidator, ValidateRequest, ValidationError};

/// Checks whether a parsed deck is structurally valid before further processing.
pub trait Validator {
    /// Validates `request.deck`, collecting every violation found.
    fn validate(&self, request: ValidateRequest) -> Result<(), ValidationError>;

    /// Returns the factory used to construct implementations of this trait.
    fn factory() -> FactoryValidator
    where
        Self: Sized,
    {
        FactoryValidator
    }
}
