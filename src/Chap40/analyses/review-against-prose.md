<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 40: Augmenting Binary Search Trees — Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6
**Source files:** `src/Chap40/BSTKeyValueStEph.rs`, `src/Chap40/BSTSizeStEph.rs`, `src/Chap40/BSTReducedStEph.rs`
**Test files:** `tests/Chap40/TestBSTKeyValueStEph.rs`, `tests/Chap40/TestBSTSizeStEph.rs`, `tests/Chap40/TestBSTReducedStEph.rs`
**Verification status:** No `verus!` blocks — entire chapter is plain Rust, gated with `#[cfg(not(verus_keep_ghost))]`

---

## Phase 2: Prose Inventory

### Definitions

| # | Prose Item | APAS Section | Description |
|---|-----------|:---:|-------------|
| 1 | Key-value augmented node | §1 | `TNode of (T × K × Z × T)` — key-value pair BST (dictionary/table) |
| 2 | Size-augmented node | §2 | `TNode of (T × K × Z × Z × T)` — 5-tuple `(L, k, p, n, R)` with size field `n` |
| 3 | Reduced-value augmented node | §3 | `TNode of (T × K × Z × Z × val × val × T)` — 7-tuple `(L, k, p, n, v, r, R)` with value `v` and reduced value `r` |
| 4 | Associative reduction function `f` | §3 | User-specified associative function with identity `I`, used for reduced values |

### Algorithms

| # | Prose Item | APAS Section | APAS Pseudocode | Description |
|---|-----------|:---:|:---:|-------------|
| 1 | `size T` | §2 | Yes | O(1) size read from augmented field — returns `n` for TNode, `0` for TLeaf |
| 2 | `makeNode (L, k, p, R)` | §2 | Yes | Creates node computing `size(L) + size(R) + 1` |
| 3 | `rank T k` | §2.1, Alg 40.1 | Yes | Number of keys ≤ k — three-way compare using `|L|` |
| 4 | `select T i` | §2.1, Alg 40.1 | Yes | Key with rank `i` (1-indexed) — three-way compare using `|L| + 1` |
| 5 | `splitRank (t, i)` | Exercise 40.1 | No (exercise) | Splits tree at rank boundary into two trees |
| 6 | `reducedVal T` | §3 | Yes | O(1) read of reduced value — returns `I` for TLeaf, `r` for TNode |
| 7 | `makeNode (L, k, v, p, R)` | §3 | Yes | Creates node computing `f(reducedVal(L), f(v, reducedVal(R)))` and `size(L) + 1 + size(R)` |

### Cost Specs from Prose

| # | Algorithm | APAS Work | APAS Span | Notes |
|---|-----------|-----------|-----------|-------|
| 1 | `size` | Θ(1) | Θ(1) | Reads augmented field directly |
| 2 | `makeNode` (both variants) | Θ(1) | Θ(1) | Constant-time construction |
| 3 | `rank` (without size augmentation) | Θ(n) | Θ(log n) | Must compute subtree sizes each time |
| 4 | `rank` (with size augmentation) | Θ(log n) | Θ(log n) | Uses stored size field |
| 5 | `select` (with size augmentation) | Θ(log n) | Θ(log n) | Uses stored size field |
| 6 | `reducedVal` | Θ(1) | Θ(1) | Reads augmented field directly |

### Exercises

| # | Exercise | Description | Status in Code |
|---|----------|-------------|----------------|
| 1 | Exercise 40.1: `splitRank (t, i)` | Split tree into t1 (rank < i) and t2 (rank ≥ i) | Implemented in `BSTSizeStEph::split_rank_link` |

