<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 43: Ordered Sets, Ordered Tables, and Augmented Tables — Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6
**Prose sources:** `prompts/Chap43.txt` (Data Type 43.1, Cost Spec 43.2, Exercise 43.1, Example 43.1), `prompts/Chap43part2.txt` (Definition 43.3, Examples 43.2–43.3)
**Verification status:** All 10 ADT modules verusified with `#[verifier::external_body]` specs. Verus: 1863 verified, 0 errors. RTT: 2698 passed.
**View types:** Tables use `Map<K::V, V::V>`; Sets use `Set<T>`. All trait methods have `ensures` clauses.

## Phase 1: Inventory

Source files: 12 (3 ordered sets, 4 ordered tables, 3 augmented ordered tables, 1 example). All 10 ADT modules inside `verus!`; Example43_1 remains outside (demo code).

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap43 | OrderedSetStEph | 1 | 22 | 0 | 1 | 25 | 0 | 0 | 25 | 0 |
| 2 | Chap43 | OrderedSetStPer | 1 | 22 | 0 | 1 | 25 | 0 | 0 | 25 | 0 |
| 3 | Chap43 | OrderedSetMtEph | 1 | 21 | 0 | 0 | 23 | 0 | 0 | 23 | 0 |
| 4 | Chap43 | OrderedTableStEph | 1 | 27 | 0 | 1 | 31 | 0 | 0 | 31 | 0 |
| 5 | Chap43 | OrderedTableStPer | 1 | 26 | 0 | 1 | 28 | 0 | 0 | 28 | 0 |
| 6 | Chap43 | OrderedTableMtEph | 1 | 29 | 0 | 1 | 32 | 0 | 0 | 32 | 0 |
| 7 | Chap43 | OrderedTableMtPer | 1 | 19 | 0 | 0 | 21 | 0 | 0 | 21 | 0 |
| 8 | Chap43 | AugOrderedTableStEph | 1 | 30 | 0 | 1 | 33 | 0 | 0 | 33 | 0 |
| 9 | Chap43 | AugOrderedTableStPer | 1 | 27 | 0 | 1 | 30 | 0 | 0 | 30 | 0 |
| 10 | Chap43 | AugOrderedTableMtEph | 1 | 31 | 0 | 2 | 35 | 0 | 0 | 35 | 0 |
| 11 | Chap43 | Example43_1 | 1 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 2 |
| | **Total** | | **11** | | | | **283** | **2** | **0** | **283** | **2** |

**Notes:** ML = module-level (`from_sorted_elements`, `from_sorted_entries`, `calculate_reduction`, `recalculate_reduction`). Hole = `external_body` count (accepted spec boundary). All ADT trait methods have `ensures` clauses.

## Phase 2: Prose Inventory

### ADT 43.1 — Ordered Sets

| # | Operation | Prose | Description |
|---|-----------|-------|-------------|
| 1 | `first` | S → (U ∪ {⊥}) | min element |
| 2 | `last` | S → (U ∪ {⊥}) | max element |
| 3 | `previous` | (S × U) → (U ∪ {⊥}) | max {k′ ∈ A \| k′ < k} |
| 4 | `next` | (S × U) → (U ∪ {⊥}) | min {k′ ∈ A \| k′ > k} |
| 5 | `split` | (S × U) → S × B × S | ({x < k}, k ∈ A, {x > k}) |
| 6 | `join` | (S × S) → S | union when max(A₁) < min(A₂) |
| 7 | `getRange` | S × U × U → S | {x ∈ A \| k₁ ≤ x ≤ k₂} |
| 8 | `rank` | (S × U) → N | \|{x ∈ A \| x < k}\| |
| 9 | `select` | (S × N) → (U ∪ {⊥}) | i-th smallest element |
| 10 | `splitRank` | (S × N) → S × S | (first i elements, rest) |

Plus all base Set operations from ADT 41.1.

### Cost Specification 43.2 — Tree-Based Ordered Sets/Tables

