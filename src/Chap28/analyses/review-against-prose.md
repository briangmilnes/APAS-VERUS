<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 28: Maximum Contiguous Subsequence Sum — Review Against Prose

**Date**: 2026-02-13
**Reviewer**: Claude-Opus-4.6
**Prose Source**: `prompts/Chap28.txt` (Chapter 28 of APAS)

## Phase 1: Inventory

### 1a. Source Files

| # | File | Algorithm | St/Mt | Description |
|---|------|-----------|-------|-------------|
| 1 | `MaxContigSubSumBruteStEph.rs` | 28.8 | St | Brute force MCSS, Θ(n³) work |
| 2 | `MaxContigSubSumReducedStEph.rs` | 28.13 | St | Reduced force via MCSSS, Θ(n²) work |
| 3 | `MaxContigSubSumOptStEph.rs` | 28.16 | St | Optimal scan-based, Θ(n) work |
| 4 | `MaxContigSubSumOptMtEph.rs` | 28.16 | Mt | Parallel optimal scan-based, Θ(n) work |
| 5 | `MaxContigSubSumDivConStEph.rs` | 28.17 | St | Simple divide-and-conquer, Θ(n log n) work |
| 6 | `MaxContigSubSumDivConMtEph.rs` | 28.17 | Mt | Parallel simple D&C, Θ(n log n) work |
| 7 | `MaxContigSubSumDivConOptStEph.rs` | 28.19 | St | Strengthened D&C, Θ(n) work (APAS) |
| 8 | `MaxContigSubSumDivConOptMtEph.rs` | 28.19 | Mt | Parallel strengthened D&C, Θ(n) work (APAS) |

### 1b. Function Table

All functions are plain Rust (outside `verus!`). None have `requires`/`ensures`.

| # | Function | File | Role | Spec Strength |
|---|----------|------|------|:---:|
| 1 | `max_with_neginf` | BruteStEph | helper | none |
| 2 | `max_contig_sub_sum_brute` | BruteStEph | trait+impl | none |
| 3 | `max_with_neginf` | DivConMtEph | helper | none |
| 4 | `max_suffix_sum` | DivConMtEph | helper (Alg 28.12) | none |
| 5 | `max_prefix_sum` | DivConMtEph | helper (Alg 28.11) | none |
| 6 | `max_contig_sub_sum_divcon_mt` | DivConMtEph | trait+impl | none |
| 7 | `max_with_neginf` | DivConOptMtEph | helper | none |
| 8 | `max_contig_sub_sum_aux_mt` | DivConOptMtEph | helper (Alg 28.19 aux) | none |
| 9 | `max_contig_sub_sum_divcon_opt_mt` | DivConOptMtEph | trait+impl | none |
| 10 | `max_with_neginf` | DivConOptStEph | helper | none |
| 11 | `max_contig_sub_sum_aux` | DivConOptStEph | helper (Alg 28.19 aux) | none |
| 12 | `max_contig_sub_sum_divcon_opt` | DivConOptStEph | trait+impl | none |
| 13 | `max_with_neginf` | DivConStEph | helper | none |
| 14 | `max_suffix_sum` | DivConStEph | helper (Alg 28.12) | none |
| 15 | `max_prefix_sum` | DivConStEph | helper (Alg 28.11) | none |
| 16 | `max_contig_sub_sum_divcon` | DivConStEph | trait+impl | none |
| 17 | `max_contig_sub_sum_opt_mt` | OptMtEph | trait+impl | none |
| 18 | `max_contig_sub_sum_opt` | OptStEph | trait+impl | none |
| 19 | `max_with_neginf` | ReducedStEph | helper | none |
| 20 | `max_contig_sub_sum_reduced` | ReducedStEph | trait+impl | none |

## Phase 2: Prose Inventory

### Definitions

| # | Ref | Name | Implemented |
|---|-----|------|:-----------:|
| 1 | Def 28.1 | Subsequence | Implicit |
| 2 | Def 28.2 | Contiguous Subsequence | Implicit |
| 3 | Def 28.3 | Maximum Contiguous Subsequence (MCS) Problem | Not as standalone |
| 4 | Def 28.4 | Maximum Contiguous Subsequence Sum (MCSS) Problem | Yes — all 8 modules |
| 5 | Def 28.9 | MCSSS (max sum with start) | Helper in DivCon modules |
| 6 | Def 28.10 | MCSSE (max sum with ending) | Helper in DivCon modules |

