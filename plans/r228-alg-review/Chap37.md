# r228 Alg Analysis Review: Chap37

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

Chapter 37 (binary search trees) states one cost itself: Algorithm 37.4
`find` takes O(h(T)). The balanced-tree costs the annotations cite come from
Chapter 38 (Cost Specification 38.11, "BSTs"), and the sequence costs cited
by the `AVLTreeSeq*` files come from Chapters 20 and 22. All rows were read
from the book text.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 37.4 | find | O(h(T)) | O(h(T)) |
| 2 | CS 38.11 | empty, singleton | O(1) | O(1) |
| 3 | CS 38.11 | split, join (joinM, joinPair) | O(lg n) | O(lg n) |
| 4 | CS 38.11 | find, insert, delete | O(lg n) | O(lg n) |
| 5 | CS 38.11 | union, intersection, difference | O(m·lg(n/m+1)) | O(lg n) |
| 6 | Ch38 prose | size (stored subtree size) | O(1) | O(1) |
| 7 | CS 38.11 (splay) | find, insert, delete | O(lg n) amortized | O(lg n) amortized |
| 8 | Alg 38.9 / 38.10 | filter, reduce | not stated | not stated |
| 9 | CS 20.6 tree seq | length, singleton, isEmpty, isSingleton | 1 | 1 |
| 10 | CS 20.6 tree seq | nth | lg\|a\| | lg\|a\| |
| 11 | CS 20.6 tree seq | subseq | 1 + lg\|a\| | 1 + lg\|a\| |
| 12 | CS 20.6 tree seq | append a b | 1 + \|lg(\|a\|/\|b\|)\| | same |
| 13 | CS 22.2 stseq | nth, update | O(1) | O(1) |
| 14 | CS 22.2 stseq | fromSeq, toSeq | O(\|a\|) | O(1) |

Conventions used in the verdicts:

1. A row without an APAS line is compared with the textbook cost when the
   textbook states one for that operation (size uses row 6; the tree
   sequences use rows 9-11).
2. An APAS line that cites a specification lacking the operation (for
   example `update` on a BST node citing CS 22.2, or traversals citing
   CS 38.11) gets "no textbook cost" with a note.
3. For the plain (unbalanced) BST, O(h(T)) matches Algorithm 37.4. For the
   balanced trees, h(T) = O(lg n), so an old O(h(T)) and a new O(lg n)
   are the same bound and are not a mismatch.
4. n = number of keys; for set operations n = |t1| + |t2|; m = copied
   length in `subseq_copy`.

## 2. Reviewed functions

Trait declarations (T) and impls (I) each carry an annotation and each
received its own review line. Rows merge T and I when the two lines say the
same; "T old wrong" marks a row where only the trait line's old analysis
disagreed. "Old" is the latest Code-review line before this review.

### 2a. BSTPlainStEph.rs (28 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTPlainStEph.rs | insert/contains/find/delete_node | 37.4 | h | h | matches textbook |
| 2 | 37 | BSTPlainStEph.rs | min/max/delete_min_node | none | h | h | no textbook cost |
| 3 | 37 | BSTPlainStEph.rs | new (T,I) | 38.11: 1 | 1 | 1 | matches textbook |
| 4 | 37 | BSTPlainStEph.rs | size (T,I) | none (Ch38) | n | n | not textbook: no stored size |
| 5 | 37 | BSTPlainStEph.rs | is_empty, height (T,I) | none | 1; n | 1; n | no textbook cost |
| 6 | 37 | BSTPlainStEph.rs | insert/contains/find/delete (T,I) | 37.4 | h | h | matches textbook |
| 7 | 37 | BSTPlainStEph.rs | minimum, maximum (T,I) | none | h | h | no textbook cost |
| 8 | 37 | BSTPlainStEph.rs | iter | none | n | n·h | no cost; old wrong [1] |

### 2b. BSTPlainMtEph.rs (33 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTPlainMtEph.rs | insert/contains/find/delete_node | 37.4 | h | h | matches textbook |
| 2 | 37 | BSTPlainMtEph.rs | min/max/delete_min_node | none | h | h | no textbook cost |
| 3 | 37 | BSTPlainMtEph.rs | new (T,I) | 38.11 | 1 | 1 | matches textbook |
| 4 | 37 | BSTPlainMtEph.rs | insert (T,I) | 38.11 | n | n | not textbook [2] |
| 5 | 37 | BSTPlainMtEph.rs | delete/contains/find (T,I) | 37.4 | h | h | matches textbook |
| 6 | 37 | BSTPlainMtEph.rs | size (T,I) | none (Ch38) | n | n | not textbook: no stored size |
| 7 | 37 | BSTPlainMtEph.rs | is_empty, height (T,I) | none | 1; n | 1; n | no textbook cost |
| 8 | 37 | BSTPlainMtEph.rs | minimum, maximum (T,I) | none | h | h | no textbook cost |
| 9 | 37 | BSTPlainMtEph.rs | in_order, pre_order, iter (T,I) | none | n | n·h | no cost; old wrong [1] |

