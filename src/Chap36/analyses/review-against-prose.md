<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 36 — Quicksort: Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6

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
| 1 | First element | Always pick `a[0]` | Θ(n²) worst (sorted input) | Θ(n²) worst (St), Θ(n) worst (Mt) |
| 2 | Median of three | Median of first, middle, last | Θ(n lg n) for sorted; Θ(n²) worst | Same as first-element worst case |
| 3 | Random element | Uniformly random pivot | Θ(n lg n) expected | Θ(lg² n) expected |

### Cost Specifications

| # | Item | Work | Span |
|---|------|------|------|
| 1 | Algorithm 36.1 (random pivot) | Θ(n log n) expected | Θ(lg² n) expected (parallel filter partition: Θ(lg n) per level × Θ(lg n) levels) |
| 2 | Algorithm 36.1 (first element, sorted) | Θ(n²) | Θ(n) Mt / Θ(n²) St |
| 3 | Partition step (prose: parallel filter) | Θ(n) work | Θ(lg n) span |
| 4 | Partition step (implementation: sequential DNF) | Θ(n) work | Θ(n) span |

### Theorems / Properties

| # | Property | Description |
|---|----------|-------------|
| 1 | Correctness | Output is a permutation of input and is sorted. |
| 2 | Random pivot expected work | Θ(n lg n) — proven in the APAS analysis section via probabilistic argument. |
| 3 | Random pivot expected depth | Θ(lg n) — recursion tree depth with high probability. |

### Design Directives (from prompt)

| # | Directive | Status |
|---|-----------|--------|
| 1 | Use `ArraySeqStEph` for St variants | Implemented in `QuickSortStEph.rs` |
| 2 | Use `ArraySeqMtEph` for Mt variants | Implemented in `QuickSortMtEph.rs` |
| 3 | Three pivot strategies: first, median3, random | All three named functions exist in each file |
| 4 | Naming: `quick_sort_{St,Mt}_PIVOT` | **Deviated**: verusified files use `quick_sort_first` (no `_st_`/`_mt_` infix); only MtEphSlice uses `quick_sort_mt_*` |
| 5 | Mutex around ArraySeq members for Mt | `QuickSortMtEphSlice` uses `with_exclusive` (Mutex-wrapped); `QuickSortMtEph` wraps `Vec` in `ArraySeqMtEphS` |
| 6 | Benchmarks | Not implemented |

### Exercises

None specified in the prompt for Chapter 36.

## Phase 3: Algorithmic Analysis

### 3a. Function Inventory

**QuickSortStEph.rs** — inside `verus!`, fully verified:

| # | Function | Visibility | Spec | Spec Strength | Notes |
|---|----------|-----------|------|:------------:|-------|
| 1 | `sort_vec` | private | `ensures result@ =~= a.seq@.sort_by(spec_leq())`, `decreases a.spec_len()` | **strong** | Recursive non-mutating quicksort; comprehensive 9-step proof |
| 2 | `quick_sort_first` | pub | `ensures a.seq@ =~= old(a).seq@.sort_by(spec_leq())` | **strong** | Delegates to `sort_vec` |
| 3 | `quick_sort_median3` | pub | same as #2 | **strong** | Delegates to `sort_vec` — uses first-element pivot, NOT median-of-3 |
| 4 | `quick_sort_random` | pub | same as #2 | **strong** | Delegates to `sort_vec` — uses first-element pivot, NOT random |

**QuickSortMtEph.rs** — inside `verus!`, fully verified:

| # | Function | Visibility | Spec | Spec Strength | Notes |
|---|----------|-----------|------|:------------:|-------|
| 5 | `sort_vec` | private | same ensures/decreases as StEph | **strong** | Identical logic/proof to StEph |
| 6 | `quick_sort_first` | pub | `ensures a.seq@ =~= old(a).seq@.sort_by(spec_leq())` | **strong** | Delegates to `sort_vec` |
| 7 | `quick_sort_median3` | pub | same | **strong** | Same pivot as #6 |
| 8 | `quick_sort_random` | pub | same | **strong** | Same pivot as #6 |

