<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 49 — Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6
**Chapter:** Two Problems (Subset Sums and Minimum Edit Distance)

---

## Phase 2: Prose Inventory

Source: `prompts/Chap49.txt`

### Definitions

| # | Name | Description |
|---|------|-------------|
| 1 | Definition 49.1 — Subset Sum (SS) Problem | Given multiset S of positive integers and positive integer k, determine if any X ⊆ S sums to k |
| 2 | Definition 49.4 — Minimum Edit Distance (MED) Problem | Given character set Σ and two sequences S, T ∈ Σ\*, determine minimum number of insertions and deletions to transform S to T |

### Algorithms

| # | Name | Description | Implemented? |
|---|------|-------------|:---:|
| 1 | Algorithm 49.3 — Recursive Subset Sum (Indexed) | SS'(i,j): indexed DP recurrence over suffix index i and remaining target j | Yes — all 4 SS variants |
| 2 | Algorithm 49.5 — Recursive MED (List-based) | MED(S,T): list-based recursive formulation with insert/delete branching | Subsumed by 49.6 |
| 3 | Algorithm 49.6 — Recursive MED (Indexed) | MED'(i,j): indexed DP recurrence over positions from end of sequences | Yes — all 4 MED variants |

### Cost Specs

| # | Algorithm | Work | Span | Source |
|---|-----------|------|------|--------|
| 1 | SS(S, k) | O(k×\|S\|) | O(\|S\|) | Prose §1, p.357 |
| 2 | MED(S, T) | O(\|S\|×\|T\|) | O(\|S\|+\|T\|) | Prose §2, p.360 |

### Theorems / Properties

| # | Property | Description |
|---|----------|-------------|
| 1 | SS subproblem count | At most \|S\|×(k+1) distinct subproblems |
| 2 | SS pseudo-polynomial | Work is O(k×\|S\|), polynomial when k ≤ \|S\|^c |
| 3 | MED subproblem count | At most (\|S\|+1)×(\|T\|+1) distinct subproblems |
| 4 | MED DAG depth | O(\|S\|+\|T\|) — each recursive call removes an element from S or T |

### Examples in Prose

| # | Example | Expected Result | Tested? |
|---|---------|-----------------|:-------:|
| 1 | Example 49.1: SS({1,4,2,9}, 8) | false | Yes — all 4 SS variants |
| 2 | Example 49.1: SS({1,4,2,9}, 12) | true | Yes — all 4 SS variants |
| 3 | Example 49.2: SS({1,1,1}, 3) | true | Yes — StPer, MtPer |
| 4 | Example 49.3: MED(⟨A,B,C,A,D,A⟩, ⟨A,B,A,D,C⟩) | 3 | Yes — StPer, MtPer |

---

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

All 16 core exec functions (8 trait-level + 8 internal rec functions) carry the standard APAS/Claude-Opus-4.6 two-line cost annotation format.

**Cost disagreements (St variants only):**

| # | Function | File | APAS | Claude-Opus-4.6 | Reason |
|---|----------|------|------|------------------|--------|
| 1 | `subset_sum` | SubsetSumStEph.rs | Work Θ(k×\|S\|), Span Θ(\|S\|) | Work Θ(k×\|S\|), Span Θ(k×\|S\|) | Sequential — span equals work |
| 2 | `subset_sum` | SubsetSumStPer.rs | Work Θ(k×\|S\|), Span Θ(\|S\|) | Work Θ(k×\|S\|), Span Θ(k×\|S\|) | Sequential — span equals work |
| 3 | `min_edit_distance` | MinEditDistStEph.rs | Work Θ(\|S\|×\|T\|), Span Θ(\|S\|+\|T\|) | Work Θ(\|S\|×\|T\|), Span Θ(\|S\|×\|T\|) | Sequential — span equals work |
| 4 | `min_edit_distance` | MinEditDistStPer.rs | Work Θ(\|S\|×\|T\|), Span Θ(\|S\|+\|T\|) | Work Θ(\|S\|×\|T\|), Span Θ(\|S\|×\|T\|) | Sequential — span equals work |

Mt variants agree with APAS — both branches are spawned with `thread::spawn`.

### Phase 3b: Implementation Fidelity