### Algorithms

| # | Ref | Name | Work | Span | Implemented | Module |
|---|-----|------|------|------|:-----------:|--------|
| 1 | 28.5 | MCSS: Brutest Force | Unbounded | — | No | Impractical; prose notes it |
| 2 | 28.6 | MCS: Brute Force | Θ(n³) | Θ(log n) | No | Returns subsequence, not sum |
| 3 | 28.7 | MCSS via MCS | Θ(n³) | Θ(log n) | No | Superseded by 28.8 |
| 4 | 28.8 | MCSS: Brute Force Strengthened | Θ(n³) | Θ(log n) | **Yes** | `BruteStEph` |
| 5 | 28.11 | MCSSS Optimal | Θ(n) | Θ(log n) | Helper | `max_prefix_sum` in DivCon |
| 6 | 28.12 | MCSSE Optimal | Θ(n) | Θ(log n) | Helper | `max_suffix_sum` in DivCon |
| 7 | 28.13 | MCSS: Reduced Force | Θ(n²) | Θ(log n) | **Yes** | `ReducedStEph` |
| 8 | 28.14 | MCSS by Reduction to MCSSE | O(n²) | O(log n) | No | Similar to 28.13 |
| 9 | 28.15 | MCSS with Iteration (Kadane) | Θ(n) | Θ(n) | No | Linear work, linear span |
| 10 | 28.16 | MCSS: Work Optimal Low Span | Θ(n) | Θ(log n) | **Yes** | `OptStEph`, `OptMtEph` |
| 11 | 28.17 | Simple D&C for MCSS | Θ(n log n) | Θ(log² n) | **Yes** | `DivConStEph`, `DivConMtEph` |
| 12 | 28.18 | bestAcross (max spanning cut) | Θ(n) | Θ(log n) | Inline | Part of DivCon combine step |
| 13 | 28.19 | Linear Work D&C MCSS | Θ(n) | Θ(log² n) | **Yes** | `DivConOptStEph`, `DivConOptMtEph` |

### Cost Specifications

| # | Algorithm | APAS Work | APAS Span |
|---|-----------|-----------|-----------|
| 1 | Brute Force (28.8) | Θ(n³) | Θ(log n) |
| 2 | Reduced Force (28.13) | Θ(n²) | Θ(log n) |
| 3 | Optimal Scan (28.16) | Θ(n) | Θ(log n) |
| 4 | Simple D&C (28.17) | Θ(n log n) | Θ(log² n) |
| 5 | Strengthened D&C (28.19) | Θ(n) | Θ(log² n) |

### Theorems and Proofs

| # | Ref | Name | Type |
|---|-----|------|------|
| 1 | Thm 28.2 | Correctness of MCSSDC | Text proof by strong induction |
| 2 | Thm 28.3 | Work recurrence W(n)=2W(n/2)+kn | Substitution method proof |
| 3 | Thm 28.4 | Work recurrence W(n)=2W(n/2)+k·lg n | Substitution method proof |
| 4 | Lemma 28.1 | MCSSE Extension | Text proof |

### Exercises

| # | Ref | Description | Type |
|---|-----|-------------|------|
| 1 | Ex 28.1 | Strengthening changes to Alg 28.6/28.7 | Design discussion |
| 2 | Ex 28.2 | Prove MCSSE Extension Lemma | Text proof |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All 8 main trait functions already had cost annotations (APAS + claude-4-sonet + claude-4-sonnet lines). Cost annotations were added to 12 helper functions that lacked them.