### 2c. BSTAVLStEph.rs (22 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTAVLStEph.rs | new (T,I) | 38.11 | 1 | 1 | matches textbook |
| 2 | 37 | BSTAVLStEph.rs | size (T,I) | none (Ch38) | n | n | not textbook: no stored size |
| 3 | 37 | BSTAVLStEph.rs | is_empty, height (T,I) | none | 1; n | 1; n | no textbook cost |
| 4 | 37 | BSTAVLStEph.rs | insert (T,I) | 38.11: lg n | lg n | n | not textbook; old wrong [3] |
| 5 | 37 | BSTAVLStEph.rs | contains, find (T,I) | 37.4 | lg n | lg n | matches textbook |
| 6 | 37 | BSTAVLStEph.rs | rotate_right, rotate_left (T) | 38.11: 1 | APAS only | 1 | matches textbook |
| 7 | 37 | BSTAVLStEph.rs | rebalance (T,I) | 38.11: 1 | I: 1 | n (subtree) | not textbook; I old wrong [3] |
| 8 | 37 | BSTAVLStEph.rs | insert_node (T) | 38.11: lg n | APAS only | n | not textbook [3] |
| 9 | 37 | BSTAVLStEph.rs | contains_node, find_node (T) | 38.11 | APAS only | lg n | matches textbook |
| 10 | 37 | BSTAVLStEph.rs | iter | none | n | n lg n | no cost; old wrong [1] |

### 2d. BSTAVLMtEph.rs (32 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTAVLMtEph.rs | rotate_right, rotate_left (T) | none | 1 | 1 | no textbook cost |
| 2 | 37 | BSTAVLMtEph.rs | rebalance (T) | none | 1 | n (subtree) | no cost; old wrong [3] |
| 3 | 37 | BSTAVLMtEph.rs | insert_node (T) | 38.11: lg n | lg n | n | not textbook; old wrong [3] |
| 4 | 37 | BSTAVLMtEph.rs | contains_node, find_node (T) | 37.4 | lg n | lg n | matches textbook |
| 5 | 37 | BSTAVLMtEph.rs | min_node, max_node (T) | none | lg n | lg n | no textbook cost |
| 6 | 37 | BSTAVLMtEph.rs | new (T,I) | 38.11 | 1 | 1 | matches textbook |
| 7 | 37 | BSTAVLMtEph.rs | insert (T,I) | 38.11 | n | n | not textbook [2][3] |
| 8 | 37 | BSTAVLMtEph.rs | contains, find (T,I) | 37.4 | lg n | lg n | matches textbook |
| 9 | 37 | BSTAVLMtEph.rs | size (T,I) | none (Ch38) | n | n | not textbook: no stored size |
| 10 | 37 | BSTAVLMtEph.rs | is_empty, height (T,I) | none | 1; n | 1; n | no textbook cost |
| 11 | 37 | BSTAVLMtEph.rs | minimum, maximum (T,I) | none | lg n | lg n | no textbook cost |
| 12 | 37 | BSTAVLMtEph.rs | in_order, pre_order, iter (T,I) | none | n | n lg n | no cost; old wrong [1] |

### 2e. BSTBBAlphaStEph.rs (52) and BSTBBAlphaMtEph.rs (53)

The node code of the two files is identical; every new line agrees with the
old Opus 5.5 line (r221-r227). Rows count T and I lines together.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTBBAlpha{St,Mt}Eph.rs | size_link | 38.11 | 1 | 1 | matches textbook |
| 2 | 37 | BSTBBAlpha{St,Mt}Eph.rs | insert/delete/find_link | 38.11 | lg n | lg n | matches textbook |
| 3 | 37 | BSTBBAlpha{St,Mt}Eph.rs | delete_min, min, max_link | none | lg n | lg n | no textbook cost |
| 4 | 37 | BSTBBAlpha{St,Mt}Eph.rs | in/pre_order_into, height_rec | none | n | n | no textbook cost |
| 5 | 37 | BSTBBAlpha{St,Mt}Eph.rs | link_size, mk_node, rotations (7) | none | 1 | 1 | no textbook cost |
| 6 | 37 | BSTBBAlpha{St,Mt}Eph.rs | new, size (T,I) | 38.11 | 1 | 1 | matches textbook |
| 7 | 37 | BSTBBAlpha{St,Mt}Eph.rs | insert/delete/contains/find | 38.11 | lg n | lg n | matches textbook |
| 8 | 37 | BSTBBAlpha{St,Mt}Eph.rs | is_empty, height, min, max | none | 1; n; lg n | same | no textbook cost |
| 9 | 37 | BSTBBAlpha{St,Mt}Eph.rs | in_order, pre_order, iter | none | n | n | no textbook cost |

