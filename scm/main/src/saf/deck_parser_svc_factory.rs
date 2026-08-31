//! Re-exports the [`DeckParser`](crate::api::DeckParser) port trait through
//! the facade. The production implementation is
//! [`FactoryDeckParser`](crate::api::FactoryDeckParser), reachable via the
//! crate root.

pub use crate::api::DeckParser;