| # | File | Prose Algorithm | Fidelity | Notes |
|---|------|----------------|----------|-------|
| 1 | SubsetSumStEph.rs | Alg 49.3 | Faithful | Indexed DP with HashMap memoization. Base cases `(_, 0)→true`, `(0, _)→false` match prose. Recursive case with `element_value > j` guard matches prose's `S[i−1] > j`. |
| 2 | SubsetSumStPer.rs | Alg 49.3 | Faithful | Same algorithm. Persistent variant clones self for memoization (clone overhead). |
| 3 | SubsetSumMtEph.rs | Alg 49.3 | Faithful | Parallel: `thread::spawn` for include/exclude branches. Shared `Arc<Mutex<HashMap>>` memoization. |
| 4 | SubsetSumMtPer.rs | Alg 49.3 | Faithful | Same as MtEph with persistent data structure. |
| 5 | MinEditDistStEph.rs | Alg 49.6 | Faithful | Indexed DP with HashMap memoization. Base cases `(i, 0)→i`, `(0, j)→j` match prose. Character match skips; mismatch tries delete `(i−1, j)` and insert `(i, j−1)`. |
| 6 | MinEditDistStPer.rs | Alg 49.6 | Faithful | Same algorithm. Persistent variant clones self. |
| 7 | MinEditDistMtEph.rs | Alg 49.6 | Faithful | Parallel: `thread::spawn` for delete/insert branches. Shared `Arc<Mutex<HashMap>>` memoization. |
| 8 | MinEditDistMtPer.rs | Alg 49.6 | Faithful | Same as MtEph with persistent data structure. |

**Deviations:**

| # | Deviation | Impact |
|---|-----------|--------|
| 1 | All implementations use `HashMap` for memoization rather than a 2D array | No asymptotic change (O(1) amortized lookup/insert), higher constant factors |
| 2 | Mt variants use `thread::spawn` + `Arc<Mutex<HashMap>>` with no granularity cutoff | Thread explosion on non-trivial inputs; lock contention degrades practical span |
| 3 | Persistent variants clone the entire solver for memoization | Defeats persistence semantics — a persistent HashMap would be more appropriate |
| 4 | MinEditDistMtEph uses a method on `impl MinEditDistMtEphS<T>` for `min_edit_distance_rec` instead of a free function | Inconsistent with the other 7 files which use free functions; no correctness impact |

**Prose typo noted:** Algorithm 49.6 line `if (S[i − 1] = T[i − 1])` should be `T[j − 1]`. The code correctly uses `table.target.nth(j - 1)`.

**Edit operations:** The prose defines MED using only insertions and deletions (no substitutions). All implementations correctly follow this — when characters mismatch, the code tries delete `(i−1, j)` and insert `(i, j−1)`, never substitution `(i−1, j−1)` on mismatch. This means MED(⟨A⟩, ⟨B⟩) = 2 (delete A + insert B), not 1.

### Phase 3c: Spec Fidelity

**N/A** — No `requires`/`ensures` specifications exist. All functions have spec strength `none`. The entire chapter is unverified Rust with no Verus blocks.

---

## Phase 4: Parallelism Review

### Phase 4a: Mt Function Classification

| # | Function | File | Classification | Mechanism |
|---|----------|------|:-------------:|-----------|
| 1 | `subset_sum` | SubsetSumMtEph.rs | Parallel | Delegates to `subset_sum_rec` |
| 2 | `subset_sum_rec` | SubsetSumMtEph.rs | Parallel | `thread::spawn` for include/exclude branches |
| 3 | `subset_sum` | SubsetSumMtPer.rs | Parallel | Delegates to `subset_sum_rec` |
| 4 | `subset_sum_rec` | SubsetSumMtPer.rs | Parallel | `thread::spawn` for include/exclude branches |
| 5 | `min_edit_distance` | MinEditDistMtEph.rs | Parallel | Delegates to `min_edit_distance_rec` |
| 6 | `min_edit_distance_rec` | MinEditDistMtEph.rs | Parallel | `thread::spawn` for delete/insert branches |
| 7 | `min_edit_distance` | MinEditDistMtPer.rs | Parallel | Delegates to `min_edit_distance_rec` |
| 8 | `min_edit_distance_rec` | MinEditDistMtPer.rs | Parallel | `thread::spawn` for delete/insert branches |

All other Mt functions (new, from\_multiset, from\_sequences, multiset, source, target, set, clear\_memo, memo\_size, eq) are **Sequential** — simple accessors/constructors.

### Phase 4b: Span Audit

