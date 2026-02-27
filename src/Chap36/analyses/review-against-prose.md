<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 36 — Quicksort: Review Against Prose

**Date:** 2026-02-27 (updated: trait-impl pattern, module renames, meaningful return names, 0 proof holes)
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory

All functions extracted by `veracity-review-module-fn-impls -d src/Chap36`.

| # | Module | Function | V! | Location | Spec | SpecStr | Lines |
|---|--------|----------|:--:|----------|:----:|:-------:|------:|
| 1 | QuickSortStEph | `sort_vec` | V! | ML | HasSpec | strong | 44–47 |
| 2 | QuickSortStEph | `sort_vec_with_idx` | V! | ML | HasSpec | strong | 464–470 |
| 3 | QuickSortStEph | `sort_vec_random` | V! | ML | HasSpec | strong | 436–439 |
| 4 | QuickSortStEph | `sort_vec_median3` | V! | ML | HasSpec | strong | 449–452 |
| 5 | QuickSortStEph | `spec_median_of_three` | V! | ML | Spec | strong | 376–386 |
| 6 | QuickSortStEph | `median_of_three` | V! | ML | HasSpec | strong | 388–415 |
| 7 | QuickSortStEph | `median3_pivot_idx` | V! | ML | HasSpec | strong | 421–434 |
| 8 | QuickSortStEph | `quick_sort_first` | V! | ML | HasSpec | strong | 367–373 |
| 9 | QuickSortStEph | `quick_sort_median3` | V! | ML | HasSpec | strong | 735–741 |
| 10 | QuickSortStEph | `quick_sort_random` | V! | ML | HasSpec | strong | 744–750 |
| 11 | QuickSortMtEph | `sort_vec` | V! | ML | HasSpec | strong | 46–49 |
| 12 | QuickSortMtEph | `sort_vec_with_idx` | V! | ML | HasSpec | strong | 443–449 |
| 13 | QuickSortMtEph | `sort_vec_random` | V! | ML | HasSpec | strong | 415–418 |
| 14 | QuickSortMtEph | `sort_vec_median3` | V! | ML | HasSpec | strong | 428–431 |
| 15 | QuickSortMtEph | `spec_median_of_three` | V! | ML | Spec | strong | 359–369 |
| 16 | QuickSortMtEph | `median_of_three` | V! | ML | HasSpec | strong | 371–398 |
| 17 | QuickSortMtEph | `median3_pivot_idx` | V! | ML | HasSpec | strong | 400–413 |
| 18 | QuickSortMtEph | `quick_sort_first` | V! | ML | HasSpec | strong | 735–741 |
| 19 | QuickSortMtEph | `quick_sort_median3` | V! | ML | HasSpec | strong | 744–750 |
| 20 | QuickSortMtEph | `quick_sort_random` | V! | ML | HasSpec | strong | 753–759 |
| 21 | QuickSortMtEphSlice | `pivot_mt_first` | -V! | Tr+IT | NoSpec | none | 16–18 |
| 22 | QuickSortMtEphSlice | `pivot_mt_median3` | -V! | Tr+IT | NoSpec | none | 19–21 |
| 23 | QuickSortMtEphSlice | `pivot_mt_random` | -V! | Tr+IT | NoSpec | none | 22–24 |
| 24 | QuickSortMtEphSlice | `quick_sort_mt_first` | -V! | Tr+IT | NoSpec | none | 25–27 |
| 25 | QuickSortMtEphSlice | `quick_sort_mt_median3` | -V! | Tr+IT | NoSpec | none | 28–30 |
| 26 | QuickSortMtEphSlice | `quick_sort_mt_random` | -V! | Tr+IT | NoSpec | none | 31–33 |
| 27 | QuickSortMtEphSlice | `sort` (inner fn, x3) | -V! | IT | NoSpec | none | 64–94 |

