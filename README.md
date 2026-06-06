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
├── Cargo.toml            # package manifest (edition 2021)
└── src/
    ├── lib.rs            # crate root, one `mod <name>;` per question
    ├── lru_cache.rs                  # LRU cache implementation + tests
    ├── rate_limiter.rs               # sliding-window rate limiter + tests
    ├── parse_log_file.rs             # log token frequency counter + tests
    ├── streaming_file_reader.rs      # tail-style streaming file reader + tests
    ├── rolling_window_aggregator.rs  # rolling-window avg/min/max aggregator + tests
    └── in_mem_kv_store.rs            # in-memory key-value store with optional TTL + tests
```

## Solutions

| Problem      | File                  | Notes                                                            |
| ------------ | --------------------- | ---------------------------------------------------------------- |
| LRU Cache    | `src/lru_cache.rs`     | `O(1)` get/put using a `HashMap` + `BTreeMap` clock ordering     |
| Rate Limiter | `src/rate_limiter.rs`  | Sliding-window limiter using a `VecDeque` of request timestamps  |
| Parse Log File | `src/parse_log_file.rs` | Token frequency counter with `top_n`, ties broken alphabetically |
| Streaming File Reader | `src/streaming_file_reader.rs` | `tail -f`-style reader using `BufReader` + `Seek` to track new lines |
| Rolling Window Aggregator | `src/rolling_window_aggregator.rs` | Sliding-window avg/min/max over timestamped `f64` values using a `VecDeque` |
| In-Memory KV Store | `src/in_mem_kv_store.rs` | `HashMap`-backed store with optional per-key TTL, expiry checked on read, and bulk purge |

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