**QuickSortMtEphSlice.rs** — outside `verus!`, unverified:

| # | Function | Visibility | Spec | Spec Strength | Notes |
|---|----------|-----------|------|:------------:|-------|
| 9 | `pivot_mt_first` | pub (trait) | APAS/Claude cost annotations only | **none** | Returns `self.nth(lo)` |
| 10 | `pivot_mt_median3` | pub (trait) | APAS/Claude cost annotations only | **none** | Median of first, middle, last |
| 11 | `pivot_mt_random` | pub (trait) | APAS/Claude cost annotations only | **none** | Uniform random via `rand::rng()` |
| 12 | `quick_sort_mt_first` | pub (trait) | APAS/Claude cost annotations only | **none** | Parallel via `thread::scope`, first-element pivot |
| 13 | `quick_sort_mt_median3` | pub (trait) | APAS/Claude cost annotations only | **none** | Parallel via `thread::scope`, median-of-3 pivot |
| 14 | `quick_sort_mt_random` | pub (trait) | APAS/Claude cost annotations only | **none** | Parallel via `thread::scope`, random pivot |
| 15 | `sort` (inner fn, ×3) | private | APAS/Claude cost annotations only | **none** | Slice-based recursive sort with DNF partition |

### 3b. Cost Analysis

| # | Module | Function | APAS Cost | Actual Cost | Match? |
|---|--------|----------|-----------|-------------|:------:|
| 1 | StEph | `sort_vec` | W Θ(n lg n) exp / Θ(n²) worst, S = W | W Θ(n²) worst (always first-element pivot), S = W (sequential) | **Partial** — work is correct for first-element, but function name claims median3/random variants without implementing them |
| 2 | StEph | `quick_sort_first` | W Θ(n lg n) exp / Θ(n²) worst, S = W | Same as sort_vec | Yes |
| 3 | StEph | `quick_sort_median3` | W Θ(n lg n) for sorted inputs | **W Θ(n²) for sorted inputs** (uses first-element pivot despite name) | **No** — pivot strategy mismatch |
| 4 | StEph | `quick_sort_random` | W Θ(n lg n) expected | **W Θ(n²) worst** (uses first-element pivot despite name) | **No** — pivot strategy mismatch |
| 5 | MtEph | all variants | W Θ(n lg n) exp / Θ(n²) worst, S Θ(lg² n) exp | W Θ(n²) worst, **S = W (sequential)** | **No** — not parallel despite Mt naming |
| 6 | MtEphSlice | `pivot_mt_*` | W Θ(1), S Θ(1) | W Θ(1), S Θ(1) | Yes |
| 7 | MtEphSlice | `quick_sort_mt_first` | W Θ(n²) worst, S Θ(n) worst | W Θ(n²) worst, S Θ(n) (parallel recursion, sequential partition) | Yes |
| 8 | MtEphSlice | `quick_sort_mt_median3` | W Θ(n lg n) for sorted, Θ(n²) worst, S Θ(lg² n) exp | W: yes, **S Θ(n) exp** (sequential partition) | **Partial** — span mismatch |
| 9 | MtEphSlice | `quick_sort_mt_random` | W Θ(n lg n) exp, S Θ(lg² n) exp | W Θ(n lg n) exp, **S Θ(n) exp** | **Partial** — span mismatch |

**Key cost disagreement:** All Mt implementations achieve **Θ(n) span** (not Θ(lg² n)) because the three-way partition is a sequential DNF loop. APAS Θ(lg² n) assumes parallel filter for partition (Θ(lg n) span per level × Θ(lg n) levels). To achieve APAS span, partition would need parallel filter/scan.

### 3c. Implementation Fidelity

