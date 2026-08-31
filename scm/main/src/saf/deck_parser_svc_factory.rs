use crate::api::{DeckParser, DeckParserFactory};
use crate::core::DefaultMarkdownDeckParser;

impl DeckParserFactory {
    /// Builds the deterministic Markdown [`DeckParser`] implementation.
    pub fn build(&self) -> impl DeckParser {
        DefaultMarkdownDeckParser
    }
}
