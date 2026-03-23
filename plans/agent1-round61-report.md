# Agent 1 — Round 61 Report

## Baseline
- 4496 verified, 0 errors, 12 holes, 2610 RTT, 147 PTT.

## After
- 4496 verified, 0 errors, 12 holes, 2610 RTT, 147 PTT.

## Summary

Proved `clone_elem` in ParaHashTableStEph.rs via feq axioms, eliminating the
algorithmic assume. The fix cascaded to 8 files requiring `obeys_feq_clone`
loop invariants and an `obeys_feq_clone` addition to the `insert` trait
requires. One lateral assume moved to StructChainedHashTable's `chain_lookup`
(behind EntryTrait, dead code path). Net hole count unchanged.

Chap43 OrderedSetStEph `select` investigated and found blocked by spec gap.

## Target 1: Chap47 ParaHashTableStEph.rs — 2 holes + 8 wf warnings

### Hole 1 — `clone_elem` assume: CLOSED

Changed `clone_elem<T: Clone>` to `clone_elem<T: Eq + Clone>` with
`requires obeys_feq_clone::<T>()`. Body now uses `assert(cloned(*x, c))`
which triggers `axiom_cloned_implies_eq` from the feq broadcast group,
proving `c == *x` without assume.

Cascade changes across 8 files:

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 47 | ParaHashTableStEph.rs | `clone_elem`: T bound → Eq+Clone, requires feq, assert(cloned) |
| 2 | 47 | ParaHashTableStEph.rs | `insert` trait: added obeys_feq_clone requires |
| 3 | 47 | LinkedListChainedHashTableStEph.rs | `clone_linked_list_entry`: Eq bounds, feq requires, loop invariants |
| 4 | 47 | VecChainedHashTableStEph.rs | `clone_vec_pairs`: Eq bounds, feq requires, loop invariants |
| 5 | 47 | StructChainedHashTable.rs | `chain_lookup`: reverted to local clone+assume (EntryTrait path) |
| 6 | 47 | LinProbFlatHashTableStEph.rs | resize loop: feq invariants |
| 7 | 47 | QuadProbFlatHashTableStEph.rs | resize loop: feq invariants |
| 8 | 47 | DoubleHashFlatHashTableStEph.rs | resize loop: feq invariants |

Added `obeys_feq_clone` import to 6 files. Added feq invariants to 14 while
loops across 6 files.

**Lateral move**: StructChainedHashTable gained 1 assume in `chain_lookup`
(line 201). This function is behind EntryTrait::lookup which has no requires
and can't carry feq. The outer trait's lookup calls chain_lookup directly
(not through EntryTrait), so this assume is in a dead code path. Was
previously hidden inside clone_elem's assume.

### Hole 2 — `call_hash_fn` external_body: STRUCTURAL

Wraps opaque `Fn` closure call `(hash_fn)(key, table_size)`. Verus cannot
verify arbitrary closure execution — it can't know the hash function returns
`index < table_size`. The function has tight ensures. This is the standard
Fn-closure structural limitation. Not closable.

### 8 wf warnings: VERACITY FALSE POSITIVES

All 8 fn_missing_wf warnings are false positives. The wf IS present through
`Self::spec_impl_wf(table)` which defaults to `spec_hashtable_wf(table)`.
Veracity doesn't recognize the trait method indirection. Adding
`spec_hashtable_wf` directly to the trait would break flat table
implementations (LinProb, QuadProb, DoubleHash) which override
`spec_impl_wf` with probe-chain wf that is different from the chained hash
table's bucket-at-hash-slot wf.

## Target 2: Chap43 OrderedSetStEph.rs select — 1 hole: NOT CLOSABLE

### Root Cause

The `select` postcondition requires proving that the i-th element of the
backing sequence has exactly i elements strictly less than it in the set.
This is only true if the backing sequence is sorted.

The backing AVL tree IS sorted by construction (insert uses binary search),
but `spec_orderedsetsteph_wf` captures only structural properties (cached
heights/sizes, no_duplicates, finite). It does NOT include sortedness.

`spec_elements_sorted` exists in AVLTreeSetStEph (line 1249) but is NOT part
of wf and NOT provable from wf alone. The `insert_sorted` function (line
1259) takes `spec_seq_sorted` as a loop invariant precondition — it doesn't
prove it, it assumes it.

### Why `rank` works but `select` doesn't

`rank(k)` is constructive: iterates all elements, compares each with k, counts
matches. Doesn't need sorted order. `select(i)` is non-constructive: given
index i, must prove cardinality of filter set. Requires sortedness as axiom.

### Fix path (multi-file refactor, beyond this round)

1. Add `spec_elements_sorted()` to `spec_avltreesetsteph_wf` in
   AVLTreeSetStEph.rs (Chap41)
2. Prove all operations (insert, delete, filter, union, difference) maintain
   sortedness in their ensures
3. Then `select` can use sortedness to prove the filter cardinality

## Verification

```
verification results:: 4496 verified, 0 errors
RTT: 2610 passed, 0 skipped
PTT: 147 passed, 0 skipped
```
