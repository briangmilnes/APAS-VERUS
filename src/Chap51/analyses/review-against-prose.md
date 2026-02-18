<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 51: Implementing Dynamic Programming — Review Against Prose

- **Date**: 2026-02-13
- **Reviewer**: Claude-Opus-4.6
- **Source**: `prompts/Chap51.txt`
- **Files**: 8 source files in `src/Chap51/`

## Phase 1: Inventory (Tool-Generated)

106 functions extracted across 8 modules. All functions are outside `verus!` (plain Rust). No Verus verification. No specs (`requires`/`ensures`). No proof holes.

| # | Module | Functions | V! | -V! | Specs | Holes |
|---|--------|:---------:|:--:|:---:|:-----:|:-----:|
| 1 | BottomUpDPMtEph | 12 | 0 | 12 | 0 | 0 |
| 2 | BottomUpDPMtPer | 10 | 0 | 10 | 0 | 0 |
| 3 | BottomUpDPStEph | 12 | 0 | 12 | 0 | 0 |
| 4 | BottomUpDPStPer | 10 | 0 | 10 | 0 | 0 |
| 5 | TopDownDPMtEph | 18 | 0 | 18 | 0 | 0 |
| 6 | TopDownDPMtPer | 16 | 0 | 16 | 0 | 0 |
| 7 | TopDownDPStEph | 15 | 0 | 15 | 0 | 0 |
| 8 | TopDownDPStPer | 13 | 0 | 13 | 0 | 0 |

## Phase 2: Prose Inventory

### Definitions
| # | Item | Description |
|---|------|-------------|
| 1 | Bottom-up method | DAG pebbling from leaves to root; compute vertices whose in-neighbors are already pebbled |
| 2 | Memoization / Memo table | Top-down approach: store argument-result pairs, look up before recomputing |

### Algorithms
| # | Algorithm | Description |
|---|-----------|-------------|
| 1 | Algorithm 51.1 | Bottom-up MED: diagonal pebbling, stores results in table M indexed by (i,j) |
| 2 | Algorithm 51.3 | The memo function: generic memoization wrapper `memo f M a` |
| 3 | Algorithm 51.4 | Memoized MED: top-down with memo table threading |

### Cost Specs
| # | Algorithm | APAS Work | APAS Span | Notes |
|---|-----------|-----------|-----------|-------|
| 1 | Algorithm 51.1 (bottom-up MED) | Θ(\|S\|×\|T\|) | Θ(\|S\|+\|T\|) | Diagonal parallelism: each diagonal position can be pebbled in parallel |
| 2 | Algorithm 51.4 (memoized MED) | Θ(\|S\|×\|T\|) | Θ(\|S\|×\|T\|) | Inherently sequential: memo table threading forces total ordering |

### Theorems/Properties
| # | Property | Description |
|---|----------|-------------|
| 1 | Limitation of top-down | Top-down with threaded memo table is inherently sequential |
| 2 | Parallelism of bottom-up | Bottom-up diagonal pebbling allows parallel computation within each diagonal |

### Exercises/Problems
None explicitly numbered in the prose.

### Examples
| # | Example | Description |
|---|---------|-------------|
| 1 | Example 51.1 | MED for S="tcat", T="atc" — DAG structure with down/horizontal/diagonal edges |
| 2 | Example 51.2 | Illustration of diagonals pebbled by Algorithm 51.1 |
| 3 | Example 51.3 | Integer-valued surrogate arguments for Subset Sum, MED, Optimal BST |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All 8 source files have been updated with APAS/Claude-Opus-4.6 cost comment pairs.

**Cost disagreements found:**

