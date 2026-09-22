// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Using HashMap Standard: `std::collections::HashMap` and `HashSet` under
//! vstd 0.2026.09.13.
//!
//! vstd `std_specs/hash.rs` specifies both types: `new`, `with_capacity`,
//! `len`, `is_empty`, `get`, `insert`, `remove`, `contains_key`/`contains`,
//! `clear`, `iter`, `keys`, `values`, and `HashMap::clone`, with the views
//! `Map<K, V>` and `Set<K>`. `src/vstdplus/hash_specs_plus.rs` adds the three
//! it lacks: `HashSet::clone`, and `PartialEq::eq` for both. APAS's wrappers
//! `HashMapWithViewPlus` and `HashSetWithViewPlus` were deleted in r209
//! (`docs/HashMigration.md`); use the std types directly, as below.
//!
//! The recipe (`docs/HashSpecsMigration.md` §3.1), each step marked in the
//! example:
//!
//! 1. Field type: `HashMap<K, V>` or `HashSet<K>`, never a wrapper.
//! 2. View: the raw view, `Map<K, V>` or `Set<K>`. A module whose view must
//!    be `Map<K::V, V::V>` uses `self.table.deep_view()` with
//!    `vstd::std_specs::hash::lemma_hashmap_deepview_properties` under
//!    `injective(|k: K| k.deep_view())`; a set view `Set<K::V>` is
//!    `self.elements@.map(|k: K| k@)` (see `src/Chap05/SetStEph.rs`).
//! 3. Preconditions: `obeys_key_model::<K>()`, folded into the module's wf
//!    and required by constructors. `group_hash_axioms` proves it for every
//!    primitive type and `Box` of one, and proves
//!    `builds_valid_hashers::<RandomState>()`. A user-defined key type states
//!    it once at the top level. The raw view needs no `obeys_feq_*`; view
//!    injectivity belongs only to a mapped view (step 2).
//! 4. Iteration: delegated. `iter()` returns `hash_map::Iter` (or
//!    `hash_set::Iter`) and restates vstd's postconditions; loops reason
//!    through `it.seq()` and `it.index()` as in
//!    `src/standards/iterators_standard.rs`.
//! 5. Broadcast: `vstd::std_specs::hash::group_hash_axioms`.
//!
//! For Mt (multi-threaded) modules:
//!
//!   BAD — `Arc<HashMap>` for sharing across `ParaPair!` closures, or cloning
//!   the map into each closure arm: an O(n) copy at every fork defeats the
//!   parallelism.
//!
//!   GOOD — top-level `RwLock` (`toplevel_coarse_rwlocks_for_mt_modules.rs`).
//!   The map lives inside the module's locked inner struct; fork-join closures
//!   acquire a read guard. No `Arc`, no clone, O(1) sharing.
//!
//! Mt modules in Chap43 and later may prefer `OrderedTableMtEph` (Chap43,
//! BST-backed through `BSTParaMtEph`) when the table is built or merged in
//! parallel: O(lg n) lookup instead of O(1), but O(lg² n) build span instead
//! of O(n). Keep `HashMap` when the table is built once and read many times,
//! when O(1) lookup is part of the work bound, or before Chap43.
//!
//! What NOT to do:
//!   - Do NOT cfg-gate a function because it uses `HashMap`; it is specified.
//!   - Do NOT wrap `HashMap` in `Arc` for fork-join sharing; use a top-level
//!     `RwLock`.
//!   - Do NOT clone maps into closure arms.
//!   - Do NOT write `obeys_feq_view_injective::<K>()` as a precondition for
//!     the raw view; it was the wrapper's requirement, not vstd's.
//!
//! All Rust primitives implement `View` (identity, `vstd/view.rs`) and are
//! `StT`, so `HashMap<V, usize>` and `HashMap<V, bool>` are ordinary.
//!
//! References:
//! - `~/projects/verus/source/vstd/std_specs/hash.rs` (the specifications).
//! - `src/vstdplus/hash_specs_plus.rs` (clone and eq).
//! - `src/experiments/vstd_hash_map_derived.rs`, `vstd_hash_set_derived.rs`
//!   (one fn per method, each postcondition derived).
//! - `src/standards/arc_usage_standard.rs` (when `Arc` is needed).

