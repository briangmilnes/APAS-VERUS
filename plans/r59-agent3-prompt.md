# R59 Agent 3 — Chap43 Mixed Holes (6 holes)

## Assignment

Close 6 holes in Chap43 across 5 files. These range from medium to hard.

## Targets by Difficulty

### Medium: RWLOCK Capacity (2 holes)

**File:** `OrderedTableMtPer.rs`

Function `map` (line 361):
```
proof { assume(len + 1 < usize::MAX as nat); } // RWLOCK_GHOST
```

Function `filter` (line 397):
```
proof { assume(len + 1 < usize::MAX as nat); } // RWLOCK_GHOST
```

Both functions acquire a read lock, call `inner.collect()` to get entries, then iterate.
The `len` comes from `entries.length()`. The RwLock invariant should bound the table
size. Check:
1. What does `OrderedTableStPer::collect()` ensure about the result length?
2. Does the RwLock predicate (`OrderedTableMtPerInv`) carry a capacity bound?
3. If not, add `len < usize::MAX as nat` to the RwLock invariant and prove it holds
   at construction and after each mutation.

If the invariant already bounds the table, the assume is provable from the lock's
postcondition. If not, you need to strengthen the invariant — which cascades to
`new`, `insert`, `delete`, `union`, etc.

### Medium: Reducer Clone Lemmas (2 holes)

**File:** `AugOrderedTableMtEph.rs` (line 121)
**File:** `AugOrderedTableStPer.rs` (line 118)

Both are `external_body` on `proof fn lemma_*_reducer_clone_total`:
```rust
requires forall|v1: &V, v2: &V| #[trigger] original.requires((v1, v2))
ensures  forall|v1: &V, v2: &V| #[trigger] cloned.requires((v1, v2))
```

These say: cloning a reducer `F: Fn(&V, &V) -> V + Clone` preserves its `requires`.
This is a closure clone axiom — Verus cannot prove it from first principles because
`Clone` on closures is opaque.

**Approach:** Check if `vstdplus` has a closure clone lemma or broadcast. Check
`src/vstdplus/feq/feq.rs` for patterns. If no existing infrastructure, this may
need an `assume` inside the proof body (like the eq/clone workaround pattern).
If so, document why and leave it — do NOT add `accept()`.

### Hard: Select Rank Invariant (2 holes)

**File:** `OrderedSetStEph.rs` (line 1146)
**File:** `OrderedSetStPer.rs` (line 1060)

Both are in `select` — selecting the i-th smallest element:
```rust
assume(self@.filter(|x: T::V| exists|t: T| #[trigger] TotalOrder::le(t, result)
    && t@ == x && t@ != result@).len() == i as int);
```

This says: the number of elements strictly less than `result` equals the rank `i`.

The proof requires:
1. The AVL tree backing is sorted (this should follow from `spec_avltreeseq*_wf()`)
2. In a sorted sequence, element at index i has exactly i predecessors
3. Converting from sorted-sequence indexing to set-filter cardinality

Check if `AVLTreeSeqStEph`/`AVLTreeSeqStPer` has a sortedness spec or lemma.
If the wf spec doesn't include sortedness, you may need to add it or find an
alternative proof path.

This is the hardest target. If you can't prove it, document what's missing and
leave the assume in place — do NOT add `accept()`.

## Validation

Run `scripts/validate.sh` after each target. Show full output. Start with the
RWLOCK holes (easiest), then reducer clones, then select.

## Report

Write `plans/agent3-round59-report.md` with:
- Holes before/after per file (table with # and Chap columns)
- For each hole: what was tried, what worked/didn't
- Verification count
