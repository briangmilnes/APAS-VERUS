//! Experiment: Returning &mut T for hashtable-shaped access patterns.
//!
//! Hypothesis: With -V new-mut-ref, we can verify get_mut/entry patterns
//! that return &mut V from a key-value store, enabling in-place mutation
//! instead of remove-modify-reinsert.
//!
//! Requires: -V new-mut-ref
//!
//! Result: ALL VERIFY. 7 verified, 0 errors.
//! - get_val_mut_by_index: returns &mut u64 to a value slot. VERIFIES.
//! - Sequential &mut borrows (borrow, mutate, drop, borrow again). VERIFIES.
//! - Read-modify-write (*r = *r + 1) through returned &mut. VERIFIES.
//! - get_entry_mut: returns &mut Entry (whole struct). VERIFIES.
//! - Caller mutates fields of returned &mut Entry. VERIFIES.
//! - Frame conditions (other slots unchanged) in ensures. VERIFIES.

use vstd::prelude::*;

verus! {

// Simple Vec-backed key-value store (no hashing — tests the &mut return pattern).
pub struct SimpleMap {
    pub keys: Vec<u64>,
    pub vals: Vec<u64>,
}

impl View for SimpleMap {
    type V = Seq<(u64, u64)>;

    open spec fn view(&self) -> Seq<(u64, u64)> {
        Seq::new(self.keys@.len(), |i: int| (self.keys@[i], self.vals@[i]))
    }
}

// Spec helpers.

pub open spec fn spec_len(m: &SimpleMap) -> nat {
    m.keys@.len()
}

pub open spec fn spec_key_at(m: &SimpleMap, i: int) -> u64 {
    m.keys@[i]
}

pub open spec fn spec_val_at(m: &SimpleMap, i: int) -> u64 {
    m.vals@[i]
}

pub open spec fn spec_find_key(m: &SimpleMap, key: u64) -> Option<int> {
    if exists|i: int| 0 <= i < m.keys@.len() && m.keys@[i] == key {
        Some(choose|i: int| 0 <= i < m.keys@.len() && m.keys@[i] == key)
    } else {
        None
    }
}

// 1. get_val_mut_by_index: return &mut to a value by index.
//
// Hypothesis: Caller can mutate through returned &mut and Verus tracks it.
pub fn get_val_mut_by_index(m: &mut SimpleMap, idx: usize) -> (r: &mut u64)
    requires
        idx < old(m).keys@.len(),
        old(m).keys@.len() == old(m).vals@.len(),
    ensures
        *r == old(m).vals@[idx as int],
        *final(r) == final(m).vals@[idx as int],
        final(m).keys@ == old(m).keys@,
        final(m).vals@.len() == old(m).vals@.len(),
        forall|j: int| 0 <= j < old(m).vals@.len() && j != idx ==>
            final(m).vals@[j] == old(m).vals@[j],
{
    &mut m.vals[idx]
}

// 2. Test: caller mutates through returned &mut.
pub fn test_mutate_by_index() {
    let mut m = SimpleMap {
        keys: Vec::new(),
        vals: Vec::new(),
    };
    m.keys.push(10);
    m.vals.push(100);
    m.keys.push(20);
    m.vals.push(200);

    assert(m.keys@.len() == 2);
    assert(m.vals@.len() == 2);
    assert(m.vals@[0] == 100);

    let r = get_val_mut_by_index(&mut m, 0);
    *r = 999;

    assert(m.vals@[0] == 999);
    assert(m.vals@[1] == 200);
    assert(m.keys@[0] == 10);
}

// 3. get_key_mut: return &mut to a key (for completeness).
pub fn get_key_mut(m: &mut SimpleMap, idx: usize) -> (r: &mut u64)
    requires
        idx < old(m).keys@.len(),
        old(m).keys@.len() == old(m).vals@.len(),
    ensures
        *r == old(m).keys@[idx as int],
        *final(r) == final(m).keys@[idx as int],
        final(m).vals@ == old(m).vals@,
        final(m).keys@.len() == old(m).keys@.len(),
{
    &mut m.keys[idx]
}

// 4. Pair of mutable accesses in sequence.
// After first borrow ends, second borrow starts.
pub fn test_sequential_mut_access() {
    let mut m = SimpleMap {
        keys: Vec::new(),
        vals: Vec::new(),
    };
    m.keys.push(1);
    m.vals.push(10);
    m.keys.push(2);
    m.vals.push(20);

    // First mutable borrow.
    let r1 = get_val_mut_by_index(&mut m, 0);
    *r1 = 11;
    // r1 dropped here.

    // Second mutable borrow.
    let r2 = get_val_mut_by_index(&mut m, 1);
    *r2 = 22;
    // r2 dropped here.

    assert(m.vals@[0] == 11);
    assert(m.vals@[1] == 22);
}

// 5. Read-modify-write through &mut.
pub fn test_increment_in_place() {
    let mut m = SimpleMap {
        keys: Vec::new(),
        vals: Vec::new(),
    };
    m.keys.push(42);
    m.vals.push(0);

    let r = get_val_mut_by_index(&mut m, 0);
    assert(*r == 0);
    *r = *r + 1;
    assert(m.vals@[0] == 1);

    let r2 = get_val_mut_by_index(&mut m, 0);
    assert(*r2 == 1);
    *r2 = *r2 + 1;
    assert(m.vals@[0] == 2);
}

// 6. Nested struct: return &mut to a field inside a boxed value.
pub struct Entry {
    pub key: u64,
    pub val: u64,
    pub count: u64,
}

pub struct EntryStore {
    pub entries: Vec<Entry>,
}

pub fn get_entry_mut(store: &mut EntryStore, idx: usize) -> (r: &mut Entry)
    requires
        idx < old(store).entries@.len(),
    ensures
        *r == old(store).entries@[idx as int],
        *final(r) == final(store).entries@[idx as int],
        final(store).entries@.len() == old(store).entries@.len(),
        forall|j: int| 0 <= j < old(store).entries@.len() && j != idx ==>
            final(store).entries@[j] == old(store).entries@[j],
{
    &mut store.entries[idx]
}

pub fn test_entry_mutation() {
    let mut store = EntryStore { entries: Vec::new() };
    store.entries.push(Entry { key: 1, val: 100, count: 0 });
    store.entries.push(Entry { key: 2, val: 200, count: 0 });

    let e = get_entry_mut(&mut store, 0);
    assert(e.val == 100);
    e.val = 999;
    e.count = e.count + 1;

    assert(store.entries@[0].val == 999);
    assert(store.entries@[0].count == 1);
    assert(store.entries@[1].val == 200);
}

} // verus!
