//! Experiment: Modern HashSet wrapping pattern
//!
//! This demonstrates the modern pattern used by vstd for BTreeSet, BTreeMap,
//! and BinaryHeap, which avoids the `pub m` problem in HashSetWithView.
//!
//! ## Two patterns in vstd:
//!
//! ### Old pattern (HashSetWithView):
//! ```ignore
//! pub struct HashSetWithView<Key> {
//!     pub m: HashSet<Key>,  // Field must be pub for extensions!
//! }
//! impl View for HashSetWithView<Key> { ... }
//! ```
//!
//! ### Modern pattern (BTreeSet, BinaryHeap):
//! ```ignore
//! #[verifier::external_type_specification]
//! #[verifier::external_body]
//! pub struct ExBTreeSet<K, A>(BTreeSet<K, A>);
//!
//! impl View for BTreeSet<K, A> { ... }  // View on std type directly!
//!
//! assume_specification[ BTreeSet::insert ](...);  // Specs for std methods
//! ```
//!
//! ## The key difference:
//! - Old: You use `HashSetWithView<T>` (a wrapper type)
//! - Modern: You use `std::collections::BTreeSet<T>` directly
//!
//! The modern pattern is better because:
//! 1. No wrapper struct needed
//! 2. No private field access issues
//! 3. Standard Rust code works with Verus specs
//! 4. Extensions can be added via additional assume_specification
//!
//! ## Testing in APAS-VERUS vs verus test harness
//!
//! The verus repo uses `test_verify_one_file!` macro which:
//! - Writes code snippets to temp files
//! - Spawns verus binary on each snippet
//! - Can test expected FAILURES via `=> Err(err) => assert_one_fails(err)`
//!
//! We cannot use that harness - it's internal to verus's vargo build.
//!
//! Instead, in APAS-VERUS we write functions that get verified when we run:
//!   `verus --crate-type=lib src/lib.rs`
//!
//! Same verification, different harness. Limitation: we can only test
//! successful verification, not expected failures.

use std::collections::HashSet;
use vstd::prelude::*;

verus! {

// Test: HashSet::new creates empty set
fn test_hashset_new() {
    use vstd::std_specs::hash::*;
    broadcast use group_hash_axioms;

    let set: HashSet<u64> = HashSet::new();
    assert(set@.len() == 0);
}

// Test: HashSet::insert adds elements
fn test_hashset_insert() {
    use vstd::std_specs::hash::*;
    broadcast use group_hash_axioms;

    let mut set: HashSet<u64> = HashSet::new();

    let was_new = set.insert(10);
    assert(was_new);  // true when element is new
    assert(set@.contains(10));
    assert(set.len() == 1);

    let was_new2 = set.insert(20);
    assert(was_new2);
    assert(set@.contains(20));
    assert(set.len() == 2);
}

// Test: HashSet::contains checks membership
fn test_hashset_contains() {
    use vstd::std_specs::hash::*;
    broadcast use group_hash_axioms;

    let mut set: HashSet<u64> = HashSet::new();
    set.insert(10);
    set.insert(20);

    let has_10 = set.contains(&10);
    assert(has_10);

    let has_30 = set.contains(&30);
    assert(!has_30);
}

// Test: HashSet::remove removes elements
fn test_hashset_remove() {
    use vstd::std_specs::hash::*;
    broadcast use group_hash_axioms;

    let mut set: HashSet<u64> = HashSet::new();
    set.insert(10);
    set.insert(20);
    assert(set.len() == 2);

    let was_present = set.remove(&10);
    assert(was_present);
    assert(!set@.contains(10));
    assert(set@.contains(20));
    assert(set.len() == 1);
}

// Compare: HashSetWithView requires wrapper type
fn test_hashset_with_view_comparison() {
    use vstd::hash_set::HashSetWithView;
    use vstd::std_specs::hash::obeys_key_model;

    assume(obeys_key_model::<u64>());

    let mut set: HashSetWithView<u64> = HashSetWithView::new();
    set.insert(10);
    assert(set@.contains(10u64));

    // The problem: extending HashSetWithView (e.g. adding to_vec())
    // requires access to set.m which must be pub.
    // With modern pattern, we'd add assume_specification instead.
}

// Conclusion:
// - For new APAS-VERUS code: use HashSet directly with vstd::std_specs::hash
// - HashSetWithView needed only for Key: View with complex key types
// - The `pub m` hack remains until vstd modernizes HashSetWithView

} // verus!
