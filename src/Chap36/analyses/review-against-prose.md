<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 36 — Quicksort: Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory

All functions extracted by `veracity-review-module-fn-impls -d src/Chap36`.

| # | Module | Function | V! | Spec | SpecStr | Lines |
|---|--------|----------|:--:|:----:|:-------:|------:|
| 1 | QuickSortMtEph | `pivot_mt_first` | -V! | NoSpec | none | 16–18 |
| 2 | QuickSortMtEph | `pivot_mt_median3` | -V! | NoSpec | none | 19–21 |
| 3 | QuickSortMtEph | `pivot_mt_random` | -V! | NoSpec | none | 22–24 |
| 4 | QuickSortMtEph | `quick_sort_mt_first` | -V! | NoSpec | none | 25–27 |
| 5 | QuickSortMtEph | `quick_sort_mt_median3` | -V! | NoSpec | none | 28–30 |
| 6 | QuickSortMtEph | `quick_sort_mt_random` | -V! | NoSpec | none | 31–33 |
| 7 | QuickSortMtEph | `quick_sort` (inner, ×3) | -V! | NoSpec | none | 62–90 |
| 8 | QuickSortMtEphSlice | `pivot_mt_first` | -V! | NoSpec | none | 16–18 |
| 9 | QuickSortMtEphSlice | `pivot_mt_median3` | -V! | NoSpec | none | 19–21 |
| 10 | QuickSortMtEphSlice | `pivot_mt_random` | -V! | NoSpec | none | 22–24 |
| 11 | QuickSortMtEphSlice | `quick_sort_mt_first` | -V! | NoSpec | none | 25–27 |
| 12 | QuickSortMtEphSlice | `quick_sort_mt_median3` | -V! | NoSpec | none | 28–30 |
| 13 | QuickSortMtEphSlice | `quick_sort_mt_random` | -V! | NoSpec | none | 31–33 |
| 14 | QuickSortMtEphSlice | `sort` (inner, ×3) | -V! | NoSpec | none | 64–92 |
| 15 | QuickSortStEph | `pivot_st_first` | -V! | NoSpec | none | 14–16 |
| 16 | QuickSortStEph | `pivot_st_median3` | -V! | NoSpec | none | 17–19 |
| 17 | QuickSortStEph | `pivot_st_random` | -V! | NoSpec | none | 20–22 |
| 18 | QuickSortStEph | `quick_sort_st_first` | -V! | NoSpec | none | 23–25 |
| 19 | QuickSortStEph | `quick_sort_st_median3` | -V! | NoSpec | none | 26–28 |
| 20 | QuickSortStEph | `quick_sort_st_random` | -V! | NoSpec | none | 29–31 |
| 21 | QuickSortStEph | `sort` (inner, ×3) | -V! | NoSpec | none | 56–85 |
| 22 | QuickSortStEph | `median3` | -V! | NoSpec | none | 91–103 |

**Key observation:** No module uses `verus!`. All code is plain Rust with zero formal specifications.

## Phase 2: Prose Inventory

Source: `prompts/Chap36.txt` — Chapter 36, "Quicksort" from APAS.

### Algorithms

| # | Item | Prose Reference |
|---|------|-----------------|
| 1 | **Algorithm 36.1** — Generic Quicksort | Partition into `a1 = ⟨x ∈ a | x < p⟩`, `a2 = ⟨x ∈ a | x = p⟩`, `a3 = ⟨x ∈ a | x > p⟩`; recursively sort `a1` and `a3` in parallel; concatenate `s1 ++ a2 ++ s3`. |

### Pivot Strategies

| # | Strategy | Description | Work | Span |
|---|----------|-------------|------|------|
| 1 | First element | Always pick `a[0]` | Θ(n²) worst (sorted input) | Θ(n²) worst |
| 2 | Median of three | Median of first, middle, last | Θ(n log n) for sorted; Θ(n²) worst | Θ(n²) worst |
| 3 | Random element | Uniformly random pivot | Θ(n log n) expected | Θ(lg² n) expected |

