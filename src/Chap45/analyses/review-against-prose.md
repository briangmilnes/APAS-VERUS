<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 45: Priority Queues — Review Against Prose

**Date:** 2026-02-17
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

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|------------|
| 1 | Data Type 45.1 | Meldable Priority Queue ADT: `empty`, `singleton`, `findMin`, `insert`, `deleteMin`, `meld`, `fromSeq` |
| 2 | Data Structure 45.3 | Naïve Meldable Binary Heap — `datatype PQ = Leaf | Node of (key × PQ × PQ)`, recursive `meld` following right spines |

### Algorithms

| # | Item | Description |
|---|------|------------|
| 1 | Algorithm 45.2 | Heapsort — insert all keys into PQ via `Sequence.iter PQ.insert PQ.empty S`, then extract in order via recursive `hsort` calling `PQ.deleteMin` |
| 2 | Meld (Data Structure 45.3) | Recursive meld: compare roots, keep smaller as root with its left subtree, meld other tree with the smaller root's right subtree |
| 3 | `fromSeq` (via reduce) | `Seq.reduce Q.meld Q.empty (Seq.map Q.singleton S)` — parallel construction |

### Cost Specs (from Prose Table)

| # | Implementation | insert | deleteMin | meld | fromSeq |
|---|---------------|--------|-----------|------|---------|
| 1 | Unsorted List | O(1) | O(n) | O(m+n) | O(n) |
| 2 | Sorted List | O(n) | O(1) | O(m+n) | O(n log n) |
| 3 | Balanced Trees | O(log n) | O(log n) | O(m log(1+n/m)) | O(n log n) |
| 4 | Binary Heaps | O(log n) | O(log n) | O(m+n) | O(n) |
| 5 | Leftist Heap | O(log n) | O(log n) | O(log m + log n) | O(n) |

### Properties

| # | Property | Description |
|---|----------|------------|
| 1 | Heap property | Key at every node ≤ keys of all its descendants (min-heap) |
| 2 | Leftist property | Left subtree rank ≥ right subtree rank at every node |
| 3 | Shape property (binary heap) | Complete binary tree, filled from left |
| 4 | Heapsort correctness | Output is a sorted permutation of input |
| 5 | Binary heap index formulas | `left(i) = 2i+1`, `right(i) = 2i+2`, `parent(i) = ⌈i/2⌉ − 1` |

### Prose Items Not Numbered as Exercises

The prose excerpt contains no numbered exercises. It mentions `decreaseKey` as "sometimes useful" but does not define it formally.

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Disagreements

The **dominant structural issue** is that all implementations use **persistent (immutable)** data structures (`ArraySeqStPerS`, `AVLTreeSeqStPerS`, `Box<LeftistHeapNode>` with `Clone`), while the APAS costs assume **ephemeral** (mutable) linked lists or arrays. This systematically inflates costs.

A secondary issue is the **quadratic rebuild pattern**: several functions rebuild sequences by repeatedly appending singletons to a growing `ArraySeqStPerS`. Each append of a singleton to a result of size k costs O(k+1), so a loop of n appends costs O(1+2+...+n) = O(n²). This affects more functions than the previous review identified.

