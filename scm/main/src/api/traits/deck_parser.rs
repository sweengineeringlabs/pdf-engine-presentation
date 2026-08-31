use crate::api::{
    DeckParserFactory, GetValidatorRequest, GetValidatorResponse, ParseRequest, ParseResponse,
    PresentationError, ValidateRequest, ValidatorFactory,
};
use std::sync::Arc;

/// Parses and validates a presentation deck from source text.
pub trait DeckParser {
    /// Parses `request.source` into a `Deck`, using `request.default_aspect_ratio`
    /// unless overridden by front matter in the source.
    fn parse(&self, request: ParseRequest) -> Result<ParseResponse, PresentationError>;

    /// Validates `request.deck` against the fixed-canvas constraints.
    fn validate(&self, request: ValidateRequest) -> Result<(), PresentationError>;

    /// Returns the factory used to construct implementations of this trait.
    fn factory() -> DeckParserFactory
    where
        Self: Sized,
    {
        DeckParserFactory
    }

    /// Returns a [`Validator`](crate::api::Validator) for decks this parser
    /// produces.
    fn validator(
        &self,
        _request: GetValidatorRequest,
    ) -> Result<GetValidatorResponse, PresentationError> {
        Ok(GetValidatorResponse {
            validator: Arc::new(ValidatorFactory.build()),
        })
    }
}
