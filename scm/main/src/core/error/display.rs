use crate::api::PresentationError;
use std::fmt;

impl fmt::Display for PresentationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDeck => formatter.write_str("presentation deck contains no slides"),
            Self::MalformedSource(message) => formatter.write_str(message),
            Self::SlideOverflow {
                slide,
                estimated_lines,
            } => write!(
                formatter,
                "slide {slide} exceeds the fixed canvas (estimated {estimated_lines} lines)"
            ),
        }
    }
}

impl std::error::Error for PresentationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_empty_deck_happy() {
        assert_eq!(
            PresentationError::EmptyDeck.to_string(),
            "presentation deck contains no slides"
        );
    }

    #[test]
    fn test_fmt_malformed_source_happy() {
        assert_eq!(
            PresentationError::MalformedSource("bad input".to_string()).to_string(),
            "bad input"
        );
    }

    #[test]
    fn test_fmt_slide_overflow_edge() {
        assert_eq!(
            PresentationError::SlideOverflow {
                slide: 3,
                estimated_lines: 40,
            }
            .to_string(),
            "slide 3 exceeds the fixed canvas (estimated 40 lines)"
        );
    }
}
