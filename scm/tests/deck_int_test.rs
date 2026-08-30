#![allow(missing_docs)]

use pdf_engine_presentation::{AspectRatio, Deck, OverflowPolicy, Slide};

/// @covers: Deck
#[test]
fn test_deck_holds_title_aspect_ratio_and_slides() {
    let deck = Deck {
        title: "Quarterly Review".to_string(),
        aspect_ratio: AspectRatio::Widescreen16x9,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(deck.title, "Quarterly Review");
    assert_eq!(deck.aspect_ratio, AspectRatio::Widescreen16x9);
    assert_eq!(deck.slides.len(), 1);
    assert_eq!(deck.overflow_policy, OverflowPolicy::Reject);
}

/// @covers: Deck
#[test]
fn test_deck_with_different_overflow_policies_are_not_equal() {
    let base = Deck {
        title: "Deck".to_string(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![],
        overflow_policy: OverflowPolicy::Reject,
    };
    let clipped = Deck {
        overflow_policy: OverflowPolicy::Clip,
        ..base.clone()
    };
    assert_ne!(base, clipped);
}
