# Rust Coding Questions

A collection of classic data-structure and algorithm problems solved in Rust,
written LeetCode-style with unit tests for each solution.

## Layout

```
rust-coding-questions/
├── Cargo.toml          # package manifest (edition 2021)
└── src/
    ├── lib.rs          # crate root — one `mod <name>;` per question
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

1. Create a new file in `src/`, e.g. `src/two_sum.rs`, with the solution and a
   `#[cfg(test)]` module.
2. Add one line to `src/lib.rs`:

   ```rust
   mod two_sum;
   ```

3. Run `cargo test` — the new tests are picked up automatically.
