<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 40 — Augmenting Binary Search Trees: Review Against Prose

**Date:** 2026-02-28
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap40.txt`
**Source files:** `src/Chap40/BSTKeyValueStEph.rs`, `src/Chap40/BSTReducedStEph.rs`, `src/Chap40/BSTSizeStEph.rs`
**Test files:** `tests/Chap40/TestBSTKeyValueStEph.rs` (6 tests), `tests/Chap40/TestBSTReducedStEph.rs` (19 tests), `tests/Chap40/TestBSTSizeStEph.rs` (8 tests)
**PTT files:** None
**Verification status:** 2898 verified, 0 errors. All code inside `verus!`.

## Phase 1: Inventory

| # | Chap | File | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|------|------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | 40 | BSTKeyValueStEph.rs | 13 | 14 | 0 | 15 | 29 | 0 | 8 | 0 | 21 |
| 2 | 40 | BSTReducedStEph.rs | 18 | 19 | 0 | 20 | 39 | 0 | 16 | 1 | 22 |
| 3 | 40 | BSTSizeStEph.rs | 14 | 15 | 0 | 21 | 36 | 0 | 20 | 0 | 16 |

**Total:** 104 functions, 104 inside `verus!`, 0 outside. 44 with specs (unknown strength), 1 hole (external_body accept), 59 no spec.

## Phase 2: Prose Inventory

### Section 1: Augmenting with Values (Key-Value)

| # | Prose Item | Reference | Type | Description |
|---|-----------|-----------|------|-------------|
| 1 | Key-value pair augmentation | §1 | Concept | Extend BST to store (key, value) pairs |
| 2 | Table/dictionary ADT via BST | §1 | Concept | BST implements the table/dictionary abstraction |

### Section 2: Augmenting with Size

| # | Prose Item | Reference | Type | Description |
|---|-----------|-----------|------|-------------|
| 3 | Size field augmentation | §2 | Concept | Store subtree count in each node |
| 4 | Algorithm 40.1 — rank(T, k) | §2 | Fn | Number of keys ≤ k |
| 5 | Algorithm 40.1 — select(T, i) | §2 | Fn | The i-th smallest key |
| 6 | Exercise 40.1 — splitRank(T, i) | §2 | Exercise | Split tree into (first i keys, rest) |

### Section 3: Augmenting with Reduced Values

| # | Prose Item | Reference | Type | Description |
|---|-----------|-----------|------|-------------|
| 7 | Monoid framework (identity, combine, lift) | §3 | Concept | General framework for augmented aggregation |
| 8 | Sum reduction example | §3 | Example | Sum of values in a range |
| 9 | Count reduction example | §3 | Example | Count of values in a range |
| 10 | Max reduction example | §3 | Example | Maximum value |
| 11 | Range queries with reduced values | §3 | Concept | Efficient range aggregation via augmentation |

### Implementation Status

| # | Prose Item | KeyValue | Reduced | Size | Notes |
|---|-----------|:--------:|:-------:|:----:|-------|
| 1 | Key-value augmentation | Yes | Yes | - | KeyValue: (K,V); Reduced: (K,V,R) |
| 2 | Table/dictionary ADT | Yes | Yes | - | find, get, contains, keys, values |
| 3 | Size field augmentation | - | Yes | Yes | Both store `size` in node |
| 4 | rank(T, k) | - | - | Yes | `rank_link` recursive |
| 5 | select(T, i) | - | - | Yes | `select_link` recursive |
| 6 | splitRank(T, i) | - | - | Yes | Exercise 40.1 |
| 7 | Monoid framework | - | Yes | - | `ReduceOp` trait |
| 8 | Sum reduction | - | Yes | - | `SumOp<T>` |
| 9 | Count reduction | - | Yes | - | `CountOp<T>` |
| 10 | Max reduction | - | No | - | Removed (was dead code) |
| 11 | Range queries | - | Yes | - | `range_reduce_link` |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All three modules implement Treap-based BSTs. Expected costs assume balanced trees (O(lg n) height).

| # | Operation | APAS Work | APAS Span | Code Work | Code Span | Match? | Module | Notes |
|---|----------|-----------|-----------|-----------|-----------|:------:|--------|-------|
| 1 | new | O(1) | O(1) | Θ(1) | Θ(1) | Yes | All | |
| 2 | size | O(1) | O(1) | Θ(1) | Θ(1) | Yes | All | |
| 3 | is_empty | O(1) | O(1) | Θ(1) | Θ(1) | Yes | All | |
| 4 | insert | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | All | With rotations |
| 5 | delete | O(lg n) | O(lg n) | Θ(n) | Θ(n) | No | All | Collect-filter-rebuild |
| 6 | find | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | All | |
| 7 | rank | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | Size | Alg 40.1 |
| 8 | select | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | Size | Alg 40.1 |
| 9 | split_rank | O(lg n) | O(lg n) | Θ(n) | Θ(n) | No | Size | Collects then splits |
| 10 | reduced_value | O(1) | O(1) | Θ(1) | Θ(1) | Yes | Reduced | |
| 11 | range_reduce | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | Reduced | |
| 12 | height | O(n) | O(n) | Θ(n) | Θ(n) | N/A | All | Not in prose |
| 13 | minimum/maximum | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | All | |

### 3b. Implementation Fidelity

| # | Algorithm | File | Fidelity | Detail |
|---|----------|------|:--------:|--------|
| 1 | rank | BSTSizeStEph.rs | Faithful | Matches Alg 40.1 |
| 2 | select | BSTSizeStEph.rs | Faithful | Matches Alg 40.1 |
| 3 | splitRank | BSTSizeStEph.rs | Divergent | Uses O(n) collect-split instead of O(lg n) treap split |
| 4 | Treap insert | All | Faithful | BST insert + priority rotations |
| 5 | delete | All | Divergent | O(n) collect-filter-rebuild vs O(lg n) treap delete |
| 6 | ReduceOp | BSTReducedStEph.rs | Faithful | identity/combine/lift match prose monoid |
| 7 | makeNode | BSTReducedStEph.rs | Faithful | Recomputes size + reduced value |
| 8 | range_reduce | BSTReducedStEph.rs | Faithful | Walks tree, combines in-range values |

### 3c. Spec Strength — Trait Methods

**BSTKeyValueStEphTrait** (13 methods):

| # | Method | Spec | Strength |
|---|--------|------|:--------:|
| 1 | new | ensures size==0 | strong |
| 2 | size | ensures == spec_size | strong |
| 3 | is_empty | ensures == (spec_size==0) | strong |
| 4 | height | ensures == spec_height | strong |
| 5 | insert | requires size < MAX | weak |
| 6 | delete | none | none |
| 7 | find | none | none |
| 8 | contains | none | none |
| 9 | get | none | none |
| 10 | keys | none | none |
| 11 | values | none | none |
| 12 | minimum_key | none | none |
| 13 | maximum_key | none | none |

**BSTReducedStEphTrait** (16 methods):

| # | Method | Spec | Strength |
|---|--------|------|:--------:|
| 1 | new | ensures size==0, wf | strong |
| 2 | size | ensures == spec_size | strong |
| 3 | is_empty | ensures == (spec_size==0) | strong |
| 4 | height | ensures == spec_height | strong |
| 5 | insert | ensures wf, size bounds | partial |
| 6 | delete | ensures wf | partial |
| 7 | find | none | none |
| 8 | contains | none | none |
| 9 | get | none | none |
| 10 | keys | none | none |
| 11 | values | none | none |
| 12 | minimum_key | none | none |
| 13 | maximum_key | none | none |
| 14 | reduced_value | none | none |
| 15 | range_reduce | none | none |
| 16 | combine (×2) | external_body | hole |

**BSTSizeStEphTrait** (15 methods):

| # | Method | Spec | Strength |
|---|--------|------|:--------:|
| 1 | new | ensures size==0, wf | strong |
| 2 | size | ensures == spec_size | strong |
| 3 | is_empty | ensures == (spec_size==0) | strong |
| 4 | height | ensures == spec_height | strong |
| 5 | insert | ensures wf, size bounds | partial |
| 6 | delete | ensures wf | partial |
| 7 | find | none | none |
| 8 | contains | none | none |
| 9 | minimum | none | none |
| 10 | maximum | none | none |
| 11 | in_order | none | none |
| 12 | rank | ensures ≤ spec_size | partial |
| 13 | select | none | none |
| 14 | split_rank | none | none |
| 15 | NodeTrait::new | none | none |

**Module-level helpers with specs** (BSTReducedStEph, BSTSizeStEph):

Both files have well-specified internal helpers: `rotate_left/right` (preserves size, wf), `insert_link` (preserves wf, size bounds), `make_node` (computes size, establishes wf), `build_treap_from_vec` (ensures size==range, wf), `height_link` (ensures == spec_height_link), `size_link` (ensures == spec_size_link). These specs are strong for their stated properties.

BSTKeyValueStEph has no size well-formedness tracking, so its helpers lack structural specs.

### Spec Strength Summary

| Classification | KV | Reduced | Size | Total |
|----------------|:--:|:-------:|:----:|------:|
| strong | 4 | 4 | 4 | 12 |
| partial | 0 | 2 | 3 | 5 |
| weak | 1 | 0 | 0 | 1 |
| hole | 0 | 1 | 0 | 1 |
| none | 8 | 9 | 8 | 25 |

Counts are trait methods only. Module-level helpers add ~15 more with specs in Reduced and Size.

## Phase 4: Parallelism Review

**The prose does NOT describe multithreaded augmented BSTs.** Chapter 40 is about augmentation patterns (values, size, reduced values), not parallelism. All three implementations are sequential ephemeral (StEph). No Mt variants exist or are needed.

## Phase 5: Runtime Test Review

### TestBSTKeyValueStEph.rs (6 tests)

| # | Test | Operations Covered |
|---|------|--------------------|
| 1 | key_value_bst_basic_operations | new, insert, find, get, size, is_empty, min, max |
| 2 | key_value_bst_update_existing_key | Duplicate key overwrites value |
| 3 | key_value_bst_collections | keys(), values() in sorted order |
| 4 | key_value_bst_macro_literal | BSTKeyValueStEphLit! macro |
| 5 | key_value_bst_height_stays_reasonable | 100 inserts, height ≤ 20 |
| 6 | key_value_bst_string_keys | String keys |

### TestBSTReducedStEph.rs (19 tests)

| # | Test | Operations Covered |
|---|------|--------------------|
| 1 | reduced_bst_sum_operations | SumOp insert, reduced_value, range_reduce |
| 2 | reduced_bst_count_operations | CountOp insert, reduced_value, range_reduce |
| 3 | reduced_bst_update_existing_key | Duplicate key updates reduced_value |
| 4 | reduced_bst_collections | keys(), values() |
| 5 | reduced_bst_macro_literal | BSTReducedStEphLit! macro |
| 6 | reduced_bst_height_stays_reasonable | 100 inserts, height check |
| 7 | reduced_bst_range_queries | Various range boundaries |
| 8 | reduced_bst_string_keys | String keys with SumOp |
| 9 | test_contains_method | contains |
| 10 | test_get_method | get |
| 11 | test_minimum_maximum_keys | min/max on growing tree |
| 12 | test_count_reducer_all_operations | CountOp full exercise |
| 13 | test_empty_tree_operations | All ops on empty tree |
| 14 | test_single_element_operations | All ops on single-element tree |
| 15 | test_large_tree_with_all_operations | 50 inserts, full exercise |
| 16 | test_duplicate_keys_overwrite | Triple overwrite |
| 17 | test_edge_case_ranges | Out-of-range, single-point |
| 18 | test_sumop_trait | SumOp trait methods directly |
| 19 | test_countop_trait | CountOp trait methods directly |

### TestBSTSizeStEph.rs (8 tests)

| # | Test | Operations Covered |
|---|------|--------------------|
| 1 | size_bst_basic_operations | new, insert, find, size, is_empty, min, max |
| 2 | size_bst_rank_operations | rank for all keys + boundaries |
| 3 | size_bst_select_operations | select for all ranks + out-of-range |
| 4 | size_bst_rank_select_consistency | rank(select(i))==i roundtrip |
| 5 | size_bst_split_rank_operations | split_rank at mid + edges |
| 6 | size_bst_macro_literal | BSTSizeStEphLit! macro |
| 7 | size_bst_duplicate_insert_is_idempotent | Same key twice |
| 8 | size_bst_large_dataset_performance | 1000 inserts |

**Coverage:** Excellent. 33 tests across 3 files. All prose algorithms exercised. Missing: delete test for BSTReducedStEph.

## Phase 6: PTT Review

No PTTs. No iterators or complex loop verification. PTTs not needed.

## Phase 7: Gap Analysis

### Prose Items Not Implemented

| # | Priority | Gap | Detail |
|---|----------|-----|--------|
| 1 | Low | MaxOp not implemented | Prose mentions max as reduction example. Removed as dead code. |
| 2 | Medium | delete is O(n) | All files use collect-filter-rebuild instead of O(lg n) treap delete. |
| 3 | Medium | split_rank is O(n) | Uses collect-then-split instead of O(lg n) treap split. |

### Code With No Prose Counterpart

| # | Item | File | Purpose |
|---|------|------|---------|
| 1 | height | All | Balance inspection |
| 2 | contains, get | KV, Reduced | Wrappers around find |
| 3 | keys, values | KV, Reduced | Collection accessors |
| 4 | minimum_key/maximum_key | KV, Reduced | Standard BST ops |
| 5 | in_order | Size | Traversal utility |
| 6 | Macros (BSTXxxLit!) | All | Test convenience |

## Phase 8: TOC Review

| # | Chap | File | TOC? | Sections Present |
|---|------|------|:----:|-----------------|
| 1 | 40 | BSTKeyValueStEph.rs | Yes | 4, 6, 8, 9, 11, 12, 13 |
| 2 | 40 | BSTReducedStEph.rs | Yes | 4, 6, 7, 8, 9, 11, 12, 13 |
| 3 | 40 | BSTSizeStEph.rs | Yes | 2, 4, 6, 7, 8, 9, 11, 12, 13 |

### In/Out Table

| # | Chap | File | Clone | Default | Debug | Macro |
|---|------|------|:-----:|:-------:|:-----:|:-----:|
| 1 | 40 | BSTKeyValueStEph.rs | ✅ in | ✅ in | ✅ out | ✅ out |
| 2 | 40 | BSTReducedStEph.rs | ✅ in | ✅ in | ✅ out | ✅ out |
| 3 | 40 | BSTSizeStEph.rs | ✅ in | ✅ in | ✅ out | ✅ out |

## Proof Holes Summary

```
✓ BSTKeyValueStEph.rs
ℹ BSTReducedStEph.rs
  Info: 2 external_body accept holes (SumOp::combine, CountOp::combine)
  1 clean proof function (lemma_wf_assemble)
