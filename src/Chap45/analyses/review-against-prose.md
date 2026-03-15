# Chap45 Review Against Prose: Priority Queues

Generated: 2026-03-15

## Phase 1: Inventory

Files reviewed (Example files skipped per CLAUDE.md):

| # | Chap | File | Functions | Proof Fns | Holes |
|---|------|------|-----------|-----------|-------|
| 1 | 45 | UnsortedListPQ.rs | 17 IT | 1 | 0 |
| 2 | 45 | SortedListPQ.rs | 21 IT | 3 | 0 |
| 3 | 45 | BalancedTreePQ.rs | 28 IT | 1 | 2 |
| 4 | 45 | BinaryHeapPQ.rs | 20 IT | 2 | 2 |
| 5 | 45 | LeftistHeapPQ.rs | 27 IT | 4 | 0 |

IT = in-trait functions. Total: 113 functions, 11 proof functions (all clean), 4 holes.

## Phase 2: Prose Inventory

APAS Chapter 45 defines:

**Data Type 45.1 (Meldable Priority Queue):**
- `empty : T`
- `singleton : S -> T`
- `findMin : T -> (S | None)`
- `insert : T x S -> T`
- `deleteMin : T -> T x (S | None)`
- `meld : T x T -> T`
- `fromSeq : S seq -> T`

**Algorithm 45.2 (Heapsort):** Implemented in HeapsortExample.rs (skipped).

**Data Structure 45.3 (Leftist Heap):** Leaf | Node(key, PQ, PQ) with meld algorithm.

**APAS Cost Table:**

| # | Implementation | insert | deleteMin | meld | fromSeq |
|---|----------------|--------|-----------|------|---------|
| 1 | Unsorted List | O(1) | O(n) | O(m+n) | O(n) |
| 2 | Sorted List | O(n) | O(1) | O(m+n) | O(n log n) |
| 3 | Balanced Trees | O(log n) | O(log n) | O(m log(1+n/m)) | O(n log n) |
| 4 | Binary Heaps | O(log n) | O(log n) | O(m+n) | O(n) |
| 5 | Leftist Heap | O(log n) | O(log n) | O(log m + log n) | O(n) |

All five implementations are present in the codebase. Each implements the full ADT.

## Phase 3a: Cost Annotations

All five files now have proper two-line cost annotations (APAS + Claude-Opus-4.6).
BinaryHeapPQ.rs had none; annotations were added to all 15 impl functions.
Four other files were updated from one-line format to two-line format.

### Cost Deviations (actual vs APAS)

| # | Chap | File | Function | APAS Cost | Actual Cost | Reason |
|---|------|------|----------|-----------|-------------|--------|
| 1 | 45 | UnsortedListPQ.rs | insert | O(1) | O(n) | Persistent array append copies |
| 2 | 45 | SortedListPQ.rs | delete_min | O(1) | O(n) | subseq_copy rebuilds array |
| 3 | 45 | SortedListPQ.rs | from_seq | O(n log n) | O(n^2) | Repeated insert, each O(n) |
| 4 | 45 | BalancedTreePQ.rs | find_min | O(log n) | O(1) | Indexed access, better than APAS |
| 5 | 45 | BalancedTreePQ.rs | insert | O(log n) | O(n) | Linear scan + from_vec rebuild |
| 6 | 45 | BalancedTreePQ.rs | delete_min | O(log n) | O(n) | Clone n-1 elements + from_vec |
| 7 | 45 | BalancedTreePQ.rs | delete_max | O(log n) | O(n) | Clone n-1 elements + from_vec |
| 8 | 45 | BalancedTreePQ.rs | meld | O(m log(1+n/m)) | O(m+n) | Merge + from_vec rebuild |
| 9 | 45 | BalancedTreePQ.rs | from_seq | O(n log n) | O(n^2) | n calls to O(n) insert |
| 10 | 45 | BinaryHeapPQ.rs | insert | O(log n) | O(n) | Persistent array append copies |
| 11 | 45 | BinaryHeapPQ.rs | delete_min | O(log n) | O(n) | Rebuild array + bubble_down |
| 12 | 45 | BinaryHeapPQ.rs | extract_all_sorted | O(n log n) | O(n^2) | n x delete_min each O(n) |
| 13 | 45 | LeftistHeapPQ.rs | from_seq | O(n) | O(n log n) | Sequential insert, not reduce |

