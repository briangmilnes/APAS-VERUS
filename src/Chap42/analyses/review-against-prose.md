<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 42: Tables — Review Against Prose

**Date:** 2026-03-01
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap42.txt` (Data Type 42.1, Algorithm 42.3, Cost Specification 42.5, Example 42.1)

## Phase 1: Inventory

Source files: 4 (`TableStEph.rs`, `TableStPer.rs`, `TableMtEph.rs`, `Example42_1.rs`). All have `verus!` blocks. StPer is the most verified; StEph and MtEph are fully `external_body`.

| # | Chap | File | Fns | Tr | IT | ML | V! | -V! | Unk | Hole | NoSpec |
|---|:---:|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | 42 | `TableStEph.rs` | 20 | 16 | 18 | 2 | 18 | 2 | 3 | 14 | 3 |
| 2 | 42 | `TableStPer.rs` | 26 | 16 | 17 | 9 | 25 | 1 | 14 | 10 | 2 |
| 3 | 42 | `TableMtEph.rs` | 19 | 16 | 17 | 2 | 18 | 1 | 3 | 14 | 2 |
| 4 | 42 | `Example42_1.rs` | 4 | 2 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| | | **Total** | **69** | | | | **62** | **7** | **20** | **38** | **11** |

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|---|---|
| 1 | Data Type 42.1 (Tables) | ADT with 15 operations: `size`, `empty`, `singleton`, `domain`, `tabulate`, `map`, `filter`, `intersection`, `union`, `difference`, `find`, `delete`, `insert`, `restrict`, `subtract` |
| 2 | Syntax 42.2 (Table Notation) | `{k1->v1, k2->v2, ...}` for table literals |
| 3 | Syntax 42.4 (Table Shorthands) | `a[k]`, `a \ m`, `a U b` etc. |

### Algorithms

| # | Item | Description |
|---|---|---|
| 1 | Algorithm 42.3 (collect) | `collect a = Seq.reduce (Table.union Seq.append) {} <{k-><v>} : (k,v) in a>` -- groups values by key, producing `Table<K, Seq<V>>` |

### Cost Specs

| # | Operation | APAS Work | APAS Span |
|---|---|---|---|
| 1 | `size a` | O(1) | O(1) |
| 2 | `singleton(k,v)` | O(1) | O(1) |
| 3 | `domain a` | O(\|a\|) | O(lg\|a\|) |
| 4 | `filter p a` | O(Sum W(p(k,v))) | O(lg\|a\| + max S(p)) |
| 5 | `map f a` | O(Sum W(f(v))) | O(lg\|a\| + max S(f)) |
| 6 | `find a k` | O(lg\|a\|) | O(lg\|a\|) |
| 7 | `delete a k` | O(lg\|a\|) | O(lg\|a\|) |
| 8 | `insert f a (k,v)` | O(lg\|a\|) | O(lg\|a\|) |
| 9 | `intersection f a b` | O(m*lg(1+n/m)) | O(lg(n+m)) |
| 10 | `difference a b` | O(m*lg(1+n/m)) | O(lg(n+m)) |
| 11 | `union f a b` | O(m*lg(1+n/m)) | O(lg(n+m)) |
| 12 | `restrict a c` | O(m*lg(1+n/m)) | O(lg(n+m)) |
| 13 | `subtract a c` | O(m*lg(1+n/m)) | O(lg(n+m)) |

### Examples

| # | Item | Description |
|---|---|---|
| 1 | Example 42.1 | Tables `a = {'a'->4, 'b'->11, 'c'->2}`, `b = {'b'->3, 'd'->5}`, `c = {3,5,7}` with 7 query demonstrations |

### Exercises

None in this chapter.

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

All implementations use a **sorted array** (`ArraySeqStEphS` / `ArraySeqStPerS` / `ArraySeqMtEphS` of `Pair<K,V>`) as the backing store. The APAS Cost Specification 42.5 assumes a balanced BST, giving O(lg n) for single-element operations. The sorted-array choice limits achievable costs.

#### TableStEph.rs

| # | Operation | APAS Work | APAS Span | Actual Work | Actual Span | Match |
|---|---|---|---|---|---|:---:|
| 1 | `size` | O(1) | O(1) | O(1) | O(1) | Yes |
| 2 | `empty` | O(1) | O(1) | O(1) | O(1) | Yes |
| 3 | `singleton` | O(1) | O(1) | O(1) | O(1) | Yes |
| 4 | `domain` | O(\|a\|) | O(lg\|a\|) | O(n^2) | O(n^2) | No |
| 5 | `tabulate` | O(\|s\|*W(f)) | O(lg\|s\|+S(f)) | O(\|s\|*W(f)+n*lg n) | O(n*(W(f)+lg n)) | Partial |
| 6 | `map` | O(Sum W(f(v))) | O(lg\|a\|+max S(f)) | O(n*W(f)) | O(n*S(f)) | Partial |
| 7 | `filter` | O(Sum W(p(k,v))) | O(lg\|a\|+max S(p)) | O(n*W(p)) | O(n*S(p)) | Partial |
| 8 | `intersection` | O(m*lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No |
| 9 | `union` | O(m*lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No |
| 10 | `difference` | O(m*lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No |
| 11 | `find` | O(lg\|a\|) | O(lg\|a\|) | O(lg n) | O(lg n) | Yes |
| 12 | `delete` | O(lg\|a\|) | O(lg\|a\|) | O(n) | O(n) | No |
| 13 | `insert` | O(lg\|a\|) | O(lg\|a\|) | O(n) | O(n) | No |
| 14 | `restrict` | O(m*lg(1+n/m)) | O(lg(n+m)) | O(n*lg m) | O(n*lg m) | No |
| 15 | `subtract` | O(m*lg(1+n/m)) | O(lg(n+m)) | O(n*lg m) | O(n*lg m) | No |

**StEph cost match: 4/15** (size, empty, singleton, find).

#### TableStPer.rs

StPer `find` uses linear scan O(n), not binary search. `union` iterates over other and calls `insert` for each entry: O(m*n) worst case. `intersection` and `difference` also use linear scan for key lookup: O(n*m). **StPer cost match: 3/15** (size, empty, singleton).

#### TableMtEph.rs

| # | Operation | Parallel? | Notes |
|---|---|:---:|---|
| 1 | `domain` | Yes | 2-way spawn/join for key extraction |
| 2 | `tabulate` | Yes | 2-way spawn/join + O(n lg n) sort |
| 3 | `map` | Yes | 2-way spawn/join |
| 4 | `filter` | Yes | 2-way spawn/join |
| 5 | `intersection` | **No** | Sequential sorted merge |
| 6 | `union` | **No** | Sequential sorted merge |
| 7 | `difference` | **No** | Sequential sorted merge |
| 8 | `find` | No | Binary search (inherently sequential) |
| 9 | `delete` | Yes | 2-way spawn/join filter |
| 10 | `insert` | Yes | 2-way spawn/join tabulate + sort |
| 11 | `restrict` | Yes | 2-way spawn/join filter |
| 12 | `subtract` | Yes | 2-way spawn/join filter |
| 13 | `entries` | No | Clone |

**MtEph cost match: 4/15.** Intersection, union, and difference are entirely sequential. Parallel operations use a single 2-way split, not recursive divide-and-conquer.

### Phase 3b: Implementation Fidelity

| # | Operation | Prose Definition | Implementation | Fidelity |
|---|---|---|---|---|
| 1 | `size(a)` | `\|a\|` | `entries.length()` | High |
| 2 | `empty` | `{}` | Empty backing array | High |
| 3 | `singleton(k,v)` | `{k->v}` | Single-entry array | High |
| 4 | `domain(a)` | Set of all keys | Extract keys into ArraySetStEph | High |
| 5 | `tabulate(f)(a:S)` | `{k->f(k) : k in a}` | Apply f to keys, sort | High |
| 6 | `map(f)(a)` | `{k->f(v) : (k->v) in a}` | Apply f to values | High |
| 7 | `filter(f)(a)` | `{(k->v) in a \| f(k,v)}` | Filter entries | High |
| 8 | `intersection(f)(a)(b)` | `{k->f(find a k, find b k)}` | Sorted merge / linear scan | High |
| 9 | `union(f)(a)(b)` | `intersect U diff(a,b) U diff(b,a)` | StPer: iter+insert; StEph/MtEph: merge | High |
| 10 | `difference(a)(b)` | `{(k->v) in a \| k not in dom(b)}` | Sorted merge / linear scan | High |
| 11 | `find(a)(k)` | `v if (k->v) in a, bot otherwise` | Binary search (StEph/MtEph), linear (StPer) | High |
| 12 | `delete(a)(k)` | `{(k'->v') in a \| k != k'}` | Filter out key | High |
| 13 | `insert(f)(a)(k,v)` | `union f a (singleton(k,v))` | Direct implementation with combine | High |
| 14 | `restrict(a)(b:S)` | `{k->v in a \| k in b}` | Filter by set membership | High |
| 15 | `subtract(a)(b:S)` | `{(k->v) in a \| k not in b}` | Filter by set non-membership | High |

**Semantic fidelity: 15/15 operations faithful to prose.**

### Phase 3c: Spec Fidelity

#### TableStEph.rs — Spec Strength

All 14 ADT impl functions are `external_body`. Trait specs exist but are unverified.

| # | Operation | Spec | Strength |
|---|---|---|---|
| 1 | `size` | `result == self@.len()` | strong (holed) |
| 2 | `empty` | `result@ == Map::empty()` | strong |
| 3 | `singleton` | `result@ == Map::empty().insert(key@, value@)` | strong (holed) |
| 4 | `domain` | `result@.finite()` | weak (holed) |
| 5 | `tabulate` | `result@.dom().finite()` | weak (holed) |
| 6 | `map` | `self@.dom() == old(self)@.dom()` | partial (holed) |
| 7 | `filter` | `self@.dom().subset_of(old(self)@.dom())` | partial (holed) |
| 8 | `intersection` | `dom subset_of intersect` | partial (holed) |
| 9 | `union` | `dom.union subset_of self@.dom()` | partial (holed) |
| 10 | `difference` | `dom subset_of difference` | partial (holed) |
| 11 | `find` | `contains_key => value match` | strong (holed) |
| 12 | `delete` | `!self@.contains_key(key@)` | partial (holed) |
| 13 | `insert` | `self@.contains_key(key@)` | partial (holed) |
| 14 | `restrict` | `dom subset_of old dom` | partial (holed) |
| 15 | `subtract` | `dom subset_of old dom` | partial (holed) |
| 16 | `entries` | none | none |

**StEph: 3 strong, 8 partial, 2 weak, 1 none. All impl bodies are external_body (14 holes).**

#### TableStPer.rs — Spec Strength

StPer has the most verification. 8 proof lemmas support the implementations. Functions are verified (not `external_body`) but use `assume` for `obeys_view_eq` and `obeys_feq_full`.

| # | Operation | Spec | Strength |
|---|---|---|---|
| 1 | `size` | `result == self@.len()` (with spec_wf) | strong (verified) |
| 2 | `empty` | `result@ == Map::empty(), result.spec_wf()` | strong (verified) |
| 3 | `singleton` | `result@ == Map::empty().insert(key@, value@), result.spec_wf()` | strong (verified) |
| 4 | `domain` | `result@.finite()` | weak (verified) |
| 5 | `tabulate` | `result@.dom().finite()` | weak (verified) |
| 6 | `map` | `result@.dom() == self@.dom()` | partial (assume x1) |
| 7 | `filter` | `result@.dom().subset_of(self@.dom())` | partial (assume x1) |
| 8 | `intersection` | `dom subset_of intersect` | partial (assume x2) |
| 9 | `union` | `dom.union subset_of result@.dom(), result.spec_wf()` | partial (assume x2) |
| 10 | `difference` | `dom subset_of difference` | partial (assume x2) |
| 11 | `find` | `contains_key => value match` (with spec_wf) | strong (assume x2) |
| 12 | `delete` | `!result@.contains_key(key@), result.spec_wf()` | partial (assume x2) |
| 13 | `insert` | `result@.contains_key(key@), result.spec_wf()` | partial (assume x2) |
| 14 | `restrict` | `dom subset_of self dom` | partial (assume x1) |
| 15 | `subtract` | `dom subset_of self dom` | partial (assume x1) |
| 16 | `collect` | none | none |

**StPer: 4 strong, 9 partial, 2 weak, 1 none. 16 assume() holes (all obeys_view_eq/obeys_feq_full). 1 clone assume (Verus workaround).**

#### TableMtEph.rs — Spec Strength

All 15 ADT impl functions are `external_body`. Specs are weaker than StEph: MtEph intersection/union/difference/find/delete/insert/restrict/subtract only ensure `dom().finite()`.

| # | Operation | Spec | Strength |
|---|---|---|---|
| 1 | `size` | `result == self@.dom().len()` | strong (holed) |
| 2 | `empty` | `result@ == Map::empty()` | strong |
| 3 | `singleton` | `dom.finite(), dom.len() == 1` | partial (holed) |
| 4 | `domain` | `result@.finite()` | weak (holed) |
| 5 | `tabulate` | `result@.dom().finite()` | weak (holed) |
| 6 | `map` | `self@.dom() == old(self)@.dom()` | partial (holed) |
| 7 | `filter` | `self@.dom().subset_of(old(self)@.dom())` | partial (holed) |
| 8 | `intersection` | `self@.dom().finite()` | weak (holed) |
| 9 | `union` | `self@.dom().finite()` | weak (holed) |
| 10 | `difference` | `self@.dom().finite()` | weak (holed) |
| 11 | `find` | `self@.dom().finite()` | weak (holed) |
| 12 | `delete` | `self@.dom().finite()` | weak (holed) |
| 13 | `insert` | `self@.dom().finite()` | weak (holed) |
| 14 | `restrict` | `self@.dom().finite()` | weak (holed) |
| 15 | `subtract` | `self@.dom().finite()` | weak (holed) |
| 16 | `entries` | none | none |

**MtEph: 2 strong, 2 partial, 10 weak, 1 none. All impl bodies external_body (15 holes). MtEph find spec is wrong -- says dom().finite() instead of key/value match.**

## Phase 4: Parallelism Review

| # | Operation | MtEph Parallelism | Classification |
|---|---|---|---|
| 1 | `domain` | 2-way spawn/join | Parallel (shallow) |
| 2 | `tabulate` | 2-way spawn/join | Parallel (shallow) |
| 3 | `map` | 2-way spawn/join | Parallel (shallow) |
| 4 | `filter` | 2-way spawn/join | Parallel (shallow) |
| 5 | `intersection` | None | **Sequential** |
| 6 | `union` | None | **Sequential** |
| 7 | `difference` | None | **Sequential** |
| 8 | `find` | Binary search | Sequential (inherent) |
| 9 | `delete` | 2-way spawn/join | Parallel (shallow) |
| 10 | `insert` | 2-way spawn/join | Parallel (shallow) |
| 11 | `restrict` | 2-way spawn/join | Parallel (shallow) |
| 12 | `subtract` | 2-way spawn/join | Parallel (shallow) |
| 13 | `entries` | Clone | Sequential |

MtEph intersection, union, and difference are sequential despite being in the Mt file. All parallel operations use only a single 2-way split, not recursive divide-and-conquer to achieve O(lg n) span.

## Phase 5: Runtime Test Review

| # | Chap | File | Tests | Operations Covered |
|---|:---:|---|:---:|---|
| 1 | 42 | `TestTableStEph.rs` | 18 | All 15 ADT ops + ephemeral semantics, macro, large ops |
| 2 | 42 | `TestTableStPer.rs` | 18 | All 15 ADT ops + persistence, macro, empty ops, combine |
| 3 | 42 | `TestTableMtEph.rs` | 19 | All 15 ADT ops + ephemeral semantics, macro, parallel ops |
| 4 | 42 | `TestExample42_1.rs` | 2 | Smoke tests: `example_42_1()` and `performance_comparison()` |
| | | **Total RTT** | **57** | |

### Test Gaps

| # | Gap | Severity |
|---|---|---|
| 1 | No edge-case tests for empty intersection/union/difference | Low |
| 2 | No tests verifying sorted invariant after operations | Low |
| 3 | No tests matching APAS Example 42.1 exact values | Low |

## Phase 6: PTT Review

No PTTs exist. Not needed at current verification level.

## Phase 7: Gap Analysis

### Prose Items Not Implemented

| # | Prose Item | Status | Notes |
|---|---|---|---|
| 1 | Algorithm 42.3 (collect) | **Not implemented** | APAS collect takes `Seq<(K,V)>` -> `Table<K, Seq<V>>`, grouping by key. StPer has `collect` method but it just returns flat entries. |

### Code with No Prose Counterpart

| # | Item | Kind | Notes |
|---|---|---|---|
| 1 | `entries/collect -> Seq<Pair<K,V>>` | Utility | Returns flat entries; not APAS collect |
| 2 | `from_sorted_entries(Vec<Pair<K,V>>)` | Constructor | Helper for macros |
| 3 | `TableStEphLit!/TableStPerLit!/TableMtEphLit!` | Macros | Convenience literal syntax |
| 4 | `Example42_1` module | Example | Uses integer keys, not APAS exact values |

## Phase 8: TOC Review

| # | Chap | File | Has TOC | Has Module Header | Copyright |
|---|:---:|---|:---:|:---:|:---:|
| 1 | 42 | `TableStEph.rs` | Yes | Yes | Yes |
| 2 | 42 | `TableStPer.rs` | Yes | Yes | Yes |
| 3 | 42 | `TableMtEph.rs` | Yes | Yes | Yes |
| 4 | 42 | `Example42_1.rs` | No | Yes | Yes |

### In/Out Table

| # | Chap | File | Clone | PartialEq/Eq | Debug | Macro |
|---|:---:|---|:---:|:---:|:---:|:---:|
| 1 | 42 | `TableStEph.rs` | Y in | X out | Y out | Y out |
| 2 | 42 | `TableStPer.rs` | Y in | X out | Y out | Y out |
| 3 | 42 | `TableMtEph.rs` | Y in (ext_body) | X out | Y out | Y out |
| 4 | 42 | `Example42_1.rs` | - | - | - | - |

PartialEq is outside `verus!` in all files (style warning).

## Proof Holes Summary

```
Modules:   1 clean (no holes), 3 holed
Holes Found: 45 total
   16 x assume()      (TableStPer)
   29 x external_body (14 TableStEph + 15 TableMtEph)
Errors: 1 total (1 assume in eq/clone, Verus workaround)
Proof Functions: 11 total (11 clean, 0 holed)
```

### Assume Pattern Analysis (TableStPer)

All 16 assumes fall into two categories:
- `assume(obeys_view_eq::<K>())` -- 7 occurrences (map, intersection, difference, find, delete, insert)
- `assume(obeys_feq_full::<...>())` -- 9 occurrences (map, filter, intersection, difference, find, delete, insert, restrict, subtract)

These are generic trait bound assumptions needed because Verus cannot yet prove that `View`-implementing types preserve equality through their view, or that `clone_plus` preserves the view. These are structural Verus limitations, not logical gaps.

## Spec Strength Summary

| Classification | StEph | StPer | MtEph | Total |
|---|:---:|:---:|:---:|:---:|
| strong | 3 | 4 | 2 | 9 |
| partial | 8 | 9 | 2 | 19 |
| weak | 2 | 2 | 10 | 14 |
| none | 3 | 1 | 2 | 6 |

## Overall Assessment

**Chapter 42 implements all 15 operations from Data Type 42.1 across 3 variants (StEph, StPer, MtEph) with 57 runtime tests. TableStPer has the most verification work: all functions verified inside `verus!` with loop invariants and proof lemmas, though 16 assumes remain for generic trait bounds. TableStEph and TableMtEph have trait-level specs but all implementations are `external_body` (29 total). MtEph specs are notably weaker than StEph, with 10 functions only ensuring `dom().finite()`. Algorithm 42.3 (collect) is not implemented.**

### Strengths

1. All 15 ADT operations semantically faithful to the prose across all 3 variants.
2. TableStPer is substantially verified: 8 proof lemmas, loop invariants, spec_wf tracking.
3. Ephemeral vs persistent semantics correctly distinguished (`&mut self` vs returns `Self`).
4. Good test coverage: 57 RTTs covering all operations, semantics, macros, and parallelism.
5. TOC headers present in main source files.

### Weaknesses

1. **StEph/MtEph fully external_body** -- 29 holes. No implementation-level verification.
2. **MtEph specs too weak** -- 10 of 15 operations only ensure `dom().finite()`. `find` spec says nothing about the returned value.
3. **16 assumes in StPer** for obeys_view_eq/obeys_feq_full. Structural Verus limitation but still proof debt.
4. **Sorted-array backing** gives O(n) insert/delete instead of O(lg n). Only 3-4/15 operations match APAS costs.
5. **Algorithm 42.3 (collect) not implemented.** StPer `collect` returns flat entries, not grouped-by-key tables.
6. **MtEph intersection/union/difference are sequential** despite being in the multi-threaded file.
7. **MtEph parallelism is shallow** -- single 2-way splits, not recursive divide-and-conquer.
8. **PartialEq outside verus!** in all files (style warning).
9. **Example42_1.rs** uses integer keys with string values instead of matching the prose.
