//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Constructor feq_clone Standard for APAS-VERUS modules.
//!
//! When a module's wf predicate includes `obeys_feq_clone::<T>()` (because internal
//! operations like `in_order()` or `clone()` need it), constructors like `empty()` and
//! `singleton()` must `requires obeys_feq_clone::<T>()` — not `assume` it.
//!
//! ## Why
//!
//! `obeys_feq_clone::<T>()` is unprovable for generic `T` in Verus. Somewhere in the
//! call chain, an `assume` must exist. The question is where.
//!
//! **Wrong**: assume in the constructor body. This hides a proof hole inside the module
//! and inflates the hole count. Every constructor that ensures wf would need its own assume.
//!
//! **Right**: require it in the constructor signature. The obligation flows to callers:
//! - **Algorithmic callers** already have `obeys_feq_clone::<T>()` in scope from their
//!   own wf (e.g., `union` calls `Self::empty()` with `self.spec_wf()` required, and
//!   wf includes `obeys_feq_clone`).
//! - **Test callers** (RTT) run outside `verus!`, so requires are not checked at runtime.
//! - **Top-level entry points** (benchmarks, main) can assume once at the outermost scope.
//!
//! This eliminates assumes from the module entirely — zero proof holes from feq_clone.
//!
//! ## Pattern
//!
//! ### Trait declaration
//!
//! ```rust
//! pub trait FooTrait<T: StTInMtT + Ord + TotalOrder>: Sized {
//!     spec fn spec_foo_wf(&self) -> bool;
//!
//!     fn empty() -> (set: Self)
//!         requires obeys_feq_clone::<T>()   // Caller provides feq_clone.
//!         ensures set.spec_foo_wf();
//!
//!     fn singleton(value: T) -> (set: Self)
//!         requires obeys_feq_clone::<T>()   // Caller provides feq_clone.
//!         ensures set.spec_foo_wf();
//!
//!     fn insert(&mut self, value: T) -> (r: Result<(), ()>)
//!         requires old(self).spec_foo_wf()  // wf includes feq_clone — no extra requires.
//!         ensures self.spec_foo_wf();
//! }
//! ```
//!
//! ### Wf includes feq_clone
//!
//! ```rust
//! open spec fn spec_foo_wf(&self) -> bool {
//!     self.inner.spec_inner_wf() && obeys_feq_clone::<T>()
//! }
//! ```
//!
//! ### Constructor body — no assume
//!
//! ```rust
//! fn empty() -> Self {
//!     // obeys_feq_clone::<T>() comes from requires — no assume needed.
//!     Self { inner: Inner::new() }
//! }
//! ```
//!
//! ### Internal callers — feq_clone flows from wf
//!
//! ```rust
//! fn union(&self, other: &Self) -> Self {
//!     // self.spec_foo_wf() is required, which includes obeys_feq_clone::<T>().
//!     // So Self::empty() requires is satisfied.
//!     if self.is_empty() { return Self::empty(); }
//!     // ...
//! }
//! ```
//!
//! ## When to apply
//!
//! Apply this pattern when:
//! 1. The module's wf includes `obeys_feq_clone::<T>()`.
//! 2. Constructors (`empty`, `singleton`, `new`, `from_*`) ensure wf.
//! 3. The constructor body would otherwise need `assume(obeys_feq_clone::<T>())`.
//!
//! ## Relationship to other standards
//!
//! - `partial_eq_eq_clone_standard.rs` (Standard 7): Defines the eq/clone workaround
//!   patterns. Pattern 5 (feq broadcast) propagates feq_clone through requires. This
//!   standard extends that: constructors that bootstrap wf require feq_clone from callers
//!   rather than assuming it.
//! - Clone body assumes (`assume(obeys_feq_clone)` inside `fn clone()`) are a different
//!   pattern and remain correct per Standard 7.

pub mod constructor_feq_standard {}