| # | File | Function | APAS Span | Claude Span | Reason |
|---|------|----------|-----------|-------------|--------|
| 1 | BottomUpDPStEph | `med_bottom_up` | Θ(\|S\|+\|T\|) | Θ(\|S\|×\|T\|) | Sequential St variant: no parallelism within diagonals |
| 2 | BottomUpDPStPer | `med_bottom_up` | Θ(\|S\|+\|T\|) | Θ(\|S\|×\|T\|) | Sequential St variant: no parallelism within diagonals |
| 3 | BottomUpDPStEph | `compute_diagonal` | Θ(1) | Θ(min(\|S\|,\|T\|)) | Sequential loop over diagonal elements |
| 4 | BottomUpDPStPer | `compute_diagonal` | Θ(1) | Θ(min(\|S\|,\|T\|)) | Sequential loop over diagonal elements |

The Mt (multi-threaded) variants agree with APAS on the main MED functions because they use `thread::spawn` per diagonal element.

### 3b. Implementation Fidelity

| # | Prose Item | Implementation | Fidelity | Notes |
|---|------------|---------------|----------|-------|
| 1 | Algorithm 51.1 (bottom-up) | `BottomUpDP*::med_bottom_up*` | High | Follows prose closely: diagonal pebbling with table M. Index calculations match. Uses `Vec<Vec<usize>>` instead of abstract array — acceptable deviation. |
| 2 | `medOne` function | `compute_cell_value` / `compute_cell_value_static` | High | Matches prose exactly: checks `S[i-1] == T[j-1]`, takes diagonal if match, else 1 + min(left, above). |
| 3 | Algorithm 51.3 (memo) | Implicit in `med_recursive` | Medium | No explicit `memo` function; memoization is inlined into the recursive function. Same effect, but the abstract pattern is not reusable. |
| 4 | Algorithm 51.4 (memoized MED) | `TopDownDP*::med_memoized*` | Medium | **Deviation**: Code includes a substitution branch (`1 + med(i-1, j-1)`) when characters don't match; APAS Algorithm 51.4 only has insert and delete (no substitute). This makes the code compute Levenshtein distance rather than the simpler MED from the textbook. |

**Key deviation**: The top-down implementations include a substitution cost (`1 + self.med_recursive(i-1, j-1)`) in the mismatch case. The APAS prose Algorithm 51.4 only considers:
- `memo medOne M (i, j-1)` (insert/left)  
- `memo medOne M (i-1, j)` (delete/above)

The substitution branch makes this the standard Levenshtein edit distance rather than the APAS version which only counts insertions and deletions (with diagonal shortcut on character match). This changes the actual MED values but does not change the asymptotic cost.

### 3c. Spec Fidelity

No Verus specifications exist. All 106 functions have spec strength **none**. There are no `requires`/`ensures` clauses to compare against prose properties.

The prose does not state formal preconditions or postconditions beyond the algorithmic description.

## Phase 4: Parallelism Review

### 4a. Mt Function Classification

| # | File | Function | Classification | Evidence |
|---|------|----------|---------------|----------|
| 1 | BottomUpDPMtEph | `med_bottom_up_parallel` | **Parallel** | `thread::spawn` per diagonal element in `compute_diagonal_parallel` |
| 2 | BottomUpDPMtEph | `compute_diagonal_parallel` | **Parallel** | `thread::spawn` per `(i,j)` position |
| 3 | BottomUpDPMtEph | `compute_cell_value_static` | Sequential | Single cell computation, no spawning |
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
| 1 | `BottomUpDPMtEph::med_bottom_up_parallel` | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | Parallel diagonals achieve the APAS span |
| 2 | `BottomUpDPMtPer::med_bottom_up_parallel` | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | Same parallel structure |
| 3 | `TopDownDPMtEph::med_memoized_concurrent` | Θ(\|S\|×\|T\|) | Θ(\|S\|×\|T\|) | Yes | Sequential — Span == Work |
| 4 | `TopDownDPMtEph::med_memoized_parallel` | N/A | Θ(\|S\|+\|T\|) | N/A | Extends beyond prose; parallel branch exploration |
| 5 | `TopDownDPMtPer::med_memoized_concurrent` | Θ(\|S\|×\|T\|) | Θ(\|S\|×\|T\|) | Yes | Sequential — Span == Work |
| 6 | `TopDownDPMtPer::med_memoized_parallel` | N/A | Θ(\|S\|+\|T\|) | N/A | Extends beyond prose; parallel branch exploration |

