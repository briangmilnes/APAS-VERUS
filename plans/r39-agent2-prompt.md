# R39 Agent 2: Restructure OrderedTableStPer.rs — BST-Backed

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

## Context

Same as Agent 1's context. The APAS textbook says ordered tables use balanced BSTs.
The current `OrderedTableStPer` wraps `TableStPer` → flat unsorted array. The `collect`
function uses `Vec::sort_by` (unverified) and is marked `external_body`.

This file is the PERSISTENT variant. Per CLAUDE.md standalone rules, it does NOT import
from OrderedTableStEph — it is self-contained. You are working independently from Agent 1
who is restructuring the ephemeral variant in parallel.

## Assignment

Restructure `src/Chap43/OrderedTableStPer.rs` to back OrderedTableStPer with
`AVLTreeSetStPer<Pair<K, V>>` instead of `TableStPer<K, V>`.

### Step 1: Change the struct definition

```rust
// OLD:
pub struct OrderedTableStPer<K: StT + Ord, V: StT> {
    pub base_table: TableStPer<K, V>,
}

// NEW:
pub struct OrderedTableStPer<K: StT + Ord, V: StT> {
    pub base_set: AVLTreeSetStPer<Pair<K, V>>,
}
```

Read `src/Chap41/AVLTreeSetStPer.rs` to understand the persistent AVL tree API.
The persistent variant returns new trees instead of mutating in place.

### Step 2: Update the View impl

View stays `Map<K::V, V::V>`. Need spec function converting AVL set of pairs → Map.

### Step 3: Reimplement all operations

The trait interface (ensures/requires) does NOT change. Only implementations.

**Base operations:**
- `size()` → `self.base_set.size()`
- `find(k)` → iterate to_seq to find pair with key k
- `insert(k, v, combine)` → persistent: return new tree with old key removed + new pair added
- `delete(k)` → persistent: return new tree with pair for key k removed
- `collect()` → `self.base_set.to_seq()` — **NO sort_by!** Already sorted by (K, V).

**Ordered operations** (leverage sorted tree):
- `first_key()` → `self.base_set.first()` extract key
- `last_key()` → `self.base_set.last()` extract key
- `previous_key(k)`, `next_key(k)` → use tree ordering
- `rank_key(k)`, `select_key(i)` → use tree rank/select

### Step 4: Handle key uniqueness

Same as Agent 1. Pair<K,V> uses lexicographic Eq, so same key + different value =
different elements. You must ensure table semantics:
- `insert`: remove existing pair with same key before inserting
- `wf`: include no-duplicate-keys predicate

### Step 5: Persistent semantics

StPer methods return new values instead of mutating:
- `insert(self, k, v, combine) -> Self` (takes self by value or &self, returns new)
- `delete(self, k) -> (Self, Option<V>)` (returns new table + old value)

Make sure the return types match the existing trait signatures exactly.

### Important Notes

- Read `OrderedSetStPer.rs` to see how the persistent ordered SET wraps AVLTreeSetStPer.
  Follow the same delegation patterns.
- The trait interface MUST NOT CHANGE.
- Remove `use crate::Chap42::TableStPer` imports, add AVLTreeSetStPer imports.
- If an operation is hard, mark it `#[verifier::external_body]` temporarily.
  Priority: collect (no sort_by), first_key, last_key, find, insert, delete.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent2-r39-report.md`.