| # | Module | Faithful to Prose? | Notes |
|---|--------|:------------------:|-------|
| 1 | QuickSortStEph | **Partial** | Correctly implements partition-sort-concat structure. Base cases for n=0 and n=1. Partition uses sequential while loop (DNF) instead of three parallel filters. All three public sort functions delegate to identical first-element-pivot code — **no actual median-of-3 or random pivot selection**. |
| 2 | QuickSortMtEph | **Poor** | Nearly identical code to StEph. Module header says "Uses sequential recursion inside verus! (parallel recursion would need external_body)." This means it is labeled Mt but performs **zero parallel execution**. Same first-element-pivot-only issue as StEph. |
| 3 | QuickSortMtEphSlice | **Good** | Genuinely parallel recursive calls via `thread::scope`. All three pivot strategies are actually implemented (first, median-of-3, random). In-place DNF partition is a standard practical optimization. Trait-based API matches prose naming convention (`quick_sort_mt_*`). |

**Summary of deviations from prose:**

| # | Deviation | Severity | Modules Affected |
|---|-----------|:--------:|-----------------|
| 1 | Sequential DNF partition instead of parallel filter | Medium | All three files |
| 2 | All three pivot variants use first-element pivot in verusified code | **High** | StEph, MtEph |
| 3 | Mt file uses sequential recursion (not parallel) | **High** | MtEph |
| 4 | In-place mutation instead of functional filter/append | Low | All (standard practice) |

### 3d. Spec Fidelity

| # | Module | Has verus! Specs? | Postcondition | Adequate? |
|---|--------|:-----------------:|---------------|:---------:|
| 1 | StEph | **Yes** | `result@ =~= a.seq@.sort_by(spec_leq())` | **Strong** — proves output is the `sort_by` of input. Uses `sorted_by`, `to_multiset` equivalence, and `lemma_sorted_unique` to establish result equals the canonical sort. |
| 2 | MtEph | **Yes** | Same as StEph | **Strong** — identical proof. |
| 3 | MtEphSlice | **No** | N/A | **None** — entirely outside `verus!`. Excluded from verification via `#[cfg(all(not(verus_keep_ghost), feature = "all_chapters"))]`. |

**Proof structure for StEph/MtEph `sort_vec`:**
The proof is a 9-step decomposition:
1. Establish `result@ == candidate` (where `candidate = sorted_left ++ equals ++ sorted_right`) by elementwise matching from loop invariants.
2. Connect `sort_vec` postconditions to `sort_by`.
3. Multiset equalities between sorted and unsorted partitions.
4. Length preservation through sorting.
5. Sorted-left elements are all `< pivot` (via multiset membership transfer).
6. Sorted-right elements are all `> pivot` (same technique).
7. Candidate is `sorted_by(leq)` — case analysis on element pairs across the three segments, using transitivity and reflexivity.
8. Candidate has same multiset as input `s` — via `lemma_multiset_commutative`.
9. Uniqueness: `lemma_sorted_unique` establishes `s.sort_by(leq) =~= candidate`.

This is a thorough and correct proof. The `TotalOrder` trait provides the necessary `le`, `reflexive`, `transitive` properties.

## Phase 4: Parallelism Review

### 4a. Classify Each Mt Function

| # | Module | Function | Classification | Mechanism | Notes |
|---|--------|----------|:--------------:|-----------|-------|
| 1 | MtEph | `sort_vec` | **Sequential** | While loops, recursive calls | No threading despite Mt module |
| 2 | MtEph | `quick_sort_first` | **Sequential** | Delegates to `sort_vec` | No threading |
| 3 | MtEph | `quick_sort_median3` | **Sequential** | Delegates to `sort_vec` | No threading |
| 4 | MtEph | `quick_sort_random` | **Sequential** | Delegates to `sort_vec` | No threading |
| 5 | MtEphSlice | `pivot_mt_first` | Sequential | O(1) | Parallelism irrelevant |
| 6 | MtEphSlice | `pivot_mt_median3` | Sequential | O(1) | Parallelism irrelevant |
| 7 | MtEphSlice | `pivot_mt_random` | Sequential | O(1) | Parallelism irrelevant |
| 8 | MtEphSlice | `quick_sort_mt_first` | **Parallel** | `thread::scope` spawns left, runs right in current thread | Genuinely parallel recursive calls |
| 9 | MtEphSlice | `quick_sort_mt_median3` | **Parallel** | Same as #8 | Genuinely parallel |
| 10 | MtEphSlice | `quick_sort_mt_random` | **Parallel** | Same as #8 | Genuinely parallel |

