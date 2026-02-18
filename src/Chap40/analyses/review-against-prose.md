<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 40: Augmenting Binary Search Trees — Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6
**Source files:** `src/Chap40/BSTKeyValueStEph.rs`, `src/Chap40/BSTSizeStEph.rs`, `src/Chap40/BSTReducedStEph.rs`

## Phase 1: Inventory (tool-generated)

Veracity extracted **83 functions** across 3 source files. All code is **plain Rust** — no `verus!` blocks, no `requires`/`ensures`, no formal specifications. All 83 entries classified as `spec_strength: none`.

| # | Module | Trait | IT | IBI | ML | V! | -V! | NoSpec |
|---|--------|:-----:|:--:|:---:|:--:|:--:|:---:|:------:|
| 1 | BSTKeyValueStEph | 12 | 12 | 0 | 9 | 0 | 23 | 23 |
| 2 | BSTSizeStEph | 13 | 13 | 15 | 0 | 0 | 28 | 28 |
| 3 | BSTReducedStEph | 17 | 17 | 18 | 0 | 0 | 32 | 32 |

## Phase 2: Prose Inventory

### Definitions
| # | Item | Description |
|---|------|-------------|
| 1 | Key-value augmented node | `TNode of (T × K × Z × T)` — key-value pair BST (dictionary/table) |
| 2 | Size-augmented node | `TNode of (T × K × Z × Z × T)` — 5-tuple with size field `n` |
| 3 | Reduced-value augmented node | `TNode of (T × K × Z × Z × val × val × T)` — 7-tuple with value `v` and reduced value `r` |
| 4 | `ReduceOp` | Associative function `f` with identity `I`, used for reduced values |

### Algorithms
| # | Item | Description |
|---|------|-------------|
| 1 | `size T` | O(1) size read from augmented field |
| 2 | `makeNode (L, k, p, R)` | Create node computing `size(L) + size(R) + 1` |
| 3 | Algorithm 40.1: `rank T k` | Number of keys ≤ k, uses `|L|` (size of left subtree) |
| 4 | Algorithm 40.1: `select T i` | Key with rank `i`, recurses using `|L|` |
| 5 | Exercise 40.1: `splitRank (t, i)` | Splits tree at rank boundary |
| 6 | `reducedVal T` | O(1) read of reduced value from root (returns identity for leaf) |
| 7 | `makeNode (L, k, v, p, R)` (reduced) | Creates node computing `f(reducedVal(L), f(v, reducedVal(R)))` and `size(L) + 1 + size(R)` |

### Cost Specs
| # | Algorithm | APAS Work | APAS Span |
|---|-----------|-----------|-----------|
| 1 | `size` | Θ(1) | Θ(1) |
| 2 | `makeNode` | Θ(1) | Θ(1) |
| 3 | `rank` (without augmentation) | Θ(n) | Θ(log n) |
| 4 | `rank` (with size augmentation) | Θ(log n) | Θ(log n) |
| 5 | `select` (with size augmentation) | Θ(log n) | Θ(log n) |
| 6 | `reducedVal` | Θ(1) | Θ(1) |

### Exercises/Problems
| # | Item | Status |
|---|------|--------|
| 1 | Exercise 40.1: `splitRank` | Implemented in `BSTSizeStEph` |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

Cost annotations have been added to all unannotated functions. Several trait methods already had `claude-4-sonet` annotations from initial development. New annotations use the `APAS` / `Claude-Opus-4.6` format.

**Cost disagreements found:** None. All implementations match the APAS cost bounds. The implementations are sequential (StEph = single-threaded ephemeral), so Work == Span for all operations.

### 3b. Implementation Fidelity

| # | Prose Item | Code Function | Fidelity | Notes |
|---|-----------|---------------|----------|-------|
| 1 | Key-value augmented BST | `BSTKeyValueStEph` | **Faithful** | Standard treap with key-value pairs. `find` returns `Option<&V>` as suggested. |
| 2 | Size-augmented BST / `makeNode` | `BSTSizeStEph::make_node` | **Faithful** | Computes `1 + size(left) + size(right)` exactly as APAS. |
| 3 | `size T` (O(1)) | `BSTSizeStEph::size_link` | **Faithful** | Reads `node.size` field directly. |
| 4 | Algorithm 40.1: `rank` | `BSTSizeStEph::rank_link` | **Faithful** | Three-way compare using `size_link` for left subtree size. Matches pseudocode exactly. |
| 5 | Algorithm 40.1: `select` | `BSTSizeStEph::select_link` | **Faithful** | Three-way compare on `left_size + 1`. 1-indexed as in APAS. |
| 6 | Exercise 40.1: `splitRank` | `BSTSizeStEph::split_rank_link` | **Faithful** | Recursive split using `make_node`. Clones subtrees (acceptable for persistent semantics within ephemeral wrapper). |
| 7 | `reducedVal T` | `BSTReducedStEph::reduced_value_link` | **Faithful** | Returns `Op::identity()` for leaf, `node.reduced_value` otherwise. |
| 8 | `makeNode` (reduced) | `BSTReducedStEph::make_node` / `update_node` | **Faithful** | Computes `f(left_reduced, f(node_value, right_reduced))` exactly as APAS §3. |

