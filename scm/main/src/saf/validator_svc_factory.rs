//! Re-exports the [`Validator`](crate::api::Validator) port trait through
//! the facade. The production implementation is
//! [`FactoryValidator`](crate::api::FactoryValidator), reachable via the
//! crate root.

pub use crate::api::Validator;
