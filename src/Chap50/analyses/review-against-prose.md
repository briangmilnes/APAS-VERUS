<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 50: Optimal Binary Search Trees — Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory (Tool-Generated)

104 functions extracted across 9 source files.

| # | File | Functions | V! | -V! | NoSpec |
|---|------|----------:|:--:|:---:|:------:|
| 1 | MatrixChainMtEph.rs | 14 | 0 | 14 | 14 |
| 2 | MatrixChainMtPer.rs | 11 | 0 | 11 | 11 |
| 3 | MatrixChainStEph.rs | 13 | 0 | 13 | 13 |
| 4 | MatrixChainStPer.rs | 9 | 0 | 9 | 9 |
| 5 | OptBinSearchTreeMtEph.rs | 13 | 0 | 13 | 13 |
| 6 | OptBinSearchTreeMtPer.rs | 10 | 0 | 10 | 10 |
| 7 | OptBinSearchTreeStEph.rs | 12 | 0 | 12 | 12 |
| 8 | OptBinSearchTreeStPer.rs | 8 | 0 | 8 | 8 |
| 9 | Probability.rs | 14 | 0 | 14 | 14 |

**Key observation:** No file contains a `verus!` block. The entire chapter is unverified Rust — no formal specifications, no proof obligations, no ghost code.

## Phase 2: Prose Inventory

Source: `prompts/Chap50.txt`

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 50.1 | Optimal Binary Search Tree (OBST) Problem: given ordered keys S and probability function p, minimize expected search cost over all BSTs on S |
| 2 | Trees(S) | Set of all BSTs on keys S |
| 3 | d(s, T) | Depth of key s in tree T (root has depth 1) |
| 4 | Cost(T) | Expected cost: Σ d(s,T)·p(s) for s∈S |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Algorithm 50.2 | Recursive Optimal Binary Search Tree — tries all roots, recurses on prefix/suffix |
| 2 | Algorithm 50.3 | Recursive OBST (indexed) — uses offset i and length l to identify subproblems |
| 3 | Matrix Chain Multiplication | Mentioned as "similar problem" with identical cost structure |

### Cost Specs

| # | Item | Work | Span |
|---|------|------|------|
| 1 | OBST subproblems | n(n+1)/2 = O(n²) | — |
| 2 | Per-vertex cost | O(n) work | O(log n) span (parallel reduction) |
| 3 | Total OBST | O(n³) | O(n log n) |
| 4 | Longest DAG path | O(n) | — |
| 5 | Matrix Chain | O(n³) (same as OBST) | O(n log n) (same as OBST) |

### Exercises

| # | Item | Description | Implemented? |
|---|------|-------------|:------------:|
| 1 | Exercise 50.1 | Find another tree with equal cost | No (text exercise) |
| 2 | Exercise 50.2 | Recurrence for number of distinct BSTs | No (text exercise) |
| 3 | Exercise 50.3 | Can greedy solve OBST? (No) | No (text exercise) |
| 4 | Exercise 50.4 | Would naive cost computation work? (No) | No (text exercise) |
| 5 | Exercise 50.5 | Return optimal tree in addition to cost | No |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

Cost annotations updated to APAS/Claude-Opus-4.6 pair format on all key algorithmic functions:

| # | File | Function | APAS Cost | Claude-Opus-4.6 Cost | Match? |
|---|------|----------|-----------|---------------------|:------:|
| 1 | OptBinSearchTreeStEph | `optimal_cost` | Work Θ(n³), Span Θ(n²) | Work O(n³), Span O(n²) | Yes |
| 2 | OptBinSearchTreeStEph | `obst_rec` | Work Θ(n³), Span Θ(n²) | Work O(n³), Span O(n²) | Yes |
| 3 | OptBinSearchTreeStPer | `optimal_cost` | Work Θ(n³), Span Θ(n²) | Work O(n³), Span O(n²) | Yes |
| 4 | OptBinSearchTreeStPer | `obst_rec` | Work Θ(n³), Span Θ(n²) | Work O(n³), Span O(n²) | Yes |
| 5 | OptBinSearchTreeMtEph | `optimal_cost` | Work Θ(n³), Span Θ(n log n) | Work O(n³), Span O(n log n) | Partial |
| 6 | OptBinSearchTreeMtEph | `obst_rec` | Work Θ(n³), Span Θ(n log n) | Work O(n³), Span O(n log n) | Partial |
| 7 | OptBinSearchTreeMtEph | `parallel_min_reduction` | Work Θ(n), Span Θ(log n) | Work O(n), Span O(log n) | Yes |
| 8 | OptBinSearchTreeMtPer | `optimal_cost` | Work Θ(n³), Span Θ(n log n) | Work O(n³), Span O(n log n) | Partial |
| 9 | OptBinSearchTreeMtPer | `obst_rec` | Work Θ(n³), Span Θ(n log n) | Work O(n³), Span O(n log n) | Partial |
| 10 | OptBinSearchTreeMtPer | `parallel_min_reduction` | Work Θ(n), Span Θ(log n) | Work O(n), Span O(log n) | Yes |
| 11 | MatrixChainStEph | `optimal_cost` | Work Θ(n³), Span Θ(n²) | Work O(n³), Span O(n²) | Yes |
| 12 | MatrixChainStEph | `matrix_chain_rec` | Work Θ(n³), Span Θ(n²) | Work O(n³), Span O(n²) | Yes |
| 13 | MatrixChainStPer | `optimal_cost` | Work Θ(n³), Span Θ(n²) | Work O(n³), Span O(n²) | Yes |
| 14 | MatrixChainStPer | `matrix_chain_rec` | Work Θ(n³), Span Θ(n²) | Work O(n³), Span O(n²) | Yes |
| 15 | MatrixChainMtEph | `optimal_cost` | Work Θ(n³), Span Θ(n log n) | Work O(n³), Span O(n log n) | Partial |
| 16 | MatrixChainMtEph | `matrix_chain_rec` | Work Θ(n³), Span Θ(n log n) | Work O(n³), Span O(n log n) | Partial |
| 17 | MatrixChainMtEph | `parallel_min_reduction` | Work Θ(n), Span Θ(log n) | Work O(n), Span O(log n) | Yes |
| 18 | MatrixChainMtPer | `optimal_cost` | Work Θ(n³), Span Θ(n log n) | Work O(n³), Span O(n log n) | Partial |
| 19 | MatrixChainMtPer | `matrix_chain_rec` | Work Θ(n³), Span Θ(n log n) | Work O(n³), Span O(n log n) | Partial |
| 20 | MatrixChainMtPer | `parallel_min_reduction` | Work Θ(n), Span Θ(log n) | Work O(n), Span O(log n) | Yes |

**"Partial" explanation:** The Mt variants annotate Span as O(n log n), matching APAS. The `parallel_min_reduction` achieves O(log n) span per level. However, the recursive subproblem computation (`obst_rec`/`matrix_chain_rec`) is computed sequentially via `.map().collect()` before the reduction. The O(n) subproblems per vertex are evaluated sequentially, so the actual per-vertex span is O(n) (dominated by sequential subproblem evaluation), not O(log n). The overall span is therefore O(n²) in practice, not O(n log n). The O(n log n) annotation reflects what APAS intends if the subproblem evaluations were also parallel, but the implementation only parallelizes the final min reduction.

### 3b. Implementation Fidelity

| # | Prose Item | Code | Fidelity | Notes |
|---|------------|------|:--------:|-------|
| 1 | Algorithm 50.3 (indexed OBST) | `obst_rec(i, l)` in all 4 OBST files | High | Faithful to the indexed formulation: offset i, length l, base case l=0, sum probabilities + min over roots |
| 2 | Matrix Chain Multiplication | `matrix_chain_rec(i, j)` in all 4 MC files | High | Standard (i,j) index formulation; tries all split points; memoized |
| 3 | Memoization (sharing) | HashMap-based memo in all files | Good | Prose describes DAG sharing; implementation uses HashMap memoization, which is equivalent |
| 4 | Parallel reduction for min | `parallel_min_reduction` in Mt files | Partial | Only the min reduction is parallelized; the subproblem evaluations that produce the costs are sequential (see Phase 4) |

**Deviations:**
1. The StPer variants clone `self` to get a mutable solver for memoization, adding O(n) overhead per `optimal_cost` call. The prose does not mention this.
2. The Mt variants use `Arc<Mutex<HashMap>>` for memo, introducing lock contention. The prose assumes ideal parallel access.
3. The probability sum is computed by iterating over the subsequence each time rather than using prefix sums. This does not change the asymptotic cost but increases constant factors.
4. Exercise 50.5 (returning the optimal tree, not just the cost) is not implemented.

