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
