# Developer guide

## Setup

```sh
./scm/bootstrap.sh     # or: pwsh scm/bootstrap.ps1 on Windows
```

Installs git hooks (`core.hooksPath` → `scm/scripts/hooks`) and fetches
dependencies. The `pre-commit` hook runs `cargo fmt --check`, `cargo clippy
-D warnings`, and `cargo test`; the `commit-msg` hook rejects commits that
carry AI-assistant attribution (see [Commit policy](#commit-policy) below).

## Build, test, lint

All commands run from `scm/`, the package root:

```sh
cargo build --all-targets
cargo test --all-targets
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo deny check --config deny.toml
```

## Publishing

```sh
cargo publish --dry-run   # verify packaging before every real publish
cargo publish
```

`Cargo.toml` sets `publish = true` explicitly and pins `version` — bump it
(and add a `CHANGELOG.md` entry) before every publish. This crate has zero
dependencies, so there is no publish-order concern with anything else.

## Commit policy

No commit, PR description, or code comment may credit an AI assistant
(`Co-Authored-By: <ai>`, "Generated with ...", a 🤖 footer, etc.). This is
enforced twice:

- Locally, by the `commit-msg` git hook installed via bootstrap — it rejects
  the commit outright, before it's even made.
- On GitHub, by the `No AI Attribution` workflow
  (`.github/workflows/no-ai-attribution.yml`) — it scans both the PR
  description and every commit message in the PR, and cannot be bypassed with
  `git commit --no-verify`.

## Adding a new public API item

1. New data types go in their own file under `main/src/api/types/` (or
   `main/src/api/error/` for error types) — one public type per file, named
   after the type in snake_case — and are re-exported by name (no globs) from
   `api/mod.rs` and then `lib.rs`. New parsing/validation logic goes in
   `main/src/saf/parser.rs`; `saf/mod.rs` itself stays a pure re-export.
2. Document every public item — `#![warn(missing_docs)]` is set crate-wide.
3. `api/` and `saf/` source files must not contain `#[cfg(test)]` blocks —
   add a test file under `tests/` instead (e.g. `tests/<type>_int_test.rs`
   for a new api/ type, or extend `tests/saf_int_test.rs` /
   `tests/presentation_contracts_e2e_test.rs` for parser behavior), with an
   `/// @covers: <item_name>` doc comment on each test.
4. Update [`scm/README.md`](../../scm/README.md)'s API surface table and, if
   the change affects design rationale, [`architecture.md`](../3-design/architecture.md).
5. Add a `CHANGELOG.md` entry under `[Unreleased]`.
