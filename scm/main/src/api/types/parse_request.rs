use crate::api::AspectRatio;

/// Request to parse Markdown source text into a [`Deck`](crate::api::Deck).
pub struct ParseRequest {
    /// Markdown source text.
    pub source: String,
    /// Aspect ratio used unless overridden by front matter in `source`.
    pub default_aspect_ratio: AspectRatio,
}
