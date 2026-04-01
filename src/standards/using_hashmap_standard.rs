//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Using HashMap Standard: verified alternatives to std::collections::HashMap.
//!
//! `std::collections::HashMap` is an unverified external type. Verus cannot reason about
//! its operations (get, insert, contains_key, values, etc.). Functions that use HashMap
//! directly cannot be verified and must remain external_body or cfg-gated.
//!
//! APAS-VERUS provides verified alternatives. Use them instead.
//!
//! Replacement types (in order of preference):
//!
//! 1. `HashMapWithViewPlus<K, V>` (from `vstdplus::hash_map_with_view_plus`)
//!    - Drop-in replacement for HashMap with Verus specs.
//!    - View type: `Map<K::V, V>` — connects to vstd Map reasoning.
//!    - Supports: new, get, insert, remove, contains_key, len, is_empty, clear, iter.
//!    - Implements Clone, PartialEq, Eq.
//!    - Has full iterator support (loop and for patterns).
//!    - Requires: `K: View + Eq + Hash`, `obeys_key_model::<K>()`.
//!    - Best for: vertex-to-vertex mappings, partition maps, label maps.
//!
//! 2. `MappingStEph<A, B>` (from `Chap05::MappingStEph`)
//!    - Set-based mapping (set of pairs).
//!    - View type: `Set<(A::V, B::V)>`.
//!    - Heavier abstraction, better for when you need set-theoretic reasoning.
//!    - Best for: when the algorithm spec is naturally set-of-pairs.
//!
//! 3. `SetStEph<(K, V)>` (from `Chap05::SetStEph`)
//!    - Lightweight: just a set of key-value pairs.
//!    - No direct key lookup (must iterate to find).
//!    - Best for: small collections where O(n) lookup is acceptable.
//!
//! Pattern: Replace HashMap with HashMapWithViewPlus.
//!
//!   BAD — unverifiable:
//!
//!     #[cfg(not(verus_keep_ghost))]
//!     use std::collections::HashMap;
//!
//!     #[verifier::external_body]
//!     fn build_partition(vertices: &SetStEph<V>) -> HashMap<V, V> {
//!         let mut map = HashMap::new();
//!         for v in vertices.iter() { map.insert(v.clone(), v.clone()); }
//!         map
//!     }
//!
//!   GOOD — verifiable:
//!
//!     use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
//!
//!     fn build_partition(vertices: &SetStEph<V>) -> (map: HashMapWithViewPlus<V, V>)
//!         requires vertices.spec_setsteph_wf(), obeys_key_model::<V>(),
//!         ensures map@.dom() =~= vertices@,
//!     {
//!         let mut map = HashMapWithViewPlus::new();
//!         for v in vertices.iter()
//!             invariant obeys_key_model::<V>(), ...
//!         {
//!             map.insert(v.clone(), v.clone());
//!         }
//!         map
//!     }
//!
//! For Mt (multi-threaded) modules:
//!
//!   BAD — Arc<HashMap> for sharing across ParaPair! closures:
//!
//!     use std::sync::Arc;
//!     use std::collections::HashMap;
//!     let shared = Arc::new(partition_map);
//!     ParaPair!(move || use_map(&shared.clone()), move || use_map(&shared.clone()))
//!
//!   BAD — cloning the map into each closure arm. Cloning an O(n) map at every fork
//!   defeats the purpose of parallelism:
//!
//!     let map_left = partition_map.clone();   // O(n) copy!
//!     let map_right = partition_map.clone();  // O(n) copy!
//!     join(move || use_map(&map_left), move || use_map(&map_right));
//!
//!   GOOD — top-level RwLock (standard Mt pattern from
//!   toplevel_coarse_rwlocks_for_mt_modules.rs). The map lives inside the module's
//!   locked inner struct. Fork-join closures acquire a read guard to access the map.
//!   No Arc, no clone, O(1) sharing:
//!
//!     // Inside the Mt module struct:
//!     pub struct FooMtEphInner<V> {
//!         partition: HashMapWithViewPlus<V, V>,
//!         ...
//!     }
//!     // Closures take &self (read guard) and access self.inner.partition.
//!
//!   See `toplevel_coarse_rwlocks_for_mt_modules.rs` for the full pattern.
//!
//! What NOT to do:
//!   - Do NOT use `std::collections::HashMap` in function bodies that should be verified.
//!   - Do NOT cfg-gate a function just because it uses HashMap.
//!   - Do NOT wrap HashMap in Arc for fork-join sharing — use top-level RwLock instead.
//!   - Do NOT clone maps into closure arms — O(n) per fork is unacceptable.
//!   - Do NOT use HashMap::values(), HashMap::keys(), or HashMap::iter() — these return
//!     unverified iterators. Use HashMapWithViewPlus::iter() which has Verus specs.
//!
//! ## Mt modules in Chap43+ should prefer OrderedTableMtEph
//!
//! `HashMapWithViewPlus` is unordered and sequential. `OrderedTableMtEph` (Chap43)
//! is BST-backed (via BSTParaMtEph from Chap38) and inherits parallel operations:
//! parallel build via tabulate, parallel union/intersect/difference via ParaPair!.
//!
//! For Mt modules in chapters AFTER Chap43, prefer `OrderedTableMtEph` over
//! `HashMapWithViewPlus` when:
//! - The map is used in a parallel context (fork-join, D&C)
//! - Parallel build or parallel merge would improve span
//! - The key type supports `Ord` (required for BST ordering)
//!
//! Trade-off: O(1) hash lookup → O(lg n) tree lookup. But parallel build
//! (O(lg² n) span) vs sequential HashMap build (O(n) span) often outweighs
//! the per-lookup cost.
//!
//! Keep `HashMapWithViewPlus` when:
//! - The map is built once and only read (no parallel build benefit)
//! - O(1) lookup is critical to the work bound
//! - The chapter is before Chap43 (can't use a later chapter's data structure)
//!
//! ## All Rust primitives implement View (and therefore StT)
//!
//! `usize`, `bool`, `u8`..`u128`, `i8`..`i128`, `isize`, `char` all have identity
//! `View` impls in vstd (`vstd/view.rs:264-292`): `View::V = Self`, `view(&self) = *self`.
//! They also satisfy `Eq + Clone + Display + Debug + Sized`, so they are `StT`.
//!
//! This means `OrderedTableMtEph<V, usize>` and `OrderedTableMtEph<V, bool>` are
//! valid — do NOT assume primitives are excluded from verified collection types.
//! Tuples of View types also implement View (`vstd/view.rs:297`), so
//! `OrderedTableMtEph<(usize, usize), T>` works if `(usize, usize): Ord`.
//!
//! See: `src/vstdplus/hash_map_with_view_plus.rs` for the implementation.
//! See: `src/Chap43/OrderedTableMtEph.rs` for the parallel ordered table.
//! See: `src/standards/arc_usage_standard.rs` for when Arc is actually needed.

pub mod using_hashmap_standard {}
