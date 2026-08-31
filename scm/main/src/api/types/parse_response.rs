use crate::api::Deck;
use std::sync::Arc;

/// Result of successfully parsing a [`ParseRequest`](crate::api::ParseRequest).
pub struct ParseResponse {
    /// The parsed deck.
    pub deck: Arc<Deck>,
}
