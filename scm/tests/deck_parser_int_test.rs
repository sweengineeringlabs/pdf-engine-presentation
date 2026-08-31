#![allow(missing_docs)]

use pdf_engine_presentation::{
    build_deck_parser, AspectRatio, Deck, DeckParser, OverflowPolicy, ParseRequest, ParseResponse,
    PresentationError, Slide, ValidateRequest,
};
use std::sync::Arc;

/// Hand-written test double verifying the `DeckParser` contract itself,
/// independent of the real Markdown implementation.
struct TestDeckParser;

impl DeckParser for TestDeckParser {
    fn parse(&self, request: ParseRequest) -> Result<ParseResponse, PresentationError> {
        if request.source.is_empty() {
            return Err(PresentationError::EmptyDeck);
        }
        Ok(ParseResponse {
            deck: Arc::new(Deck {
                title: request.source,
                aspect_ratio: request.default_aspect_ratio,
                slides: vec![Slide { elements: vec![] }],
                overflow_policy: OverflowPolicy::Reject,
            }),
        })
    }

    fn validate(&self, request: ValidateRequest) -> Result<(), PresentationError> {
        if request.deck.slides.is_empty() {
            return Err(PresentationError::EmptyDeck);
        }
        Ok(())
    }
}

/// @covers: DeckParser
#[test]
fn test_parse_valid_source_happy() {
    let response = TestDeckParser
        .parse(ParseRequest {
            source: "title".to_string(),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("test double failed on valid input: {error}"));
    assert_eq!(response.deck.title, "title");
    assert_eq!(response.deck.aspect_ratio, AspectRatio::Widescreen16x9);
}

/// @covers: DeckParser
#[test]
fn test_parse_empty_source_error() {
    assert_eq!(
        TestDeckParser
            .parse(ParseRequest {
                source: String::new(),
                default_aspect_ratio: AspectRatio::Standard4x3,
            })
            .err(),
        Some(PresentationError::EmptyDeck)
    );
}

/// @covers: DeckParser
#[test]
fn test_parse_minimal_source_edge() {
    // A single-character source is the smallest non-empty input the
    // contract must still accept.
    let response = TestDeckParser
        .parse(ParseRequest {
            source: "x".to_string(),
            default_aspect_ratio: AspectRatio::Standard4x3,
        })
        .unwrap_or_else(|error| panic!("test double failed on minimal input: {error}"));
    assert_eq!(response.deck.slides.len(), 1);
}

/// @covers: DeckParser
#[test]
fn test_validate_within_budget_happy() {
    let deck = Deck {
        title: "title".to_string(),
        aspect_ratio: AspectRatio::Widescreen16x9,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        TestDeckParser.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}

/// @covers: DeckParser
#[test]
fn test_validate_empty_deck_error() {
    let deck = Deck {
        title: String::new(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        TestDeckParser.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Err(PresentationError::EmptyDeck)
    );
}

/// @covers: DeckParser
#[test]
fn test_validate_clip_policy_edge() {
    let deck = Deck {
        title: "edge".to_string(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Clip,
    };
    assert_eq!(
        TestDeckParser.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}

/// @covers: build_deck_parser
#[test]
fn test_build_deck_parser_parse_matches_free_function_happy() {
    let response = build_deck_parser()
        .parse(ParseRequest {
            source: "# Slide\n\nBody text.".to_string(),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("valid source failed to parse: {error}"));
    assert_eq!(response.deck.slides.len(), 1);
    assert_eq!(response.deck.aspect_ratio, AspectRatio::Widescreen16x9);
}

/// @covers: build_deck_parser
#[test]
fn test_build_deck_parser_validate_overfull_slide_error() {
    let response = build_deck_parser()
        .parse(ParseRequest {
            source: format!("# Slide\n{}", "line\n".repeat(40)),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("overflow fixture failed to parse: {error}"));
    assert!(matches!(
        build_deck_parser().validate(ValidateRequest {
            deck: response.deck
        }),
        Err(PresentationError::SlideOverflow { .. })
    ));
}

/// @covers: build_deck_parser
#[test]
fn test_build_deck_parser_parse_preserves_deck_fields() {
    let response = build_deck_parser()
        .parse(ParseRequest {
            source: "+++\ntitle = \"Quarterly Review\"\n+++\n# Results".to_string(),
            default_aspect_ratio: AspectRatio::Standard4x3,
        })
        .unwrap_or_else(|error| panic!("front-matter source failed to parse: {error}"));
    let expected = Deck {
        title: "Quarterly Review".to_string(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: response.deck.slides.clone(),
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(*response.deck, expected);
}
