# r228 Alg Analysis Review: Chap45

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap45.txt` gives the priority-queue cost table (Section 45.1), the
leftist heap (Data Structure 45.6, Theorem 45.2), and heapsort
(Algorithm 45.2). All bounds are Work; the chapter states Span only for
leftist-heap `fromSeq` by reduce.

| # | Structure | insert | deleteMin | meld | fromSeq |
|---|---|---|---|---|---|
| 1 | Unsorted list | 1 | n | m+n | n |
| 2 | Sorted list | n | 1 | m+n | n lg n |
| 3 | Balanced trees | lg n | lg n | m lg(1+n/m) | n lg n |
| 4 | Binary heaps | lg n | lg n | m+n | n |
| 5 | Leftist heap | lg n | lg n | lg m + lg n | n (Span lg² n) |

Heapsort (Alg 45.2) is O(n lg n) Work with O(lg n) insert and deleteMin.
Where a file carries its own "APAS (Ch45 ref)" line for a function the table
does not cover (for example `delete_max`, `level_elements`), the verdict is
compared with that line, at both the trait and the impl site.

## 2. Callee costs

From the Chap18/19 reviews: `ArraySeqStPer::append` O(|a|+|b|) (copies both),
`subseq_copy` O(L), `from_vec` O(1), `clone` O(n). From the Chap37 review:
`AVLTreeSeqStPer::nth` O(lg n), `from_vec` O(n), `values_in_order`
O(n lg n) (n `nth` calls), `clone` O(1) (Arc root). Nothing in Chap45 is
parallel, so Span equals Work in every new line.

## 3. Reviewed functions

T = trait site, I = impl site; each row covers both unless noted.

### 3a. UnsortedListPQ.rs (30 lines)

| # | Chap | File | Function | New review | Verdict |
|---|---|---|---|---|---|
| 1 | 45 | UnsortedListPQ.rs | empty, singleton | 1 | matches textbook |
| 2 | 45 | UnsortedListPQ.rs | find_min | n | matches textbook |
| 3 | 45 | UnsortedListPQ.rs | insert | n | not textbook (1) [1] |
| 4 | 45 | UnsortedListPQ.rs | delete_min | n² | not textbook (n) [2] |
| 5 | 45 | UnsortedListPQ.rs | meld, from_seq | m+n, n | matches textbook |
| 6 | 45 | UnsortedListPQ.rs | size, is_empty | 1 | no textbook cost |
| 7 | 45 | UnsortedListPQ.rs | to_seq, from_vec, to_vec, insert_all | n (m+n) | no textbook cost |
| 8 | 45 | UnsortedListPQ.rs | extract_all_sorted, to_sorted_vec | n³ | no cost; old wrong [2] |

### 3b. SortedListPQ.rs (36 lines)

| # | Chap | File | Function | New review | Verdict |
|---|---|---|---|---|---|
| 1 | 45 | SortedListPQ.rs | empty, singleton, find_min | 1 | matches textbook |
| 2 | 45 | SortedListPQ.rs | insert | n | matches textbook |
| 3 | 45 | SortedListPQ.rs | delete_min | n | not textbook (1) [3] |
| 4 | 45 | SortedListPQ.rs | meld | (m+n)² | not textbook (m+n) [1] |
| 5 | 45 | SortedListPQ.rs | from_seq | n² | not textbook (n lg n) [4] |
| 6 | 45 | SortedListPQ.rs | insert_all | (m+n)² | no cost; old wrong [1] |
| 7 | 45 | SortedListPQ.rs | from_vec | n² | no cost; old wrong [4] |
| 8 | 45 | SortedListPQ.rs | size, is_empty, find_max | 1 | no textbook cost |
| 9 | 45 | SortedListPQ.rs | to_seq, extract_all_sorted, delete_max, to_vec, to_sorted_vec, is_sorted | n | no textbook cost |

### 3c. BalancedTreePQ.rs (52 lines)

The "balanced tree" is an `AVLTreeSeqStPer` used as a sorted sequence, not a
BST keyed by priority. Every update flattens it by `nth` and rebuilds it.

| # | Chap | File | Function | New review | Verdict |
|---|---|---|---|---|---|
| 1 | 45 | BalancedTreePQ.rs | empty, singleton | 1 | matches textbook |
| 2 | 45 | BalancedTreePQ.rs | find_min | lg n | matches textbook; old said 1 |
| 3 | 45 | BalancedTreePQ.rs | insert, delete_min, delete_max | n lg n | not textbook (lg n) [5] |
| 4 | 45 | BalancedTreePQ.rs | meld | (m+n) lg(m+n) | not textbook [5] |
| 5 | 45 | BalancedTreePQ.rs | from_seq | n² lg n | not textbook (n lg n) [5] |
| 6 | 45 | BalancedTreePQ.rs | size, is_empty | 1 | no textbook cost |
| 7 | 45 | BalancedTreePQ.rs | to_seq, extract_all_sorted | 1 | no cost; old said n [6] |
| 8 | 45 | BalancedTreePQ.rs | find_max, height | lg n | no textbook cost [7] |
| 9 | 45 | BalancedTreePQ.rs | contains, remove, range, to_vec, to_sorted_vec, is_sorted | n lg n | no cost; old said n |
| 10 | 45 | BalancedTreePQ.rs | from_vec, split, filter, map | n² lg n | no cost; old wrong [5] |
| 11 | 45 | BalancedTreePQ.rs | insert_all, join | m² lg m + (m+n) lg(m+n) | no cost; old wrong |

### 3d. BinaryHeapPQ.rs (47 lines)

The heap array is an `ArraySeqStPer`; `swap_elements` rebuilds the array by
one-element appends, so every swap is O(n²).

| # | Chap | File | Function | New review | Verdict |
|---|---|---|---|---|---|
| 1 | 45 | BinaryHeapPQ.rs | empty, singleton, find_min, size, is_empty | 1 | matches textbook |
| 2 | 45 | BinaryHeapPQ.rs | to_seq, is_valid_heap, height | n, n, lg n | matches textbook |
| 3 | 45 | BinaryHeapPQ.rs | insert | n | not textbook (lg n) [1] |
| 4 | 45 | BinaryHeapPQ.rs | delete_min | n² | not textbook (lg n) [1] |
| 5 | 45 | BinaryHeapPQ.rs | meld, insert_all | (m+n)³ | not textbook [8] |
| 6 | 45 | BinaryHeapPQ.rs | from_seq, from_vec | n³ | not textbook (n) [8] |
| 7 | 45 | BinaryHeapPQ.rs | extract_all_sorted, to_sorted_vec | n³ | not textbook (n lg n) |
| 8 | 45 | BinaryHeapPQ.rs | level_elements | level + L² | not textbook [1] |
| 9 | 45 | BinaryHeapPQ.rs | to_vec (T only) | n | no textbook cost [9] |
| 10 | 45 | BinaryHeapPQ.rs | left_child, right_child, parent | 1 | no textbook cost |
| 11 | 45 | BinaryHeapPQ.rs | swap_elements | n² | no cost; old wrong [8] |
| 12 | 45 | BinaryHeapPQ.rs | bubble_up, bubble_down | n² lg n | no cost; old wrong [8] |
| 13 | 45 | BinaryHeapPQ.rs | heapify | n³ | no cost; old wrong [8] |
| 14 | 45 | BinaryHeapPQ.rs | bubble_up_heap, bubble_down_heap | lg n | no textbook cost |
| 15 | 45 | BinaryHeapPQ.rs | is_heap, exec_pow2, exec_log2 | n, e, lg n | no textbook cost |

### 3e. LeftistHeapPQ.rs (55 lines)

| # | Chap | File | Function | New review | Verdict |
|---|---|---|---|---|---|
| 1 | 45 | LeftistHeapPQ.rs | Node meld_nodes | lg m + lg n | matches textbook (Thm 45.2) |
| 2 | 45 | LeftistHeapPQ.rs | Node rank, make_node | 1 | no textbook cost |
| 3 | 45 | LeftistHeapPQ.rs | Node size, height, is_leftist, is_heap | n | no textbook cost |
| 4 | 45 | LeftistHeapPQ.rs | Node is_rank_bounded | n·h | no cost; old said n [10] |
| 5 | 45 | LeftistHeapPQ.rs | Node to_vec | n lg n | no cost; old said n [11] |
| 6 | 45 | LeftistHeapPQ.rs | PQ empty, singleton, find_min | 1 | matches textbook |
| 7 | 45 | LeftistHeapPQ.rs | PQ insert, delete_min | n | not textbook (lg n) [12] |
| 8 | 45 | LeftistHeapPQ.rs | PQ meld | m+n | not textbook (lg m + lg n) [12] |
| 9 | 45 | LeftistHeapPQ.rs | PQ from_seq | n² | not textbook (n, lg² n) [13] |
| 10 | 45 | LeftistHeapPQ.rs | PQ size, height | n | no textbook cost |
| 11 | 45 | LeftistHeapPQ.rs | PQ is_empty, root_rank | 1 | no textbook cost |
| 12 | 45 | LeftistHeapPQ.rs | PQ extract_all_sorted, to_sorted_vec, from_vec, split | n² | no cost; old wrong [12] |
| 13 | 45 | LeftistHeapPQ.rs | PQ is_valid_leftist_heap | n·h | no cost; old said n [10] |
| 14 | 45 | LeftistHeapPQ.rs | PQ to_vec | n lg n | no cost; old said n [11] |
| 15 | 45 | LeftistHeapPQ.rs | PQ meld_multiple | k·N | no cost; old said k lg n [12] |
| 16 | 45 | LeftistHeapPQ.rs | total_order_le | 1 | no textbook cost |

### 3f. HeapsortExample.rs (5 lines)

| # | Chap | File | Function | APAS line | New review | Verdict |
|---|---|---|---|---|---|---|
| 1 | 45 | HeapsortExample.rs | heapsort_unsorted_list | n² | n³ | not textbook; old wrong |
| 2 | 45 | HeapsortExample.rs | heapsort_sorted_list | n² | n² | matches textbook |
| 3 | 45 | HeapsortExample.rs | heapsort_balanced_tree | n lg n | n² lg n | not textbook; old wrong |
| 4 | 45 | HeapsortExample.rs | heapsort_binary_heap | n lg n | n³ | not textbook; old wrong |
| 5 | 45 | HeapsortExample.rs | heapsort_leftist_heap | n lg n | n² | not textbook |

`Example45_2.rs` has no Alg Analysis annotations.

### Footnotes

1. A one-element `ArraySeqStPer::append` copies the whole array. `insert`
   calls it once (O(n)); `delete_min`, `meld`, and `level_elements` call it
   once per element inside a loop, which makes linear work quadratic.
2. UnsortedListPQ `delete_min` finds the minimum in O(n), then rebuilds the
   rest by n − 1 one-element appends: O(n²). `extract_all_sorted` calls it
   n times: O(n³). The old lines said O(n²).
3. SortedListPQ `delete_min` uses `subseq_copy`, which copies the n − 1
   remaining elements; the textbook's O(1) needs a shared tail.
4. SortedListPQ `from_seq` does n sequential sorted inserts, O(i) each:
   O(n²) against the textbook's O(n lg n) (sort first).
5. BalancedTreePQ `insert` flattens the AVL sequence with `values_in_order`
   (n `nth` calls, O(n lg n)), calls `Vec::insert`, and rebuilds with
   `from_vec`. `delete_min` and `delete_max` copy n − 1 elements by `nth`.
   `from_seq`, `from_vec`, `split`, `filter`, and `map` do one such insert
   per element: O(n² lg n).
6. `AVLTreeSeqStPer::clone` shares the Arc root, so `to_seq` and
   `extract_all_sorted` are O(1), not O(n).
7. BalancedTreePQ `height` halves n until 1: it computes floor(lg n), not
   the tree's height.
8. BinaryHeapPQ `swap_elements` rebuilds the array with n one-element
   appends (O(n²)). `bubble_up` and `bubble_down` clone the array and do up
   to lg n swaps (O(n² lg n)); `heapify` does O(n) swaps (O(n³)); `meld`,
   `from_seq`, `from_vec`, and `insert_all` go through `heapify`. The
   in-place `bubble_up_heap` and `bubble_down_heap` helpers are O(lg n) but
   only `insert` and `delete_min` use them.
9. The impl `to_vec` annotation is malformed (see Section 6), so only the
   trait site got a new line.
10. `is_rank_bounded` calls `size()` on both children at every node, so its
    work is the sum of subtree sizes, O(n·h) with h ≤ n.
11. Node `to_vec` copies the right subtree's vector at every node. Rank
    drops by exactly one on each right step and never rises, so an element
    lies in at most rank(root) ≤ lg(n + 1) right subtrees: O(n lg n).
12. `LeftistHeapNode::clone` is a deep recursive clone. PQ `meld` clones
    both roots before `meld_nodes`, and `delete_min` clones the whole root,
    so `insert`, `delete_min`, and `meld` are O(n) or O(m + n) although the
    spine meld itself is O(lg m + lg n). Every loop over them inherits the
    linear factor: `extract_all_sorted` and `split` are O(n²),
    `meld_multiple` is O(k·N).
13. `from_seq` does n sequential inserts (O(i) each with the clone), not the
    textbook's reduce of melds (Work O(n), Span O(lg² n)). The trait line
    said O(n) and the impl line O(n lg n).

## 4. Counts (per annotation site)

225 new lines were added, one per well-formed annotated site, in 6 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 46 |
| 2 | does not match textbook | 50 |
| 3 | does not match old analysis | 103 |
| 4 | no textbook cost | 129 |
| 5 | unannotated functions | 65 |
| 6 | malformed annotations | 1 |

Rows 1, 2, and 4 partition the 225 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 45 | UnsortedListPQ.rs | 30 | 10 | 4 | 6 | 16 |
| 2 | 45 | SortedListPQ.rs | 36 | 8 | 6 | 7 | 22 |
| 3 | 45 | BalancedTreePQ.rs | 52 | 6 | 10 | 40 | 36 |
| 4 | 45 | BinaryHeapPQ.rs | 47 | 16 | 18 | 21 | 13 |
| 5 | 45 | LeftistHeapPQ.rs | 55 | 5 | 8 | 26 | 42 |
| 6 | 45 | HeapsortExample.rs | 5 | 1 | 4 | 3 | 0 |
| 7 | 45 | Example45_2.rs | 0 | 0 | 0 | 0 | 0 |

## 5. Unannotated functions (65)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 45 | UnsortedListPQ.rs | default, clone, eq, fmt x2 |
| 2 | 45 | SortedListPQ.rs | default, clone, eq, fmt x2 |
| 3 | 45 | BalancedTreePQ.rs | default, clone, eq, fmt x2 |
| 4 | 45 | BinaryHeapPQ.rs | default, clone, eq, fmt x2 |
| 5 | 45 | LeftistHeapPQ.rs | clone x2, eq x2, default, efficient_multi_way_merge, parallel_heap_construction, fmt x4, format_node x2 |
| 6 | 45 | HeapsortExample.rs | clone, eq, is_vec_sorted_exec, all_results_match, all_results_sorted, compare_all_heapsorts, fmt x2, textbook_example, reverse_sorted_example, already_sorted_example, duplicates_example, single_element_example, empty_example, large_example, efficiency_demonstration, complexity_analysis, correctness_verification, vec_to_array_seq, vec_to_avl_seq, is_sorted, generate_test_sequences |
| 7 | 45 | Example45_2.rs | the 8 example_45_2_* / run_example_45_2 functions (trait, impl, and free copies), fmt x2 |

## 6. Malformed annotations (1)

| # | Chap | File | Function | Problem |
|---|---|---|---|---|
| 1 | 45 | BinaryHeapPQ.rs | to_vec (impl) | the Alg Analysis text is glued onto a `// Veracity: UNNEEDED proof block` line, so it is not a doc comment; no new line added |