### 2f. BSTRBStEph.rs (63) and BSTRBMtEph.rs (88)

The St file agrees everywhere with the old lines. The Mt file adds the r225
fork-join traversals, checked as the batch notes asked [4].

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTRB{St,Mt}Eph.rs | size_link | 38.11 | 1 | 1 | matches textbook |
| 2 | 37 | BSTRB{St,Mt}Eph.rs | insert/delete/find_link | 38.11 | lg n | lg n | matches textbook |
| 3 | 37 | BSTRB{St,Mt}Eph.rs | is_red, rotations, flips, fix_up | none | 1 | 1 | no textbook cost |
| 4 | 37 | BSTRB{St,Mt}Eph.rs | delete helpers (5 free fns) | none | lg n | lg n | no textbook cost |
| 5 | 37 | BSTRB{St,Mt}Eph.rs | new_node, update, toggle, move_red | 22.2 miscite | 1 | 1 | no textbook cost [5] |
| 6 | 37 | BSTRB{St,Mt}Eph.rs | new, size (T,I) | 38.11 | 1 | 1 | matches textbook |
| 7 | 37 | BSTRBStEph.rs | insert (T,I) | 38.11 | lg n | lg n | matches textbook |
| 8 | 37 | BSTRBMtEph.rs | insert (T) | 38.11 | n | lg n | matches; old wrong [6] |
| 9 | 37 | BSTRB{St,Mt}Eph.rs | delete/contains/find (T,I) | 38.11 | lg n | lg n | matches textbook |
| 10 | 37 | BSTRB{St,Mt}Eph.rs | is_empty, height, min, max | none | 1; n; lg n | same | no textbook cost |
| 11 | 37 | BSTRBStEph.rs | in_order, pre_order, iter | none | n | n | no textbook cost |
| 12 | 37 | BSTRBMtEph.rs | in/pre_order_collect, clone_link | none | n | n | no textbook cost |
| 13 | 37 | BSTRBMtEph.rs | in/pre_order_parallel, filter_par | none | n lg n | n lg n | no cost; see [4] |
| 14 | 37 | BSTRBMtEph.rs | in/pre/filter_owned | none | n lg n | n lg n | no cost; span tight [4] |
| 15 | 37 | BSTRBMtEph.rs | reduce_owned | none | n, lg n | n, lg n | no textbook cost |
| 16 | 37 | BSTRBMtEph.rs | reduce_parallel, reduce (T,I) | none | n, n | n, n | no cost; see [4] |
| 17 | 37 | BSTRBMtEph.rs | in_order, pre_order, filter, iter | none | n lg n | n lg n | no cost; see [4] |
| 18 | 37 | BSTRBMtEph.rs | from_sorted_slice, build_* | none | n | n | no cost: sequential build |
| 19 | 37 | BSTRBMtEph.rs | compute_link_spec_size | none | n | n | no textbook cost |

### 2g. BSTSplayStEph.rs (37 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTSplayStEph.rs | new_node, update (T,I) | 22.2 miscite | 1 | 1 | no textbook cost [5] |
| 2 | 37 | BSTSplayStEph.rs | splay (I) | 38.11 | lg n amort | lg n amort | matches textbook |
| 3 | 37 | BSTSplayStEph.rs | new, size (T,I) | 38.11 | 1 | 1 | matches textbook |
| 4 | 37 | BSTSplayStEph.rs | insert (T,I), insert_link | 38.11 | lg n amort | lg n amort | matches textbook |
| 5 | 37 | BSTSplayStEph.rs | find, contains (T,I), find_link | 38.11 | h | h | not textbook [7] |
| 6 | 37 | BSTSplayStEph.rs | size_link, bst_insert | 38.11 / 37.4 | 1; h | 1; h | matches textbook |
| 7 | 37 | BSTSplayStEph.rs | is_empty, height(_link) | none | 1; n | 1; n | no textbook cost |
| 8 | 37 | BSTSplayStEph.rs | min, max (T,I, link) | none | h | h | no textbook cost |
| 9 | 37 | BSTSplayStEph.rs | in/pre_order(_collect), iter | 38.11 miscite | n | n | no textbook cost |

### 2h. BSTSplayMtEph.rs (66 lines)