StEph and MtEph are inside `verus!` with strong `sort_by` postconditions and trait-impl pattern (`QuickSortStEphTrait`, `QuickSortMtEphTrait`). Both implement all three pivot strategies (first-element, median-of-three, random) with separate `sort_vec`, `sort_vec_with_idx`, `sort_vec_random`, and `sort_vec_median3` functions. MtEphSlice is inside `verus!` with `Chapter36MtSliceTrait` having precondition specs on pivot functions; sort functions lack ensures (unverified implementations).

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
| 1 | StEph | `sort_vec` | W Θ(n²) worst (first-element), S = W | W Θ(n²) worst, S = W (sequential) | Yes |
| 2 | StEph | `quick_sort_first` | W Θ(n²) worst, S = W | Same | Yes |
| 3 | StEph | `quick_sort_median3` | W Θ(n lg n) for sorted | W Θ(n lg n) for sorted — uses actual median-of-3 pivot | Yes |
| 4 | StEph | `quick_sort_random` | W Θ(n lg n) expected | W Θ(n lg n) expected — uses actual random pivot | Yes |
| 5 | MtEph | `sort_vec` | W Θ(n²) worst, S Θ(n) worst | W Θ(n²) worst, S O(n) — parallel recursion via ParaPair!, sequential partition | Yes |
| 6 | MtEph | `quick_sort_first` | W Θ(n²) worst, S Θ(n) worst | Matches (parallel recursion, sequential partition) | Yes |
| 7 | MtEph | `quick_sort_median3` | W Θ(n lg n) for sorted, S Θ(lg² n) exp | W yes, **S Θ(n)** (sequential partition) | Partial (span) |
| 8 | MtEph | `quick_sort_random` | W Θ(n lg n) exp, S Θ(lg² n) exp | W yes, **S Θ(n)** (sequential partition) | Partial (span) |
| 9 | MtEphSlice | `quick_sort_mt_first` | W Θ(n²) worst, S Θ(n) worst | Matches | Yes |
| 10 | MtEphSlice | `quick_sort_mt_median3` | W Θ(n lg n) for sorted, S Θ(lg² n) exp | W yes, **S Θ(n)** (sequential partition) | Partial (span) |
| 11 | MtEphSlice | `quick_sort_mt_random` | W Θ(n lg n) exp, S Θ(lg² n) exp | W yes, **S Θ(n)** (sequential partition) | Partial (span) |

All Mt implementations achieve Θ(n) span (not Θ(lg² n)) because the three-way partition is a sequential DNF loop. APAS Θ(lg² n) assumes parallel filter for partition.

### 3b. Implementation Fidelity

| # | Module | Faithful? | Notes |
|---|--------|:---------:|-------|
| 1 | QuickSortStEph | **Yes** | Correct partition-sort-concat structure. All three pivot strategies genuinely implemented: `sort_vec` (first-element), `sort_vec_median3` (median-of-three via `median3_pivot_idx`), `sort_vec_random` (random via `random_usize_range`). Core partition logic shared via `sort_vec_with_idx`. |
| 2 | QuickSortMtEph | **Yes** | Same structure as StEph with parallel recursion via `ParaPair!(f1, f2)` for left/right recursive calls. All three pivot strategies genuinely implemented. Core shared via `sort_vec_with_idx`. |
| 3 | QuickSortMtEphSlice | **Good** | Genuinely parallel recursive calls via `thread::scope`. All three pivot strategies actually implemented (first, median-of-3, random). In-place DNF partition. |

### 3c. Spec Fidelity

| # | Module | Postcondition | Adequate? |
|---|--------|---------------|:---------:|
| 1 | StEph | `sorted@ =~= a.seq@.sort_by(spec_leq())` | **Strong** — proves output is `sort_by` of input |
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
| 1 | MtEph | `sort_vec` / `sort_vec_with_idx` | **Parallel** | `ParaPair!(f1, f2)` for recursive calls | Verified; partition is sequential |
| 2 | MtEph | `quick_sort_first` | **Parallel** | Delegates to `sort_vec` | |
| 3 | MtEph | `quick_sort_median3` | **Parallel** | Delegates to `sort_vec_median3` → `sort_vec_with_idx` | Actual median-of-3 pivot |
| 4 | MtEph | `quick_sort_random` | **Parallel** | Delegates to `sort_vec_random` → `sort_vec_with_idx` | Actual random pivot |
| 5 | MtEphSlice | `pivot_mt_*` | Sequential | O(1) | Parallelism irrelevant |
| 6 | MtEphSlice | `quick_sort_mt_first` | **Parallel** | `thread::scope` | Genuinely parallel |
| 7 | MtEphSlice | `quick_sort_mt_median3` | **Parallel** | `thread::scope` | Genuinely parallel |
| 8 | MtEphSlice | `quick_sort_mt_random` | **Parallel** | `thread::scope` | Genuinely parallel |

