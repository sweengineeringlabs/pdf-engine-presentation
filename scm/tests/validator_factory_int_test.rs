#![allow(missing_docs)]

use pdf_engine_presentation::{
    AspectRatio, Deck, OverflowPolicy, Slide, ValidateRequest, Validator, ValidatorFactory,
};
use std::sync::Arc;

/// @covers: ValidatorFactory
#[test]
fn test_validator_factory_build_returns_working_validator() {
    let deck = Deck {
        title: "title".to_string(),
        aspect_ratio: AspectRatio::Widescreen16x9,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        ValidatorFactory.build().validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}