### Cost Specifications

| # | Item | Work | Span |
|---|------|------|------|
| 1 | Algorithm 36.1 (random pivot) | Θ(n log n) expected | Θ(lg² n) expected |
| 2 | Algorithm 36.1 (first element, sorted) | Θ(n²) | Θ(n²) |
| 3 | Partition step | Θ(n) work, Θ(lg n) span (parallel filter) | Per-level |

### Design Directives (from prompt)

| # | Item | Description |
|---|------|-------------|
| 1 | Use `ArraySeqStEph` for St variants | Confirmed in `QuickSortStEph.rs` |
| 2 | Use `ArraySeqMtEph` for Mt variants | Confirmed in `QuickSortMtEph.rs` |
| 3 | Three pivot strategies: first, median3, random | All three implemented in all modules |
| 4 | Naming: `quick_sort_{St,Mt}_PIVOT` | Followed: `quick_sort_st_first`, etc. |
| 5 | Mutex around ArraySeq members for Mt | `QuickSortMtEphSlice` uses `with_exclusive` (Mutex-wrapped); `QuickSortMtEph` clones to Vec then rebuilds |

### Exercises

None specified in the prompt for Chapter 36.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Module | Function | APAS Cost | Claude-Opus-4.6 Cost | Match? |
|---|--------|----------|-----------|----------------------|:------:|
| 1 | QuickSortStEph | `pivot_st_first` | W Θ(1), S Θ(1) | W Θ(1), S Θ(1) | Yes |
| 2 | QuickSortStEph | `pivot_st_median3` | W Θ(1), S Θ(1) | W Θ(1), S Θ(1) | Yes |
| 3 | QuickSortStEph | `pivot_st_random` | W Θ(1), S Θ(1) | W Θ(1), S Θ(1) | Yes |
| 4 | QuickSortStEph | `quick_sort_st_first` | W Θ(n lg n) exp / Θ(n²) worst, S = W | W Θ(n lg n) exp / Θ(n²) worst, S = W | Yes |
| 5 | QuickSortStEph | `quick_sort_st_median3` | W Θ(n lg n) exp / Θ(n²) worst, S = W | W Θ(n lg n) exp / Θ(n²) worst, S = W | Yes |
| 6 | QuickSortStEph | `quick_sort_st_random` | W Θ(n lg n) exp / Θ(n²) worst, S = W | W Θ(n lg n) exp / Θ(n²) worst, S = W | Yes |
| 7 | QuickSortMtEph | `pivot_mt_first` | W Θ(1), S Θ(1) | W Θ(1), S Θ(1) | Yes |
| 8 | QuickSortMtEph | `pivot_mt_median3` | W Θ(1), S Θ(1) | W Θ(1), S Θ(1) | Yes |
| 9 | QuickSortMtEph | `pivot_mt_random` | W Θ(1), S Θ(1) | W Θ(1), S Θ(1) | Yes |
| 10 | QuickSortMtEph | `quick_sort_mt_first` | W Θ(n lg n) exp / Θ(n²) worst, S Θ(lg² n) exp / Θ(n) worst | Same | Yes |
| 11 | QuickSortMtEph | `quick_sort_mt_median3` | W Θ(n lg n) exp / Θ(n²) worst, S Θ(lg² n) exp / Θ(n) worst | Same | Yes |
| 12 | QuickSortMtEph | `quick_sort_mt_random` | W Θ(n lg n) exp / Θ(n²) worst, S Θ(lg² n) exp / Θ(n) worst | Same | Yes |
| 13 | QuickSortMtEphSlice | (all pivots) | W Θ(1), S Θ(1) | Same | Yes |
| 14 | QuickSortMtEphSlice | (all sorts) | W Θ(n lg n) exp / Θ(n²) worst, S Θ(lg² n) exp / Θ(n) worst | Same | Yes |