---

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Module | Function | Annotated Cost | APAS Cost | Match | Notes |
|---|--------|----------|---------------|-----------|:-----:|-------|
| 1 | BSTSizeStEph | `size_link` | Θ(1) | Θ(1) | Yes | |
| 2 | BSTSizeStEph | `make_node` | Θ(1) | Θ(1) | Yes | |
| 3 | BSTSizeStEph | `rank_link` | Θ(log n) | Θ(log n) | Yes | With size augmentation |
| 4 | BSTSizeStEph | `select_link` | Θ(log n) | Θ(log n) | Yes | |
| 5 | BSTSizeStEph | `split_rank_link` | Θ(log n) | Θ(log n) | **No** | Clones entire subtrees via `Box<Node>` deep clone; actual work is O(n) worst case due to ownership model. APAS assumes persistent (shared) nodes giving O(log n). |
| 6 | BSTReducedStEph | `reduced_value_link` | Θ(1) | Θ(1) | Yes | |
| 7 | BSTReducedStEph | `update_node` / `make_node` | Θ(1) | Θ(1) | Yes | |
| 8 | BSTReducedStEph | `range_reduce_link` | Θ(log n) | N/A | **Wrong** | No APAS counterpart. Annotated O(log n) but actual cost is O(log n + k) where k = keys in range, because the implementation recursively visits every node in [low, high] instead of exploiting stored reduced values for subtrees fully within range. |

**Cost disagreements:** Two issues found:

1. **`split_rank_link` clone cost:** The APAS O(log n) bound assumes persistent (functional) node sharing. The Rust implementation uses `Box<Node>` which requires deep cloning of subtrees at each recursion level. Each `node.left.clone()` or `node.right.clone()` deep-copies the entire subtree. Worst case: O(n) total cloning work. This is a fundamental Rust ownership limitation — an `Rc`/`Arc`-based persistent tree would achieve the APAS bound.

2. **`range_reduce_link` not exploiting augmentation:** The function visits every node in [low, high] individually, giving O(log n + k) cost. A true O(log n) augmented range reduction would: (a) find the split node where the low/high paths diverge, (b) going left from split, combine the stored reduced value of right subtrees entirely within range (O(1) each), (c) going right from split, combine stored reduced values of left subtrees entirely within range. This avoids visiting the k interior nodes. The current implementation misses this optimization.

### 3b. Implementation Fidelity

| # | Prose Item | Code Location | Fidelity | Notes |
|---|-----------|---------------|:--------:|-------|
| 1 | Key-value augmented BST (§1) | `BSTKeyValueStEph` | **Faithful** | Treap with key-value pairs. `find` returns `Option<&V>` as prose suggests. |
| 2 | Size field at each node (§2) | `BSTSizeStEph::Node::size` | **Faithful** | Per-node `size: N` field, matches APAS 5-tuple structure. |
| 3 | `size T` — O(1) read (§2) | `BSTSizeStEph::size_link` | **Faithful** | `link.as_ref().map_or(0, |n| n.size)` — returns 0 for leaf, `n.size` for node. |
| 4 | `makeNode (L, k, p, R)` (§2) | `BSTSizeStEph::make_node` | **Faithful** | Computes `1 + size_link(left) + size_link(right)` via `update_size`. |
| 5 | Replace `TNode(·,·,·,·)` with `makeNode` in join (§2) | `BSTSizeStEph::insert_link` | **Faithful** | All node creation goes through `make_node` or `update_size` after rotation. |
| 6 | Algorithm 40.1: `rank T k` (§2.1) | `BSTSizeStEph::rank_link` | **Faithful** | Three-way compare: Less → recurse left, Equal → `|L|+1`, Greater → `|L|+1+rank(R,k)`. Exact match to pseudocode. |
| 7 | Algorithm 40.1: `select T i` (§2.1) | `BSTSizeStEph::select_link` | **Faithful** | Three-way compare on `(rank, left_size+1)`: Less/Equal → go left, Equal → return key, Greater → recurse right with adjusted rank. 1-indexed as in APAS. |
| 8 | Exercise 40.1: `splitRank (t, i)` | `BSTSizeStEph::split_rank_link` | **Faithful** | Recursive split using `make_node` to reconstruct complementary half. Semantics correct; cost differs (see 3a). |
| 9 | `reducedVal T` (§3) | `BSTReducedStEph::reduced_value_link` | **Faithful** | Returns `Op::identity()` for leaf, `node.reduced_value.clone()` for node. Matches APAS exactly. |
| 10 | `makeNode (L, k, v, p, R)` with reduced values (§3) | `BSTReducedStEph::make_node` + `update_node` | **Faithful** | Computes `combine(left_reduced, combine(lift(value), right_reduced))`. Uses `lift()` to convert V→R (generalizes APAS where V=R). Size computed simultaneously. |
| 11 | Reduction operation (§3) | `ReduceOp` trait | **Faithful** | `identity()`, `combine(a, b)`, `lift(value)` — clean generalization of APAS associative function `f` with identity `I`. |