### Span Audit

| # | Module | Function | APAS Span | Actual Span | Match? | Root Cause |
|---|--------|----------|-----------|-------------|:------:|------------|
| 1 | MtEph | `quick_sort_first` | Θ(n) worst | Θ(n) | Yes | Sequential partition, first-element pivot worst case |
| 2 | MtEph | `quick_sort_median3` | Θ(lg² n) exp | **Θ(n)** | **No** | Sequential partition dominates: S(n) = Θ(n) + S(3n/4) = Θ(n) |
| 3 | MtEph | `quick_sort_random` | Θ(lg² n) exp | **Θ(n)** | **No** | Sequential partition dominates |
| 4 | MtEphSlice | `quick_sort_mt_first` | Θ(n) worst | Θ(n) | Yes | First-element pivot worst case |
| 5 | MtEphSlice | `quick_sort_mt_median3` | Θ(lg² n) exp | **Θ(n)** | **No** | Sequential partition |
| 6 | MtEphSlice | `quick_sort_mt_random` | Θ(lg² n) exp | **Θ(n)** | **No** | Sequential partition |

## Phase 5: Runtime Test Review

| # | Test File | Tests | Coverage |
|---|-----------|:-----:|----------|
| 1 | TestQuickSortStEph.rs | 2 | Sorted output for all 3 variants, edge cases (empty, single, sorted, reversed, pair) |
| 2 | TestQuickSortMtEph.rs | 4 | Sorted output, edge cases, all-duplicates, large inputs (n=500) with sorted verification |
| 3 | TestQuickSortMtEphSlice.rs | 16 | Sorted output, edge cases, large inputs, pivot strategies, concurrent sorting stress test, large data handling |

**Test quality:** Good coverage. StEph and MtEph tests use trait method call syntax (`ArraySeqStEphS::quick_sort_first(&mut arr)`). MtEphSlice tests exercise all three pivot strategies distinctly and include concurrent stress tests.

## Phase 6: PTT Review

No PTT files exist in `rust_verify_test/tests/Chap36/`.

The partition loop has a complex multiset-preserving invariant that could benefit from a PTT. No iterators or `for` loops exist in the verusified files. **PTT for partition loop invariant would be valuable but is not critical.**

## Phase 7: Gap Analysis

### Prose items with no implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Parallel filter partition | **Not implemented** | All implementations use sequential DNF. Prose: 3 parallel filters with Θ(lg n) span. |
| 2 | ~~Median-of-3 pivot (verusified)~~ | **Implemented** | StEph and MtEph now implement actual median-of-3 via `median3_pivot_idx` + `sort_vec_median3`. |
| 3 | ~~Random pivot (verusified)~~ | **Implemented** | StEph and MtEph now implement actual random pivot via `random_usize_range` + `sort_vec_random`. |
| 4 | Benchmarks | **Not implemented** | Prompt mentions benchmarks; none exist. |
| 5 | Expected cost analysis (probabilistic) | **Not formalized** | No spec-level cost model. |

### Code with no prose counterpart

| # | Item | Purpose |
|---|------|---------|
| 1 | `QuickSortMtEphSlice` module | Slice-based variant using `with_exclusive` (Mutex) — implementation optimization. |
| 2 | `pivot_mt_*` standalone functions | Named implementations of underspecified pivot choice. |
| 3 | Base case n=1 with singleton proof | Prose only specifies base case for `|a| = 0`. Implementation adds n=1. |

