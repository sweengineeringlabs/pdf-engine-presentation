//! Presentation deck contracts and deterministic source parsing.

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;

pub use api::*;
pub use saf::{parse_markdown, validate_deck};