### 4b. Span Audit

| # | Module | Function | APAS Span | Actual Span | Match? | Root Cause |
|---|--------|----------|-----------|-------------|:------:|------------|
| 1 | MtEph | all `quick_sort_*` | Θ(lg² n) exp | **W (sequential)** | **No** | No parallel execution at all |
| 2 | MtEphSlice | `quick_sort_mt_first` | Θ(n) worst | Θ(n) | Yes | First-element pivot worst case |
| 3 | MtEphSlice | `quick_sort_mt_median3` | Θ(lg² n) exp | **Θ(n) exp** | **No** | Sequential DNF partition: S(n) = Θ(n) + S(3n/4) = Θ(n) |
| 4 | MtEphSlice | `quick_sort_mt_random` | Θ(lg² n) exp | **Θ(n) exp** | **No** | Same: sequential partition dominates |

### 4c. Parallelism Gap Table

| # | Module | Function | Prose Expects | Actually Parallel? | Span Gap | Fix |
|---|--------|----------|:------------:|:------------------:|----------|-----|
| 1 | MtEph | `sort_vec` | Parallel recursion | **No** | S = W vs S << W | Add `thread::scope` or `join` for recursive calls (requires `external_body` wrapper or HF scheduler) |
| 2 | MtEph | partition loop | Parallel filter | **No** | Θ(n) vs Θ(lg n) | Replace DNF with parallel filter (3 passes with parallel scan) |
| 3 | MtEphSlice | recursive calls | Parallel | **Yes** | None | — |
| 4 | MtEphSlice | partition loop | Parallel filter | **No** | Θ(n) vs Θ(lg n) | Replace DNF with parallel filter |

**MtEph is the critical gap:** The entire file is a sequential implementation wrapped in an Mt module name. It provides no parallelism advantage over StEph. The only genuinely parallel implementation is MtEphSlice.

## Phase 5: Runtime Test Review

### 5a. Test Inventory

| # | Source Module | RTT File | Tests | Status |
|---|-------------|----------|:-----:|--------|
| 1 | QuickSortStEph | `tests/Chap36/TestQuickSortStEph.rs` | 4 | **Likely broken** — references `quick_sort_st_first`, `pivot_st_first` methods that don't exist in current source |
| 2 | QuickSortMtEph | `tests/Chap36/TestQuickSortMtEph.rs` | 6 | **Likely broken** — references `Chapter36MtTrait` trait that doesn't exist in current source |
| 3 | QuickSortMtEphSlice | `tests/Chap36/TestQuickSortMtEphSlice.rs` | 12 | **Should work** — uses `Chapter36MtSliceTrait` which exists in source |

### 5b. Interface Mismatch Detail

The verusification of StEph and MtEph changed the public API:

| # | Old Interface (tests expect) | New Interface (source provides) | Breaking? |
|---|-------|------|:---:|
| 1 | Trait method `quick_sort_st_first(&mut self)` | Free fn `quick_sort_first(a: &mut ArraySeqStEphS<T>)` | **Yes** |
| 2 | Trait method `pivot_st_first(&self, lo, hi) -> T` | No pivot functions exist | **Yes** |
| 3 | Trait `Chapter36MtTrait` with `quick_sort_mt_first(&mut self)` | Free fn `quick_sort_first(a: &mut ArraySeqMtEphS<T>)` | **Yes** |
| 4 | Trait `Chapter36MtSliceTrait` | Trait `Chapter36MtSliceTrait` (unchanged) | No |