### Pivot Strategy Implementation (Resolved)

The verusified code (StEph and MtEph) now genuinely implements all three pivot strategies:
- `quick_sort_first` delegates to `sort_vec` (first-element pivot at index 0)
- `quick_sort_median3` delegates to `sort_vec_median3` which calls `median3_pivot_idx` to pick the median of `a[0]`, `a[n/2]`, `a[n-1]`, then passes that index to `sort_vec_with_idx`
- `quick_sort_random` delegates to `sort_vec_random` which calls `random_usize_range(0, n)` then passes that index to `sort_vec_with_idx`

The `sort_vec_with_idx` function is a generalized version of `sort_vec` that accepts an arbitrary valid pivot index. Its proof is identical to `sort_vec` except the pivot-tracking invariant uses `pivot_idx` instead of 0. All three public wrappers have the same `sort_by` postcondition.

## Phase 8: TOC Review

### TOC Standard Compliance

| # | File | Has TOC? | Correct Sections? | Notes |
|---|------|:--------:|:-----------------:|-------|
| 1 | QuickSortStEph.rs | Yes | Yes — 1, 2, 3, 6, 8, 9 | Module, imports, broadcast use, spec fns, traits, impls. |
| 2 | QuickSortMtEph.rs | Yes | Yes — 1, 2, 3, 6, 8, 9 | Same structure as StEph. |
| 3 | QuickSortMtEphSlice.rs | No | N/A | Inside verus! but no TOC headers. |

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | QuickSortStEph.rs | - | - | - | - | - | - | - | - | - |
| 2 | QuickSortMtEph.rs | - | - | - | - | - | - | - | - | - |
| 3 | QuickSortMtEphSlice.rs | - | - | - | - | - | - | - | - | `Chapter36MtSliceTrait` inside verus! |

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap36/

✓ QuickSortMtEph.rs
✓ QuickSortMtEphSlice.rs
✓ QuickSortStEph.rs

Modules: 3 clean, 0 holed
Holes Found: 0 total
```

All three modules are clean. The previous `external_body` on `median3_pivot_idx` was resolved — the function now uses `TotalOrder::cmp` equality matching which Verus can verify directly.

## Spec Strength Summary

| Classification | Count |
|:--------------:|:-----:|
| strong | 20 |
| partial | 0 |
| weak | 0 |
| none | 7 |

The 20 strong specs are the 10 verusified functions in StEph and 10 in MtEph (each: `sort_vec`, `sort_vec_with_idx`, `sort_vec_random`, `sort_vec_median3`, `spec_median_of_three`, `median_of_three`, `median3_pivot_idx`, `quick_sort_first`, `quick_sort_median3`, `quick_sort_random`). The 7 with no spec are the MtEphSlice functions (3 pivots + 3 sorts + inner `sort` fn x3 counted as 1).

## Overall Assessment

**Maturity: Well verified.** StEph and MtEph have strong `sort_by` postconditions with all three pivot strategies genuinely implemented and verified. Zero proof holes. Both follow the trait-impl pattern with `QuickSortStEphTrait` and `QuickSortMtEphTrait`. MtEph uses `ParaPair!` for parallel recursion, matching the prose structure.

| # | Issue | Severity |
|---|-------|:--------:|
| 1 | **Sequential partition in all variants** — Θ(n) span vs APAS Θ(lg n) per level | Medium |
| 2 | **MtEphSlice is unverified** — has precondition specs on pivots but no sort postconditions; commented out in lib.rs due to missing `with_exclusive` method and nested functions | Medium |
| 3 | **No benchmarks** — prompt explicitly requests them | Low |

### Review TODOs

| # | Priority | Action |
|---|:--------:|--------|
| 1 | Medium | Consider verusifying MtEphSlice (requires extracting nested fns, adding `with_exclusive` to ArraySeqMtEphSlice) |
| 2 | Low | Parallelize partition step (replace sequential DNF with parallel filter) |
| 3 | Low | Add benchmarks comparing St vs Mt vs MtEphSlice |
