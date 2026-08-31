#![allow(missing_docs)]

//! Smoke tests for the `saf/deck_parser_svc_factory.rs` facade re-export.
//! Full behavioral coverage of the underlying type lives in
//! `deck_parser_int_test.rs` and `deck_parser_factory_int_test.rs`.

use pdf_engine_presentation::{
    AspectRatio, DeckParser, FactoryDeckParser, ParseRequest, PresentationError,
};

/// @covers: FactoryDeckParser
#[test]
fn test_deck_parser_svc_factory_import_path_valid_source_happy() {
    let response = FactoryDeckParser
        .parse(ParseRequest {
            source: "# Title".to_string(),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("facade import path failed on valid input: {error}"));
    assert_eq!(response.deck.slides.len(), 1);
}

/// @covers: FactoryDeckParser
#[test]
fn test_deck_parser_svc_factory_import_path_empty_source_error() {
    assert_eq!(
        FactoryDeckParser
            .parse(ParseRequest {
                source: String::new(),
                default_aspect_ratio: AspectRatio::Standard4x3,
            })
            .err(),
        Some(PresentationError::EmptyDeck)
    );
}

/// @covers: FactoryDeckParser
#[test]
fn test_deck_parser_svc_factory_import_path_multi_slide_edge() {
    let response = FactoryDeckParser
        .parse(ParseRequest {
            source: "# One\n---\n# Two".to_string(),
            default_aspect_ratio: AspectRatio::Standard4x3,
        })
        .unwrap_or_else(|error| panic!("facade import path failed on multi-slide input: {error}"));
    assert_eq!(response.deck.slides.len(), 2);
}
