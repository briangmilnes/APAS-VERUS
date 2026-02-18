<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 49 — Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6
**Chapter:** Two Problems (Subset Sums and Minimum Edit Distance)

---

## Phase 1: Inventory (Tool-Generated)

Generated via `veracity-review-module-fn-impls -d src/Chap49`.

- **Files:** 8
- **Functions extracted:** 72
- **Spec strengths classified:** 72 (all `none`)

All 8 source files are **plain Rust** — no `verus!` blocks, no Verus verification, no `requires`/`ensures` specifications. The chapter is entirely unverified.

---

## Phase 2: Prose Inventory

Source: `prompts/Chap49.txt`

### Definitions

| # | Name | Description |
|---|------|-------------|
| 1 | Definition 49.1 — Subset Sum (SS) Problem | Given multiset S of positive integers and positive integer k, determine if any X ⊆ S sums to k |
| 2 | Definition 49.4 — Minimum Edit Distance (MED) Problem | Given character set Σ and two sequences S, T ∈ Σ*, determine minimum insertions and deletions to transform S to T |

### Algorithms

| # | Name | Description |
|---|------|-------------|
| 1 | Algorithm 49.3 — Recursive Subset Sum (Indexed) | SS'(i,j): indexed DP recurrence over suffix index i and remaining target j |
| 2 | Algorithm 49.5 — Recursive MED | MED(S,T): list-based recursive formulation with insert/delete branching |
| 3 | Algorithm 49.6 — Recursive MED (Indexed) | MED'(i,j): indexed DP recurrence over positions i,j from end of sequences |

### Cost Specs

| # | Algorithm | Work | Span | Source |
|---|-----------|------|------|--------|
| 1 | SS(S, k) | O(k×\|S\|) | O(\|S\|) | Prose §1, p.357 |
| 2 | MED(S, T) | O(\|S\|×\|T\|) | O(\|S\|+\|T\|) | Prose §2, p.360 |

### Theorems/Properties

| # | Property | Description |
|---|----------|-------------|
| 1 | SS subproblem count | At most \|S\|×(k+1) distinct subproblems |
| 2 | SS pseudo-polynomial | Work is O(k×\|S\|), polynomial when k ≤ \|S\|^c |
| 3 | MED subproblem count | At most (\|S\|+1)×(\|T\|+1) distinct subproblems |
| 4 | MED DAG depth | O(\|S\|+\|T\|) — each call removes an element from S or T |

### Exercises/Problems

None explicitly numbered in the provided prose.

---

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

All 16 core exec functions (8 trait-level + 8 internal rec functions) updated to standard APAS/Claude-Opus-4.6 two-line format.

**Cost disagreements found:**

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
| 1 | SubsetSumStEph.rs | Alg 49.3 (Indexed SS) | Faithful | Implements indexed DP with HashMap memoization. Base cases (i,0)→true, (0,j)→false match prose. Recursive case with element_value > j guard matches prose's S[i−1] > j check. |
| 2 | SubsetSumStPer.rs | Alg 49.3 (Indexed SS) | Faithful | Same algorithm as StEph. Persistent variant clones self for memoization (clone overhead). |
| 3 | SubsetSumMtEph.rs | Alg 49.3 (Indexed SS) | Faithful | Parallel variant uses `thread::spawn` for both include/exclude branches. Shared `Arc<Mutex<HashMap>>` memoization. |
| 4 | SubsetSumMtPer.rs | Alg 49.3 (Indexed SS) | Faithful | Same as MtEph with persistent data structure. |
| 5 | MinEditDistStEph.rs | Alg 49.6 (Indexed MED) | Faithful | Implements indexed DP with HashMap memoization. Base cases (i,0)→i, (0,j)→j match prose. Character match skips with no edit; mismatch tries delete and insert. |
| 6 | MinEditDistStPer.rs | Alg 49.6 (Indexed MED) | Faithful | Same algorithm as StEph. Persistent variant clones self for memoization. |
| 7 | MinEditDistMtEph.rs | Alg 49.6 (Indexed MED) | Faithful | Parallel variant uses `thread::spawn` for delete/insert branches when characters differ. Shared `Arc<Mutex<HashMap>>` memoization. |
| 8 | MinEditDistMtPer.rs | Alg 49.6 (Indexed MED) | Faithful | Same as MtEph with persistent data structure. |