| # | Module | Function | APAS Cost | Actual Cost | Reason |
|---|--------|----------|-----------|-------------|--------|
| 1 | UnsortedListPQ | `insert` | O(1) | O(n) | `append` copies entire persistent array |
| 2 | UnsortedListPQ | `delete_min` | O(n) | O(n²) | O(n) scan + O(n²) quadratic rebuild via repeated append |
| 3 | SortedListPQ | `insert` | O(n) | O(n²) | O(n) search + O(n²) quadratic rebuild via repeated append |
| 4 | SortedListPQ | `delete_min` | O(1) | O(n²) | Rebuilds array without first element via repeated append |
| 5 | SortedListPQ | `meld` | O(m+n) | O((m+n)²) | Two-pointer merge is correct, but rebuilds via repeated append |
| 6 | SortedListPQ | `from_seq` | O(n log n) | O(n³) | Repeated O(n²) insert, n times |
| 7 | BalancedTreePQ | `insert` | O(log n) | O(n) | Converts to Vec (O(n)), linear scan, rebuilds tree (O(n)) |
| 8 | BalancedTreePQ | `delete_min` | O(log n) | O(n) | Converts to Vec, removes index 0, rebuilds |
| 9 | BalancedTreePQ | `meld` | O(m log(1+n/m)) | O(m+n) | Flattens both to Vec, merges sorted, rebuilds |
| 10 | BalancedTreePQ | `from_seq` | O(n log n) | O(n²) | Repeated O(n) insert |
| 11 | BinaryHeapPQ | `insert` | O(log n) | O(n log n) | O(log n) swaps, each swap rebuilds entire array O(n) |
| 12 | BinaryHeapPQ | `delete_min` | O(log n) | O(n log n) | O(n) initial rebuild + O(log n) swaps at O(n) each |
| 13 | BinaryHeapPQ | `meld` | O(m+n) | O((m+n)² log(m+n)) | Heapify: O(m+n) bubble-downs, each O((m+n)) due to swap rebuilding the array |
| 14 | BinaryHeapPQ | `from_seq` | O(n) | O(n² log n) | Heapify with O(n) per swap |
| 15 | LeftistHeapPQ | `insert` | O(log n) | O(n) | `meld` clones entire tree O(n) before O(log n) recursive meld |
| 16 | LeftistHeapPQ | `delete_min` | O(log n) | O(n) | Clones left and right subtrees O(n) before meld |
| 17 | LeftistHeapPQ | `meld` | O(log m + log n) | O(m+n) | Clones both trees O(m+n) before O(log m + log n) recursive meld |
| 18 | LeftistHeapPQ | `from_seq` | O(n) | O(n log n) | Pairwise reduction with O(n) total cloning per level, O(log n) levels |

**Corrections from previous review:**
- Row 2: Previous review marked UnsortedListPQ `delete_min` as "agrees with APAS" at O(n). The implementation uses a quadratic rebuild loop, making it O(n²).
- Rows 3–5: Previous review did not identify the quadratic rebuild pattern in SortedListPQ's `insert`, `delete_min`, and `meld`. These are O(n²), O(n²), and O((m+n)²) respectively, worse than the previous O(n), O(n), O(m+n) assessments.
- Row 18: Previous review assessed LeftistHeapPQ `from_seq` as O(n²). The implementation uses pairwise reduction (not left-fold), making it O(n log n) — better than O(n²) but still worse than APAS's O(n) due to tree cloning.

### Phase 3b: Implementation Fidelity

| # | Prose Item | Implementation | Fidelity | Notes |
|---|-----------|---------------|----------|-------|
| 1 | Data Type 45.1 (MPQ ADT) | All 5 modules | ✅ Faithful | All seven ADT operations implemented in each module |
| 2 | Algorithm 45.2 (Heapsort) | `HeapsortExample.rs` | ✅ Faithful | Iterative version of the recursive `hsort`; functionally equivalent |
| 3 | Data Structure 45.3 (Meld) | `LeftistHeapPQ::meld_nodes` | ✅ Faithful (improved) | Follows the prose algorithm structure; additionally enforces leftist property via `make_node` (rank-based child swapping), making it a proper leftist heap rather than the naïve version |
| 4 | Binary heap index formulas | `BinaryHeapPQ::{left_child, right_child, parent}` | ✅ Faithful | `left(i)=2i+1`, `right(i)=2i+2`, `parent(i)=(i-1)/2` — matches prose's `⌈i/2⌉−1` |
| 5 | Binary heap insert | `BinaryHeapPQ::insert` | ✅ Faithful | Adds to end, bubbles up — matches prose description |
| 6 | Binary heap deleteMin | `BinaryHeapPQ::delete_min` | ✅ Faithful | Replaces root with last element, bubbles down — matches prose description |
| 7 | Unsorted List PQ | `UnsortedListPQ.rs` | ⚠️ Deviation | Uses persistent array instead of linked list — changes insert from O(1) to O(n) |
| 8 | Sorted List PQ | `SortedListPQ.rs` | ⚠️ Deviation | Uses persistent array — delete_min becomes O(n²) instead of O(1) |
| 9 | Balanced Trees PQ | `BalancedTreePQ.rs` | ❌ Major deviation | All operations flatten to Vec and rebuild — destroys O(log n) advantage of balanced trees |
| 10 | `fromSeq` (prose: parallel reduce) | `LeftistHeapPQ::from_seq` | ⚠️ Deviation | Uses sequential pairwise reduction, not parallel reduce. Algorithmically similar but not parallel. |
| 11 | Leftist heap `meld` uses `<=` | `LeftistHeapNode::meld_nodes` | ⚠️ Minor | Prose uses `<`; implementation uses `<=`. Changes tie-breaking behavior but does not affect correctness. |
| 12 | Separate traits per implementation | All 5 modules | ⚠️ Design deviation | Prose defines a single ADT (Data Type 45.1); code defines separate traits (`UnsortedListPQTrait`, `SortedListPQTrait`, etc.) rather than a unified `MeldablePriorityQueue` trait. |

