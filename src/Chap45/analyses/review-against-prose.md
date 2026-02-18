<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 45: Priority Queues — Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap45.txt`

## Critical Note: Non-Verus Chapter

Chapter 45 is gated with `#[cfg(all(not(any(feature = "experiments_only", feature = "dev_only")), not(verus_keep_ghost)))]` in `src/lib.rs`. This means:

- **No `verus!` blocks** — all code is plain Rust.
- **No specifications** — no `requires`, `ensures`, `spec fn`, `proof fn`.
- **No Verus verification** — the chapter is excluded from Verus runs entirely.
- **No proof holes** — trivially, since nothing is verified.
- **No spec strength classifications** — no entries extracted by veracity.

This is the most fundamental finding of the review: Chapter 45 has **zero formal verification**.

## Phase 1: Inventory (Tool-Generated)

The veracity tool (`veracity-review-module-fn-impls -d src/Chap45`) found **0 entries** for Chap45 because no functions exist inside `verus!` blocks.

### Source Files

| # | File | Lines | Description |
|---|------|------:|------------|
| 1 | `UnsortedListPQ.rs` | 235 | PQ using unsorted ArraySeqStPerS |
| 2 | `SortedListPQ.rs` | 311 | PQ using sorted ArraySeqStPerS |
| 3 | `BalancedTreePQ.rs` | 419 | PQ using AVLTreeSeqStPerS |
| 4 | `BinaryHeapPQ.rs` | 383 | PQ using ArraySeqStPerS as binary heap |
| 5 | `LeftistHeapPQ.rs` | 428 | PQ using recursive leftist heap tree |
| 6 | `HeapsortExample.rs` | 355 | Algorithm 45.2 — heapsort using all five PQ types |
| 7 | `Example45_2.rs` | 155 | Wrapper demonstrating heapsort examples |

### Function Count per Module (exec functions only)

| # | Module | Trait decls | Impl fns | Inherent fns | Free fns | Total |
|---|--------|:----------:|:--------:|:------------:|:--------:|:-----:|
| 1 | UnsortedListPQ | 14 | 14 | 0 | 0 | 14 |
| 2 | SortedListPQ | 16 | 16 | 0 | 0 | 16 |
| 3 | BalancedTreePQ | 18 | 18 | 6 | 0 | 24 |
| 4 | BinaryHeapPQ | 9 | 9 | 9 | 0 | 18 |
| 5 | LeftistHeapPQ | 15 | 15 | 0 | 2 | 17 |
| 6 | HeapsortExample | 12 | 0 | 2 | 14 | 16 |
| 7 | Example45_2 | 9 | 0 | 0 | 9 | 9 |

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|------------|
| 1 | Data Type 45.1 | Meldable Priority Queue ADT: empty, singleton, findMin, insert, deleteMin, meld, fromSeq |
| 2 | Data Structure 45.3 | Naïve Meldable Binary Heap (leftist heap precursor) — datatype with Leaf/Node, meld algorithm |

### Algorithms

| # | Item | Description |
|---|------|------------|
| 1 | Algorithm 45.2 | Heapsort — insert all keys into PQ, extract in order via deleteMin |
| 2 | Meld (Data Structure 45.3) | Recursive meld following right spines |

### Cost Specs (from prose Table)

| # | Implementation | insert | deleteMin | meld | fromSeq |
|---|---------------|--------|-----------|------|---------|
| 1 | Unsorted List | O(1) | O(n) | O(m+n) | O(n) |
| 2 | Sorted List | O(n) | O(1) | O(m+n) | O(n log n) |
| 3 | Balanced Trees | O(log n) | O(log n) | O(m log(1+n/m)) | O(n log n) |
| 4 | Binary Heaps | O(log n) | O(log n) | O(m+n) | O(n) |
| 5 | Leftist Heap | O(log n) | O(log n) | O(log m + log n) | O(n) |

### Theorems/Properties