**Root cause of deviations:** Persistent backing stores (ArraySeqStPerS, AVLTreeSeqStPerS)
require O(n) copies for structural modifications. APAS assumes mutable/ephemeral arrays
or tree-pointer updates. The LeftistHeapPQ is closest to APAS costs because its tree
structure naturally supports persistent operations, except from_seq which uses sequential
insert instead of the APAS reduce-based parallel approach.

## Phase 3b: Implementation Fidelity

| # | Chap | File | Operation | APAS Algorithm | Implementation | Match? |
|---|------|------|-----------|----------------|----------------|--------|
| 1 | 45 | UnsortedListPQ.rs | findMin | Linear scan | Linear scan with TotalOrder | Yes |
| 2 | 45 | UnsortedListPQ.rs | insert | Prepend to list | Append to persistent array | Semantically |
| 3 | 45 | UnsortedListPQ.rs | deleteMin | Scan + remove | Scan + rebuild | Semantically |
| 4 | 45 | UnsortedListPQ.rs | meld | Concatenate | Append persistent arrays | Yes |
| 5 | 45 | SortedListPQ.rs | findMin | Head of list | nth(0) | Yes |
| 6 | 45 | SortedListPQ.rs | insert | Sorted insert | TotalOrder scan + rebuild | Yes |
| 7 | 45 | SortedListPQ.rs | deleteMin | Remove head | subseq_copy(1, n-1) | Semantically |
| 8 | 45 | SortedListPQ.rs | meld | Merge sorted | Merge with 3 loops | Yes |
| 9 | 45 | BalancedTreePQ.rs | All ops | BST operations | Sorted seq + linear scan | No |
| 10 | 45 | BinaryHeapPQ.rs | insert | Append + bubble up | Append + bubble_up | Yes |
| 11 | 45 | BinaryHeapPQ.rs | deleteMin | Swap root/last + bubble down | Rebuild + bubble_down | Semantically |
| 12 | 45 | BinaryHeapPQ.rs | parent/left/right | i/2-1, 2i+1, 2i+2 | Same formulas | Yes |
| 13 | 45 | LeftistHeapPQ.rs | meld | DS 45.3 algorithm | Exact match | Yes |
| 14 | 45 | LeftistHeapPQ.rs | insert | meld(singleton, Q) | singleton then meld | Yes |
| 15 | 45 | LeftistHeapPQ.rs | deleteMin | meld(L, R) | meld children of root | Yes |
| 16 | 45 | LeftistHeapPQ.rs | fromSeq | reduce meld (parallel) | Sequential insert loop | No |

**BalancedTreePQ (row 9):** Does not use AVL tree operations for PQ operations. Instead
stores elements in a sorted AVLTreeSeqStPerS and does linear scans. This is an
implementation fidelity issue -- it uses a balanced tree as backing store but does not
leverage the tree's O(log n) search/insert capabilities.

**LeftistHeapPQ fromSeq (row 16):** APAS specifies `reduce meld empty (map singleton S)` which
achieves O(n) work. The implementation uses sequential insert which is O(n log n).

## Phase 3c: Spec Fidelity

