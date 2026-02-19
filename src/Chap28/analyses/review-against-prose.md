<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 28: Maximum Contiguous Subsequence Sum — Review Against Prose

**Date**: 2026-02-17
**Last mechanical audit:** 2026-02-19 — return variable renames, IntoIterator additions, eq→equal/clone→cloned renames; no functional changes.
**Reviewer**: Claude-Opus-4.6
**Prose Source**: `prompts/Chap28.txt` (Chapter 28 of APAS)

## Phase 1: Inventory

### 1a. Source Files

| # | File | Algorithm | St/Mt | Verusified? | Description |
|---|------|-----------|-------|:-----------:|-------------|
| 1 | `MCSSSpec.rs` | — | — | **Yes** | Shared spec definitions and lemmas |
| 2 | `MaxContigSubSumBruteStEph.rs` | 28.8 | St | **Yes** | Brute force MCSS, Θ(n³) work |
| 3 | `MaxContigSubSumReducedStEph.rs` | 28.13 | St | **Yes** | Reduced force via MCSSS, Θ(n²) work |
| 4 | `MaxContigSubSumOptStEph.rs` | 28.16 | St | **Yes** | Optimal prefix-sum based, Θ(n) work |
| 5 | `MaxContigSubSumIterStEph.rs` | 28.15 | St | **Yes** | Kadane's iterative, Θ(n) work/span |
| 6 | `MaxContigSubSumDivConStEph.rs` | 28.17 | St | **Yes** | Simple divide-and-conquer, Θ(n log n) work |
| 7 | `MaxContigSubSumReducedMcsseStEph.rs` | 28.14 | St | **Yes** | Reduction to MCSSE, O(n²) work |
| 8 | `MaxContigSubSumDivConOptStEph.rs` | 28.19 | St | **Yes** | Strengthened D&C, Θ(n) work (APAS) |
| 9 | `MaxContigSubSumOptMtEph.rs` | 28.16 | Mt | **Yes** | Verified seq. under `verus_keep_ghost`; parallel scan/reduce at runtime |
| 10 | `MaxContigSubSumDivConMtEph.rs` | 28.17 | Mt | **Yes** | Verified seq. under `verus_keep_ghost`; parallel `ParaPair!` at runtime |
| 11 | `MaxContigSubSumDivConOptMtEph.rs` | 28.19 | Mt | **Yes** | Verified seq. under `verus_keep_ghost`; parallel `ParaPair!` at runtime |

### 1b. Spec Module (`MCSSSpec.rs`)

| # | Definition/Lemma | Type | Purpose |
|---|-----------------|------|---------|
| 1 | `spec_range_sum` | spec fn | Sum of elements s[lo..hi) as unbounded int |
| 2 | `is_mcss_of` | spec fn | Predicate: m is the MCSS of sequence s |
| 3 | `spec_mcss` | spec fn | Computes MCSS as Option<int> (None for empty) |
| 4 | `sums_fit_i32` | spec fn | All partial sums fit in i32 (precondition) |
| 5 | `spec_prefix_sum` | spec fn | Prefix sum: range_sum(s, 0, k) |
| 6 | `spec_min_prefix_sum` | spec fn | Min of prefix sums over 0..=k |
| 7 | `is_max_suffix_sum` | spec fn | Predicate: m is the max suffix sum |
| 8 | `is_max_prefix_sum` | spec fn | Predicate: m is the max prefix sum |
| 9 | `lemma_range_sum_snoc` | proof fn | Extending range sum by one element |
| 10 | `lemma_range_sum_single` | proof fn | Range sum of a single element |
| 11 | `lemma_range_sum_empty` | proof fn | Empty range sums to 0 |
| 12 | `lemma_range_sum_split` | proof fn | Splitting range sum at midpoint |
| 13 | `lemma_range_sum_via_prefix` | proof fn | Range sum = prefix difference |
| 14 | `lemma_min_prefix_sum_is_min` | proof fn | Min prefix sum is a lower bound |
| 15 | `lemma_min_prefix_sum_achieved` | proof fn | Min prefix sum is achieved |
| 16 | `lemma_range_sum_subseq` | proof fn | Range sum transfer across subsequences |
| 17 | `lemma_crossing_decompose` | proof fn | Crossing sum = left + right |
| 18 | `lemma_sums_fit_subseq` | proof fn | sums_fit_i32 propagates to subsequences |

### 1c. Kadane Specs (`MaxContigSubSumIterStEph.rs`)

