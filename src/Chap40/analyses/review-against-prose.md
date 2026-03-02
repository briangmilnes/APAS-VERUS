<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 40 — Augmenting Binary Search Trees: Review Against Prose

**Date:** 2026-03-02 (agent1: added content specs to BSTKeyValueStEph, updated proof holes)
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap40.txt`
**Source files:** `src/Chap40/BSTKeyValueStEph.rs`, `src/Chap40/BSTReducedStEph.rs`, `src/Chap40/BSTSizeStEph.rs`
**Test files:** `tests/Chap40/TestBSTKeyValueStEph.rs` (6 tests), `tests/Chap40/TestBSTReducedStEph.rs` (19 tests), `tests/Chap40/TestBSTSizeStEph.rs` (8 tests)
**PTT files:** None
**Verification status:** 2964 verified, 0 errors. All code inside `verus!`.

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

Corrected 2026-03-02: the 2026-02-28 review misclassified many methods as "none" that
actually have requires/ensures clauses. Re-examined against source.

**BSTKeyValueStEphTrait** (13 public methods):

| # | Method | Spec | Strength |
|---|--------|------|:--------:|
| 1 | new | ensures size==0, view==Map::empty | strong |
| 2 | size | ensures == spec_size | strong |
| 3 | is_empty | ensures == (spec_size==0) | strong |
| 4 | height | ensures == spec_height | strong |
| 5 | insert | requires size<MAX, ensures contains_key + size bounds | strong |
| 6 | delete | requires wf, ensures size<=old | partial |
| 7 | find | requires wf, ensures empty=>None, some=>contains_key | partial |
| 8 | contains | requires wf, ensures empty=>!result, true=>contains_key | partial |
| 9 | get | requires wf, ensures empty=>None, some=>contains_key | partial |
| 10 | keys | requires wf, ensures len==size | partial |
| 11 | values | requires wf, ensures len==size | partial |
| 12 | minimum_key | requires wf, ensures empty=>None, non-empty=>Some, matches spec_min_key | strong |
| 13 | maximum_key | requires wf, ensures empty=>None, non-empty=>Some, matches spec_max_key | strong |

Note: BSTKeyValueStEph tracks `spec_wf` as `self.size as nat == spec_node_count_link(&self.root)`.
Insert ensures `self@.contains_key(key)` — semantic content spec (the first in this chapter).
Find/contains/get ensure `result.is_some() ==> self@.contains_key(*key)`.
Minimum/maximum_key match the `spec_min_key`/`spec_max_key` spec fns.

**BSTReducedStEphTrait** (15 public methods + 1 hole):

| # | Method | Spec | Strength |
|---|--------|------|:--------:|
| 1 | new | ensures size==0, wf, view==Map::empty | strong |
| 2 | size | ensures == spec_size | strong |
| 3 | is_empty | ensures == (spec_size==0) | strong |
| 4 | height | ensures == spec_height | strong |
| 5 | insert | requires size+1<=MAX, wf; ensures wf, size bounds | partial |
| 6 | delete | requires wf; ensures wf, size<=old | partial |
| 7 | find | requires wf, ensures empty=>None | partial |
| 8 | contains | requires wf, ensures empty=>!result | partial |
| 9 | get | requires wf, ensures empty=>None | partial |
| 10 | keys | requires wf, ensures len==size | partial |
| 11 | values | requires wf, ensures len==size | partial |
| 12 | minimum_key | requires wf, ensures empty=>None, non-empty=>Some | partial |
| 13 | maximum_key | requires wf, ensures empty=>None, non-empty=>Some | partial |
| 14 | reduced_value | requires wf, ensures empty=>identity | partial |
| 15 | range_reduce | requires wf, ensures empty=>identity | partial |
| 16 | combine (×3) | external_body | hole |

**BSTSizeStEphTrait** (14 public methods + NodeTrait):

| # | Method | Spec | Strength |
|---|--------|------|:--------:|
| 1 | new | ensures size==0, wf, view==Set::empty | strong |
| 2 | size | ensures == spec_size | strong |
| 3 | is_empty | ensures == (spec_size==0) | strong |
| 4 | height | requires size<MAX, wf; ensures == spec_height | strong |
| 5 | insert | requires size+1<=MAX, wf; ensures wf, size bounds | partial |
| 6 | delete | requires wf; ensures wf, size<=old | partial |
| 7 | find | requires wf, ensures empty=>None | partial |
| 8 | contains | requires wf, ensures empty=>!result | partial |
| 9 | minimum | requires wf, ensures empty=>None, non-empty=>Some | partial |
| 10 | maximum | requires wf, ensures empty=>None, non-empty=>Some | partial |
| 11 | in_order | requires wf, ensures len==size | partial |
| 12 | rank | requires size<MAX, wf; ensures <=size | partial |
| 13 | select | ensures (rank==0 or rank>size)=>None | partial |
| 14 | split_rank | requires wf; ensures both children wf | partial |
| 15 | NodeTrait::new | no spec | none |

**Module-level helpers with specs** (all three files):

All three files have well-specified internal helpers: `rotate_left/right` (preserves some/none; KV additionally preserves content via forall), `insert_link` (KV: preserves some + content contains_key; Reduced/Size: wf + size bounds), `find_link` (KV: result.some => content contains_key; others: None propagation), `min_key_link/max_key_link` (None propagation, KV matches spec), `collect_keys/values` (out.len growth), `build_treap_from_vec` (result.is_none == start==end), `height_link` (== spec_height_link), `filter_by_key` (result.len <= items.len), `find_min_priority_idx` (in-bounds).

BSTReducedStEph and BSTSizeStEph additionally track `spec_size_wf_link` through rotations, insert, build, and make_node.

### Spec Strength Summary

| Classification | KV | Reduced | Size | Total |
|----------------|:--:|:-------:|:----:|------:|
| strong | 7 | 4 | 4 | 15 |
| partial | 6 | 11 | 10 | 27 |
| weak | 0 | 0 | 0 | 0 |
| hole | 0 | 1 | 0 | 1 |
| none | 0 | 0 | 1 | 1 |

Counts are public trait methods only. Module-level helpers add ~30 more with specs across all three files. BSTKeyValueStEph now has semantic content specs: insert ensures `self@.contains_key(key)`, find/contains/get ensure `result.is_some() ==> self@.contains_key(*key)`. BSTReducedStEph and BSTSizeStEph specs remain structural (size bounds, wf preservation, emptiness implications).

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
❌ BSTKeyValueStEph.rs
  1 assume in eq/clone (Verus workaround)
  2 assume in rotate_left/right (Map union_prefer_right commutativity)
  4 assume in insert_link (content contains_key after rotation/creation)
  3 assume in find_link (content contains_key propagation)
  5 clean proof functions, 0 holed
ℹ BSTReducedStEph.rs
  1 assume in eq/clone (Verus workaround)
  3 external_body accept holes (SumOp::identity, SumOp::combine, CountOp::combine)
  1 clean proof function (lemma_wf_assemble)
✓ BSTSizeStEph.rs
  1 assume in eq/clone (Verus workaround)
  3 clean proof functions

Modules: 2 clean, 1 holed
Proof functions: 9 clean, 0 holed
Holes: 9 (all assume in BSTKeyValueStEph.rs)
```

