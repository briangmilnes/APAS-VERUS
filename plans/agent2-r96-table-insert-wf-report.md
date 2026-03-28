# Agent 2 — R96 Report: Table insert_wf

## Objective

Add `insert_wf` to TableStEph and TableStPer that preserves stored-value
well-formedness through the combine closure and internal entry cloning.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | — | `clone_view.rs` | No change needed (blanket impl reverted — verus! impls suffice) |
| 2 | 42 | `TableStEph.rs` | Added `use clone_view::*` import, `insert_wf` trait method + impl |
| 3 | 42 | `TableStPer.rs` | Added `use clone_view::*` import, `insert_wf` trait method + impl |

## Design

### Method Signature

```rust
fn insert_wf<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
    where K: ClonePreservesView, V: ClonePreservesWf
    requires
        old(self).spec_tablesteph_wf(),
        forall|v1: &V, v2: &V| combine.requires((v1, v2)),
        obeys_view_eq::<K>(),
        value.spec_wf(),
        forall|k: K::V| old(self)@.contains_key(k) ==>
            old(self).spec_stored_value(k).spec_wf(),
        forall|v1: &V, v2: &V, r: V|
            combine.ensures((v1, v2), r) && v1.spec_wf() && v2.spec_wf()
            ==> r.spec_wf(),
    ensures
        // All ensures from insert, plus:
        forall|k: K::V| self@.contains_key(k) ==>
            self.spec_stored_value(k).spec_wf();
```

The StPer version uses `&self -> (updated: Self)` instead of `&mut self`.

### Key Insight

The existing `insert` uses `clone_plus()` on whole `Pair<K, V>` entries, which
preserves view (`cloned(a, b)`) but not exec-level well-formedness. The new
`insert_wf` clones components separately:
- **Key**: `pair.0.clone_view()` — preserves view via `ClonePreservesView`
- **Value**: `pair.1.clone_wf()` — preserves both view AND wf via `ClonePreservesWf`

This required proving `pair.1.spec_wf()` inside the loop by connecting each
entry's value to the `spec_stored_value` in the requires quantifier, using key
uniqueness (`spec_keys_no_dups`) to show the `choose` index equals the loop index.

### Why Not Modify Existing `insert`

Adding `K: ClonePreservesView, V: ClonePreservesWf` bounds to the existing
`insert` would break all callers that don't need wf tracking. A new method
with per-method `where` clauses is the safest approach.

### Caller Pattern

AdjTableGraph callers can switch from:
```rust
self.adj.insert(u, neighbors, |_old, new| new.clone());
// assume(forall|k| ... spec_stored_value(k).spec_avltreesetsteph_wf());
```
To:
```rust
self.adj.insert_wf(u, neighbors,
    |_old: &AVLTreeSetStEph<V>, new: &AVLTreeSetStEph<V>| -> (r: AVLTreeSetStEph<V>)
        ensures r@ == new@, r.spec_avltreesetsteph_wf()
    { new.clone_wf() });
// No assume needed — insert_wf ensures stored-value wf.
```

## Verification

| Metric | Value |
|--------|-------|
| Verified | 5388 |
| RTT | 3083 |
| PTT | 157 |
| Errors | 0 |
| Iterations | 4 (of 20) |

## What This Unblocks

The ~3 stored-value-wf assumes in AdjTableGraphStEph (lines 473, 612, 653)
can be eliminated in a follow-up round by switching `insert` calls to
`insert_wf` with `clone_wf()`-based combine closures. The 2 Verus-ICE-blocked
assumes (lines 507, 521) are unrelated and remain.