| # | Operation | APAS Work | APAS Span |
|---|-----------|-----------|-----------|
| 1 | `first` / `last` | O(lg n) | O(lg n) |
| 2 | `previous` / `next` | O(lg n) | O(lg n) |
| 3 | `split` | O(lg n) | O(lg n) |
| 4 | `join` | O(lg(m+n)) | O(lg(m+n)) |
| 5 | `getRange` | O(lg n) | O(lg n) |
| 6 | `rank` | O(lg n) | O(lg n) |
| 7 | `select` | O(lg n) | O(lg n) |
| 8 | `splitRank` | O(lg n) | O(lg n) |
| 9 | `reduceVal` (augmented) | O(1) | O(1) |

Bulk ops (intersection, union, difference, restrict, subtract): O(m·lg(1+n/m)) work, O(lg²(n+m)) span.

### Definition 43.3 — Reducer-Augmented Ordered Table

| # | Operation | Description |
|---|-----------|-------------|
| 1 | `reduceVal` | Table.reduce f I A — reduction over all values |
| 2 | Cache maintenance | insert/delete maintain cached reduction |

### Examples

| # | Item | Description |
|---|------|-------------|
| 1 | Example 43.1 | A = {artie, burt, finn, mike, rachel, sam, tina}; first, next, getRange, rank, select, splitRank |
| 2 | Example 43.2 (TRAMLAW) | Reducer-augmented table for inventory; sum reducer; range queries |
| 3 | Example 43.3 (QADSAN) | Reducer-augmented table for NASDAQ; max reducer; time-range queries |

### Exercises

| # | Item | Description |
|---|------|-------------|
| 1 | Exercise 43.1 | Describe how to implement previous and next using other ordered set functions |

## Phase 3: Implementation Status per ADT Operation

### Ordered Sets (ADT 43.1 + base Set)

| # | Operation | StEph | StPer | MtEph |
|---|-----------|:-----:|:-----:|:-----:|
| 1 | size | Yes | Yes | Yes |
| 2 | empty | Yes | Yes | Yes |
| 3 | singleton | Yes | Yes | Yes |
| 4 | find | Yes | Yes | Yes |
| 5 | insert | Yes | Yes | Yes |
| 6 | delete | Yes | Yes | Yes |
| 7 | filter | Yes | Yes | Yes |
| 8 | intersection | Yes | Yes | Yes |
| 9 | union | Yes | Yes | Yes |
| 10 | difference | Yes | Yes | Yes |
| 11 | to_seq | Yes | Yes | Yes |
| 12 | from_seq | Yes | Yes | Yes |
| 13 | first | Yes | Yes | Yes |
| 14 | last | Yes | Yes | Yes |
| 15 | previous | Yes | Yes | Yes |
| 16 | next | Yes | Yes | Yes |
| 17 | split | Yes | Yes | Yes |
| 18 | join | Yes | Yes | Yes |
| 19 | get_range | Yes | Yes | Yes |
| 20 | rank | Yes | Yes | Yes |
| 21 | select | Yes | Yes | Yes |
| 22 | split_rank | Yes | Yes | Yes |

All 10 ordering operations implemented in all 3 ordered set variants.

### Ordered Tables (ADT 42.1 + ADT 43.1 for keys)

| # | Operation | StEph | StPer | MtEph | MtPer |
|---|-----------|:-----:|:-----:|:-----:|:-----:|
| 1 | size | Yes | Yes | Yes | Yes |
| 2 | empty | Yes | Yes | Yes | Yes |
| 3 | singleton | Yes | Yes | Yes | Yes |
| 4 | find | Yes | Yes | Yes | Yes |
| 5 | lookup | Yes | Yes | Yes | - |
| 6 | is_empty | Yes | - | Yes | - |
| 7 | insert | Yes | Yes | Yes | Yes |
| 8 | delete | Yes | Yes | Yes | Yes |
| 9 | domain | Yes | Yes | Yes | Yes |
| 10 | tabulate | Yes | Yes | Yes | - |
| 11 | map | Yes | Yes | Yes | Yes |
| 12 | filter | Yes | Yes | Yes | Yes |
| 13 | reduce | Yes | - | Yes | - |
| 14 | intersection | Yes | Yes | Yes | - |
| 15 | union | Yes | Yes | Yes | - |
| 16 | difference | Yes | Yes | Yes | - |
| 17 | restrict | Yes | Yes | Yes | - |
| 18 | subtract | Yes | Yes | Yes | - |
| 19 | collect | Yes | Yes | Yes | - |
| 20 | first_key | Yes | Yes | Yes | Yes |
| 21 | last_key | Yes | Yes | Yes | Yes |
| 22 | previous_key | Yes | Yes | Yes | Yes |
| 23 | next_key | Yes | Yes | Yes | Yes |
| 24 | split_key | Yes | Yes | Yes | Yes |
| 25 | join_key | Yes | Yes | Yes | Yes |
| 26 | get_key_range | Yes | Yes | Yes | Yes |
| 27 | rank_key | Yes | Yes | Yes | Yes |
| 28 | select_key | Yes | Yes | Yes | Yes |
| 29 | split_rank_key | Yes | Yes | Yes | Yes |

