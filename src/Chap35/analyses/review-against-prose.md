<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 35 — Order Statistics: Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory

All functions extracted by `veracity-review-module-fn-impls -d src/Chap35`.

| # | Module | Function | V! | Location | Spec | SpecStr | Lines |
|---|--------|----------|:--:|----------|:----:|:-------:|------:|
| 1 | OrderStatSelectStEph | `spec_leq` | V! | ML | Spec | strong | 42–44 |
| 2 | OrderStatSelectStEph | `spec_kth` | V! | ML | Spec | strong | 48–52 |
| 3 | OrderStatSelectStEph | `lemma_total_ordering` | V! | ML | Proof | strong | 57–79 |
| 4 | OrderStatSelectStEph | `select` (trait) | V! | Tr+IT | HasSpec | strong | 87–92 |
| 5 | OrderStatSelectStEph | `select_inner` | V! | ML | HasSpec | strong | 110–117 |
| 6 | OrderStatSelectStPer | `select` (trait) | V! | Tr+IT | HasSpec | strong | 46–51 |
| 7 | OrderStatSelectStPer | `select_inner` | V! | ML | HasSpec | strong | 69–76 |
| 8 | OrderStatSelectMtEph | `select` (trait) | V! | Tr+IT | HasSpec | strong | 50–55 |
| 9 | OrderStatSelectMtEph | `select_inner` | V! | ML | HasSpec | strong | 71–80 |
| 10 | OrderStatSelectMtPer | `select` (trait) | V! | Tr+IT | HasSpec | strong | 50–55 |
| 11 | OrderStatSelectMtPer | `select_inner` | V! | ML | HasSpec | strong | 71–80 |

All 4 modules are inside `verus!`. All 9 exec/proof functions have `requires`/`ensures`. Zero `external_body`, zero `assume`, zero `admit`.

## Phase 2: Prose Inventory

Source: Chapter 35, "Order Statistics" from APAS.

### Definitions

| # | Item | Prose Reference |
|---|------|-----------------|
| 1 | **Definition 35.1** — Order Statistics Problem | Given sequence `a`, integer `k` where `0 ≤ k < |a|`, and a comparison `<` defining a total order, find the kth smallest element. |

### Algorithms

| # | Item | Prose Reference |
|---|------|-----------------|
| 1 | **Algorithm 35.2** — Contraction-Based Select | Randomized pivot selection; partition into `ℓ = ⟨x ∈ a | x < p⟩` and `r = ⟨x ∈ a | x > p⟩`; recurse on the appropriate side with adjusted `k`. |

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

| # | Module | Function | APAS Cost | Actual Cost | Match? |
|---|--------|----------|-----------|-------------|:------:|
| 1 | OrderStatSelectStEph | `select` | W O(n) exp, S O(lg² n) exp | W O(n) exp, S O(n) — sequential | Partial (span) |
| 2 | OrderStatSelectStPer | `select` | W O(n) exp, S O(lg² n) exp | W O(n) exp, S O(n) — sequential | Partial (span) |
| 3 | OrderStatSelectMtEph | `select` | W O(n) exp, S O(lg² n) exp | W O(n) exp, S O(n) — sequential partition, sequential recursion | **No** |
| 4 | OrderStatSelectMtPer | `select` | W O(n) exp, S O(lg² n) exp | W O(n) exp, S O(n) — sequential partition, sequential recursion | **No** |

St modules have correct work but sequential span (expected for St). Mt modules are labeled Mt but contain no parallelism — they have the same sequential span as St.

### 3b. Implementation Fidelity

| # | Module | Faithful? | Notes |
|---|--------|:---------:|-------|
| 1 | OrderStatSelectStEph | **Yes** | Single-pass O(n) partition via Vec collect into `left`/`right`/`equals`. Correctly recurses on appropriate side with adjusted `k`. Uses `TotalOrder::cmp` for three-way comparison. Random pivot via `random_usize_range`. |
| 2 | OrderStatSelectStPer | **Yes** | Same algorithm as StEph, uses `ArraySeqStPerS`. |
| 3 | OrderStatSelectMtEph | **Partial** | Correct algorithm but no parallelism despite Mt naming. Sequential partition and sequential recursion. |
| 4 | OrderStatSelectMtPer | **Partial** | Same issue as MtEph — no parallelism. |

