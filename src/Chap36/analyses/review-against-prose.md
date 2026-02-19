<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 36 — Quicksort: Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory

All functions extracted by `veracity-review-module-fn-impls -d src/Chap36`.

| # | Module | Function | V! | Location | Spec | SpecStr | Lines |
|---|--------|----------|:--:|----------|:----:|:-------:|------:|
| 1 | QuickSortStEph | `sort_vec` | V! | ML | HasSpec | strong | 44–47 |
| 2 | QuickSortStEph | `quick_sort_first` | V! | ML | HasSpec | strong | 358–360 |
| 3 | QuickSortStEph | `quick_sort_median3` | V! | ML | HasSpec | strong | 367–369 |
| 4 | QuickSortStEph | `quick_sort_random` | V! | ML | HasSpec | strong | 376–378 |
| 5 | QuickSortMtEph | `sort_vec` | V! | ML | HasSpec | strong | 45–48 |
| 6 | QuickSortMtEph | `quick_sort_first` | V! | ML | HasSpec | strong | 359–361 |
| 7 | QuickSortMtEph | `quick_sort_median3` | V! | ML | HasSpec | strong | 368–370 |
| 8 | QuickSortMtEph | `quick_sort_random` | V! | ML | HasSpec | strong | 377–379 |
| 9 | QuickSortMtEphSlice | `pivot_mt_first` | -V! | Tr+IT | NoSpec | none | 16–18 |
| 10 | QuickSortMtEphSlice | `pivot_mt_median3` | -V! | Tr+IT | NoSpec | none | 19–21 |
| 11 | QuickSortMtEphSlice | `pivot_mt_random` | -V! | Tr+IT | NoSpec | none | 22–24 |
| 12 | QuickSortMtEphSlice | `quick_sort_mt_first` | -V! | Tr+IT | NoSpec | none | 25–27 |
| 13 | QuickSortMtEphSlice | `quick_sort_mt_median3` | -V! | Tr+IT | NoSpec | none | 28–30 |
| 14 | QuickSortMtEphSlice | `quick_sort_mt_random` | -V! | Tr+IT | NoSpec | none | 31–33 |
| 15 | QuickSortMtEphSlice | `sort` (inner fn, ×3) | -V! | IT | NoSpec | none | 64–94 |

StEph and MtEph are inside `verus!` with strong `sort_by` postconditions. MtEphSlice is entirely outside `verus!` with no specs.

## Phase 2: Prose Inventory

Source: `prompts/Chap36.txt` — Chapter 36, "Quicksort" from APAS.

### Definitions

| # | Definition | Prose Reference |
|---|-----------|-----------------|
| 1 | Pivot tree | The recursive decomposition tree induced by pivot choices; depth determines span. |
| 2 | Balanced vs. lopsided pivot tree | Sorted input with first-element pivot yields depth n (lopsided); random pivot yields depth Θ(lg n) expected. |

### Algorithms

| # | Algorithm | Prose Reference |
|---|-----------|-----------------|
| 1 | **Algorithm 36.1** — Generic Quicksort | Partition into `a1 = ⟨x ∈ a | x < p⟩`, `a2 = ⟨x ∈ a | x = p⟩`, `a3 = ⟨x ∈ a | x > p⟩`; recursively sort `a1` and `a3` in parallel; concatenate `s1 ++ a2 ++ s3`. |

### Pivot Strategies

| # | Strategy | Description | Work | Span |
|---|----------|-------------|------|------|
| 1 | First element | Always pick `a[0]` | Θ(n²) worst (sorted input) | Θ(n²) St / Θ(n) Mt worst |
| 2 | Median of three | Median of first, middle, last | Θ(n lg n) for sorted; Θ(n²) worst | Same as first-element worst case |
| 3 | Random element | Uniformly random pivot | Θ(n lg n) expected | Θ(lg² n) expected |

### Cost Specifications

| # | Item | Work | Span |
|---|------|------|------|
| 1 | Algorithm 36.1 (random pivot) | Θ(n lg n) expected | Θ(lg² n) expected |
| 2 | Algorithm 36.1 (first element, sorted) | Θ(n²) | Θ(n) Mt / Θ(n²) St |
| 3 | Partition step (prose: parallel filter) | Θ(n) work | Θ(lg n) span |
| 4 | Partition step (implementation: sequential DNF) | Θ(n) work | Θ(n) span |

