<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 35 — Order Statistics: Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory

All functions extracted by `veracity-review-module-fn-impls -d src/Chap35`.

| # | Module | Function | V! | Spec | SpecStr | Lines |
|---|--------|----------|:--:|:----:|:-------:|------:|
| 1 | OrderStatSelectMtEph | `select` | -V! | NoSpec | none | 15–18 |
| 2 | OrderStatSelectMtPer | `select` | -V! | NoSpec | none | 16–17 |
| 3 | OrderStatSelectStEph | `select` | -V! | NoSpec | none | 16–17 |
| 4 | OrderStatSelectStPer | `select` | -V! | NoSpec | none | 17–18 |

**Key observation:** No module uses `verus!`. All code is plain Rust with zero formal specifications.

## Phase 2: Prose Inventory

Source: Chapter 35, "Order Statistics" from APAS.

### Definitions

| # | Item | Prose Reference |
|---|------|-----------------|
| 1 | **Definition 35.1** — Order Statistics Problem | Given sequence `a`, integer `k` where `0 ≤ k < |a|`, and a comparison `<` defining a total order, find the kth smallest element. |

### Algorithms

| # | Item | Prose Reference |
|---|------|-----------------|
| 1 | **Algorithm 35.2** — Contraction-Based Select | Randomized pivot selection; partition into `ℓ = ⟨x ∈ a | x < p⟩` and `r = ⟨x ∈ a | x > p⟩`; recurse on the appropriate side. |

### Cost Specifications

| # | Item | Work | Span |
|---|------|------|------|
| 1 | Algorithm 35.2 | O(n) expected | O(lg² n) expected / w.h.p. |

### Theorems / Analysis

| # | Item | Description |
|---|------|-------------|
| 1 | Dart game analysis (Section 3.1) | Pivot with rank in (n/4, 3n/4) contracts by 3/4 with probability 1/2; O(lg n) rounds w.h.p. |
| 2 | Expected work = O(n) | Geometric series from contraction factor 0.875 |
| 3 | Expected span = O(lg² n) | O(lg n) rounds × O(lg n) span per round (parallel filter) |

### Exercises

| # | Item | Description |
|---|------|-------------|
| 1 | Exercise 35.1 | Redo work analysis without using the "size decreases by ≥ 1" fact |
| 2 | Exercise 35.2 | Redo span analysis using expected span instead of expected work |
| 3 | Exercise 35.3 | Prove pivot tree has O(lg n) height w.h.p. |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Module | Function | APAS Cost | Claude-Opus-4.6 Cost | Match? |
|---|--------|----------|-----------|----------------------|:------:|
| 1 | OrderStatSelectMtEph | `select` | Work O(n) exp, Span O(lg² n) exp | Work O(n) exp, Span O(lg² n) exp | Yes |
| 2 | OrderStatSelectMtPer | `select` | Work O(n) exp, Span O(lg² n) exp | Work O(n) exp, Span O(lg n) exp per level — mutex contention | Partial |
| 3 | OrderStatSelectStEph | `select` | Work O(n) exp, Span O(lg² n) exp | Work O(n²) exp per level — tabulate O(n) scan per element | **No** |
| 4 | OrderStatSelectStPer | `select` | Work O(n) exp, Span O(lg² n) exp | Work O(n²) exp per level — tabulate O(n) scan per element | **No** |

### 3b. Implementation Fidelity

| # | Module | Faithful? | Notes |
|---|--------|:---------:|-------|
| 1 | OrderStatSelectMtEph | **Mostly** | Uses `filter_par` for `ℓ` and `r` — matches prose filter semantics. Correctly recurses on appropriate side with adjusted `k`. |
| 2 | OrderStatSelectMtPer | **Partial** | Spawns one thread per element with `Arc<Mutex<Vec>>` — correct result but poor parallel structure (O(n) thread spawns with lock contention). Not the intended parallel filter. |
| 3 | OrderStatSelectStEph | **No** | Partitions via `tabulate` with nested linear scan — each of `left_count` calls to the closure does an O(n) scan, yielding O(n²) work per recursive level. Prose uses O(n) filter. |
| 4 | OrderStatSelectStPer | **No** | Same `tabulate`-with-scan issue as StEph. O(n²) per level instead of O(n). |

**Critical issue:** The St* implementations build partition arrays using `tabulate` where each closure invocation scans the entire input to find the i-th matching element. This is O(n × partition_size) = O(n²) per recursive call, not O(n) as the prose specifies. The correct approach is to use `filter` (building left/right in a single pass).

### 3c. Spec Fidelity

No Verus specifications exist in any module. There are no `requires`, `ensures`, or `spec fn` definitions. The code is entirely unverified plain Rust.

## Phase 4: Parallelism Review

Two Mt modules exist:

| # | Module | Parallel? | Mechanism | Faithful Parallelism? |
|---|--------|:---------:|-----------|:---------------------:|
| 1 | OrderStatSelectMtEph | Yes | `filter_par` | **Yes** — parallel filter matches prose |
| 2 | OrderStatSelectMtPer | Yes | `thread::scope` + `Arc<Mutex<Vec>>` | **No** — spawns n threads with mutex; not a clean parallel filter |

