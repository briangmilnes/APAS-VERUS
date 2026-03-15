# Chapter 36: Quicksort -- Review Against Prose

Reviewer: Claude-Opus-4.6 (agent2)
Date: 2026-03-15
Prose source: `prompts/Chap36.txt` (APAS Algorithm 36.1, pivot selection analysis)

## Phase 1: Inventory

| # | Chap | File | Module | Type | Lines | Status |
|---|------|------|--------|------|------:|--------|
| 1 | 36 | QuickSortStEph.rs | QuickSortStEph | St | 689 | Clean, 0 holes |
| 2 | 36 | QuickSortMtEph.rs | QuickSortMtEph | Mt | 738 | Clean, 0 holes |
| 3 | 36 | QuickSortMtEphSlice.rs | QuickSortMtEphSlice | Mt | 835 | Clean, 0 holes |

Total functions: 50 (14 spec/proof, 36 exec with complete specs).
Proof functions: 7 clean, 0 holed.
Modules: 3 clean (100%).

## Phase 2: Prose Mapping

APAS Algorithm 36.1 defines a generic quicksort with an underspecified pivot step.
Three pivot strategies are analyzed: first-element, median-of-three, random.

| # | Chap | File | Prose Concept | Implementation | Coverage |
|---|------|------|---------------|----------------|----------|
| 1 | 36 | QuickSortStEph.rs | quicksort(a) with first-element pivot | `quick_sort_first` | Full |
| 2 | 36 | QuickSortStEph.rs | quicksort(a) with median-of-three | `quick_sort_median3` | Full |
| 3 | 36 | QuickSortStEph.rs | quicksort(a) with random pivot | `quick_sort_random` | Full |
| 4 | 36 | QuickSortStEph.rs | Three-way partition (< p, = p, > p) | Inline partition loop | Full |
| 5 | 36 | QuickSortStEph.rs | Sorted-left ++ equals ++ sorted-right | `concat_three` | Full |
| 6 | 36 | QuickSortStEph.rs | Median-of-three computation | `median_of_three`, `median3_pivot_idx` | Full |
| 7 | 36 | QuickSortMtEph.rs | Parallel recursive calls | `ParaPair!` on left/right | Full |
| 8 | 36 | QuickSortMtEphSlice.rs | Parallel recursive calls (slice) | `ParaPair!` on left/right | Full |

**Missing from prose:** Nothing. All three pivot strategies and the generic algorithm
structure are fully implemented in all three modules.

**Extra beyond prose:** The `QuickSortMtEphSlice` module adds a slice-based parallel
variant using `ArraySeqMtEphSliceS` with `Clone`-based element access rather than `Copy`.

## Phase 3: Cost Annotations

### Phase 3a: Annotation Summary

| # | Chap | File | Function | APAS Cost | Agent Cost | Match? |
|---|------|------|----------|-----------|------------|--------|
| 1 | 36 | QuickSortStEph.rs | quick_sort_first | W O(n^2) worst | Span = Work | Yes |
| 2 | 36 | QuickSortStEph.rs | quick_sort_median3 | W O(n^2) worst / O(n lg n) sorted | Span = Work | Yes |
| 3 | 36 | QuickSortStEph.rs | quick_sort_random | W O(n lg n) expected | Span = Work | Yes |
| 4 | 36 | QuickSortMtEph.rs | quick_sort_first | W O(n^2), S O(n lg n) (par filter) | S O(n^2) seq partition | Deviation |
| 5 | 36 | QuickSortMtEph.rs | quick_sort_median3 | W O(n lg n), S O(lg^2 n) sorted | S O(n) sorted, seq partition | Deviation |
| 6 | 36 | QuickSortMtEph.rs | quick_sort_random | W O(n lg n), S O(lg^2 n) expected | S O(n) expected, seq partition | Deviation |
| 7 | 36 | QuickSortMtEphSlice.rs | quick_sort_first | Same as MtEph | Same as MtEph | Deviation |
| 8 | 36 | QuickSortMtEphSlice.rs | quick_sort_median3 | Same as MtEph | Same as MtEph | Deviation |
| 9 | 36 | QuickSortMtEphSlice.rs | quick_sort_random | Same as MtEph | Same as MtEph | Deviation |

### Span Deviation Analysis (Mt modules)

APAS assumes partition is done via parallel filter, giving O(lg n) span per level.
Both Mt implementations use a sequential while loop for the three-way partition,
which yields O(n) span per partition level. This affects span but not work:

- **First-element worst case:** APAS span O(n lg n), implementation O(n^2).
  Depth n with O(n) sequential partition at each level.
- **Median-of-three sorted case:** APAS span O(lg^2 n), implementation O(n).
  Depth O(lg n) with O(n) partition at root level dominating.
- **Random expected case:** APAS span O(lg^2 n), implementation O(n).
  Depth O(lg n) with O(n) partition at root level dominating.

