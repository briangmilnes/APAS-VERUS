<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 51: Implementing Dynamic Programming — Review Against Prose

- **Date**: 2026-02-17
- **Reviewer**: Claude-Opus-4.6
- **Source**: `prompts/Chap51.txt`
- **Files**: 8 source files in `src/Chap51/`, 8 test files in `tests/Chap51/`

## Phase 2: Prose Inventory

### Definitions

| # | Item | Prose Reference | Description |
|---|------|----------------|-------------|
| 1 | Bottom-up method | Section 1 | DAG pebbling from leaves to root; compute vertices whose in-neighbors are already pebbled |
| 2 | Memoization / Memo table | Definition 51.2 | Top-down approach: store argument-result pairs in a mapping, look up before recomputing |

### Algorithms

| # | Algorithm | Prose Reference | Description |
|---|-----------|----------------|-------------|
| 1 | Algorithm 51.1 | Bottom-up MED | Diagonal pebbling, stores results in table M indexed by (i,j). `medOne` computes each cell: base cases `(i,0)⇒i`, `(0,j)⇒j`; match → diagonal; mismatch → `1 + min(M[i,j-1], M[i-1,j])` |
| 2 | Algorithm 51.3 | The memo function | Generic memoization wrapper `memo f M a`: look up `a` in `M`; if found return cached value, else evaluate `f`, store result, return |
| 3 | Algorithm 51.4 | Memoized MED | Top-down with memo table threading. `medOne` for mismatch: `memo medOne M (i,j-1)` and `memo medOne M (i-1,j)` — insert and delete only, no substitute |

### Cost Specs

| # | Algorithm | APAS Work | APAS Span | Notes |
|---|-----------|-----------|-----------|-------|
| 1 | Algorithm 51.1 (bottom-up MED) | Θ(\|S\|×\|T\|) | Θ(\|S\|+\|T\|) | Diagonal parallelism: each diagonal position can be pebbled in parallel |
| 2 | Algorithm 51.4 (memoized MED) | Θ(\|S\|×\|T\|) | Θ(\|S\|×\|T\|) | Inherently sequential: memo table threading forces total ordering |

### Theorems/Properties

| # | Property | Prose Reference | Description |
|---|----------|----------------|-------------|
| 1 | Limitation of top-down | Section 2, final paragraph | Top-down with threaded memo table is inherently sequential; parallelism requires hidden state, concurrent hash tables, and synchronization variables — "beyond the scope of this book" |
| 2 | Parallelism of bottom-up | Section 1 | Diagonal pebbling allows parallel computation within each diagonal |

### Examples

| # | Example | Description |
|---|---------|-------------|
| 1 | Example 51.1 | MED for S="tcat", T="atc" — DAG structure with down/horizontal/diagonal edges |
| 2 | Example 51.2 | Illustration of diagonals pebbled by Algorithm 51.1 |
| 3 | Example 51.3 | Integer-valued surrogate arguments for Subset Sum, MED, Optimal BST |

### Exercises/Problems

None explicitly numbered in the prose.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All 8 source files have APAS/Claude-Opus-4.6 cost comment pairs on every function.

**Cost disagreements found:**

| # | File | Function | APAS Span | Claude Span | Reason |
|---|------|----------|-----------|-------------|--------|
| 1 | BottomUpDPStEph | `med_bottom_up` | Θ(\|S\|+\|T\|) | Θ(\|S\|×\|T\|) | Sequential St variant: diagonals processed sequentially, no parallelism within diagonals |
| 2 | BottomUpDPStPer | `med_bottom_up` | Θ(\|S\|+\|T\|) | Θ(\|S\|×\|T\|) | Sequential St variant: same reason |
| 3 | BottomUpDPStEph | `compute_diagonal` | Θ(1) | Θ(min(\|S\|,\|T\|)) | Sequential loop over diagonal elements |
| 4 | BottomUpDPStPer | `compute_diagonal` | Θ(1) | Θ(min(\|S\|,\|T\|)) | Sequential loop over diagonal elements |

The Mt variants agree with APAS on the main MED functions because they use `thread::spawn` per diagonal element, achieving the APAS span of Θ(|S|+|T|).

### 3b. Implementation Fidelity