| # | Function | APAS Span | Actual Span | Match? | Notes |
|---|----------|-----------|-------------|:------:|-------|
| 1 | `subset_sum` (MtEph) | Θ(\|S\|) | Θ(\|S\|) | Yes | Both branches spawned; DAG depth is \|S\| |
| 2 | `subset_sum` (MtPer) | Θ(\|S\|) | Θ(\|S\|) | Yes | Same as MtEph |
| 3 | `min_edit_distance` (MtEph) | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | Both branches spawned; DAG depth is \|S\|+\|T\| |
| 4 | `min_edit_distance` (MtPer) | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | Same as MtEph |

Caveat: Theoretical span bounds assume ideal parallel scheduling. The `Arc<Mutex<HashMap>>` serializes memo access across all threads, potentially degrading practical span.

### Phase 4c: Parallelism Gap Table

| # | Function | APAS Span | Actual | Parallel? | Notes |
|---|----------|-----------|--------|:---------:|-------|
| 1 | `subset_sum` (MtEph) | Θ(\|S\|) | Θ(\|S\|) | Yes | Both branches spawned |
| 2 | `subset_sum` (MtPer) | Θ(\|S\|) | Θ(\|S\|) | Yes | Both branches spawned |
| 3 | `min_edit_distance` (MtEph) | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | Both branches spawned |
| 4 | `min_edit_distance` (MtPer) | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | Both branches spawned |

No parallelism gaps — all Mt operations that should be parallel are parallel.

**Practical concern:** The `thread::spawn`-per-recursive-call pattern creates an enormous number of OS threads for non-trivial inputs. There is no granularity cutoff to fall back to sequential execution for small subproblems. Production use would require a work-stealing thread pool and a granularity threshold.

---

## Phase 5: Runtime Test Review

### Phase 5a: Coverage Check

| # | Source Module | RTT File | Test Count | Status |
|---|-------------|----------|:----------:|:------:|
| 1 | SubsetSumStEph.rs | `tests/Chap49/TestSubsetSumStEph.rs` | 24 | Covered |
| 2 | SubsetSumStPer.rs | `tests/Chap49/TestSubsetSumStPer.rs` | 16 | Covered |
| 3 | SubsetSumMtEph.rs | `tests/Chap49/TestSubsetSumMtEph.rs` | 12 | Covered |
| 4 | SubsetSumMtPer.rs | `tests/Chap49/TestSubsetSumMtPer.rs` | 10 | Covered |
| 5 | MinEditDistStEph.rs | `tests/Chap49/TestMinEditDistStEph.rs` | 20 | Covered |
| 6 | MinEditDistStPer.rs | `tests/Chap49/TestMinEditDistStPer.rs` | 19 | Covered |
| 7 | MinEditDistMtEph.rs | `tests/Chap49/TestMinEditDistMtEph.rs` | 22 | Covered |
| 8 | MinEditDistMtPer.rs | `tests/Chap49/TestMinEditDistMtPer.rs` | 13 | Covered |

**Total: 136 runtime tests across 8 test files.** All source modules have corresponding test files.

### Phase 5b: Test Quality

**Prose example coverage:**

| # | Prose Example | Tested In | Exact Value Checked? |
|---|---------------|-----------|:--------------------:|
| 1 | Ex 49.1: SS({1,4,2,9}, 8)→false | StEph, StPer, MtEph, MtPer | Yes |
| 2 | Ex 49.1: SS({1,4,2,9}, 12)→true | StEph, StPer, MtEph, MtPer | Yes |
| 3 | Ex 49.2: SS({1,1,1}, 3)→true | StPer (example\_49\_2), MtPer (example) | Yes |
| 4 | Ex 49.3: MED(⟨A,B,C,A,D,A⟩, ⟨A,B,A,D,C⟩)=3 | StPer (example\_49\_3), MtPer (basic) | Yes |

**Base case coverage:**

| # | Base Case | SS Tests | MED Tests |
|---|-----------|:--------:|:---------:|
| 1 | Empty input, zero target/both empty | StEph, StPer, MtEph, MtPer | StEph, StPer, MtEph, MtPer |
| 2 | Empty input, non-zero target / one side empty | StEph, StPer, MtEph, MtPer | StEph, StPer, MtEph, MtPer |
| 3 | Single element | StEph, StPer, MtPer | MtPer |
| 4 | Negative target (SS) | StEph, StPer | N/A |
| 5 | Identical sequences (MED) | N/A | StEph, StPer, MtEph, MtPer |