Node trait (T) and node impl (I) rows carry identical lines.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTSplayMtEph.rs | new_node, update | 22.2 miscite | 1 | 1 | no textbook cost [5] |
| 2 | 37 | BSTSplayMtEph.rs | size_link | 38.11 | 1 | 1 | matches textbook |
| 3 | 37 | BSTSplayMtEph.rs | splay | none | h | h | no cost; lg n amortized |
| 4 | 37 | BSTSplayMtEph.rs | bst_insert, min_link, max_link | none | h | h | no textbook cost |
| 5 | 37 | BSTSplayMtEph.rs | insert_link | 38.11 | lg n amort | lg n amort | matches textbook |
| 6 | 37 | BSTSplayMtEph.rs | find_link | 38.11 | h | h | not textbook [7] |
| 7 | 37 | BSTSplayMtEph.rs | in/pre_order_collect, clone_link | none | n | n | no textbook cost |
| 8 | 37 | BSTSplayMtEph.rs | in_order_parallel | none | n, n | n, n | no cost: sequential [8] |
| 9 | 37 | BSTSplayMtEph.rs | pre_order_parallel | none | n, lg n | n, n | no cost; old wrong [8] |
| 10 | 37 | BSTSplayMtEph.rs | build_balanced, height_rec | none | n | n | no textbook cost |
| 11 | 37 | BSTSplayMtEph.rs | filter_parallel | none | n lg n, lg² n | n·h | no cost; old wrong [8] |
| 12 | 37 | BSTSplayMtEph.rs | reduce_parallel | none | n, lg n | n, n | no cost; old wrong [8] |
| 13 | 37 | BSTSplayMtEph.rs | compute_link_spec_size | none | n | 1 | no cost; old wrong [9] |
| 14 | 37 | BSTSplayMtEph.rs | new, size (T,I) | 38.11 | 1 | 1 | matches textbook |
| 15 | 37 | BSTSplayMtEph.rs | insert (T,I) | 38.11 | n | lg n amort | matches; old wrong [9] |
| 16 | 37 | BSTSplayMtEph.rs | find, contains (T,I) | 37.4 | h | h | not textbook [7] |
| 17 | 37 | BSTSplayMtEph.rs | from_sorted_slice, height | none | n | n | no textbook cost |
| 18 | 37 | BSTSplayMtEph.rs | is_empty, min, max (T,I) | none | 1; h | 1; h | no textbook cost |
| 19 | 37 | BSTSplayMtEph.rs | in/pre_order, filter, reduce | none | n; n·h | n; n·h | no textbook cost |

### 2i. BSTSet*MtEph.rs (set wrappers over the five trees)

All five set files share one structure: `union`, `intersection`, and
`difference` take a minimum as pivot, `split` and `join_*` flatten the trees
through `in_order` and rebuild by sorted inserts, and `delete` (except in
the red-black file) rebuilds the whole tree [10]. Costs therefore depend on
the underlying tree's `insert` and `in_order`.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTSet*MtEph.rs (5) | empty, singleton (T,I) | 38.11 | 1 | 1 | matches textbook |
| 2 | 37 | BSTSetPlainMtEph.rs | size | none (Ch38) | n | n | not textbook: no stored size |
| 3 | 37 | BSTSetAVLMtEph.rs | size | none (Ch38) | n | n | not textbook: no stored size |
| 4 | 37 | BSTSet{BBAlpha,RB}MtEph.rs | size | none (Ch38) | n | 1 | matches; old wrong |
| 5 | 37 | BSTSetSplayMtEph.rs | size | none (Ch38) | 1 | 1 | matches textbook |
| 6 | 37 | BSTSetPlainMtEph.rs | find, contains | 37.4 | h | h | matches textbook |
| 7 | 37 | BSTSet{AVL,BBAlpha,RB}MtEph.rs | find, contains | 37.4 | h / lg n | lg n | matches textbook |
| 8 | 37 | BSTSetSplayMtEph.rs | find, contains | 37.4 | h | h | not textbook [7] |
| 9 | 37 | BSTSetPlainMtEph.rs | insert | none | h | n | not textbook; old wrong [2] |
| 10 | 37 | BSTSetAVLMtEph.rs | insert | none | lg n | n | not textbook; old wrong [3] |
| 11 | 37 | BSTSet{BBAlpha,RB}MtEph.rs | insert | none | h | lg n | matches textbook |
| 12 | 37 | BSTSetSplayMtEph.rs | insert | none | h | lg n amort | matches textbook |
| 13 | 37 | BSTSet{Plain,AVL}MtEph.rs | delete | none | h / lg n | n² | not textbook; old wrong [10] |
| 14 | 37 | BSTSetBBAlphaMtEph.rs | delete | none | h | n lg n | not textbook; old wrong [10] |
| 15 | 37 | BSTSetRBMtEph.rs | delete | 37.4 | lg n (r227) | lg n | matches textbook |
| 16 | 37 | BSTSetSplayMtEph.rs | delete | none | h | n | not textbook; old wrong [10] |
| 17 | 37 | BSTSet{Plain,AVL}MtEph.rs | union/intersection/diff | none | n² | n³ | not textbook; old wrong [10] |
| 18 | 37 | BSTSet{BBAlpha,RB}MtEph.rs | union/intersection/diff | none | n² | n² lg n | not textbook; old wrong [10] |
| 19 | 37 | BSTSetSplayMtEph.rs | union/intersection/diff | none | n² | n² | not textbook [10] |
| 20 | 37 | BSTSet{Plain,AVL}MtEph.rs | split, join_pair, join_m | none | n² | n² | not textbook [10] |
| 21 | 37 | BSTSet{BBAlpha,RB}MtEph.rs | split, join_pair, join_m | none | n² | n lg n | not textbook; old wrong |
| 22 | 37 | BSTSetSplayMtEph.rs | split, join_pair, join_m | none | n² | n | not textbook; old wrong [11] |
| 23 | 37 | BSTSet{Plain,AVL}MtEph.rs | filter | none | n² | n² | no textbook cost |
| 24 | 37 | BSTSet{BBAlpha,RB}MtEph.rs | filter | none | n² | n lg n | no cost; old wrong |
| 25 | 37 | BSTSetSplayMtEph.rs | filter | none | n² | n | no cost; old wrong [11] |
| 26 | 37 | BSTSet*MtEph.rs (5) | reduce, iter_in_order | none | n; 1 | cost of in_order | no cost; old wrong [12] |
| 27 | 37 | BSTSet*MtEph.rs (5) | minimum, maximum, is_empty, as_tree | none | h / 1 | h or lg n / 1 | no textbook cost |
| 28 | 37 | BSTSet*MtEph.rs (5) | copy_set, values_vec, rebuild | none | n | n to n² | no cost; mostly old wrong [12] |

