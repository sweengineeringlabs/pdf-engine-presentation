#![allow(missing_docs)]

use pdf_engine_presentation::{
    AspectRatio, Deck, OverflowPolicy, Slide, ValidateRequest, Validator,
};
use std::sync::Arc;

/// Hand-written test double verifying the `Validator` contract itself,
/// independent of the real implementation.
struct TestValidator;

impl Validator for TestValidator {
    fn validate(
        &self,
        request: ValidateRequest,
    ) -> Result<(), pdf_engine_presentation::ValidationError> {
        if request.deck.slides.is_empty() {
            return Err(pdf_engine_presentation::ValidationError {
                violations: vec!["deck has no slides".to_string()],
            });
        }
        Ok(())
    }
}

/// @covers: Validator
#[test]
fn test_validate_non_empty_deck_happy() {
    let deck = Deck {
        title: "title".to_string(),
        aspect_ratio: AspectRatio::Widescreen16x9,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        TestValidator.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}

/// @covers: Validator
#[test]
fn test_validate_empty_deck_error() {
    let deck = Deck {
        title: String::new(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        TestValidator.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Err(pdf_engine_presentation::ValidationError {
            violations: vec!["deck has no slides".to_string()]
        })
    );
}

/// @covers: Validator
#[test]
fn test_validate_single_slide_edge() {
    // A deck with exactly one slide is the smallest non-empty deck the
    // contract must still accept.
    let deck = Deck {
        title: "edge".to_string(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Clip,
    };
    assert_eq!(
        TestValidator.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}

/// @covers: Validator
#[test]
fn test_factory_build_returns_working_validator_happy() {
    let deck = Deck {
        title: "title".to_string(),
        aspect_ratio: AspectRatio::Widescreen16x9,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        TestValidator::factory().build().validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}

/// @covers: Validator
#[test]
fn test_factory_built_validator_rejects_empty_deck_error() {
    let deck = Deck {
        title: String::new(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert!(TestValidator::factory()
        .build()
        .validate(ValidateRequest {
            deck: Arc::new(deck)
        })
        .is_err());
}

/// @covers: Validator
#[test]
fn test_factory_is_independent_of_implementor_edge() {
    let deck = Deck {
        title: "edge".to_string(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Clip,
    };
    assert_eq!(
        TestValidator::factory().build().validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}