### Phase 3c: Spec Fidelity

**N/A** — No Verus specifications exist in any Chap45 file. All functions are unspecified plain Rust. Zero spec strength entries.

## Phase 4: Parallelism Review

**No Mt (multi-threaded) modules exist in Chapter 45.** All implementations are single-threaded (`St`-style).

The prose specifically describes parallel `fromSeq` via `Seq.reduce Q.meld Q.empty (Seq.map Q.singleton S)` for leftist heaps, which would have O(n) work and O(log² n) span. The implementation's `from_seq` uses a sequential pairwise reduction (while loop with `chunks(2)`), which has the right algorithmic structure for parallelism but executes sequentially.

| # | Function | APAS Work | APAS Span | Actual Work | Parallel? | Notes |
|---|----------|-----------|-----------|-------------|-----------|-------|
| 1 | LeftistHeapPQ::from_seq | O(n) | O(log² n) | O(n log n) | No | Sequential pairwise reduction; cloning inflates work |
| 2 | HeapsortExample (all variants) | varies | varies | varies | No | All five heapsort implementations are sequential loops |

## Phase 5: Runtime Test Review

### 5a: Coverage Check

| # | Source Module | RTT File | Test Count | Status |
|---|-------------|----------|:----------:|--------|
| 1 | UnsortedListPQ | `TestUnsortedListPQ.rs` | 29 | ✅ Exists |
| 2 | SortedListPQ | `TestSortedListPQ.rs` | 38 | ✅ Exists |
| 3 | BalancedTreePQ | `TestBalancedTreePQ.rs` | 43 | ✅ Exists |
| 4 | BinaryHeapPQ | `TestBinaryHeapPQ.rs` | 33 | ✅ Exists |
| 5 | LeftistHeapPQ | `TestLeftistHeapPQ.rs` | 34 | ✅ Exists |
| 6 | HeapsortExample | `TestHeapsortExample.rs` | 25 | ✅ Exists |
| 7 | Example45_2 | `TestExample45_2.rs` | 8 | ✅ Exists |

**Total: 210 test functions** across 7 test files. Coverage is excellent.

### 5b: Test Quality

Tests cover:
- ✅ Happy path (insert, find_min, delete_min, meld, from_seq)
- ✅ Edge cases (empty, singleton, all-same elements)
- ✅ Duplicate elements
- ✅ Negative numbers and mixed positive/negative
- ✅ String elements (non-numeric types)
- ✅ Large datasets (100 elements)
- ✅ Persistent semantics (original unchanged after operations)
- ✅ Macro creation (`*PQLit!`)
- ✅ Display/Debug formatting
- ✅ Structural invariants (heap property via `is_valid_heap`, leftist property via `is_valid_leftist_heap`, sorted order via `is_sorted`)
- ✅ Cross-implementation correctness (all five heapsorts produce identical sorted output)
- ✅ Sequential full extraction (delete all elements, verify sorted order)
- ✅ Additional operations: `find_max`, `delete_max`, `contains`, `remove`, `range`, `split`, `join`, `filter`, `map` (BalancedTreePQ)
- ✅ Multi-way merge and parallel heap construction demos (LeftistHeapPQ)

### 5c: Missing Tests

| # | Gap | Severity | Notes |
|---|-----|----------|-------|
| 1 | No stability tests | Low | Heapsort stability (equal-key ordering preservation) is not tested, but APAS does not claim stability |
| 2 | No negative `from_seq` tests | Low | `from_seq` tested only with positive integers for some PQ types |
| 3 | No cross-type `meld` of different PQ types | N/A | Not possible since each PQ type has its own trait |

