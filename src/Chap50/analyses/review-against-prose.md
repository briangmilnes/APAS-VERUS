<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 50: Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6 (automated)
**Project:** APAS-VERUS-agent2
**Chapter:** 50 — DP: MatrixChain, OptBinSearchTree, Probability

## Phase 1: Prose Summary

Chapter 50 covers the **Optimal Binary Search Tree (OBST) problem** (Definition 50.1, Algorithms 50.2 and 50.3). The textbook:

1. Defines the OBST problem: given sorted keys S and probability function p, find the BST minimizing expected search cost.
2. Establishes the **optimal substructure property**: subtrees of an optimal BST are themselves optimal.
3. Derives the recursive cost formula: `Cost(T) = Σ p(s) + Cost(T_L) + Cost(T_R)`.
4. Presents **Algorithm 50.2** (recursive OBST by key sets) and **Algorithm 50.3** (indexed OBST using offset `i` and length `l`).
5. Analyzes the memoized DAG: **O(n²) vertices**, **O(n) longest path**, each vertex O(n) work / O(log n) span.
6. Concludes: **Work O(n³)**, **Span O(n log n)** for the parallel version.
7. Mentions **Matrix Chain Multiplication** as a "similar problem" solvable with the same DP structure and identical cost bounds.

## Phase 2: Source Files Inventory

| # | File | Algorithm | Variant | Lines | verus! content |
|---|---|---|---|---|---|
| 1 | `MatrixChainStEph.rs` | Matrix Chain DP | St/Eph | ~230 | empty |
| 2 | `MatrixChainStPer.rs` | Matrix Chain DP | St/Per | ~188 | empty |
| 3 | `MatrixChainMtEph.rs` | Matrix Chain DP | Mt/Eph | ~310 | empty |
| 4 | `MatrixChainMtPer.rs` | Matrix Chain DP | Mt/Per | ~246 | empty |
| 5 | `OptBinSearchTreeStEph.rs` | OBST (Alg 50.3) | St/Eph | ~224 | empty |
| 6 | `OptBinSearchTreeStPer.rs` | OBST (Alg 50.3) | St/Per | ~188 | empty |
| 7 | `OptBinSearchTreeMtEph.rs` | OBST (Alg 50.3) | Mt/Eph | ~315 | empty |
| 8 | `OptBinSearchTreeMtPer.rs` | OBST (Alg 50.3) | Mt/Per | ~247 | empty |
| 9 | `Probability.rs` | Probability wrapper | Infrastructure | ~154 | empty |

**Total:** 9 source files, 9 test files.

## Phase 3: Algorithm Fidelity

### OBST — Algorithm 50.3 (indexed recursive)

The implementation matches Algorithm 50.3 exactly:

| # | Prose element | Code | Match? |
|---|---|---|---|
| 1 | Subproblem indexed by `(i, l)` (offset + length) | `obst_rec(i, l)` | ✅ |
| 2 | Base case: `l = 0 → 0` | `if l == 0 { Probability::zero() }` | ✅ |
| 3 | Sum probabilities: `Σ p(S[i+k])` for k=0..l-1 | `(0..l).map(\|k\| self.keys[i+k].prob).fold(zero, +)` | ✅ |
| 4 | Min over splits: `min_{k=0..l-1}(OBST'(i,k) + OBST'(i+k+1, l-k-1))` | `(0..l).map(\|k\| left + right).fold(infinity, min)` | ✅ |
| 5 | Result: `prob_sum + min_cost` | `prob_sum + min_cost` | ✅ |
| 6 | Memoization via hash table | `HashMap<(usize, usize), Probability>` | ✅ |
| 7 | Top-level call: `OBST'(0, \|S\|)` | `obst_rec(0, n)` | ✅ |

### Matrix Chain Multiplication

The prose mentions Matrix Chain as a "similar problem" (pp. 365–366) without giving a formal algorithm. The implementation follows the standard textbook DP:

| # | Element | Code | Match? |
|---|---|---|---|
| 1 | Subproblem indexed by `(i, j)` (start, end) | `matrix_chain_rec(i, j)` | ✅ |
| 2 | Base case: `i = j → 0` | `if i == j { 0 }` | ✅ |
| 3 | Min over splits with multiply cost | `min(left + right + rows[i]*cols[k]*cols[j])` | ✅ |
| 4 | Memoization via hash table | `HashMap<(usize, usize), usize>` | ✅ |

### Probability wrapper

Not in the textbook. Infrastructure type wrapping f64 with total ordering (Eq, Ord, Hash) needed for HashMap keys and min comparisons. Implements arithmetic ops (Add, Sub, Mul, Div) for f64 probability values.

## Phase 4: Cost Analysis

### Main algorithms

| # | Algorithm | Variant | Work | Span | Source |
|---|---|---|---|---|---|
| 1 | Matrix Chain | St | Θ(n³) | Θ(n³) | Sequential: n² subproblems × O(n) each |
| 2 | Matrix Chain | Mt | Θ(n³) | Θ(n² lg n) | Parallel min reduction: O(lg n) per subproblem |
| 3 | OBST | St | Θ(n³) | Θ(n³) | Sequential: n² subproblems × O(n) each |
| 4 | OBST | Mt | Θ(n³) | Θ(n² lg n) | Parallel min reduction: O(lg n) per subproblem |
| 5 | Parallel min reduction | Mt | Θ(n) | Θ(lg n) | Divide-and-conquer with thread::spawn |
| 6 | Probability ops | All | Θ(1) | Θ(1) | f64 arithmetic |

### Textbook comparison

The textbook claims Span O(n log n) for the optimal parallel OBST. The implementations achieve Span Θ(n² lg n) because:
- The inner loop over split positions `k` runs **sequentially** (collecting costs into a Vec before the parallel reduction).
- A fully parallel version would compute subproblems at the same DP level in parallel (bottom-up), achieving the textbook bound.
- The implementations use **top-down memoized recursion**, which serializes the dependency chain.

### Cost annotations

All 9 files have been annotated with two-line cost comments on every exec function:
```
/// - APAS: Work Θ(...), Span Θ(...)
/// - Claude-Opus-4.6: Work Θ(...), Span Θ(...) — [analysis]
```

## Phase 5: Verification Status

### Proof holes

```
veracity-review-proof-holes output:
✓ All 9 files clean — 0 proof holes found.
```

### verus! block status

All 9 files have **empty** `verus! {}` blocks. No spec functions, no requires/ensures, no proof functions. All code lives outside `verus!` due to Verus limitations with:
- `&mut self` / `&mut` return types (ephemeral mutation)
- `HashMap` (not supported in Verus)
- Complex trait bounds (`T: MtVal + Send + Sync + 'static`)
- `Arc<Mutex<...>>` concurrency primitives

The 0 proof holes result is trivially correct: there are no proofs to have holes in.

## Phase 6: In/Out Table

All code is outside `verus!` due to the Verus limitations above. The in/out designation reflects what _should_ be inside vs what _is_ inside.

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Notes |
|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|---|
| 1 | MatrixChainStEph | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | ✅ out | All derive via `#[derive]` outside verus! |
| 2 | MatrixChainStPer | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | ✅ out | Same pattern |
| 3 | MatrixChainMtEph | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | ✅ out | Manual PartialEq for Arc<Mutex> |
| 4 | MatrixChainMtPer | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | ✅ out | Same pattern |
| 5 | OBSTStEph | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | ✅ out | All derive via `#[derive]` |
| 6 | OBSTStPer | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | ✅ out | Same pattern |
| 7 | OBSTMtEph | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | ✅ out | Manual PartialEq for Arc<Mutex> |
| 8 | OBSTMtPer | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | ✅ out | Same pattern |
| 9 | Probability | ❌ out | ❌ out | ❌ out | - | - | ✅ out | ✅ out | ✅ out | Total ordering wrapper for f64 |

