# Architecture

## What

`pdf-engine-presentation` is a single library crate with two modules:

- `api` — pure data types: `Deck`, `Slide`, `SlideElement`, `AspectRatio`,
  `OverflowPolicy`, `PresentationError`. No logic, no I/O.
- `saf` — the parser and validator: `parse_markdown` (source text → `Deck`)
  and `validate_deck` (`Deck` → `Result<(), PresentationError>`).

`lib.rs` re-exports both modules' public items, so callers only ever import
from the crate root (`pdf_engine_presentation::{parse_markdown, Deck, ...}`),
never from `api`/`saf` directly — those module names are an internal
organizational detail, not part of the public API.

```mermaid
graph TD
    Caller["Caller code"]
    Lib["lib.rs<br/>pub use api::*<br/>pub use saf::{parse_markdown, validate_deck}"]
    Api["api module<br/>Deck, Slide, SlideElement,<br/>AspectRatio, OverflowPolicy, PresentationError"]
    Saf["saf module<br/>parse_markdown, validate_deck"]

    Caller --> Lib
    Lib --> Api
    Lib --> Saf
    Saf --> Api
```

## Why

This crate is extracted from the `pdf-engine` monorepo's
`main/port/presentation` module, where it served as the "port" half of a
hexagonal port/adapter pair: `pdf-engine-presentation` defines the pure
parsing/validation contract, and a separate `pdf-engine-adapter-presentation`
crate (which stays inside the monorepo) implements Chromium-based PDF/HTML
rendering on top of it.

That split is why this crate has zero dependencies and no rendering code of
its own: parsing Markdown into a `Deck` and deciding whether it fits a fixed
canvas are both pure, deterministic, and independently useful — worth
depending on directly without pulling in a Chromium-based rendering pipeline.
Standing it up as its own crate makes that boundary a real one instead of an
internal convention: consumers who only need the parsing/validation contract
(for example, a project needing a similar dependency-free Markdown subset
parser) can depend on it from crates.io without a path/git dependency into a
private monorepo.

## Determinism

`parse_markdown` and `validate_deck` are both pure functions: same input,
same output, every time. There is no floating-point arithmetic, no locale-
or platform-dependent behavior, and no reliance on iteration order that isn't
already source order. `validate_deck`'s line-count estimate
(`SlideOverflow`'s `estimated_lines`) is an integer computation over `char`
counts and `\n`-delimited line counts — reproducible across platforms.

## Module data flow

```mermaid
flowchart TD
    Source["&str (Markdown source)"]
    Parse["parse_markdown<br/>strips optional front matter (+++ or ---),<br/>splits on --- slide markers, tracks fenced-code<br/>and :::notes state per line"]
    DeckNode["Deck { title, aspect_ratio, slides, overflow_policy }"]
    Validate["validate_deck<br/>sums a deterministic per-slide line estimate"]
    Overflow(["Err(PresentationError::SlideOverflow)"])
    Valid(["Ok(())"])
    Render["caller renders the validated Deck<br/>(out of scope for this crate)"]
    ParseErr(["Err: EmptyDeck / MalformedSource"])

    Source --> Parse
    Parse -->|"parse error"| ParseErr
    Parse -->|"parsed"| DeckNode
    DeckNode --> Validate
    Validate -->|"overflow_policy = Reject and over budget"| Overflow
    Validate -->|"overflow_policy = Clip, or within budget"| Valid
    Valid --> Render
```

## Error handling

`PresentationError` is a plain enum implementing `std::error::Error` and
`Display` — no `unwrap`/`expect` anywhere in the parser (enforced by
`#![deny(unsafe_code)]` at the crate level and
`unwrap_used`/`expect_used = "deny"` lint configuration in `Cargo.toml`).
Every failure path returns a `PresentationError` variant with enough context
(the offending slide number, the estimated line count, or a message) for a
caller to act on without re-parsing the source.
