# pdf-engine-presentation

> **TLDR:** Standalone, dependency-free, deterministic Markdown presentation-deck parser — headings, paragraphs, fenced code, speaker notes, slide breaks, TOML/YAML front matter. No runtime dependency.

## What

A Markdown-to-`Deck` parser and validator, extracted from the
[`pdf-engine`](https://github.com/sweengineeringlabs/pdf-engine) document
engine. `parse_markdown` turns a Markdown source string into a `Deck` of
`Slide`s made of `SlideElement`s (headings, paragraphs, fenced code, speaker
notes); `validate_deck` checks the result against a deterministic fixed-canvas
line estimate. Both functions and every public type (`Deck`, `Slide`,
`SlideElement`, `AspectRatio`, `OverflowPolicy`, `PresentationError`) have zero
dependencies of their own.

## Why

`pdf-engine` needed a small, deterministic Markdown subset parser for its
presentation-deck feature and it turned out to be generally useful independent
of that: no async runtime, no regex engine, no external crate at all — just
line-oriented parsing over `&str`. Byte-identical output for byte-identical
input, every time.

## When / How

Use it when you want a slide deck's structure (title, aspect ratio, per-slide
elements) out of a small Markdown subset without pulling in a general-purpose
Markdown/CommonMark implementation.

```toml
[dependencies]
pdf-engine-presentation = "1.9"
```

```rust
use pdf_engine_presentation::{parse_markdown, validate_deck, AspectRatio};

let source = "# Quarterly review\n\nRevenue increased.\n---\n## Appendix";
let deck = parse_markdown(source, AspectRatio::Widescreen16x9)?;
validate_deck(&deck)?;
println!("{} slide(s)", deck.slides.len());
```

Slides are separated by a `---` line. Speaker notes are wrapped in
`:::notes` / `:::` and excluded from rendering by default. Optional front
matter sets `title` and `aspect_ratio` (`16:9` or `4:3`), either YAML-style
(`---` delimited, `key: value`) or TOML-style (`+++` delimited, `key = value`).

## Known limitations

- `Deck`/`Slide` model slide *structure*, not slide *rendering* — this crate
  has no HTML/PDF export of its own (that lives in `pdf-engine`'s adapter
  layer, which depends on Chromium and is not part of this crate).
- Fenced code blocks are captured as plain text; the language tag after
  ` ``` ` (e.g. `rust`, `mermaid`) is not currently preserved.
- Only `#`/`##` headings, paragraphs, fenced code, and `:::notes` blocks are
  recognized — this is a deliberate subset, not a CommonMark implementation.

## Further reading

Full crate layout and the source-level API surface live in
[`scm/README.md`](scm/README.md) — the source-code documentation, which in
turn maps to:

| Document | Description |
|----------|-------------|
| [README](scm/README.md) | Crate layout and full usage examples |
| [Architecture](docs/3-design/architecture.md) | WHAT + WHY — module layout and design rationale |
| [Developer guide](docs/4-development/developer_guide.md) | Build, test, lint, and publish workflow |
| [Changelog](CHANGELOG.md) | Version history |
| [Rustdoc](https://docs.rs/pdf-engine-presentation) | Full API reference |

## License

MIT

---

A [Software Engineering Labs](https://swelabs.io) (SWE Labs) project.
