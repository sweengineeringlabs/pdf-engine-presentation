#![allow(missing_docs)]

use pdf_engine_presentation::{Slide, SlideElement};

/// @covers: Slide
#[test]
fn test_slide_holds_elements_in_source_order() {
    let slide = Slide {
        elements: vec![
            SlideElement::Paragraph("first".to_string()),
            SlideElement::Paragraph("second".to_string()),
        ],
    };
    assert_eq!(slide.elements.len(), 2);
    assert_eq!(
        slide.elements[0],
        SlideElement::Paragraph("first".to_string())
    );
    assert_eq!(
        slide.elements[1],
        SlideElement::Paragraph("second".to_string())
    );
}

/// @covers: Slide
#[test]
fn test_slide_with_no_elements_is_empty() {
    let slide = Slide { elements: vec![] };
    assert!(slide.elements.is_empty());
}