### 4c. Parallelism Gap Table

| # | Function | APAS Span | Actual | Parallel? | Notes |
|---|----------|-----------|--------|-----------|-------|
| 1 | BU MtEph `med_bottom_up_parallel` | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | thread::spawn per diagonal element |
| 2 | BU MtPer `med_bottom_up_parallel` | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | thread::spawn per diagonal element |
| 3 | TD MtEph `med_memoized_concurrent` | Θ(\|S\|×\|T\|) | Θ(\|S\|×\|T\|) | No | Sequential recursive calls; concurrent memo is thread-safe but not parallel |
| 4 | TD MtEph `med_memoized_parallel` | N/A | Θ(\|S\|+\|T\|) | Yes | thread::spawn per branch — goes beyond APAS |
| 5 | TD MtPer `med_memoized_concurrent` | Θ(\|S\|×\|T\|) | Θ(\|S\|×\|T\|) | No | Sequential recursive calls |
| 6 | TD MtPer `med_memoized_parallel` | N/A | Θ(\|S\|+\|T\|) | Yes | thread::spawn per branch — goes beyond APAS |

The `med_memoized_concurrent` functions in both Mt files are thread-safe (Arc<Mutex>) but not parallel — they make sequential recursive calls. The APAS prose explicitly notes this: "the top-down approach as described is inherently sequential." The `med_memoized_parallel` functions go beyond the prose by using techniques the prose mentions as "advanced techniques beyond the scope of this book."

## Phase 5: Runtime Test Review

**No runtime tests exist for Chapter 51.** No files matching `tests/*Chap51*` were found.

### 5a. Coverage Check

All 106 exec functions have zero test coverage.

### 5b. Missing Tests (Proposed)

| # | Priority | Test | Rationale |
|---|----------|------|-----------|
| 1 | High | Test `med_bottom_up` for "tcat"/"atc" (Example 51.1) | Validates core algorithm against prose example |
| 2 | High | Test `med_memoized` for "tcat"/"atc" | Cross-validates top-down against bottom-up |
| 3 | High | Test empty strings: med("", "") = 0, med("abc", "") = 3 | Base case validation |
| 4 | Medium | Test equal strings: med("abc", "abc") = 0 | Diagonal-only path |
| 5 | Medium | Test `med_memoized_parallel` against `med_memoized_concurrent` | Parallel correctness |
| 6 | Medium | Test all 8 variants produce same result | Cross-variant consistency |
| 7 | Low | Test memo table size after computation | Verifies memoization is working |
| 8 | Low | Test `set_s`/`set_t` clears memo | Verifies invariant maintenance |

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed.** Chapter 51 has no iterators, no verified loops, and no `verus!` blocks. All code is plain Rust.

### 6a. Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | BottomUpDPStEph | — | — | Missing both |
| 2 | BottomUpDPStPer | — | — | Missing both |
| 3 | BottomUpDPMtEph | — | — | Missing both |
| 4 | BottomUpDPMtPer | — | — | Missing both |
| 5 | TopDownDPStEph | — | — | Missing both |
| 6 | TopDownDPStPer | — | — | Missing both |
| 7 | TopDownDPMtEph | — | — | Missing both |
| 8 | TopDownDPMtPer | — | — | Missing both |

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status | Notes |
|---|------------|--------|-------|
| 1 | Algorithm 51.3 (generic `memo` function) | Not implemented as reusable function | Memoization is inlined into each `med_recursive`; no abstract `memo f M a` function |

### Code With No Prose Counterpart

