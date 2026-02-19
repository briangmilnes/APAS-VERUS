<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 35 — Order Statistics: Review Against Prose

**Date:** 2026-02-19 (updated: Mt* select_inner now fully proven, 0 holes across all modules)
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory

All functions extracted by `veracity-review-module-fn-impls -d src/Chap35`.

| # | Module | Function | V! | Spec | SpecStr | Lines |
|---|--------|----------|:--:|:----:|:-------:|------:|
| 1 | OrderStatSelectStEph | `spec_leq` | V! | Spec | strong | 34–36 |
| 2 | OrderStatSelectStEph | `spec_kth` | V! | Spec | strong | 40–44 |
| 3 | OrderStatSelectStEph | `select` (trait) | V! | HasSpec | strong | 48–56 |
| 4 | OrderStatSelectStEph | `select` (impl) | V! | HasSpec | strong | 62–67 |
| 5 | OrderStatSelectStPer | `select` (trait) | V! | HasSpec | strong | 37–45 |
| 6 | OrderStatSelectStPer | `select` (impl) | V! | HasSpec | strong | 49–54 |
| 7 | OrderStatSelectMtEph | `select` (trait) | V! | HasSpec | strong | 39–47 |
| 8 | OrderStatSelectMtEph | `select` (impl) | V! | HasSpec | strong | 51–56 |
| 9 | OrderStatSelectMtPer | `select` (trait) | V! | HasSpec | strong | 39–47 |
| 10 | OrderStatSelectMtPer | `select` (impl) | V! | HasSpec | strong | 51–56 |
| 11 | OrderStatSelectStEph | `lemma_total_ordering` | V! | Proof | strong | — |
| 12 | OrderStatSelectStEph | `select_inner` | V! | HasSpec | strong | — |
| 13 | OrderStatSelectStPer | `select_inner` | V! | HasSpec | strong | — |
| 14 | OrderStatSelectMtEph | `select_inner` | V! | HasSpec | strong | — |
| 15 | OrderStatSelectMtPer | `select_inner` | V! | HasSpec | strong | — |

**Key observation:** All four modules are verusified with 0 proof holes. All `select_inner` functions are fully proven (no assumes, no external_body). `lemma_total_ordering` bridges `TotalOrder` to vstd's `total_ordering` predicate.

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
| 1 | OrderStatSelectStEph | `select` | Work O(n) exp, Span O(lg² n) exp | Work O(n) exp, Span O(n) — sequential | Partial (span) |
| 2 | OrderStatSelectStPer | `select` | Work O(n) exp, Span O(lg² n) exp | Work O(n) exp, Span O(n) — sequential | Partial (span) |
| 3 | OrderStatSelectMtEph | `select` | Work O(n) exp, Span O(lg² n) exp | Work O(n) exp, Span O(lg² n) exp — parallel filter | Yes |
| 4 | OrderStatSelectMtPer | `select` | Work O(n) exp, Span O(lg² n) exp | Work O(n) exp, Span O(lg² n) exp — parallel filter | Yes |

### 3b. Implementation Fidelity

| # | Module | Faithful? | Notes |
|---|--------|:---------:|-------|
| 1 | OrderStatSelectStEph | **Yes** | Single-pass O(n) partition via Vec collect into `left`/`right`. Correctly recurses on appropriate side with adjusted `k`. Uses `TotalOrder::cmp` for three-way comparison. |
| 2 | OrderStatSelectStPer | **Yes** | Same single-pass O(n) partition pattern as StEph. |
| 3 | OrderStatSelectMtEph | **Yes** | Uses `filter_par` for parallel partition into `left`/`right` — matches prose filter semantics. Correctly recurses with adjusted `k`. |
| 4 | OrderStatSelectMtPer | **Yes** | Uses `filter_par` for parallel partition — matches prose filter semantics. Previously used `Arc<Mutex<Vec>>` with thread-per-element; now uses `filter_par`. |

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
| 1 | OrderStatSelectMtEph | Yes | `filter_par` | **Yes** — parallel filter matches prose |
| 2 | OrderStatSelectMtPer | Yes | `filter_par` | **Yes** — parallel filter matches prose |

Both Mt modules now use `filter_par` from their respective array sequence types for the partition step, matching the prose algorithm's expected O(lg n) span per level.