Overall test quality is strong. No significant gaps for a non-verified chapter.

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed or present.** Chapter 45 has no `verus!` blocks, no iterators with ghost state, and no verified loops. PTTs are not applicable.

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | `decreaseKey` operation | Not implemented | Prose mentions it as "sometimes useful"; not part of Data Type 45.1 |
| 2 | Parallel `fromSeq` | Partially implemented | Pairwise structure present in LeftistHeapPQ but executed sequentially |
| 3 | Leftist heap rank/balance proof | Not formally proved | `is_valid_leftist_heap` runtime check exists but no formal proof |
| 4 | Heap property proof | Not formally proved | `is_valid_heap` / `is_heap` runtime checks exist but no formal proof |
| 5 | Cost bound proofs | Not present | No cost specifications or complexity proofs for any operation |
| 6 | Example 45.1 (min-heap illustration) | Not present | The prose illustrates a min-heap example; no direct rendering in code |

### Code with No Prose Counterpart

| # | Item | Module | Purpose |
|---|------|--------|---------|
| 1 | `size()`, `is_empty()` | All 5 PQ types | Utility functions |
| 2 | `to_seq()`, `to_vec()`, `to_sorted_vec()` | All 5 PQ types | Conversion/inspection |
| 3 | `insert_all()`, `extract_all_sorted()` | All 5 PQ types | Bulk operation helpers |
| 4 | `from_vec()` | All 5 PQ types | Rust Vec interop |
| 5 | `find_max()`, `delete_max()` | SortedListPQ, BalancedTreePQ | Additional operations not in ADT |
| 6 | `contains()`, `remove()`, `range()` | BalancedTreePQ | BST-like operations |
| 7 | `split()`, `join()`, `filter()`, `map()` | BalancedTreePQ | Functional operations |
| 8 | `split()` | LeftistHeapPQ | Splits heap at a key value |
| 9 | `meld_multiple()` | LeftistHeapPQ | Melds a slice of heaps |
| 10 | `is_valid_heap()`, `is_valid_leftist_heap()` | BinaryHeapPQ, LeftistHeapPQ | Invariant checkers for testing |
| 11 | `height()`, `root_rank()`, `level_elements()` | BinaryHeapPQ, LeftistHeapPQ | Structural inspection |
| 12 | `is_sorted()` | SortedListPQ, BalancedTreePQ | Sorted-order invariant check |
| 13 | `efficient_multi_way_merge()` | LeftistHeapPQ (free fn) | Demo of meld power |
| 14 | `parallel_heap_construction()` | LeftistHeapPQ (free fn) | Demo (actually sequential) |
| 15 | `Example45_2.rs` entirely | Example45_2 | Heapsort demonstration wrapper |
| 16 | `HeapsortComparison` struct | HeapsortExample | Cross-implementation comparison infrastructure |
| 17 | `*PQLit!` macros | All 5 PQ types | Convenience macros for literal construction |
| 18 | `Default`, `Display`, `Debug` impls | All 5 PQ types | Trait impls for ergonomics |
| 19 | `complexity_analysis()`, `correctness_verification()` | HeapsortExample | Analysis/demo functions |
| 20 | `generate_test_sequences()`, `large_example()` | HeapsortExample | Test data generation |

## Phase 8: Table of Contents and In/Out Review

### TOC Headers

**No TOC headers present** in any of the 7 files. Since these are non-Verus files (outside `verus!`), the TOC standard does not strictly apply.

### In/Out Table

Not applicable in the standard sense — no `verus!` blocks exist. All trait impls are in plain Rust outside `verus!`.

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | UnsortedListPQ | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 2 | SortedListPQ | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 3 | BalancedTreePQ | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 4 | BinaryHeapPQ | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 5 | LeftistHeapPQ | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 6 | HeapsortExample | - | ✅ out | - | - | - | ✅ out | - | - | HeapsortComparison |
| 7 | Example45_2 | - | - | - | - | - | - | - | - | - |

All placements are correct for a non-Verus chapter: everything is outside `verus!` because there is no `verus!`.

### Source File Summary

| # | File | Lines | Description |
|---|------|------:|------------|
| 1 | `UnsortedListPQ.rs` | 239 | PQ using unsorted `ArraySeqStPerS` |
| 2 | `SortedListPQ.rs` | 315 | PQ using sorted `ArraySeqStPerS` |
| 3 | `BalancedTreePQ.rs` | 423 | PQ using `AVLTreeSeqStPerS` (degenerately) |
| 4 | `BinaryHeapPQ.rs` | 387 | PQ using `ArraySeqStPerS` as binary heap |
| 5 | `LeftistHeapPQ.rs` | 430 | PQ using recursive leftist heap tree |
| 6 | `HeapsortExample.rs` | 354 | Algorithm 45.2 — heapsort via all five PQ types |
| 7 | `Example45_2.rs` | 154 | Wrapper demonstrating heapsort examples |
| | **Total** | **2302** | |