| # | Definition/Lemma | Type | Purpose |
|---|-----------------|------|---------|
| 1 | `spec_max_ending_at` | spec fn | Kadane recurrence: max sum of contiguous subsequence ending at position j |
| 2 | `lemma_max_ending_at_is_max` | proof fn | `spec_max_ending_at(s, j)` is an upper bound on all sums ending at j+1 |
| 3 | `lemma_max_ending_at_achieved` | proof fn | `spec_max_ending_at(s, j)` is achieved by some contiguous range |

### 1d. Function Table (Verified Modules)

| # | Function | File | Role | Spec Strength |
|---|----------|------|------|:---:|
| 1 | `max_with_neginf` | BruteStEph | helper | strong |
| 2 | `max_contig_sub_sum_brute` | BruteStEph | trait+impl | strong |
| 3 | `max_with_neginf` | ReducedStEph | helper | strong |
| 4 | `max_contig_sub_sum_reduced` | ReducedStEph | trait+impl | strong |
| 5 | `max_contig_sub_sum_opt` | OptStEph | trait+impl | strong |
| 6 | `lemma_prefix_opt_is_mcss` | OptStEph | proof fn | strong |
| 7 | `max_with_neginf` | IterStEph | helper | strong |
| 8 | `max_contig_sub_sum_iter` | IterStEph | trait+impl | strong |
| 9 | `max_with_neginf` | ReducedMcsseStEph | helper | strong |
| 10 | `max_contig_sub_sum_reduced_mcsse` | ReducedMcsseStEph | trait+impl | strong |
| 11 | `max_with_neginf` | DivConStEph | helper | strong |
| 12 | `max_suffix_sum` | DivConStEph | helper (Alg 28.12) | strong |
| 13 | `max_prefix_sum` | DivConStEph | helper (Alg 28.11) | strong |
| 14 | `lemma_divcon_combine` | DivConStEph | proof fn | strong |
| 15 | `max_contig_sub_sum_divcon` | DivConStEph | trait+impl | strong |
| 16 | `max_with_neginf` | DivConOptStEph | helper | strong |
| 17 | `lemma_strength_combine` | DivConOptStEph | proof fn | strong |
| 18 | `max_contig_sub_sum_aux` | DivConOptStEph | recursive helper | strong |
| 19 | `max_contig_sub_sum_divcon_opt` | DivConOptStEph | trait+impl | strong |
| 20 | `max_contig_sub_sum_opt_mt` | OptMtEph | trait+impl | strong |
| 21 | `max_with_neginf` | DivConMtEph | helper | strong |
| 22 | `max_suffix_sum` | DivConMtEph | helper (Alg 28.12) | strong |
| 23 | `max_prefix_sum` | DivConMtEph | helper (Alg 28.11) | strong |
| 24 | `max_contig_sub_sum_divcon_mt` | DivConMtEph | trait+impl | strong |
| 25 | `max_with_neginf` | DivConOptMtEph | helper | strong |
| 26 | `max_contig_sub_sum_aux` | DivConOptMtEph | recursive helper | strong |
| 27 | `max_contig_sub_sum_divcon_opt_mt` | DivConOptMtEph | trait+impl | strong |

**All 27 verified functions have strong specifications.**

## Phase 2: Prose Inventory

### Definitions

| # | Ref | Name | Implemented |
|---|-----|------|:-----------:|
| 1 | Def 28.1 | Subsequence | Implicit |
| 2 | Def 28.2 | Contiguous Subsequence | Implicit |
| 3 | Def 28.3 | Maximum Contiguous Subsequence (MCS) Problem | Not as standalone |
| 4 | Def 28.4 | Maximum Contiguous Subsequence Sum (MCSS) Problem | **Yes** — `is_mcss_of` in `MCSSSpec` |
| 5 | Def 28.9 | MCSSS (max sum with start) | **Yes** — `is_max_prefix_sum` in `MCSSSpec` |
| 6 | Def 28.10 | MCSSE (max sum with ending) | **Yes** — `is_max_suffix_sum` in `MCSSSpec` |

### Algorithms