### 2j. AVLTreeSeq.rs, AVLTreeSeqStEph.rs, AVLTreeSeqStPer.rs, AVLTreeSeqMtPer.rs

These are AVL-tree-backed sequences. Nodes cache size and height, so
rotations and `rebalance` are O(1) and insertion at an index is O(lg n).

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 37 | AVLTreeSeq*.rs (4) | rotations, rebalance, size/height fns | none | 1 | 1 | no textbook cost |
| 2 | 37 | AVLTreeSeq*.rs (4) | insert_at_link, nth_*, set_* | none | lg n | lg n | no textbook cost |
| 3 | 37 | AVLTreeSeq*.rs (4) | push_inorder / inorder_collect | none | n | n | no textbook cost |
| 4 | 37 | AVLTreeSeq.rs | compare_trees (T,I) | none | n | n lg n | no cost; old wrong [13] |
| 5 | 37 | AVLTreeSeq{StEph,StPer,MtPer}.rs | compare_trees | none | n lg n | n lg n | no textbook cost |
| 6 | 37 | AVLTreeSeq*.rs (4) | length, singleton, isEmpty, isSingleton | none (20.6) | 1 | 1 | matches textbook |
| 7 | 37 | AVLTreeSeq{,StEph,StPer}.rs | nth (T,I) | 22.2: 1 | lg n | lg n | not textbook [14] |
| 8 | 37 | AVLTreeSeqMtPer.rs | nth (T,I) | 38.11: lg n | lg n | lg n | matches textbook [14] |
| 9 | 37 | AVLTreeSeq{,StEph}.rs | update (T,I) | 22.2: 1 | lg n | lg n | not textbook [14] |
| 10 | 37 | AVLTreeSeq*.rs (4) | set (T,I) | none | lg n | lg n | no textbook cost |
| 11 | 37 | AVLTreeSeq{,StEph,StPer}.rs | subseq_copy (T,I) | none (20.6) | n or n lg n | m lg n | not textbook [15] |
| 12 | 37 | AVLTreeSeqMtPer.rs | subseq_copy (T,I) | none (20.6) | n lg n, n lg n | m lg n, m + lg n | not textbook; old wrong [15] |
| 13 | 37 | AVLTreeSeq{,StEph}.rs | from_vec (T,I) | none | T: n; I: n lg n | n lg n | no cost; T old wrong |
| 14 | 37 | AVLTreeSeq{StPer,MtPer}.rs | from_vec, build_balanced | none | n lg n | n | no cost; old wrong [16] |
| 15 | 37 | AVLTreeSeq.rs | contains/delete_value (T,I) | none | T: n; I: n lg n | n lg n | no cost; T old wrong [13] |
| 16 | 37 | AVLTreeSeqStEph.rs | contains/delete_value, to_arrayseq | none | n lg n (T to_arr: n) | n lg n | no cost; T old wrong |
| 17 | 37 | AVLTreeSeqStPer.rs | values_in_order, to_arrayseq | none | n | n lg n | no cost; old wrong [13] |
| 18 | 37 | AVLTreeSeq{,MtPer}.rs | values_in_order, to_arrayseq | none | n | n | no textbook cost |
| 19 | 37 | AVLTreeSeq*.rs | empty, new, new_root, push_back, etc. | none | 1 / lg n | 1 / lg n | no textbook cost |
| 20 | 37 | AVLTreeSeq{StEph,StPer}.rs | push_left_iter(_stper), mk | none | lg n; 1 | lg n; 1 | no textbook cost |

