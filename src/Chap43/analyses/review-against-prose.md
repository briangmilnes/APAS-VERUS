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

## Phase 1: Inventory

Source files: 12 (3 ordered sets, 4 ordered tables, 3 augmented ordered tables, 1 ordered table Mt persistent, 1 example). All plain Rust, no `verus!` blocks.

| # | File | Functions | Traits | Trait Impls | Bare Impls | V! | -V! | NoSpec |
|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | `OrderedSetStEph.rs` | 21 | 1 (20 methods) | 1 (20 methods) | 0 | 0 | 21 | 21 |
| 2 | `OrderedSetStPer.rs` | 21 | 1 (20 methods) | 1 (20 methods) | 0 | 0 | 21 | 21 |
| 3 | `OrderedSetMtEph.rs` | 20 | 1 (20 methods) | 1 (20 methods) | 0 | 0 | 20 | 20 |
| 4 | `OrderedTableStEph.rs` | 30 | 1 (25 methods) | 1 (25 methods) | 0 | 0 | 30 | 30 |
| 5 | `OrderedTableStPer.rs` | 30 | 1 (25 methods) | 1 (25 methods) | 0 | 0 | 30 | 30 |
| 6 | `OrderedTableMtEph.rs` | 30 | 1 (25 methods) | 1 (25 methods) | 0 | 0 | 30 | 30 |
| 7 | `OrderedTableMtPer.rs` | 14 | 1 (14 methods) | 1 (14 methods) | 0 | 0 | 14 | 14 |
| 8 | `AugOrderedTableStEph.rs` | 20 | 1 (15 methods) | 1 (15 methods) | 1 (5 methods) | 0 | 20 | 20 |
| 9 | `AugOrderedTableStPer.rs` | 20 | 1 (15 methods) | 1 (15 methods) | 1 (5 methods) | 0 | 20 | 20 |
| 10 | `AugOrderedTableMtEph.rs` | 22 | 1 (16 methods) | 1 (16 methods) | 1 (6 methods) | 0 | 22 | 22 |
| 11 | `Example43_1.rs` | 3 | 0 | 0 | 0 | 0 | 3 | 3 |
| | **Total** | **231** | | | | **0** | **231** | **231** |

**Veracity Notes:** 2 bare-impl warnings on `AugOrderedTableStEph.rs` and `AugOrderedTableStPer.rs`. The bare impls contain `reduce_val`, `reduce_range`, `insert`, `delete`, and `recalculate_cached_reduction` which should be in the trait impl.

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|---|---|
| 1 | Data Type 43.1 (Ordered Sets) | Extends Set ADT with 10 ordering operations: `first`, `last`, `previous`, `next`, `split`, `join`, `getRange`, `rank`, `select`, `splitRank` |
| 2 | ADT 43.1 (Ordered Tables) | Same 10 ordering operations extended to tables: `first`, `last`, `previous`, `next`, `split`, `join`, `getRange`, `rank`, `select`, `splitRank`. Plus all base Table ops. |
| 3 | Definition 43.3 (Reducer-Augmented Ordered Table) | Extends ordered table with reducer `g:T×T→T` and identity `I`, cached reduction `r(a)`. Operations: `reduceVal`, plus maintaining cache on insert/delete. |

### Cost Specs

| # | Item | Description |
|---|---|---|
| 1 | Cost Specification 43.2 (Tree-Based) | All 10 ordering ops: O(lg n) work and span. Bulk ops (`intersection`, `union`, `difference`, `restrict`, `subtract`): O(m·lg(1+n/m)) work, O(lg²(n+m)) span. |

Specific ordering operation costs from APAS:

| # | Operation | APAS Work | APAS Span |
|---|---|---|---|
| 1 | `first(a)` / `last(a)` | O(lg\|a\|) | O(lg\|a\|) |
| 2 | `previous(a)(k)` / `next(a)(k)` | O(lg\|a\|) | O(lg\|a\|) |
| 3 | `split(a)(k)` | O(lg\|a\|) | O(lg\|a\|) |
| 4 | `join(a)(b)` | O(lg(\|a\|+\|b\|)) | O(lg(\|a\|+\|b\|)) |
| 5 | `getRange(a)(k₁)(k₂)` | O(lg\|a\|) | O(lg\|a\|) |
| 6 | `rank(a)(k)` | O(lg\|a\|) | O(lg\|a\|) |
| 7 | `select(a)(i)` | O(lg\|a\|) | O(lg\|a\|) |
| 8 | `splitRank(a)(i)` | O(lg\|a\|) | O(lg\|a\|) |
| 9 | `reduceVal(a)` | O(1) | O(1) |