| # | Prose Item | Implementation | Fidelity | Notes |
|---|------------|---------------|----------|-------|
| 1 | Algorithm 51.1 (bottom-up, `medOne`) | `compute_cell_value` / `compute_cell_value_static` | **High** | Matches prose exactly: base cases `(i,0)⇒i`, `(0,j)⇒j`; match → `M[i-1,j-1]`; mismatch → `1 + min(M[i,j-1], M[i-1,j])`. Insert/delete only, no substitute. |
| 2 | Algorithm 51.1 (bottom-up, `diagonals`) | `med_bottom_up` / `med_bottom_up_parallel` | **High** | Diagonal pebbling with table M. Base cases pre-computed in `initialize_base_cases` rather than inline in `medOne` — efficiency improvement, not a semantic deviation. Index calculations: `start = max(1, k - \|T\|)`, `end = min(k, \|S\|)` match prose `s = max(0, k - \|T\|)`, `e = min(k, \|S\|)` after accounting for base-case pre-computation. |
| 3 | Algorithm 51.3 (generic `memo`) | Not implemented as reusable function | **Low** | Memoization is inlined into each `med_recursive*` function. The abstract `memo f M a` pattern from the prose is absent. Same effect for MED but the abstraction is not reusable for other DP problems. |
| 4 | Algorithm 51.4 (memoized MED, `medOne`) | `med_recursive` / `med_recursive_concurrent` | **DEVIATION** | See "Critical Deviation" below. Code includes a substitution branch (`1 + med(i-1, j-1)`) on mismatch that APAS Algorithm 51.4 does not have. |
| 5 | Memo table threading | TopDownDP St variants | **High** | Memo table is threaded through: StEph uses `&mut self` with `self.memo_table`; StPer passes `&mut HashMap` as parameter. Both match the prose's functional threading pattern. |
| 6 | Limitation: top-down is sequential | `med_memoized_concurrent` (Mt variants) | **High** | Sequential recursive calls despite `Arc<Mutex<HashMap>>`. Correctly models the prose limitation. |
| 7 | "Beyond the scope" parallelism | `med_memoized_parallel` (Mt variants) | N/A | Goes beyond the prose. Uses `thread::spawn` per recursive branch with shared concurrent memo table. The prose mentions this as an "advanced technique beyond the scope of this book." |

### 3c. Critical Deviation: Substitution Branch in Top-Down

**APAS Algorithm 51.4** for the mismatch case:

```
(i, j) => if (S[i-1] = T[j-1]) then
             memo medOne M (i-1, j-1)
          else let
             (M2, v1) = memo medOne M (i, j-1)
             (M3, v2) = memo medOne M2 (i-1, j)
          in (M3, 1 + min(v1, v2)) end
```

Two recursive calls on mismatch: insert (`i, j-1`) and delete (`i-1, j`). No substitute.

**Code** (all 4 top-down variants) for the mismatch case:

```rust
let insert_cost = 1 + self.med_recursive(i, j - 1);
let delete_cost = 1 + self.med_recursive(i - 1, j);
let substitute_cost = 1 + self.med_recursive(i - 1, j - 1);
insert_cost.min(delete_cost).min(substitute_cost)
```

Three recursive calls on mismatch: insert, delete, **and substitute** (`i-1, j-1`).

**Consequence**: The top-down implementations compute standard Levenshtein edit distance, while the bottom-up implementations compute the APAS MED (insert/delete only). These are different metrics:

| # | Input S | Input T | Bottom-Up (APAS) | Top-Down (Levenshtein) | Agree? |
|---|---------|---------|:-----------------:|:----------------------:|:------:|
| 1 | "tcat" | "atc" | 3 | 3 | Yes (coincidence) |
| 2 | "abc" | "abc" | 0 | 0 | Yes |
| 3 | "" | "" | 0 | 0 | Yes |
| 4 | "a" | "b" | **2** | **1** | **No** |
| 5 | "abc" | "xyz" | **6** | **3** | **No** |

Row 4: APAS requires delete 'a' + insert 'b' = 2; Levenshtein substitutes 'a'→'b' = 1.
Row 5: APAS requires 3 deletes + 3 inserts = 6; Levenshtein uses 3 substitutions = 3.