| # | Function | File | APAS Cost | Claude-Opus-4.6 Cost | Match? |
|---|----------|------|-----------|-----------------------|:------:|
| 1 | `max_contig_sub_sum_brute` | BruteStEph | W Θ(n³), S Θ(log n) | W Θ(n³), S Θ(n³) | Work ✓, Span ✗ |
| 2 | `max_contig_sub_sum_reduced` | ReducedStEph | W Θ(n²), S Θ(log n) | W Θ(n²), S Θ(n²) | Work ✓, Span ✗ |
| 3 | `max_contig_sub_sum_opt` | OptStEph | W Θ(n), S Θ(log n) | W Θ(n), S Θ(n) | Work ✓, Span ✗ |
| 4 | `max_contig_sub_sum_opt_mt` | OptMtEph | W Θ(n), S Θ(log n) | W Θ(n), S Θ(log n) | ✓ |
| 5 | `max_contig_sub_sum_divcon` | DivConStEph | W Θ(n log n), S Θ(log² n) | W Θ(n log n), S Θ(n log n) | Work ✓, Span ✗ |
| 6 | `max_contig_sub_sum_divcon_mt` | DivConMtEph | W Θ(n log n), S Θ(log² n) | W Θ(n log n), S Θ(log² n) | ✓ |
| 7 | `max_contig_sub_sum_divcon_opt` | DivConOptStEph | W Θ(n), S Θ(log² n) | W Θ(n log n), S Θ(n) | ✗ |
| 8 | `max_contig_sub_sum_divcon_opt_mt` | DivConOptMtEph | W Θ(n), S Θ(log² n) | W Θ(n log n), S Θ(n) | ✗ |

**Key discrepancies**:

1. **St span mismatches (rows 1–3, 5)**: Expected — St modules are sequential implementations; APAS span assumes parallel operations.
2. **DivConOpt work mismatch (rows 7–8)**: `subseq_copy` costs O(n) per level instead of APAS's O(1) or O(log n) `splitMid`. This inflates work from O(n) to O(n log n). This is a fundamental implementation fidelity issue — the algorithm needs subarray views (not copies) to achieve the APAS bound.

### 3b. Implementation Fidelity

| # | Issue | Severity | Details |
|---|-------|----------|---------|
| 1 | `subseq_copy` vs `splitMid` | **High** | DivConOpt variants use `subseq_copy` which is O(n), not O(1)/O(log n). This breaks the O(n) work bound that Algorithm 28.19 achieves. The strengthened D&C degrades to O(n log n) work — same as the non-strengthened version. |
| 2 | `-∞` representation | Low | `Option<i32>` for results (None = −∞), `i32::MIN / 2` for intermediates. Using `i32::MIN / 2` avoids overflow but is imprecise; the prose uses proper −∞. Pragmatic tradeoff. |
| 3 | Brute force is truly brute | None | `BruteStEph` recomputes each subsequence sum from scratch (triple loop), matching Algorithm 28.8's specification exactly. |
| 4 | ReducedStEph doesn't use scan | Low | Algorithm 28.13 reduces to MCSSS (Algorithm 28.11, which uses scan). The implementation uses manual nested loops instead, but achieves the same O(n²) work. |
| 5 | Kadane's algorithm missing | Medium | Algorithm 28.15 (MCSSIterative) — the classic linear-work linear-span algorithm — is not implemented. This is historically significant (Kadane, 1977). |
| 6 | DivConMtEph `max_suffix_sum`/`max_prefix_sum` use parallel scan | None | Correctly implements Algorithms 28.11/28.12 using MtEph parallel scan+reduce. |

### 3c. Spec Fidelity

**No Verus specifications exist.** All 8 modules are plain Rust without `verus!` blocks. There are:
- No `requires`/`ensures` clauses
- No `spec fn` definitions (e.g., `spec fn spec_mcss`)
- No loop invariants
- No proof functions

This is the most significant gap. The MCSS problem has a clean mathematical definition (Def 28.4) that could be expressed as a spec function, and the correctness theorem (Thm 28.2) could be mechanized.

## Phase 4: Parallelism Review

Three Mt (multi-threaded) modules exist:

| # | Module | Parallel Mechanism | Parallelism Correct? |
|---|--------|--------------------|:--------------------:|
| 1 | `OptMtEph` | Parallel scan + reduce + tabulate via `ArraySeqMtEphBaseTrait` | ✓ |
| 2 | `DivConMtEph` | `ParaPair!` for recursive calls + parallel bestAcross | ✓ |
| 3 | `DivConOptMtEph` | `ParaPair!` for recursive calls | ✓ |

