#![allow(missing_docs)]

use pdf_engine_presentation::{AspectRatio, DeckParser, FactoryDeckParser, ParseRequest};

fn main() {
    let source = "# Quarterly review\n\nRevenue increased.\n---\n## Appendix";
    let request = ParseRequest {
        source: source.to_string(),
        default_aspect_ratio: AspectRatio::Widescreen16x9,
    };
    match FactoryDeckParser.parse(request) {
        Ok(response) => println!("parsed {} slides", response.deck.slides.len()),
        Err(error) => eprintln!("presentation parse failed: {error}"),
    }
}
