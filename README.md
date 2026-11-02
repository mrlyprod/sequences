mrly-sequences; a MrlyMath crate: the sequence rows, a design sequence's address, its measure and axis, its terms and its status

# SEQUENCES

- Rust only for now; no bindings sit beside it yet.
- Builds on `mrly-core` and `mrly-math`, by version.
- `scripts/check.sh` is the one gate: fmt, clippy, test, doc, leak.
- Until the crates are published, clone the fleet beside this repo: a `.cargo/config.toml` with `[patch.crates-io]` rows points every `mrly-*` dependency at its sibling folder.

```rust
use mrly_sequences::{Axis, Cost, Key, Measure};

let carpet = Key::new(7, 2, 2, Measure::Fills, Axis::Side);
assert_eq!(carpet.number(), 3);
assert_eq!(carpet.measure.cost(carpet.axis), Cost::Closed);
```
