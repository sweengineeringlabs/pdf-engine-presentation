mod deck_parser_svc_factory;
mod parser;

pub use deck_parser_svc_factory::build_deck_parser;
pub use parser::{parse_markdown, validate_deck};