## 7. Notable findings

1. **Persistent updates copy the whole structure.** In every Chap45
   implementation the per-operation cost is dominated by copying, not by the
   algorithm: one-element `ArraySeqStPer::append` inside loops
   (Unsorted/Sorted/Binary), flatten-and-rebuild of the AVL sequence
   (Balanced), and deep `LeftistHeapNode::clone` before meld (Leftist). No
   implementation meets the textbook's insert and deleteMin bounds except
   SortedListPQ `insert`.
2. **BinaryHeapPQ is cubic.** `swap_elements` is O(n²), so `bubble_up` and
   `bubble_down` are O(n² lg n) and `heapify`, `from_seq`, `meld`, and
   `extract_all_sorted` are O(n³), against the textbook's O(n), O(m + n), and
   O(n lg n). The O(lg n) in-place `bubble_*_heap` helpers exist but are not
   used by `heapify`.
3. **The leftist heap's core is right but its wrapper is not.**
   `meld_nodes` meets Theorem 45.2 (O(lg m + lg n)), but PQ `meld`,
   `insert`, and `delete_min` deep-clone their inputs first (O(m + n)), and
   `from_seq` inserts sequentially (O(n²)) instead of reducing with meld
   (O(n), Span O(lg² n)). Heapsort with it is O(n²), not O(n lg n); with the
   binary heap and unsorted list it is O(n³).
4. BalancedTreePQ stores a sorted `AVLTreeSeqStPer`, not a BST, and
   reads it by `nth`; `insert` and `delete_min` are O(n lg n) and `find_min`
   is O(lg n) (the old lines said O(1)).
5. Nothing in Chap45 is parallel.