**Deviations:**
- All implementations use `HashMap` for memoization rather than a 2D array. This doesn't change asymptotic work (O(1) amortized lookup/insert) but has higher constant factors.
- The prose describes the recurrence abstractly; the code uses explicit `match` on `(i, j)` tuples, which is a natural Rust translation.
- The Mt variants use `thread::spawn` + `Arc<Mutex<HashMap>>` for parallelism, which introduces lock contention not present in the theoretical cost model. Thread creation overhead is also non-trivial for small subproblems (no granularity cutoff).

### Phase 3c: Spec Fidelity

**N/A** — No `requires`/`ensures` specifications exist. All 72 functions have spec strength `none`. The entire chapter is unverified Rust code with no Verus specs.

---

## Phase 4: Parallelism Review

### Phase 4a: Mt Function Classification

| # | Function | File | Classification | Mechanism |
|---|----------|------|---------------|-----------|
| 1 | `subset_sum` | SubsetSumMtEph.rs | Parallel | Delegates to `subset_sum_rec` which spawns threads |
| 2 | `subset_sum_rec` | SubsetSumMtEph.rs | Parallel | `thread::spawn` for include/exclude branches |
| 3 | `subset_sum` | SubsetSumMtPer.rs | Parallel | Delegates to `subset_sum_rec` which spawns threads |
| 4 | `subset_sum_rec` | SubsetSumMtPer.rs | Parallel | `thread::spawn` for include/exclude branches |
| 5 | `min_edit_distance` | MinEditDistMtEph.rs | Parallel | Delegates to `min_edit_distance_rec` which spawns threads |
| 6 | `min_edit_distance_rec` | MinEditDistMtEph.rs | Parallel | `thread::spawn` for delete/insert branches |
| 7 | `min_edit_distance` | MinEditDistMtPer.rs | Parallel | Delegates to `min_edit_distance_rec` which spawns threads |
| 8 | `min_edit_distance_rec` | MinEditDistMtPer.rs | Parallel | `thread::spawn` for delete/insert branches |

All other Mt functions (new, from_multiset, from_sequences, multiset, set, clear_memo, memo_size, eq) are **Sequential** — simple accessors/constructors with no parallelism needed.

### Phase 4b: Span Audit

| # | Function | APAS Span | Actual Span | Match? | Notes |
|---|----------|-----------|-------------|--------|-------|
| 1 | `subset_sum` (MtEph) | Θ(\|S\|) | Θ(\|S\|) | Yes | Both branches spawned; DAG depth is \|S\| |
| 2 | `subset_sum` (MtPer) | Θ(\|S\|) | Θ(\|S\|) | Yes | Same as MtEph |
| 3 | `min_edit_distance` (MtEph) | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | Both branches spawned; DAG depth is \|S\|+\|T\| |
| 4 | `min_edit_distance` (MtPer) | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | Same as MtEph |

Note: Theoretical span bounds assume ideal parallel scheduling. The `Arc<Mutex<HashMap>>` memoization introduces serial lock contention that could degrade practical span.

### Phase 4c: Parallelism Gap Table

