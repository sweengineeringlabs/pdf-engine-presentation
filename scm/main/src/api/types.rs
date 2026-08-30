use std::fmt;

/// Slide aspect ratio.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AspectRatio {
    /// Widescreen 16:9 slides.
    Widescreen16x9,
    /// Traditional 4:3 slides.
    Standard4x3,
}

/// A supported slide element.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SlideElement {
    /// Plain text content.
    Paragraph(String),
    /// Heading content and level.
    Heading {
        /// Heading level.
        level: u8,
        /// Heading text.
        text: String,
    },
    /// Fenced code content.
    Code(String),
    /// Speaker notes excluded from slide rendering by default.
    Notes(String),
}

/// Policy used when estimated slide content exceeds the fixed canvas.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OverflowPolicy {
    /// Reject the deck with an actionable validation error.
    Reject,
    /// Keep the fixed canvas and clip content at its boundary.
    Clip,
}

/// Parsed presentation slide.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Slide {
    /// Elements in source order.
    pub elements: Vec<SlideElement>,
}

/// Parsed presentation deck.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Deck {
    /// Deck title.
    pub title: String,
    /// Configured aspect ratio.
    pub aspect_ratio: AspectRatio,
    /// Slides in source order.
    pub slides: Vec<Slide>,
    /// Content overflow behavior for PDF export.
    pub overflow_policy: OverflowPolicy,
}

/// Presentation parsing and validation error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PresentationError {
    /// No slide content was found.
    EmptyDeck,
    /// A slide marker or fenced block is malformed.
    MalformedSource(String),
    /// Estimated content exceeds the configured fixed slide canvas.
    SlideOverflow {
        /// One-based slide number.
        slide: usize,
        /// Deterministic estimated line count.
        estimated_lines: usize,
    },
}

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
