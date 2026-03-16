//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! # Standard 19: Helper Function Placement
//!
//! When to put helpers in the trait vs. as module-level free functions.
//!
//! ## The Problem
//!
//! Every APAS module uses the trait-impl pattern: a public trait defines the API,
//! an impl provides the bodies. But algorithms often need internal helpers —
//! recursive cores, subrange processors, rotation routines. Where do these go?
//!
//! Rust has no `pub(crate)` on trait methods. Every method on a `pub trait` is
//! public. Verus adds no visibility controls. So we use placement to signal
//! intent: trait methods are the public API; free functions are internal.
//!
//! ## Rule: Needs `&self` or `&mut self`? → Trait Method. Otherwise → Free Function.
//!
//! **In the trait** (public API + methods that need `self`):
//! - All APAS-specified operations (new, insert, delete, find, size, etc.)
//! - Helpers that need `&self` or `&mut self` — e.g., accessing locked state,
//!   memoization tables, or the type's own fields through its interface.
//! - Subrange processors that produce `Self` and need the type's wf invariant
//!   in their ensures.
//!
//! **Module-level free functions** (internal helpers, not in any trait or impl):
//! - Recursive algorithm cores that operate on bare data types: `Link<T>`,
//!   `Option<Box<Node<T>>>`, `&[T]`, `ArraySeqStEphS<T>`, `Vec<T>`.
//! - Rotation, rebalancing, and structural transformation routines.
//! - Pure proof lemmas (section 7) that reason about sequences, maps, or sets.
//! - Any helper that takes all its inputs as parameters (no `self`).
//!
//! Free functions use `pub(crate)` — visible within the crate (so RTT and PTT
//! can call them) but not exported to external consumers. This is the only
//! visibility mechanism available for signaling "internal, not part of the API."
//! Do not use bare `fn` (module-private) — that hides helpers from tests.
//!
//! ## Naming Conventions
//!
//! | Suffix | Meaning | Example |
//! |--------|---------|---------|
//! | `_inner` | Algorithmic core that a trait method delegates to | `union_inner`, `split_inner` |
//! | `_rec` | Recursive helper (often with memoization) | `subset_sum_rec`, `height_rec` |
//! | `_link` | Operates on bare `Link<T>` (tree pointer) | `insert_link`, `find_link` |
//! | (none) | Descriptive name when the suffix adds nothing | `splay`, `bst_insert`, `merge_dc` |
//!
//! No suffix is mandatory. Use whichever reads best. Do not use `_helper` or
//! `_impl` — describe what the function does, not that it helps.
//!
//! ## Layer 1 / Layer 2 Pattern (Mt Modules)
//!
//! Mt modules that wrap state behind `RwLock` naturally split into two layers:
//!
//! - **Layer 1** (section 9): `pub(crate)` free functions implementing the
//!   verified algorithm on bare data types. All proof work lives here. These
//!   have real `requires` and `ensures` — never `requires true`.
//!
//! - **Layer 2** (section 11): Trait methods that acquire the lock, call a
//!   Layer 1 function, and release the lock. Thin wrappers with specs derived
//!   from Layer 1.
//!
//! Reference: `src/Chap37/BSTSplayMtEph.rs` (17 Layer 1 free functions,
//! trait methods delegate through RwLock).
//!
//! ## Fork-Join Helpers
//!
//! When a trait method uses `join(f1, f2)` and each arm needs to do non-trivial
//! work, write a helper for the arm's work. The join closure calls the helper.
//!
//! **If the helper needs `&self`** (e.g., processing self's entries):
//! Put it in the trait. Each join arm captures the relevant state and calls
//! `Self::process_range(...)`.
//!
//! ```rust
//! // In the trait:
//! fn tabulate_range(keys: &ArraySeqMtEphS<K>, f: &F, start: usize, len: usize)
//!     -> (result: Self)
//!     requires
//!         start + len <= keys@.len(),
//!         forall|k: &K| f.requires((k,)),
//!     ensures
//!         result.spec_tablemteph_wf(),
//!         // ... result contract ...
//! ```
//!
//! **If the helper operates on bare data** (Link, raw arrays, sequences):
//! Make it a free function. Each join arm captures the data and calls the
//! free function.
//!
//! ```rust
//! // Module-level free function:
//! pub(crate) fn merge_dc<T: StT + TotalOrder>(
//!     left: &ArraySeqMtPerS<T>,
//!     right: &ArraySeqMtPerS<T>,
//! ) -> (result: ArraySeqMtPerS<T>)
//!     requires /* ... */
//!     ensures /* ... */
//! ```
//!
//! The join arm then calls the helper directly — no nested closure:
//!
//! ```rust
//! let f1 = move || -> (r: Self)
//!     requires /* propagate f_arc specs */
//!     ensures /* result contract */
//! {
//!     Self::tabulate_range(&left_keys, &*f_arc1, 0, mid)
//! };
//! ```
//!
//! ## Anti-Patterns
//!
//! - **Nested closures in join arms.** Never create a closure inside a join
//!   closure. Verus cannot reason about the inner closure's specs depending on
//!   captured outer closure specs. Write a helper and call it.
//!
//! - **`requires true` on helpers.** Internal helpers must have real specs.
//!   If it operates on a Link, it likely requires `spec_is_bst_link` or
//!   `spec_ordered_link`. If it operates on `&self`, it likely requires
//!   `self.spec_<module>_wf()`. A helper with `requires true` has no contract
//!   and provides nothing to the caller's proof.
//!
//! - **Top-level when `&self` is natural.** If the function's first instinct
//!   is to take `&self` and access the type's fields, don't force it into a
//!   free function by destructuring self into its fields.
//!
//! - **In the trait when it doesn't need `self`.** If the function takes a
//!   `Link<T>` and returns a `Link<T>`, it belongs as a `pub(crate)` free
//!   function. Putting pure tree operations in the trait pollutes the public API.
//!
//! ## Examples from the Codebase
//!
//! | # | Chap | File | Helpers | Placement | Why |
//! |---|------|------|---------|-----------|-----|
//! | 1 | 37 | BSTSplayMtEph.rs | `splay`, `insert_link`, `find_link` | Free fn | Operate on `Link<T>`, no `self` |
//! | 2 | 38 | BSTParaMtEph.rs | `split_inner`, `union_inner` | Free fn | Operate on `Link<K,V>` |
//! | 3 | 26 | MergeSortMtPer.rs | `merge_dc` | Free fn | Operates on `ArraySeqMtPerS` |
//! | 4 | 49 | SubsetSumMtEph.rs | `subset_sum_rec` | Free fn | Takes `Arc<RwLock>`, not `&self` |
//! | 5 | 50 | MatrixChainMtEph.rs | `matrix_chain_rec` | Trait | Needs `&self` for memo table |
//! | 6 | 42 | TableStEph.rs | (none — inline) | In impl | Simple loops, no delegation needed |

use crate::prelude_verus::*;

verus! {

// This file is a standard — it contains no executable code.
// The documentation above defines the placement conventions.

} // verus!