**Legend:** ✅ out = correctly outside verus! (required). ❌ out = incorrectly outside (should be inside but can't due to Verus limitations). `-` = not implemented.

## Phase 7: Test Coverage

| # | Test File | Tests | Coverage |
|---|---|---|---|
| 1 | TestMatrixChainStEph.rs | 17 | empty, single, 2/3/4 matrices, mutation, memo, iteration, display, macros |
| 2 | TestMatrixChainStPer.rs | — | Structural parity with StEph minus mutation tests |
| 3 | TestMatrixChainMtEph.rs | 21 | Same as StEph plus parallel execution, thread safety, large chain |
| 4 | TestMatrixChainMtPer.rs | — | Structural parity with MtEph |
| 5 | TestOptBinSearchTreeStEph.rs | 18 | empty, 1/2/3/4 keys, mutation, memo, iteration, display, macros |
| 6 | TestOptBinSearchTreeStPer.rs | — | Structural parity with StEph minus mutation tests |
| 7 | TestOBSTMtEph.rs | — | Structural parity with MtPer |
| 8 | TestOBSTMtPer.rs | 14 | empty, 1-5 keys, skewed probs, from_keys_probs, memo, display, equality, iteration |
| 9 | TestProbability.rs | 21 | new, zero, infinity, arithmetic, eq, ord, clone, display, debug, hash, conversions, macro |

**Correctness validation highlights:**
- Matrix chain 2 matrices (10×20, 20×30): expected 6000 ✅
- Matrix chain 3 matrices: expected 18000 ✅
- Matrix chain 4 matrices (classic): expected 1750 ✅
- OBST single key p=0.5: expected 0.5 ✅
- OBST two equal keys: expected 1.5 ✅
- OBST three keys (0.25, 0.5, 0.25): expected 1.5 ✅

## Phase 8: Action Items

| # | Priority | Item | Effort | Notes |
|---|---|---|---|---|
| 1 | Low | Move simple accessors inside verus! with specs | High | Blocked by HashMap and &mut limitations; would need alternative data structures |
| 2 | Low | Add spec functions for OBST cost formula | Medium | `spec fn spec_obst_cost(keys: Seq<(T, f64)>) -> f64` |
| 3 | Low | Add spec functions for MatrixChain cost formula | Medium | `spec fn spec_mc_cost(dims: Seq<(usize, usize)>) -> usize` |
| 4 | Info | Consider bottom-up DP for better parallel span | Medium | Would achieve textbook Θ(n log n) span but requires 2D array, not HashMap |
| 5 | Info | Exercise 50.5 (return optimal tree structure) not implemented | Low | Prose mentions it as exercise; optional |
| 6 | Info | MatrixChain not formally specified in prose | None | Prose mentions it as "similar problem"; implementation follows standard DP |
| 7 | Info | MatrixChainStPer macro is outside module closure | Trivial | Inconsistent with StEph which has macro inside |
| 8 | Info | Probability type duplicates f64 ordering logic | None | Necessary for HashMap key usage and Ord trait |

## Summary

Chapter 50 implements two interval-DP algorithms (OBST and Matrix Chain) across the standard 4-variant matrix (St/Mt × Eph/Per), plus a Probability infrastructure type. The OBST implementation faithfully follows Algorithm 50.3 from the textbook. Matrix Chain follows the standard DP formulation mentioned in the prose as a "similar problem."

**Verification status:** All code lives outside `verus!` due to Verus limitations with HashMap and mutable references. The `verus!` blocks are empty shells. There are 0 proof holes — trivially, because there are no proofs. This chapter is **functionally correct** (validated by 91+ runtime tests) but **unverified** by Verus.

**Cost conformance:** Work Θ(n³) matches the textbook for both algorithms. The Mt implementations achieve Span Θ(n² lg n) via parallel min reduction, which is worse than the textbook's optimal Θ(n log n) due to the sequential inner loop in the memoized recursive approach.