**Scaffolding coverage:** All test files cover constructors (`new`, `from_multiset`/`from_sequences`), getters, Display, memoization management, and macros.

**Assertion quality issues:**

| # | Test File | Issue | Severity |
|---|-----------|-------|:--------:|
| 1 | TestMinEditDistMtEph `test_completely_different` | Asserts `dist > 0` instead of `== 6` | Medium |
| 2 | TestMinEditDistMtEph `test_single_insert` | Asserts `dist > 0` instead of `== 1` | Medium |
| 3 | TestMinEditDistMtEph `test_single_delete` | Asserts `dist > 0` instead of `== 1` | Medium |
| 4 | TestMinEditDistMtEph `test_single_substitute` | Asserts `dist > 0` instead of `== 2` (delete+insert, no substitution) | Medium |
| 5 | TestMinEditDistMtEph `test_kitten_to_sitting` | Asserts `dist > 0 && dist < 10` instead of `== 5` | Medium |
| 6 | TestMinEditDistMtEph `test_with_integers` | Asserts `dist > 0` instead of `== 2` | Medium |
| 7 | TestMinEditDistMtEph `test_prefix_match` | Asserts `dist > 0` instead of `== 3` | Medium |
| 8 | TestMinEditDistMtEph `test_suffix_match` | Asserts `dist > 0` instead of `== 3` | Medium |
| 9 | TestMinEditDistStEph `test_longer_sequences` | Comment says "3 edits" for kitten→sitting but asserts 5 (5 is correct for insert/delete-only) | Low — comment-only bug |

The MtEph MED test file has systematically weak assertions — it checks sign/range rather than exact edit distances. All other test files use exact value assertions.

### Phase 5c: Missing Tests

| # | Missing Test | Priority | Rationale |
|---|-------------|:--------:|-----------|
| 1 | Cross-variant consistency (same input → same output for all 4 variants) | High | Ensures algorithm equivalence across St/Mt × Eph/Per |
| 2 | Example 49.3 in StEph and MtEph MED tests | Medium | Prose example not tested in 2 of 4 variants |
| 3 | Example 49.2 in StEph and MtEph SS tests | Low | Covered in StPer/MtPer; StEph tests `{1,1,1}` with mutation |
| 4 | Strengthen MtEph MED assertions to exact values | Medium | 8 tests use weak inequality assertions |

---

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed.** Chapter 49 has no `verus!` blocks, no iterators with `GhostIterator`/`ForLoopGhostIterator`, and no verified loops. The entire chapter is unverified Rust.

### Phase 6a: Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|:------:|
| 1 | SubsetSumStEph.rs | TestSubsetSumStEph.rs (24) | — | RTT complete |
| 2 | SubsetSumStPer.rs | TestSubsetSumStPer.rs (16) | — | RTT complete |
| 3 | SubsetSumMtEph.rs | TestSubsetSumMtEph.rs (12) | — | RTT complete |
| 4 | SubsetSumMtPer.rs | TestSubsetSumMtPer.rs (10) | — | RTT complete |
| 5 | MinEditDistStEph.rs | TestMinEditDistStEph.rs (20) | — | RTT complete |
| 6 | MinEditDistStPer.rs | TestMinEditDistStPer.rs (19) | — | RTT complete |
| 7 | MinEditDistMtEph.rs | TestMinEditDistMtEph.rs (22) | — | RTT complete, weak assertions |
| 8 | MinEditDistMtPer.rs | TestMinEditDistMtPer.rs (13) | — | RTT complete |

---

## Phase 7: Gap Analysis

### Prose Items With No Implementation

None — all algorithms from the prose (49.3, 49.5/49.6) are implemented. Algorithm 49.5 (list-based MED) is subsumed by the indexed variant (Algorithm 49.6), which is the natural translation to array-based Rust.

### Code With No Prose Counterpart