## Proof Holes

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

## Overall Assessment

### Strengths

1. **Complete ADT coverage**: All five priority queue implementations from the prose are present (Unsorted List, Sorted List, Balanced Trees, Binary Heaps, Leftist Heap), each implementing the full Data Type 45.1 interface.
2. **Faithful meld algorithm**: `LeftistHeapNode::meld_nodes` precisely follows Data Structure 45.3, with the additional leftist-property enforcement (`make_node`) that makes it a proper leftist heap.
3. **Excellent test coverage**: 210 tests across 7 test files, covering happy paths, edge cases, invariant checking, persistence semantics, cross-implementation correctness, and non-numeric types.
4. **Correct binary heap formulas**: The `left_child`, `right_child`, and `parent` functions match the prose exactly.
5. **Heapsort cross-comparison**: All five heapsort variants are shown to produce identical sorted output on every test case.
6. **Pairwise reduction**: `LeftistHeapPQ::from_seq` uses the correct pairwise-meld structure (not naïve left-fold), which is ready for parallelization.

### Weaknesses

1. **No Verus verification**: The entire chapter is unverified plain Rust. No formal guarantees on correctness, invariant maintenance, or cost bounds.
2. **Systematic cost inflation from persistence**: All implementations use persistent data structures with clone costs. 18 cost disagreements identified (Phase 3a), ranging from constant-factor inflation (LeftistHeapPQ) to polynomial blowup (SortedListPQ).
3. **Quadratic rebuild pattern**: UnsortedListPQ, SortedListPQ, and BinaryHeapPQ rebuild sequences by repeatedly appending singletons, producing O(n²) cost where O(n) is possible with a single bulk construction.
4. **BalancedTreePQ is degenerate**: Converts to `Vec` for every mutating operation, losing all O(log n) tree advantages. Effectively a sorted array with O(n) rebuilds. Does not use `AVLTreeSeqStPerS` insert/delete operations.
5. **No parallelism**: Despite the prose describing parallel `fromSeq` for leftist heaps, no parallel implementations exist. No Mt variants at all.
6. **No unified trait**: Each PQ type defines its own trait instead of a single `MeldablePriorityQueue<T>` trait that all five would implement. This prevents generic programming over PQ implementations.

## Review TODOs

| # | Priority | TODO | Notes |
|---|:--------:|------|-------|
| 1 | High | Verusify LeftistHeapPQ | Most algorithmically interesting structure; heap property and leftist property are ideal verification targets |
| 2 | High | Fix BalancedTreePQ | Use actual AVL tree insert/delete/union operations instead of Vec round-tripping; current implementation defeats the purpose of balanced trees |
| 3 | High | Eliminate quadratic rebuild pattern | Replace repeated `append(result, singleton(x))` loops with bulk construction (e.g., collect to Vec then construct ArraySeqStPerS once) in UnsortedListPQ, SortedListPQ, and BinaryHeapPQ |
| 4 | Medium | Unify PQ trait | Define a single `MeldablePriorityQueue<T>` trait matching Data Type 45.1 and implement it for all five types |
| 5 | Medium | Add Mt variant for LeftistHeapPQ | Prose describes parallel `fromSeq` via reduce; natural fit for fork-join parallelism with `join()` |
| 6 | Medium | BinaryHeapPQ swap cost | Implement O(1) in-place swap (e.g., using `ArraySeqStEphS` or an ephemeral variant) to achieve the APAS O(log n) insert/deleteMin bounds |
| 7 | Low | Consider ephemeral variants | StEph implementations using `&mut` would match APAS costs more closely, especially for BinaryHeapPQ where in-place operations are critical |
| 8 | Low | Fix `parallel_heap_construction` naming | Function name implies parallelism but implementation is sequential; rename or actually parallelize |
| 9 | Low | Add `decreaseKey` | Prose mentions it as "sometimes useful"; would be needed for Dijkstra/Prim implementations |