**No fidelity issues found.** All prose algorithms are implemented correctly. The `ReduceOp` trait with separate `lift` function is a clean generalization that handles the case where value type V differs from reduced type R (e.g., `CountOp` where V is any type but R is `usize`).

### 3c. Spec Fidelity

**Not applicable.** All code is outside `verus!` blocks (gated with `not(verus_keep_ghost)`). No `requires`/`ensures` exist. Zero formal specifications.

Key prose properties that should be specified upon verusification:

| # | Property | APAS Source |
|---|----------|-------------|
| 1 | BST ordering invariant: ∀ k_l in left(n), k_r in right(n): k_l < n.key < k_r | Assumed throughout |
| 2 | Treap heap property: parent.priority ≤ child.priority | Chapter 39 |
| 3 | Size correctness: `node.size == 1 + size(left) + size(right)` | §2 |
| 4 | Reduced value correctness: `node.reduced == f(reduced(left), f(lift(value), reduced(right)))` | §3 |
| 5 | `rank(select(i)) == i` for valid ranks | Implied by Alg 40.1 |
| 6 | `select(rank(k)) == k` for keys in tree | Implied by Alg 40.1 |
| 7 | `splitRank(t, i)` partitions: all keys in t1 have rank < i, all keys in t2 have rank ≥ i | Exercise 40.1 |
| 8 | `reduced_value` equals fold of `f` over all values in tree | §3 definition |

---

## Phase 4: Parallelism Review

**Not applicable.** Chapter 40 contains only `*StEph*` (single-threaded ephemeral) modules. No `*Mt*` modules exist. The APAS prose does not describe parallel variants of augmented BST operations in this chapter — parallelism enters through the underlying BST operations (join, split) from earlier chapters.

---

## Phase 5: Runtime Test (RTT) Review

### 5a. Unified Test Inventory

| # | Source Module | RTT File | RTT Count | PTT File | PTT Count | Status |
|---|-------------|----------|:---------:|----------|:---------:|--------|
| 1 | `BSTKeyValueStEph` | `tests/Chap40/TestBSTKeyValueStEph.rs` | 6 | — | 0 | RTT only |
| 2 | `BSTSizeStEph` | `tests/Chap40/TestBSTSizeStEph.rs` | 8 | — | 0 | RTT only |
| 3 | `BSTReducedStEph` | `tests/Chap40/TestBSTReducedStEph.rs` | 19 | — | 0 | RTT only |
| | **Total** | | **33** | | **0** | |

### 5b. Test Quality — BSTKeyValueStEph (6 tests)

| # | Test | What It Covers | Quality |
|---|------|----------------|:-------:|
| 1 | `key_value_bst_basic_operations` | insert, find, get, contains, size, is_empty, min_key, max_key | Good |
| 2 | `key_value_bst_update_existing_key` | Overwrite semantics — size unchanged, value updated | Good |
| 3 | `key_value_bst_collections` | `keys()` and `values()` in sorted order | Good |
| 4 | `key_value_bst_macro_literal` | `BSTKeyValueStEphLit!` empty and non-empty | Good |
| 5 | `key_value_bst_height_stays_reasonable` | Height ≤ 20 for 100 elements + find all | Good |
| 6 | `key_value_bst_string_keys` | Non-integer key type (String), lexicographic ordering | Good |

**Assessment:** Solid coverage of the core API. Tests verify both integer and string key types.

### 5b. Test Quality — BSTSizeStEph (8 tests)

| # | Test | What It Covers | Quality |
|---|------|----------------|:-------:|
| 1 | `size_bst_basic_operations` | insert, find, contains, size, is_empty, min, max | Good |
| 2 | `size_bst_rank_operations` | Rank for all 9 elements + out-of-range keys (0, 10) | Excellent |
| 3 | `size_bst_select_operations` | Select for ranks 0..10 (invalid and valid, 1-indexed) | Excellent |
| 4 | `size_bst_rank_select_consistency` | `rank(select(i)) == i` and `select(rank(k)) == k` for 15 elements | Excellent |
| 5 | `size_bst_split_rank_operations` | Split at rank 5: verify partition, edge cases (rank 0, beyond size) | Good |
| 6 | `size_bst_macro_literal` | `BSTSizeStEphLit!` empty and non-empty, rank/select on literal | Good |
| 7 | `size_bst_duplicate_insert_is_idempotent` | Duplicate insert does not change size or rank | Good |
| 8 | `size_bst_large_dataset_performance` | 1000 elements: rank, select, split, height check | Good |

