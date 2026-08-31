mod error;
mod traits;
mod types;

pub use error::PresentationError;
pub use traits::DeckParser;
pub use types::{
    AspectRatio, Deck, OverflowPolicy, ParseRequest, ParseResponse, Slide, SlideElement,
    ValidateRequest,
};
