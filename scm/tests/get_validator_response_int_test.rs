#![allow(missing_docs)]

use pdf_engine_presentation::{
    AspectRatio, Deck, DeckParser, DeckParserFactory, GetValidatorRequest, OverflowPolicy,
    ValidateRequest,
};
use std::sync::Arc;

/// @covers: GetValidatorResponse
#[test]
fn test_get_validator_response_holds_usable_validator() {
    let response = DeckParserFactory
        .build()
        .validator(GetValidatorRequest)
        .unwrap_or_else(|error| panic!("validator() failed: {error}"));
    let deck = Deck {
        title: String::new(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        response.validator.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Err(pdf_engine_presentation::ValidationError {
            violations: vec!["presentation deck contains no slides".to_string()]
        })
    );
}