✓ BSTSizeStEph.rs
  3 clean proof functions

Modules: 3 clean, 0 holed
Proof functions: 4 clean, 0 holed
Holes: 0
```

The 2 `external_body` accept holes are on `SumOp::combine` and `CountOp::combine`. These wrap Rust arithmetic (`a + b`) which Verus cannot verify for generic `ArithmeticT` / `usize` addition without overflow proofs. Accepted as documented holes.

## Overall Assessment

Chapter 40 faithfully implements all three augmentation patterns from the prose: key-value pairs (§1), size with rank/select (§2), and reduced values with range queries (§3). All code is inside `verus!` and verifies cleanly.

**Strengths:**
- All prose algorithms implemented (rank, select, splitRank, ReduceOp framework, range_reduce)
- Strong structural specs on new/size/is_empty/height across all three files
- BSTReducedStEph and BSTSizeStEph maintain size well-formedness invariant through insert/rotate/build
- 4 clean proof lemmas, 0 proof holes
- 33 runtime tests with excellent coverage
- No multithreading needed — prose is about augmentation patterns, not parallelism

**Weaknesses:**
- BSTKeyValueStEph has no size well-formedness tracking (no `spec_wf`)
- No abstract content model (Map/Set view), so find/insert/delete lack semantic specs
- delete and split_rank are O(n) (collect-filter-rebuild) instead of O(lg n)
- MaxOp not implemented (prose mentions it as an example)
- 25 trait methods have no spec at all (none classification)