| # | Item | Files | Purpose |
|---|------|-------|---------|
| 1 | `new()` constructor | All 8 | Rust scaffolding — empty solver |
| 2 | `from_multiset()` / `from_sequences()` | All 8 | Constructor from input data |
| 3 | `multiset()` / `source()` / `target()` | All 8 | Read accessors |
| 4 | `multiset_mut()` / `source_mut()` / `target_mut()` | Eph files | Mutable accessors — ephemeral pattern |
| 5 | `set()` / `set_source()` / `set_target()` | Eph files | Mutation — ephemeral pattern |
| 6 | `clear_memo()` / `memo_size()` | All 8 | Memo management — implementation detail |
| 7 | `PartialEq::eq()` | Mt files (manual), St files (derive) | Equality comparison |
| 8 | `Display::fmt()` | All 8 | Display formatting |
| 9 | `IntoIterator` impls | St files | Iterator support — Rust idiom |
| 10 | Macros (`SubsetSumStEphLit!`, etc.) | All 8 | Literal construction — test convenience |

All are expected Rust scaffolding — no prose counterparts needed.

### Structural Observations

| # | Observation | Files Affected |
|---|-------------|----------------|
| 1 | MtEph/MtPer memo uses `Arc<Mutex<HashMap>>` (shared lock) while St uses plain `HashMap` (no lock) — correct for thread safety | Mt files |
| 2 | StPer variants clone self to create a mutable copy, defeating persistent semantics for the memo table | SubsetSumStPer, MinEditDistStPer |
| 3 | Copyright header uses `//!` (doc comment) for both copyright and module doc; should use `//` for copyright per module-header rule | All 8 files |
| 4 | No TOC comment blocks in any file | All 8 files |
| 5 | MinEditDistMtEph uses `impl` method for `min_edit_distance_rec` instead of free function — inconsistent with other 7 files | MinEditDistMtEph.rs |

---

## Phase 8: Table of Contents Review

### TOC Presence

No file in Chapter 49 has a Table of Contents comment block. None of the files contain `verus!` blocks, so the standard TOC sections 1–11 (inside verus!) do not apply. Once Verus verification is added, TOC headers should be introduced.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | SubsetSumStEph.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |
| 2 | SubsetSumStPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |
| 3 | SubsetSumMtEph.rs | ✅ out | ✅ out | - | - | - | ✅ out | ✅ out | ✅ out | - |
| 4 | SubsetSumMtPer.rs | ✅ out | ✅ out | - | - | - | ✅ out | ✅ out | ✅ out | - |
| 5 | MinEditDistStEph.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |
| 6 | MinEditDistStPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out | - |
| 7 | MinEditDistMtEph.rs | ✅ out | ✅ out | - | - | - | ✅ out | ✅ out | ✅ out | - |
| 8 | MinEditDistMtPer.rs | ✅ out | ✅ out | - | - | - | ✅ out | ✅ out | ✅ out | - |

All derive impls are outside `verus!` because there are no `verus!` blocks. Once Verus verification is added, Clone, PartialEq/Eq should move inside per project standards.

---

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap49/

✓ MinEditDistMtEph.rs
✓ MinEditDistMtPer.rs
✓ MinEditDistStEph.rs
✓ MinEditDistStPer.rs
✓ SubsetSumMtEph.rs
✓ SubsetSumMtPer.rs
✓ SubsetSumStEph.rs
✓ SubsetSumStPer.rs

Modules: 8 clean, 0 holed
Holes Found: 0 total
```

No proof holes — vacuously clean since there is no Verus code.

---

## Review TODOs

| # | Action | Priority | Category |
|---|--------|:--------:|----------|
| 1 | Strengthen MtEph MED test assertions to exact values (8 tests use weak inequalities) | High | Tests |
| 2 | Add cross-variant consistency tests (same input → same output across all 4 variants) | High | Tests |
| 3 | Add Example 49.3 MED test to StEph and MtEph test files | Medium | Tests |
| 4 | Fix comment in TestMinEditDistStEph `test_longer_sequences` — says "3 edits" but correct answer is 5 (insert/delete only, no substitution) | Low | Tests |
| 5 | Add Verus verification with `requires`/`ensures` for core functions | High | Verification |
| 6 | Add TOC headers to all 8 source files (once verusified) | Medium | Style |
| 7 | Fix copyright header format: use `//` for copyright, `//!` for module doc | Low | Style |
| 8 | Make `min_edit_distance_rec` in MinEditDistMtEph a free function for consistency with other 7 files | Low | Style |
| 9 | Add granularity cutoff to Mt variants to prevent thread explosion | Medium | Performance |
| 10 | Consider persistent HashMap for StPer/MtPer memo tables | Low | Semantics |
