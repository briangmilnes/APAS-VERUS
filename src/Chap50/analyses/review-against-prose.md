# Review Against Prose -- Chap50

**Date**: 2026-03-15
**Reviewer**: Claude-Opus-4.6 (Agent 4, Round 21)

## Phase 1: Inventory

8 modules: MatrixChainStEph, MatrixChainStPer, MatrixChainMtEph, MatrixChainMtPer, OptBinSearchTreeStEph, OptBinSearchTreeStPer, OptBinSearchTreeMtEph, OptBinSearchTreeMtPer.

Total: 90 functions across 8 files. 0 proof holes. All 8 modules clean.
Info-level accepts: 18 total (2 per MatrixChain file for clone/eq, 2-3 per OBST file for clone/eq/KeyProb clone).

## Phase 2: Prose Inventory

| # | Chap | Item | Type | Prose Reference |
|---|------|------|------|-----------------|
| 1 | 50 | OBST Problem | Definition 50.1 | Minimize expected cost sum(d(s,T)*p(s)) over all BSTs |
| 2 | 50 | Recursive OBST | Algorithm 50.2 | OBST(S) = sum(p) + min_r(OBST(S_left) + OBST(S_right)) |
| 3 | 50 | Recursive OBST (indexed) | Algorithm 50.3 | OBST'(i,l) with offset i and length l |
| 4 | 50 | OBST Work | Cost | W = O(n^3) |
| 5 | 50 | OBST Span | Cost | S = O(n log n) |
| 6 | 50 | Matrix Chain Product | Similar Problems | Mentioned as similar DP structure to OBST |
| 7 | 50 | Optimal substructure property | Property | Subtrees of optimal BST are optimal |
| 8 | 50 | Sharing bound | Property | O(n^2) distinct subproblems, O(n) longest path |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

MatrixChain and OBST StEph/StPer files already have cost annotations on outside-verus! functions. Inside-verus! trait functions in MatrixChainStEph have no explicit APAS/Claude lines but the spec fns document intent. All Chap50 files have Claude-Opus-4.6 lines on Display/IntoIterator methods.

Key algorithmic cost annotations present in trait comments:
- MatrixChain: optimal_cost, multiply_cost, matrix_chain_rec -- no doc-comment cost lines but strong functional specs.
- OBST: optimal_cost, obst_rec -- no doc-comment cost lines, weak specs.

### 3b. Implementation Fidelity

| # | Chap | File | Function | Prose Match | Notes |
|---|------|------|----------|-------------|-------|
| 1 | 50 | MatrixChainStEph.rs | matrix_chain_rec | Algorithm 50.3 analog | Faithful: indexed recursion with k-loop for splits, memoization |
| 2 | 50 | MatrixChainStEph.rs | multiply_cost | Definition from prose | rows[i]*cols[k]*cols[j] |
| 3 | 50 | MatrixChainStEph.rs | optimal_cost | Top-level OBST call | Clears memo, calls rec(0, n-1) |
| 4 | 50 | OptBinSearchTreeStEph.rs | obst_rec_st_eph | Algorithm 50.3 | Faithful: indexed with (i, l), prob sum + min split |
| 5 | 50 | OptBinSearchTreeStEph.rs | optimal_cost | Top-level call | Clears memo, calls rec(0, n) |
| 6 | 50 | MatrixChainMtEph.rs | matrix_chain_rec | Parallel version | Uses Arc<RwLock> memo |
| 7 | 50 | OptBinSearchTreeMtEph.rs | obst_rec | Parallel version | Uses Arc<RwLock> memo |

**Deviations**:
- Matrix Chain uses `MatrixDim { rows, cols }` struct; prose uses dimension sequences. Faithful adaptation.
- OBST uses `KeyProb<T>` struct combining key and probability; prose treats them separately. Faithful adaptation.
- OBST uses `Probability` wrapper (f64-based) for costs; prevents spec-level arithmetic reasoning.
- Matrix Chain multiply_cost computes `rows[i] * cols[k] * cols[j]`; standard matrix chain would be `rows[i] * cols[i] * cols[j]` where consecutive matrices share a dimension. The implementation assumes `dims[k].cols` is the split point column count, which is correct for the indexed formulation.

### 3c. Spec Fidelity

| # | Chap | File | Spec Function | Prose Match | Strength |
|---|------|------|---------------|-------------|----------|
| 1 | 50 | MatrixChainStEph.rs | spec_chain_cost | Algorithm 50.3 | Strong -- full recursive optimal cost |
| 2 | 50 | MatrixChainStEph.rs | spec_multiply_cost | Cost formula | Strong -- rows*cols*cols |
| 3 | 50 | MatrixChainStEph.rs | spec_dims_bounded | -- | Verus-specific (overflow bounds) |
| 4 | 50 | MatrixChainStEph.rs | spec_costs_fit | -- | Verus-specific (cost fits in usize) |
| 5 | 50 | MatrixChainStEph.rs | spec_memo_correct | -- | Verus-specific (memo invariant) |
| 6 | 50 | OptBinSearchTreeStEph.rs | (none) | Algorithm 50.3 | **Missing** -- no spec_obst_cost |

