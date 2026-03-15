# Review Against Prose -- Chap49

**Date**: 2026-03-15
**Reviewer**: Claude-Opus-4.6 (Agent 4, Round 21)

## Phase 1: Inventory

8 modules: MinEditDistStEph, MinEditDistStPer, MinEditDistMtEph, MinEditDistMtPer, SubsetSumStEph, SubsetSumStPer, SubsetSumMtEph, SubsetSumMtPer.

Total: 82 functions across 8 files. 0 proof holes. All 8 modules clean.

## Phase 2: Prose Inventory

| # | Chap | Item | Type | Prose Reference |
|---|------|------|------|-----------------|
| 1 | 49 | Subset Sum (SS) Problem | Definition 49.1 | Given multiset S, target k, find X subset S s.t. sum(X) = k |
| 2 | 49 | Recursive Subset Sum (Indexed) | Algorithm 49.3 | SS'(i,j) with base cases and recursion |
| 3 | 49 | SS Work | Cost | W(SS(S,k)) = O(k*\|S\|) |
| 4 | 49 | SS Span | Cost | S(SS(S,k)) = O(\|S\|) |
| 5 | 49 | MED Problem | Definition 49.4 | Min insertions/deletions to transform S to T |
| 6 | 49 | Recursive MED | Algorithm 49.5 | MED(S,T) with list-based recursion |
| 7 | 49 | Recursive MED (Indexed) | Algorithm 49.6 | MED'(i,j) with integer indices |
| 8 | 49 | MED Work | Cost | W(MED(S,T)) = O(\|S\|*\|T\|) |
| 9 | 49 | MED Span | Cost | S(MED(S,T)) = O(\|S\|+\|T\|) |
| 10 | 49 | Subproblem sharing | Property | O(k*\|S\|) distinct subproblems for SS; O(\|S\|*\|T\|) for MED |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

Cost annotations added to all exec functions in SubsetSumStEph.rs and MinEditDistStEph.rs:
- `subset_sum`, `subset_sum_rec`: APAS Work O(k*\|S\|), Span O(\|S\|) -- code agrees.
- `min_edit_distance`, `min_edit_distance_rec`: APAS Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|) -- code agrees.
- Scaffolding (new, from_*, accessors, set, clear_memo, memo_size): marked N/A.

### 3b. Implementation Fidelity

| # | Chap | File | Function | Prose Match | Notes |
|---|------|------|----------|-------------|-------|
| 1 | 49 | SubsetSumStEph.rs | subset_sum_rec | Algorithm 49.3 | Faithful: indexed recursion with memoization via HashMap |
| 2 | 49 | MinEditDistStEph.rs | min_edit_distance_rec | Algorithm 49.6 | Faithful: indexed recursion with memoization via HashMap |
| 3 | 49 | SubsetSumMtEph.rs | subset_sum_rec | Algorithm 49.3 | Parallel: fork-join on both branches via HFScheduler |
| 4 | 49 | MinEditDistMtEph.rs | min_edit_distance_rec | Algorithm 49.6 | Parallel: fork-join on delete/insert branches |

**Deviations**:
- Prose uses list-based recursion (Algorithm 49.5); implementation uses indexed variant (Algorithm 49.6). This is intentional -- prose recommends indexed form.
- SS implementation requires `T: Into<i32>` with `element_value < 0` guard; prose assumes positive integers. The extra guard handles the type bridge safely.
- MED uses `HashMapWithViewPlus<Pair<usize, usize>, usize>` for memo; prose uses abstract mapping. Faithful adaptation.

### 3c. Spec Fidelity

| # | Chap | File | Spec Function | Prose Match | Strength |
|---|------|------|---------------|-------------|----------|
| 1 | 49 | SubsetSumStEph.rs | spec_subset_sum | Definition 49.1 + Algorithm 49.3 | Correct recursive definition |
| 2 | 49 | MinEditDistStEph.rs | spec_med | Algorithm 49.6 | Correct recursive MED definition |
| 3 | 49 | MinEditDistStEph.rs | spec_memo_bounded | -- | Verus-specific invariant (memo values bounded) |

**Spec gap**: `spec_subset_sum` and `spec_med` are defined on `Seq<int>` but exec code uses generic `T: StT`. No `spec_to_int_seq` bridge function exists. Trait methods lack ensures connecting the return value to spec functions. Core algorithms (subset_sum_rec, min_edit_distance_rec) have structural ensures (lengths preserved, memo bounded) but not functional ensures (result == spec_xxx).

MinEditDistStEph.min_edit_distance_rec has `ensures dist <= i + j` -- a partial spec proving the result is bounded, but not equal to spec_med.

## Phase 4: Parallelism Review

| # | Chap | File | Operation | Parallel? | Mechanism |
|---|------|------|-----------|-----------|-----------|
| 1 | 49 | SubsetSumMtEph.rs | subset_sum_rec | Yes | fork-join via HFScheduler on both SS branches |
| 2 | 49 | MinEditDistMtEph.rs | min_edit_distance_rec | Yes | fork-join via HFScheduler on delete/insert branches |
| 3 | 49 | SubsetSumMtPer.rs | subset_sum_rec | Yes | fork-join via HFScheduler |
| 4 | 49 | MinEditDistMtPer.rs | min_edit_distance_rec | Yes | fork-join via HFScheduler |

All Mt modules use genuine parallelism via `join()` from HFSchedulerMtEph. Memo tables use `Arc<RwLock<HashMapWithViewPlus>>` for thread-safe concurrent access. No sequential degradation.

## Phase 5: Runtime Test Review

All 8 files have corresponding RTTs:
- TestSubsetSumStEph.rs, TestSubsetSumStPer.rs, TestSubsetSumMtEph.rs, TestSubsetSumMtPer.rs
- TestMinEditDistStEph.rs, TestMinEditDistStPer.rs, TestMinEditDistMtEph.rs, TestMinEditDistMtPer.rs

Coverage: tests exercise construction, basic SS queries (positive/negative results from Example 49.1), MED computation, and accessor methods.

## Phase 6: PTT Review

No PTTs exist for Chap49. No iterators or complex loop forms that would benefit from proof-time testing. **No PTTs needed.**

## Phase 7: Gap Analysis

### Prose items without implementation

None. Both algorithms (SS and MED) are implemented in all four variants.

### Code without prose counterpart

| # | Chap | File | Item | Notes |
|---|------|------|------|-------|
| 1 | 49 | all | new, from_*, set_*, clear_memo, memo_size | Verus container scaffolding |
| 2 | 49 | all | source_mut, target_mut, multiset_mut | &mut-returning methods (outside verus!) |
| 3 | 49 | all | Clone, PartialEq, Eq, Debug, Display, IntoIterator | Derive/trait impls |

### Spec gaps

- `spec_subset_sum` not connected to `subset_sum` return value (T-to-int bridge needed).
- `spec_med` not connected to `min_edit_distance` return value (T-to-int bridge needed).
- Mt variant spec functions duplicate St spec functions (chapter standalone rule).

## Phase 8: TOC Review

All 8 files follow the standard TOC ordering:
1. module, 2. imports, 3. broadcast use, 4. type definitions, 6. spec fns, 8. traits, 9. impls, 11. derive impls in verus!, 13. derive impls outside verus!

No section ordering violations detected.

## Proof Holes Summary

**0 holes** across all 8 modules. All 8 modules clean. No assumes, admits, or external_body markers.

Accepts noted: 0.
Info-level accepts: 0.

All proof obligations are discharged. The only weakness is the absence of functional postconditions (spec_subset_sum/spec_med connected to return values), which is an architectural limitation, not a proof hole.