### Theorems / Properties

| # | Property | Description |
|---|----------|-------------|
| 1 | Correctness | Output is a permutation of input and is sorted. |
| 2 | Random pivot expected work | Θ(n lg n) — probabilistic argument. |
| 3 | Random pivot expected depth | Θ(lg n) — recursion tree depth w.h.p. |

### Exercises

None specified in the prompt for Chapter 36.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Module | Function | APAS Cost | Actual Cost | Match? |
|---|--------|----------|-----------|-------------|:------:|
| 1 | StEph | `sort_vec` | W Θ(n lg n) exp / Θ(n²) worst, S = W | W Θ(n²) worst (first-element pivot), S = W (sequential) | Yes (first-element) |
| 2 | StEph | `quick_sort_first` | W Θ(n²) worst, S = W | Same | Yes |
| 3 | StEph | `quick_sort_median3` | W Θ(n lg n) for sorted | **W Θ(n²) for sorted** (uses first-element pivot despite name) | **No** |
| 4 | StEph | `quick_sort_random` | W Θ(n lg n) expected | **W Θ(n²) worst** (uses first-element pivot despite name) | **No** |
| 5 | MtEph | `sort_vec` | W Θ(n lg n) exp, S Θ(lg² n) exp | W Θ(n²) worst, S O(n) — parallel recursion via ParaPair!, sequential partition | Partial |
| 6 | MtEph | `quick_sort_first` | W Θ(n²) worst, S Θ(n) worst | Matches (parallel recursion, sequential partition) | Yes |
| 7 | MtEph | `quick_sort_median3` | Same as #3 | Uses first-element pivot despite name | **No** |
| 8 | MtEph | `quick_sort_random` | Same as #4 | Uses first-element pivot despite name | **No** |
| 9 | MtEphSlice | `quick_sort_mt_first` | W Θ(n²) worst, S Θ(n) worst | Matches | Yes |
| 10 | MtEphSlice | `quick_sort_mt_median3` | W Θ(n lg n) for sorted, S Θ(lg² n) exp | W yes, **S Θ(n)** (sequential partition) | Partial (span) |
| 11 | MtEphSlice | `quick_sort_mt_random` | W Θ(n lg n) exp, S Θ(lg² n) exp | W yes, **S Θ(n)** (sequential partition) | Partial (span) |

All Mt implementations achieve Θ(n) span (not Θ(lg² n)) because the three-way partition is a sequential DNF loop. APAS Θ(lg² n) assumes parallel filter for partition.

### 3b. Implementation Fidelity

| # | Module | Faithful? | Notes |
|---|--------|:---------:|-------|
| 1 | QuickSortStEph | **Partial** | Correct partition-sort-concat structure. All three public sort functions delegate to identical first-element-pivot code — no actual median-of-3 or random pivot selection. |
| 2 | QuickSortMtEph | **Yes** | Correct structure with parallel recursion via `ParaPair!(f1, f2)` for left/right recursive calls. Same first-element-pivot-only issue as StEph. |
| 3 | QuickSortMtEphSlice | **Good** | Genuinely parallel recursive calls via `thread::scope`. All three pivot strategies actually implemented (first, median-of-3, random). In-place DNF partition. |

### 3c. Spec Fidelity

| # | Module | Postcondition | Adequate? |
|---|--------|---------------|:---------:|
| 1 | StEph | `result@ =~= a.seq@.sort_by(spec_leq())` | **Strong** — proves output is `sort_by` of input |
| 2 | MtEph | Same as StEph | **Strong** — identical proof |
| 3 | MtEphSlice | N/A — no `verus!` | **None** |