The existing test for "tcat"/"atc" passes for both because the values coincidentally agree. The TestBottomUpDPStPer test `test_med_bottom_up_single_char_different` correctly asserts `== 2` with comment "Algorithm uses delete+insert, not substitute". The TopDownDP tests for "abc"/"xyz" assert `== 3`, confirming they compute Levenshtein.

**No cross-variant consistency test exists** to catch this discrepancy.

### 3d. Spec Fidelity

No Verus specifications exist. All 106 functions have spec strength **none** — no `requires`/`ensures` clauses. The entire chapter is unverified plain Rust.

## Phase 4: Parallelism Review

### 4a. Mt Function Classification

| # | File | Function | Classification | Evidence |
|---|------|----------|---------------|----------|
| 1 | BottomUpDPMtEph | `med_bottom_up_parallel` | **Parallel** | `thread::spawn` per diagonal element in `compute_diagonal_parallel` |
| 2 | BottomUpDPMtEph | `compute_diagonal_parallel` | **Parallel** | `thread::spawn` per `(i,j)` position; results collected then written under single lock |
| 3 | BottomUpDPMtEph | `compute_cell_value_static` | Sequential | Single cell computation; acquires lock to read table values |
| 4 | BottomUpDPMtPer | `med_bottom_up_parallel` | **Parallel** | Same structure as MtEph variant |
| 5 | BottomUpDPMtPer | `compute_diagonal_parallel` | **Parallel** | `thread::spawn` per position |
| 6 | BottomUpDPMtPer | `compute_cell_value_static` | Sequential | Single cell computation |
| 7 | TopDownDPMtEph | `med_memoized_concurrent` | Sequential | Sequential recursive calls with concurrent memo table |
| 8 | TopDownDPMtEph | `med_memoized_parallel` | **Parallel** | `thread::spawn` for each recursive branch (insert/delete/substitute) |
| 9 | TopDownDPMtPer | `med_memoized_concurrent` | Sequential | Sequential recursive calls with concurrent memo table |
| 10 | TopDownDPMtPer | `med_memoized_parallel` | **Parallel** | `thread::spawn` for each recursive branch |

### 4b. Span Audit

| # | Function | APAS Span | Actual Span | Match? | Notes |
|---|----------|-----------|-------------|--------|-------|
| 1 | BU MtEph `med_bottom_up_parallel` | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | Parallel diagonals achieve the APAS span |
| 2 | BU MtPer `med_bottom_up_parallel` | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | Same parallel structure |
| 3 | TD MtEph `med_memoized_concurrent` | Θ(\|S\|×\|T\|) | Θ(\|S\|×\|T\|) | Yes | Sequential — Span == Work |
| 4 | TD MtEph `med_memoized_parallel` | N/A | Θ(\|S\|+\|T\|) | N/A | Beyond prose; parallel branch exploration |
| 5 | TD MtPer `med_memoized_concurrent` | Θ(\|S\|×\|T\|) | Θ(\|S\|×\|T\|) | Yes | Sequential — Span == Work |
| 6 | TD MtPer `med_memoized_parallel` | N/A | Θ(\|S\|+\|T\|) | N/A | Beyond prose; parallel branch exploration |

### 4c. Parallelism Notes

The `med_memoized_concurrent` functions in both Mt files are thread-safe (`Arc<Mutex>`) but not parallel — they make sequential recursive calls. The APAS prose explicitly notes this: "the top-down approach as described is inherently sequential."

The `med_memoized_parallel` functions go beyond the prose by implementing the "advanced techniques" the prose mentions as out of scope: hidden state via `Arc<Mutex<HashMap>>`, concurrent memo table, and implicit synchronization through Mutex. Correctness relies on the idempotency of the memo table: if two threads compute the same `(i,j)` concurrently, both produce the same value, so the last-writer-wins is harmless.

The bottom-up Mt variants use a pattern where threads read the table under lock, compute independently, then results are collected and written back under a single lock acquisition. This avoids lock contention during computation.

## Phase 5: Runtime Test Review

### 5a. Test Inventory

Tests now exist for all 8 modules.

