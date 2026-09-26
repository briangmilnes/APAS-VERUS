# r228 Alg Analysis Review: Chap39

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap39.txt` implements the Chapter 38 parametric interface with
treaps (Data Structure 39.3) and bounds the height through Algorithm 39.2
(treap-generating quicksort). It states: `priority` is O(1); `join` does
constant work per level, so O(h(T1) + h(T2)) = O(lg(|T1| + |T2|)) with high
probability; `split` does constant work per level (the rebuild joins take the
"lucky" branch because the root key outranks both subtrees), so O(lg |T|) with
high probability. All other operations take their costs from CS 38.11
(Chap38 report, section 1). The rotation-based treaps (`BSTTreapMtEph.rs`,
`BSTTreapStEph.rs`) have no algorithm in this chapter's prose; their APAS
lines cite CS 38.11 and are taken as the textbook cost.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | DS 39.3 | priority, expose | 1 | 1 |
| 2 | DS 39.3 | join(T1, (k,p), T2), joinMid | lg(\|T1\|+\|T2\|) whp | same |
| 3 | DS 39.3 prose | split T k | lg \|T\| whp | lg \|T\| whp |
| 4 | Alg 39.2 | treap height | lg n whp | — |
| 5 | CS 38.11 | empty, singleton, size | 1 | 1 |
| 6 | CS 38.11 | find, insert, delete | lg n | lg n |
| 7 | CS 38.11 | union, intersect, difference | m lg(n/m) | lg n |
| 8 | file APAS lines | filter, reduce | \|t\| | lg \|t\| |
| 9 | file APAS lines | in_order, pre_order, height | \|t\| | \|t\| |

## 2. Reviewed functions

Notation: n = |t|; for set operations m = |t1| (the tree whose keys drive the
recursion) and n = |t2|; Σ = Σ W(f) or Σ S(f). "exp" = expected. T and I mark
trait and impl lines; each got its own line.

### 2a. BSTParaTreapMtEph.rs (parametric treap under RwLock)

Its `expose_internal` deep-copies both children through `ParamTreap::clone`
(footnote [1]), the same defect as Chap38. Unlike Chap38, `join_with_priority`
keeps the treap heap order, so h = O(lg n) expected and search-path costs sum
geometrically to O(n) expected.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 39 | BSTParaTreapMtEph.rs | clone_elem, new_param_treap | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 39 | BSTParaTreapMtEph.rs | new_leaf, tree_priority_internal | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 39 | BSTParaTreapMtEph.rs | make_node | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 4 | 39 | BSTParaTreapMtEph.rs | expose_internal | none (39.3) | W 1, S 1 | W n, S n | not textbook; old wrong [1] |
| 5 | 39 | BSTParaTreapMtEph.rs | expose_with_priority_internal | none | W 1, S 1 | W n, S n | no cost; old wrong [1] |
| 6 | 39 | BSTParaTreapMtEph.rs | priority_for | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 7 | 39 | BSTParaTreapMtEph.rs | join_with_priority | 39.3: lg | W lg, S lg | W t1+t2 exp, S same | not textbook; old wrong [2] |
| 8 | 39 | BSTParaTreapMtEph.rs | split_inner | none (38.3) | W lg exp | W n exp, S n exp | not textbook; old wrong [1] |
| 9 | 39 | BSTParaTreapMtEph.rs | join_pair_inner | 39.3: lg | W lg, S lg | W n lg n exp, S same | not textbook; old wrong [3] |
| 10 | 39 | BSTParaTreapMtEph.rs | union_inner | none (38.6) | W n lg n, S lg² | W (m+n)lg m, S m+n lg m | not textbook; old wrong [4] |
| 11 | 39 | BSTParaTreapMtEph.rs | intersect/difference_inner | none (38.7-8) | W n lg n, S lg² | W m lg²m+n lg m [4] | not textbook; old wrong [4] |
| 12 | 39 | BSTParaTreapMtEph.rs | filter_inner, filter_parallel | none (38.9) | W n lg n, S lg² | W n lg²n+Σ, S same | not textbook; old wrong [5] |
| 13 | 39 | BSTParaTreapMtEph.rs | reduce_inner, reduce_parallel | none | n,n / n,lg n | W n lg n+Σ, S n+lg n·maxS | no cost; old wrong [6] |
| 14 | 39 | BSTParaTreapMtEph.rs | collect_in_order | none | W n, S n | W n lg n exp, S same | no cost; old wrong [1] |
| 15 | 39 | BSTParaTreapMtEph.rs | new (T,I) | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 16 | 39 | BSTParaTreapMtEph.rs | size (T,I) | 38.11: 1,1 | T 1; I n | W 1, S 1 | matches; I old wrong [7] |
| 17 | 39 | BSTParaTreapMtEph.rs | is_empty (T,I) | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 18 | 39 | BSTParaTreapMtEph.rs | expose (T,I) | 39.3: 1,1 | W 1, S 1 | W n, S n | not textbook; old wrong [1] |
| 19 | 39 | BSTParaTreapMtEph.rs | join_mid (T,I) | 39.3: lg | W lg | W t1+t2 exp, S same | not textbook; old wrong [2] |
| 20 | 39 | BSTParaTreapMtEph.rs | insert (T,I) | 38.11: lg | W lg | W n exp, S n exp | not textbook; old wrong [1] |
| 21 | 39 | BSTParaTreapMtEph.rs | delete (T,I) | 38.11: lg | W lg | W n lg n exp, S same | not textbook; old wrong [3] |
| 22 | 39 | BSTParaTreapMtEph.rs | find, split (T,I) | lg | W lg | W n exp, S n exp | not textbook; old wrong [1] |
| 23 | 39 | BSTParaTreapMtEph.rs | join_pair (T,I) | 39.3: lg | W lg | W n lg n exp, S same | not textbook; old wrong [3] |
| 24 | 39 | BSTParaTreapMtEph.rs | union (T,I) | m lg(n/m), lg n | T same; I n lg n, lg² | W (m+n)lg m, S m+n lg m | not textbook; old wrong [4] |
| 25 | 39 | BSTParaTreapMtEph.rs | intersect, difference (T,I) | m lg(n/m), lg n | T same; I n lg n, lg² | W m lg²m+n lg m [4] | not textbook; old wrong [4] |
| 26 | 39 | BSTParaTreapMtEph.rs | filter (T,I) | \|t\|, lg \|t\| | T same; I n lg n, lg² | W n lg²n+Σ, S same | not textbook; old wrong [5] |
| 27 | 39 | BSTParaTreapMtEph.rs | reduce (T,I) | \|t\|, lg \|t\| | T same; I n, n | W n lg n+Σ, S n+lg n·maxS | not textbook; old wrong [6] |
| 28 | 39 | BSTParaTreapMtEph.rs | in_order (T,I) | \|t\|, \|t\| | W n, S n | W n lg n exp, S same | not textbook; old wrong [1] |
| 29 | 39 | BSTParaTreapMtEph.rs | iter | none | W n, S n | W n lg n exp, S same | no cost; old wrong [1] |

### 2b. BSTSetTreapMtEph.rs (set shim over ParamTreap)

Every operation delegates to `BSTParaTreapMtEph.rs`, so each inherits the
costs in 2a.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 39 | BSTSetTreapMtEph.rs | empty (T,I) | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 39 | BSTSetTreapMtEph.rs | singleton (T,I) | lg n [8] | T lg n; I 1 | W 1, S 1 | matches; T old wrong [8] |
| 3 | 39 | BSTSetTreapMtEph.rs | size (T,I) | 38.11: 1,1 | T 1; I n | W 1, S 1 | matches; I old wrong [7] |
| 4 | 39 | BSTSetTreapMtEph.rs | is_empty, as_tree (T,I) | 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 5 | 39 | BSTSetTreapMtEph.rs | find, contains (T,I) | 38.11: lg | W lg | W n exp, S n exp | not textbook; old wrong [1] |
| 6 | 39 | BSTSetTreapMtEph.rs | minimum, maximum (T,I) | 38.11: lg | W lg | W n exp, S n exp | not textbook; old wrong [1] |
| 7 | 39 | BSTSetTreapMtEph.rs | minimum_inner, maximum_inner | none | W lg exp | W n exp, S n exp | no cost; old wrong [1] |
| 8 | 39 | BSTSetTreapMtEph.rs | insert (T,I) | 38.11: lg | W lg | W n exp, S n exp | not textbook; old wrong [1] |
| 9 | 39 | BSTSetTreapMtEph.rs | delete (T,I) | 38.11: lg | W lg | W n lg n exp, S same | not textbook; old wrong [3] |
| 10 | 39 | BSTSetTreapMtEph.rs | union (T,I) | m lg(n/m), lg n | T same; I n lg n, lg² | W (m+n)lg m, S m+n lg m | not textbook; old wrong [4] |
| 11 | 39 | BSTSetTreapMtEph.rs | intersection, difference | m lg(n/m), lg n | T same; I n lg n, lg² | W m lg²m+n lg m [4] | not textbook; old wrong [4] |
| 12 | 39 | BSTSetTreapMtEph.rs | split (T,I) | 39.3: lg | W lg | W n exp, S n exp | not textbook; old wrong [1] |
| 13 | 39 | BSTSetTreapMtEph.rs | join_pair (T,I) | 39.3: lg | W lg | W n lg n exp, S same | not textbook; old wrong [3] |
| 14 | 39 | BSTSetTreapMtEph.rs | join_m (T,I) | 39.3: lg | W lg | W t1+t2 exp, S same | not textbook; old wrong [2] |
| 15 | 39 | BSTSetTreapMtEph.rs | filter (T,I) | n, lg n | T n, lg n; I n lg n | W n lg²n+Σ, S same | not textbook; old wrong [5] |
| 16 | 39 | BSTSetTreapMtEph.rs | reduce (T,I) | n, lg n | T n, lg n; I n, n | W n lg n+Σ, S n+lg n·maxS | not textbook; old wrong [6] |
| 17 | 39 | BSTSetTreapMtEph.rs | iter_in_order (T,I) | n, n | T n; I 1 | W n lg n exp, S same | not textbook; old wrong [9] |
| 18 | 39 | BSTSetTreapMtEph.rs | iter | none | W n, S n | W n lg n exp, S same | no cost; old wrong [1] |

### 2c. BSTTreapMtEph.rs (rotation treap under one RwLock)

Plain `Box` links, no cloning on descent, rotations on the way up. All costs
assume random priorities; `insert` takes a caller-supplied `u64` priority
(footnote [10]).

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 39 | BSTTreapMtEph.rs | size_link (free, T) | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 39 | BSTTreapMtEph.rs | size_link (I) | none | W n, S n | W 1, S 1 | matches; old wrong [7] |
| 3 | 39 | BSTTreapMtEph.rs | update | none | W h(T), S h(T) | W 1, S 1 | no cost; old wrong [11] |
| 4 | 39 | BSTTreapMtEph.rs | rotate_left | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 5 | 39 | BSTTreapMtEph.rs | rotate_right | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 6 | 39 | BSTTreapMtEph.rs | insert_link, delete_link | lg exp | lg exp, n worst | W lg n exp, S same | matches textbook |
| 7 | 39 | BSTTreapMtEph.rs | find/min/max_link (free,T,I) | lg exp | lg exp, n worst | W lg n exp, S same | matches textbook |
| 8 | 39 | BSTTreapMtEph.rs | height_link (free) | none | W n, S n | W n, S n | no textbook cost |
| 9 | 39 | BSTTreapMtEph.rs | height_link (T,I), height (T,I) | n, n | W n, S n | W n, S n | matches textbook |
| 10 | 39 | BSTTreapMtEph.rs | in/pre_order_collect | n, n | W n, S n | W n, S n | matches textbook |
| 11 | 39 | BSTTreapMtEph.rs | lemma_wf_assemble_node (free,T) | 38.11: 1 [12] | W 1, S 1 | W 1, S 1 | no textbook cost |
| 12 | 39 | BSTTreapMtEph.rs | new, is_empty (T,I) | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 13 | 39 | BSTTreapMtEph.rs | size (T,I) | 1, 1 | T 1; I n | W 1, S 1 | matches; I old wrong [7] |
| 14 | 39 | BSTTreapMtEph.rs | insert, delete, find (T,I) | lg exp | lg exp | W lg n exp, S same | matches textbook |
| 15 | 39 | BSTTreapMtEph.rs | contains, min, max (T,I) | lg exp | lg exp | W lg n exp, S same | matches textbook |
| 16 | 39 | BSTTreapMtEph.rs | in_order, pre_order (T,I) | n, n | W n, S n | W n, S n | matches textbook |
| 17 | 39 | BSTTreapMtEph.rs | clone_link | none | W n, S n | W n, S n | no textbook cost |

### 2d. BSTTreapStEph.rs (rotation treap plus a parametric API)

This file has two APIs on one `Box`-linked struct. The rotation API
(`BSTTreapStEphTrait`) matches the textbook except for its traversals. The
parametric API (`ParamBSTTreapStEphTrait`) moves subtrees, so its inner
`expose_to_parts_st` is O(1), but every trait entry point first clones the
whole tree with `clone_with_view` (footnote [13]).

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 39 | BSTTreapStEph.rs | new, is_empty, height (T,I) | 38.11 | W 1 or n | same | matches textbook |
| 2 | 39 | BSTTreapStEph.rs | size (T,I) | 1, 1 | T 1; I n | W 1, S 1 | matches; I old wrong [7] |
| 3 | 39 | BSTTreapStEph.rs | insert/delete/find (T,I) | lg exp | lg exp, n worst | W lg n exp, S same | matches textbook |
| 4 | 39 | BSTTreapStEph.rs | contains/min/max (T,I) | lg exp | lg exp, n worst | W lg n exp, S same | matches textbook |
| 5 | 39 | BSTTreapStEph.rs | in_order, pre_order (T,I) | n, n | W n, S n | W n lg n exp, S same | not textbook; old wrong [14] |
| 6 | 39 | BSTTreapStEph.rs | new_node, size_link (T,I) | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 7 | 39 | BSTTreapStEph.rs | update_size (T,I) | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 8 | 39 | BSTTreapStEph.rs | rotate_left/right (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 9 | 39 | BSTTreapStEph.rs | clone_link, height_link (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 10 | 39 | BSTTreapStEph.rs | insert/delete_link (T,I) | none | lg exp, n worst | W lg n exp, S same | no textbook cost |
| 11 | 39 | BSTTreapStEph.rs | find/min/max_link (T,I) | none | lg exp, n worst | W lg n exp, S same | no textbook cost |
| 12 | 39 | BSTTreapStEph.rs | in/pre_order_vec (T,I) | none | W n, S n | W n lg n exp, S same | no cost; old wrong [14] |
| 13 | 39 | BSTTreapStEph.rs | param_new, singleton (T,I) | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 14 | 39 | BSTTreapStEph.rs | expose (T,I) | 39.3: 1,1 | W 1, S 1 | W n, S n | not textbook; old wrong [13] |
| 15 | 39 | BSTTreapStEph.rs | join_mid (T,I) | 39.3: lg | lg | W lg exp, S same | matches textbook |
| 16 | 39 | BSTTreapStEph.rs | param_size (T,I) | 1, 1 | T 1; I n | W 1, S 1 | matches; I old wrong [7] |
| 17 | 39 | BSTTreapStEph.rs | param_is_empty, in_order | 38.11 | 1 / \|t\| | same | matches textbook |
| 18 | 39 | BSTTreapStEph.rs | param_insert, delete (T,I) | lg | lg | W n, S n | not textbook; old wrong [13] |
| 19 | 39 | BSTTreapStEph.rs | param_find, split (T,I) | lg | lg | W n (exp), S same | not textbook; old wrong [13] |
| 20 | 39 | BSTTreapStEph.rs | param_join_pair (T,I) | lg | lg | W t1+t2 exp, S same | not textbook; old wrong [15] |
| 21 | 39 | BSTTreapStEph.rs | param_union (T,I) | m lg(n/m), lg n | T m lg(n/m); I n lg n | W m+n exp, S same | not textbook; old wrong [13] |
| 22 | 39 | BSTTreapStEph.rs | param_intersect/diff (T,I) | m lg(n/m), lg n | T m lg(n/m); I n lg n | W n+m lg²(m+n) | not textbook; old wrong [16] |
| 23 | 39 | BSTTreapStEph.rs | param_filter (T) | \|t\|, lg \|t\| | W \|t\| | W n lg n+Σ, S same | not textbook; old wrong [17] |
| 24 | 39 | BSTTreapStEph.rs | param_filter (I) | none | W n lg n exp | W n lg n+Σ, S same | not textbook [17] |
| 25 | 39 | BSTTreapStEph.rs | param_reduce (T,I) | \|t\|, lg \|t\| | W \|t\| or n,n | W n+Σ, S n+Σ | not textbook: St seq |
| 26 | 39 | BSTTreapStEph.rs | clone_elem_st, priority_for_st | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 27 | 39 | BSTTreapStEph.rs | make_node, tree_priority_st | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 28 | 39 | BSTTreapStEph.rs | expose_to_parts_st | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 29 | 39 | BSTTreapStEph.rs | clone_with_view | none | W 1, S 1 | W n, S n | no cost; old wrong [13] |
| 30 | 39 | BSTTreapStEph.rs | join_with_priority_st | 39.3: lg | lg | W lg exp, S same | matches textbook |
| 31 | 39 | BSTTreapStEph.rs | split_inner_st | none | lg exp, n worst | W lg n exp, S same | no textbook cost |
| 32 | 39 | BSTTreapStEph.rs | join_pair_inner_st | 39.3: lg | lg | W t2+lg²n exp, S same | not textbook; old wrong [15] |
| 33 | 39 | BSTTreapStEph.rs | union_inner_st | none (38.6) | W n lg n exp | W m lg(n/m+1)+m | not textbook; old wrong [18] |
| 34 | 39 | BSTTreapStEph.rs | intersect/diff_inner_st | none (38.7-8) | W n lg n exp | W m lg²(m+n) exp | not textbook; old wrong [16] |
| 35 | 39 | BSTTreapStEph.rs | filter_inner_st | none | W n lg n exp | W n lg n+Σ exp | no textbook cost |
| 36 | 39 | BSTTreapStEph.rs | reduce/collect_in_order_st | none | W n, S n | W n (+Σ), S same | no textbook cost |
| 37 | 39 | BSTTreapStEph.rs | iter | none | W n, S n | W n lg n exp, S same | no cost; old wrong [14] |

### Footnotes

1. `expose_internal` (ParaTreap) calls `node.left.clone()` and
   `node.right.clone()`; `ParamTreap::clone` (unannotated) recursively copies
   every node, so `expose` is Θ(size). A search path pays Σ of subtree sizes
   along the path, which is O(n) expected in a treap (subtree sizes shrink by
   a constant factor in expectation, as in quickselect); a full traversal
   pays Σ of all subtree sizes = O(n lg n) expected.
2. `join_with_priority` follows DS 39.3, but each descent step calls
   `expose_internal`, a Θ(size) copy. Sizes along a spine shrink
   geometrically in expectation, so O(|t1| + |t2|) expected instead of
   O(lg(|t1| + |t2|)). In `split_inner` the rebuild joins take the lucky
   branch (the old root's priority exceeds both parts), so they are O(1).
3. `join_pair_inner` is not Algorithm 38.4. It exposes t2, splits t1 at t2's
   root key, and recurses on both sides: a sequential union. Because every
   key of t1 is less than every key of t2, the right recursion is always
   `join_pair_inner(empty, r_right)`, which still visits every node of
   `r_right` with a Θ(size) expose. Total O(n lg n) expected. `delete` and
   `join_pair` (which also clones t1 first) inherit this.
4. Per node of a: an O(|a_i|) expose, an O(|b_i|) expected split, and a
   join whose descent pays O(size) exposes. Union: Work O((m+n) lg m), Span
   O(m + n lg m) expected (the forks run under `ParaPair!`, but the per-node
   copies are on the critical path; b's pieces need not shrink along a
   path). Intersect and difference add a sequential `join_pair_inner`
   (O(k lg k) on a result of size k) at nodes where the key is dropped:
   Work O(m lg² m + n lg m), Span O(m lg m + n lg m) expected.
5. `filter_inner` recurses with two plain calls (no `ParaPair!`);
   `filter_parallel` only wraps the predicate in an `Arc`. Each node pays a
   Θ(size) expose, and each rejected key pays an O(k lg k) `join_pair_inner`:
   O(n lg² n + Σ W(f)) expected, Span equal to Work.
6. `reduce_inner` forks with `ParaPair!`; work is Σ of subtree sizes,
   O(n lg n) expected; the critical path sums sizes along one path, O(n)
   expected, plus lg n applications of `op`.
7. `size` / `size_link` read the cached `size` field: O(1). The impl lines
   said O(n).
8. The `singleton` APAS line in `BSTSetTreapMtEph.rs` says O(lg n); CS 38.11
   gives O(1). The code is O(1) (`join_mid` of two leaves takes the lucky
   branch), matching the textbook; the old trait line copied the wrong APAS
   cost.
9. `iter_in_order` impl line said O(1); it calls `in_order`, a full
   traversal.
10. `insert(value, priority: u64)` in both rotation treaps takes the priority
    from the caller. The expected bounds hold only if callers pass random
    priorities; the `BSTTreapStEphLit!` macro uses a key hash. `BSTTreapMtEph`
    `find` also carries an `assume` in its body (not a cost issue).
11. `update` reads two cached child sizes; the old line said O(h(T)).
12. `lemma_wf_assemble_node` is a `proof fn`, erased at run time; its APAS
    line cites CS 38.11, which has no such entry.
13. `clone_with_view` calls `tree.clone()`, which is `clone_link`, a full
    O(n) copy; the old line said O(1). Every parametric trait entry point
    (`expose`, `param_insert`, `param_delete`, `param_split`,
    `param_join_pair`, the set operations, filter, reduce, in_order) clones
    first. `param_find` clones its current subtree at every level, O(n)
    expected. For `param_union` the O(m + n) clones dominate the
    O(m lg(n/m + 1)) union.
14. `in_order_vec` / `pre_order_vec` build a `Vec` per subtree and
    `append` child results into the parent's `Vec`, copying O(subtree size)
    per node: O(n h) = O(n lg n) expected, not O(n).
15. `join_pair_inner_st` has the same union-style shape as footnote 3 but
    without clones: the (empty, r_right) recursion is O(|r_right|), plus
    O(lg n) splits and joins down t2's left spine: O(|t2| + lg² n) expected.
16. Intersect/difference: sequential; each dropped key pays a
    `join_pair_inner_st` of O(|right result| + lg² n):
    O(m lg²(m + n)) expected, plus O(m + n) clones at the trait level.
17. `param_filter`: sequential; each rejected key pays a
    `join_pair_inner_st` that walks its right result: O(n lg n + Σ W(f))
    expected. The impl's old line (O(n lg n)) agrees; the trait's old line
    said O(|t|).
18. `union_inner_st` is Algorithm 38.6 without the parallel fork; work
    matches CS 38.11 when |a| ≤ |b| (it does not swap arguments, so for
    |a| > |b| it still visits all of a).

## 3. Counts (per annotation site)

246 new lines were added in four files. `BSTTreapSpecsAndLemmas.rs` has no
annotations.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 99 |
| 2 | does not match textbook | 95 |
| 3 | does not match old analysis | 114 |
| 4 | no textbook cost | 52 |
| 5 | unannotated functions | 33 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 246 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 39 | BSTParaTreapMtEph.rs | 52 | 7 | 35 | 41 | 10 |
| 2 | 39 | BSTSetTreapMtEph.rs | 43 | 10 | 30 | 35 | 3 |
| 3 | 39 | BSTTreapMtEph.rs | 49 | 43 | 0 | 3 | 6 |
| 4 | 39 | BSTTreapStEph.rs | 102 | 39 | 30 | 35 | 33 |

## 4. Unannotated functions (33)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 39 | BSTParaTreapMtEph.rs | clone (Exposed, NodeInner, ParamTreap) |
| 2 | 39 | BSTParaTreapMtEph.rs | fmt x8 |
| 3 | 39 | BSTSetTreapMtEph.rs | clone, fmt x2 |
| 4 | 39 | BSTTreapMtEph.rs | into_iter, clone (Node, tree), default |
| 5 | 39 | BSTTreapMtEph.rs | fmt x6 |
| 6 | 39 | BSTTreapStEph.rs | clone (Node, tree), default |
| 7 | 39 | BSTTreapStEph.rs | fmt x6 |

`ParamTreap::clone` (O(n), recursive) is the cost driver of the whole
parametric treap and has no annotation.

## 5. Malformed annotations (0)

None. One annotation in `BSTParaTreapMtEph.rs` (line 92) sits inside the
commented-out (BYPASSED) function `param_treap_assert_finite`; it is not a
live annotation site and received no line.

## 6. Notable findings

1. **The parametric treap's `expose` is Θ(n).** As in Chap38,
   `BSTParaTreapMtEph.rs` clones both children through a recursive
   `ParamTreap::clone`. Treap balance keeps h = O(lg n), so search-path
   operations cost O(n) expected rather than O(n h), but find, split, and
   insert are linear, not logarithmic, and `BSTSetTreapMtEph.rs` inherits
   every one of these costs.
2. **`join_pair` is a sequential union in both parametric treaps.**
   `join_pair_inner` / `join_pair_inner_st` split t1 at each root of t2 and
   recurse on both sides; with ordered inputs the right recursion visits
   every node of t2. `delete`, `join_pair`, intersect, difference, and
   filter all pay for it (O(n lg n) expected in the Mt file, O(|t2|) in the
   St file) instead of O(lg n).
3. **`BSTTreapStEph.rs` parametric API clones the whole tree on entry**
   (`clone_with_view`, whose old line said O(1)), so `param_insert`,
   `param_find`, `param_split`, and `param_union` are all Ω(n). Separately,
   the file's two APIs keep opposite heap orders on the same nodes: the
   rotation API is a min-heap on priority (`insert_link` rotates a child up
   when its priority is smaller), the parametric API a max-heap
   (`join_with_priority_st` puts the larger priority on top). Mixing the two
   on one tree breaks the treap invariant and voids the O(lg n) expected
   bounds (BST order and correctness are preserved).
4. The rotation treaps (`BSTTreapMtEph.rs`, rotation API of
   `BSTTreapStEph.rs`) match CS 38.11 for insert, delete, find, min, and
   max, given random caller-supplied priorities. The St traversals are
   O(n lg n) because of `Vec::append` in each recursive frame.
5. Five impl lines claimed O(n) for `size`/`size_link`/`param_size`, which
   read a cached field; one impl line claimed O(1) for `iter_in_order`, a
   full traversal.
</content>
</invoke>
<invoke name="Bash">
<parameter name="command">cd /home/milnes/projects/APAS-VERUS-r228 && git status --short | grep -v '^??' | head; git status --short plans/r228-alg-review/ src/Chap38 src/Chap39