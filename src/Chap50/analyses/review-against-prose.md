<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 50: Optimal Binary Search Trees — Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6

## Phase 2: Prose Inventory

Source: `prompts/Chap50.txt`

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 50.1 | Optimal Binary Search Tree (OBST) Problem: given ordered set of keys S and probability function p : S → [0:1], minimize Σ d(s,T)·p(s) over all BSTs T on S |
| 2 | Trees(S) | Set of all BSTs on keys S |
| 3 | d(s, T) | Depth of key s in tree T (root has depth 1) |
| 4 | Cost(T) | Expected cost: Σ d(s,T)·p(s) for s∈T |
| 5 | S_{i,j} | Contiguous subsequence of sorted keys from rank i to rank j (inclusive) |
| 6 | Optimal Substructure Property | Each subtree of an optimal BST is itself an optimal BST |

### Algorithms

| # | Item | Description | Implemented? |
|---|------|-------------|:------------:|
| 1 | Algorithm 50.2 | Recursive OBST — tries all roots r in S, recurses on prefix S_{1,r−1} and suffix S_{r+1,|S|} | Yes (via Alg 50.3) |
| 2 | Algorithm 50.3 | Recursive OBST (indexed) — uses offset i and length l: OBST'(i,l) = if l=0 then 0 else Σp(S[i+k]) + min_k(OBST'(i,k) + OBST'(i+k+1,l−k−1)) | Yes |
| 3 | Matrix Chain Multiplication | "Similar problem" with identical DP structure and cost bounds | Yes (full implementation) |

### Cost Specifications

| # | Item | Work | Span |
|---|------|------|------|
| 1 | Number of subproblems | n(n+1)/2 = O(n²) | — |
| 2 | Per-vertex cost | O(n) work (enumerate roots + sum probabilities) | O(log n) span (parallel min reduction) |
| 3 | Total OBST | O(n³) | O(n log n) |
| 4 | Longest DAG path | O(n) (each call removes one key) | — |
| 5 | Matrix Chain | O(n³) (same structure as OBST) | O(n log n) (same structure) |

### Exercises