### 5c. Test Coverage (for working MtEphSlice tests)

| # | Test Name | What It Tests | Quality |
|---|-----------|--------------|:-------:|
| 1 | `quick_sort_slice_variants_produce_sorted_output` | All 3 pivot strategies on [5,3,1,4,2,2,3] | Good |
| 2 | `quick_sort_slice_edge_cases` | Empty, singleton, already-sorted, reversed, pair | Good |
| 3 | `quick_sort_slice_large_inputs` | 230 descending + 230 random elements | Good |
| 4 | `slice_pivot_strategies_match_expectations` | Pivot selection for first, median3, random | Good |
| 5 | `quick_sort_slice_small_inputs_use_shared_pivots` | Pivot + sort on 3- and 5-element inputs | Good |
| 6 | `slice_length_method` | Length on empty, single, multi | Basic infrastructure |
| 7 | `slice_nth_cloned_method` | Element access | Basic infrastructure |
| 8 | `slice_to_vec_method` | Conversion to Vec | Basic infrastructure |
| 9 | `slice_from_vec_constructor` | Construction from Vec | Basic infrastructure |
| 10 | `slice_clone_functionality` | Clone independence | Basic infrastructure |
| 11 | `slice_pivot_mt_first_edge_cases` | First pivot with subranges | Good |
| 12 | `slice_pivot_mt_median3_edge_cases` | Median3 with 3 elements and larger range | Good |
| 13 | `slice_pivot_mt_random_range_validation` | Random pivot in all valid subranges | Good |
| 14 | `slice_concurrent_sorting_stress_test` | 6 threads sorting concurrently | Good — tests thread safety |
| 15 | `slice_pivot_concurrent_access` | Concurrent pivot selection | Good |
| 16 | `slice_large_data_handling` | 10,000 reverse-sorted elements | Good — stress test |

### 5d. Missing Tests

| # | Priority | Test Needed | Reason |
|---|:--------:|------------|--------|
| 1 | **Critical** | Fix StEph tests to match new free-function API | Tests reference nonexistent methods |
| 2 | **Critical** | Fix MtEph tests to match new free-function API | Tests reference nonexistent trait |
| 3 | High | StEph: all-duplicates input | Exercises the equals partition exclusively |
| 4 | High | MtEph: concurrent execution test (once API fixed) | Moot until MtEph is actually parallel |
| 5 | Medium | StEph/MtEph: large inputs (n=1000+) | Stress test for recursive depth |
| 6 | Low | Cross-module consistency: StEph vs MtEph vs MtEphSlice produce same multiset | Validates all implementations agree |

## Phase 6: PTT Review

No PTT files exist in `rust_verify_test/tests/Chap36/`.

### Loop and Iterator Analysis

| # | Module | Construct | Location | PTT Candidate? |
|---|--------|-----------|----------|:--------------:|
| 1 | StEph | `while i < n` (partition loop) | `sort_vec` lines 86–135 | Yes — loop invariant maintains multiset equality, could test invariant holds at boundaries |
| 2 | StEph | `while j < sl` (copy sorted_left) | `sort_vec` lines 181–196 | Low value — simple copy loop |
| 3 | StEph | `while j < el` (copy equals) | `sort_vec` lines 199–216 | Low value |
| 4 | StEph | `while j < sr` (copy sorted_right) | `sort_vec` lines 219–238 | Low value |
| 5 | MtEph | Same 4 loops as StEph | `sort_vec` | Same assessment |

**Recommendation:** PTTs for the partition loop invariant (row 1) would be valuable — the multiset-preserving invariant is the most complex loop in the proof. The concatenation loops (rows 2–4) are straightforward copy loops where PTTs add little value.

No iterators or `for` loops exist in the verusified files.

## Phase 7: Gap Analysis