| # | Source Module | Test File | Test Count |
|---|-------------|----------|:----------:|
| 1 | BottomUpDPStEph | `tests/Chap51/TestBottomUpDPStEph.rs` | 13 |
| 2 | BottomUpDPStPer | `tests/Chap51/TestBottomUpDPStPer.rs` | 15 |
| 3 | BottomUpDPMtEph | `tests/Chap51/TestBottomUpDPMtEph.rs` | 11 |
| 4 | BottomUpDPMtPer | `tests/Chap51/TestBottomUpDPMtPer.rs` | 11 |
| 5 | TopDownDPStEph | `tests/Chap51/TestTopDownDPStEph.rs` | 17 |
| 6 | TopDownDPStPer | `tests/Chap51/TestTopDownDPStPer.rs` | 15 |
| 7 | TopDownDPMtEph | `tests/Chap51/TestTopDownDPMtEph.rs` | 18 |
| 8 | TopDownDPMtPer | `tests/Chap51/TestTopDownDPMtPer.rs` | 14 |
| | **Total** | | **114** |

### 5b. Coverage Analysis

| # | Category | Tests Present | Notes |
|---|----------|:------------:|-------|
| 1 | Constructor (`new`) | 8/8 | All variants tested |
| 2 | Default | 8/8 | All variants tested |
| 3 | Core algorithm (textbook example) | 8/8 | "tcat"/"atc" → 3 across all variants |
| 4 | Empty strings | 8/8 | Both empty → 0 |
| 5 | Identical strings | 6/8 | BU St variants + some TD; MtEph BU/MtPer BU have it |
| 6 | One empty string | 2/8 | Only BottomUpDPStPer and BottomUpDPMtPer |
| 7 | Single char different | 1/8 | Only BottomUpDPStPer (`assert_eq!(2)`) |
| 8 | Accessors (s_length, t_length, is_empty) | 8/8 | All variants |
| 9 | Memo table ops (insert, get, is_memoized, clear) | 4/4 | All TD variants |
| 10 | Display | 8/8 | All variants |
| 11 | PartialEq | 8/8 | All variants (some equality-only, some also inequality) |
| 12 | Mutators (set_s, set_t) | 3/4 | StEph + MtEph BU + TD StEph; not tested for TD MtEph |
| 13 | `med_memoized_parallel` | 2/2 | Both Mt TD variants tested |
| 14 | Cross-variant consistency | **0** | **No test compares BU vs TD results** |

### 5c. Missing Tests

| # | Priority | Test | Rationale |
|---|----------|------|-----------|
| 1 | **Critical** | Cross-variant consistency: BU vs TD on shared inputs | Would immediately reveal the substitution deviation. E.g., "a" vs "b": BU=2, TD=1. |
| 2 | High | Single-char different for all TD variants | Currently only BU StPer has this. TD variants would return 1 (substitute), exposing the deviation. |
| 3 | Medium | One-empty-string for remaining 6 variants | Only 2 of 8 currently tested |
| 4 | Medium | `med_memoized_concurrent` vs `med_memoized_parallel` consistency | Verify parallel TD matches sequential TD within the same algorithm |
| 5 | Low | Larger inputs (stress test) | e.g., random strings of length 50+ |
| 6 | Low | Thread-safety stress test for Mt parallel variants | Multiple concurrent invocations |

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed.** Chapter 51 has no `verus!` blocks, no iterators verified in Verus, and no spec functions. All code is plain Rust. PTTs will become relevant when the chapter is verusified.

### 6a. Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | BottomUpDPStEph | `TestBottomUpDPStEph.rs` | — | RTT present, no PTT needed |
| 2 | BottomUpDPStPer | `TestBottomUpDPStPer.rs` | — | RTT present, no PTT needed |
| 3 | BottomUpDPMtEph | `TestBottomUpDPMtEph.rs` | — | RTT present, no PTT needed |
| 4 | BottomUpDPMtPer | `TestBottomUpDPMtPer.rs` | — | RTT present, no PTT needed |
| 5 | TopDownDPStEph | `TestTopDownDPStEph.rs` | — | RTT present, no PTT needed |
| 6 | TopDownDPStPer | `TestTopDownDPStPer.rs` | — | RTT present, no PTT needed |
| 7 | TopDownDPMtEph | `TestTopDownDPMtEph.rs` | — | RTT present, no PTT needed |
| 8 | TopDownDPMtPer | `TestTopDownDPMtPer.rs` | — | RTT present, no PTT needed |

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status | Notes |
|---|------------|--------|-------|
| 1 | Algorithm 51.3 (generic `memo` function) | Not implemented as reusable function | Memoization is inlined into each `med_recursive*`; no abstract `memo f M a` function exists. The reusable pattern would be valuable if other DP problems (Subset Sum, Optimal BST) are added. |

