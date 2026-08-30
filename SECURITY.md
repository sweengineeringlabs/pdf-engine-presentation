# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.9.x   | Yes       |

## Reporting a Vulnerability

Report security vulnerabilities to **engineers@swelabs.io**.

Please include:
- A description of the vulnerability
- Steps to reproduce
- Potential impact

We aim to acknowledge reports within 72 hours and provide a resolution timeline within 7 days.

Do **not** open a public GitHub issue for security vulnerabilities.

## Threat model

`pdf-engine-presentation` is a deterministic, dependency-free Markdown parser. It:

- Performs no file, network, or process I/O of its own — it only transforms an
  in-memory `&str` into a `Deck`.
- Denies `unsafe_code` at the crate level (`#![deny(unsafe_code)]`).
- Runs in a single pass over the input's lines with no recursion, so parse time
  and memory scale linearly with input size — there is no unbounded recursion
  or exponential-blowup path to guard against.

Because it has no dependencies, its supply-chain surface is limited to the
Rust toolchain itself. Callers embedding untrusted Markdown should still apply
their own input-size limits at the boundary, since this crate does not impose
one.
