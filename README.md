# arrays-with-gaps

A Rust project exploring array-based alternatives to red-black trees with a
performance-first baseline.

## Goal

- Build a baseline red-black tree implementation in Rust
- Measure performance with `criterion`
- Prepare the repo for future array-based alternatives such as gap arrays,
  packed-memory arrays, and cache-friendly implicit structures

## Project structure

- `src/rb_tree.rs` — red-black tree baseline implementation
- `src/lib.rs` — crate entrypoint
- `src/main.rs` — example usage of the baseline tree
- `benches/benchmark.rs` — `criterion` benchmark harness
- `devlog.md` — development notes, benchmark observations, and next steps

## Usage

- `cargo build`
- `cargo test`
- `cargo bench`
- `deno fmt *.md` for formatting documentation.

## Next work

1. Add one or more array-based alternatives
2. Compare search/insert/delete performance against the RB tree baseline
3. Capture results and insights in `devlog.md`
