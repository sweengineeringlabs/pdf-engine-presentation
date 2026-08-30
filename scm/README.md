# Source

This is the Cargo package for `pdf-engine-presentation` — see the
[root README](../README.md) for what/why/how. This file documents the crate
layout and the full API surface; for the design rationale and the development
workflow, see:

- [Architecture](../docs/3-design/architecture.md)
- [Developer guide](../docs/4-development/developer_guide.md)

## Crate layout

```
scm/
├── Cargo.toml       package manifest ([lib] path = main/src/lib.rs, license = "MIT")
├── deny.toml        license/advisory policy (cargo deny check --config deny.toml)
├── bootstrap.sh / bootstrap.ps1   installs git hooks, fetches dependencies
├── scripts/hooks/   pre-commit (fmt/clippy/test), commit-msg (AI-attribution guard)
├── main/src/
│   ├── lib.rs       crate root — re-exports api::* and saf::{parse_markdown, validate_deck}
│   ├── api/         Deck/Slide/SlideElement/AspectRatio/OverflowPolicy/PresentationError
│   └── saf/         parse_markdown, validate_deck, and their unit tests
├── examples/        basic — minimal parse_markdown call
└── tests/           presentation_contracts_e2e_test — public-API integration tests
```

## API surface

```toml
[dependencies]
pdf-engine-presentation = "1.9"
```

```rust
use pdf_engine_presentation::{parse_markdown, validate_deck, AspectRatio, OverflowPolicy};

let deck = parse_markdown(
    "+++\ntitle = \"Quarterly Review\"\naspect_ratio = \"16:9\"\n+++\n# Results\n\nRevenue increased 20%.\n---\n## Appendix",
    AspectRatio::Widescreen16x9,
)?;

assert_eq!(deck.title, "Quarterly Review");
assert_eq!(deck.slides.len(), 2);

validate_deck(&deck)?;
```

| Item | What it does |
|------|-------------|
| `parse_markdown(source, default_aspect_ratio)` | Parses Markdown into a `Deck`; front matter overrides `default_aspect_ratio` |
| `validate_deck(&deck)` | Rejects a deck whose estimated per-slide line count exceeds the fixed canvas, unless `overflow_policy` is `Clip` |
| `Deck { title, aspect_ratio, slides, overflow_policy }` | Parsed deck |
| `Slide { elements }` | One slide's elements, in source order |
| `SlideElement::{Paragraph, Heading, Code, Notes}` | A single slide element |
| `AspectRatio::{Widescreen16x9, Standard4x3}` | Slide aspect ratio |
| `OverflowPolicy::{Reject, Clip}` | What `validate_deck` does when a slide overflows |
| `PresentationError::{EmptyDeck, MalformedSource, SlideOverflow}` | Parse/validation failure, implements `std::error::Error` |

### Source syntax

- Slides are separated by a line containing only `---`.
- `# Heading` and `## Heading` produce `SlideElement::Heading { level, text }`;
  any other non-empty, non-fenced line becomes a `SlideElement::Paragraph`.
- ` ``` `-fenced blocks become a single `SlideElement::Code` (language tags
  after the opening fence are not currently captured).
- `:::notes` / `:::` blocks become `SlideElement::Notes` lines, excluded from
  rendering by default but retained in the parsed deck.
- Optional front matter, before the first slide, sets `title` and
  `aspect_ratio` (`16:9`/`16x9` or `4:3`/`4x3`):
  - YAML-style: `---` delimited block of `key: value` lines.
  - TOML-style: `+++` delimited block of `key = value` lines.
  - Or bare `title:` / `aspect_ratio:` lines before the first slide, with no
    delimiters at all.

### Known limitations

- No HTML/PDF rendering — this crate only produces the `Deck` data structure.
  Rendering a parsed deck to slide HTML/PDF is out of scope; see the parent
  [`pdf-engine`](https://github.com/sweengineeringlabs/pdf-engine) project's
  presentation adapter for that.
- Fenced code blocks do not currently preserve the language tag (e.g. `rust`
  vs `mermaid`) — all fenced content collapses into one `SlideElement::Code`.
- This is a deliberate Markdown subset, not a CommonMark implementation:
  only `#`/`##` headings, paragraphs, fenced code, and `:::notes` blocks are
  recognized. Lists, emphasis, links, tables, and nested headings (`###`+) are
  not parsed as such — they pass through as plain paragraph text.