### Code With No Prose Counterpart

| # | Function | Category | Notes |
|---|----------|----------|-------|
| 1 | `new` (all variants) | Constructor | Standard Rust scaffolding |
| 2 | `s_length`, `t_length`, `is_empty` | Accessors | Convenience methods |
| 3 | `set_s`, `set_t` (Eph variants) | Mutators | Ephemeral mutation support; clears memo in TD variants |
| 4 | `with_memo_table` (Per TD variants) | Persistent builder | Functional update pattern |
| 5 | `memo_size`, `is_memoized`, `get_memoized` | Memo table accessors | Testing/debugging support |
| 6 | `insert_memo` (Eph TD variants) | Memo mutator | Direct memo table manipulation |
| 7 | `clear_memo` | Memo reset | Reuse support |
| 8 | `initialize_base_cases` | Helper | Factored out from `medOne`; pre-computes base cases for efficiency |
| 9 | `compute_diagonal` / `compute_diagonal_parallel` | Helper | Factored out from `diagonals` |
| 10 | `med_memoized_parallel`, `med_recursive_parallel` (Mt variants) | Parallel top-down | Goes beyond prose; prose says this is "beyond the scope of this book" |
| 11 | Trait definitions (`BottomUpDP*Trait`, `TopDownDP*Trait`) | Trait scaffolding | Standard APAS-VERUS pattern |
| 12 | `Default`, `Display`, `PartialEq`, `Eq`, `Clone`, `Debug` impls | Derive impls | Standard Rust trait implementations |

### Algorithmic Deviations Summary

| # | Deviation | Severity | Details |
|---|----------|----------|---------|
| 1 | **Substitution branch in top-down** | **Critical** | Top-down computes Levenshtein distance (insert/delete/substitute); bottom-up computes APAS MED (insert/delete only). These are different edit metrics that give different results on some inputs. |
| 2 | No generic `memo` function | Minor | Memoization inlined rather than abstracted per Algorithm 51.3. Same computational effect. |
| 3 | Base-case pre-computation | None (improvement) | `initialize_base_cases` pre-fills row 0 and column 0 rather than handling them inline in `medOne`. More efficient. |

## Phase 8: Table of Contents and Style Review

### TOC Headers

No TOC headers present in any file. None of the 8 source files contain a `// Table of Contents` block or numbered section headers. Expected since none use `verus!` blocks — the TOC standard primarily applies to verusified modules.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | BottomUpDPStEph | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | - | - |
| 2 | BottomUpDPStPer | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | - | - |
| 3 | BottomUpDPMtEph | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | - | - |
| 4 | BottomUpDPMtPer | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | - | - |
| 5 | TopDownDPStEph | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | - | - |
| 6 | TopDownDPStPer | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | - | - |
| 7 | TopDownDPMtEph | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | - | - |
| 8 | TopDownDPMtPer | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | - | - |

All derive impls are outside `verus!` because there are no `verus!` blocks. Once verusified, Clone/PartialEq/Eq/Default should move inside `verus!` with specs; Debug/Display must remain outside.

### Style Notes

| # | File | Issue | Notes |
|---|------|-------|-------|
| 1 | TopDownDPMtEph | Missing `Eq` marker | `PartialEq` implemented manually but no `Eq` marker trait |
| 2 | TopDownDPMtPer | Missing `Eq` marker | Same issue |
| 3 | All BU variants | `#[derive(PartialEq, Eq)]` | Derive macro — fine for now, will need manual impl with specs after verusification |
| 4 | All files | Module uses `ArraySeqStEph`/`ArraySeqMtEph` from Chap18/19 but DP table is `Vec<Vec<usize>>` | Pragmatic choice documented in comments; sequences lack nested mutation |

## Proof Holes Summary