| # | Property | Description |
|---|----------|------------|
| 1 | Heap property | Key at every node ≤ keys of all descendants |
| 2 | Leftist property | Left subtree rank ≥ right subtree rank |
| 3 | Shape property (binary heap) | Complete binary tree, filled from left |
| 4 | Heapsort correctness | Output is sorted permutation of input |

### Exercises/Problems

None explicitly numbered in the prose excerpt.

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Disagreements

The **dominant finding** is that all implementations use **persistent (immutable)** data structures (`ArraySeqStPerS`, `AVLTreeSeqStPerS`, `Box<LeftistHeapNode>` with `Clone`), while the APAS costs assume **ephemeral** (mutable linked lists or arrays). This systematically inflates all costs by at least an O(n) factor.

| # | Module | Function | APAS Cost | Claude-Opus-4.6 Cost | Reason |
|---|--------|----------|-----------|---------------------|--------|
| 1 | UnsortedListPQ | insert | Θ(1) | Θ(n) | `ArraySeqStPerS::append` copies entire sequence |
| 2 | SortedListPQ | delete_min | Θ(1) | Θ(n) | Rebuilds array without first element |
| 3 | SortedListPQ | from_seq | Θ(n log n) | Θ(n²) | Repeated O(n) insert, not a sort-based approach |
| 4 | BalancedTreePQ | insert | Θ(log n) | Θ(n) | Converts to Vec, linear scan, rebuilds tree |
| 5 | BalancedTreePQ | delete_min | Θ(log n) | Θ(n) | Converts to Vec, removes index 0, rebuilds |
| 6 | BalancedTreePQ | meld | Θ(m log(1+n/m)) | Θ(m+n) | Flattens both to Vec, merges, rebuilds |
| 7 | BalancedTreePQ | from_seq | Θ(n log n) | Θ(n²) | Repeated O(n) insert |
| 8 | BinaryHeapPQ | insert | Θ(log n) | Θ(n log n) | Each swap rebuilds entire array O(n) |
| 9 | BinaryHeapPQ | delete_min | Θ(log n) | Θ(n log n) | Each swap rebuilds entire array O(n) |
| 10 | BinaryHeapPQ | meld | Θ(m+n) | Θ((m+n) log(m+n)) | heapify: each bubble_down has O(n) swap cost |
| 11 | BinaryHeapPQ | from_seq | Θ(n) | Θ(n² log n) | heapify with O(n) per swap |
| 12 | LeftistHeapPQ | insert | Θ(log n) | Θ(n) | meld clones entire tree O(n) |
| 13 | LeftistHeapPQ | delete_min | Θ(log n) | Θ(n) | Clones subtrees O(n) before meld |
| 14 | LeftistHeapPQ | meld | Θ(log m + log n) | Θ(m+n) | Clones entire trees before recursion |
| 15 | LeftistHeapPQ | from_seq | Θ(n) | Θ(n²) | Sequential reduce; each meld clones O(n) |

### Phase 3b: Implementation Fidelity

| # | Prose Item | Implementation | Fidelity | Notes |
|---|-----------|---------------|----------|-------|
| 1 | Data Type 45.1 (MPQ ADT) | All 5 modules | ✅ Faithful | All seven ADT operations implemented in each |
| 2 | Algorithm 45.2 (Heapsort) | `HeapsortExample.rs` | ✅ Faithful | Follows insert-all-then-extract pattern exactly |
| 3 | Data Structure 45.3 (Leftist Heap) | `LeftistHeapPQ.rs` | ✅ Faithful | `meld_nodes` follows the prose algorithm precisely |
| 4 | Unsorted List PQ | `UnsortedListPQ.rs` | ⚠️ Deviation | Uses persistent array instead of linked list — changes insert from O(1) to O(n) |
| 5 | Sorted List PQ | `SortedListPQ.rs` | ⚠️ Deviation | Uses persistent array — delete_min becomes O(n) instead of O(1) |
| 6 | Balanced Trees PQ | `BalancedTreePQ.rs` | ❌ Major deviation | All operations flatten to Vec and rebuild — destroys O(log n) advantage |
| 7 | Binary Heaps PQ | `BinaryHeapPQ.rs` | ⚠️ Deviation | Correct algorithm but swap_elements is O(n) per swap instead of O(1) |
| 8 | fromSeq (Leftist Heap) | `LeftistHeapPQ::from_seq` | ⚠️ Deviation | APAS uses parallel reduce; implementation is sequential while-loop |

