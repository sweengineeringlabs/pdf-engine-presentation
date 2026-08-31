mod dto;
mod error;
mod parser;
mod traits;
mod types;

pub use dto::{
    GetValidatorRequest, GetValidatorResponse, ParseRequest, ParseResponse, ValidateRequest,
};
pub use error::{PresentationError, ValidationError};
pub use traits::{DeckParser, Validator};
pub use types::{
    AspectRatio, Deck, DeckParserFactory, OverflowPolicy, Slide, SlideElement, ValidatorFactory,
};
