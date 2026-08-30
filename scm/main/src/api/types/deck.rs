use crate::api::types::aspect_ratio::AspectRatio;
use crate::api::types::overflow_policy::OverflowPolicy;
use crate::api::types::slide::Slide;

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