### 7a. Prose Items With No Code

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Parallel filter partition | **Not implemented** | All implementations use sequential DNF. Prose: 3 parallel filters with Θ(lg n) span. |
| 2 | Parallel recursive calls (Mt) | **Missing in MtEph** | `QuickSortMtEph` uses sequential recursion. Only `QuickSortMtEphSlice` is parallel. |
| 3 | Median-of-3 pivot (verusified) | **Not implemented** | Verusified StEph/MtEph claim `quick_sort_median3` but use first-element pivot. |
| 4 | Random pivot (verusified) | **Not implemented** | Verusified StEph/MtEph claim `quick_sort_random` but use first-element pivot. |
| 5 | Benchmarks | **Not implemented** | Prompt mentions benchmarks; none exist. |
| 6 | Expected cost analysis (probabilistic) | **Not formalized** | APAS proves Θ(n lg n) expected work for random pivot; no spec-level cost model exists. |

### 7b. Code With No Prose Counterpart

| # | Item | Purpose |
|---|------|---------|
| 1 | `QuickSortMtEphSlice` module | Slice-based variant using `with_exclusive` (Mutex) — implementation optimization not in the prose. |
| 2 | `pivot_mt_*` standalone functions (MtEphSlice) | The prose leaves pivot selection "underspecified"; these are named implementations. |
| 3 | `Chapter36MtSliceTrait` trait | Trait abstraction for the slice-based API — not in prose. |
| 4 | Base case n=1 with singleton proof | Prose only specifies base case for `|a| = 0`. Implementation adds n=1 for efficiency with a one-element proof. |

### 7c. Critical Observation: Pivot Strategy Gap

The verusified code (StEph and MtEph) provides three public functions named after different pivot strategies, but all three delegate to the same `sort_vec` which always uses `*a.nth(0)` (first-element pivot). The module header acknowledges this: "All three pivot variants use first-element pivot (correctness is pivot-independent)."

This is **correct for the formal proof** — the `sort_by` postcondition is pivot-independent. However, it means:
- `quick_sort_median3` does NOT use median-of-3 pivot selection.
- `quick_sort_random` does NOT use random pivot selection.
- The naming is misleading: callers would expect different runtime behavior from these functions.
- The cost analysis differs: first-element pivot has Θ(n²) worst case on sorted input; median-of-3 would be Θ(n lg n) on sorted input.

Only `QuickSortMtEphSlice` actually implements all three pivot strategies distinctly.

## Phase 8: TOC Review

### 8a. TOC Standard Compliance

| # | File | Has TOC? | Correct Sections? | Notes |
|---|------|:--------:|:-----------------:|-------|
| 1 | QuickSortStEph.rs | **Yes** | Yes — lists sections 1, 2, 3, 9 | Correct: module, imports, broadcast use, impls |
| 2 | QuickSortMtEph.rs | **Yes** | Yes — lists sections 1, 2, 3, 9 | Correct: same structure as StEph |
| 3 | QuickSortMtEphSlice.rs | **No** | N/A | No `verus!` block — TOC not applicable |

### 8b. In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | QuickSortStEph.rs | - | - | - | - | - | - | - | - | - |
| 2 | QuickSortMtEph.rs | - | - | - | - | - | - | - | - | - |
| 3 | QuickSortMtEphSlice.rs | - | - | - | - | - | - | - | - | `Chapter36MtSliceTrait` trait + impl outside verus! |

No derive impls in any file. QuickSortStEph and QuickSortMtEph are purely algorithmic modules inside `verus!`. QuickSortMtEphSlice is entirely outside `verus!` (gated by `#[cfg(all(not(verus_keep_ghost), feature = "all_chapters"))]` in `lib.rs`).

### 8c. Module Header Compliance