| # | Item | Description | Implemented? |
|---|------|-------------|:------------:|
| 1 | Exercise 50.1 | Find another tree with equal cost to Example 50.2 | No (text exercise) |
| 2 | Exercise 50.2 | Recurrence for number of distinct BSTs with n keys | No (text exercise) |
| 3 | Exercise 50.3 | Can greedy solve OBST? (Answer: No — highest-probability key as root doesn't guarantee optimality) | No (text exercise) |
| 4 | Exercise 50.4 | Would naive cost computation (sum subtree costs + p(S_r)) work? (Answer: No — misses depth adjustment) | No (text exercise) |
| 5 | Exercise 50.5 | Return the optimal tree in addition to the cost | No |

### Prose-to-Code Cost Decomposition

The prose derives the cost recurrence by expanding the expected cost:

```
Cost(T) = Σ_{s∈T} p(s) + Cost(T_L) + Cost(T_R)
```

This decomposition is the key insight: the cost of a tree equals the total probability of its keys (accounting for the +1 depth from being below the root) plus the costs of the left and right subtrees. The code faithfully implements this as `prob_sum + min_cost` where `prob_sum = Σ p(S[i+k])` for k∈[0,l) and `min_cost = min_k(OBST'(i,k) + OBST'(i+k+1,l-k-1))`.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

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

**"Partial" explanation:** The Mt annotations claim O(n log n) span, matching APAS. However, the actual implementations achieve only O(n²) span because the subproblem evaluations at each vertex are sequential (`.map().collect()`), not parallel. Only the final min reduction is parallelized. The O(n) subproblems per vertex are evaluated sequentially, so per-vertex span is O(n) (dominated by sequential subproblem evaluation), not O(log n). Over O(n) DAG levels, actual span is O(n²). To achieve O(n log n), the subproblem evaluations themselves would need parallel tabulation.

### 3b. Implementation Fidelity

| # | Prose Item | Code | Fidelity | Notes |
|---|------------|------|:--------:|-------|
| 1 | Algorithm 50.3 (indexed OBST) | `obst_rec(i, l)` in all 4 OBST files | High | Faithful to the indexed formulation: offset i, length l, base case l=0, sum probabilities + min over roots |
| 2 | Cost decomposition: Cost(T) = Σp(s) + Cost(T_L) + Cost(T_R) | `prob_sum + min_cost` | High | Directly implements the derived recurrence from the prose |
| 3 | Matrix Chain Multiplication | `matrix_chain_rec(i, j)` in all 4 MC files | High | Standard (i,j) inclusive-index formulation; tries all split points; memoized |
| 4 | Memoization (DAG sharing) | HashMap-based memo in all files | Good | Prose describes n(n+1)/2 subproblems with O(n²) DAG vertices; HashMap achieves this |
| 5 | Parallel reduction for min | `parallel_min_reduction` in Mt files | Partial | Only the min reduction is parallelized; subproblem evaluations remain sequential |
| 6 | Depth convention: root = 1 | Implicit in cost formula | High | Single key returns p(k) (depth 1 × probability), consistent with root-at-depth-1 |

**Deviations from prose:**

| # | Deviation | Impact | Severity |
|---|-----------|--------|:--------:|
| 1 | StPer `optimal_cost` clones the entire solver for memoization | Adds O(n) clone overhead per call; memo is never retained across calls, defeating memoization for repeated queries | Medium |
| 2 | Mt variants use `Arc<Mutex<HashMap>>` for memo | Introduces lock contention; prose assumes ideal parallel access | Low |
| 3 | Probability sum recomputed from scratch each time | Prose implies this is O(n) per vertex, which is correct asymptotically; prefix sums could reduce constant factor | Low |
| 4 | Exercise 50.5 (return optimal tree structure) not implemented | Only cost is returned; the tree itself is not reconstructable | Medium |
| 5 | Mt comments say "Compute costs for each possible root in parallel" but code is sequential `.map().collect()` | Misleading documentation | Low |
| 6 | MtPer `optimal_cost` clears the shared memo then recomputes | Concurrent calls clear each other's partial memo entries (race on clear+compute), causing redundant work | Medium |

### 3c. Spec Fidelity

**Not applicable.** No `requires`/`ensures` exist on any function. The entire chapter lacks Verus verification. All 104 functions have no Verus specification.

### 3d. Structural Observations

| # | Observation | Details |
|---|-------------|---------|
| 1 | `KeyProb<T>` defined 4 times | Separate definitions in each OBST file with different trait bounds (StT vs MtVal). Could be a single generic definition. |
| 2 | `MatrixDim` defined 4 times | Separate definitions in each MatrixChain file. All identical. |
| 3 | Per-variant traits with no shared abstraction | OBSTStEphTrait, OBSTStPerTrait, OBSTMtEphTrait, OBSTMtPerTrait are separate traits with largely identical method signatures. No unifying trait. |
| 4 | `ProbabilityTrait` declared but unused | Lines 18-26 of Probability.rs define a trait that nothing implements via the trait path. Dead code. |
| 5 | Module header style | All files use `//!` for copyright line. Per project standard, copyright should be `//` (regular comment), with only module description using `//!`. |

## Phase 4: Parallelism Review

### 4a. Mt Function Classification

| # | File | Function | Classification | Mechanism |
|---|------|----------|:-------------:|-----------|
| 1 | OptBinSearchTreeMtEph | `obst_rec` | Sequential | `.map().collect()` for subproblems |
| 2 | OptBinSearchTreeMtEph | `parallel_min_reduction` | Parallel | `thread::spawn` for left/right halves |
| 3 | OptBinSearchTreeMtEph | `optimal_cost` | Delegating | Calls `obst_rec` |
| 4 | OptBinSearchTreeMtPer | `obst_rec` | Sequential | `.map().collect()` for subproblems |
| 5 | OptBinSearchTreeMtPer | `parallel_min_reduction` | Parallel | `thread::spawn` for left/right halves |
| 6 | OptBinSearchTreeMtPer | `optimal_cost` | Delegating | Calls `obst_rec` |
| 7 | MatrixChainMtEph | `matrix_chain_rec` | Sequential | `.map().collect()` for split costs |
| 8 | MatrixChainMtEph | `parallel_min_reduction` | Parallel | `thread::spawn` for left/right halves |
| 9 | MatrixChainMtEph | `optimal_cost` | Delegating | Calls `matrix_chain_rec` |
| 10 | MatrixChainMtPer | `matrix_chain_rec` | Sequential | `.map().collect()` for split costs |
| 11 | MatrixChainMtPer | `parallel_min_reduction` | Parallel | `thread::spawn` for left/right halves |
| 12 | MatrixChainMtPer | `optimal_cost` | Delegating | Calls `matrix_chain_rec` |

### 4b. Span Audit

The prose specifies O(n log n) overall span, derived from O(n) DAG levels × O(log n) per-vertex span (parallel min reduction). The per-vertex span of O(log n) assumes the O(n) subproblem evaluations at each vertex run in parallel, with only the reduction being sequential over O(log n) levels.

In the implementation, however:

1. **Subproblem evaluation**: `(0..l).map(|k| { obst_rec(...) }).collect()` — this is a sequential `.map()` over l elements. Each element involves a recursive call that is O(1) amortized (memoized), but the l calls happen sequentially. Per-vertex span: O(l) = O(n).
2. **Min reduction**: `parallel_min_reduction(costs)` — truly parallel, O(log n) span.
3. **Combined per-vertex**: O(n) + O(log n) = O(n), dominated by sequential subproblem evaluation.
4. **Overall**: O(n) levels × O(n) per level = O(n²), not O(n log n).

### 4c. Parallelism Gap Table

| # | Function | APAS Span | Actual Span | Parallel? | Root Cause |
|---|----------|-----------|-------------|:---------:|------------|
| 1 | `obst_rec` (MtEph) | Θ(n log n) | O(n²) | Partial | Sequential `.map().collect()` for subproblems |
| 2 | `obst_rec` (MtPer) | Θ(n log n) | O(n²) | Partial | Sequential `.map().collect()` for subproblems |
| 3 | `matrix_chain_rec` (MtEph) | Θ(n log n) | O(n²) | Partial | Sequential `.map().collect()` for split costs |
| 4 | `matrix_chain_rec` (MtPer) | Θ(n log n) | O(n²) | Partial | Sequential `.map().collect()` for split costs |
| 5 | `parallel_min_reduction` (all Mt) | Θ(log n) | O(log n) | Yes | Correctly parallel via thread::spawn |

### 4d. Thread Spawning Analysis

The `parallel_min_reduction` functions spawn O(n) threads per invocation (binary recursion tree over n costs). This is called once per non-base-case vertex, so over all O(n²) vertices the total thread spawn count is O(n³). Each thread is a full OS thread (`thread::spawn`), not a lightweight task. For large n, this will exhaust system thread limits or cause severe overhead. A thread pool or work-stealing scheduler would be more appropriate.

### 4e. Mt Concurrency Correctness

| # | Issue | Severity | Details |
|---|-------|:--------:|---------|
| 1 | MtPer memo race on `optimal_cost` | Medium | `optimal_cost` clears the mutex-protected memo then recomputes. Concurrent calls from different threads (as tested in `test_concurrent_reads`) can clear each other's partial results. Correctness is preserved (deterministic subproblems) but efficiency degrades. |
| 2 | MtEph `set_key_prob` / `update_prob` non-atomic | Low | Keys and memo are locked separately: keys are updated, then memo is cleared in a separate lock acquisition. A concurrent `obst_rec` between the two locks could read new keys but old memo entries. |
| 3 | Excessive cloning in `parallel_min_reduction` | Low | Both MtEph and MtPer `parallel_min_reduction` clone the entire solver struct (including `Arc<Mutex<HashMap>>`) to move into spawned threads. The Arc clone is O(1) but contributes to reference count contention. |

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
- **Happy path**: Optimal cost computation for 1-8 keys/matrices with known answers
- **Edge cases**: Empty input, single element, uniform probabilities, skewed probabilities
- **Constructors**: `new`, `from_keys_probs`, `from_key_probs`, `from_dimensions`, `from_dim_pairs`
- **Accessors**: keys, dimensions, num_keys, num_matrices, memo_size
- **Mutators** (Eph): set_key_prob, update_prob, set_dimension, update_dimension, clear_memo
- **Traits**: Display, Clone, PartialEq/Eq, IntoIterator (owned, &, &mut)
- **Macros**: OBSTStEphLit, OBSTStPerLit, OBSTMtEphLit, OBSTMtPerLit, MatrixChainStEphLit, MatrixChainStPerLit, MatrixChainMtEphLit, MatrixChainMtPerLit
- **Parallelism**: Mt tests verify concurrent read access via Arc + thread::spawn
- **Persistence**: StPer/MtPer tests verify immutability semantics

### 5c. Test Correctness Audit

| # | Test | File | Verdict | Notes |
|---|------|------|:-------:|-------|
| 1 | `test_two_keys` | TestOptBinSearchTreeStEph | Correct | For keys A(0.4), B(0.6): optimal is B at root (cost 0.6 + 0.4×2 = 1.4). Code asserts `prob_sum(0.4+0.6) + min(left+right)` which equals `1.0 + 0.4 = 1.4`. |
| 2 | `test_obst_three_keys` | TestOBSTMtPer | Correct | For probs (0.25, 0.5, 0.25) with B at root: cost = 0.25×2 + 0.5×1 + 0.25×2 = 1.5. |
| 3 | `test_optimal_cost_two_matrices` | TestMatrixChainStEph | Correct | 10×20 · 20×30 = 6000 scalar multiplications. |
| 4 | `test_optimal_cost_three_matrices` | TestMatrixChainStEph | Correct | (A×B)×C = 10·20·30 + 10·30·40 = 18000; A×(B×C) = 20·30·40 + 10·20·40 = 32000. Optimal = 18000. |
| 5 | `test_optimal_cost_four_matrices` | TestMatrixChainStEph | Correct | Dimensions (10×100, 100×5, 5×50, 50×1): optimal = 1750. Verified by hand: ((A×B)×C)×D = 5000+2500+500 = 8000; A×((B×C)×D) = 25000+250+10000 = 35250; (A×B)×(C×D) = 5000+250+500 = 5750; A×(B×(C×D)) = 250+500+1000 = 1750. |

### 5d. Missing Tests

| # | Gap | Priority | Notes |
|---|-----|:--------:|-------|
| 1 | Example 50.2 validation | Medium | Textbook specifies keys k1-k6 with probs (1/8, 1/32, 1/16, 1/32, 1/4, 1/2) and optimal cost 31/16 = 1.9375. No test validates this. |
| 2 | Cross-variant consistency | Medium | No test verifies that all 4 OBST variants produce the same optimal cost for the same input. Same for MatrixChain. |
| 3 | MtPer concurrent `optimal_cost` correctness | Low | `test_concurrent_reads` calls `optimal_cost` from 4 threads but only asserts cost > 0 — doesn't verify it equals the known-correct value. |
| 4 | Probability edge: +0.0 vs -0.0 | Low | Bit-level equality in Probability means `Probability(0.0) != Probability(-0.0)`, which could cause subtle memoization misses. No test covers this. |
| 5 | Probability: negative values via Sub | Low | `Probability::new(0.1) - Probability::new(0.5)` produces a negative probability. The debug_assert only fires in constructor. |
| 6 | OBSTMtPer missing `OBSTMtPerLit` macro test for empty | Low | Other variants test empty macro construction; MtPer's test file has `OBSTMtPerLit!()` for empty but only in `test_obst_empty`. |

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs exist and none are needed.** The chapter has no `verus!` blocks, no iterators with ghost state, no verified loops, and no Verus-specific constructs. PTTs would be meaningless without Verus verification.

### 6a. Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|:------:|
| 1 | Probability.rs | TestProbability.rs (24) | — | RTT only |
| 2 | OptBinSearchTreeStEph.rs | TestOptBinSearchTreeStEph.rs (20) | — | RTT only |
| 3 | OptBinSearchTreeStPer.rs | TestOptBinSearchTreeStPer.rs (12) | — | RTT only |
| 4 | OptBinSearchTreeMtEph.rs | TestOBSTMtEph.rs (25) | — | RTT only |
| 5 | OptBinSearchTreeMtPer.rs | TestOBSTMtPer.rs (14) | — | RTT only |
| 6 | MatrixChainStEph.rs | TestMatrixChainStEph.rs (18) | — | RTT only |
| 7 | MatrixChainStPer.rs | TestMatrixChainStPer.rs (14) | — | RTT only |
| 8 | MatrixChainMtEph.rs | TestMatrixChainMtEph.rs (22) | — | RTT only |
| 9 | MatrixChainMtPer.rs | TestMatrixChainMtPer.rs (20) | — | RTT only |

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Item | Status | Notes |
|---|------|:------:|-------|
| 1 | Exercise 50.5 | Not implemented | Return the optimal tree structure, not just the cost. Would require tracking the optimal root at each subproblem. |
| 2 | Example 50.2 validation | Not implemented | Textbook gives k1-k6 with specific probabilities; optimal cost = 31/16 |
| 3 | Brute force OBST (O(4^n)) | Not implemented | Prose mentions for motivation; not worth implementing |
| 4 | Greedy approach (Exercise 50.3) | Not implemented | Prose proves greedy doesn't work; text exercise only |
| 5 | Prefix-sum optimization for probability summation | Not implemented | Prose doesn't explicitly call for it but O(n²) precomputation would reduce per-vertex probability sum from O(n) to O(1) |

### Code with No Prose Counterpart

| # | Item | Justification |
|---|------|---------------|
| 1 | Probability.rs | Infrastructure: wraps f64 to satisfy Eq/Ord/Hash for HashMap keys and MtVal trait |
| 2 | MatrixChain*.rs (full implementations) | Mentioned in prose as "similar problem" solved "in a very similar structure" — the implementation is a reasonable extrapolation |
| 3 | KeyProb struct | Data structure pairing keys with probabilities; mechanical scaffolding |
| 4 | MatrixDim struct | Data structure for matrix dimensions; mechanical scaffolding |
| 5 | Convenience macros (OBSTStEphLit, etc.) | Testing/usage ergonomics |
| 6 | IntoIterator impls | Rust collection ergonomics |
| 7 | ProbabilityTrait (unused) | Dead code; declared but never implemented via the trait |

### Algorithmic Completeness Assessment

The prose describes the OBST problem, derives the cost recurrence, presents two algorithm formulations (50.2 and 50.3), analyzes cost bounds, and mentions Matrix Chain as a similar problem. The implementation covers:

| # | Prose Element | Coverage | Notes |
|---|--------------|:--------:|-------|
| 1 | Problem definition (Def 50.1) | Full | Keys + probability → minimize expected cost |
| 2 | Cost recurrence derivation | Full | `prob_sum + min_cost` directly implements Cost(T) = Σp(s) + Cost(T_L) + Cost(T_R) |
| 3 | Algorithm 50.3 (indexed) | Full | `obst_rec(i, l)` matches OBST'(i, l) |
| 4 | Memoization/sharing | Full | HashMap with O(n²) entries |
| 5 | O(n³) work | Full | Correctly achieved in all variants |
| 6 | O(n log n) span | Partial | Only min reduction is parallel; subproblem evaluation is sequential |
| 7 | Matrix Chain as similar problem | Full+ | Fully implemented as separate data structure (goes beyond prose which only mentions it) |
| 8 | Return optimal tree (Ex 50.5) | None | Only cost is returned |

## Phase 8: Table of Contents / Style Review

### TOC Presence

No file has a Table of Contents block. Since no file uses `verus!`, the standard 13-section TOC does not apply. Files would benefit from lightweight section organization when verusified.

| # | File | TOC? | verus!? | Lines |
|---|------|:----:|:-------:|------:|
| 1 | Probability.rs | No | No | 147 |
| 2 | OptBinSearchTreeStEph.rs | No | No | 221 |
| 3 | OptBinSearchTreeStPer.rs | No | No | 183 |
| 4 | OptBinSearchTreeMtEph.rs | No | No | 313 |
| 5 | OptBinSearchTreeMtPer.rs | No | No | 244 |
| 6 | MatrixChainStEph.rs | No | No | 221 |
| 7 | MatrixChainStPer.rs | No | No | 183 |
| 8 | MatrixChainMtEph.rs | No | No | 307 |
| 9 | MatrixChainMtPer.rs | No | No | 241 |

### In/Out Table

Not applicable — no `verus!` blocks exist. All trait impls are outside `verus!` by default.

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|
| 1 | Probability.rs | - (Copy) | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out |
| 2 | OptBinSearchTreeStEph.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 3 | OptBinSearchTreeStPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 4 | OptBinSearchTreeMtEph.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 5 | OptBinSearchTreeMtPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 6 | MatrixChainStEph.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 7 | MatrixChainStPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 8 | MatrixChainMtEph.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 9 | MatrixChainMtPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |

All placements are currently correct for plain Rust. When verusified, Clone/PartialEq/Eq should move inside `verus!` with specs per project standard.

### Module Header Audit

| # | File | Copyright format | Correct? |
|---|------|:---------------:|:--------:|
| 1 | Probability.rs | `//!` | No — should be `//` |
| 2 | OptBinSearchTreeStEph.rs | `//!` | No — should be `//` |
| 3 | OptBinSearchTreeStPer.rs | `//!` | No — should be `//` |
| 4 | OptBinSearchTreeMtEph.rs | `//!` | No — should be `//` |
| 5 | OptBinSearchTreeMtPer.rs | `//!` | No — should be `//` |
| 6 | MatrixChainStEph.rs | `//!` | No — should be `//` |
| 7 | MatrixChainStPer.rs | `//!` | No — should be `//` |
| 8 | MatrixChainMtEph.rs | `//!` | No — should be `//` |
| 9 | MatrixChainMtPer.rs | `//!` | No — should be `//` |

Per the module-header rule, the copyright line should use `//` (regular comment), not `//!` (doc comment). Only the module description should use `//!`.

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

No proof holes — trivially true since no `verus!` blocks exist. The absence of holes reflects the absence of verification, not the completeness of proofs.

## Review TODOs

| # | Priority | Category | Description |
|---|:--------:|----------|-------------|
| 1 | High | Verification | Verusify the St variants (StEph first, then StPer) — sequential DP is most amenable to verification. Core specs: `obst_rec` ensures cost equals minimum over all root choices; `optimal_cost` ensures result equals OBST cost for the full sequence. |
| 2 | High | Parallelism | Parallelize subproblem evaluation in Mt variants to achieve APAS O(n log n) span. Replace sequential `.map().collect()` with parallel tabulation or `join()`. |
| 3 | Medium | Correctness | Add test for Example 50.2 (k1-k6 with probs 1/8, 1/32, 1/16, 1/32, 1/4, 1/2; optimal cost = 31/16 = 1.9375) to validate against the textbook. |
| 4 | Medium | Correctness | Add cross-variant consistency test: all 4 OBST variants should produce identical costs for the same input. Same for MatrixChain. |
| 5 | Medium | Feature | Implement Exercise 50.5: return the optimal tree structure in addition to the cost. Track the optimal root at each (i, l) subproblem. |
| 6 | Medium | Performance | Replace O(n³) thread spawns in `parallel_min_reduction` with a thread pool or bounded-depth recursion. Current approach spawns one OS thread per element in the binary recursion tree. |
| 7 | Medium | Design | Deduplicate `KeyProb` and `MatrixDim` structs — define once with appropriate trait bounds, import in each variant file. |
| 8 | Low | Design | Remove unused `ProbabilityTrait` from Probability.rs (dead code). |
| 9 | Low | Correctness | Fix Probability `From<f64>` to go through `new()` so the `debug_assert!(value >= 0.0)` check applies. Currently `From<f64>` bypasses the assertion. |
| 10 | Low | Style | Fix copyright lines: change `//!` to `//` in all 9 source files per module-header standard. |
| 11 | Low | Style | Add TOC headers when verusifying. |
| 12 | Low | Optimization | Add prefix-sum precomputation for probability summation to reduce per-vertex constant factor. |
| 13 | Low | Correctness | Probability bit-level equality: `Probability(0.0) != Probability(-0.0)`. Document this as intended behavior or normalize in constructor. |
| 14 | Low | Mt Semantics | Document the MtPer `optimal_cost` memo race: concurrent calls clear each other's partial results. Correct but suboptimal. |

## Overall Assessment

### Strengths

1. **Faithful algorithm implementation.** Both OBST and Matrix Chain faithfully implement the textbook's indexed recursive formulation (Algorithm 50.3) with correct cost decomposition (Cost(T) = Σp(s) + Cost(T_L) + Cost(T_R)).
2. **Complete variant coverage.** All four variants (StEph, StPer, MtEph, MtPer) are implemented for both algorithms, plus the Probability wrapper type — 9 source files total.
3. **Thorough runtime testing.** 169 tests across 9 files cover constructors, accessors, mutators, edge cases, trait impls, macros, parallelism, and persistence semantics. Several tests verify exact optimal costs against hand-computed answers.
4. **Correct memoization.** HashMap-based memoization correctly implements the DAG sharing described in the prose, with O(n²) subproblems cached.
5. **Matrix Chain goes beyond prose.** The textbook only mentions Matrix Chain as a "similar problem"; the implementation provides a complete, tested, four-variant data structure.

### Weaknesses

1. **No Verus verification.** The chapter is entirely unverified — no `verus!` blocks, no specs, no proofs. All 104 functions have spec strength "none". This is the most significant gap.
2. **Mt parallelism is shallow.** The Mt variants only parallelize the final min reduction. The O(n) subproblem evaluations per vertex run sequentially (`.map().collect()`), so actual span is O(n²), not the APAS-intended O(n log n).
3. **Excessive thread spawning.** `parallel_min_reduction` recursively spawns O(n) OS threads per invocation, totaling O(n³) thread creates across the algorithm. No thread pool or granularity cutoff.
4. **StPer clones for memoization.** The persistent variants clone the entire solver for each `optimal_cost` call, preventing memo reuse across queries and adding O(n) overhead.
5. **Structural duplication.** `KeyProb` is defined 4 times, `MatrixDim` 4 times, and per-variant traits share nearly identical signatures with no unifying abstraction.
6. **No Exercise 50.5.** Only cost is returned; the optimal tree structure cannot be reconstructed.
7. **Probability type hazards.** `From<f64>` bypasses the non-negativity assertion; `Sub` can produce negative probabilities; bit-level equality treats +0.0 and -0.0 as different values.
