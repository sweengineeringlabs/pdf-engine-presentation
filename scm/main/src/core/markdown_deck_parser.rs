use crate::api::{DeckParser, ParseRequest, ParseResponse, PresentationError, ValidateRequest};
use crate::saf;
use std::sync::Arc;

/// Deterministic Markdown implementation of [`DeckParser`].
pub(crate) struct MarkdownDeckParser;

impl DeckParser for MarkdownDeckParser {
    fn parse(&self, request: ParseRequest) -> Result<ParseResponse, PresentationError> {
        let deck = saf::parse_markdown(&request.source, request.default_aspect_ratio)?;
        Ok(ParseResponse {
            deck: Arc::new(deck),
        })
    }

    fn validate(&self, request: ValidateRequest) -> Result<(), PresentationError> {
        saf::validate_deck(&request.deck)
    }
}