| # | Chap | File | Function | APAS Spec | Verus Spec | Strength |
|---|------|------|----------|-----------|------------|----------|
| 1 | 45 | UnsortedListPQ.rs | empty | Empty set | len==0, multiset empty | Strong |
| 2 | 45 | UnsortedListPQ.rs | singleton | Singleton set | len==1, multiset singleton | Strong |
| 3 | 45 | UnsortedListPQ.rs | findMin | Min element | forall le + option spec | Strong |
| 4 | 45 | UnsortedListPQ.rs | insert | Add element | len+1, multiset insert | Strong |
| 5 | 45 | UnsortedListPQ.rs | deleteMin | Remove min | len-1, ms, le, exists | Strong |
| 6 | 45 | UnsortedListPQ.rs | meld | Union | len sum, multiset add | Strong |
| 7 | 45 | UnsortedListPQ.rs | fromSeq | From sequence | view equality | Strong |
| 8 | 45 | SortedListPQ.rs | All 7 ADT ops | As above | As above + sorted inv | Strong |
| 9 | 45 | BalancedTreePQ.rs | findMin | Min element | view[0] (sorted) | Strong |
| 10 | 45 | BalancedTreePQ.rs | insert | Add element | len+1, ms insert, wf | Strong |
| 11 | 45 | BalancedTreePQ.rs | deleteMin | Remove min | len-1, wf | Partial |
| 12 | 45 | BalancedTreePQ.rs | meld | Union | len sum, wf | Partial |
| 13 | 45 | BinaryHeapPQ.rs | findMin | Min element | ms count + forall le | Strong |
| 14 | 45 | BinaryHeapPQ.rs | insert | Add element | len+1, ms insert | Strong |
| 15 | 45 | BinaryHeapPQ.rs | deleteMin | Remove min | len-1, ms decomposition | Strong |
| 16 | 45 | BinaryHeapPQ.rs | meld | Union | len sum, ms add | Strong |
| 17 | 45 | LeftistHeapPQ.rs | All 7 ADT ops | As above | Multiset-based, le, wf | Strong |

**BalancedTreePQ deleteMin/meld (rows 11-12):** Missing multiset postconditions.
delete_min ensures len-1 and wf but not multiset decomposition.
meld ensures len sum and wf but not multiset union.

**LeftistHeapPQ:** Strongest specs in the chapter. Uses Multiset<T> as view type, proving
multiset preservation through all operations. Fully verified meld with heap+leftist+rank
invariant preservation. find_min proves root is minimum via lemma_heap_root_is_min.

## Phase 4: Parallelism Review

No Mt (multi-threaded) modules exist for Chapter 45. This is appropriate -- APAS does not
present parallel PQ implementations except for the parallel fromSeq via reduce, which is
noted as a deviation in LeftistHeapPQ.

## Phase 5: Runtime Test Review

| # | Chap | File | Test File | Tests |
|---|------|------|-----------|-------|
| 1 | 45 | UnsortedListPQ.rs | TestUnsortedListPQ.rs | 15 |
| 2 | 45 | SortedListPQ.rs | TestSortedListPQ.rs | Present |
| 3 | 45 | BalancedTreePQ.rs | TestBalancedTreePQ.rs | Present |
| 4 | 45 | BinaryHeapPQ.rs | TestBinaryHeapPQ.rs | 20+ |
| 5 | 45 | LeftistHeapPQ.rs | TestLeftistHeapPQ.rs | 20+ |

All five algorithm files have comprehensive RTT coverage. Tests cover:
- Empty/singleton/basic operations
- insert, find_min, delete_min, meld, from_seq
- from_vec, to_vec, to_sorted_vec
- Duplicate handling, large datasets
- String elements (generic type testing)
- Persistent semantics (original unchanged after operations)
- Structural validity checks (is_valid_heap, is_valid_leftist_heap)

RTT coverage is thorough. No gaps identified.

## Phase 6: PTT Review

No PTTs exist for Chapter 45. None are needed -- there are no iterators and no complicated
callability patterns that would benefit from proof-time testing.

## Phase 7: Gap Analysis

### Proof Holes (4 total, excluding Example files)

