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
│   ├── lib.rs       crate root — re-exports saf::*
│   ├── api/
│   │   ├── traits/  DeckParser, Validator — the two public contracts
│   │   ├── types/   Deck, Slide, SlideElement, AspectRatio, OverflowPolicy;
│   │   │            types/factory/ — FactoryDeckParser, FactoryValidator
│   │   ├── dto/     ParseRequest, ParseResponse, ValidateRequest
│   │   ├── error/   PresentationError, ValidationError (declarations only)
│   │   └── parser/  marker module pairing the `parser` domain with core/parser/
│   ├── core/
│   │   ├── parser/  DefaultMarkdownDeckParser, DefaultDeckValidator (the real
│   │   │            logic), plus the DeckParser/Validator impls for
│   │   │            FactoryDeckParser/FactoryValidator that delegate to them
│   │   └── error/   Display/Error impls for PresentationError
│   └── saf/         contract_svc (pub use api::*), deck_parser_svc_factory,
│                     validator_svc_factory — re-export the port traits
├── examples/        basic — minimal FactoryDeckParser.parse(...) call
└── tests/           one *_int_test.rs per public type/trait, plus
                      markdown_deck_parser_int_test.rs and
                      presentation_contracts_e2e_test.rs for end-to-end coverage
```

## API surface

```toml
[dependencies]
pdf-engine-presentation = "1.9"
```

```rust
use pdf_engine_presentation::{AspectRatio, DeckParser, FactoryDeckParser, ParseRequest};

let response = FactoryDeckParser.parse(ParseRequest {
    source: "+++\ntitle = \"Quarterly Review\"\naspect_ratio = \"16:9\"\n+++\n# Results\n\nRevenue increased 20%.\n---\n## Appendix".to_string(),
    default_aspect_ratio: AspectRatio::Widescreen16x9,
})?;

assert_eq!(response.deck.title, "Quarterly Review");
assert_eq!(response.deck.slides.len(), 2);
```

| Item | What it does |
|------|-------------|
| `DeckParser` | Trait: `parse(ParseRequest) -> Result<ParseResponse, PresentationError>`, `validate(ValidateRequest) -> Result<(), PresentationError>`, `validator(GetValidatorRequest) -> Result<GetValidatorResponse, PresentationError>` (default: hands back a `FactoryValidator`) |
| `FactoryDeckParser` | The deterministic Markdown `DeckParser` implementation; also what `<T as DeckParser>::factory()` returns |
| `Validator` | Trait: `validate(ValidateRequest) -> Result<(), ValidationError>` — the same fixed-canvas check as `DeckParser::validate`, wrapped in itemized diagnostics |
| `FactoryValidator` | The default `Validator` implementation; also what `<T as Validator>::factory()` returns |
| `ParseRequest { source, default_aspect_ratio }` | Input to `DeckParser::parse` |
| `ParseResponse { deck: Arc<Deck> }` | Output of `DeckParser::parse` |
| `ValidateRequest { deck: Arc<Deck> }` | Input to `DeckParser::validate` / `Validator::validate` |
| `GetValidatorRequest` | Input to `DeckParser::validator` (unit struct) |
| `GetValidatorResponse { validator: Arc<dyn Validator> }` | Output of `DeckParser::validator` |
| `Deck { title, aspect_ratio, slides, overflow_policy }` | Parsed deck |
| `Slide { elements }` | One slide's elements, in source order |
| `SlideElement::{Paragraph, Heading, Code, Notes}` | A single slide element |
| `AspectRatio::{Widescreen16x9, Standard4x3}` | Slide aspect ratio |
| `OverflowPolicy::{Reject, Clip}` | What validation does when a slide overflows |
| `PresentationError::{EmptyDeck, MalformedSource, SlideOverflow}` | Parse/validation failure, implements `std::error::Error` |
| `ValidationError { violations: Vec<String> }` | `Validator::validate`'s itemized failure |

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
- `Validator` and `DeckParser::validate` currently perform the identical
  fixed-canvas check — `Validator` exists as a separate, standard shape so
  the crate exposes a uniform validation contract, and it re-reports
  `DeckParser`'s structured `PresentationError` as a generic, itemized
  `ValidationError` rather than duplicating the validation logic itself.