pub mod using_hashmap_standard {

    use std::collections::hash_map;
    use std::collections::HashMap;
    use std::hash::Hash;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;

    verus! {

    // 3. broadcast use — step 5.
    broadcast use group_hash_axioms;

    // 4. type definitions — step 1: the field is the std type.

    /// A table of counts keyed by `K`.
    #[verifier::reject_recursive_types(K)]
    pub struct CountTable<K: Eq + Hash> {
        pub counts: HashMap<K, u64>,
    }

    // 5. view impls — step 2: the raw view.

    impl<K: Eq + Hash> View for CountTable<K> {
        type V = Map<K, u64>;

        open spec fn view(&self) -> Map<K, u64> {
            self.counts@
        }
    }

    // 8. traits

    pub trait CountTableTrait<K: Eq + Hash>: Sized + View<V = Map<K, u64>> {
        /// Step 3: the key model is the module's well-formedness.
        spec fn spec_counttable_wf(&self) -> bool;

        fn new() -> (t: Self)
            requires
                obeys_key_model::<K>(),
            ensures
                t.spec_counttable_wf(),
                t@ == Map::<K, u64>::empty(),
        ;

        fn count(&self, k: &K) -> (n: u64)
            requires
                self.spec_counttable_wf(),
            ensures
                n == if self@.contains_key(*k) { self@[*k] } else { 0 },
        ;

        fn bump(&mut self, k: K)
            requires
                old(self).spec_counttable_wf(),
                old(self)@.contains_key(k) ==> old(self)@[k] < u64::MAX,
            ensures
                self.spec_counttable_wf(),
                self@ == old(self)@.insert(
                    k,
                    if old(self)@.contains_key(k) { (old(self)@[k] + 1) as u64 } else { 1 },
                ),
        ;

        /// Step 4: delegated iteration, vstd's `HashMap::iter` postconditions
        /// restated over the module's view.
        fn iter(&self) -> (it: hash_map::Iter<'_, K, u64>)
            requires
                self.spec_counttable_wf(),
            ensures
                IteratorSpec::remaining(&it).len() == self@.len(),
                IteratorSpec::remaining(&it).unref().to_set() == self@.kv_pairs(),
                IteratorSpec::remaining(&it).no_duplicates(),
                into_iter(it) == IteratorSpec::remaining(&it).unref(),
                IteratorSpec::decrease(&it) is Some,
        ;

        fn size(&self) -> (n: usize)
            requires
                self.spec_counttable_wf(),
            ensures
                n == self@.len(),
        ;
    }

    // 9. impls

    impl<K: Eq + Hash> CountTableTrait<K> for CountTable<K> {
        open spec fn spec_counttable_wf(&self) -> bool {
            obeys_key_model::<K>()
        }

        fn new() -> (t: Self) {
            CountTable { counts: HashMap::new() }
        }

        fn count(&self, k: &K) -> (n: u64) {
            match self.counts.get(k) {
                Some(n) => *n,
                None => 0,
            }
        }

        fn bump(&mut self, k: K) {
            let n = self.count(&k);
            let _previous = self.counts.insert(k, n + 1);
        }

        fn iter(&self) -> (it: hash_map::Iter<'_, K, u64>) {
            self.counts.iter()
        }

        // A for-loop over the delegated iterator: the invariant names `it.seq()`
        // and `it.index()`; termination comes from `decrease is Some`. The
        // exec `len()` bounds the count by `usize`.
        fn size(&self) -> (n: usize) {
            let len = self.counts.len();
            let mut n: usize = 0;
            for kv in it: self.counts.iter()
                invariant
                    obeys_key_model::<K>(),
                    it.seq().len() == self@.len(),
                    len == self@.len(),
                    n == it.index(),
            {
                n = n + 1;
            }
            n
        }
    }

    } // verus!

    // 14. derive impls outside verus!

    impl<K: Eq + Hash + std::fmt::Debug> std::fmt::Debug for CountTable<K> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "CountTable({:?})", self.counts)
        }
    }
    impl<K: Eq + Hash> std::fmt::Display for CountTable<K> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "CountTable(len={})", self.counts.len())
        }
    }
}
