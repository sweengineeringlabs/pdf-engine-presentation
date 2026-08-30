#![allow(missing_docs)]

use pdf_engine_presentation::{parse_markdown, validate_deck, AspectRatio, OverflowPolicy};

/// @covers: parse_markdown
/// @covers: validate_deck
#[test]
fn test_parse_markdown_validate_deck_happy() {
    let deck = parse_markdown("# Slide", AspectRatio::Widescreen16x9)
        .unwrap_or_else(|error| panic!("valid deck failed: {error}"));
    assert_eq!(deck.slides.len(), 1);
    assert!(validate_deck(&deck).is_ok());
}

/// @covers: parse_markdown
#[test]
fn test_parse_markdown_error() {
    assert!(parse_markdown("", AspectRatio::Standard4x3).is_err());
}

/// @covers: validate_deck
#[test]
fn test_validate_deck_edge() {
    let mut deck = parse_markdown("# Slide", AspectRatio::Standard4x3)
        .unwrap_or_else(|error| panic!("valid deck failed: {error}"));
    deck.overflow_policy = OverflowPolicy::Clip;
    assert_eq!(validate_deck(&deck), Ok(()));
}
