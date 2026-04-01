//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Mt Type Bounds Standard: use trait aliases from Types.rs and Concurrency.rs.
//!
//! APAS-VERUS defines trait aliases that bundle the bounds needed for sequential (St)
//! and multi-threaded (Mt) code. Mt modules MUST use these aliases instead of spelling
//! out raw bounds. Raw bounds (`T: Clone + Eq + Send + Sync + 'static`) are verbose,
//! error-prone, and inconsistent across files.
//!
//! ## Element type aliases (src/Types.rs, src/Concurrency.rs)
//!
//! | Alias       | Expands to                                          | Use for                          |
//! |-------------|-----------------------------------------------------|----------------------------------|
//! | `StT`       | `Eq + PartialEq + Clone + Display + Debug + Sized + View` | St element types           |
//! | `StTInMtT`  | `StT + Send + Sync + 'static`                       | St elements used in Mt code      |
//! | `MtT`       | `Sized + Send + Sync`                                | Raw Mt-safe types                |
//! | `MtKey`     | `StTInMtT + Ord + 'static`                           | Keys in Mt ordered collections   |
//! | `MtVal`     | `StTInMtT + 'static`                                 | Values in Mt collections         |
//!
//! ## Closure type aliases (src/Concurrency.rs)
//!
//! | Alias            | Expands to                                        | Use for               |
//! |------------------|---------------------------------------------------|-----------------------|
//! | `MtReduceFn<V>`  | `Fn(&V, &V) -> V + Clone + Send + Sync + 'static` | reduce, fold          |
//! | `Pred<T>`        | `Fn(&T) -> bool + Send + Sync + 'static`           | filter, contains      |
//! | `MtPred<T>`      | `Fn(&T) -> bool + Clone + Send + Sync + 'static`  | filter in D&C (needs Clone) |
//! | `MtMapFn<T, U>`  | `Fn(&T) -> U + Clone + Send + Sync + 'static`     | map                   |
//! | `MtTabulateFn<T>` | `Fn(usize) -> T + Clone + Send + Sync + 'static` | tabulate              |
//!
//! Note: `Pred<T>` lacks `Clone`. For sequential filter (no join), `Pred<T>` suffices.
//! For D&C filter via `join()`, use `MtPred<T>` which adds `Clone` so the predicate
//! can be cloned into both join arms via `clone_pred`.
//!
//! ## Rules
//!
//! 1. **NEVER spell out raw bounds in Mt trait signatures.** Use the aliases.
//!    Bad:  `fn map<U: Clone + Eq + Send + Sync + 'static, F: Fn(&T) -> U + Clone + Send + Sync + 'static>`
//!    Good: `fn map<U: StTInMtT, F: MtMapFn<T, U>>`
//!
//! 2. **St files use `StT`.** They do not need `Send + Sync + 'static`.
//!
//! 3. **Mt files use `StTInMtT` (or `MtKey`/`MtVal`) for element types** and
//!    `MtReduceFn`/`MtPred`/`MtMapFn`/`MtTabulateFn` for closure types.
//!
//! 4. **All aliases have blanket impls.** Any type satisfying the component bounds
//!    automatically implements the alias trait. You never need to write
//!    `impl StTInMtT for MyType` — it's automatic.
//!
//! 5. **Use `clone_fn`, `clone_fn2`, `clone_pred` from `vstdplus::clone_plus`** to
//!    clone closures in D&C join arms. These preserve `requires`/`ensures` specs
//!    through the clone. See `using_closures_standard.rs`.
//!
//! ## Adding new aliases
//!
//! If a new closure pattern appears (e.g., `Fn(&K, &V) -> bool` for table filter),
//! add the alias to `src/Concurrency.rs` with a blanket impl. Do not use raw bounds.
//!
//! ## Verus updates (as of 2026-03-28)
//!
//! Verus commit `3390e9af0` treats **Copy, FnOnce, FnMut, Fn, and Tuple as first-class
//! traits**. This may improve Verus's ability to verify `Clone` on closure types and
//! reduce the need for `external_body` bridges when passing closures to functions
//! requiring `Clone`. After upgrading Verus, re-test whether named closures satisfy
//! `Clone` bounds without bridges.
//!
//! Also: `356a04302` adds `tuple: Clone` support, `cb63005df` fixes `Copy` for tuples.
//! These help with closure captures that include tuples.

// This file is documentation-only. No compilable code.
// The actual trait definitions live in:
//   src/Types.rs        — StT, StPred, valid_key_type
//   src/Concurrency.rs  — StTInMtT, MtT, MtKey, MtVal, MtReduceFn, Pred, MtPred, MtMapFn, MtTabulateFn