### Footnotes

1. Chap23 `BalBinTree::in_order` and `pre_order` build each subtree's
   vector and then `Vec::append` it into the parent's, copying every key
   once per ancestor: Work and Span O(n·h(T)). For the plain BST h(T)
   reaches n (O(n²)); for AVL it is O(n lg n). The old lines said O(n).
2. `BSTPlainMtEph::insert` and `BSTAVLMtEph::insert` check capacity by
   recounting `size()` and `height()`, both O(n) recursive walks, before
   inserting. The set wrappers forward to these inserts.
3. The AVL trees store no height: `rebalance` calls `BalBinTree::height`
   on children and grandchildren, an O(subtree size) walk. Along an
   insertion path the subtree sizes sum to O(n), so `insert_node` and
   `insert` are O(n), not the CS 38.11 O(lg n).
4. The r225 fork-join traversals in `BSTRBMtEph.rs` (`in_order_owned`,
   `pre_order_owned`, `filter_owned`, and the `*_parallel` link fns) each
   fork the two subtrees and then `Vec::append` the results, O(size) per
   node: W(n) = W(a) + W(b) + O(n) and S(n) = max(S(a), S(b)) + O(n) with
   height O(lg n), giving Work and Span O(n lg n). The stated O(n lg n) for
   both is correct, and the span bound is tight: a red-black tree can keep
   Θ(n) keys under each of Θ(lg n) nodes on one path. The public
   `in_order`/`pre_order` first clone the tree sequentially (O(n)), then
   run the fork-join traversal, so they cost more than the sequential
   `in_order_collect` (Work O(n), Span O(n)). `reduce` likewise loses the
   O(lg n) span of Alg 38.10 to the sequential `clone_link`.
5. The APAS line on `update` (and on `new_node`) cites CS 22.2, which is
   the single-threaded array sequence; these functions recompute a cached
   size, O(1), and have no textbook row.
6. `BSTRBMtEph` trait `insert` said O(n); the capacity check reads the
   cached root size in O(1), and the red-black insert is O(lg n).
7. The splay trees' `find`, `contains`, and `find_link` walk the tree
   without splaying, so the amortized bound of CS 38.11 does not apply and
   a single call costs O(h(T)), which reaches n (for example after sorted
   inserts, which leave a path).
8. `BSTSplayMtEph` `*_parallel` functions are sequential recursions despite
   their names. `filter_parallel` also appends the right result at each
   node, O(n·h(T)).
9. `compute_link_spec_size` reads the cached root size, O(1); the old line
   said O(n), and the trait/impl `insert` lines inherited that O(n).
10. Set operations in all five `BSTSet*MtEph.rs` files pick a minimum as
    pivot, so each recursion level removes one key from one side (depth up
    to |t1|), and every level runs `split` and `join_*`, which flatten
    through `in_order` and rebuild by sorted inserts. With per-level cost
    C(n): Plain and AVL C = O(n²) (O(n) inserts), BB[α] and red-black
    C = O(n lg n), splay C = O(n) (see [11]); total Work = Span = O(n·C).
    The ParaPair fork does not reduce span because the right branch keeps
    all but one key. `delete` rebuilds the tree the same way, except in
    `BSTSetRBMtEph.rs`, which since r227 calls the red-black `delete`.
11. Inserting keys in increasing order into a splay tree costs O(1) each:
    the new maximum lands at depth 1 and one zig makes it the root. Every
    rebuild in `BSTSetSplayMtEph.rs` inserts sorted keys, so rebuilds are
    O(n); the resulting path makes `minimum` O(n), which is why set
    operations are O(n²). `rebuild_from_vec`/`build_from_vec` take an
    arbitrary vector, so their lines give O(n lg n) amortized and note
    that callers pass sorted input.
12. `reduce` and `iter_in_order` cost what `in_order` costs (O(n·h(T))
    plain, O(n lg n) AVL and red-black Mt, O(n) BB[α] and splay); the old
    lines said O(n) and O(1). `copy_set` and the rebuild helpers cost one
    `in_order` plus n inserts.