### 3c. Spec Fidelity

**Not applicable.** No `requires`/`ensures` exist on any function. The entire chapter lacks Verus verification.

## Phase 4: Parallelism Review

### 4a. Mt Function Classification

| # | File | Function | Classification | Mechanism |
|---|------|----------|:-------------:|-----------|
| 1 | OptBinSearchTreeMtEph | `obst_rec` | Sequential | Uses `.map().collect()` for subproblems (no spawning) |
| 2 | OptBinSearchTreeMtEph | `parallel_min_reduction` | Parallel | `thread::spawn` for left/right halves |
| 3 | OptBinSearchTreeMtEph | `optimal_cost` | Delegating | Calls `obst_rec` |
| 4 | OptBinSearchTreeMtPer | `obst_rec` | Sequential | Uses `.map().collect()` for subproblems |
| 5 | OptBinSearchTreeMtPer | `parallel_min_reduction` | Parallel | `thread::spawn` for left/right halves |
| 6 | OptBinSearchTreeMtPer | `optimal_cost` | Delegating | Calls `obst_rec` |
| 7 | MatrixChainMtEph | `matrix_chain_rec` | Sequential | Uses `.map().collect()` for split costs |
| 8 | MatrixChainMtEph | `parallel_min_reduction` | Parallel | `thread::spawn` for left/right halves |
| 9 | MatrixChainMtEph | `optimal_cost` | Delegating | Calls `matrix_chain_rec` |
| 10 | MatrixChainMtPer | `matrix_chain_rec` | Sequential | Uses `.map().collect()` for split costs |
| 11 | MatrixChainMtPer | `parallel_min_reduction` | Parallel | `thread::spawn` for left/right halves |
| 12 | MatrixChainMtPer | `optimal_cost` | Delegating | Calls `matrix_chain_rec` |

### 4b. Span Audit

The core recursive functions (`obst_rec`, `matrix_chain_rec`) compute all O(l) subproblems sequentially, then pass the results to `parallel_min_reduction`. The parallel min gives O(log n) span for the reduction, but the sequential subproblem computation dominates at O(n) per vertex. Over O(n) levels, the actual span is O(n²), not O(n log n).

To achieve the APAS O(n log n) span, the subproblem evaluations themselves would need to be parallelized (e.g., parallel map/tabulate), not just the final reduction.

### 4c. Parallelism Gap Table

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|:---------:|-------|
| 1 | `obst_rec` (MtEph) | Θ(n log n) | O(n²) | Partial | Only min reduction is parallel; subproblems sequential |
| 2 | `obst_rec` (MtPer) | Θ(n log n) | O(n²) | Partial | Same issue |
| 3 | `matrix_chain_rec` (MtEph) | Θ(n log n) | O(n²) | Partial | Same issue |
| 4 | `matrix_chain_rec` (MtPer) | Θ(n log n) | O(n²) | Partial | Same issue |
| 5 | `parallel_min_reduction` (all Mt) | Θ(log n) | O(log n) | Yes | Correctly parallel |

## Phase 5: Runtime Test Review

### 5a. Coverage Check

| # | Source Module | Test File | Test Count | Status |
|---|-------------|-----------|:----------:|:------:|
| 1 | Probability.rs | TestProbability.rs | 24 | Good |
| 2 | OptBinSearchTreeStEph.rs | TestOptBinSearchTreeStEph.rs | 20 | Good |
| 3 | OptBinSearchTreeStPer.rs | TestOptBinSearchTreeStPer.rs | 12 | Good |
| 4 | OptBinSearchTreeMtEph.rs | TestOBSTMtEph.rs | 25 | Good |
| 5 | OptBinSearchTreeMtPer.rs | TestOBSTMtPer.rs | 14 | Good |
| 6 | MatrixChainStEph.rs | TestMatrixChainStEph.rs | 18 | Good |
| 7 | MatrixChainStPer.rs | TestMatrixChainStPer.rs | 14 | Good |
| 8 | MatrixChainMtEph.rs | TestMatrixChainMtEph.rs | 22 | Good |
| 9 | MatrixChainMtPer.rs | TestMatrixChainMtPer.rs | 20 | Good |

