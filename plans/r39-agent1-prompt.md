# R39 Agent 1: Restructure OrderedTableStEph.rs — BST-Backed

## Baseline
- Main at `e6e3c688`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4337 verified, 175 holes, 29 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and these standards before starting:
- `src/standards/mod_standard.rs`
- `src/standards/partial_eq_eq_clone_standard.rs`
- `src/standards/total_order_standard.rs`

## Context: Why This Restructure

The APAS textbook (Chapter 43) says ordered tables are implemented using **balanced
binary search trees**, with all ADT 43.1 operations (first, last, previous, next,
rank, select) costing **O(log n)**.

The current `OrderedTableStEph` wraps `TableStEph` → `ArraySeqStEph<Pair<K,V>>` — a
**flat unsorted array**. This means:
- `find`, `insert`, `delete` are O(n) linear scans
- `first_key`, `last_key` do O(n) linear scans with 60-80 line loop invariants
- `collect` needs `Vec::sort_by` (unverified!) to produce sorted output → external_body hole
- All ordering operations call `collect()` first (O(n log n)) instead of O(log n) tree ops

In contrast, `OrderedSetStEph` correctly wraps `AVLTreeSetStEph<T>`.

## Assignment

Restructure `src/Chap43/OrderedTableStEph.rs` to back OrderedTableStEph with
`AVLTreeSetStEph<Pair<K, V>>` instead of `TableStEph<K, V>`.

### Step 1: Change the struct definition

```rust
// OLD:
pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
    pub base_table: TableStEph<K, V>,
}

// NEW:
pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
    pub base_set: AVLTreeSetStEph<Pair<K, V>>,
}
```

### Step 2: Update the View impl

The View should still be `Map<K::V, V::V>`. You need a spec function that converts
the AVL tree set of pairs into a Map:

```rust
pub open spec fn spec_pairs_to_map<KV: View, VV: View>(s: Set<(KV, VV)>) -> Map<KV, VV> {
    // Map from key to value, for all pairs in the set
}
```

Or use the existing `spec_entries_to_map` from the TableStEph module if it works on
sequences (you'll need to convert set → seq → map, or define a set-based version).

### Step 3: Reimplement all operations

The trait interface (ensures/requires) does NOT change. Only the implementations.

**Base operations** (delegate to AVL tree set):
- `size()` → `self.base_set.size()`
- `find(k)` → iterate `to_seq()` to find pair with key k. Or use OrderedSet's
  operations if you can construct a search key.
- `insert(k, v, combine)` → find existing value for k (if any), compute combined value,
  delete old pair, insert new pair
- `delete(k)` → find and remove pair with key k
- `empty()` → `AVLTreeSetStEph::empty()`
- `singleton(k, v)` → `AVLTreeSetStEph::singleton(Pair(k, v))`

**Ordered operations** (leverage sorted tree structure):
- `first_key()` → `self.base_set.first()` then extract `.0` (key). The AVL tree's
  first element is the minimum Pair, which has the minimum key (Pair sorts K-first).
- `last_key()` → `self.base_set.last()` then extract `.0`
- `previous_key(k)` → use the tree's ordering to find the predecessor key
- `next_key(k)` → use the tree's ordering to find the successor key
- `collect()` → `self.base_set.to_seq()` — **NO sort_by needed!** The AVL tree's
  in-order traversal produces pairs sorted by (K, V), which means sorted by key.

**Key insight**: `Pair<K,V>` sorts lexicographically (K first, V second). So `to_seq`
from the AVL tree produces pairs sorted by key (primary) then value (secondary).
The `collect()` function becomes trivial — just convert the tree to a sequence.

### Step 4: Handle key uniqueness

`AVLTreeSetStEph` is a SET — no duplicates. `Pair<K,V>` uses lexicographic equality,
so `Pair(k, v1)` and `Pair(k, v2)` are different elements. You need to ensure table
semantics (one value per key):

- `insert(k, v)`: first remove any existing pair with key k, then insert `Pair(k, v)`
- `delete(k)`: find the pair with key k, remove it
- `spec_orderedtablesteph_wf()`: must include a no-duplicate-keys predicate

### Step 5: Simplify ordering operations

The current `first_key` is a 78-line O(n) loop. With BST backing, it becomes:

```rust
fn first_key(&self) -> (first: Option<K>) {
    let first_pair = self.base_set.first();
    match first_pair {
        None => None,
        Some(p) => Some(p.0.clone()),
    }
}
```

Similar dramatic simplifications for `last_key`, `previous_key`, `next_key`, `rank_key`,
`select_key`. Most reduce from 60-80 lines to 5-15 lines.

### Step 6: Remove sort_by from collect

The whole point. `collect()` should be:

```rust
fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>) {
    // to_seq already returns sorted pairs
    let seq = self.base_set.to_seq();
    // Convert from StEph seq to StPer seq if needed
    ...
}
```

No `#[verifier::external_body]`. No `Vec::sort_by`. Fully verified.

### Step 7: Update or remove TableStEph imports

Remove `use crate::Chap42::TableStEph::TableStEph::*` and related imports.
Add `use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*`.

### Important Notes

- The trait (ensures, requires) MUST NOT CHANGE. Same API, different implementation.
- Read how `OrderedSetStEph` delegates to `AVLTreeSetStEph` — follow the same patterns.
- Existing helper lemmas (`lemma_entries_to_map_*`) may need View-level equivalents
  that work on AVL tree sets instead of ArraySeq.
- `collect` currently returns `AVLTreeSeqStPerS` (persistent seq). You may need to
  convert from the ephemeral `to_seq` result.
- Some operations like `filter`, `map`, `reduce`, `tabulate` also exist in the trait.
  These may need adaptation too.
- Run `scripts/validate.sh` after each major change. Must be 0 errors.

### What to do if stuck

If a particular operation is hard to restructure (e.g., `tabulate`, `map`, `reduce`),
mark it `#[verifier::external_body]` temporarily and move on to the next operation.
The priority is: collect (no sort_by), first_key, last_key, find, insert, delete.
Report which operations you couldn't complete.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent1-r39-report.md`.
