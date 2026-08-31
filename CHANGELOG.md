# Changelog

## [Unreleased]

### Changed

- **Breaking:** `parse_markdown` and `validate_deck` free functions are
  replaced by the `DeckParser` trait (`parse`/`validate` methods, taking
  `ParseRequest`/`ValidateRequest` and returning `ParseResponse`/`()`), built
  via `DeckParserFactory.build()`. This crate had not yet been published to
  crates.io, so there are no external callers to migrate.

### Added

- `Validator` trait (`ValidatorFactory.build()`) — a generic itemized
  validation contract (`ValidationError { violations: Vec<String> }`)
  alongside `DeckParser::validate`'s structured `PresentationError`.
- `ParseRequest`, `ParseResponse`, `ValidateRequest` DTOs; `DeckParserFactory`,
  `ValidatorFactory` marker types.

## [1.9.7] — 2026-08-30

### Added

- Initial standalone publication of `pdf-engine-presentation`, extracted from
  the `pdf-engine` monorepo (`scm/main/port/presentation`). Deterministic
  Markdown deck parsing (`parse_markdown`) and fixed-canvas validation
  (`validate_deck`) with `Deck`, `Slide`, `SlideElement`, `AspectRatio`,
  `OverflowPolicy`, and `PresentationError` contracts. Zero runtime
  dependencies.