| # | Ref | Name | Work | Span | Implemented | Verified? |
|---|-----|------|------|------|:-----------:|:---------:|
| 1 | 28.5 | MCSS: Brutest Force | Unbounded | — | No | — |
| 2 | 28.6 | MCS: Brute Force | Θ(n³) | Θ(log n) | No | — |
| 3 | 28.7 | MCSS via MCS | Θ(n³) | Θ(log n) | No | — |
| 4 | 28.8 | MCSS: Brute Force Strengthened | Θ(n³) | Θ(log n) | **Yes** | **Yes** |
| 5 | 28.11 | MCSSS Optimal | Θ(n) | Θ(log n) | **Yes** — `max_prefix_sum` in DivCon{St,Mt}Eph | **Yes** (St+Mt) |
| 6 | 28.12 | MCSSE Optimal | Θ(n) | Θ(log n) | **Yes** — `max_suffix_sum` in DivCon{St,Mt}Eph | **Yes** (St+Mt) |
| 7 | 28.13 | MCSS: Reduced Force | Θ(n²) | Θ(log n) | **Yes** | **Yes** |
| 8 | 28.14 | MCSS by Reduction to MCSSE | O(n²) | O(log n) | **Yes** | **Yes** |
| 9 | 28.15 | MCSS with Iteration (Kadane) | Θ(n) | Θ(n) | **Yes** | **Yes** |
| 10 | 28.16 | MCSS: Work Optimal Low Span | Θ(n) | Θ(log n) | **Yes** | **Yes** (St+Mt) |
| 11 | 28.17 | Simple D&C for MCSS | Θ(n log n) | Θ(log² n) | **Yes** | **Yes** (St+Mt) |
| 12 | 28.18 | bestAcross (max spanning cut) | Θ(n) | Θ(log n) | **Yes** — inline in `lemma_divcon_combine` / `lemma_strength_combine` | **Yes** (St+Mt) |
| 13 | 28.19 | Linear Work D&C MCSS | Θ(n) | Θ(log² n) | **Yes** | **Yes** (St+Mt) |

### Theorems and Proofs

| # | Ref | Name | Type | Mechanized? |
|---|-----|------|------|:-----------:|
| 1 | Thm 28.2 | Correctness of MCSSDC | Text proof by strong induction | **Yes** — `lemma_divcon_combine` and recursive ensures |
| 2 | Thm 28.3 | Work recurrence W(n)=2W(n/2)+kn | Substitution method | No (cost analysis) |
| 3 | Thm 28.4 | Work recurrence W(n)=2W(n/2)+k·lg n | Substitution method | No (cost analysis) |
| 4 | Lemma 28.1 | MCSSE Extension | Text proof | **Yes** — `lemma_range_sum_snoc` / `lemma_range_sum_split` |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Function | File | APAS Cost | Claude-Opus-4.6 Cost | Match? |
|---|----------|------|-----------|-----------------------|:------:|
| 1 | `max_contig_sub_sum_brute` | BruteStEph | W Θ(n³), S Θ(log n) | W Θ(n³), S Θ(n³) | Work ✓, Span ✗ (sequential) |
| 2 | `max_contig_sub_sum_reduced` | ReducedStEph | W Θ(n²), S Θ(log n) | W Θ(n²), S Θ(n²) | Work ✓, Span ✗ (sequential) |
| 3 | `max_contig_sub_sum_opt` | OptStEph | W Θ(n), S Θ(log n) | W Θ(n), S Θ(n) | Work ✓, Span ✗ (sequential) |
| 4 | `max_contig_sub_sum_iter` | IterStEph | W Θ(n), S Θ(n) | W Θ(n), S Θ(n) | Work ✓, Span ✓ |
| 5 | `max_contig_sub_sum_reduced_mcsse` | ReducedMcsseStEph | W O(n²), S O(log n) | W O(n²), S O(n²) | Work ✓, Span ✗ (sequential) |
| 6 | `max_contig_sub_sum_divcon` | DivConStEph | W Θ(n log n), S Θ(log² n) | W Θ(n log n), S Θ(n log n) | Work ✓, Span ✗ (sequential) |
| 7 | `max_contig_sub_sum_divcon_opt` | DivConOptStEph | W Θ(n), S Θ(log² n) | W Θ(n log n), S Θ(n) | Work ✗ (`subseq_copy`), Span ✗ |

**Key discrepancies**:

1. **St span mismatches** (rows 1-3, 5-6): Expected — St modules are sequential; APAS span assumes parallel `scan`/`reduce`/`tabulate`.
2. **DivConOpt work mismatch** (row 7): `subseq_copy` costs O(n) per D&C level instead of APAS's O(1) `splitMid`. This inflates work from Θ(n) to Θ(n log n). See §3b.
3. **Kadane's** (row 4) is the only algorithm where both work and span match APAS, since it is inherently sequential (Θ(n) work, Θ(n) span).

### 3b. Implementation Fidelity

