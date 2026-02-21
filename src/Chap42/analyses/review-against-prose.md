<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 42: Tables — Review Against Prose

**Date:** 2026-02-19
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap42.txt` (Data Type 42.1, Algorithm 42.3, Cost Specification 42.5, Example 42.1)

## Phase 1: Inventory

Source files: 4 (`TableStEph.rs`, `TableStPer.rs`, `TableMtEph.rs`, `Example42_1.rs`). All plain Rust, no `verus!` blocks.

| # | File | Functions | Traits | Trait Impls | Bare Impls | V! | -V! | NoSpec |
|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | `TableStEph.rs` | 17 | 1 (15 methods) | 1 (15 methods) | 0 | 0 | 17 | 17 |
| 2 | `TableStPer.rs` | 17 | 1 (16 methods) | 1 (16 methods) | 0 | 0 | 17 | 17 |
| 3 | `TableMtEph.rs` | 17 | 1 (15 methods) | 1 (15 methods) | 0 | 0 | 17 | 17 |
| 4 | `Example42_1.rs` | 3 | 1 (2 methods) | 0 | 0 | 0 | 3 | 3 |
| | **Total** | **54** | | | | **0** | **54** | **54** |

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|---|---|
| 1 | Data Type 42.1 (Tables) | ADT with 15 operations: `size`, `empty`, `singleton`, `domain`, `tabulate`, `map`, `filter`, `intersection`, `union`, `difference`, `find`, `delete`, `insert`, `restrict`, `subtract` |
| 2 | Syntax 42.2 (Table Notation) | `{k₁→v₁, k₂→v₂, ...}` for table literals |
| 3 | Syntax 42.4 (Table Shorthands) | `a[k]`, `a \ m`, `a ∪ b` etc. |

### Algorithms

| # | Item | Description |
|---|---|---|
| 1 | Algorithm 42.3 (collect) | `collect a = Seq.reduce (Table.union Seq.append) {} ⟨{k→⟨v⟩} : (k,v)∈a⟩` — takes a *sequence* of key-value pairs, groups values by key, producing `Table<K, Seq<V>>` |

### Cost Specs

| # | Operation | APAS Work | APAS Span |
|---|---|---|---|
| 1 | `size a` | O(1) | O(1) |
| 2 | `singleton(k,v)` | O(1) | O(1) |
| 3 | `domain a` | O(\|a\|) | O(lg\|a\|) |
| 4 | `filter p a` | O(Σ W(p(k,v))) | O(lg\|a\| + max S(p)) |
| 5 | `map f a` | O(Σ W(f(v))) | O(lg\|a\| + max S(f)) |
| 6 | `find a k` | O(lg\|a\|) | O(lg\|a\|) |
| 7 | `delete a k` | O(lg\|a\|) | O(lg\|a\|) |
| 8 | `insert f a (k,v)` | O(lg\|a\|) | O(lg\|a\|) |
| 9 | `intersection f a b` | O(m·lg(1+n/m)) | O(lg(n+m)) |
| 10 | `difference a b` | O(m·lg(1+n/m)) | O(lg(n+m)) |
| 11 | `union f a b` | O(m·lg(1+n/m)) | O(lg(n+m)) |
| 12 | `restrict a c` | O(m·lg(1+n/m)) | O(lg(n+m)) |
| 13 | `subtract a c` | O(m·lg(1+n/m)) | O(lg(n+m)) |

### Examples

| # | Item | Description |
|---|---|---|
| 1 | Example 42.1 | Tables `a = {'a'→4, 'b'→11, 'c'→2}`, `b = {'b'→3, 'd'→5}`, `c = {3,5,7}` with 7 query demonstrations |

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
| 4 | `domain` | O(\|a\|) | O(lg\|a\|) | O(n²) | O(n²) | No |
| 5 | `tabulate` | O(\|s\|·W(f)) | O(lg\|s\|+S(f)) | O(\|s\|·W(f) + n log n) | O(n·(W(f)+log n)) | Partial |
| 6 | `map` | O(Σ W(f(v))) | O(lg\|a\|+max S(f)) | O(n·W(f)) | O(n·S(f)) | Partial |
| 7 | `filter` | O(Σ W(p(k,v))) | O(lg\|a\|+max S(p)) | O(n·W(p)) | O(n·S(p)) | Partial |
| 8 | `intersection` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No |
| 9 | `union` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No |
| 10 | `difference` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No |
| 11 | `find` | O(lg\|a\|) | O(lg\|a\|) | O(log n) | O(log n) | Yes |
| 12 | `delete` | O(lg\|a\|) | O(lg\|a\|) | O(n) | O(n) | No |
| 13 | `insert` | O(lg\|a\|) | O(lg\|a\|) | O(n) | O(n) | No |
| 14 | `restrict` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(n·log m) | O(n·log m) | No |
| 15 | `subtract` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(n·log m) | O(n·log m) | No |

**StEph cost match: 4/15** (size, empty, singleton, find).

#### TableStPer.rs

Costs are similar to StEph. **StPer cost match: 4/15.** Note: StPer `union` is O((m+n)log(m+n)) due to re-sorting after intersection + 2 differences decomposition — worse than a direct merge.

#### TableMtEph.rs

| # | Operation | Parallel? | Notes |
|---|---|:---:|---|
| 1 | `domain` | Yes | 2-way spawn/join for key extraction, but sequential `ArraySetStEph` insert is O(n²) |
| 2 | `tabulate` | Yes | 2-way spawn/join + O(n log n) sort |
| 3 | `map` | Yes | 2-way spawn/join |
| 4 | `filter` | Yes | 2-way spawn/join |
| 5 | `intersection` | **No** | Sequential merge despite Mt file |
| 6 | `union` | **No** | Sequential merge |
| 7 | `difference` | **No** | Sequential merge |
| 8 | `find` | No | Binary search (inherently sequential) |
| 9 | `delete` | Yes | 2-way spawn/join filter |
| 10 | `insert` | Yes | 2-way spawn/join tabulate + sort |
| 11 | `restrict` | Yes | 2-way spawn/join filter |
| 12 | `subtract` | Yes | 2-way spawn/join filter |
| 13 | `entries` | No | Clone |

**MtEph cost match: 4/15.** Intersection, union, and difference are entirely sequential. Only a single 2-way split, not recursive divide-and-conquer.

### Phase 3b: Implementation Fidelity

| # | Operation | Prose Definition | Implementation | Fidelity |
|---|---|---|---|---|
| 1 | `size(a)` | `\|a\|` | `entries.length()` | High |
| 2 | `empty` | `∅` | Empty backing array | High |
| 3 | `singleton(k,v)` | `{k→v}` | Single-entry array | High |
| 4 | `domain(a)` | Set of all keys | Extract keys into ArraySetStEph | High |
| 5 | `tabulate(f)(a:S)` | `{k→f(k) : k∈a}` | Apply f to keys, sort | High |
| 6 | `map(f)(a)` | `{k→f(v) : (k→v)∈a}` | Apply f to values | High |
| 7 | `filter(f)(a)` | `{(k→v)∈a \| f(k,v)}` | Filter entries | High |
| 8 | `intersection(f)(a)(b)` | `{k→f(find a k, find b k) : k∈dom(a)∩dom(b)}` | Sorted merge | High |
| 9 | `union(f)(a)(b)` | `(intersection f a b) ∪ (diff a b) ∪ (diff b a)` | StPer: decomposition; StEph/MtEph: direct merge | High |
| 10 | `difference(a)(b)` | `{(k→v)∈a \| k∉dom(b)}` | Sorted merge | High |
| 11 | `find(a)(k)` | `v if (k→v)∈a, ⊥ otherwise` | Binary search, `Option<V>` | High |
| 12 | `delete(a)(k)` | `{(k'→v')∈a \| k≠k'}` | Filter out key | High |
| 13 | `insert(f)(a)(k,v)` | `union f a (singleton(k,v))` | Direct implementation with combine | High |
| 14 | `restrict(a)(b:S)` | `{k→v∈a \| k∈b}` | Filter by set membership | High |
| 15 | `subtract(a)(b:S)` | `{(k→v)∈a \| k∉b}` | Filter by set non-membership | High |

**Semantic fidelity: 15/15 operations faithful to prose.**

### Phase 3c: Spec Fidelity

No Verus specs exist. All functions have `spec_strength = none`.

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

| # | Test File | Tests | Operations Covered |
|---|---|:---:|---|
| 1 | `TestTableStEph.rs` | 18 | All 15 ADT ops + ephemeral semantics, macro, large ops |
| 2 | `TestTableStPer.rs` | 18 | All 15 ADT ops + persistence, macro, empty ops, combine |
| 3 | `TestTableMtEph.rs` | 19 | All 15 ADT ops + ephemeral semantics, macro, parallel ops, parallel tabulate |
| 4 | `TestExample42_1.rs` | 2 | Smoke tests: `example_42_1()` and `performance_comparison()` |
| | **Total RTT** | **57** | |

### Test Gaps

| # | Gap | Severity |
|---|---|---|
| 1 | No edge-case tests for empty intersection/union/difference | Low |
| 2 | No tests verifying sorted invariant after operations | Low |
| 3 | No tests for `entries`/`collect` function | Low |
| 4 | No tests matching APAS Example 42.1 exact values | Low |

## Phase 6: PTT Review

No PTTs exist. No Verus code to test.

## Phase 7: Gap Analysis

### Prose Items Not Implemented

| # | Prose Item | Status | Notes |
|---|---|---|---|
| 1 | Algorithm 42.3 (collect) | **Not implemented** | APAS collect takes `Seq<(K,V)>` → `Table<K, Seq<V>>`. The implementations provide `entries(&self) → Seq<Pair<K,V>>` which returns flat entries — a different operation. |

### Code with No Prose Counterpart

| # | Item | Kind | Notes |
|---|---|---|---|
| 1 | `entries(&self) → Seq<Pair<K,V>>` | Utility | Returns flat entries; not APAS collect |
| 2 | `from_sorted_entries(Vec<Pair<K,V>>)` | Constructor | Helper for macros |
| 3 | `TableStEphLit!` / `TableStPerLit!` / `TableMtEphLit!` | Macros | Convenience literal syntax |
| 4 | `Example42_1` module | Example | Demonstrates API, doesn't match APAS Example 42.1 values |

## Phase 8: TOC Review

| # | File | Has TOC | Has Module Header | Copyright |
|---|---|:---:|:---:|:---:|
| 1 | `TableStEph.rs` | No | Yes | Yes (`//!`) |
| 2 | `TableStPer.rs` | No | Yes | Yes (`//!`) |
| 3 | `TableMtEph.rs` | No | Yes | Yes (`//!`) |
| 4 | `Example42_1.rs` | No | Yes | Yes (`//!`) |

### In/Out Table

| # | File | Clone | PartialEq/Eq | Debug | Display | Macro |
|---|---|:---:|:---:|:---:|:---:|:---:|
| 1 | `TableStEph.rs` | ❌ out (derive) | ❌ out (derive) | ❌ out (derive) | - | ✅ out |
| 2 | `TableStPer.rs` | ❌ out (derive) | ❌ out (derive) | ❌ out (derive) | - | ✅ out |
| 3 | `TableMtEph.rs` | ❌ out (derive) | ❌ out (derive) | ✅ out (manual) | - | ✅ out |
| 4 | `Example42_1.rs` | - | - | - | - | - |

No verus! blocks exist, so the "should be inside" classification is aspirational.

## Proof Holes Summary

```
Modules:   4 clean (no holes), 0 holed
Holes Found: 0 total
```

Zero proof holes — vacuously clean since no Verus code exists.

## Spec Strength Summary

| Classification | Count |
|---|---|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 54 |

All functions lack Verus specifications.

## Overall Assessment

**Chapter 42 implements all 15 operations from Data Type 42.1 across 3 variants (StEph, StPer, MtEph) with 57 runtime tests. No Verus verification exists. All files are proof-hole clean (trivially). Algorithm 42.3 (collect) is not implemented.**

### Strengths

1. All 15 ADT operations semantically faithful to the prose across all 3 variants.
2. Ephemeral vs persistent semantics correctly distinguished (`&mut self` vs returns `Self`).
3. Good test coverage: 57 RTTs covering all operations, semantics, macros, and parallelism.
4. Binary search for `find` achieves the prose's O(lg n) bound.

### Weaknesses

1. **No Verus verification** — zero functions inside `verus!`, zero specs.
2. **Sorted-array backing** gives O(n) insert/delete instead of APAS-required O(lg n). Only 4/15 operations match APAS costs.
3. **Algorithm 42.3 (collect) not implemented.** The `entries` function returns flat pairs, not grouped-by-key tables.
4. **MtEph intersection/union/difference are sequential** despite being in the multi-threaded file.
5. **MtEph parallelism is shallow** — single 2-way splits, not recursive divide-and-conquer for O(lg n) span.
6. **Cost annotations in trait declarations are inaccurate** — several claim better costs than implementations achieve.
7. **No TOC headers** in any source file.
8. **Example42_1.rs** uses integer keys with string values instead of matching the prose's `{'a'→4, 'b'→11, 'c'→2}`.