## Phase 5: Runtime Test Review

All four modules have corresponding test files in `tests/Chap35/`.

| # | Test File | Tests | Coverage |
|---|-----------|:-----:|----------|
| 1 | TestOrderStatSelectStEph.rs | 10 | empty, single, two-element, small (with duplicates), sorted, reverse-sorted, all-duplicates, negative, mixed, large (n=1000) |
| 2 | TestOrderStatSelectStPer.rs | 10 | Same suite |
| 3 | TestOrderStatSelectMtEph.rs | 10 | Same suite |
| 4 | TestOrderStatSelectMtPer.rs | 10 | Same suite |

**Test quality:** Good coverage of edge cases. The `test_small` test checks all k values against a known sorted array including duplicates (`[3,1,4,1,5,9,2,6]`). Negative numbers and mixed signs are tested. Two-element sequences test the minimal contraction step. Large-input stress test (n=1000) exercises randomized recursion depth and checks k=0, k=n/2, and k=n-1.

**Missing tests:** No critical gaps remaining.

## Phase 6: PTT Review

No PTT files exist in `rust_verify_test/tests/Chap35/`.

All `select_inner` functions are fully verified inline (no external_body, no assumes). **No PTTs needed currently.**

## Phase 7: Gap Analysis

### Prose items with no implementation

| # | Prose Item | Status |
|---|------------|--------|
| 1 | ~~Definition 35.1 — formal spec~~ | **Implemented** as `spec_kth` using `Seq::sort_by` |
| 2 | Exercises 35.1, 35.2, 35.3 | **Missing** — text proofs, not expected as code |

### Code with no prose counterpart

| # | Code Item | Notes |
|---|-----------|-------|
| 1 | `None` return for `k >= n` | Reasonable defensive extension; prose assumes `0 ≤ k < |a|` as a precondition |
| 2 | Duplicate handling (`k < n - right_count → pivot`) | Correct generalization; prose assumes unique elements but implementation handles duplicates |

### Structural gaps

| # | Gap | Severity | Status |
|---|-----|:--------:|--------|
| 1 | ~~No `verus!` blocks~~ | ~~High~~ | **Fixed** — all modules verusified |
| 2 | ~~No `spec fn` for order statistics~~ | ~~High~~ | **Fixed** — `spec_leq`, `spec_kth` defined |
| 3 | ~~No `requires`/`ensures`~~ | ~~High~~ | **Fixed** — strong ensures on all `select` functions |
| 4 | ~~St* implementations O(n²)~~ | ~~High~~ | **Fixed** — single-pass O(n) Vec collect |
| 5 | ~~MtPer uses mutex-based threading~~ | ~~Medium~~ | **Fixed** — now uses `filter_par` |
| 6 | ~~No TOC headers~~ | ~~Low~~ | **Fixed** |
| 7 | ~~Exec bodies are `external_body` (rand)~~ | ~~Medium~~ | **Fixed** — `vstdplus/rand.rs` provides `random_usize_range` with spec; St* `select_inner` now fully inside verus! |
| 8 | `TotalOrder` bounds instead of generic `Ord` | Low | Trade-off: `TotalOrder` provides spec-level `le` for the kth order statistic spec; `Ord` would need a similar bridge |
| 9 | ~~St* `select_inner` has 9 assumes each~~ | ~~Medium~~ | **Fixed** — all 18 assumes closed via `lemma_total_ordering`, multiset-tracking loop invariant, and partition-sort decomposition proof |
| 10 | ~~Mt* `select_inner` is `external_body`~~ | ~~Medium~~ | **Fixed** — Mt* `select_inner` now fully proven inside verus! |

## Phase 8: TOC Review

### TOC Presence

| # | File | Has TOC? | Has Section Headers? |
|---|------|:--------:|:--------------------:|
| 1 | OrderStatSelectStEph.rs | Yes | Yes (1,2,3,6,7,8,9) |
| 2 | OrderStatSelectStPer.rs | Yes | Yes (1,2,3,8,9) |
| 3 | OrderStatSelectMtEph.rs | Yes | Yes (1,2,3,8,9) |
| 4 | OrderStatSelectMtPer.rs | Yes | Yes (1,2,3,8,9) |

### In/Out Table

