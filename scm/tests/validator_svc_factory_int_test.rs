#![allow(missing_docs)]

//! Smoke tests for the `saf/validator_svc_factory.rs` facade re-export.
//! Full behavioral coverage of the underlying type lives in
//! `validator_int_test.rs` and `validator_factory_int_test.rs`.

use pdf_engine_presentation::{
    AspectRatio, Deck, FactoryValidator, OverflowPolicy, Slide, ValidateRequest, Validator,
};
use std::sync::Arc;

/// @covers: FactoryValidator
#[test]
fn test_validator_svc_factory_import_path_within_budget_happy() {
    let deck = Deck {
        title: "Slide".to_string(),
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

/// @covers: FactoryValidator
#[test]
fn test_validator_svc_factory_import_path_empty_deck_error() {
    let deck = Deck {
        title: String::new(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        FactoryValidator.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Err(pdf_engine_presentation::ValidationError {
            violations: vec!["presentation deck contains no slides".to_string()]
        })
    );
}

/// @covers: FactoryValidator
#[test]
fn test_validator_svc_factory_import_path_clip_policy_edge() {
    let deck = Deck {
        title: "edge".to_string(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Clip,
    };
    assert_eq!(
        FactoryValidator.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}