13. `compare_trees`, `contains_value`, `delete_value`, and StPer
    `values_in_order` loop over indices calling `nth`, O(lg n) each, so
    they are O(n lg n) even where an O(n) in-order walk exists.
14. `nth` and `update` descend the tree, O(lg n). The APAS lines in three
    files cite CS 22.2 (stseq, O(1)), so they do not match; the textbook's
    tree-sequence cost (CS 20.6 nth lg|a|) agrees with the code. The
    MtPer line cites CS 38.11 with O(lg n), which the code meets.
15. `subseq_copy` copies m = e − s elements by m `nth` calls and rebuilds,
    Work O(m lg n); CS 20.6 subseq is O(lg n). The MtPer version spawns one
    thread per element from a sequential loop and joins them in a
    sequential loop, then builds sequentially: Span O(m + lg n).
16. StPer and MtPer `from_vec` call `build_balanced_from_slice`, which
    makes one node per element on O(1) subslices and recurses
    sequentially: Work and Span O(n), not O(n lg n).

## 3. Counts (per annotation site)

905 new lines were added, one per annotated function site (trait and impl
counted separately), in 19 files. `BSTSpecsAndLemmas.rs` has no
annotations. Every diff line in `src/Chap37/` is an added
`/// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26)` line.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 228 |
| 2 | does not match textbook | 127 |
| 3 | does not match old analysis | 154 |
| 4 | no textbook cost | 550 |
| 5 | unannotated functions | 161 |
| 6 | malformed annotations | 4 |

Rows 1, 2, and 4 partition the 905 lines; row 3 overlaps them.

Per file:

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTPlainStEph.rs | 28 | 14 | 2 | 1 | 12 |
| 2 | 37 | BSTPlainMtEph.rs | 33 | 12 | 4 | 6 | 17 |
| 3 | 37 | BSTAVLStEph.rs | 22 | 10 | 7 | 4 | 5 |
| 4 | 37 | BSTAVLMtEph.rs | 32 | 8 | 5 | 8 | 19 |
| 5 | 37 | BSTBBAlphaStEph.rs | 52 | 20 | 0 | 0 | 32 |
| 6 | 37 | BSTBBAlphaMtEph.rs | 53 | 20 | 0 | 0 | 33 |
| 7 | 37 | BSTRBStEph.rs | 63 | 20 | 0 | 0 | 43 |
| 8 | 37 | BSTRBMtEph.rs | 88 | 20 | 0 | 1 | 68 |
| 9 | 37 | BSTSplayStEph.rs | 37 | 10 | 5 | 0 | 22 |
| 10 | 37 | BSTSplayMtEph.rs | 66 | 10 | 6 | 10 | 50 |
| 11 | 37 | BSTSetPlainMtEph.rs | 45 | 8 | 18 | 18 | 19 |
| 12 | 37 | BSTSetAVLMtEph.rs | 45 | 8 | 18 | 19 | 19 |
| 13 | 37 | BSTSetBBAlphaMtEph.rs | 45 | 12 | 14 | 24 | 19 |
| 14 | 37 | BSTSetRBMtEph.rs | 45 | 14 | 12 | 25 | 19 |
| 15 | 37 | BSTSetSplayMtEph.rs | 48 | 8 | 18 | 16 | 22 |
| 16 | 37 | AVLTreeSeq.rs | 60 | 8 | 6 | 7 | 46 |
| 17 | 37 | AVLTreeSeqStEph.rs | 57 | 8 | 6 | 3 | 43 |
| 18 | 37 | AVLTreeSeqStPer.rs | 45 | 8 | 4 | 7 | 33 |
| 19 | 37 | AVLTreeSeqMtPer.rs | 41 | 10 | 2 | 5 | 29 |

## 4. Unannotated functions (161)

Commented-out code (the r212 `IntoIterator` impls) is not counted.