**Note on Mt span:** The APAS prose states Θ(lg² n) span for random pivots because each level has Θ(lg n) span (parallel filter) and there are Θ(lg n) levels expected. The implementation achieves this through `thread::scope` which parallelizes the two recursive calls. However, the *partition step* is sequential (a single-pass Dutch National Flag loop with Θ(n) work and Θ(n) span), not the parallel filter the prose describes. This means each level has Θ(n) span (partition) + Θ(1) (spawn overhead), giving Span = Θ(n) at each level, but the levels overlap via parallel recursion. The recurrence S(n) = Θ(n) + S(3n/4) gives S(n) = Θ(n), not Θ(lg² n). However, the recursive calls are parallel, so the total span is dominated by the partition at the top level plus the span of the larger recursive subproblem: S(n) = Θ(n) + S(3n/4) = Θ(n). The actual achieved span is **Θ(n) expected** for all Mt variants, not Θ(lg² n), because the partition is not parallelized. The APAS Θ(lg² n) span assumes parallel filter for partition.

**Cost disagreement summary:** The Mt implementations achieve Θ(n) span (not Θ(lg² n)) because the three-way partition is sequential. To achieve Θ(lg² n) span, the partition would need to use parallel filter/scan.

### 3b. Implementation Fidelity

| # | Module | Faithful? | Notes |
|---|--------|:---------:|-------|
| 1 | QuickSortStEph | **Partial** | Uses in-place Dutch National Flag partition (3-way) instead of the prose's three parallel filters. Correct result; same work; different constant factors. Sequential recursion matches St semantics. |
| 2 | QuickSortMtEph | **Partial** | Same in-place partition. Copies `self.seq` to a `Vec`, sorts via slice, rebuilds `ArraySeqMtEphS`. The copy adds Θ(n) work overhead. Recursive calls are genuinely parallel via `thread::scope`. |
| 3 | QuickSortMtEphSlice | **Mostly** | Like MtEph but uses `with_exclusive` for in-place mutation on the underlying `Mutex<Vec<T>>` — avoids the copy. Recursive calls parallel via `thread::scope`. |

**Key deviations from prose:**
1. **Partition strategy:** Prose uses three parallel filters (`⟨x ∈ a | x < p⟩`, etc.). Implementation uses sequential Dutch National Flag (DNF) three-way partition. DNF is Θ(n) work, Θ(n) span. Parallel filter would be Θ(n) work, Θ(lg n) span.
2. **No parallel filter:** None of the Mt implementations parallelize the partition step. This is the root cause of the span discrepancy.
3. **In-place vs. functional:** Prose is functional (creates new sequences). Implementation is in-place (swaps on mutable slices). This is a common and acceptable optimization.

### 3c. Spec Fidelity

No Verus specs exist. All code is outside `verus!` blocks. There are no `requires`, `ensures`, `spec fn`, or `proof fn` in any Chap36 file.

**What specs should express if added:**
- **Precondition:** For pivot functions: `lo < hi && hi <= self.length()`.
- **Postcondition for sort:** Output is a permutation of the input and is sorted. In Verus spec language: `ensures self@.to_multiset() == old(self)@.to_multiset() && forall|i, j| 0 <= i < j < self@.len() ==> self@[i] <= self@[j]`.

## Phase 4: Parallelism Review

### 4a. Classify Each Mt Function