| # | Issue | Severity | Details |
|---|-------|----------|---------|
| 1 | `subseq_copy` vs `splitMid` | **Medium** | D&C modules (28.17 St/Mt, 28.19 St/Mt) use `subseq_copy` O(n) per level instead of APAS's O(1) `splitMid`. This degrades DivConOpt from Θ(n) to Θ(n log n) work and DivCon from Θ(n log n) to Θ(n log n) (no change — dominated by other work). Chap19 `ArraySeqMtEphSlice` provides an O(1) `slice()` operation that could serve as `splitMid`. |
| 2 | `-∞` representation | Low | `Option<i32>` for results (None = −∞). Clean spec via `is_mcss_of`. |

### 3c. Spec Fidelity

**All 10 algorithmic modules (7 St + 3 Mt) are fully verified** with strong specifications:

- `spec fn spec_range_sum`: mathematical definition of contiguous subsequence sum (Definition 28.4)
- `spec fn is_mcss_of`: correctness predicate — both achievability and maximality
- `spec fn sums_fit_i32`: overflow-freedom precondition
- `spec fn spec_max_ending_at`: Kadane recurrence for max sum ending at a position
- `spec fn is_max_prefix_sum` / `is_max_suffix_sum`: helper specs for D&C combine
- All main functions have `ensures is_mcss_of(a.seq@, result.unwrap() as int)`
- 10 proof lemmas in MCSSSpec + 2 Kadane lemmas in IterStEph + 3 combine lemmas in DivCon modules = 15 total proof functions
- Mt modules reuse StEph proof lemmas via pub imports (zero duplication of proof logic)

## Phase 4: Parallelism Review

All three Mt modules are **fully verusified** using a cfg-gated dual-implementation architecture:
- Under `verus_keep_ghost`: verified sequential implementation (same algorithm, loop-based)
- Under normal Rust compilation: parallel implementation using Chap19 primitives

The trait + specs live inside `verus!{}` and are visible to both impls.

### 4a. Parallel Mechanisms vs. Prose

| # | Module | Prose Parallel Structure | Runtime Parallel Impl | Parallelism Match? |
|---|--------|-------------------------|----------------------|:------------------:|
| 1 | `OptMtEph` (28.16) | `scan '+' 0 a` → `scan min ∞ c` → `tabulate` → `reduce max` | `scan` → `singleton+append` → `scan` (via `tabulate+reduce`) → `tabulate` → `reduce` | ✓ Full |
| 2 | `DivConMtEph` (28.17) | `(MCSSDC b ‖ MCSSDC c)` then `bestAcross(b,c)` = `MCSSE(left) ‖ MCSSS(right)`, each using `scan+reduce` | `ParaPair!(recurse left, recurse right)` then `ParaPair!(max_suffix_sum_par, max_prefix_sum_par)`, each using `scan+reduce` | ✓ Full |
| 3 | `DivConOptMtEph` (28.19) | `(MCSSDCAux b ‖ MCSSDCAux c)` then O(1) combine of 4-tuple | `ParaPair!(aux left, aux right)` then O(1) combine of `(mcss, prefix, suffix, total)` | ✓ Full |

### 4b. Verified Impl (Verus) vs. Runtime Impl

| # | Module | Verified Impl (`verus_keep_ghost`) | Runtime Impl (`not(verus_keep_ghost)`) |
|---|--------|-----------------------------------|---------------------------------------|
| 1 | `OptMtEph` | 3 sequential while loops (prefix sums, min prefixes, max scan) | Chap19 `scan` + `reduce` + `tabulate` parallel primitives |
| 2 | `DivConMtEph` | Sequential recursive D&C; loop-based `max_suffix_sum` + `max_prefix_sum` | `ParaPair!` for recursion + `ParaPair!` for suffix/prefix; `scan`+`reduce` in helpers |
| 3 | `DivConOptMtEph` | Sequential recursive strengthened D&C returning 4-tuple | `ParaPair!` for recursion; O(1) combine |

### 4c. Architecture Notes

- Each Mt module defines one trait with `requires`/`ensures` inside `verus!{}`.
- The verified impl (`#[cfg(verus_keep_ghost)]`) proves the spec using sequential code.
- The parallel impl (`#[cfg(not(verus_keep_ghost))]`) provides runtime performance.
- Proof lemmas are reused from StEph modules via pub imports — zero proof duplication.
- D&C Mt modules require `obeys_feq_clone::<i32>()` as a precondition (for `subseq_copy`).

## Phase 5: Runtime Test Review