The parallel filter could be achieved by using a parallel `filter` operation from
Chapter 4 or by implementing a parallel partition step. This is a known architectural
choice, not a bug -- the partition parallelism is orthogonal to the recursive
parallelism that `ParaPair!` provides.

## Phase 4: Parallelism Audit (Mt modules only)

| # | Chap | File | Operation | Classification | Notes |
|---|------|------|-----------|----------------|-------|
| 1 | 36 | QuickSortMtEph.rs | Recursive sort of left/right | Parallel (ParaPair!) | Correct |
| 2 | 36 | QuickSortMtEph.rs | Three-way partition | Sequential (while loop) | Deviation |
| 3 | 36 | QuickSortMtEph.rs | concat_three | Sequential (three loops) | Correct for O(n) |
| 4 | 36 | QuickSortMtEphSlice.rs | Recursive sort of left/right | Parallel (ParaPair!) | Correct |
| 5 | 36 | QuickSortMtEphSlice.rs | Three-way partition | Sequential (while loop) | Deviation |
| 6 | 36 | QuickSortMtEphSlice.rs | concat_three_vecs | Sequential (three loops) | Correct for O(n) |

**Parallelism classification:**
- Recursive calls: correctly parallelized via `ParaPair!` macro with named closures
  and explicit `ensures` clauses. This follows the APAS structure.
- Partition: done sequentially. APAS presents the partition as three parallel filters
  (lines 6-8 of Algorithm 36.1: `<x in a | x < p>`, `<x in a | x = p>`,
  `<x in a | x > p>`). The implementation uses a single sequential scan that produces
  three output vectors. This reduces span from O(lg n) to O(n) per partition level.
- Concatenation: sequential, which is fine since it is O(n) work and bounded by the
  partition span already.

**No sequentialization of parallel algorithms.** The recursive parallelism is preserved.
The partition could be parallelized in future work.

## Phase 5: Spec Fidelity

| # | Chap | File | Function | Spec Strength | Notes |
|---|------|------|----------|:-------------:|-------|
| 1 | 36 | QuickSortStEph.rs | quick_sort_first | Strong | sort_by(leq) + len preserved |
| 2 | 36 | QuickSortStEph.rs | quick_sort_median3 | Strong | sort_by(leq) + len preserved |
| 3 | 36 | QuickSortStEph.rs | quick_sort_random | Strong | sort_by(leq) + len preserved |
| 4 | 36 | QuickSortStEph.rs | median_of_three | Strong | result in {a,b,c}, == spec fn |
| 5 | 36 | QuickSortStEph.rs | median3_pivot_idx | Strong | idx in {0, n/2, n-1}, a[idx] == median |
| 6 | 36 | QuickSortStEph.rs | concat_three | Strong | out@ =~= left@ + mid@ + right@ |
| 7 | 36 | QuickSortMtEph.rs | quick_sort_first | Strong | same as StEph |
| 8 | 36 | QuickSortMtEph.rs | quick_sort_median3 | Strong | same as StEph |
| 9 | 36 | QuickSortMtEph.rs | quick_sort_random | Strong | same as StEph |
| 10 | 36 | QuickSortMtEph.rs | median_of_three | Strong | same as StEph |
| 11 | 36 | QuickSortMtEph.rs | median3_pivot_idx | Strong | same as StEph |
| 12 | 36 | QuickSortMtEph.rs | concat_three | Strong | same as StEph |
| 13 | 36 | QuickSortMtEphSlice.rs | quick_sort_first | Strong | elements(*a) =~= sort_by + wf + len |
| 14 | 36 | QuickSortMtEphSlice.rs | quick_sort_median3 | Strong | same |
| 15 | 36 | QuickSortMtEphSlice.rs | quick_sort_random | Strong | same |
| 16 | 36 | QuickSortMtEphSlice.rs | median_of_three | Strong | same as StEph |
| 17 | 36 | QuickSortMtEphSlice.rs | median3_pivot_idx | Strong | wf preserved, same |
| 18 | 36 | QuickSortMtEphSlice.rs | concat_three_vecs | Strong | out@ =~= left@ + mid@ + right@ |

All exec functions: 18/18 strong specs. All sorting functions ensure:
- Output is the input sorted by the total ordering (`sort_by(spec_leq::<T>())`).
- Length is preserved.
- MtEphSlice additionally ensures `spec_arrayseqmtephslice_wf()`.

The `sort_by` ensures both sortedness and permutation (multiset equality) via vstd's
`lemma_sort_by_ensures`, which is the strongest possible sorting specification.

## Phase 6: RTT/PTT Review

### Runtime Tests (RTT)

| # | Chap | File | Test Count | Coverage |
|---|------|------|:----------:|----------|
| 1 | 36 | TestQuickSortStEph.rs | 2 | All 3 sort variants, edge cases |
| 2 | 36 | TestQuickSortMtEph.rs | 4 | All 3 sort variants, edge, large, concurrent |
| 3 | 36 | TestQuickSortMtEphSlice.rs | 10 | All 3 sort variants, edge, large, pivot, concurrent |
| 4 | 36 | BenchQuickSort.rs | 1 | St vs Mt benchmarks, 3 sizes |

