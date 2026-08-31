#![allow(missing_docs)]

use pdf_engine_presentation::{AspectRatio, DeckParser, FactoryDeckParser, ParseRequest};

/// @covers: FactoryDeckParser
#[test]
fn test_deck_parser_factory_implements_deck_parser() {
    let response = FactoryDeckParser
        .parse(ParseRequest {
            source: "# Slide".to_string(),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("built parser failed on valid input: {error}"));
    assert_eq!(response.deck.slides.len(), 1);
}