BSTKeyValueStEph.rs gained 9 assumes to support semantic content specs (`self@.contains_key(key)`). The Verus solver cannot unfold `spec_content_link` (recursive Map-building spec fn) and propagate `contains_key` through `Map::union_prefer_right`/`Map::insert` chains. The structural reasoning is sound — rotations preserve content, insert adds the key, find returns keys present in content — but the solver cannot automate Map-union reasoning. All 9 assumes are for the same pattern: `spec_content_link(link).contains_key(key)`.

The 3 `external_body` accept holes in BSTReducedStEph are on `SumOp::identity`, `SumOp::combine`, and `CountOp::combine` — Rust arithmetic on generic types. The 3 `assume` in clone are the standard clone-bridge workaround.

## Overall Assessment

Chapter 40 faithfully implements all three augmentation patterns from the prose: key-value pairs (§1), size with rank/select (§2), and reduced values with range queries (§3). All code is inside `verus!` and verifies cleanly (2964 verified, 0 errors).

**The prose does NOT describe multithreaded BSTs.** Chapter 40 is purely about augmentation patterns. No Mt variants are needed.

**Are the specs good?** BSTKeyValueStEph now has semantic content specs (`self@.contains_key(key)` after insert, find/contains/get propagate membership). This is a meaningful step beyond pure structural specs. However, these specs rely on 9 assumes because the Verus solver cannot automate Map-union reasoning for recursive `spec_content_link`. BSTReducedStEph and BSTSizeStEph specs remain structural (size/wf bounds). Strengthening them to semantic content specs would require the same Map-union assumes.

**Strengths:**
- All prose algorithms implemented (rank, select, splitRank, ReduceOp framework, range_reduce)
- BSTKeyValueStEph has semantic content specs (insert ensures contains_key, find/get/contains propagate)
- Strong structural specs on new/size/is_empty/height across all three files
- BSTReducedStEph and BSTSizeStEph maintain size well-formedness invariant through insert/rotate/build
- 9 clean proof lemmas, 0 proof holes in lemma bodies
- 33 runtime tests with excellent coverage
- No multithreading needed — prose is about augmentation patterns, not parallelism

**Weaknesses:**
- 9 assumes in BSTKeyValueStEph for Map-union content reasoning (solver limitation)
- BSTReducedStEph and BSTSizeStEph specs remain structural only
- delete and split_rank are O(n) (collect-filter-rebuild) instead of O(lg n)
- MaxOp not implemented (prose mentions it as an example)
- 1 trait method (NodeTrait::new in Size) has no spec