| # | Module | Function | Classification | Notes |
|---|--------|----------|:--------------:|-------|
| 1 | QuickSortMtEph | `pivot_mt_first` | Sequential | O(1) — no parallelism needed |
| 2 | QuickSortMtEph | `pivot_mt_median3` | Sequential | O(1) — no parallelism needed |
| 3 | QuickSortMtEph | `pivot_mt_random` | Sequential | O(1) — no parallelism needed |
| 4 | QuickSortMtEph | `quick_sort_mt_first` | **Parallel** | `thread::scope` spawns left recursive call |
| 5 | QuickSortMtEph | `quick_sort_mt_median3` | **Parallel** | `thread::scope` spawns left recursive call |
| 6 | QuickSortMtEph | `quick_sort_mt_random` | **Parallel** | `thread::scope` spawns left recursive call |
| 7 | QuickSortMtEphSlice | `pivot_mt_first` | Sequential | O(1) |
| 8 | QuickSortMtEphSlice | `pivot_mt_median3` | Sequential | O(1) |
| 9 | QuickSortMtEphSlice | `pivot_mt_random` | Sequential | O(1) |
| 10 | QuickSortMtEphSlice | `quick_sort_mt_first` | **Parallel** | `thread::scope` inside `with_exclusive` |
| 11 | QuickSortMtEphSlice | `quick_sort_mt_median3` | **Parallel** | `thread::scope` inside `with_exclusive` |
| 12 | QuickSortMtEphSlice | `quick_sort_mt_random` | **Parallel** | `thread::scope` inside `with_exclusive` |

### 4b. Span Audit

| # | Function | APAS Span | Actual Span | Match? | Notes |
|---|----------|-----------|-------------|:------:|-------|
| 1 | `quick_sort_mt_first` | Θ(lg² n) exp / Θ(n) worst | **Θ(n) expected** | **No** | Sequential partition dominates; see note above |
| 2 | `quick_sort_mt_median3` | Θ(lg² n) exp / Θ(n) worst | **Θ(n) expected** | **No** | Same reason |
| 3 | `quick_sort_mt_random` | Θ(lg² n) exp / Θ(n) worst | **Θ(n) expected** | **No** | Same reason |
| 4 | MtEphSlice variants | Same as above | Same as above | **No** | Same partition bottleneck |

**Root cause:** The three-way partition loop is sequential Θ(n). APAS assumes parallel partition via filter with Θ(lg n) span. To fix: replace the DNF loop with three parallel `filter` calls, which would require a scan-based partition.

### 4c. Parallelism Gap Table

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|:---------:|-------|
| 1 | `pivot_mt_*` | Θ(1) | Θ(1) | N/A | O(1) — parallelism irrelevant |
| 2 | `quick_sort_mt_first` | Θ(n) worst | Θ(n) | Yes (recursive) | Partition sequential |
| 3 | `quick_sort_mt_median3` | Θ(lg² n) exp | Θ(n) | Yes (recursive) | Partition sequential |
| 4 | `quick_sort_mt_random` | Θ(lg² n) exp | Θ(n) | Yes (recursive) | Partition sequential |

## Phase 5: Runtime Test Review

### 5a. Coverage Check

**No runtime test files exist for Chapter 36.**

Expected files:
- `tests/Chap36/TestQuickSortStEph.rs`
- `tests/Chap36/TestQuickSortMtEph.rs`
- `tests/Chap36/TestQuickSortMtEphSlice.rs`

### 5b. Test Quality

N/A — no tests exist.

### 5c. Missing Tests (Proposed)

| # | Priority | Test | Why |
|---|:--------:|------|-----|
| 1 | High | All St sort variants on empty, singleton, sorted, reverse-sorted, random inputs | No specs — runtime tests are the only correctness evidence |
| 2 | High | All Mt sort variants on same inputs, using set equality | Mt partition may reorder equal elements |
| 3 | Medium | Pivot functions return an element from the input range | Sanity check |
| 4 | Medium | Large random inputs (n = 10000+) verify sorting | Stress test |
| 5 | Low | MtEphSlice vs MtEph produce same multiset | Cross-implementation consistency |

## Phase 6: Proof-Time Test (PTT) Review

No `verus!` blocks exist in any Chap36 file. There are no iterators, no verified loops, no ghost state, and no proof functions. **No PTTs are needed.**

### 6a. Unified Test Inventory Table

