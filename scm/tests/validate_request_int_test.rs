#![allow(missing_docs)]

use pdf_engine_presentation::{AspectRatio, Deck, OverflowPolicy, ValidateRequest};
use std::sync::Arc;

/// @covers: ValidateRequest
#[test]
fn test_validate_request_holds_deck_to_validate() {
    let deck = Deck {
        title: "Quarterly Review".to_string(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![],
        overflow_policy: OverflowPolicy::Reject,
    };
    let request = ValidateRequest {
        deck: Arc::new(deck),
    };
    assert_eq!(request.deck.title, "Quarterly Review");
}