**MatrixChain spec strength**: Strong. `optimal_cost` ensures `cost == spec_chain_cost(dims, 0, n-1, 0)`. `matrix_chain_rec` ensures `cost == spec_chain_cost(dims, i, j, i)` and maintains `spec_memo_correct`. Full functional verification.

**OBST spec gap**: No spec function defines the optimal search cost. `optimal_cost` has no ensures. `obst_rec_st_eph` ensures only `s@.keys =~= old(s)@.keys` and `s@.memo.dom().finite()` -- structural, not functional. The Probability type wraps f64, which has no spec arithmetic axioms, making it impossible to define `spec_obst_cost` without float axiom infrastructure.

## Phase 4: Parallelism Review

| # | Chap | File | Operation | Parallel? | Mechanism |
|---|------|------|-----------|-----------|-----------|
| 1 | 50 | MatrixChainMtEph.rs | matrix_chain_rec | Partial | Sequential k-loop with Arc<RwLock> memo, parallel_min_reduction helper |
| 2 | 50 | MatrixChainMtPer.rs | matrix_chain_rec | Partial | Same pattern as MtEph |
| 3 | 50 | OptBinSearchTreeMtEph.rs | obst_rec | Thread-safe | Sequential k-loop with Arc<RwLock> memo, no fork-join |
| 4 | 50 | OptBinSearchTreeMtPer.rs | obst_rec | Thread-safe | Same as MtEph |

**Parallelism gap**: Both MatrixChain and OBST Mt files use `Arc<RwLock>` for thread-safe memo access, but the inner k-loop for finding the minimum split is sequential. MatrixChain has a `parallel_min_reduction` trait method, but the inner split-point exploration loop is still sequential. OBST's Mt implementation is purely thread-safe (concurrent memo access) but not parallel in the computational sense. This matches the prose's description of the DP structure -- the parallelism comes from independent subproblems across the DAG, not from the inner minimization loop.

The MatrixChain MtEph file does import `join` from HFScheduler but uses it for the parallel_min_reduction, not for the recursive subproblem calls.

## Phase 5: Runtime Test Review

All 8 files + 1 extra have corresponding RTTs:
- TestMatrixChainStEph.rs, TestMatrixChainStPer.rs, TestMatrixChainMtEph.rs, TestMatrixChainMtPer.rs
- TestOptBinSearchTreeStEph.rs, TestOptBinSearchTreeStPer.rs, TestOBSTMtEph.rs, TestOBSTMtPer.rs
- TestProbability.rs (tests Probability type from Chap30)

Coverage: tests exercise construction, optimal cost computation (using textbook example values), dimension/key management.

## Phase 6: PTT Review

No PTTs exist for Chap50. No iterators or complex loop forms. **No PTTs needed.**

## Phase 7: Gap Analysis

### Prose items without implementation

None. Both OBST (Algorithm 50.3) and Matrix Chain (similar problem) are implemented in all 4 variants.

### Code without prose counterpart

| # | Chap | File | Item | Notes |
|---|------|------|------|-------|
| 1 | 50 | MatrixChain* | MatrixDim struct | Verus adaptation of dimension pairs |
| 2 | 50 | OBST* | KeyProb struct | Verus adaptation of key-probability pairs |
| 3 | 50 | all | new, from_*, set_*, clear_memo, memo_size, dimensions, keys | Container scaffolding |
| 4 | 50 | all | Clone, PartialEq, Eq, Debug, Display, IntoIterator | Derive/trait impls |
| 5 | 50 | MatrixChain* | multiply_cost (standalone) | Helper extracted from rec for spec clarity |
| 6 | 50 | MatrixChain* | parallel_min_reduction | Mt-specific reduction helper |

### Spec gaps

- **OBST**: No `spec_obst_cost` function. Blocked by Probability (f64) lacking spec arithmetic.
- Matrix Chain is fully specified and verified.

## Phase 8: TOC Review

All 8 files follow the standard TOC ordering. MatrixChain files use sections 1-11 (inside verus!) plus 13 (outside verus!). OBST files use the same structure. No section ordering violations.

Note: MatrixChainStEph.rs has a "12. macros" section label at line 442 after the outside-verus! derives, which should be section 13. Minor labeling inconsistency.

## Proof Holes Summary

**0 holes** across all 8 modules. All 8 modules clean.

**Info-level accepts**: 18 total across all files. All are in Clone::clone or PartialEq::eq bodies (standard accept pattern for view-based equality bridging). None in algorithmic code.

**Key strength**: MatrixChain has fully verified functional specs -- `optimal_cost` returns exactly `spec_chain_cost(dims, 0, n-1, 0)`.

**Key weakness**: OBST lacks any functional spec. The Probability type's f64 internals prevent defining `spec_obst_cost` without float arithmetic axioms.