| # | Source module | RTT file | PTT file | Status |
|---|-------------|----------|----------|--------|
| 1 | QuickSortStEph | (none) | N/A | Missing RTT |
| 2 | QuickSortMtEph | (none) | N/A | Missing RTT |
| 3 | QuickSortMtEphSlice | (none) | N/A | Missing RTT |

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Item | Status |
|---|------|--------|
| 1 | Parallel filter partition | Not implemented — all variants use sequential DNF |

The prose describes partition as three parallel filters. The implementation uses an in-place sequential three-way partition (Dutch National Flag). This is a conscious design choice (in-place is faster in practice) but means the Θ(lg² n) span bound is not achieved.

### Code With No Prose Counterpart

| # | Item | Purpose |
|---|------|---------|
| 1 | `QuickSortMtEphSlice` module | Slice-based variant using `with_exclusive` — an implementation optimization not in the prose |
| 2 | `pivot_*` standalone functions | The prose leaves pivot selection "underspecified"; these are named implementations |
| 3 | `median3` inner function | Helper for median-of-three pivot, used inside `quick_sort_st_median3` |

### Missing from Prose Prompt

The prompt mentions `ArraySeqMtEphChap18.rs`, `ArraySeqMtEphChap19.rs`, and `ArraySeqMtEph.rs` as prerequisites. These are in `src/Chap18/` and `src/Chap19/`, not in Chap36. The prompt also mentions benchmarks — none exist in the current codebase for Chap36.

## Phase 8: Table of Contents Review

None of the Chap36 files have a Table of Contents block. None use `verus!` blocks, so the standard TOC sections (type definitions, view impls, spec fns, proof fns, etc.) do not apply. The files are plain Rust modules.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | QuickSortStEph.rs | - | - | - | - | - | - | - | - | - |
| 2 | QuickSortMtEph.rs | - | - | - | - | - | - | - | - | - |
| 3 | QuickSortMtEphSlice.rs | - | - | - | - | - | - | - | - | - |

No derive impls in any file. All files are purely algorithmic.

**TOC action items:** None — files contain only a trait + impl block and are too simple to warrant TOC scaffolding.

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap36/

✓ QuickSortMtEph.rs
✓ QuickSortMtEphSlice.rs
✓ QuickSortStEph.rs

Modules: 3 clean, 0 holed
Proof Functions: 0 total
Holes Found: 0

No proof holes found — but trivially so, since no verus! blocks exist.
```

## Spec Strength Summary

| Classification | Count |
|:--------------:|:-----:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 22 |

All 22 functions have **no formal specification**. This is expected: Chapter 36 has not been verusified.

## Overall Assessment

**Maturity: Unverified implementation only.** Chapter 36 provides three working quicksort variants (first-element, median-of-3, random pivot) in St, Mt, and Mt-slice flavors. The algorithms are correct implementations of the quicksort pattern, but:

1. **No Verus verification.** Zero `verus!` blocks, zero specs, zero proofs.
2. **No runtime tests.** No test files exist anywhere in the project for Chap36.
3. **Span discrepancy.** Mt implementations achieve Θ(n) span (not Θ(lg² n)) because the three-way partition is sequential rather than the parallel filter described in APAS.
4. **In-place vs. functional.** All implementations use in-place Dutch National Flag partitioning rather than the functional filter/append style in the prose. This is standard practice but changes the constant factors and prevents parallel partition.
5. **Copy overhead in MtEph.** `QuickSortMtEph` clones the inner `Vec`, sorts it, then rebuilds the `ArraySeqMtEphS`. The slice variant (`QuickSortMtEphSlice`) avoids this by using `with_exclusive`.

### Recommended Next Steps

| # | Priority | Action |
|---|:--------:|--------|
| 1 | High | Add runtime tests for all 9 sort entry points |
| 2 | Medium | Consider verusifying at least the St variant with a permutation + sorted postcondition |
| 3 | Low | Parallelize the partition step in Mt variants to achieve the APAS Θ(lg² n) span |
| 4 | Low | Add benchmarks as mentioned in the prose prompt |
