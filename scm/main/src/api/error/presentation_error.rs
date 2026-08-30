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