| # | File | Copyright? | Module Doc? | Compliant? |
|---|------|:----------:|:-----------:|:----------:|
| 1 | QuickSortStEph.rs | Yes (`//!`) | Yes — describes chapter, data structure, proof strategy | Yes |
| 2 | QuickSortMtEph.rs | Yes (`//!`) | Yes — notes sequential recursion limitation | Yes |
| 3 | QuickSortMtEphSlice.rs | Yes (`//!`) | Brief — just copyright + chapter description | Minimal |

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap36/

✓ QuickSortStEph.rs
✓ QuickSortMtEph.rs
✓ QuickSortMtEphSlice.rs

Modules: 3 clean, 0 holed
Proof Functions: 0 total
Holes Found: 0
```

All 3 files are clean. The verusified files (StEph, MtEph) contain no `assume`, `admit`, or `external_body`. The MtEphSlice file is entirely outside `verus!` so has no proof obligations.

## Spec Strength Summary

| Classification | Count |
|:--------------:|:-----:|
| strong | 8 |
| partial | 0 |
| weak | 0 |
| none | 7 |

The 8 strong specs are the 4 verusified functions in StEph and the 4 in MtEph (each: `sort_vec` + 3 public wrappers). The 7 with no spec are the MtEphSlice functions (3 pivots + 3 sorts + trait definition overhead).

## Overall Assessment

**Maturity: Partially verified.** The StEph and MtEph files have been fully verusified since the last review (2026-02-13), with strong sort-by postconditions and clean proofs. This is a significant improvement. However, several substantial gaps remain:

| # | Issue | Severity |
|---|-------|:--------:|
| 1 | **MtEph is sequential** — labeled Mt but performs no parallel execution | High |
| 2 | **Pivot strategies are fake in verusified code** — all three variants use first-element pivot | High |
| 3 | **Tests are broken** — StEph and MtEph tests reference old trait-based API that no longer exists | High |
| 4 | **MtEphSlice is unverified** — the only genuinely parallel implementation has no Verus specs | Medium |
| 5 | **Sequential partition in all Mt variants** — Θ(n) span vs APAS Θ(lg n) per level | Medium |
| 6 | **No benchmarks** — prompt explicitly requests them | Low |

## Review TODOs

| # | Priority | Action | Details |
|---|:--------:|--------|---------|
| 1 | **Critical** | Fix StEph runtime tests | Update `TestQuickSortStEph.rs` to use free-function API: `quick_sort_first(&mut first)` instead of `first.quick_sort_st_first()`. Remove `pivot_st_*` test calls (functions no longer exist). |
| 2 | **Critical** | Fix MtEph runtime tests | Update `TestQuickSortMtEph.rs` to use free-function API. Remove `Chapter36MtTrait` references (trait no longer exists). Remove `pivot_mt_*` test calls. |
| 3 | **High** | Implement actual pivot strategies in verusified code | Either: (a) add median-of-3 and random pivot selection to `sort_vec` (parametrize on pivot strategy), or (b) rename `quick_sort_median3`/`quick_sort_random` to clearly indicate they use first-element pivot (e.g., just expose a single `quick_sort`). |
| 4 | **High** | Make MtEph actually parallel | Add `thread::scope` or `join` for the two recursive calls in `sort_vec`. This requires an `external_body` wrapper at the spawn boundary per project conventions. Alternatively, use HF Scheduler `join`. |
| 5 | **Medium** | Add PTTs for partition loop invariant | The multiset-preserving partition loop is the most complex verified loop — a PTT would validate the invariant holds at boundary conditions. |
| 6 | **Medium** | Consider verusifying MtEphSlice | Currently the only genuinely parallel implementation has no specs. At minimum, add `external_body` wrapper with sort postcondition. |
| 7 | **Low** | Parallelize partition step | Replace sequential DNF with parallel filter to achieve APAS Θ(lg n) per-level span. This is a significant implementation effort. |
| 8 | **Low** | Add benchmarks | Compare St vs Mt vs MtEphSlice, across pivot strategies and input distributions (sorted, reversed, random). |

## Date and Reviewer

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6
