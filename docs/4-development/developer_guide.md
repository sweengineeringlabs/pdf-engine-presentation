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

1. Add the type/function under `main/src/api/` (data) or `main/src/saf/`
   (logic) and re-export it from `lib.rs` if it isn't already covered by a
   `pub use *`.
2. Document every public item — `#![warn(missing_docs)]` is set crate-wide.
3. Add a unit test alongside the implementation (in the same module's
   `#[cfg(test)] mod tests`) and, for anything reachable from the crate root,
   an integration test in `tests/presentation_contracts_e2e_test.rs` with an
   `/// @covers: <function_name>` doc comment.
4. Update [`scm/README.md`](../../scm/README.md)'s API surface table and, if
   the change affects design rationale, [`architecture.md`](../3-design/architecture.md).
5. Add a `CHANGELOG.md` entry under `[Unreleased]`.