| # | File | Trait def | `select` | `select_inner` | `spec_leq` | `spec_kth` |
|---|------|:---------:|:--------:|:--------------:|:----------:|:----------:|
| 1 | OrderStatSelectStEph.rs | ✅ in | ✅ in (proven) | ✅ in (proven) | ✅ in | ✅ in |
| 2 | OrderStatSelectStPer.rs | ✅ in | ✅ in (proven) | ✅ in (proven) | - | - |
| 3 | OrderStatSelectMtEph.rs | ✅ in | ✅ in (proven) | ✅ in (proven) | - | - |
| 4 | OrderStatSelectMtPer.rs | ✅ in | ✅ in (proven) | ✅ in (proven) | - | - |

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap35/

✓ OrderStatSelectMtEph.rs
✓ OrderStatSelectMtPer.rs
✓ OrderStatSelectStEph.rs — 1 clean proof function
✓ OrderStatSelectStPer.rs

Modules: 4 clean, 0 holed
Holes Found: 0 total
```

**All four modules fully proven (0 holes).** All `select_inner` functions are completely verified inside verus! with zero assumes. The proof uses:
- `lemma_total_ordering<T>` — bridges `TotalOrder` trait to vstd's `total_ordering` predicate (for `lemma_sort_by_ensures`, `lemma_sorted_unique`)
- Multiset-tracking loop invariant — tracks `s.subrange(0, i).to_multiset() =~= left.to_multiset() + right.to_multiset() + equals.to_multiset()`
- Pivot-tracking invariant — `i > pivot_idx ==> left.len() + right.len() < i` (proves termination: partition always removes at least the pivot)
- Partition-sort decomposition — after the loop, builds `candidate = sort(left) ++ equals ++ sort(right)`, proves `sorted_by(candidate, leq)` and `candidate.to_multiset() == s.to_multiset()`, then uses `lemma_sorted_unique` to conclude `sort(s) == candidate`

**vstdplus/rand.rs:** Adds 1 external_body (`random_usize_range`) with spec `ensures lo <= result < hi`. This is infrastructure, not counted in Chap35's holes.

## Spec Strength Summary

| Classification | Count |
|----------------|:-----:|
| strong | 15 |
| partial | 0 |
| weak | 0 |
| none | 0 |

All spec fns and functions have strong specifications:
- `spec_leq`: captures the total order via `TotalOrder::le`
- `spec_kth`: uses `Seq::sort_by` to define the kth order statistic (Definition 35.1)
- `lemma_total_ordering`: proves `total_ordering(spec_leq())` from `TotalOrder` axioms
- All `select` and `select_inner` functions: `ensures k < n ==> result == Some(spec_kth(view, k))`

## Verification Summary

```
Verus verification: 1885 verified, 0 errors
Runtime tests: 40 tests, 40 passed
Proof holes: 0 (all modules clean)
```

## Overall Assessment

**Chapter 35 is fully verusified with strong specifications and 0 proof holes.** All four module variants implement Algorithm 35.2 (contraction-based select) with formal `requires`/`ensures` connecting to `spec_kth`, which captures Definition 35.1 using `Seq::sort_by` from vstd.

1. **All four modules fully proven (0 holes)**: All `select_inner` functions are completely verified inside verus! with:
   - `lemma_total_ordering` bridging `TotalOrder` to vstd's `total_ordering`
   - Multiset-tracking loop invariant proving the partition decomposes the input
   - Pivot-tracking invariant proving termination (the pivot is "consumed" by the equals bucket)
   - Partition-sort decomposition proof: builds `sort(left) ++ equals ++ sort(right)`, proves sorted and multiset-equal, uses `lemma_sorted_unique` to equate with `sort(s)`, then indexes into the concatenation

2. **Algorithmic fidelity**:
   - **StEph and StPer**: single-pass O(n) Vec partition (was O(n²) tabulate).
   - **MtPer**: `filter_par` (was `Arc<Mutex<Vec>>` thread-per-element).
   - **MtEph**: `filter_par` (unchanged).

3. **Test coverage is good** (10 tests × 4 modules = 40 tests).

4. **Infrastructure**: `vstdplus/rand.rs` provides `random_usize_range` with `ensures lo <= result < hi` — reusable by any randomized algorithm.

### Remaining work
1. Exercises 35.1–35.3 are text proofs and not expected as code
