<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 40 — Augmenting Binary Search Trees: Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap40.txt`
**Source files:** `src/Chap40/BSTSizeStEph.rs`, `src/Chap40/BSTKeyValueStEph.rs`, `src/Chap40/BSTReducedStEph.rs`
**Test files:** `tests/Chap40/TestBSTSizeStEph.rs` (8 tests), `tests/Chap40/TestBSTKeyValueStEph.rs` (6 tests), `tests/Chap40/TestBSTReducedStEph.rs` (19 tests)
**PTT files:** None
**Verification status:** No `verus!` blocks — entire chapter is plain Rust

## Phase 1: Inventory

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap40 | BSTKeyValueStEph | 13 | 15 | 0 | 11 | 0 | 26 | 0 | 0 | 26 |
| 2 | Chap40 | BSTReducedStEph | 18 | 20 | 16 | 0 | 0 | 35 | 0 | 0 | 35 |
| 3 | Chap40 | BSTSizeStEph | 14 | 16 | 15 | 0 | 0 | 30 | 0 | 0 | 30 |

**Total:** 91 functions, 0 inside `verus!`, 91 outside, 0 with specs, 0 proof holes.

**Note:** BSTReducedStEph (16 IBI) and BSTSizeStEph (15 IBI) have bare `impl` blocks for internal helpers on `Node` and on the main struct. These are flagged by veracity as `bare_impl` style issues.

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
| 4 | Algorithm 40.1 — rank(T, k) | §2 | Fn | Number of keys less than k |
| 5 | Algorithm 40.1 — select(T, i) | §2 | Fn | The i-th smallest key |
| 6 | Exercise 40.1 — splitRank(T, i) | §2 | Exercise | Split tree into (first i keys, rest) |

### Section 3: Augmenting with Reduced Values

| # | Prose Item | Reference | Type | Description |
|---|-----------|-----------|------|-------------|
| 7 | Monoid-based reduction (identity, combine, lift) | §3 | Concept | General framework for augmented aggregation |
| 8 | Sum reduction example | §3 | Example | Sum of values in a range |
| 9 | Count reduction example | §3 | Example | Count of values in a range |
| 10 | Max reduction | §3 | Example | Maximum value |
| 11 | Range queries with reduced values | §3 | Concept | Efficient range aggregation via augmentation |

### Cost Specifications

The prose references Cost Specification 38.11 from Chapter 38. No new cost specification is defined in Chapter 40 — augmented operations (rank, select, range_reduce) are all O(lg |t|) by the same argument as the base BST operations.

### Implementation Status

| # | Prose Item | BSTSizeStEph | BSTKeyValueStEph | BSTReducedStEph | Notes |
|---|-----------|:------------:|:----------------:|:---------------:|-------|
| 1 | Key-value augmentation | - | Yes | Yes | KeyValue stores (K, V); Reduced stores (K, V, R) |
| 2 | Table/dictionary ADT | - | Yes | Yes | find, get, contains, keys, values |
| 3 | Size field augmentation | Yes | - | Yes | Both store `size` in node |
| 4 | rank(T, k) | Yes | - | - | `rank_link` recursive implementation |
| 5 | select(T, i) | Yes | - | - | `select_link` recursive implementation |
| 6 | splitRank(T, i) | Yes | - | - | Exercise 40.1 implemented |
| 7 | Monoid framework | - | - | Yes | `ReduceOp` trait with identity/combine/lift |
| 8 | Sum reduction | - | - | Yes | `SumOp<T>` |
| 9 | Count reduction | - | - | Yes | `CountOp<T>` |
| 10 | Max reduction | - | - | Partial | `MaxOp<T>` struct declared but trait impl not provided |
| 11 | Range queries | - | - | Yes | `range_reduce` and `range_reduce_link` |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All three modules implement Treap-based BSTs (using random priorities for balance). Expected costs assume balanced trees (O(lg n) height).

| # | Operation | APAS Work | APAS Span | Code Work | Code Span | Match? | Module | Notes |
|---|----------|-----------|-----------|-----------|-----------|:------:|--------|-------|
| 1 | new | O(1) | O(1) | Θ(1) | Θ(1) | Yes | All | |
| 2 | size | O(1) | O(1) | Θ(1) | Θ(1) | Yes | All | Stored in node |
| 3 | is_empty | O(1) | O(1) | Θ(1) | Θ(1) | Yes | All | |
| 4 | insert | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | All | Treap insert with rotations |
| 5 | delete | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | Size, KV | Not in Reduced |
| 6 | find | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | All | |
| 7 | rank | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | Size | Uses left subtree size |
| 8 | select | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | Size | Uses left subtree size |
| 9 | split_rank | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | Size | Exercise 40.1 |
| 10 | reduced_value | O(1) | O(1) | Θ(1) | Θ(1) | Yes | Reduced | Stored in node |
| 11 | range_reduce | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | Reduced | Two tree walks |
| 12 | height | O(n) | O(n) | Θ(n) | Θ(n) | N/A | All | Utility, not in prose |
| 13 | in_order | O(n) | O(n) | Θ(n) | Θ(n) | N/A | Size | Utility, not in prose |
| 14 | minimum/maximum | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes | All | |
| 15 | keys/values | O(n) | O(n) | Θ(n) | Θ(n) | N/A | KV, Reduced | Traversal |

### 3b. Implementation Fidelity

| # | Algorithm | Module | Fidelity | Detail |
|---|----------|--------|:--------:|--------|
| 1 | rank | BSTSizeStEph | Faithful | Matches Algorithm 40.1: if k ≤ node.key, recurse left; else 1 + left_size + recurse right |
| 2 | select | BSTSizeStEph | Faithful | Matches Algorithm 40.1: compare i with left_size to decide direction |
| 3 | splitRank | BSTSizeStEph | Faithful | Exercise 40.1: uses select to find split key, then splits |
| 4 | Treap insert (all) | All | Faithful | Standard BST insert followed by rotations to restore heap property on priority |
| 5 | Treap rotations | All | Faithful | Left/right rotations maintain BST invariant and update sizes |
| 6 | Key-value insert | BSTKeyValueStEph | Faithful | Same as BST insert; update value on duplicate key |
| 7 | ReduceOp trait | BSTReducedStEph | Faithful | identity/combine/lift match the prose monoid framework |
| 8 | range_reduce | BSTReducedStEph | Faithful | Walks tree, combines reduced values from subtrees within range bounds |
| 9 | update_node/make_node | BSTReducedStEph | Faithful | Recomputes reduced_value = combine(combine(left_rv, lift(value)), right_rv) |
| 10 | delete | BSTSizeStEph | Not in Reduced | BSTReducedStEph has no delete — would need `update_node` after restructuring |

### 3c. Spec Fidelity

Not applicable — no `verus!` blocks, no `requires`/`ensures`.

## Phase 4: Parallelism Review

No Mt (multi-threaded) modules exist in Chapter 40. All three implementations are single-threaded ephemeral (StEph). No parallelism review needed.

## Phase 5: Runtime Test Review

### TestBSTSizeStEph.rs (8 tests)

| # | Test | Operations Covered | Edge Cases? |
|---|------|--------------------|:-----------:|
| 1 | `size_bst_basic_operations` | new, insert, find, size, is_empty | 7 elements |
| 2 | `size_bst_rank_operations` | rank for existing and non-existing keys | Boundary keys |
| 3 | `size_bst_select_operations` | select for all valid indices | Out-of-range returns None |
| 4 | `size_bst_rank_select_consistency` | rank(select(i)) == i roundtrip | Full range 0..n |
| 5 | `size_bst_split_rank_operations` | split_rank, sizes sum to total | Mid-split |
| 6 | `size_bst_macro_literal` | BSTSizeStEphLit! macro | Macro interface |
| 7 | `size_bst_duplicate_insert_is_idempotent` | Insert same key twice | Idempotency |
| 8 | `size_bst_large_dataset_performance` | 1000 inserts, rank, select | Performance |

### TestBSTKeyValueStEph.rs (6 tests)

| # | Test | Operations Covered | Edge Cases? |
|---|------|--------------------|:-----------:|
| 1 | `key_value_bst_basic_operations` | insert, find, get, size | 3 elements |
| 2 | `key_value_bst_update_existing_key` | Insert duplicate key updates value | Duplicate key |
| 3 | `key_value_bst_collections` | keys(), values() | Sorted order |
| 4 | `key_value_bst_macro_literal` | BSTKeyValueStEphLit! macro | Macro interface |
| 5 | `key_value_bst_height_stays_reasonable` | height after 100 inserts | Balance check |
| 6 | `key_value_bst_string_keys` | String keys and values | Non-integer keys |

### TestBSTReducedStEph.rs (19 tests)

| # | Test | Operations Covered | Edge Cases? |
|---|------|--------------------|:-----------:|
| 1 | `reduced_bst_sum_operations` | Insert, reduced_value with SumOp | |
| 2 | `reduced_bst_count_operations` | Insert, reduced_value with CountOp | |
| 3 | `reduced_bst_update_existing_key` | Duplicate key updates value | |
| 4 | `reduced_bst_collections` | keys(), values() | |
| 5 | `reduced_bst_macro_literal` | BSTReducedStEphLit! macro | |
| 6 | `reduced_bst_height_stays_reasonable` | Height after 100 inserts | Balance |
| 7 | `reduced_bst_range_queries` | range_reduce with SumOp | Various ranges |
| 8 | `reduced_bst_string_keys` | String keys with SumOp | |
| 9 | `reduced_bst_contains_and_get` | contains, get | Missing keys |
| 10 | `reduced_bst_min_max_keys` | minimum_key, maximum_key | Empty tree |
| 11 | `reduced_bst_empty_operations` | Operations on empty tree | All return None/0 |
| 12 | `reduced_bst_single_element` | Single-element tree | Boundary |
| 13 | `reduced_bst_large_tree` | 500 inserts | Scale |
| 14 | `reduced_bst_duplicate_keys` | Same key with different values | Latest wins |
| 15 | `reduced_bst_edge_case_ranges` | Out-of-range, single-point | Range boundaries |
| 16–17 | `test_sumop_trait`, `test_countop_trait` | ReduceOp trait methods directly | Unit |
| 18 | `reduced_bst_range_reduce_with_count` | range_reduce with CountOp | Count validation |
| 19 | `reduced_bst_empty_range` | Range reduce on empty tree | Empty |

### Coverage Assessment

- **BSTSizeStEph:** Excellent. All prose algorithms tested including rank-select consistency roundtrip and Exercise 40.1 (split_rank).
- **BSTKeyValueStEph:** Good. Covers basic CRUD, duplicate key handling, and non-integer key types.
- **BSTReducedStEph:** Excellent. 19 tests cover both reduction operations, range queries, edge cases, and direct trait unit tests.
- **Missing:** No test for delete on BSTReducedStEph (not implemented). No test for MaxOp (incomplete implementation).

## Phase 6: PTT Review

No PTTs exist. No `verus!` blocks, so no proof-time verification to test.

## Phase 7: Gap Analysis

### Prose Items Not Implemented

| # | Priority | Gap | Detail | Recommendation |
|---|----------|-----|--------|----------------|
| 1 | Medium | No delete in BSTReducedStEph | Size and KeyValue have delete; Reduced does not. Deletion requires recomputing reduced_value up the path, which update_node already handles. | Implement delete_link using same pattern as insert_link |
| 2 | Medium | MaxOp incomplete | Struct declared but ReduceOp trait not implemented. Prose mentions max as a reduction example. | Implement ReduceOp for MaxOp |
| 3 | Medium | No Mt variants | Prose discusses parallel BST operations (Ch38). Augmented variants could benefit from parallelism for range_reduce on subtrees. | Consider MtEph variants if needed by later chapters |
| 4 | High | No `verus!` blocks | No formal specifications or proofs for any module. | Future verusification |
| 5 | Low | Bare impl style issues | BSTReducedStEph has 16 functions in bare `impl Node`/`impl BSTReducedStEph`. BSTSizeStEph has 15. Veracity flags these as `bare_impl` errors. | Move internal helpers to trait or document exemption |

### Code With No Prose Counterpart

| # | Item | Module | Purpose |
|---|------|--------|---------|
| 1 | `height` / `height_rec` | All | Utility to inspect tree balance |
| 2 | `contains` | KV, Reduced | Boolean wrapper around `find` |
| 3 | `get` | KV, Reduced | Returns value only (vs find returning key) |
| 4 | `keys` / `values` | KV, Reduced | Collection accessors |
| 5 | `minimum_key` / `maximum_key` | KV, Reduced | Standard BST operations |
| 6 | `in_order` | Size | Traversal utility |
| 7 | `build_treap_from_sorted` | All | Efficient batch construction |
| 8 | `BSTSizeStEphLit!` etc. | All | Convenience macros |
| 9 | `in_order_collect_with_priority` | Size | Debug utility |

## Phase 8: TOC Review

No files contain `verus!` blocks. The 13-section TOC standard does not apply.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:---:|:------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | BSTSizeStEph.rs | ✅ out (derive) | ✅ out (derive) | ✅ out (impl) | - | - | ✅ out (derive) | ✅ out (impl) | ✅ out (BSTSizeStEphLit!) | No `verus!` block |
| 2 | BSTKeyValueStEph.rs | ✅ out (derive) | ✅ out (derive) | ✅ out (impl) | - | - | ✅ out (derive) | ✅ out (impl) | ✅ out (BSTKeyValueStEphLit!) | No `verus!` block |
| 3 | BSTReducedStEph.rs | ✅ out (derive) | ✅ out (derive) | ✅ out (impl) | - | - | ✅ out (derive) | ✅ out (impl) | ✅ out (BSTReducedStEphLit!) | No `verus!` block |

## Proof Holes Summary

```
✓ BSTKeyValueStEph.rs
❌ BSTReducedStEph.rs  (2 bare_impl errors)
❌ BSTSizeStEph.rs     (2 bare_impl errors)