### Examples

| # | Item | Description |
|---|---|---|
| 1 | Example 43.1 (Ordered Set) | `a = {1,3,5,7,9}`, demonstrating first, last, previous, next, split, rank, select, getRange |
| 2 | Example 43.2 (TRAMLAW) | Reducer-augmented table for inventory with max reducer |
| 3 | Example 43.3 (QADSAN) | Reducer-augmented table for NASDAQ quote ranges |

### Exercises

| # | Item | Description |
|---|---|---|
| 1 | Exercise 43.1 | 11 questions about ordered set/table operations on specific data |

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations — Ordered Sets

#### OrderedSetStEph.rs (delegates to `AVLTreeSetStEph`)

| # | Operation | APAS Work | APAS Span | Actual Work | Actual Span | Match |
|---|---|---|---|---|---|:---:|
| 1 | `first` | O(lg n) | O(lg n) | O(n) | O(n) | No |
| 2 | `last` | O(lg n) | O(lg n) | O(n) | O(n) | No |
| 3 | `previous` | O(lg n) | O(lg n) | O(n) | O(n) | No |
| 4 | `next` | O(lg n) | O(lg n) | O(n) | O(n) | No |
| 5 | `split` | O(lg n) | O(lg n) | O(n) | O(n) | No |
| 6 | `join` | O(lg(m+n)) | O(lg(m+n)) | O(m+n) | O(m+n) | No |
| 7 | `get_range` | O(lg n) | O(lg n) | O(n) | O(n) | No |
| 8 | `rank` | O(lg n) | O(lg n) | O(n) | O(n) | No |
| 9 | `select` | O(lg n) | O(lg n) | O(n) | O(n) | No |
| 10 | `split_rank` | O(lg n) | O(lg n) | O(n) | O(n) | No |

**All 10 ordering operations are O(n)** because they call `values_in_order()` (collects into a `Vec`) or convert to `AVLTreeSeqStPerS` and iterate. **0/10 match APAS costs.**

#### OrderedSetStPer.rs (delegates to `AVLTreeSetStPer`)

Same as StEph: all 10 ordering ops are O(n). **0/10 match.**

#### OrderedSetMtEph.rs (uses `ParamTreap<T>`)

| # | Operation | APAS Work | APAS Span | Actual Work | Actual Span | Match |
|---|---|---|---|---|---|:---:|
| 1 | `first` | O(lg n) | O(lg n) | O(lg n) | O(lg n) | Yes |
| 2 | `last` | O(lg n) | O(lg n) | O(lg n) | O(lg n) | Yes |
| 3 | `previous` | O(lg n) | O(lg n) | O(lg n) | O(lg n) | Yes |
| 4 | `next` | O(lg n) | O(lg n) | O(lg n) | O(lg n) | Yes |
| 5 | `split` | O(lg n) | O(lg n) | O(lg n) | O(lg n) | Yes |
| 6 | `join` | O(lg(m+n)) | O(lg(m+n)) | O(lg(m+n)) | O(lg(m+n)) | Yes |
| 7 | `get_range` | O(lg n) | O(lg n) | O(lg n) | O(lg n) | Yes |
| 8 | `rank` | O(lg n) | O(lg n) | O(lg n) | O(lg n) | Yes |
| 9 | `select` | O(lg n) | O(lg n) | **O(n)** | **O(n)** | **No** |
| 10 | `split_rank` | O(lg n) | O(lg n) | **O(n)** | **O(n)** | **No** |

**MtEph matches 8/10.** `select` and `split_rank` call `in_order()` (O(n)) instead of using the treap's rank operations directly.

### Phase 3a: Cost Annotations — Ordered Tables

#### OrderedTableStEph.rs (delegates to `TableStEph`)

All 10 ordering operations call `collect()` (O(n)) then linear scan. **0/10 match APAS costs.** Base table costs same as Chapter 42 (4/15 match).

#### OrderedTableStPer.rs (delegates to `TableStPer`)

Same as StEph. **0/10 ordering ops match.** Base table: 4/15 match.

#### OrderedTableMtEph.rs (delegates to `TableMtEph`)

Same as StEph. **0/10 ordering ops match.** Base table: 4/15 match.

#### OrderedTableMtPer.rs (uses `ParamTreap<Pair<K,V>>`)

Only implements 9 base table operations (size, empty, singleton, find, insert, delete, domain, map, filter). **None of the 10 ADT 43.1 ordering operations are implemented.** The `map` function has **filter semantics** — it takes a predicate and calls `self.tree.filter`, not a mapping function.

