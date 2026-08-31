use crate::api::{Validator, ValidatorFactory};
use crate::core::DefaultDeckValidator;

impl ValidatorFactory {
    /// Builds the default [`Validator`] implementation.
    pub fn build(&self) -> impl Validator {
        DefaultDeckValidator
    }
}