**Deviations:**
- `BSTKeyValueStEph` tracks `size` as a separate field on the struct (not on each node), unlike `BSTSizeStEph` which augments each node. This is a simplification since key-value BSTs don't need rank/select.
- `BSTReducedStEph::range_reduce_link` has no direct APAS counterpart — it's an extension that prunes subtrees using BST ordering. The implementation correctly leverages the BST property to avoid visiting subtrees outside the range.
- `split_rank` in `BSTSizeStEph` clones nodes (`node.key.clone()`, `node.left.clone()`) giving O(n) work per split in the worst case due to deep cloning, rather than the O(log n) APAS bound. This is a Rust ownership limitation — a persistent (Rc/Arc) node structure would achieve true O(log n).

### 3c. Spec Fidelity

**Not applicable.** All 83 functions have `spec_strength: none` — no `requires`/`ensures` exist. The code is entirely outside `verus!` blocks.

Prose properties that *should* be specified once verusified:
- `rank(select(i)) == i` for valid ranks (tested in RTT but not formally proven)
- `select(rank(k)) == k` for keys in the tree
- `reduced_value` equals the fold of all values under the reduction operation
- `size` equals the number of distinct keys in the tree
- BST ordering invariant: all keys in left subtree < node key < all keys in right subtree
- Treap heap property: parent priority ≤ child priority

## Phase 4: Parallelism Review

**Not applicable.** Chapter 40 contains only `*StEph*` (single-threaded ephemeral) modules. No `*Mt*` modules exist. All operations are sequential. The APAS prose does not describe parallel variants of augmented BST operations in this chapter.

## Phase 5: Runtime Test Review

### 5a. Unified Test Inventory

| # | Source module | RTT file | PTT file | Status |
|---|-------------|----------|----------|--------|
| 1 | `BSTKeyValueStEph` | `tests/Chap40/TestBSTKeyValueStEph.rs` | — | RTT only |
| 2 | `BSTSizeStEph` | `tests/Chap40/TestBSTSizeStEph.rs` | — | RTT only |
| 3 | `BSTReducedStEph` | `tests/Chap40/TestBSTReducedStEph.rs` | — | RTT only |

### 5b. Test Quality

**BSTKeyValueStEph** (5 tests):
- Happy path: insert, find, get, contains, keys, values — **good**
- Edge cases: empty tree, update existing key, string keys — **good**
- Performance: height check for 100 elements — **good**
- Macro literal test — **good**
- Missing: empty tree operations (find on empty returns None)

**BSTSizeStEph** (7 tests):
- Happy path: insert, find, contains, min, max — **good**
- Rank tests: comprehensive for 9-element tree with all ranks — **excellent**
- Select tests: comprehensive 1-indexed selection with boundary — **excellent**
- Rank-select consistency: `rank(select(i)) == i` and `select(rank(k)) == k` — **excellent**
- Split rank: verifies left/right partition, edge cases (rank 0, beyond size) — **good**
- Large dataset (1000 elements): rank, select, split, height check — **good**
- Duplicate insert idempotency — **good**

**BSTReducedStEph** (13 tests):
- Sum reduction: basic operations, range queries — **excellent**
- Count reduction: verifies count equals size — **good**
- Update existing key: verifies reduced value updates — **good**
- Range queries: comprehensive (all, partial, single, empty, beyond bounds) — **excellent**
- String keys with range queries — **good**
- Empty tree: all operations — **excellent**
- Single element: all operations — **excellent**
- Large tree (50 elements): all operations — **good**
- Trait unit tests for `SumOp` and `CountOp` — **good**
- Missing: `MaxOp` not tested (struct defined but `ReduceOp` impl not provided)

### 5c. Missing Tests

| # | Priority | Proposed Test |
|---|----------|---------------|
| 1 | Low | `BSTKeyValueStEph`: empty tree find/contains/min_key/max_key returns None |
| 2 | Medium | `BSTReducedStEph`: implement and test `MaxOp` (struct exists but no impl) |
| 3 | Low | `BSTSizeStEph`: split_rank consistency — union of split halves equals original |

## Phase 6: Proof-Time Test (PTT) Review