### Phase 3a: Cost Annotations — Augmented Ordered Tables

#### AugOrderedTableStEph.rs

| # | Operation | APAS Work | APAS Span | Actual Work | Actual Span | Match |
|---|---|---|---|---|---|:---:|
| 1 | `reduce_val` | O(1) | O(1) | O(1) | O(1) | Yes |
| 2 | `reduce_range` | O(lg n) | O(lg n) | O(n) | O(n) | No |
| 3 | `insert` | O(lg n) | O(lg n) | O(n) | O(n) | No |
| 4 | `delete` | O(lg n) | O(lg n) | O(n) | O(n) | No |

`reduce_val` returns cached value: O(1). `reduce_range` calls `get_key_range` (O(n)) then folds. `insert`/`delete` delegate to inner table then recalculate reduction (O(n) fold over all entries).

#### AugOrderedTableStPer.rs

Same pattern as StEph. **1/4 augmented ops match** (only `reduce_val`).

#### AugOrderedTableMtEph.rs

Same as StEph for `reduce_val` (O(1)) and `reduce_range` (O(n)). Adds `reduce_range_parallel` using `ParaPair!` but still depends on O(n) `get_key_range`. **1/4 match.**

### Phase 3b: Implementation Fidelity

#### Ordered Sets

| # | Operation | Prose | Implementation | Fidelity |
|---|---|---|---|---|
| 1 | `first(a)` | Smallest element | Converts to sorted vec/seq, takes first | Correct (slow) |
| 2 | `last(a)` | Largest element | Converts to sorted vec/seq, takes last | Correct (slow) |
| 3 | `previous(a)(k)` | Largest element < k | Linear scan of sorted sequence | Correct (slow) |
| 4 | `next(a)(k)` | Smallest element > k | Linear scan of sorted sequence | Correct (slow) |
| 5 | `split(a)(k)` | `({x∈a : x<k}, k∈a, {x∈a : x>k})` | Partition into three parts | Correct (slow) |
| 6 | `join(a)(b)` | Union when max(a) < min(b) | `self.union(&other)` (ignores ordering precondition) | Partial |
| 7 | `get_range(a)(k₁)(k₂)` | `{x∈a : k₁≤x≤k₂}` | Filter elements in range | Correct (slow) |
| 8 | `rank(a)(k)` | `\|{x∈a : x<k}\|` | Count elements less than k | Correct (slow) |
| 9 | `select(a)(i)` | i-th smallest element | Index sorted vec/seq | Correct (slow) |
| 10 | `split_rank(a)(i)` | `(first i elements, rest)` | Partition by index | Correct (slow) |

#### Ordered Tables

| # | Issue | Details |
|---|---|---|
| 1 | `split_key` signature | APAS returns `(Table, Option<V>, Table)`. StEph/StPer/MtEph return `(Self, Self)` — the found key's value is lost. |
| 2 | `join` | Prose requires `max(domain a) < min(domain b)`. Implementation is just `self.union(&other)` with no precondition check. |
| 3 | `OrderedTableMtPer::map` | **Semantic error:** has filter semantics. Takes a predicate `&dyn Fn(&Pair<K,V>) -> bool` and calls `self.tree.filter`. Should take `&dyn Fn(&V) -> V`. |

#### Augmented Tables

| # | Issue | Details |
|---|---|---|
| 1 | `reduce_range` cost | O(n) instead of O(lg n) because `get_key_range` collects all entries and linearly scans them. |
| 2 | Cache invalidation | `insert`/`delete` recalculate by folding over all entries: O(n) instead of O(lg n) incremental update. A balanced-tree backing would allow O(lg n) via subtree-cached reductions. |

### Phase 3c: Spec Fidelity

No Verus specs exist. All functions have `spec_strength = none`.

## Phase 4: Parallelism Review

### OrderedSetMtEph.rs (ParamTreap backing)

| # | Operation | Parallel? | Classification |
|---|---|---|---|
| 1 | `first` / `last` | No | Sequential (O(lg n) tree walk) |
| 2 | `previous` / `next` | No | Sequential (O(lg n) tree walk) |
| 3 | `split` / `join` | No | Sequential (O(lg n) tree ops) |
| 4 | `get_range` | No | Sequential (O(lg n)) |
| 5 | `rank` | No | Sequential (O(lg n)) |
| 6 | `select` / `split_rank` | No | Sequential (O(n) — collects to vec) |
| 7 | Base set ops (`union`, `intersection`, etc.) | **Yes** | Parallel via treap's parallel merge |