Modules:     1 clean, 2 with style errors
Proof Holes: 0
Style Errors: 4 bare_impl (helpers on Node and main struct outside trait)
```

The bare_impl errors are style issues — internal helper functions (rotations, link operations, build_treap_from_sorted) are implemented in bare `impl Node` and `impl BSTSizeStEph`/`BSTReducedStEph` blocks rather than in the trait. Per the `trait-impl-pattern` rule, these should be in the trait, though internal helpers on `Node` are a reasonable exception.

## Spec Strength Summary

| Classification | Count |
|---------------|------:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 91 |

## Overall Assessment

Chapter 40 faithfully implements the three forms of BST augmentation described in the prose: size (with rank/select), key-value pairs, and general reduced values (with range queries). All implementations use Treaps for balancing and follow the prose algorithms closely.

**Strengths:**
- All three augmentation patterns implemented with faithful algorithms
- rank, select, and split_rank (Exercise 40.1) all present and tested
- ReduceOp trait cleanly captures the monoid framework (identity/combine/lift)
- Excellent test coverage: 33 tests across three modules, including edge cases and large datasets
- range_reduce correctly handles arbitrary ranges using the augmented reduced values

**Weaknesses:**
- No formal verification — entirely plain Rust with zero `verus!` code
- BSTReducedStEph missing `delete` operation
- MaxOp declared but not implemented
- 4 bare_impl style warnings from veracity
- No Mt variants (Ch40 is entirely single-threaded)
