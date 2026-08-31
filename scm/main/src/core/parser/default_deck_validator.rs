use crate::api::{DeckParser, ValidateRequest, ValidationError, Validator};
use crate::core::DefaultMarkdownDeckParser;

/// Adapts [`DefaultMarkdownDeckParser`]'s fixed-canvas validation to the
/// generic [`Validator`] contract, collecting the single structural failure
/// it can report as an itemized [`ValidationError`].
pub(crate) struct DefaultDeckValidator;

impl Validator for DefaultDeckValidator {
    fn validate(&self, request: ValidateRequest) -> Result<(), ValidationError> {
        DefaultMarkdownDeckParser
            .validate(ValidateRequest { deck: request.deck })
            .map_err(|error| ValidationError {
                violations: vec![error.to_string()],
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{AspectRatio, Deck, OverflowPolicy, Slide};
    use std::sync::Arc;

    #[test]
    fn test_validate_within_budget_happy() {
        let deck = Deck {
            title: "Slide".to_string(),
            aspect_ratio: AspectRatio::Widescreen16x9,
            slides: vec![Slide { elements: vec![] }],
            overflow_policy: OverflowPolicy::Reject,
        };
        assert_eq!(
            DefaultDeckValidator.validate(ValidateRequest {
                deck: Arc::new(deck)
            }),
            Ok(())
        );
    }

    #[test]
    fn test_validate_empty_deck_error() {
        let deck = Deck {
            title: String::new(),
            aspect_ratio: AspectRatio::Standard4x3,
            slides: vec![],
            overflow_policy: OverflowPolicy::Reject,
        };
        assert_eq!(
            DefaultDeckValidator.validate(ValidateRequest {
                deck: Arc::new(deck)
            }),
            Err(ValidationError {
                violations: vec!["presentation deck contains no slides".to_string()]
            })
        );
    }

    #[test]
    fn test_validate_clip_policy_edge() {
        let deck = Deck {
            title: "edge".to_string(),
            aspect_ratio: AspectRatio::Standard4x3,
            slides: vec![Slide { elements: vec![] }],
            overflow_policy: OverflowPolicy::Clip,
        };
        assert_eq!(
            DefaultDeckValidator.validate(ValidateRequest {
                deck: Arc::new(deck)
            }),
            Ok(())
        );
    }
}