OrderedTableMtPer implements all 10 ordering ops plus base table ops (size, empty, singleton, find, insert, delete, domain, map, filter). No tabulate, reduce, intersection, union, difference, restrict, subtract, collect.

### Augmented Ordered Tables (ADT 43.3)

| # | Operation | StEph | StPer | MtEph |
|---|-----------|:-----:|:-----:|:-----:|
| 1 | All ordered table ops | Yes | Yes | Yes |
| 2 | reduce_val | Yes | Yes | Yes |
| 3 | reduce_range | Yes | Yes | Yes |
| 4 | reduce_range_parallel | - | - | Yes |

## Phase 4: Proof Holes Summary

```
Modules:   1 clean (Example43_1), 10 holed (external_body)
Holes Found: 283 total
   283 × external_body
```

All 10 ADT modules use `#[verifier::external_body]` on exec methods and the View `view` spec. This is the accepted verusification pattern: specs (`requires`/`ensures`) are verified; implementations are trusted at the boundary. Example43_1 has no Verus code and reports clean.

## Phase 5: Action Items

| # | Priority | Item | Notes |
|---|----------|------|-------|
| 1 | Medium | Replace `external_body` with verified bodies | Long-term: verify implementations instead of trusting specs |
| 2 | Medium | OrderedTableMtPer: add tabulate, reduce, bulk ops | Currently missing intersection, union, difference, restrict, subtract, collect |
| 3 | Low | Examples 43.2 (TRAMLAW), 43.3 (QADSAN) | Not implemented as standalone demos |
| 4 | Low | Exercise 43.1 | No solution code |
| 5 | Low | Cost fidelity | StEph/StPer ordering ops are O(n) via collect; MtEph/MtPer achieve O(lg n) with treap backing |

## Spec Strength Summary

| Classification | Count |
|----------------|------:|
| strong | 283 |
| partial | 0 |
| weak | 0 |
| none | 2 |

All 283 ADT functions inside `verus!` have `ensures` clauses (strong specs). Example43_1's 2 functions have no specs.

## Overall Assessment

**Chapter 43 is fully verusified.** All 10 ADT modules have code inside `verus!` with View impls (`Set<T>` for sets, `Map<K::V, V::V>` for tables), trait definitions, and exec methods with `#[verifier::external_body]` and `ensures` clauses. Verus verification: 1863 verified, 0 errors. RTT: 2698 passed.

### Strengths

1. **Complete verusification** — All ADT modules inside `verus!` with specs.
2. **Full ADT 43.1 coverage** — All 10 ordering operations (first, last, previous, next, split, join, getRange, rank, select, splitRank) implemented in ordered sets and tables.
3. **Correct `split_key` signature** — Returns `(Self, Option<V>, Self)` per APAS.
4. **OrderedTableMtPer** — Now implements all 10 ordering ops via ParamTreap backing.
5. **Augmented tables** — reduce_val, reduce_range, reduce_range_parallel with specs.
6. **Example43_1** — Demonstrates ordered set operations from prose.

### Weaknesses

1. **All exec code behind `external_body`** — Implementations are trusted; no verified bodies.
2. **OrderedTableMtPer** — Missing tabulate, reduce, intersection, union, difference, restrict, subtract, collect.
3. **Cost fidelity** — StEph/StPer use O(n) collect for ordering ops; only Mt variants achieve O(lg n) with treap.
4. **Examples 43.2, 43.3** — TRAMLAW and QADSAN not implemented as standalone demos.
