# CLAUDE.md

Guidance for working in this repository.

## What this is

A collection of coding problems solved in Rust, aimed at SREs, platform, and
production engineers. The focus is on data structures and patterns that show up
in real infrastructure work: caches, rate limiters, queues, schedulers, and
retry/backoff logic. Every solution ships with unit tests that pin down the
edge cases that matter in production.

## Project layout

- `Cargo.toml` — package manifest (edition 2021).
- `src/lib.rs` — crate root. One `mod <name>;` line per question, nothing else.
- `src/<question>.rs` — one file per question: the solution plus a
  `#[cfg(test)]` module exercising it.

## Conventions

- Solutions are written LeetCode-style with private types. `lib.rs` sets
  `#![allow(dead_code)]` at the crate root so unused public surface does not
  warn — the in-module tests are what exercise the code.
- Each solution lives in its own file and is registered with a single
  `mod <name>;` line in `src/lib.rs`.
- Comments are dense and explanatory by design: this is a learning repo, so code
  reads top-to-bottom with the reasoning inline. Match that style.
- Tests go in a `#[cfg(test)] mod tests` block at the bottom of the same file,
  with `use super::*;` and assertions that document the expected behavior.

## Adding a new question

1. Create `src/<name>.rs` with the solution and a `#[cfg(test)]` module.
2. Add `mod <name>;` to `src/lib.rs`.
3. Run `cargo test` — the new tests are picked up automatically.

## Validating

```bash
cargo test    # run all tests
cargo build   # type-check / compile
```
