# Rust Coding Questions

A collection of coding problems solved in Rust, aimed at **SREs, platform, and
production engineers**, not just classic LeetCode practice.

The focus is on the data structures and patterns that show up in real
infrastructure work: caches, rate limiters, queues, schedulers, retry/backoff
logic, and the kind of systems-flavored problems you hit when building and
operating production services. Each solution is written with unit tests so the
behavior, and the edge cases that matter in production, is pinned down.

## Layout

```
rust-coding-questions/
├── Cargo.toml          # package manifest (edition 2021)
└── src/
    ├── lib.rs          # crate root, one `mod <name>;` per question
    └── lru_cache.rs    # LRU cache implementation + tests
```

## Solutions

| Problem   | File                | Notes                                                        |
| --------- | ------------------- | ------------------------------------------------------------ |
| LRU Cache | `src/lru_cache.rs`  | `O(1)` get/put using a `HashMap` + `BTreeMap` clock ordering |

## Running the tests

```bash
cargo test
```

## Adding a new question

1. Create a new file in `src/`, e.g. `src/rate_limiter.rs`, with the solution and a
   `#[cfg(test)]` module.
2. Add one line to `src/lib.rs`:

   ```rust
   mod rate_limiter;
   ```

3. Run `cargo test`, the new tests are picked up automatically.