| # | Chap | File | Hole Type | Function | Severity | Blocker |
|---|------|------|-----------|----------|----------|---------|
| 1 | 45 | BalancedTreePQ.rs | external_body | insert | High | Multiset proof through from_vec |
| 2 | 45 | BalancedTreePQ.rs | external | BalancedTreePQExtTrait impl | Low | Closure verification |
| 3 | 45 | BinaryHeapPQ.rs | external_body | find_min | Medium | Heap property invariant proof |
| 4 | 45 | BinaryHeapPQ.rs | assume | extract_all_sorted sortedness | Medium | Heap property invariant (#8) |

### Missing/Weak Specs

| # | Chap | File | Issue | Impact |
|---|------|------|-------|--------|
| 1 | 45 | BalancedTreePQ.rs | delete_min missing ms postcondition | Callers cannot reason about content |
| 2 | 45 | BalancedTreePQ.rs | meld missing ms postcondition | Callers cannot reason about union |
| 3 | 45 | BalancedTreePQ.rs | extract_all_sorted missing sorted spec | Only ensures len |
| 4 | 45 | BinaryHeapPQ.rs | find_min needs external_body removed | Strong spec exists but unproven |

### Implementation Gaps

| # | Chap | File | Gap | Notes |
|---|------|------|-----|-------|
| 1 | 45 | BalancedTreePQ.rs | Does not use tree ops | Linear scan instead of O(log n) |
| 2 | 45 | LeftistHeapPQ.rs | fromSeq not reduce-based | O(n log n) instead of O(n) |
| 3 | 45 | BinaryHeapPQ.rs | No heap invariant spec | spec_is_heap defined but unused |

### Proof Targets (priority order)

1. **BinaryHeapPQ find_min**: Remove external_body. Proof needs heap invariant maintenance
   through insert/delete_min to establish root is minimum. Currently spec_is_heap is defined
   but never established as an invariant.
2. **BinaryHeapPQ extract_all_sorted**: Replace assume with proof. Requires establishing
   heap invariant, then showing repeated deleteMin produces sorted output.
3. **BalancedTreePQ insert**: Remove external_body. Needs multiset proof through
   linear-scan-and-rebuild. Alternatively, restructure to use AVL tree insert directly.
4. **BalancedTreePQ specs**: Add multiset postconditions to delete_min, meld, extract_all_sorted.

## Phase 8: TOC Review

| # | Chap | File | TOC Present | Sections Correct | Issues |
|---|------|------|-------------|------------------|--------|
| 1 | 45 | UnsortedListPQ.rs | Yes | Yes | Section 6 missing (no spec fns) -- OK |
| 2 | 45 | SortedListPQ.rs | Yes | Yes | Section 6 missing -- OK |
| 3 | 45 | BalancedTreePQ.rs | Yes | Yes | Section 6 missing -- OK |
| 4 | 45 | BinaryHeapPQ.rs | Yes | Mostly | Sections 12/13 swapped (macro after derive) |
| 5 | 45 | LeftistHeapPQ.rs | Yes | Yes | Section 6 missing -- OK |

**BinaryHeapPQ.rs TOC issue:** Section 12 (macros) appears after section 13 (derive impls
outside verus!). Per the standard, macros should come before outside-verus derive impls.
The macro block at line 1129 comes after the Debug/Display impls at line 1107.

## Summary

| Metric | Value |
|--------|-------|
| Files reviewed | 5 |
| Total functions | 113 |
| Clean proof functions | 11 (100%) |
| Proof holes | 4 |
| Clean modules | 3 (UnsortedListPQ, SortedListPQ, LeftistHeapPQ) |
| Holed modules | 2 (BalancedTreePQ, BinaryHeapPQ) |
| Cost annotation coverage | 100% (all impl fns annotated) |
| RTT coverage | 100% (all files have tests) |
| PTT coverage | N/A (none needed) |
| Spec strength | 3 strong, 1 partial (BalancedTreePQ), 1 strong with holes (BinaryHeapPQ) |
| APAS ADT coverage | 100% (all 7 operations for all 5 implementations) |
| Implementation fidelity | Good except BalancedTreePQ (linear ops) and LeftistHeap fromSeq (sequential) |