**Assessment:** Excellent coverage of rank/select, including the critical bidirectional consistency property `rank(select(i)) == i ∧ select(rank(k)) == k`. Large dataset test provides regression coverage.

### 5b. Test Quality — BSTReducedStEph (19 tests)

| # | Test | What It Covers | Quality |
|---|------|----------------|:-------:|
| 1 | `reduced_bst_sum_operations` | Sum reduction: insert, reduced_value, find, range_reduce | Good |
| 2 | `reduced_bst_count_operations` | Count reduction: count equals size, range count | Good |
| 3 | `reduced_bst_update_existing_key` | Value update propagates to reduced value | Good |
| 4 | `reduced_bst_collections` | keys() and values() in sorted order, total sum | Good |
| 5 | `reduced_bst_macro_literal` | `BSTReducedStEphLit!` empty and non-empty | Good |
| 6 | `reduced_bst_height_stays_reasonable` | Height ≤ 20 for 100 elements, total sum, find all | Good |
| 7 | `reduced_bst_range_queries` | Range [1,5], [2,4], [1,1], [5,5], [0,0], [6,10], [3,3] | Excellent |
| 8 | `reduced_bst_string_keys` | String keys with sum reduction, range query | Good |
| 9 | `test_contains_method` | contains() for present and absent keys | Good |
| 10 | `test_get_method` | get() for present and absent keys | Good |
| 11 | `test_minimum_maximum_keys` | Progressive insert: min/max update correctly | Good |
| 12 | `test_count_reducer_all_operations` | CountOp: count, min, max, contains, get | Good |
| 13 | `test_empty_tree_operations` | All operations on empty tree | Excellent |
| 14 | `test_single_element_operations` | All operations on single-element tree | Excellent |
| 15 | `test_large_tree_with_all_operations` | 50 elements: all operations, range queries | Good |
| 16 | `test_duplicate_keys_overwrite` | Triple overwrite: size stable, value/reduced updated | Excellent |
| 17 | `test_edge_case_ranges` | Empty range, single, full, partial, beyond bounds | Excellent |
| 18 | `test_sumop_trait` | SumOp: identity, combine, lift unit tests | Good |
| 19 | `test_countop_trait` | CountOp: identity, combine, lift unit tests | Good |

**Assessment:** Thorough coverage. Empty tree, single element, edge-case ranges, and trait unit tests are all present. The test suite is the strongest of the three modules.

### 5c. Missing / Recommended Tests

| # | Priority | Module | Proposed Test | Reason |
|---|:--------:|--------|---------------|--------|
| 1 | Medium | BSTReducedStEph | Implement and test `MaxOp` | `MaxOp` struct is declared but `ReduceOp` impl is missing |
| 2 | Low | BSTKeyValueStEph | Empty tree: find, contains, min_key, max_key all return None | Currently only tested through Reduced module's empty tree test |
| 3 | Low | BSTSizeStEph | Split rank consistency: in_order(left) ++ in_order(right) == in_order(original) | Current test only checks sizes and membership |
| 4 | Low | BSTKeyValueStEph | Delete operation (when implemented) | delete not yet implemented |
| 5 | Low | BSTSizeStEph | String key type (non-integer) | Only tested with i32 |

### 5d. Test Code Quality Notes

| # | File | Issue |
|---|------|-------|
| 1 | `TestBSTSizeStEph.rs` | Imports `apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*` — appears unused |
| 2 | `TestBSTReducedStEph.rs` | Imports `apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*` — appears unused |

---

## Phase 6: Proof-Time Test (PTT) Review

**Not applicable.** Chapter 40 has no `verus!` blocks, no iterators with ghost state, and no verified loops. Zero PTTs exist. No PTTs are needed until the chapter is verusified.

---

## Phase 7: Gap Analysis

### 7a. Prose Items with No Implementation