### Phase 3c: Spec Fidelity

**N/A** — No Verus specifications exist in any Chap45 file. All functions are unspecified plain Rust.

## Phase 4: Parallelism Review

**No Mt (multi-threaded) modules exist in Chapter 45.** All implementations are single-threaded (`St`-style). The prose mentions that `fromSeq` for leftist heaps can be parallelized via `reduce`, and the implementation's `from_seq` comment mentions "can be done in parallel" but actually uses a sequential `while` loop.

| # | Function | APAS Span | Actual | Parallel? | Notes |
|---|----------|-----------|--------|-----------|-------|
| 1 | LeftistHeapPQ::from_seq | Θ(log² n) | Θ(n²) | No | Sequential reduce with while loop |

## Phase 5: Runtime Test Review

### 5a: Coverage Check

| # | Source Module | RTT File | Test Count | Status |
|---|-------------|----------|:----------:|--------|
| 1 | UnsortedListPQ | `TestUnsortedListPQ.rs` | 23 | ✅ Exists |
| 2 | SortedListPQ | `TestSortedListPQ.rs` | 29 | ✅ Exists |
| 3 | BalancedTreePQ | `TestBalancedTreePQ.rs` | 28 | ✅ Exists |
| 4 | BinaryHeapPQ | `TestBinaryHeapPQ.rs` | 27 | ✅ Exists |
| 5 | LeftistHeapPQ | `TestLeftistHeapPQ.rs` | 26 | ✅ Exists |
| 6 | HeapsortExample | `TestHeapsortExample.rs` | 17 | ✅ Exists |
| 7 | Example45_2 | `TestExample45_2.rs` | 8 | ✅ Exists |

Total: **158 test functions** across 7 test files. Coverage is excellent.

### 5b: Test Quality

Tests cover:
- ✅ Happy path (insert, find_min, delete_min, meld)
- ✅ Edge cases (empty, singleton, all-same elements)
- ✅ Duplicate elements
- ✅ Negative numbers and mixed positive/negative
- ✅ String elements (non-numeric types)
- ✅ Large datasets (100 elements)
- ✅ Persistent semantics (original unchanged after operations)
- ✅ Macro creation (`*PQLit!`)
- ✅ Display formatting
- ✅ Structural invariants (heap property, leftist property, sorted order)
- ✅ Cross-implementation correctness (all heapsorts produce same output)

### 5c: Missing Tests

No significant gaps. The test suite is comprehensive for a non-verified chapter.

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed.** Chapter 45 has no `verus!` blocks, no iterators with ghost state, and no verified loops. PTTs are not applicable.

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | decreaseKey operation | Not implemented (prose mentions it as "sometimes useful") |
| 2 | Parallel fromSeq | Not implemented — prose describes parallel reduce, code is sequential |
| 3 | Leftist heap rank/balance analysis | Not formally proved |

### Code with No Prose Counterpart

| # | Item | Purpose |
|---|------|---------|
| 1 | `size()`, `is_empty()`, `to_seq()` | Utility functions for all PQ types |
| 2 | `insert_all()`, `extract_all_sorted()` | Bulk operation helpers |
| 3 | `from_vec()`, `to_vec()`, `to_sorted_vec()` | Rust Vec interop for testing |
| 4 | `find_max()`, `delete_max()` (SortedListPQ, BalancedTreePQ) | Additional operations not in ADT |
| 5 | `contains()`, `remove()`, `range()` (BalancedTreePQ) | BST-like operations |
| 6 | `split()`, `join()`, `filter()`, `map()` (BalancedTreePQ) | Functional operations |
| 7 | `is_valid_heap()`, `is_valid_leftist_heap()` | Invariant checkers for testing |
| 8 | `height()`, `root_rank()`, `level_elements()` | Structural inspection for testing |
| 9 | `efficient_multi_way_merge()`, `parallel_heap_construction()` | Demo functions for leftist heap |
| 10 | `Example45_2.rs` entirely | Heapsort demonstration wrapper |
| 11 | `*PQLit!` macros | Convenience macros for literal construction |
| 12 | `Default`, `Display`, `Debug` impls | Trait impls for ergonomics |
| 13 | `HeapsortComparison` struct | Test/demo infrastructure |

