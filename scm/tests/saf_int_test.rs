#![allow(missing_docs)]

use pdf_engine_presentation::{
    parse_markdown, validate_deck, AspectRatio, PresentationError, SlideElement,
};

/// @covers: parse_markdown
#[test]
fn test_parse_markdown_notes_and_code_source_order_happy() {
    let deck = parse_markdown(
        "# First\nBody\n:::notes\nsecret\n:::\n---\n# Second\n```rust\nfn main() {}\n```",
        AspectRatio::Widescreen16x9,
    )
    .unwrap_or_else(|error| panic!("valid fixture failed to parse: {error}"));
    assert_eq!(deck.slides.len(), 2);
    assert!(deck.slides[0]
        .elements
        .iter()
        .any(|element| matches!(element, SlideElement::Notes(_))));
    assert!(deck.slides[1]
        .elements
        .iter()
        .any(|element| matches!(element, SlideElement::Code(text) if text == "fn main() {}")));
}

/// @covers: parse_markdown
#[test]
fn test_parse_markdown_unterminated_block_or_empty_source_error() {
    assert_eq!(
        parse_markdown("", AspectRatio::Standard4x3),
        Err(PresentationError::EmptyDeck)
    );
    assert!(matches!(
        parse_markdown("```\ncode", AspectRatio::Standard4x3),
        Err(PresentationError::MalformedSource(_))
    ));
}

/// @covers: parse_markdown
#[test]
fn test_parse_markdown_bare_metadata_lines_preserve_slide_order_happy() {
    let deck = parse_markdown(
        "title: Quarterly Review\naspect_ratio: 4:3\n---\n# Slide",
        AspectRatio::Widescreen16x9,
    )
    .unwrap_or_else(|error| panic!("valid metadata fixture failed to parse: {error}"));
    assert_eq!(deck.title, "Quarterly Review");
    assert_eq!(deck.aspect_ratio, AspectRatio::Standard4x3);
    assert_eq!(deck.slides.len(), 1);
}

/// @covers: parse_markdown
#[test]
fn test_parse_markdown_toml_front_matter_happy() {
    let deck = parse_markdown(
        "+++\ntitle = 'Quarterly Review'\naspect_ratio = '4:3'\n+++\n# Slide",
        AspectRatio::Widescreen16x9,
    )
    .unwrap_or_else(|error| panic!("valid TOML metadata fixture failed: {error}"));
    assert_eq!(deck.title, "Quarterly Review");
    assert_eq!(deck.aspect_ratio, AspectRatio::Standard4x3);
}

/// @covers: parse_markdown
#[test]
fn test_parse_markdown_yaml_front_matter_happy() {
    let deck = parse_markdown(
        "---\ntitle: Front Matter\naspect_ratio: 4:3\n---\n# Slide",
        AspectRatio::Widescreen16x9,
    )
    .unwrap_or_else(|error| panic!("frontmatter fixture failed to parse: {error}"));
    assert_eq!(deck.title, "Front Matter");
    assert_eq!(deck.aspect_ratio, AspectRatio::Standard4x3);
    assert_eq!(deck.slides.len(), 1);
}

/// @covers: validate_deck
#[test]
fn test_validate_deck_overfull_slide_error() {
    let deck = parse_markdown(
        &format!("# Slide\n{}", "line\n".repeat(40)),
        AspectRatio::Widescreen16x9,
    )
    .unwrap_or_else(|error| panic!("valid overflow fixture failed to parse: {error}"));
    assert!(matches!(
        validate_deck(&deck),
        Err(PresentationError::SlideOverflow { .. })
    ));
}