| # | Test File | Module Tested | Tests | All Pass? |
|---|-----------|--------------|:-----:|:---------:|
| 1 | `TestMaxContigSubSumBruteStEph.rs` | BruteStEph | 9 | ✓ |
| 2 | `TestMaxContigSubSumReducedStEph.rs` | ReducedStEph | 5 | ✓ |
| 3 | `TestMaxContigSubSumOptStEph.rs` | OptStEph | 6 | ✓ |
| 4 | `TestMaxContigSubSumIterStEph.rs` | IterStEph | 9 | ✓ |
| 5 | `TestMaxContigSubSumReducedMcsseStEph.rs` | ReducedMcsseStEph | 9 | ✓ |
| 6 | `TestMaxContigSubSumDivConStEph.rs` | DivConStEph | 6 | ✓ |
| 7 | `TestMaxContigSubSumDivConOptStEph.rs` | DivConOptStEph | 6 | ✓ |
| 8 | `TestMaxContigSubSumOptMtEph.rs` | OptMtEph | 6 | ✓ |
| 9 | `TestMaxContigSubSumDivConMtEph.rs` | DivConMtEph | 6 | ✓ |
| 10 | `TestMaxContigSubSumDivConOptMtEph.rs` | DivConOptMtEph | 6 | ✓ |

**Total tests**: 68 across 10 modules. All pass.

## Phase 6: PTT Review

No PTTs needed — the Verus verification subsumes all proof-time testing. The 15 proof functions and all loop invariants verify successfully.

## Phase 7: Gap Analysis

### 7a. Prose Items Without Implementation

All significant MCSS algorithms from the prose are implemented and verified. The only unimplemented
algorithms (28.5, 28.6, 28.7) are pedagogical stepping stones superseded by the strengthened versions.

### 7b. Code Without Prose Counterpart

| # | Item | Notes |
|---|------|-------|
| 1 | `MCSSSpec.rs` | Spec module — mathematical definitions from prose, formalized. |

## Proof Holes Summary

```
✓ MCSSSpec.rs — 10 clean proof functions
✓ MaxContigSubSumBruteStEph.rs
✓ MaxContigSubSumDivConMtEph.rs
✓ MaxContigSubSumDivConOptMtEph.rs
✓ MaxContigSubSumDivConOptStEph.rs — 1 clean proof function
✓ MaxContigSubSumDivConStEph.rs — 1 clean proof function
✓ MaxContigSubSumIterStEph.rs — 2 clean proof functions
✓ MaxContigSubSumReducedMcsseStEph.rs
✓ MaxContigSubSumOptMtEph.rs
✓ MaxContigSubSumOptStEph.rs — 1 clean proof function
✓ MaxContigSubSumReducedStEph.rs

Modules: 11 clean, 0 holed
Proof Functions: 15 clean, 0 holed
Proof holes: 0
```

## Spec Strength Summary

| Classification | Count |
|:---:|:---:|
| strong | 27 |
| partial | 0 |
| weak | 0 |
| none | 0 |

## Verification Statistics

```
verification results:: 1602 verified, 0 errors
```

## Overall Assessment

**Chapter 28 is fully verusified — all 11 modules (7 St + 3 Mt + 1 spec) verified with strong specifications and zero proof holes.**

### Strengths

1. **Strong mathematical specifications**: `is_mcss_of` captures both achievability and maximality.
   All 10 algorithmic modules prove this spec (27 verified functions total).
2. **Rich spec library**: 18 spec/proof definitions in `MCSSSpec.rs` + 3 Kadane specs in `IterStEph`
   forming a reusable foundation.
3. **Zero proof holes**: No `assume()`, `admit()`, or `external_body` in any module.
4. **Comprehensive runtime tests**: 68 tests across 10 modules, all passing.
5. **Complete algorithmic coverage**: All 7 significant MCSS algorithms from the prose are implemented
   and verified (28.8, 28.13, 28.14, 28.15, 28.16, 28.17, 28.19).
6. **Mechanized correctness theorems**: Thm 28.2 (D&C correctness) and Lemma 28.1
   (MCSSE extension) are mechanized as proof functions.
7. **Verified Mt modules**: All 3 parallel modules are verified using cfg-gated dual implementations —
   verified sequential under `verus_keep_ghost`, parallel at runtime. Proof lemmas reused from StEph
   modules via pub imports.

### Remaining Work

1. **`subseq_copy` → `slice`**: D&C modules use `subseq_copy` (O(n) per level) instead of APAS's
   O(1) `splitMid`. Chap19's `ArraySeqMtEphSlice::slice()` provides O(1) splitting via
   `Arc<Mutex<Box<[T]>>>` range views, but it has no Verus specs. Migrating D&C modules to use
   `ArraySeqMtEphSliceS` would restore DivConOpt to Θ(n) work, matching the prose.
