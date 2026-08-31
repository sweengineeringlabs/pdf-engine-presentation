#![allow(missing_docs)]

use pdf_engine_presentation::{
    AspectRatio, Deck, DeckParser, DeckParserFactory, GetValidatorRequest, OverflowPolicy,
    ParseRequest, ParseResponse, PresentationError, Slide, ValidateRequest,
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

    fn validator(
        &self,
        _request: GetValidatorRequest,
    ) -> Result<pdf_engine_presentation::GetValidatorResponse, PresentationError> {
        // This test double deliberately overrides the default to exercise
        // the error path: it has no validator of its own to offer.
        Err(PresentationError::MalformedSource(
            "test double has no validator configured".to_string(),
        ))
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

/// @covers: DeckParser
#[test]
fn test_factory_build_returns_working_parser_happy() {
    let factory: DeckParserFactory = TestDeckParser::factory();
    let parser = factory.build();
    let response = parser
        .parse(ParseRequest {
            source: "# Slide".to_string(),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("factory-built parser failed on valid input: {error}"));
    assert_eq!(response.deck.slides.len(), 1);
}

/// @covers: DeckParser
#[test]
fn test_factory_built_parser_rejects_empty_source_error() {
    let parser = TestDeckParser::factory().build();
    assert_eq!(
        parser
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
fn test_factory_is_independent_of_implementor_edge() {
    // The default factory() always returns a DeckParserFactory that builds
    // the crate's own DefaultMarkdownDeckParser, regardless of which
    // DeckParser implementor Self is -- confirmed here by calling it on
    // TestDeckParser rather than the real implementation.
    let deck = Deck {
        title: String::new(),
        aspect_ratio: AspectRatio::Widescreen16x9,
        slides: vec![Slide { elements: vec![] }],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        TestDeckParser::factory().build().validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Ok(())
    );
}

/// @covers: DeckParser
#[test]
fn test_validator_default_impl_returns_working_validator_happy() {
    // The default `validator()` implementation (inherited by the real
    // factory-built parser) must return a Validator that actually validates.
    let parser = DeckParserFactory.build();
    let validator = parser
        .validator(GetValidatorRequest)
        .unwrap_or_else(|error| panic!("default validator() failed: {error}"))
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

/// @covers: DeckParser
#[test]
fn test_validator_unavailable_error() {
    assert!(matches!(
        TestDeckParser.validator(GetValidatorRequest),
        Err(PresentationError::MalformedSource(_))
    ));
}

/// @covers: DeckParser
#[test]
fn test_validator_default_impl_detects_empty_deck_edge() {
    // The validator returned by the default implementation must apply the
    // same fixed-canvas rule as DeckParser::validate itself, including on
    // the boundary case of an empty deck.
    let parser = DeckParserFactory.build();
    let validator = parser
        .validator(GetValidatorRequest)
        .unwrap_or_else(|error| panic!("default validator() failed: {error}"))
        .validator;
    let deck = Deck {
        title: String::new(),
        aspect_ratio: AspectRatio::Standard4x3,
        slides: vec![],
        overflow_policy: OverflowPolicy::Reject,
    };
    assert_eq!(
        validator.validate(ValidateRequest {
            deck: Arc::new(deck)
        }),
        Err(pdf_engine_presentation::ValidationError {
            violations: vec!["presentation deck contains no slides".to_string()]
        })
    );
}