| # | Prose Item | APAS Section | Gap Type | Priority | Notes |
|---|-----------|:---:|----------|:--------:|-------|
| 1 | `delete` operation | Implied §1-§3 | Missing operation | Medium | Standard BST operation — needed for complete table ADT. Requires updating size and reduced values on the path to root. |
| 2 | Standalone `join` with `makeNode` | §2 | Internal only | Low | APAS says "replace three occurrences of TNode with makeNode in join." Code handles this through `insert_link` + rotations rather than exposing a standalone `join`. Correct but different structure. |
| 3 | `MaxOp` reduction implementation | §3 (general) | Stub only | Low | `MaxOp` struct declared but `ReduceOp` impl is missing (requires `Option<T>` for identity). |
| 4 | O(log n) range reduction | §3 (implied) | Suboptimal | Medium | Current `range_reduce_link` is O(log n + k) not O(log n). Could be improved to O(log n) by combining stored reduced values for subtrees entirely within range. |

### 7b. Code with No Prose Counterpart

| # | Function / Feature | Module | Justification |
|---|-------------------|--------|---------------|
| 1 | `rotate_left` / `rotate_right` | All three | Internal treap balancing — standard, not in APAS pseudocode for this chapter |
| 2 | `height()` | All three | Debugging/testing utility |
| 3 | `collect_keys` / `collect_values` / `in_order_collect` | All three | Collection helpers for `keys()`/`values()`/`in_order()` methods |
| 4 | `contains()` / `get()` | KV, Reduced | Convenience wrappers around `find` |
| 5 | `Default` impl | All three | Rust trait — delegates to `new()` |
| 6 | `BSTKeyValueStEphLit!` / `BSTSizeStEphLit!` / `BSTReducedStEphLit!` | All three | Test convenience macros |
| 7 | `range_reduce` / `range_reduce_link` | Reduced | Extension beyond APAS — efficient range query leveraging BST ordering |
| 8 | `BSTSumStEph` / `BSTCountStEph` type aliases | Reduced | Convenience aliases for common reduction types |

### 7c. Structural Observations

| # | Observation | Impact |
|---|-----------|--------|
| 1 | **No Verus code at all** — entire chapter gated with `not(verus_keep_ghost)` | Cannot be included in verification builds. All algorithmic properties are tested but not proven. |
| 2 | **Separate struct for each augmentation level** — KeyValue, Size, Reduced are independent types | Prose describes augmentation as progressive (§1 → §2 → §3 adds fields). Code uses three separate types rather than a single parameterized type. This is reasonable for Rust but means code duplication across modules (rotations, find, insert are nearly identical). |
| 3 | **Ephemeral mutation** — all three modules use `&mut self` insert with in-place rotation | Matches APAS's imperative remark (§3) about recomputing reduced values on the path from modified node to root. |
| 4 | **`BSTKeyValueStEph` tracks size on struct, not nodes** — `self.size: N` vs per-node `node.size: N` in BSTSizeStEph | Deliberate simplification — key-value BST doesn't need rank/select, so per-node size is unnecessary. |
| 5 | **`split_rank` deep-clones subtrees** — O(n) vs APAS O(log n) | Fundamental Rust ownership limitation. Could be fixed with `Rc<Node>` persistent structure, at the cost of `Clone` on every insert path. |

---

## Phase 8: TOC / In-Out Table

### TOC Presence

| # | File | TOC Present | Section Ordering | Notes |
|---|------|:-----------:|:----------------:|-------|
| 1 | `BSTKeyValueStEph.rs` | No | N/A | No `verus!` blocks — TOC standard does not apply |
| 2 | `BSTSizeStEph.rs` | No | N/A | No `verus!` blocks — TOC standard does not apply |
| 3 | `BSTReducedStEph.rs` | No | N/A | No `verus!` blocks — TOC standard does not apply |

### In/Out Table

Since there are no `verus!` blocks, all trait impls are trivially outside. The table reflects what exists:

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | `BSTKeyValueStEph.rs` | `✅ out` | - | `✅ out` | - | - | `✅ out` | - | `✅ out` | - |
| 2 | `BSTSizeStEph.rs` | `✅ out` | - | `✅ out` | - | - | `✅ out` | - | `✅ out` | - |
| 3 | `BSTReducedStEph.rs` | `✅ out` | - | `✅ out` | - | - | `✅ out` | - | `✅ out` | - |