**Assessment**: All Mt modules use legitimate parallelism. `ParaPair!` is used for recursive divide-and-conquer and for independent bestAcross computations. `OptMtEph` uses the parallel scan/reduce/tabulate primitives from `ArraySeqMtEph`. No Mt module has been sequentialized.

**Issue**: `DivConOptMtEph` achieves parallel recursion but the `subseq_copy` call is O(n) and sequential, limiting the span to O(n) instead of O(log² n). The combine step is O(1) as APAS specifies.

## Phase 5: Runtime Test Review

| # | Test File | Module Tested | Tests | Coverage |
|---|-----------|--------------|:-----:|----------|
| 1 | `TestMaxContigSubSumBruteStEph.rs` | BruteStEph | 9 | empty, single±, book example, all−, all+, starts−, ends−, zeros |
| 2 | `TestMaxContigSubSumReducedStEph.rs` | ReducedStEph | 5 | empty, single+, book example, all−, all+ |
| 3 | `TestMaxContigSubSumOptStEph.rs` | OptStEph | 6 | empty, single+, book example, all−, all+, larger |
| 4 | `TestMaxContigSubSumOptMtEph.rs` | OptMtEph | 6 | empty, single+, book example, all−, all+, larger |
| 5 | `TestMaxContigSubSumDivConStEph.rs` | DivConStEph | 6 | empty, single+, book example, all−, all+, crossing |
| 6 | `TestMaxContigSubSumDivConMtEph.rs` | DivConMtEph | 6 | empty, single+, book example, all−, all+, crossing |
| 7 | `TestMaxContigSubSumDivConOptStEph.rs` | DivConOptStEph | 6 | empty, single+, book example, all−, all+, larger |
| 8 | `TestMaxContigSubSumDivConOptMtEph.rs` | DivConOptMtEph | 6 | empty, single+, book example, all−, all+, larger |

**Total tests**: 50 across 8 files.

**Quality**: Good. Every module has tests for the critical cases:
- Empty sequence → None
- Singleton
- Book example (a = ⟨1, −2, 0, 3, −1, 0, 2, −3⟩, result = 4)
- All negative (should pick the least negative element)
- All positive (should sum all elements)

**Missing test cases**:
1. `ReducedStEph` has fewer tests (5 vs 6–9 for others) — missing `single_negative`, `larger_example`, `starts_negative`, `ends_negative`
2. No cross-module equivalence tests (e.g., asserting all algorithms produce the same result on the same input)
3. No stress/randomized tests
4. Mt test files use `#![cfg(feature = "all_chapters")]` gate — correct for CI

## Phase 6: PTT Review

No PTTs exist in `rust_verify_test/tests/Chap28/`. Since:
- There are no `verus!` blocks in any Chap28 source file
- There are no verified loops or iterators
- There are no proof functions

**No PTTs needed** until the modules are verusified.

## Phase 7: Gap Analysis

### 7a. Prose Items Without Implementation

