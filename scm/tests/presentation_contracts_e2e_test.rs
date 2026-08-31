#![allow(missing_docs)]

use pdf_engine_presentation::{
    AspectRatio, DeckParser, DeckParserFactory, OverflowPolicy, ParseRequest, ValidateRequest,
};
use std::sync::Arc;

/// @covers: DeckParser
#[test]
fn test_parse_markdown_validate_deck_happy() {
    let parser = DeckParserFactory.build();
    let response = parser
        .parse(ParseRequest {
            source: "# Slide".to_string(),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("valid deck failed: {error}"));
    assert_eq!(response.deck.slides.len(), 1);
    assert!(parser
        .validate(ValidateRequest {
            deck: response.deck
        })
        .is_ok());
}

/// @covers: DeckParser
#[test]
fn test_parse_markdown_error() {
    assert!(DeckParserFactory
        .build()
        .parse(ParseRequest {
            source: String::new(),
            default_aspect_ratio: AspectRatio::Standard4x3,
        })
        .is_err());
}

/// @covers: DeckParser
#[test]
fn test_validate_deck_edge() {
    let parser = DeckParserFactory.build();
    let response = parser
        .parse(ParseRequest {
            source: "# Slide".to_string(),
            default_aspect_ratio: AspectRatio::Standard4x3,
        })
        .unwrap_or_else(|error| panic!("valid deck failed: {error}"));
    let mut deck = (*response.deck).clone();
    deck.overflow_policy = OverflowPolicy::Clip;
    assert_eq!(
        parser.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}