```
✓ BottomUpDPMtEph.rs
✓ BottomUpDPMtPer.rs
✓ BottomUpDPStEph.rs
✓ BottomUpDPStPer.rs
✓ TopDownDPMtEph.rs
✓ TopDownDPMtPer.rs
✓ TopDownDPStEph.rs
✓ TopDownDPStPer.rs

Modules: 8 clean, 0 holed
Holes Found: 0 total
```

Trivially clean — no Verus verification code exists.

## Spec Strength Summary

| Classification | Count |
|---------------|:-----:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 106 |

All 106 functions have spec strength **none**. No `requires`/`ensures` clauses exist.

## Overall Assessment

### Status: Unverified Plain Rust — Critical Algorithmic Deviation

Chapter 51 implements the two core dynamic programming techniques from the APAS textbook — bottom-up diagonal pebbling and top-down memoization — applied to Minimum Edit Distance. The code is well-structured across the standard 4-variant matrix (St/Mt × Eph/Per), with 114 runtime tests across 8 test files.

### Strengths

| # | Strength |
|---|----------|
| 1 | **Bottom-up matches prose closely.** Diagonal pebbling, `medOne`, index calculations, and base-case handling all correspond to Algorithm 51.1. |
| 2 | **Genuine parallelism in Mt bottom-up variants.** `thread::spawn` per diagonal element achieves the APAS span Θ(\|S\|+\|T\|). Lock strategy (read, compute, batch-write) is sound. |
| 3 | **Comprehensive test coverage.** 114 tests across 8 files covering constructors, accessors, core algorithms, edge cases, memo operations, and trait impls. |
| 4 | **Mt top-down includes both sequential and parallel versions**, correctly modeling the prose limitation (sequential `med_memoized_concurrent`) and extending beyond it (parallel `med_memoized_parallel`). |
| 5 | **Clean proof holes.** Zero unverified assumptions (trivially, since there's no Verus code). |

### Issues

| # | Severity | Issue | Details |
|---|----------|-------|---------|
| 1 | **Critical** | **Substitution deviation** | Top-down implements Levenshtein (insert/delete/substitute); bottom-up implements APAS MED (insert/delete only). These give different results: e.g., "a" vs "b" = 2 (BU) vs 1 (TD). The test suites do not catch this because "tcat"/"atc" coincidentally gives 3 for both. |
| 2 | **High** | **No cross-variant consistency test** | A single test comparing `BU.med("a","b")` vs `TD.med("a","b")` would immediately reveal the deviation. |
| 3 | Medium | **No generic memo function** | APAS Algorithm 51.3 defines a reusable `memo f M a` pattern; implementations inline memoization. The abstraction would be valuable for adding other DP problems. |
| 4 | Medium | **No Verus verification** | Zero specs, zero proofs. The chapter is entirely unverified plain Rust. |
| 5 | Low | **Missing `Eq` marker on Mt TD types** | `TopDownDPMtEphS` and `TopDownDPMtPerS` implement `PartialEq` manually but don't implement `Eq`. |
| 6 | Low | **No TOC headers** | Expected; will be added when verusified. |

## Review TODOs

| # | Priority | Action | Rationale |
|---|----------|--------|-----------|
| 1 | **Critical** | Fix substitution deviation: remove the substitute branch from all 4 top-down `med_recursive*` functions to match APAS Algorithm 51.4 | Bottom-up and top-down must compute the same edit distance. The APAS definition uses insert/delete only. |
| 2 | **Critical** | Add cross-variant consistency test | After fixing #1, add a test that verifies all 8 variants agree on a suite of inputs including "a"/"b", "abc"/"xyz", single-char-same, single-char-different. |
| 3 | High | Add single-char-different test for all variants | Currently only BottomUpDPStPer has this. After fixing #1, all variants should return 2 for "a" vs "b". |
| 4 | Medium | Add one-empty-string test for remaining 6 variants | Currently only BottomUpDPStPer and BottomUpDPMtPer have this. |
| 5 | Medium | Add `Eq` marker impl for `TopDownDPMtEphS` and `TopDownDPMtPerS` | Missing despite having manual `PartialEq`. |
| 6 | Low | Consider extracting a generic `memo` function matching Algorithm 51.3 | Would enable reuse for other DP problems (Subset Sum, Optimal BST). |
| 7 | Future | Verusify the chapter | Add `verus!` blocks, specs, TOC headers, and move Clone/PartialEq/Eq/Default inside `verus!`. |
