use crate::api::DeckParser;
use crate::core::MarkdownDeckParser;

/// Builds the deterministic Markdown [`DeckParser`] implementation.
pub fn build_deck_parser() -> impl DeckParser {
    MarkdownDeckParser
}