| # | Function | APAS Span | Actual | Parallel? | Notes |
|---|----------|-----------|--------|-----------|-------|
| 1 | `subset_sum` (MtEph) | Θ(\|S\|) | Θ(\|S\|) | Yes | `thread::spawn` on both branches |
| 2 | `subset_sum` (MtPer) | Θ(\|S\|) | Θ(\|S\|) | Yes | `thread::spawn` on both branches |
| 3 | `min_edit_distance` (MtEph) | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | `thread::spawn` on delete/insert |
| 4 | `min_edit_distance` (MtPer) | Θ(\|S\|+\|T\|) | Θ(\|S\|+\|T\|) | Yes | `thread::spawn` on delete/insert |

No parallelism gaps — all Mt operations that should be parallel are parallel.

**Caveat:** The `thread::spawn`-per-recursive-call pattern creates an enormous number of OS threads for non-trivial inputs. There is no granularity cutoff to fall back to sequential execution for small subproblems. This will cause thread exhaustion or extreme overhead on real inputs. A production implementation would need:
1. A work-stealing thread pool (e.g., rayon) instead of raw `thread::spawn`
2. A granularity cutoff to sequentialize small subproblems

---

## Phase 5: Runtime Test Review

### Phase 5a: Coverage Check

**No runtime test files exist for Chapter 49.** Zero test coverage.

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | SubsetSumStEph.rs | — | Missing |
| 2 | SubsetSumStPer.rs | — | Missing |
| 3 | SubsetSumMtEph.rs | — | Missing |
| 4 | SubsetSumMtPer.rs | — | Missing |
| 5 | MinEditDistStEph.rs | — | Missing |
| 6 | MinEditDistStPer.rs | — | Missing |
| 7 | MinEditDistMtEph.rs | — | Missing |
| 8 | MinEditDistMtPer.rs | — | Missing |

### Phase 5b: Test Quality

N/A — no tests exist.

### Phase 5c: Missing Tests

Priority proposed tests:

| # | Test | Priority | Rationale |
|---|------|----------|-----------|
| 1 | SubsetSum basic: SS({1,4,2,9}, 8)→false, SS({1,4,2,9}, 12)→true | High | Example 49.1 from prose |
| 2 | SubsetSum edge: SS({}, 0)→true, SS({}, 5)→false, SS({3}, 3)→true | High | Base cases |
| 3 | SubsetSum negative target: SS({1,2}, -1)→false | Medium | Guard clause |
| 4 | MED basic: MED(⟨A,B,C,A,D,A⟩, ⟨A,B,A,D,C⟩)→3 | High | Example 49.3 from prose |
| 5 | MED edge: MED(⟨⟩, ⟨A,B⟩)→2, MED(⟨A,B⟩, ⟨⟩)→2, MED(⟨⟩, ⟨⟩)→0 | High | Base cases |
| 6 | MED identical: MED(⟨A,B,C⟩, ⟨A,B,C⟩)→0 | Medium | No edits needed |
| 7 | All 4 variants per algorithm produce identical results | High | Cross-variant consistency |

---

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed.** Chapter 49 has no `verus!` blocks, no iterators with `GhostIterator`/`ForLoopGhostIterator`, and no verified loops. The entire chapter is unverified Rust.

### Phase 6a: Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | SubsetSumStEph.rs | — | — | Missing RTT |
| 2 | SubsetSumStPer.rs | — | — | Missing RTT |
| 3 | SubsetSumMtEph.rs | — | — | Missing RTT |
| 4 | SubsetSumMtPer.rs | — | — | Missing RTT |
| 5 | MinEditDistStEph.rs | — | — | Missing RTT |
| 6 | MinEditDistStPer.rs | — | — | Missing RTT |
| 7 | MinEditDistMtEph.rs | — | — | Missing RTT |
| 8 | MinEditDistMtPer.rs | — | — | Missing RTT |

---

## Phase 7: Gap Analysis

### Prose Items With No Implementation

None — all three algorithms (49.3, 49.5, 49.6) are implemented. The implementations follow Algorithm 49.3 (indexed SS) and Algorithm 49.6 (indexed MED). Algorithm 49.5 (list-based MED) is subsumed by the indexed variant.

