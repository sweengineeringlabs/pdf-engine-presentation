#![allow(missing_docs)]

use pdf_engine_presentation::{
    AspectRatio, DeckParser, FactoryDeckParser, ParseRequest, PresentationError, SlideElement,
    ValidateRequest,
};

/// @covers: DeckParser
#[test]
fn test_parse_notes_and_code_source_order_happy() {
    let response = FactoryDeckParser
        .parse(ParseRequest {
            source:
                "# First\nBody\n:::notes\nsecret\n:::\n---\n# Second\n```rust\nfn main() {}\n```"
                    .to_string(),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("valid fixture failed to parse: {error}"));
    assert_eq!(response.deck.slides.len(), 2);
    assert!(response.deck.slides[0]
        .elements
        .iter()
        .any(|element| matches!(element, SlideElement::Notes(_))));
    assert!(response.deck.slides[1]
        .elements
        .iter()
        .any(|element| matches!(element, SlideElement::Code(text) if text == "fn main() {}")));
}

/// @covers: DeckParser
#[test]
fn test_parse_unterminated_block_or_empty_source_error() {
    assert_eq!(
        FactoryDeckParser
            .parse(ParseRequest {
                source: String::new(),
                default_aspect_ratio: AspectRatio::Standard4x3,
            })
            .err(),
        Some(PresentationError::EmptyDeck)
    );
    assert!(matches!(
        FactoryDeckParser
            .parse(ParseRequest {
                source: "```\ncode".to_string(),
                default_aspect_ratio: AspectRatio::Standard4x3,
            })
            .err(),
        Some(PresentationError::MalformedSource(_))
    ));
}

/// @covers: DeckParser
#[test]
fn test_parse_bare_metadata_lines_preserve_slide_order_happy() {
    let response = FactoryDeckParser
        .parse(ParseRequest {
            source: "title: Quarterly Review\naspect_ratio: 4:3\n---\n# Slide".to_string(),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("valid metadata fixture failed to parse: {error}"));
    assert_eq!(response.deck.title, "Quarterly Review");
    assert_eq!(response.deck.aspect_ratio, AspectRatio::Standard4x3);
    assert_eq!(response.deck.slides.len(), 1);
}

/// @covers: DeckParser
#[test]
fn test_parse_toml_front_matter_happy() {
    let response = FactoryDeckParser
        .parse(ParseRequest {
            source: "+++\ntitle = 'Quarterly Review'\naspect_ratio = '4:3'\n+++\n# Slide"
                .to_string(),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("valid TOML metadata fixture failed: {error}"));
    assert_eq!(response.deck.title, "Quarterly Review");
    assert_eq!(response.deck.aspect_ratio, AspectRatio::Standard4x3);
}

/// @covers: DeckParser
#[test]
fn test_parse_yaml_front_matter_happy() {
    let response = FactoryDeckParser
        .parse(ParseRequest {
            source: "---\ntitle: Front Matter\naspect_ratio: 4:3\n---\n# Slide".to_string(),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("frontmatter fixture failed to parse: {error}"));
    assert_eq!(response.deck.title, "Front Matter");
    assert_eq!(response.deck.aspect_ratio, AspectRatio::Standard4x3);
    assert_eq!(response.deck.slides.len(), 1);
}

/// @covers: DeckParser
#[test]
fn test_validate_overfull_slide_error() {
    let response = FactoryDeckParser
        .parse(ParseRequest {
            source: format!("# Slide\n{}", "line\n".repeat(40)),
            default_aspect_ratio: AspectRatio::Widescreen16x9,
        })
        .unwrap_or_else(|error| panic!("valid overflow fixture failed to parse: {error}"));
    assert!(matches!(
        FactoryDeckParser.validate(ValidateRequest {
            deck: response.deck
        }),
        Err(PresentationError::SlideOverflow { .. })
    ));
}
