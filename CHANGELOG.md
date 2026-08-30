# Changelog

## [Unreleased]

## [1.9.7] — 2026-08-30

### Added

- Initial standalone publication of `pdf-engine-presentation`, extracted from
  the `pdf-engine` monorepo (`scm/main/port/presentation`). Deterministic
  Markdown deck parsing (`parse_markdown`) and fixed-canvas validation
  (`validate_deck`) with `Deck`, `Slide`, `SlideElement`, `AspectRatio`,
  `OverflowPolicy`, and `PresentationError` contracts. Zero runtime
  dependencies.
