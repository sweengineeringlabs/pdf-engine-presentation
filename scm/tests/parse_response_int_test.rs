#![allow(missing_docs)]

use pdf_engine_presentation::{AspectRatio, Deck, OverflowPolicy, ParseResponse};
use std::sync::Arc;

/// @covers: ParseResponse
#[test]
fn test_parse_response_holds_parsed_deck() {
    let deck = Deck {
        title: "Quarterly Review".to_string(),
        aspect_ratio: AspectRatio::Widescreen16x9,
        slides: vec![],
        overflow_policy: OverflowPolicy::Reject,
    };
    let response = ParseResponse {
        deck: Arc::new(deck),
    };
    assert_eq!(response.deck.title, "Quarterly Review");
}
