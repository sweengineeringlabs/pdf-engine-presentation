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