### 3c. Spec Fidelity

| # | Spec fn | Description | Faithful? |
|---|---------|-------------|:---------:|
| 1 | `spec_leq<T: TotalOrder>` | Returns `spec_fn(T, T) -> bool` closure over `T::le` | **Yes** — captures the total order |
| 2 | `spec_kth<T: TotalOrder>(s, k)` | `s.sort_by(spec_leq())[k]` — kth element of sorted permutation | **Yes** — Definition 35.1 |
| 3 | `select` ensures | `k < n ==> result == Some(spec_kth(view, k))` | **Strong** |
| 4 | `select` requires | `a.spec_len() <= usize::MAX` | **Adequate** — prevents overflow |

The specs correctly capture Definition 35.1: the kth order statistic is the kth element of the sorted sequence. The `spec_kth` function uses `Seq::sort_by` from vstd with the `TotalOrder::le` relation.

## Phase 4: Parallelism Review

Two Mt modules exist:

| # | Module | Parallel? | Mechanism | Faithful Parallelism? |
|---|--------|:---------:|-----------|:---------------------:|
| 1 | OrderStatSelectMtEph | **No** | Sequential while loop + sequential recursion | **No** — labeled Mt but no parallel execution |
| 2 | OrderStatSelectMtPer | **No** | Sequential while loop + sequential recursion | **No** — labeled Mt but no parallel execution |

Both Mt modules contain a `TODO(parallelism)` comment acknowledging that the partition loop is sequential and that APAS expects O(lg² n) span via parallel filter-partition. No `ParaPair!`, `thread::scope`, `join`, or any other threading primitive is used.

## Phase 5: Runtime Test Review

| # | Test File | Tests | Coverage |
|---|-----------|:-----:|----------|
| 1 | TestOrderStatSelectStEph.rs | 10 | empty, single, two-element, small (with duplicates), sorted, reverse-sorted, all-duplicates, negative, mixed, large (n=1000) |
| 2 | TestOrderStatSelectStPer.rs | 10 | Same suite |
| 3 | TestOrderStatSelectMtEph.rs | 10 | Same suite |
| 4 | TestOrderStatSelectMtPer.rs | 10 | Same suite |

**Test quality:** Good coverage of edge cases. The `test_small` test checks all k values against a known sorted array including duplicates (`[3,1,4,1,5,9,2,6]`). Negative numbers and mixed signs tested. Two-element sequences test minimal contraction step. Large-input stress test (n=1000) exercises randomized recursion depth and checks k=0, k=n/2, k=n-1.

## Phase 6: PTT Review

No PTT files exist in `rust_verify_test/tests/Chap35/`. All `select_inner` functions are fully verified inline with no `external_body`. **No PTTs needed.**

## Phase 7: Gap Analysis

### Prose items with no implementation

| # | Prose Item | Status |
|---|------------|--------|
| 1 | Definition 35.1 — formal spec | **Implemented** as `spec_kth` using `Seq::sort_by` |
| 2 | Algorithm 35.2 — contraction-based select | **Implemented** in all 4 modules |
| 3 | Parallel filter-partition (prose O(lg n) span per level) | **Not implemented** — all modules use sequential partition |
| 4 | Exercises 35.1, 35.2, 35.3 | **Missing** — text proofs, not expected as code |

### Code with no prose counterpart

| # | Code Item | Notes |
|---|-----------|-------|
| 1 | `None` return for `k >= n` | Defensive extension; prose assumes `0 ≤ k < |a|` |
| 2 | Duplicate handling (`k < n - right_count → pivot`) | Correct generalization; prose assumes unique elements |

### Structural gaps

| # | Gap | Severity | Notes |
|---|-----|:--------:|-------|
| 1 | Mt modules have no parallelism | Medium | Both Mt modules are sequential; TODO comment acknowledges this |
| 2 | `TotalOrder` bounds instead of generic `Ord` | Low | Trade-off: `TotalOrder` provides spec-level `le` for the kth order statistic spec |

## Phase 8: TOC Review

### TOC Presence

