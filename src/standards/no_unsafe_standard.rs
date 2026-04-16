// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! No Unsafe Standard: APAS-VERUS does not use unsafe Rust.
//!
//! **NO UNSAFES.** Do not write `unsafe impl`, `unsafe fn`, or `unsafe { }` blocks
//! in APAS-VERUS source code. This applies to all files in `src/` — chapters,
//! vstdplus, experiments, everything.
//!
//! ## Why
//!
//! The point of APAS-VERUS is formal verification. `unsafe` bypasses Rust's type
//! system guarantees — the very guarantees Verus builds its proofs on. An `unsafe impl
//! Send for Foo` tells the compiler "trust me, Foo is thread-safe" without proof.
//! That's antithetical to the project.
//!
//! ## What to do instead
//!
//! ### Instead of `unsafe impl Send/Sync`:
//!
//! Use `Arc<RwLock<T, Inv>>` for thread-shared state. Arc and RwLock are Send+Sync
//! by construction when T satisfies the right bounds. If your type wraps Arc<RwLock>,
//! it inherits Send+Sync automatically — no unsafe needed.
//!
//! If a type stores raw pointers or non-Send fields that prevent auto-deriving
//! Send+Sync, restructure to use Arc<RwLock<...>> or PCell with PointsTo tokens.
//! See `hfscheduler_standard.rs` and `arc_usage_standard.rs`.
//!
//! ### Instead of `unsafe { }` blocks:
//!
//! Use Verus's verified alternatives: PCell instead of UnsafeCell, tracked permissions
//! instead of raw pointers, vstd atomics instead of std atomics.
//!
//! ### If you think unsafe is truly necessary:
//!
//! Stop. Ask the user. Explain why no safe alternative works. The answer is almost
//! always that a safe restructuring exists. In the rare case where unsafe is genuinely
//! needed (e.g., FFI boundaries), it must be approved by the user before writing.
//!
//! ## Current violations (to be fixed)
//!
//! The following files have `unsafe impl Send/Sync` that need to be restructured:
//! - `src/Chap38/BSTParaMtEph.rs` — ParamBST (root cause; others inherit)
//! - `src/Chap39/BSTParaTreapMtEph.rs` — ParamTreap
//! - `src/Chap41/AVLTreeSetMtEph.rs` — wraps ParamBST
//! - `src/Chap41/AVLTreeSetMtPer.rs` — wraps ParamBST
//! - `src/Chap43/OrderedTableMtEph.rs` — wraps ParamBST
//! - `src/vstdplus/threads_plus.rs` — ThreadShareablePlus
//!
//! These exist because ParamBST uses internal node-level locking (per-node RwLock
//! with raw pointer traversal) that doesn't auto-derive Send+Sync. The fix is to
//! restructure ParamBST to use Arc-based node links, or to wrap it in Arc<RwLock>
//! at the module boundary.

pub mod no_unsafe_standard {}