### OrderedTableMtEph.rs (delegates to TableMtEph)

Same as Chapter 42 MtEph parallelism: some 2-way spawns for filter/map, but intersection/union/difference are sequential.

### OrderedTableMtPer.rs (ParamTreap backing)

| # | Operation | Parallel? | Notes |
|---|---|---|---|
| 1 | `insert` / `delete` / `find` | No | Tree operations |
| 2 | `domain` | **Yes** | Uses `thread::spawn` for parallel traversal |
| 3 | `map` (has filter semantics) | **Yes** | Uses treap's parallel filter |
| 4 | `filter` | **Yes** | Uses treap's parallel filter |

No ordering operations implemented in this file.

### AugOrderedTableMtEph.rs

| # | Operation | Parallel? | Notes |
|---|---|---|---|
| 1 | `reduce_val` | No | O(1) cached lookup |
| 2 | `reduce_range` | No | Sequential fold |
| 3 | `reduce_range_parallel` | **Yes** | `ParaPair!` with 2-way split, but still O(n) total from `get_key_range` |

## Phase 5: Runtime Test Review

| # | Test File | Tests | Operations Covered |
|---|---|:---:|---|
| 1 | `TestOrderedSetStEph.rs` | ~15 | All 10 ordering ops + base set ops |
| 2 | `TestOrderedSetStPer.rs` | ~15 | All 10 ordering ops + base set ops |
| 3 | `TestOrderedSetMtEph.rs` | ~15 | All 10 ordering ops + base set ops + parallel ops |
| 4 | `TestOrderedTableStEph.rs` | ~20 | Base table ops + ordering ops |
| 5 | `TestOrderedTableStPer.rs` | ~20 | Base table ops + ordering ops |
| 6 | `TestOrderedTableMtEph.rs` | ~20 | Base table ops + ordering ops |
| 7 | `TestOrderedTableMtPer.rs` | ~12 | Base table ops (no ordering ops) |
| 8 | `TestAugOrderedTableStEph.rs` | ~10 | Augmented ops: reduce_val, reduce_range, insert, delete |
| 9 | `TestAugOrderedTableStPer.rs` | ~10 | Augmented ops |
| 10 | `TestAugOrderedTableMtEph.rs` | ~12 | Augmented ops + reduce_range_parallel |
| 11 | `TestExample43_1.rs` | 1 | Example 43.1 demonstration |
| | **Total RTT** | **~150** | |

### Test Gaps

| # | Gap | Severity |
|---|---|---|
| 1 | No tests for OrderedTableMtPer ordering operations (not implemented) | High |
| 2 | No tests verifying APAS Example 43.1 exact values against code output | Medium |
| 3 | No tests for APAS Example 43.2 (TRAMLAW augmented table) | Medium |
| 4 | No tests for APAS Example 43.3 (QADSAN augmented table) | Medium |
| 5 | No tests verifying `split_key` returns the found value | Medium |
| 6 | No tests for Exercise 43.1 answers | Low |

## Phase 6: PTT Review

No PTTs exist. No Verus code to test.

## Phase 7: Gap Analysis

### Prose Items Not Implemented

| # | Prose Item | Status | Notes |
|---|---|---|---|
| 1 | OrderedTableMtPer ordering ops | **Missing** | 10 operations from ADT 43.1 not implemented in MtPer variant |
| 2 | `split_key` returns `(Table, Option<V>, Table)` | **Wrong signature** | Implementations return `(Self, Self)`, losing the found value |
| 3 | Example 43.2 (TRAMLAW) | **Not implemented** | No code demonstrating inventory max reducer |
| 4 | Example 43.3 (QADSAN) | **Not implemented** | No code demonstrating NASDAQ quote ranges |
| 5 | Exercise 43.1 | **Not implemented** | No exercise solution code |
| 6 | `collect` for ordered tables | **Not implemented** | Same gap as Chapter 42 |

### Code with No Prose Counterpart

| # | Item | Kind | Notes |
|---|---|---|---|
| 1 | `OrderedTableMtPer::map` with filter semantics | Bug | Takes predicate, calls filter — not a mapping function |
| 2 | `reduce_range_parallel` in AugMtEph | Extension | Not in APAS prose; parallel version of reduce_range |
| 3 | `entries` / `collect` methods | Utility | Returns flat entries, not APAS collect |
| 4 | Macros (`OrderedSetStEphLit!`, etc.) | Convenience | Literal syntax |

## Phase 8: TOC Review