## Phase 8: Table of Contents Review

**No TOC headers present** in any of the 7 files. Since these are non-Verus files (outside `verus!`), the TOC standard does not strictly apply, but it would improve navigation.

### In/Out Table

Not applicable — no `verus!` blocks exist. All derive impls and trait impls are in plain Rust.

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | UnsortedListPQ | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 2 | SortedListPQ | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 3 | BalancedTreePQ | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 4 | BinaryHeapPQ | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 5 | LeftistHeapPQ | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 6 | HeapsortExample | - | ✅ out | - | - | - | ✅ out | - | - | - |
| 7 | Example45_2 | - | - | - | - | - | - | - | - | - |

All placements are correct for a non-Verus chapter: everything is outside `verus!` because there is no `verus!`.

## Proof Holes Summary

```
✓ BalancedTreePQ.rs
✓ BinaryHeapPQ.rs
✓ Example45_2.rs
✓ HeapsortExample.rs
✓ LeftistHeapPQ.rs
✓ SortedListPQ.rs
✓ UnsortedListPQ.rs

Modules: 7 clean, 0 holed
Proof Functions: 0 total
Holes Found: 0 total
```

**0 proof holes** — trivially clean because no Verus verification exists.

## Spec Strength Summary

| Classification | Count |
|---------------|:-----:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | all |

**All functions have no specifications** — the entire chapter is plain Rust without `verus!` blocks.

## Overall Assessment

### Strengths

1. **Complete ADT coverage**: All five priority queue implementations from the prose are present (Unsorted List, Sorted List, Balanced Trees, Binary Heaps, Leftist Heap).
2. **Faithful algorithmic structure**: The leftist heap meld algorithm (`meld_nodes`) precisely follows Data Structure 45.3 from the prose.
3. **Excellent test coverage**: 158 tests across 7 test files, covering happy paths, edge cases, invariant checking, persistence semantics, and cross-implementation correctness.
4. **Heapsort demonstration**: Algorithm 45.2 is implemented cleanly with a cross-comparison framework.

### Weaknesses

1. **No Verus verification**: The entire chapter is unverified plain Rust. This is the single largest gap — no formal guarantees on correctness, invariant maintenance, or cost bounds.
2. **Systematic cost inflation**: All implementations use persistent (immutable) data structures with O(n) clone costs that the APAS costs don't account for. **15 cost disagreements** found (see Phase 3a). The worst case is BinaryHeapPQ where costs inflate from O(log n) to O(n log n) per operation.
3. **BalancedTreePQ is a degenerate implementation**: Converts to `Vec` for every operation, losing all O(log n) tree advantages. Effectively a sorted array with O(n) rebuilds.
4. **No parallelism**: Despite the prose describing parallel `fromSeq` for leftist heaps, no parallel implementations exist. No Mt variants at all.
5. **fromSeq inefficiency**: SortedListPQ and BalancedTreePQ use repeated insert (O(n²)) instead of more efficient construction methods.

### Recommendations

1. **Verusify the leftist heap** — it's the most algorithmically interesting structure in the chapter and would benefit most from formal verification of the heap and leftist properties.
2. **Fix BalancedTreePQ** — use actual AVL tree insert/delete operations instead of Vec round-tripping.
3. **Add Mt variant for LeftistHeapPQ** — the prose specifically describes parallel fromSeq via reduce, which is a natural fit for fork-join parallelism.
4. **Consider ephemeral variants** — StEph implementations using `&mut` would match APAS costs more closely, especially for BinaryHeapPQ where in-place swaps are critical.