Proof structure for StEph/MtEph `sort_vec`:
1. `result@ == candidate` (where `candidate = sorted_left ++ equals ++ sorted_right`) via elementwise matching from loop invariants
2. Connect `sort_vec` postconditions to `sort_by`
3. Multiset equalities between sorted and unsorted partitions
4. Length preservation through sorting
5. Sorted-left elements are all `< pivot` (via multiset membership transfer)
6. Sorted-right elements are all `> pivot` (same technique)
7. Candidate is `sorted_by(leq)` — case analysis across three segments using transitivity/reflexivity
8. Candidate has same multiset as input `s` via `lemma_multiset_commutative`
9. Uniqueness: `lemma_sorted_unique` establishes `s.sort_by(leq) =~= candidate`

## Phase 4: Parallelism Review

### Classify Each Mt Function

| # | Module | Function | Classification | Mechanism | Notes |
|---|--------|----------|:--------------:|-----------|-------|
| 1 | MtEph | `sort_vec` | **Parallel** | `ParaPair!(f1, f2)` for recursive calls | Verified; partition is sequential |
| 2 | MtEph | `quick_sort_first` | **Parallel** | Delegates to `sort_vec` | |
| 3 | MtEph | `quick_sort_median3` | **Parallel** | Delegates to `sort_vec` | |
| 4 | MtEph | `quick_sort_random` | **Parallel** | Delegates to `sort_vec` | |
| 5 | MtEphSlice | `pivot_mt_*` | Sequential | O(1) | Parallelism irrelevant |
| 6 | MtEphSlice | `quick_sort_mt_first` | **Parallel** | `thread::scope` | Genuinely parallel |
| 7 | MtEphSlice | `quick_sort_mt_median3` | **Parallel** | `thread::scope` | Genuinely parallel |
| 8 | MtEphSlice | `quick_sort_mt_random` | **Parallel** | `thread::scope` | Genuinely parallel |

### Span Audit

| # | Module | Function | APAS Span | Actual Span | Match? | Root Cause |
|---|--------|----------|-----------|-------------|:------:|------------|
| 1 | MtEph | all `quick_sort_*` | Θ(lg² n) exp | **Θ(n)** | **No** | Sequential partition dominates: S(n) = Θ(n) + S(3n/4) = Θ(n) |
| 2 | MtEphSlice | `quick_sort_mt_first` | Θ(n) worst | Θ(n) | Yes | First-element pivot worst case |
| 3 | MtEphSlice | `quick_sort_mt_median3` | Θ(lg² n) exp | **Θ(n)** | **No** | Sequential partition |
| 4 | MtEphSlice | `quick_sort_mt_random` | Θ(lg² n) exp | **Θ(n)** | **No** | Sequential partition |

## Phase 5: Runtime Test Review

| # | Test File | Tests | Coverage |
|---|-----------|:-----:|----------|
| 1 | TestQuickSortStEph.rs | 2 | Sorted output for all 3 variants, edge cases (empty, single, sorted, reversed, pair) |
| 2 | TestQuickSortMtEph.rs | 4 | Sorted output, edge cases, all-duplicates, large inputs (n=500) with sorted verification |
| 3 | TestQuickSortMtEphSlice.rs | 16 | Sorted output, edge cases, large inputs, pivot strategies, concurrent sorting stress test, large data handling |

**Test quality:** Good coverage. MtEph tests correctly use the free-function API (`quick_sort_first(&mut arr)`). MtEphSlice tests exercise all three pivot strategies distinctly and include concurrent stress tests.

## Phase 6: PTT Review

No PTT files exist in `rust_verify_test/tests/Chap36/`.

The partition loop has a complex multiset-preserving invariant that could benefit from a PTT. No iterators or `for` loops exist in the verusified files. **PTT for partition loop invariant would be valuable but is not critical.**

## Phase 7: Gap Analysis

### Prose items with no implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Parallel filter partition | **Not implemented** | All implementations use sequential DNF. Prose: 3 parallel filters with Θ(lg n) span. |
| 2 | Median-of-3 pivot (verusified) | **Not implemented** | Verusified StEph/MtEph claim `quick_sort_median3` but use first-element pivot. Only MtEphSlice implements actual median-of-3. |
| 3 | Random pivot (verusified) | **Not implemented** | Verusified StEph/MtEph claim `quick_sort_random` but use first-element pivot. Only MtEphSlice implements actual random pivot. |
| 4 | Benchmarks | **Not implemented** | Prompt mentions benchmarks; none exist. |
| 5 | Expected cost analysis (probabilistic) | **Not formalized** | No spec-level cost model. |

