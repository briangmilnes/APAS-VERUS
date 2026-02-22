# Proposed Fixes Table

Based on the `veracity-review-proof-holes` report and the PBOGH (Prove Big Or Go Home) principle, the true verification holes (excluding all `accept()` and `assume(false)` threading/cloning workarounds) have been isolated. 

Per your prioritization, target files are ordered strictly by **StPer** -> **StEph** -> **MtEph / MtPer**.

| # | Priority | Category | Chapter | File | Issue | Description |
|---|----------|----------|---------|------|-------|-------------|
| 1 | critical | **StPer** | Chap43 | `OrderedSetStPer.rs` | 11 `external_body` | Core ordered set logic (`first`, `last`, `split`, `get_range`, `rank`, `select`) is entirely unverified. |
| 2 | critical | **StPer** | Chap43 | `OrderedTableStPer.rs` | `external_body` | Companion to OrderedSet, core persistent table mapping logic is unverified. |
| 3 | critical | **StPer** | Chap43 | `AugOrderedTableStPer.rs` | `external_body` | Core augmented persistent table logic is unverified. |
| 4 | critical | **StPer** | Chap37 | `AVLTreeSeqStPer.rs` | 11 `external_body` | Deep structural operations (`rebalance`, `rotate_right`, `rotate_left`, `mk`, `build_balanced`) are unverified. |
| 5 | critical | **StPer** | Chap41 | `AVLTreeSetStPer.rs` | Bogus view / `external_body` | `spec_set_view` is an external_body, meaning the mathematical specification of the persistent set is faked. |
| 6 | critical | **StEph** | Chap43 | `OrderedSetStEph.rs` | `external_body` | Ephemeral ordered set core operations unverified. (Equivalent to the StPer holes). |
| 7 | critical | **StEph** | Chap43 | `OrderedTableStEph.rs` | `external_body` | Ephemeral ordered table core operations unverified. |
| 8 | critical | **StEph** | Chap43 | `AugOrderedTableStEph.rs` | `external_body` | Ephemeral augmented table core operations unverified. |
| 9 | critical | **StEph** | Chap39 | `BSTTreapStEph.rs` | 5 `external_body` | Ephemeral treap structural logic (`insert_link`, `find_link`) is unverified. |
| 10 | critical | **StEph** | Chap40 | `BSTKeyValueStEph.rs` | 7 `external_body` | Ephemeral BST Key/Value logic (`insert_link`, `delete`, `keys`, `values`) is unverified. |
| 11 | critical | **StEph** | Chap40 | `BSTReducedStEph.rs` | 11 `external_body` | Ephemeral BST Reduced logic (`update_node`, `range_reduce_link`) is unverified. |
| 12 | critical | **StEph** | Chap40 | `BSTSizeStEph.rs` | 7 `external_body` | Ephemeral BST Size logic (`rank_link`, `split_rank`) is unverified. |
| 13 | critical | **StEph** | Chap37 | `AVLTreeSeqStEph.rs` | 11 `external_body` | Ephemeral AVL rotations (`rebalance`, `rotate_left`) unverified. |
| 14 | critical | **StEph** | Chap26 | `ETSPStEph.rs` | 2 `external_body` | Euclidean TSP algorithmic core (`sort_and_split`, `find_best_swap`) is unverified. |
| 15 | critical | **MtPer** | Chap35 | `OrderStatSelectMtPer.rs` | 1 `external_body` | `parallel_three_way_partition` is unverified. |
| 16 | critical | **MtEph** | Chap39 | `BSTSetTreapMtEph.rs` | 14 `external_body` | Concurrent treap set structures (`split`, `join_pair`, `filter`, `reduce`) are unverified. |
| 17 | critical | **MtEph** | Chap41 | `AVLTreeSetMtEph.rs` | 14 `external_body` | Concurrent AVL set logic entirely faked (`intersection`, `union`, `difference`). |
| 18 | critical | **MtEph** | Chap26 | `ETSPMtEph.rs` | 2 `external_body` | Concurrent Euclidean TSP algorithmic core is unverified. |

*(Note: 58 `info: accept()` holes for threading/clones have been completely filtered with priority NONE per your request, as well as RwLock workaround warnings).*

## Summary

| Severity | Count |
|----------|-------|
| critical | ~362 `external_body` and Unverified Assumes (Sorted by StPer > StEph > MtEph above) |