**Not applicable.** Chapter 40 has no `verus!` blocks, no iterators with ghost state, and no verified loops. No PTTs are needed.

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Gap Type | Priority |
|---|-----------|----------|----------|
| 1 | `delete` operation | Not implemented in any Chap40 module | Medium — standard BST operation, useful for table ADT |
| 2 | `join` with `makeNode` | Only `insert` uses `makeNode`; no standalone `join` exposed | Low — internal to treap implementation |
| 3 | `MaxOp` reduction | Struct declared but `ReduceOp` impl missing (commented out) | Low |

### Code with No Prose Counterpart

| # | Function | Justification |
|---|----------|---------------|
| 1 | `rotate_left` / `rotate_right` | Internal treap balancing — standard, not in APAS pseudocode for this chapter |
| 2 | `height` | Debugging/testing utility — not part of APAS augmented BST interface |
| 3 | `collect_keys` / `collect_values` / `in_order_collect` | Collection helpers for testing and the `keys()`/`values()`/`in_order()` methods |
| 4 | `contains` / `get` | Convenience wrappers around `find` |
| 5 | `default` | Rust `Default` trait — delegates to `new()` |
| 6 | Macros (`BSTKeyValueStEphLit`, `BSTSizeStEphLit`, `BSTReducedStEphLit`) | Test convenience — literal constructors |
| 7 | `range_reduce` | Extension beyond APAS — leverages BST ordering for efficient range queries |

## Phase 8: Table of Contents Review

### TOC Presence

| # | File | TOC Present | Section Ordering | In/Out Correct | Section Headers |
|---|------|:-----------:|:----------------:|:--------------:|:---------------:|
| 1 | `BSTKeyValueStEph.rs` | No | N/A | N/A (no verus!) | No |
| 2 | `BSTSizeStEph.rs` | No | N/A | N/A (no verus!) | No |
| 3 | `BSTReducedStEph.rs` | No | N/A | N/A (no verus!) | No |

**Note:** Since these files have no `verus!` blocks, the TOC standard for verus sections (1-13) does not directly apply. When verusified, TOC headers should be added.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | `BSTKeyValueStEph.rs` | `✅ out` | - | `✅ out` | - | - | `✅ out` | - | `✅ out` | - |
| 2 | `BSTSizeStEph.rs` | `✅ out` | - | `✅ out` | - | - | `✅ out` | - | `✅ out` | - |
| 3 | `BSTReducedStEph.rs` | `✅ out` | - | `✅ out` | - | - | `✅ out` | - | `✅ out` | - |

All derive impls are outside `verus!` which is trivially correct since there are no `verus!` blocks. When verusified, Clone/PartialEq/Eq/Default should move inside `verus!` with specs per project rules.

## Proof Holes Summary

```
✓ BSTKeyValueStEph.rs
✓ BSTReducedStEph.rs
✓ BSTSizeStEph.rs

Modules:   3 clean, 0 holed
Holes Found: 0 total
```

No proof holes — trivially, since there is no verus! code to have holes in.

## Spec Strength Summary

| Classification | Count |
|----------------|------:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 83 |

All 83 functions have no formal specifications. The entire chapter is unverified plain Rust.

## Overall Assessment

**Chapter 40 implements all three augmentation types described in the APAS prose:**

1. **Key-value augmentation** (§1) — `BSTKeyValueStEph` provides a dictionary/table interface over a treap.
2. **Size augmentation** (§2) — `BSTSizeStEph` stores subtree sizes, enabling O(1) size queries and O(log n) rank/select/splitRank (Algorithm 40.1, Exercise 40.1).
3. **Reduced-value augmentation** (§3) — `BSTReducedStEph` generalizes augmentation with an associative `ReduceOp` trait, supporting `SumOp` and `CountOp` (with `MaxOp` stub).

**Strengths:**
- All APAS algorithms are faithfully implemented with correct cost characteristics
- Excellent runtime test coverage, especially for rank/select consistency
- Clean separation of concerns across three modules
- The `ReduceOp` trait is a good generalization of the APAS pattern

**Weaknesses:**
- **No formal verification** — zero `verus!` code, zero specs, zero proof obligations
- `split_rank` clones subtrees (O(n) for deep clone) rather than sharing (O(log n) with Rc/Arc)
- No `delete` operation in any module
- `MaxOp` declared but not implemented
- No TOC headers (expected — will be needed when verusified)
- Existing cost annotations use non-standard `claude-4-sonet` format (legacy from initial generation)

**Priority actions for verusification:**
1. Add `verus!` blocks with BST ordering invariant and size/reduced-value correctness specs
2. Prove `rank(select(i)) == i` and `select(rank(k)) == k`
3. Prove `reduced_value` equals fold over all values
4. Implement `delete` for all three modules
5. Add TOC headers per standard