| # | Function | Category | Notes |
|---|----------|----------|-------|
| 1 | `new` (all variants) | Constructor | Standard Rust scaffolding |
| 2 | `s_length`, `t_length`, `is_empty` | Accessors | Convenience methods |
| 3 | `set_s`, `set_t` (Eph variants) | Mutators | Ephemeral mutation support |
| 4 | `with_memo_table` (Per variants) | Persistent builder | Functional update pattern |
| 5 | `memo_size`, `is_memoized`, `get_memoized` | Memo table accessors | Testing/debugging support |
| 6 | `insert_memo` (Eph variants) | Memo mutator | Direct memo table manipulation |
| 7 | `clear_memo` | Memo reset | Reuse support |
| 8 | `initialize_base_cases` | Helper | Factored out from main algorithm |
| 9 | `compute_diagonal` / `compute_diagonal_parallel` | Helper | Factored out from main algorithm |
| 10 | `med_memoized_parallel`, `med_recursive_parallel` (Mt variants) | Parallel top-down | Goes beyond prose; prose says this is "beyond the scope of this book" |
| 11 | Trait definitions (`BottomUpDP*Trait`, `TopDownDP*Trait`) | Trait scaffolding | Standard APAS-VERUS pattern |
| 12 | `Default`, `Display`, `PartialEq`, `Eq`, `Clone`, `Debug` impls | Derive impls | Standard Rust trait implementations |

### Algorithmic Deviation: Substitution Cost

The top-down implementations include a substitution branch that APAS Algorithm 51.4 does not have. The APAS version only considers:
- Character match → diagonal (cost 0)
- Character mismatch → min(left, above) + 1

The code implements the full Levenshtein distance:
- Character match → diagonal (cost 0)
- Character mismatch → min(left, above, diagonal) + 1

This is arguably more standard but deviates from the textbook.

## Phase 8: Table of Contents Review

**No TOC headers present in any file.** None of the 8 source files contain a `// Table of Contents` block or numbered section headers. This is expected since none use `verus!` blocks — the TOC standard primarily applies to verusified modules.

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

No proof holes found because there is no Verus verification code. This is trivially clean — there are no proofs at all.

## Spec Strength Summary

| Classification | Count |
|---------------|:-----:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 106 |

All 106 functions have spec strength **none** — no `requires`/`ensures` clauses exist in any file. The entire chapter is unverified plain Rust.

## Overall Assessment

### Status: Unverified Plain Rust

Chapter 51 implements the two core dynamic programming techniques from the APAS textbook — bottom-up diagonal pebbling and top-down memoization — applied to Minimum Edit Distance. The code is well-structured across the standard 4-variant matrix (St/Mt × Eph/Per), but it has not been verusified.

### Strengths
1. **Correct algorithmic structure**: Bottom-up diagonal pebbling matches the prose closely.
2. **Genuine parallelism in Mt variants**: The Mt bottom-up variants use `thread::spawn` per diagonal element, achieving the APAS span of Θ(|S|+|T|).
3. **The Mt top-down variants include both sequential and parallel versions**, with the parallel version going beyond what the prose covers.
4. **Clean proof holes**: No unverified assumptions (trivially, since there's no Verus code).

### Issues
1. **No Verus verification**: Zero specs, zero proofs. The chapter is entirely unverified.
2. **No runtime tests**: No test files exist for any of the 8 modules.
3. **Substitution deviation**: Top-down implementations include a substitution cost not present in APAS Algorithm 51.4. This computes Levenshtein distance rather than the simpler insert/delete-only MED from the textbook.
4. **No generic memo function**: APAS Algorithm 51.3 defines a reusable `memo f M a` pattern; the implementations inline memoization directly.
5. **No TOC headers**: Expected to be added when files are verusified.
6. **Mt `med_memoized_concurrent` is sequential**: Despite being in Mt modules with concurrent data structures, the concurrent memoized variants make sequential recursive calls. The APAS prose acknowledges this limitation.

### Priority Actions
1. Add runtime tests (at minimum: "tcat"/"atc" example, empty strings, equal strings, cross-variant consistency).
2. Fix the substitution deviation in top-down to match APAS prose, or document the intentional deviation.
3. Verusify the chapter when ready — this will require `verus!` blocks, specs, and proof work.
