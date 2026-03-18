# Iterative vs Recursive Implementation Inventory

APAS presents most algorithms recursively (divide-and-conquer is the core
paradigm). This table identifies where our implementations diverge.

## Legend

- **recursive**: Self-calls or tree-node recursive traversal (matches textbook)
- **iterative**: while/for loops (may diverge from textbook)
- **parallel-recursive**: Recursive with fork-join parallelism
- **Match?**: YES = matches textbook, NO = diverges, OK = iterative is correct for this algorithm

## Mismatches (iterative where textbook is recursive)

| # | Chap | File | Function | Textbook | Ours | Notes |
|---|------|------|----------|----------|------|-------|
| 1 | 41 | AVLTreeSetStEph.rs | insert | recursive | iterative | while loop over vec, binary search for position |
| 2 | 41 | AVLTreeSetStEph.rs | delete | recursive | iterative | while loop to filter elements into result_vec |
| 3 | 41 | AVLTreeSetStEph.rs | find | recursive | iterative | while loop, linear scan (not even binary search) |
| 4 | 41 | AVLTreeSetStEph.rs | from_seq | recursive | iterative | while loop calling insert() each time |
| 5 | 41 | AVLTreeSetStEph.rs | filter | recursive | iterative | while loop over sequence indices |
| 6 | 41 | AVLTreeSetStEph.rs | intersection | recursive | iterative | while loop, calls find() for each element |
| 7 | 41 | AVLTreeSetStEph.rs | union | recursive | iterative | two consecutive while loops |
| 8 | 41 | AVLTreeSetStEph.rs | difference | recursive | iterative | while loop, calls find() to exclude |
| 9 | 41 | AVLTreeSetStPer.rs | insert | recursive | iterative | same pattern as StEph |
| 10 | 41 | AVLTreeSetStPer.rs | delete | recursive | iterative | same pattern as StEph |
| 11 | 41 | AVLTreeSetStPer.rs | find | recursive | iterative | same pattern as StEph |
| 12 | 41 | AVLTreeSetStPer.rs | filter | recursive | iterative | same pattern as StEph |
| 13 | 41 | AVLTreeSetStPer.rs | intersection | recursive | iterative | same pattern as StEph |
| 14 | 41 | AVLTreeSetStPer.rs | union | recursive | iterative | same pattern as StEph |
| 15 | 41 | AVLTreeSetStPer.rs | difference | recursive | iterative | same pattern as StEph |
| 16 | 43 | OrderedTableStEph.rs | first_key | recursive | iterative | while loop linear scan over entries |
| 17 | 43 | OrderedTableStEph.rs | last_key | recursive | iterative | while loop linear scan over entries |
| 18 | 43 | OrderedTableStEph.rs | previous_key | recursive | iterative | collect() then iterate sorted |
| 19 | 43 | OrderedTableStEph.rs | next_key | recursive | iterative | collect() then iterate sorted |
| 20 | 43 | OrderedTableStEph.rs | rank_key | recursive | iterative | collect() then count via loop |
| 21 | 43 | OrderedTableStEph.rs | select_key | recursive | iterative | collect() then index |
| 22 | 43 | OrderedSetStEph.rs | first | recursive | iterative | while loop linear scan |
| 23 | 43 | OrderedSetStEph.rs | last | recursive | iterative | while loop linear scan |
| 24 | 43 | OrderedSetStEph.rs | previous | recursive | iterative | to_seq() then scan backward |
| 25 | 43 | OrderedSetStEph.rs | next | recursive | iterative | to_seq() then scan forward |
| 26 | 43 | OrderedSetStEph.rs | rank | recursive | iterative | to_seq() then count loop |
| 27 | 43 | OrderedSetStEph.rs | select | recursive | iterative | to_seq() then index |

## Matches (recursive, matching textbook)

| # | Chap | File | Function | Style | Notes |
|---|------|------|----------|-------|-------|
| 1 | 26 | MergeSortStPer.rs | merge_sort | recursive | decreases a.spec_len() |
| 2 | 26 | MergeSortStPer.rs | merge | iterative | OK — merge is iterative in textbook too |
| 3 | 36 | QuickSortStEph.rs | quick_sort_first | recursive | partition + recurse left/right |
| 4 | 36 | QuickSortStEph.rs | quick_sort_median3 | recursive | same with median-of-three pivot |
| 5 | 36 | QuickSortStEph.rs | quick_sort_random | recursive | same with random pivot |
| 6 | 38 | BSTParaStEph.rs | expose | recursive | pattern match on tree, decreases self@.len() |
| 7 | 38 | BSTParaStEph.rs | split | recursive | recursive descent via expose() |
| 8 | 38 | BSTParaStEph.rs | join | recursive | split both, rejoin subtrees |
| 9 | 38 | BSTParaStEph.rs | union | recursive | expose, split other, recurse subtrees |
| 10 | 38 | BSTParaStEph.rs | intersect | recursive | same pattern as union |
| 11 | 38 | BSTParaStEph.rs | difference | recursive | same pattern as union |
| 12 | 38 | BSTParaStEph.rs | filter | recursive | filter_inner() recursive descent |
| 13 | 39 | BSTTreapStEph.rs | insert | recursive | insert_link() recursive descent + rotations |
| 14 | 39 | BSTTreapStEph.rs | delete | recursive | delete_link() recursive descent + rotations |
| 15 | 39 | BSTTreapStEph.rs | find | recursive | find_link() recursive descent |

## Matches (iterative, matching textbook)

| # | Chap | File | Function | Style | Notes |
|---|------|------|----------|-------|-------|
| 1 | 47 | LinProbFlatHashTableStEph.rs | lookup | iterative | linear probe loop — correct |
| 2 | 47 | LinProbFlatHashTableStEph.rs | insert | iterative | linear probe loop — correct |
| 3 | 47 | LinProbFlatHashTableStEph.rs | delete | iterative | linear probe loop — correct |
| 4 | 47 | DoubleHashFlatHashTableStEph.rs | all ops | iterative | double hash probe — correct |
| 5 | 47 | QuadProbFlatHashTableStEph.rs | all ops | iterative | quadratic probe — correct |

## Analysis

### Root Cause of Mismatches

**Chap41 AVLTreeSetStEph/StPer**: These wrap `AVLTreeSeqStEph/StPer`
(a sequence backed by an AVL tree). The wrapper treats the backing store
as a flat sequence and iterates by index. The textbook presents AVL set
operations as recursive tree traversals (descend left/right based on key
comparison). Our implementations are O(n) linear scans where the textbook
is O(log n) recursive.

**Chap43 OrderedTableStEph/OrderedSetStEph**: Same root cause. These wrap
Chap41 AVLTreeSet types. The ordering operations (first, last, previous,
next, rank, select) do linear scans or collect-then-scan instead of
exploiting the tree's sorted structure recursively.

### Impact

27 functions are iterative where the textbook is recursive. All 27 are
in Chap41 (15) and Chap43 (12). The Chap38/39 BST and Chap36 sort
implementations correctly match the textbook's recursive style.

### Recommendation

For training data purposes, we should provide recursive alternatives
for the 27 mismatched functions. Priority:

1. **Chap41 AVLTreeSetStEph find** — O(n) linear scan vs O(log n)
   recursive binary search. Most impactful mismatch.
2. **Chap41 insert/delete** — Should descend the tree recursively,
   rebalancing on the way back up.
3. **Chap43 ordering ops** — Should exploit tree structure for
   O(log n) min/max/predecessor/successor.
