#![allow(missing_docs)]

use pdf_engine_presentation::{
    AspectRatio, Deck, DeckParser, FactoryDeckParser, GetValidatorRequest, OverflowPolicy, Slide,
    ValidateRequest,
};
use std::sync::Arc;

/// @covers: GetValidatorRequest
#[test]
fn test_get_validator_request_is_accepted_by_deck_parser() {
    // GetValidatorRequest carries no data of its own; its only role is to be
    // a valid request value DeckParser::validator accepts and act on.
    let validator = FactoryDeckParser
        .validator(GetValidatorRequest)
        .unwrap_or_else(|error| panic!("validator() rejected GetValidatorRequest: {error}"))
        .validator;
    let deck = Deck {
        title: "title".to_string(),
        aspect_ratio: AspectRatio::Widescreen16x9,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        validator.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}
