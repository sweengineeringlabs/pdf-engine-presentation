use crate::api::Validator;
use std::sync::Arc;

/// Result of [`DeckParser::validator`](crate::api::DeckParser::validator).
pub struct GetValidatorResponse {
    /// A validator for decks this parser produces.
    pub validator: Arc<dyn Validator>,
}
