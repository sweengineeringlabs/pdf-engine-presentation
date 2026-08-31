#![allow(missing_docs)]

use pdf_engine_presentation::{
    AspectRatio, Deck, FactoryValidator, OverflowPolicy, Slide, ValidateRequest, Validator,
};
use std::sync::Arc;

/// @covers: FactoryValidator
#[test]
fn test_validator_factory_implements_validator() {
    let deck = Deck {
        title: "title".to_string(),
        aspect_ratio: AspectRatio::Widescreen16x9,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        FactoryValidator.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}
