#![allow(missing_docs)]

use pdf_engine_presentation::SlideElement;

/// @covers: SlideElement
#[test]
fn test_slide_element_heading_holds_level_and_text() {
    let element = SlideElement::Heading {
        level: 2,
        text: "Intro".to_string(),
    };
    match element {
        SlideElement::Heading { level, text } => {
            assert_eq!(level, 2);
            assert_eq!(text, "Intro");
        }
        other => panic!("expected Heading variant, got {other:?}"),
    }
}

/// @covers: SlideElement
#[test]
fn test_slide_element_variants_are_distinct() {
    assert_ne!(
        SlideElement::Paragraph("a".to_string()),
        SlideElement::Code("a".to_string())
    );
    assert_ne!(
        SlideElement::Notes("secret".to_string()),
        SlideElement::Paragraph("secret".to_string())
    );
}