### Code with no prose counterpart

| # | Item | Purpose |
|---|------|---------|
| 1 | `QuickSortMtEphSlice` module | Slice-based variant using `with_exclusive` (Mutex) — implementation optimization. |
| 2 | `pivot_mt_*` standalone functions | Named implementations of underspecified pivot choice. |
| 3 | Base case n=1 with singleton proof | Prose only specifies base case for `|a| = 0`. Implementation adds n=1. |

### Critical Observation: Pivot Strategy Gap

The verusified code (StEph and MtEph) provides three public functions named after different pivot strategies, but all three delegate to the same `sort_vec` which always uses `*a.nth(0)` (first-element pivot). This is correct for the formal proof (the `sort_by` postcondition is pivot-independent), but the naming is misleading: `quick_sort_median3` does not use median-of-3 pivot selection, and `quick_sort_random` does not use random pivot selection. Only `QuickSortMtEphSlice` actually implements all three pivot strategies distinctly.

## Phase 8: TOC Review

### TOC Standard Compliance

| # | File | Has TOC? | Correct Sections? | Notes |
|---|------|:--------:|:-----------------:|-------|
| 1 | QuickSortStEph.rs | Yes | Yes — 1, 2, 3, 9 | Module, imports, broadcast use, impls |
| 2 | QuickSortMtEph.rs | Yes | Yes — 1, 2, 3, 9 | Same structure |
| 3 | QuickSortMtEphSlice.rs | No | N/A | No `verus!` block — TOC not applicable |

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | QuickSortStEph.rs | - | - | - | - | - | - | - | - | - |
| 2 | QuickSortMtEph.rs | - | - | - | - | - | - | - | - | - |
| 3 | QuickSortMtEphSlice.rs | - | - | - | - | - | - | - | - | `Chapter36MtSliceTrait` outside verus! |

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap36/

✓ QuickSortStEph.rs
✓ QuickSortMtEph.rs
✓ QuickSortMtEphSlice.rs

Modules: 3 clean, 0 holed
Holes Found: 0
```

All 3 files are clean. The verusified files (StEph, MtEph) contain no `assume`, `admit`, or `external_body`. MtEphSlice is entirely outside `verus!` so has no proof obligations.

## Spec Strength Summary

| Classification | Count |
|:--------------:|:-----:|
| strong | 8 |
| partial | 0 |
| weak | 0 |
| none | 7 |

The 8 strong specs are the 4 verusified functions in StEph and 4 in MtEph (each: `sort_vec` + 3 public wrappers). The 7 with no spec are the MtEphSlice functions (3 pivots + 3 sorts + inner `sort` fn ×3 counted as 1).

## Overall Assessment

**Maturity: Partially verified.** StEph and MtEph have strong `sort_by` postconditions and clean proofs with zero holes. MtEph uses `ParaPair!` for parallel recursion, matching the prose structure. Several substantial gaps remain:

| # | Issue | Severity |
|---|-------|:--------:|
| 1 | **Pivot strategies are fake in verusified code** — all three variants use first-element pivot | High |
| 2 | **Sequential partition in all variants** — Θ(n) span vs APAS Θ(lg n) per level | Medium |
| 3 | **MtEphSlice is unverified** — the only implementation with all 3 actual pivot strategies has no Verus specs | Medium |
| 4 | **No benchmarks** — prompt explicitly requests them | Low |

### Review TODOs

| # | Priority | Action |
|---|:--------:|--------|
| 1 | High | Implement actual pivot strategies in verusified code (median-of-3, random) or rename functions honestly |
| 2 | Medium | Add PTTs for partition loop invariant |
| 3 | Medium | Consider verusifying MtEphSlice (at minimum `external_body` wrapper with sort postcondition) |
| 4 | Low | Parallelize partition step (replace sequential DNF with parallel filter) |
| 5 | Low | Add benchmarks comparing St vs Mt vs MtEphSlice |