**Note:** All derive/trait impls are outside `verus!` trivially, since no `verus!` blocks exist. Upon verusification: Clone, Default should move inside `verus!` with specs. Debug must remain outside. Macros must remain outside.

### Module Header Format

| # | File | Copyright Format | SPDX | Module Doc | Issue |
|---|------|:---:|:---:|:---:|-------|
| 1 | `BSTKeyValueStEph.rs` | `//!` | Missing | `//!` | Copyright should use `//` not `//!` per module-header rule |
| 2 | `BSTSizeStEph.rs` | `//!` | Missing | `//!` | Same |
| 3 | `BSTReducedStEph.rs` | `//!` | Missing | `//!` | Same |

---

## Proof Holes

```
✓ BSTKeyValueStEph.rs
✓ BSTReducedStEph.rs
✓ BSTSizeStEph.rs

Modules:     3 clean, 0 holed
Holes Found: 0 total
```

Trivially clean — no `verus!` code exists to contain proof holes.

---

## Review TODOs

### High Priority

| # | Category | Item |
|---|----------|------|
| 1 | Verusification | Add `verus!` blocks with BST ordering invariant and treap heap property |
| 2 | Verusification | Prove size correctness: `node.size == 1 + size(left) + size(right)` |
| 3 | Verusification | Prove `rank(select(i)) == i` and `select(rank(k)) == k` |
| 4 | Verusification | Prove reduced value correctness: `node.reduced == f(reduced(left), f(lift(value), reduced(right)))` |
| 5 | Cost annotation | Fix `range_reduce_link` cost: O(log n + k) not O(log n) |

### Medium Priority

| # | Category | Item |
|---|----------|------|
| 6 | Algorithm | Implement O(log n) range reduction exploiting stored reduced values |
| 7 | Missing op | Implement `delete` for all three modules (with size/reduced value path updates) |
| 8 | Cost annotation | Document `split_rank` clone cost as O(n) in Rust (vs APAS O(log n) with persistent nodes) |
| 9 | RTT | Implement and test `MaxOp` (struct exists but `ReduceOp` impl is missing) |

### Low Priority

| # | Category | Item |
|---|----------|------|
| 10 | Code quality | Remove redundant `update_size(&mut y)` / `update_node(&mut y)` inside rotation functions — y's size/reduced value is computed with y.left=None, then immediately overwritten by the caller's update |
| 11 | Style | Fix module headers: copyright should use `//` not `//!`, add SPDX line |
| 12 | Style | Remove unused `Chap19` imports from `TestBSTSizeStEph.rs` and `TestBSTReducedStEph.rs` |
| 13 | RTT | Add empty-tree test for `BSTKeyValueStEph` |
| 14 | RTT | Add string-key test for `BSTSizeStEph` |
| 15 | Structure | Consider refactoring: shared base treap with augmentation as type parameter (reduces code duplication across three nearly-identical treap implementations) |
| 16 | TOC | Add TOC headers when verusified |

---

## Overall Assessment

Chapter 40 faithfully implements all three augmentation types from the APAS prose:

1. **Key-value augmentation (§1)** — `BSTKeyValueStEph` provides a dictionary/table interface over a treap with `find` returning `Option<&V>` as prose specifies.
2. **Size augmentation (§2)** — `BSTSizeStEph` stores per-node subtree sizes enabling O(1) size and O(log n) rank/select. Algorithm 40.1 and Exercise 40.1 are both implemented with exact pseudocode fidelity.
3. **Reduced-value augmentation (§3)** — `BSTReducedStEph` generalizes augmentation with a `ReduceOp` trait (identity, combine, lift), supporting `SumOp` and `CountOp` concretely.

**Strengths:**
- All APAS algorithms faithfully implemented with correct semantics
- 33 runtime tests with excellent coverage, including rank-select consistency and comprehensive edge cases
- Clean `ReduceOp` trait abstraction generalizing the APAS reduction concept
- Zero proof holes (trivially)

**Weaknesses:**
- No formal verification — entire chapter is plain Rust with zero `verus!` code
- `range_reduce` is O(log n + k) but annotated O(log n) — misses the key optimization that augmented BSTs enable
- `split_rank` has O(n) clone cost vs APAS O(log n) — Rust ownership limitation
- No `delete` operation in any module
- Three nearly-identical treap implementations with no shared base
