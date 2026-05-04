# Devlog

## 2026-05-04

- Added a baseline red-black tree implementation in `src/rb_tree.rs`.
- Added `criterion` as a benchmark dependency in `Cargo.toml`.
- Created `benches/benchmark.rs` for insertion, lookup, and removal benchmarks.
- Updated `src/main.rs` with a small usage example.
- Added `README.md` and this `devlog.md`.

## Baseline status

- [x] RB tree implementation
- [x] Criterion benchmark harness
- [x] Example crate usage
- [x] Documentation and project structure

## Next steps

- Add array-based alternatives for comparison.
- Target gap array / packed-memory array implementations next.
- Use the benchmark harness to measure and compare against the RB baseline.