| # | Prose Ref | Name | Priority | Notes |
|---|-----------|------|----------|-------|
| 1 | Alg 28.15 | MCSSIterative (Kadane's algorithm) | Medium | Linear work, linear span. Historically important. Should be `MaxContigSubSumKadaneStEph.rs`. |
| 2 | Alg 28.14 | MCSS by Reduction to MCSSE | Low | Very similar to 28.13; optional. |
| 3 | Alg 28.5 | Brutest Force | None | Impractical by design. Prose uses it as motivation only. |
| 4 | Alg 28.6/28.7 | MCS / MCSS via MCS | None | Superseded by 28.8. |
| 5 | Ex 28.1 | Strengthening analysis | None | Design exercise, not code. |
| 6 | Ex 28.2 | Prove MCSSE Extension | None | Text proof exercise. |
| 7 | Thm 28.2 | Correctness proof | Medium | Could be mechanized if modules are verusified. |

### 7b. Code Without Prose Counterpart

| # | Item | Notes |
|---|------|-------|
| 1 | `max_with_neginf` helper (6 copies) | Utility function; could be extracted to a shared module to reduce duplication. |

## Phase 8: TOC Review

### TOC Presence

| # | File | Has TOC? | Has Section Headers? |
|---|------|:--------:|:--------------------:|
| 1 | MaxContigSubSumBruteStEph.rs | No | No |
| 2 | MaxContigSubSumReducedStEph.rs | No | No |
| 3 | MaxContigSubSumOptStEph.rs | No | No |
| 4 | MaxContigSubSumOptMtEph.rs | No | No |
| 5 | MaxContigSubSumDivConStEph.rs | No | No |
| 6 | MaxContigSubSumDivConMtEph.rs | No | No |
| 7 | MaxContigSubSumDivConOptStEph.rs | No | No |
| 8 | MaxContigSubSumDivConOptMtEph.rs | No | No |

No files have TOC headers. Since these are non-Verus files (no `verus!` blocks, no sections to distinguish), the standard TOC format does not apply in its current form. Once verusified, TOCs should be added.

### In/Out Table

Not applicable — no `verus!` blocks exist. All code is outside any verification boundary. No trait impls require in/out classification until verusification occurs.

## Proof Holes Summary

```
✓ MaxContigSubSumBruteStEph.rs
✓ MaxContigSubSumDivConMtEph.rs
✓ MaxContigSubSumDivConOptMtEph.rs
✓ MaxContigSubSumDivConOptStEph.rs
✓ MaxContigSubSumDivConStEph.rs
✓ MaxContigSubSumOptMtEph.rs
✓ MaxContigSubSumOptStEph.rs
✓ MaxContigSubSumReducedStEph.rs

Modules: 8 clean, 0 holed
Proof holes: 0
```

**No proof holes** — but this is trivially true because there is no Verus code to contain holes.

## Spec Strength Summary

| Classification | Count |
|:---:|:---:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 20 |

All 20 functions have **no Verus specifications**. The entire chapter is unverified plain Rust.

## Overall Assessment

**Chapter 28 is functionally complete but entirely unverified.**

### Strengths

1. **Good algorithmic coverage**: 5 of the 6 significant MCSS algorithms from the prose are implemented (28.8, 28.13, 28.16, 28.17, 28.19).
2. **Parallel variants exist**: 3 Mt modules with genuine parallelism using `ParaPair!` and parallel scan/reduce.
3. **Comprehensive runtime tests**: 50 tests across 8 files, covering edge cases and the textbook example.
4. **Zero proof holes**: Clean slate for verusification.

### Weaknesses

1. **No Verus specifications at all**: Every function has `spec_strength = none`. No `spec fn spec_mcss`, no `requires`/`ensures`, no loop invariants. This is the single largest gap.
2. **Algorithm 28.15 (Kadane) missing**: The classic O(n) work, O(n) span iterative algorithm is not implemented. It's historically significant and demonstrates a different design technique (iteratePrefixes).
3. **`subseq_copy` breaks D&C work bounds**: The strengthened D&C (Algorithm 28.19) achieves O(n log n) work instead of O(n) because `subseq_copy` is O(n), not O(1). This defeats the purpose of strengthening.
4. **`max_with_neginf` duplicated 6 times**: Should be extracted to a shared utility.
5. **Inconsistent test coverage**: `ReducedStEph` has only 5 tests while others have 6–9.
6. **Old cost annotation format**: Uses `claude-4-sonet`/`claude-4-sonnet` instead of `Claude-Opus-4.6`.

### Recommendations (Priority Order)

1. **Verusify**: Add `verus!` blocks, define `spec fn spec_mcss`, add `requires`/`ensures` to all functions. Start with the simplest (`BruteStEph`) and work outward.
2. **Implement Kadane's algorithm** (Algorithm 28.15) as `MaxContigSubSumKadaneStEph.rs`.
3. **Fix `subseq_copy` overhead**: Use subarray views or pass index ranges instead of copying, to restore the O(n) work bound for D&C Opt variants.
4. **Extract `max_with_neginf`** to a shared utility in the chapter or `vstdplus`.
5. **Normalize cost annotations** to the `/// - APAS:` / `/// - Claude-Opus-4.6:` format.
6. **Add more tests** to `ReducedStEph` (single negative, larger example, etc.).
