# Agent 3 Round 34 Report

## Summary

Closed 2 real holes across Chap37 and Chap45. Chap57 and Chap59 holes were assessed
but require deeper proof infrastructure changes beyond quick wins.

## Results

| # | Chap | File | Before | After | Delta | Technique |
|---|------|------|--------|-------|-------|-----------|
| 1 | 37 | AVLTreeSeqMtPer.rs | 2 | 1 | -1 | Extracted nested fn to module level, sequential proof |
| 2 | 45 | BalancedTreePQ.rs | 1 | 0 | -1 | Removed external, added closure requires, while-loop invariants |

**Total: -2 real holes. Verification: 4158 verified, 0 errors. RTT: 2613 passed.**

## Details

### Chap37: AVLTreeSeqMtPer.rs `build_balanced_from_slice` (-1 hole)

Removed `#[verifier::external_body]` from `build_balanced_from_slice`. The function had
a nested `fn rec` using `ParaPair!` which was unverifiable because:
1. Verus doesn't support nested functions
2. `ParaPair!` requires `Send + 'static` closures, but `&[T]` captures aren't `'static`

Fix: extracted recursive logic to module level using `slice_subrange` (matching the
proven StPer pattern in AVLTreeSeqStPer.rs). Key proof steps:
- `lemma_size_eq_inorder_len` + `lemma_height_le_size` for `mk` preconditions
- `assert(obeys_feq_full_trigger::<T>())` to trigger broadcast axiom for clone view equality
- `assert(cloned(a@[mid as int], val))` for clone postcondition
- Sequence decomposition: `a@ =~= subrange(0,mid) + seq![a@[mid]] + subrange(mid+1,n)`

Remaining hole: `subseq_copy` at line 642 — flagged as OPAQUE_EXTERNAL structural FP.
**Chap37 real actionable holes: 0.**

### Chap45: BalancedTreePQ.rs filter/map (-1 hole)

Removed `#[verifier::external]` from `BalancedTreePQExtTrait` impl. Functions `filter`
and `map` now verify inside `verus!` with:
- Added `BalancedTreePQTrait<T>` as supertrait for `spec_balancedtreepq_wf` access
- Added `requires self.spec_balancedtreepq_wf()` and closure requires
- Added `ensures filtered/mapped.spec_balancedtreepq_wf()` (fixes fn_missing_wf_ensures)
- Converted `for` loops to `while` loops with wf-maintaining invariants

**BalancedTreePQ.rs now clean (0 holes).**

### Chap57: DijkstraStEphU64.rs (deferred)

Two assumes remain:
1. **Line 202**: `assume(spec_is_exec_heap(pq.spec_seq()))` — requires adding
   `ensures spec_is_exec_heap(...)` to BinaryHeapPQ `insert` and `delete_min`, which
   requires proving `bubble_up` and `sift_down` maintain the heap property.
2. **Line 243**: `assume(remaining_budget > 0)` — requires tracking which edges have
   been processed per vertex to prove total insertions ≤ |E|.

Both are substantial proof engineering tasks on BinaryHeapPQ.rs internals.

### Chap59: JohnsonStEphI64.rs (deferred)

**Line 437**: `assume(reweighted@.A.len() * 2 + 2 <= usize::MAX)` — requires proving
that `reweight_graph` preserves the edge count bound. This needs:
1. `from_weighed_edges` to expose `g@.A =~= edges@` or `g@.A.len() <= edges@.len()`
2. `reweight_graph` to track edge set cardinality through its construction loop
3. The edge bound propagated as a precondition to `johnson_apsp`

The graph module's `from_weighed_edges` doesn't expose the edge-set-to-A relationship,
making this a multi-file change.

## Verification

```
verification results:: 4158 verified, 0 errors
RTT: 2613 tests run: 2613 passed, 0 skipped
```