### Code With No Prose Counterpart

| # | Item | File | Purpose |
|---|------|------|---------|
| 1 | `new()` constructor | All 8 files | Creates empty solver — Rust scaffolding |
| 2 | `from_multiset()` / `from_sequences()` | All 8 files | Constructor from input — Rust scaffolding |
| 3 | `multiset()` / `source()` / `target()` | All 8 files | Accessor — Rust scaffolding |
| 4 | `multiset_mut()` / `source_mut()` / `target_mut()` | Eph files | Mutable accessor — ephemeral pattern |
| 5 | `set()` / `set_source()` / `set_target()` | Eph files | Mutation — ephemeral pattern |
| 6 | `clear_memo()` / `memo_size()` | All 8 files | Memo management — implementation detail |
| 7 | `PartialEq::eq()` | Mt files | Equality comparison — Rust derive |
| 8 | `Display::fmt()` | All 8 files | Display formatting — Rust derive |
| 9 | `IntoIterator` impls | St files | Iterator support — Rust idiom |
| 10 | Macros (`SubsetSumStEphLit!`, etc.) | All 8 files | Literal construction — test convenience |

All are expected Rust scaffolding — no prose counterparts needed.

---

## Phase 8: Table of Contents Review

### TOC Presence

No file in Chapter 49 has a Table of Contents comment block. None of the files contain `verus!` blocks, so the standard TOC sections 1-11 (inside verus!) do not apply.

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

All derive impls are outside `verus!` because there are no `verus!` blocks. Once Verus verification is added, Clone, PartialEq/Eq should move inside `verus!` per project standards.

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

No proof holes — but only because there is no Verus code at all. The "clean" status is vacuously true.

---

## Spec Strength Summary

| Classification | Count |
|---|---|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 72 |

**100% of functions have no Verus specifications.** The entire chapter is unverified Rust.

---

## Overall Assessment

Chapter 49 implements two dynamic programming algorithms — Subset Sum (Algorithm 49.3) and Minimum Edit Distance (Algorithm 49.6) — in four variants each (StEph, StPer, MtEph, MtPer). All implementations are **algorithmically faithful** to the textbook prose.

### Strengths

1. **Algorithmic fidelity** — All implementations correctly follow the indexed DP recurrences from the prose.
2. **Memoization** — All variants use HashMap-based memoization, correctly identifying and reusing subproblem solutions.
3. **Parallelism** — Mt variants use `thread::spawn` for genuine parallelism on the two branches of each recurrence.
4. **Complete variant coverage** — All 4 standard variants (StEph, StPer, MtEph, MtPer) are implemented for both algorithms.

### Weaknesses

1. **No Verus verification** — Zero `verus!` blocks, zero specs, zero proofs. This is the highest-priority gap.
2. **No tests** — Zero runtime tests. Not a single function is tested.
3. **No TOC headers** — None of the files follow the table-of-contents standard.
4. **Thread explosion in Mt variants** — `thread::spawn` per recursive call with no granularity cutoff will exhaust OS threads on non-trivial inputs. Needs a thread pool or cutoff threshold.
5. **Mutex contention** — `Arc<Mutex<HashMap>>` serializes memo access across all threads, potentially negating parallelism benefits.
6. **No persistent variant differentiation** — StPer variants clone self to create a mutable copy for memoization, defeating the purpose of persistence. The persistent variant should use a persistent data structure for the memo table.

### Priority Actions

| # | Action | Priority |
|---|--------|----------|
| 1 | Add runtime tests for both algorithms with prose examples | High |
| 2 | Add Verus verification with `requires`/`ensures` for core functions | High |
| 3 | Add TOC headers to all files | Medium |
| 4 | Add granularity cutoff to Mt variants | Medium |
| 5 | Consider replacing `thread::spawn` with thread pool | Low |
| 6 | Consider using persistent HashMap for StPer/MtPer memo tables | Low |