**Total: 169 runtime tests across 9 files.**

### 5b. Test Quality

Tests cover:
- **Happy path**: Optimal cost computation for 1-6 keys/matrices with known answers
- **Edge cases**: Empty input, single element, uniform probabilities
- **Constructors**: `new`, `from_keys_probs`, `from_key_probs`, `from_dimensions`, `from_dim_pairs`
- **Accessors**: keys, dimensions, num_keys, num_matrices, memo_size
- **Mutators** (Eph): set_key_prob, update_prob, set_dimension, update_dimension, clear_memo
- **Traits**: Display, Clone, PartialEq/Eq, IntoIterator
- **Macros**: OBSTStEphLit, OBSTStPerLit, OBSTMtEphLit, MatrixChainStEphLit, etc.
- **Parallelism**: Mt tests verify thread execution and concurrent reads
- **Persistence**: StPer/MtPer tests verify immutability semantics

### 5c. Missing Tests

| # | Gap | Priority | Notes |
|---|-----|:--------:|-------|
| 1 | Example 50.2 from prose | Medium | The textbook gives specific keys k1-k6 with probabilities; could validate against known optimal cost 31/16 |
| 2 | Boundary: out-of-bounds set_dimension (MtEph) | Low | StEph has this test but MtEph does not |
| 3 | Hash trait on Probability | Low | Tested in TestProbability.rs |

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs are needed.** The chapter has no `verus!` blocks, no iterators with ghost state, no verified loops, and no Verus-specific constructs. PTTs would be meaningless without any Verus verification to test against.

### 6a. Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|:------:|
| 1 | Probability.rs | TestProbability.rs | — | RTT only (no verus) |
| 2 | OptBinSearchTreeStEph.rs | TestOptBinSearchTreeStEph.rs | — | RTT only |
| 3 | OptBinSearchTreeStPer.rs | TestOptBinSearchTreeStPer.rs | — | RTT only |
| 4 | OptBinSearchTreeMtEph.rs | TestOBSTMtEph.rs | — | RTT only |
| 5 | OptBinSearchTreeMtPer.rs | TestOBSTMtPer.rs | — | RTT only |
| 6 | MatrixChainStEph.rs | TestMatrixChainStEph.rs | — | RTT only |
| 7 | MatrixChainStPer.rs | TestMatrixChainStPer.rs | — | RTT only |
| 8 | MatrixChainMtEph.rs | TestMatrixChainMtEph.rs | — | RTT only |
| 9 | MatrixChainMtPer.rs | TestMatrixChainMtPer.rs | — | RTT only |

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Item | Status | Notes |
|---|------|:------:|-------|
| 1 | Exercise 50.5 | Not implemented | Return the optimal tree structure, not just the cost |
| 2 | Example 50.2 validation | Not implemented | Could verify the k1-k6 example yields cost 31/16 |
| 3 | Brute force OBST | Not implemented | Prose mentions O(4^n) brute force; not worth implementing |
| 4 | Greedy approach (Exercise 50.3) | Not implemented | Prose proves greedy doesn't work; text exercise only |

### Code with No Prose Counterpart

| # | Item | Justification |
|---|------|---------------|
| 1 | Probability.rs | Infrastructure: wraps f64 to satisfy Eq/Ord for type-class requirements |
| 2 | MatrixChain*.rs | Mentioned in prose as "similar problem" but not given its own algorithm number |
| 3 | KeyProb struct | Data structure to pair keys with probabilities; mechanical scaffolding |
| 4 | Memoization (HashMap) | Implementation detail of DAG sharing described in prose |
| 5 | Convenience macros (OBSTStEphLit, etc.) | Testing/usage scaffolding |
| 6 | IntoIterator impls | Rust collection ergonomics, not in prose |

## Phase 8: Table of Contents Review

### TOC Presence

No file has a Table of Contents block. Since no file uses `verus!`, the standard TOC (sections 1-13) does not directly apply. However, the files would benefit from a lightweight section organization.

