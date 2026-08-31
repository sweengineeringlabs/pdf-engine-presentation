use crate::api::Deck;
use std::sync::Arc;

/// Request to validate a parsed [`Deck`] against the fixed-canvas constraints.
pub struct ValidateRequest {
    /// The deck to validate.
    pub deck: Arc<Deck>,
}
