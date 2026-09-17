# Changelog

## [1.98.1] — 2026-09-17

### Changed

- **Breaking:** `parse_markdown` and `validate_deck` free functions are
  replaced by the `DeckParser` trait (`parse`/`validate` methods, taking
  `ParseRequest`/`ValidateRequest` and returning `ParseResponse`/`()`),
  implemented directly by `FactoryDeckParser`. This crate had not yet been
  published to crates.io, so there are no external callers to migrate.
- **Breaking:** the factory marker types are named `FactoryDeckParser` and
  `FactoryValidator` (trait-name suffix), not `DeckParserFactory` /
  `ValidatorFactory` — required by this org's `core_impl_name_has_trait_suffix`
  architecture rule, since these types implement `DeckParser`/`Validator`
  directly in `core/`.
- **Breaking:** MSRV raised from `1.95` to `1.98.1` (`rust-version` in
  `Cargo.toml`).

### Added

- `Validator` trait, implemented directly by `FactoryValidator` — a generic
  itemized validation contract (`ValidationError { violations: Vec<String> }`)
  alongside `DeckParser::validate`'s structured `PresentationError`.
- `ParseRequest`, `ParseResponse`, `ValidateRequest` DTOs; `FactoryDeckParser`,
  `FactoryValidator` marker/entry-point types.
- `DeckParser::validator()` default method — recovers a `Validator` from any
  `DeckParser` implementor.

### Known limitations (arch audit)

This crate is audited against this org's internal `arch` SEA-compliance tool.
Two findings remain open — not code defects, but either provable
contradictions between two of the tool's own rules, or a documented gap
between a rule's stated behavior and its runtime behavior. A third,
previously listed here (`app_type_forbids_saf` vs `app_type_requires_saf`
reporting mutually exclusive application types for this crate in the same
audit run), was resolved by the `saf/` -> `core/` restructuring in
`ba7d9ea` and no longer fires.

- `api_impl_public_tests_external` fires despite its own documented edge case
  ("If api/ has no standalone pub fn ... this rule does not fire") — `api/`
  has zero standalone `pub fn` (trait declarations only), confirmed via
  `grep -rn "pub fn" main/src/api`.
- `saf_impl_public_tests_external` / `trait_svc_fn_scenario_coverage` vs
  `encapsulation.package_access_violation`: the latter bans re-exporting
  *concrete types* from `saf/` (only trait re-exports are permitted); the
  former two only recognize a re-exported *struct* as a valid "pub item" in a
  `*_svc_factory.rs` file, not a re-exported trait. No arrangement of
  `saf/deck_parser_svc_factory.rs` / `saf/validator_svc_factory.rs` satisfies
  both rule families simultaneously — verified empirically in both
  directions, including a `pub fn -> impl DeckParser` factory function
  (opaque return type, no concrete-type re-export): it does not satisfy
  `trait_svc_fn_scenario_coverage`, and its presence causes the audit run to
  silently drop 61 unrelated structural rules from the report rather than
  fail or skip them, so it is a net regression, not a workaround.

Separately, `test_coverage_threshold` and `test_mutation_score` both report
`INTERNAL ERROR: builtin handler '<rule>' not registered` on this `arch`
build — a gap in the `arch` binary itself (missing `cargo-llvm-cov` /
`cargo-mutants` handler registration), not a finding about this crate.

## [1.9.7] — 2026-08-30

### Added

- Initial standalone publication of `pdf-engine-presentation`, extracted from
  the `pdf-engine` monorepo (`scm/main/port/presentation`). Deterministic
  Markdown deck parsing (`parse_markdown`) and fixed-canvas validation
  (`validate_deck`) with `Deck`, `Slide`, `SlideElement`, `AspectRatio`,
  `OverflowPolicy`, and `PresentationError` contracts. Zero runtime
  dependencies.