**MtPer concern:** Spawning one thread per element and collecting into a `Mutex<Vec>` is technically parallel but has:
- O(n) thread creation overhead per recursive call
- Lock contention serializes the actual inserts
- Result ordering is nondeterministic (though correctness doesn't require order)

A better approach would be to use the underlying `ArraySeqMtPerS::filter_par` or a parallel partition built from `tabulate_par`.

## Phase 5: Runtime Test Review

All four modules have corresponding test files in `tests/Chap35/`.

| # | Test File | Tests | Coverage |
|---|-----------|:-----:|----------|
| 1 | TestOrderStatSelectStEph.rs | 7 | empty, single, small (with duplicates), sorted, reverse-sorted, all-duplicates, negative, mixed |
| 2 | TestOrderStatSelectStPer.rs | 7 | Same suite |
| 3 | TestOrderStatSelectMtEph.rs | 7 | Same suite |
| 4 | TestOrderStatSelectMtPer.rs | 7 | Same suite |

**Test quality:** Good coverage of edge cases. The `test_small` test checks all k values against a known sorted array including duplicates (`[3,1,4,1,5,9,2,6]`). Negative numbers and mixed signs are tested.

**Missing tests:**
1. Large input stress test (e.g., n=1000 or n=10000) to exercise the randomized recursion depth
2. k = n-1 (maximum valid k, finding the maximum element) — partially covered by the loop tests but not explicitly named
3. Two-element sequences
4. No timeout protection on Mt tests (risk of deadlock hang)

## Phase 6: PTT Review

No PTT files exist in `rust_verify_test/tests/Chap35/`.

No modules use `verus!`, so there is nothing to verify at proof time. No iterators or verified loops exist. **No PTTs needed** until the modules are verusified.

## Phase 7: Gap Analysis

### Prose items with no implementation

| # | Prose Item | Status |
|---|------------|--------|
| 1 | Definition 35.1 — formal spec of the order statistics problem | **Missing** — no `spec fn` defines what it means to find the kth order statistic |
| 2 | Exercises 35.1, 35.2, 35.3 | **Missing** — text proofs, not expected as code |

### Code with no prose counterpart

| # | Code Item | Notes |
|---|-----------|-------|
| 1 | `None` return for `k >= n` | Reasonable defensive extension; prose assumes `0 ≤ k < |a|` as a precondition |
| 2 | Duplicate handling (`k < n - right_count → pivot`) | Correct generalization; prose assumes unique elements but implementation handles duplicates |

### Structural gaps

| # | Gap | Severity |
|---|-----|:--------:|
| 1 | No `verus!` blocks anywhere — zero formal verification | **High** |
| 2 | No `spec fn` for the order statistics problem | **High** |
| 3 | No `requires`/`ensures` on `select` | **High** |
| 4 | St* implementations have O(n²) per-level work instead of O(n) | **High** |
| 5 | MtPer uses mutex-based thread-per-element instead of parallel filter | **Medium** |
| 6 | No TOC headers in any source file | **Low** |
| 7 | Cost annotations used `claude-4-sonet` (typo) — now corrected | **Low** |

## Phase 8: TOC Review

### TOC Presence

| # | File | Has TOC? | Has Section Headers? |
|---|------|:--------:|:--------------------:|
| 1 | OrderStatSelectStEph.rs | No | No |
| 2 | OrderStatSelectStPer.rs | No | No |
| 3 | OrderStatSelectMtEph.rs | No | No |
| 4 | OrderStatSelectMtPer.rs | No | No |

No file has a TOC or section headers. Since none are verusified, the standard TOC structure does not yet apply, but should be added when verusification begins.

### In/Out Table

Not applicable — no `verus!` blocks exist. All code is plain Rust outside any verification boundary.

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap35/

✓ OrderStatSelectMtEph.rs
✓ OrderStatSelectMtPer.rs
✓ OrderStatSelectStEph.rs
✓ OrderStatSelectStPer.rs

Modules: 4 clean, 0 holed
Holes Found: 0 total
```

No proof holes — but this is trivially true since no code is inside `verus!`. There is nothing to have holes in.

## Spec Strength Summary

| Classification | Count |
|----------------|:-----:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 4 |

All four functions have **no** Verus specifications.

## Overall Assessment

**Chapter 35 is a plain Rust implementation with no formal verification.** The code correctly implements Algorithm 35.2 (contraction-based select) across four module variants, but:

1. **No verusification**: Zero `verus!` blocks, zero specs, zero proofs. This is the most significant gap.

2. **Algorithmic fidelity issues**:
   - **StEph and StPer** use `tabulate` with a nested O(n) scan to build partition arrays, yielding O(n²) work per recursive level instead of the O(n) filter the prose describes. These should use `filter` or build left/right in a single pass.
   - **MtPer** uses `thread::scope` with one thread per element and `Arc<Mutex<Vec>>` — technically parallel but with poor scalability due to lock contention. Should use `filter_par` or equivalent.
   - **MtEph** is the most faithful implementation, using `filter_par` for the parallel partition.

3. **Test coverage is decent** (7 tests × 4 modules = 28 tests) covering edge cases, duplicates, negatives, and sorted/reverse-sorted inputs. Missing: large-input stress tests and timeout protection on Mt tests.

4. **Exercises 35.1–35.3** are text proofs and not expected as code implementations.

### Recommended priorities for verusification:
1. Define `spec fn spec_kth_order_statistic` capturing Definition 35.1
2. Fix St* implementations to use O(n) filter instead of O(n²) tabulate
3. Fix MtPer to use `filter_par` instead of mutex-based thread-per-element
4. Add `verus!` blocks with `requires`/`ensures` to all `select` functions
5. Add TOC headers when verusifying
