---
# project-os-730i
title: Reduce WASM binary size
status: completed
type: task
priority: normal
created_at: 2026-04-10T19:54:13Z
updated_at: 2026-04-10T20:19:37Z
---

Research and iterate on compiler options to minimize the WASM binary size. Find the relevant documentation/page on WASM size optimization for Rust/trunk projects, then try different compiler options (opt-level, lto, codegen-units, wasm-opt, etc.) until finding the best configuration optimizing for file size alone.

## Implementation

Baseline size: ~3.4MB (pre-existing dist artifact built without release optimizations).

### Cargo.toml `[profile.release]`
- `opt-level = "z"` — optimize for minimum binary size
- `lto = true` — link-time optimization eliminates dead code across all crates
- `codegen-units = 1` — single codegen unit enables better whole-program optimization
- `panic = "abort"` — removes unwinding machinery, significantly reduces binary size

### index.html
- Added `<link data-trunk rel="rust" data-wasm-opt="z"/>` — enables Trunk's built-in wasm-opt pass (`-Oz`) which runs Binaryen's optimizer on the final WASM binary for additional size reduction beyond what rustc alone achieves.