| # | File | Has TOC? | Has Section Headers? |
|---|------|:--------:|:--------------------:|
| 1 | OrderStatSelectStEph.rs | Yes | Yes (1,2,3,6,7,8,9) |
| 2 | OrderStatSelectStPer.rs | Yes | Yes (1,2,3,8,9) |
| 3 | OrderStatSelectMtEph.rs | Yes | Yes (1,2,3,8,9) |
| 4 | OrderStatSelectMtPer.rs | Yes | Yes (1,2,3,8,9) |

### In/Out Table

| # | File | Trait def | `select` | `select_inner` | `spec_leq` | `spec_kth` | `lemma_total_ordering` |
|---|------|:---------:|:--------:|:--------------:|:----------:|:----------:|:----------------------:|
| 1 | OrderStatSelectStEph.rs | ✅ in | ✅ in (proven) | ✅ in (proven) | ✅ in | ✅ in | ✅ in |
| 2 | OrderStatSelectStPer.rs | ✅ in | ✅ in (proven) | ✅ in (proven) | - (imports) | - (imports) | - (imports) |
| 3 | OrderStatSelectMtEph.rs | ✅ in | ✅ in (proven) | ✅ in (proven) | - (imports) | - (imports) | - (imports) |
| 4 | OrderStatSelectMtPer.rs | ✅ in | ✅ in (proven) | ✅ in (proven) | - (imports) | - (imports) | - (imports) |

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap35/

✓ OrderStatSelectStEph.rs — 1 clean proof function
✓ OrderStatSelectStPer.rs — clean
✓ OrderStatSelectMtEph.rs — clean
✓ OrderStatSelectMtPer.rs — clean

Modules: 4 clean, 0 holed
Holes Found: 0 total
```

All four modules are fully proven with zero `assume`, `admit`, or `external_body`. The proof uses:
- `lemma_total_ordering<T>` — bridges `TotalOrder` trait to vstd's `total_ordering` predicate
- Multiset-tracking loop invariant — tracks `s.subrange(0, i).to_multiset() =~= left + right + equals`
- Pivot-tracking invariant — `i > pivot_idx ==> left.len() + right.len() < i` (proves termination)
- Partition-sort decomposition — builds `candidate = sort(left) ++ equals ++ sort(right)`, proves `sorted_by(candidate, leq)` and `candidate.to_multiset() == s.to_multiset()`, uses `lemma_sorted_unique` to conclude `sort(s) == candidate`

## Spec Strength Summary

| Classification | Count |
|:--------------:|:-----:|
| strong | 11 |
| partial | 0 |
| weak | 0 |
| none | 0 |

All spec fns and functions have strong specifications. `spec_leq` captures the total order via `TotalOrder::le`. `spec_kth` uses `Seq::sort_by` to define the kth order statistic (Definition 35.1). All `select` and `select_inner` functions ensure `result == Some(spec_kth(view, k))` when `k < n`.

## Overall Assessment

**Chapter 35 is fully verusified with strong specifications and zero proof holes.** All four module variants implement Algorithm 35.2 (contraction-based select) with formal `requires`/`ensures` connecting to `spec_kth`, which captures Definition 35.1 using `Seq::sort_by` from vstd.

1. **All 4 modules fully proven (0 holes)**: Every `select_inner` is completely verified inside `verus!` with multiset-tracking loop invariants and partition-sort decomposition proofs.

2. **Algorithmic fidelity**: All four modules correctly implement the contraction-based selection algorithm with random pivot, three-way partition, and recursive descent on the appropriate side.

3. **Mt modules lack parallelism**: Both `OrderStatSelectMtEph` and `OrderStatSelectMtPer` are labeled Mt but perform zero parallel execution. A `TODO(parallelism)` comment acknowledges this. The partition loop is sequential and recursion is sequential.

4. **Test coverage is good** (10 tests × 4 modules = 40 tests).

5. **Infrastructure**: `vstdplus/rand.rs` provides `random_usize_range` with `ensures lo <= result < hi` — reusable by any randomized algorithm.

### Remaining work

| # | Priority | Action |
|---|:--------:|--------|
| 1 | Medium | Add parallelism to Mt modules (parallel filter-partition or `ParaPair!` for recursive calls) |
| 2 | Low | Exercises 35.1–35.3 are text proofs and not expected as code |