**RTT observations:**
- TestQuickSortStEph.rs: good coverage of the three sort variants plus edge cases
  (empty, single, sorted, reversed, pair).
- TestQuickSortMtEph.rs: adds large-input and concurrent-execution tests with barriers.
- TestQuickSortMtEphSlice.rs: the most thorough, but references `Chapter36MtEphSlice`
  module and `Chapter36MtSliceTrait` trait which do NOT exist in the current source.
  This file is behind `#![cfg(feature = "all_chapters")]` and will not compile.
  The module is `QuickSortMtEphSlice` and the trait is `QuickSortMtEphSliceTrait`.
  This test file is stale/broken.
- BenchQuickSort.rs: benchmarks St and Mt at n = 100, 500, 2000 using shuffled data.

### Proof Time Tests (PTT)

No PTTs exist for Chapter 36. None are needed -- the sorting functions have
straightforward callability patterns and no iterators.

## Phase 7: Gap Analysis

| # | Chap | File | Gap | Severity | Action |
|---|------|------|-----|----------|--------|
| 1 | 36 | QuickSortMtEph.rs | Sequential partition | Low | Could parallelize filter |
| 2 | 36 | QuickSortMtEphSlice.rs | Sequential partition | Low | Same as MtEph |
| 3 | 36 | TestQuickSortMtEphSlice.rs | Broken test references | Medium | Fix module/trait names |
| 4 | 36 | QuickSortStEph.rs | spec_leq/spec_median_of_three free fns | Low | Style warning [22] |
| 5 | 36 | QuickSortMtEph.rs | spec_leq/spec_median_of_three free fns | Low | Style warning [22] |
| 6 | 36 | QuickSortMtEphSlice.rs | 8 style warnings (free fn bounds) | Low | Style warnings [22,23] |

**Gap 1-2: Sequential partition.** APAS Algorithm 36.1 lines 6-8 use parallel filter.
The implementation uses a sequential while loop. This is a span-only impact; work is
unaffected. Parallelizing the partition would bring span from O(n) to O(lg n) per level,
matching APAS. This requires a verified parallel filter (Chapter 4 operations).

**Gap 3: Broken test file.** `TestQuickSortMtEphSlice.rs` imports from
`crate::Chap36::QuickSortMtEphSlice::Chapter36MtEphSlice` and uses trait name
`Chapter36MtSliceTrait`, function names `quick_sort_mt_first`, `pivot_mt_first`, etc.
None of these exist in the current source. The actual module is `QuickSortMtEphSlice`,
the trait is `QuickSortMtEphSliceTrait`, and the functions are `quick_sort_first`,
`median_of_three`, etc. The test is gated behind `all_chapters` feature so it does
not cause build failures, but it provides zero test coverage.

**Gap 4-6: Style warnings.** Free spec functions (`spec_leq`, `spec_median_of_three`,
`elements`) are flagged by the style checker as candidates for trait placement.
These are shared utility specs used by proof lemmas, so the warning is advisory.
The free fn type bound mismatches in MtEphSlice ([23]) are also advisory -- the
free fns correctly use narrower bounds than the trait requires.

## Phase 8: TOC Review

| # | Chap | File | TOC Present | Sections Used | Correct |
|---|------|------|:-----------:|---------------|:-------:|
| 1 | 36 | QuickSortStEph.rs | Yes | 1,2,3,6,7,8,9 | Yes |
| 2 | 36 | QuickSortMtEph.rs | Yes | 1,2,3,6,7,8,9 | Yes |
| 3 | 36 | QuickSortMtEphSlice.rs | Yes | 1,2,3,6,7,8,9 | Yes |

All three files follow the standard TOC ordering. Sections 4 (type definitions),
5 (view impls), 10 (iterators), and 11 (top level coarse locking) are correctly
omitted -- these modules define no new types and have no iterators.

## Summary

Chapter 36 is exemplary: 3 modules, 0 holes, 7 clean proof functions, 36 exec
functions with complete (strong) specs. The three APAS pivot strategies are all
implemented in both sequential and parallel forms. The key proof -- that
three-way-partitioning followed by independent sorting of the less-than and
greater-than portions produces the full sort -- is captured in
`lemma_partition_sort_concat`, which is duplicated across all three modules
per the standalone rule.

**Strengths:**
- All sorting specs use vstd's `sort_by`, the strongest possible specification
  (ensures both sortedness and permutation via multiset equality).
- Parallel recursion via `ParaPair!` with named closures and explicit `ensures`.
- All three pivot strategies faithfully implement APAS's descriptions.
- Proof of the main sorting lemma is clean and well-structured.

**Actionable items:**
1. Fix `TestQuickSortMtEphSlice.rs` to reference correct module/trait/function names.
2. Consider parallelizing the partition step for span improvement in Mt modules.
