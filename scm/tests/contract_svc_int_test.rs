#![allow(missing_docs)]

use pdf_engine_presentation::{AspectRatio, Deck, OverflowPolicy, PresentationError, Slide};

/// @covers: contract_svc
#[test]
fn test_contract_svc_reexported_deck_type_happy() {
    let deck = Deck {
        title: "Contract re-export smoke test".to_string(),
        aspect_ratio: AspectRatio::Widescreen16x9,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(deck.slides.len(), 1);
}

/// @covers: contract_svc
#[test]
fn test_contract_svc_reexported_error_display_error() {
    assert_eq!(
        PresentationError::EmptyDeck.to_string(),
        "presentation deck contains no slides"
    );
}

/// @covers: contract_svc
#[test]
fn test_contract_svc_reexported_deck_empty_slides_edge() {
    let deck = Deck {
        title: String::new(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![],
        overflow_policy: OverflowPolicy::Clip,
    };
    assert!(deck.slides.is_empty());
}