| # | Chap | File | Count | Functions |
|---|---|---|---|---|
| 1 | 37 | BSTPlainStEph.rs | 9 | node impls (7) [a]; fmt ×2 |
| 2 | 37 | BSTPlainMtEph.rs | 11 | node impls (7) [a]; fmt ×4 |
| 3 | 37 | BSTAVLStEph.rs | 11 | min/max_node (T); node impls (7) [b] |
| 4 | 37 | BSTAVLMtEph.rs | 12 | node impls (8) [c]; fmt ×4 |
| 5 | 37 | BSTBBAlphaStEph.rs | 3 | fmt ×3 |
| 6 | 37 | BSTBBAlphaMtEph.rs | 5 | fmt ×5 |
| 7 | 37 | BSTRBStEph.rs | 6 | fmt ×6 |
| 8 | 37 | BSTRBMtEph.rs | 8 | default; fmt ×7 |
| 9 | 37 | BSTSplayStEph.rs | 17 | link trait decls (10) [d]; other 7 [e] |
| 10 | 37 | BSTSplayMtEph.rs | 7 | clone; default; fmt ×5 |
| 11 | 37 | BSTSetPlainMtEph.rs | 4 | iter (T,I); fmt ×2 |
| 12 | 37 | BSTSetAVLMtEph.rs | 4 | iter (T,I); fmt ×2 |
| 13 | 37 | BSTSetBBAlphaMtEph.rs | 4 | iter (T,I); fmt ×2 |
| 14 | 37 | BSTSetRBMtEph.rs | 4 | iter (T,I); fmt ×2 |
| 15 | 37 | BSTSetSplayMtEph.rs | 4 | iter (T,I); fmt ×2 |
| 16 | 37 | AVLTreeSeq.rs | 13 | 2 iter, next, 2 clone, default, eq [f] |
| 17 | 37 | AVLTreeSeqStEph.rs | 13 | 2 iter, next, 2 clone, default, eq [f] |
| 18 | 37 | AVLTreeSeqStPer.rs | 12 | iter ×2, next, clone, default, eq [f] |
| 19 | 37 | AVLTreeSeqMtPer.rs | 14 | iter ×2, rotate_right [g], 5 more [h] |

- [a] Node-trait impls `insert_node`, `contains_node`, `find_node`,
  `min_node`, `max_node`, `delete_min_node`, `delete_node`; their trait
  declarations are annotated and reviewed.
- [b] Node impls `rotate_right`, `rotate_left`, `insert_node`,
  `contains_node`, `find_node`, `min_node`, `max_node`; plus fmt ×2.
- [c] Node impls `rotate_right`, `rotate_left`, `rebalance`,
  `insert_node`, `contains_node`, `find_node`, `min_node`, `max_node`.
- [d] `BSTSplayLinkFns` / node trait declarations `splay`, `size_link`,
  `height_link`, `bst_insert`, `insert_link`, `find_link`, `min_link`,
  `max_link`, `in_order_collect`, `pre_order_collect`; their impls are
  annotated and reviewed.
- [e] Node `clone`, tree `clone`, `default`, fmt ×4.
- [f] Plus fmt ×6.
- [g] The annotation meant for it sits on the impl line (section 5,
  row 4).
- [h] `Iterator::next`, `into_iter`, `default`, `eq`, `clone`, and fmt ×6
  (the 11 functions not named in the row).

## 5. Malformed annotations (4)

| # | Chap | File | Line | Function | Problem |
|---|---|---|---|---|---|
| 1 | 37 | BSTSplayStEph.rs | 1553 | new (I) | APAS line glued after `// Veracity:` |
| 2 | 37 | BSTSplayStEph.rs | 1574 | insert (I) | APAS line glued after `// Veracity:` |
| 3 | 37 | AVLTreeSeqStEph.rs | 154 | set (I) | Review line glued after `// Veracity:` |
| 4 | 37 | AVLTreeSeqMtPer.rs | 437 | (impl block) | Annotation sits on an `impl` line |

Rows 1-3 were left as they are. Rows 1 and 2 have a well-formed Code-review
line below the glued line, and row 3's function had no other line; each got
its review line directly above the `fn`. Row 4 annotates the
`AVLTreeSeqMtPerNodeFns` impl block rather than a function (apparently
meant for `rotate_right`, whose `#[verifier::rlimit(80)]` follows); it was
left as it is and given no line.

## 6. Notable findings

1. AVL insertion is O(n), not O(lg n). `BSTAVLStEph` and `BSTAVLMtEph`
   store no heights, so `rebalance` recomputes subtree heights by recursive
   walks, and the Mt `insert` also recounts `size()` and `height()`. Every
   caller inherits this, including `BSTSetAVLMtEph`, whose rebuilds become
   O(n²). The `AVLTreeSeq*` files cache height and do not have this defect.
2. The five `BSTSet*MtEph.rs` set operations are far from CS 38.11:
   pivoting on a minimum peels one key per recursion level, and `split`/
   `join` flatten and rebuild the tree, so union, intersection, and
   difference cost O(n³) (plain, AVL), O(n² lg n) (BB[α], red-black), or
   O(n²) (splay), with equal span; `delete` rebuilds the tree in four of
   the five files. The old lines understated these (O(n²), O(h(T))).
3. Parallel traversals are not faster than sequential ones. The r225
   red-black fork-join traversals do cost Work and Span O(n lg n), as
   stated, but the per-node `Vec::append` and the initial sequential clone
   make them costlier than the O(n) sequential walk; Chap23's
   `in_order`/`pre_order` append costs O(n·h(T)) for the plain and AVL
   trees; and the splay Mt `*_parallel` functions are sequential. Separately,
   splay `find` never splays, so it has no amortized bound.