| # | File | Has TOC | Has Module Header | Copyright |
|---|---|:---:|:---:|:---:|
| 1 | `OrderedSetStEph.rs` | No | Yes | Yes |
| 2 | `OrderedSetStPer.rs` | No | Yes | Yes |
| 3 | `OrderedSetMtEph.rs` | No | Yes | Yes |
| 4 | `OrderedTableStEph.rs` | No | Yes | Yes |
| 5 | `OrderedTableStPer.rs` | No | Yes | Yes |
| 6 | `OrderedTableMtEph.rs` | No | Yes | Yes |
| 7 | `OrderedTableMtPer.rs` | No | Yes | Yes |
| 8 | `AugOrderedTableStEph.rs` | No | Yes | Yes |
| 9 | `AugOrderedTableStPer.rs` | No | Yes | Yes |
| 10 | `AugOrderedTableMtEph.rs` | No | Yes | Yes |
| 11 | `Example43_1.rs` | No | Yes | Yes |

### In/Out Table

| # | File | Clone | PartialEq/Eq | Debug | Display | Macro |
|---|---|:---:|:---:|:---:|:---:|:---:|
| 1 | `OrderedSetStEph.rs` | ❌ out | ❌ out | ❌ out | - | ✅ out |
| 2 | `OrderedSetStPer.rs` | ❌ out | ❌ out | ❌ out | - | ✅ out |
| 3 | `OrderedSetMtEph.rs` | ❌ out | ❌ out | ❌ out | - | ✅ out |
| 4 | `OrderedTableStEph.rs` | ❌ out | ❌ out | ❌ out | - | ✅ out |
| 5 | `OrderedTableStPer.rs` | ❌ out | ❌ out | ❌ out | - | ✅ out |
| 6 | `OrderedTableMtEph.rs` | ❌ out | ❌ out | ❌ out | - | ✅ out |
| 7 | `OrderedTableMtPer.rs` | ❌ out | ❌ out | ❌ out | - | - |
| 8 | `AugOrderedTableStEph.rs` | ❌ out | ❌ out | ❌ out | - | ✅ out |
| 9 | `AugOrderedTableStPer.rs` | ❌ out | ❌ out | ❌ out | - | ✅ out |
| 10 | `AugOrderedTableMtEph.rs` | ❌ out | ❌ out | ❌ out | - | ✅ out |

No verus! blocks exist, so the "should be inside" classification is aspirational.

## Proof Holes Summary

```
Modules:   11 clean (no holes), 0 holed
Holes Found: 0 total
Veracity Errors: 2 bare impl(s) in AugOrderedTableStEph.rs and AugOrderedTableStPer.rs
```

Zero proof holes — vacuously clean since no Verus code exists. Two bare-impl warnings from veracity.

## Spec Strength Summary

| Classification | Count |
|---|---|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 231 |

All functions lack Verus specifications.

## Overall Assessment

**Chapter 43 implements ordered sets (3 variants), ordered tables (4 variants), and augmented ordered tables (3 variants) across 11 source files with ~150 runtime tests. No Verus verification exists. OrderedTableMtPer is critically incomplete, missing all 10 ordering operations. OrderedTableMtPer::map has filter semantics (bug).**

### Strengths

1. **OrderedSetMtEph achieves O(lg n)** for 8/10 ordering operations using ParamTreap — the best cost profile in this chapter.
2. All ordered set variants semantically implement all 10 ADT 43.1 operations.
3. Augmented table `reduce_val` correctly returns cached reduction in O(1).
4. Comprehensive test coverage: ~150 RTTs across 11 test files.
5. Example43_1 demonstrates basic ordered set operations.

### Weaknesses

1. **No Verus verification** — zero functions inside `verus!`, zero specs.
2. **StEph/StPer ordering operations are all O(n)** due to collecting into sorted vectors. The balanced-tree backing is present (`AVLTreeSetStEph/StPer`) but unused for navigation.
3. **OrderedTableMtPer is critically incomplete** — only base table ops, no ordering operations, and `map` has wrong semantics (filter instead of map).
4. **`split_key` returns wrong type** — `(Self, Self)` instead of `(Self, Option<V>, Self)`, discarding the found key's value.
5. **Augmented table insert/delete are O(n)** — recalculate full reduction instead of O(lg n) incremental update.
6. **2 bare-impl veracity warnings** in AugOrderedTableStEph and AugOrderedTableStPer.
7. **No TOC headers** in any source file.
8. **Examples 43.2 (TRAMLAW) and 43.3 (QADSAN) not implemented.**
9. **Exercise 43.1 not implemented.**
10. **MtEph ordered table parallelism is inherited from Chapter 42** — shallow 2-way splits; intersection/union/difference still sequential.