| # | File | TOC Present? | Notes |
|---|------|:------------:|-------|
| 1 | Probability.rs | No | Plain Rust; no verus! |
| 2 | OptBinSearchTreeStEph.rs | No | Plain Rust; no verus! |
| 3 | OptBinSearchTreeStPer.rs | No | Plain Rust; no verus! |
| 4 | OptBinSearchTreeMtEph.rs | No | Plain Rust; no verus! |
| 5 | OptBinSearchTreeMtPer.rs | No | Plain Rust; no verus! |
| 6 | MatrixChainStEph.rs | No | Plain Rust; no verus! |
| 7 | MatrixChainStPer.rs | No | Plain Rust; no verus! |
| 8 | MatrixChainMtEph.rs | No | Plain Rust; no verus! |
| 9 | MatrixChainMtPer.rs | No | Plain Rust; no verus! |

### In/Out Table

Not applicable — no `verus!` blocks exist. All trait impls are outside `verus!` by default.

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | Probability.rs | - (Copy) | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | Hash, Ord, From, Add/Sub/Mul/Div |
| 2 | OptBinSearchTreeStEph.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |
| 3 | OptBinSearchTreeStPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |
| 4 | OptBinSearchTreeMtEph.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |
| 5 | OptBinSearchTreeMtPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - | - |
| 6 | MatrixChainStEph.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |
| 7 | MatrixChainStPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |
| 8 | MatrixChainMtEph.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |
| 9 | MatrixChainMtPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |

All placements are correct for plain Rust (everything outside `verus!`). When the chapter is verusified, Clone/PartialEq/Eq should move inside `verus!` with specs per the project standard.

## Proof Holes Summary

```
✓ MatrixChainMtEph.rs
✓ MatrixChainMtPer.rs
✓ MatrixChainStEph.rs
✓ MatrixChainStPer.rs
✓ OptBinSearchTreeMtEph.rs
✓ OptBinSearchTreeMtPer.rs
✓ OptBinSearchTreeStEph.rs
✓ OptBinSearchTreeStPer.rs
✓ Probability.rs

Modules: 9 clean, 0 holed
Proof Functions: 0 total
Holes Found: 0
```

No proof holes — but this is trivially true since no `verus!` blocks exist. The absence of holes reflects the absence of verification, not the completeness of proofs.

## Spec Strength Summary

| Classification | Count |
|:--------------|------:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 104 |

**All 104 functions have no Verus specification.** The entire chapter is unverified Rust.

## Overall Assessment

### Strengths
1. **Faithful algorithm implementation.** Both OBST and Matrix Chain follow the textbook's indexed recursive formulation (Algorithm 50.3) correctly.
2. **Complete variant coverage.** All four variants (StEph, StPer, MtEph, MtPer) are implemented for both algorithms, plus the Probability wrapper.
3. **Thorough runtime testing.** 169 tests across 9 files provide good coverage of functionality, edge cases, trait impls, and macros.
4. **Correct memoization.** HashMap-based memoization correctly implements the DAG sharing described in the prose.

### Weaknesses
1. **No Verus verification.** The chapter is entirely unverified — no `verus!` blocks, no specs, no proofs. This is the most significant gap.
2. **Mt parallelism is shallow.** The Mt variants only parallelize the final min reduction. The subproblem computations are sequential (`.map().collect()`), so the actual span is O(n²), not the APAS-intended O(n log n). To achieve true O(n log n) span, the subproblem evaluations would need parallel tabulation.
3. **StPer clones for memoization.** The persistent variants clone the entire solver to get a mutable copy, adding O(n) overhead and defeating persistence semantics for the internal memo.
4. **No Exercise 50.5.** The implementations only return the optimal cost, not the optimal tree structure.
5. **No TOC headers.** Files lack the standard Table of Contents block (acceptable for pre-verus code but should be added during verusification).
6. **Probability type is unverified.** The `Probability` wrapper implements `Eq` and `Ord` for `f64` with custom NaN handling, but this is not formally verified. The bit-level equality comparison and NaN ordering could harbor subtle bugs.

### Recommended Actions (Priority Order)
1. Verusify the St variants (StEph, StPer) first — the sequential DP is most amenable to verification.
2. Add prefix-sum optimization for the probability summation (reduces per-vertex work constant).
3. Parallelize subproblem evaluation in Mt variants to achieve the APAS O(n log n) span.
4. Implement Exercise 50.5 (return optimal tree, not just cost).
5. Add TOC headers when verusifying.
