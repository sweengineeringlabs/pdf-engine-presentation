#![allow(missing_docs)]

use pdf_engine_presentation::PresentationError;

/// @covers: PresentationError
#[test]
fn test_presentation_error_display_messages() {
    assert_eq!(
        PresentationError::EmptyDeck.to_string(),
        "presentation deck contains no slides"
    );
    assert_eq!(
        PresentationError::MalformedSource("bad input".to_string()).to_string(),
        "bad input"
    );
    assert_eq!(
        PresentationError::SlideOverflow {
            slide: 3,
            estimated_lines: 40,
        }
        .to_string(),
        "slide 3 exceeds the fixed canvas (estimated 40 lines)"
    );
}

/// @covers: PresentationError
#[test]
fn test_presentation_error_variants_are_distinct() {
    assert_ne!(
        PresentationError::EmptyDeck,
        PresentationError::MalformedSource("x".to_string())
    );
}
