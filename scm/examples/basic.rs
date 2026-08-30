#![allow(missing_docs)]

use pdf_engine_presentation::{parse_markdown, AspectRatio};

fn main() {
    let source = "# Quarterly review\n\nRevenue increased.\n---\n## Appendix";
    match parse_markdown(source, AspectRatio::Widescreen16x9) {
        Ok(deck) => println!("parsed {} slides", deck.slides.len()),
        Err(error) => eprintln!("presentation parse failed: {error}"),
    }
}
