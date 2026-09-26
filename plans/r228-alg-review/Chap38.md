# r228 Alg Analysis Review: Chap38

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap38.txt` gives Data Type 38.1 (parametric BST: `size`, `expose`,
`joinMid`), Algorithms 38.2-38.10, and Cost Specification 38.11. The prose
requires `size` in O(1) by caching the subtree size, and states that `expose`
exposes the root. It states no separate cost for `joinMid`; CS 38.11 lists
`join`. Filter, reduce, and in-order have no entry in CS 38.11; the files'
APAS lines for them are taken as the textbook cost.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | CS 38.11 | empty, singleton | 1 | 1 |
| 2 | CS 38.11 | split t k | lg \|t\| | lg \|t\| |
| 3 | CS 38.11 | join t1 t2 | lg(\|t1\|+\|t2\|) | lg(\|t1\|+\|t2\|) |
| 4 | CS 38.11 | find, insert, delete | lg \|t\| | lg \|t\| |
| 5 | CS 38.11 | intersect, difference, union | m lg(n/m) | lg n |
| 6 | DT 38.1 prose | size | 1 | 1 |
| 7 | Alg 38.4 | joinPair (minKey + split + joinM) | lg(\|t1\|+\|t2\|) | same |
| 8 | Alg 38.6-38.8 | union/intersect/difference, parallel D&C | CS 38.11 | lg n (lg m lg n as analyzed) |
| 9 | Alg 38.9-38.10 | filter, reduce (file APAS lines) | Σ W(f) | lg \|t\| + max S(f) |

## 2. Reviewed functions

Notation: n = |t| (for set operations n = |t1| + |t2|), h = h(T), the tree
height. T and I mark the trait declaration and the impl; each received its own
line. W = Work, S = Span.

Every function below rests on one fact (footnote [1]): `expose` in both files
calls `node.left.clone()` and `node.right.clone()`, and `ParamBST::clone` is
a recursive `expose` + `join_mid` copy, so `expose` costs Θ(n), not O(1). And
`join_mid` wraps a node without rebalancing (footnote [2]), so h is not
bounded by O(lg n): `insert` puts the new key at the root, and sorted inserts
build a path of height n. A search-path operation therefore costs the sum of
the subtree sizes along the path: O(n h), which is O(n) when subtree sizes
halve and O(n²) on a path-shaped tree.

### 2a. BSTParaStEph.rs

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 38 | BSTParaStEph.rs | new_param_bst | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 38 | BSTParaStEph.rs | reveal_param_bst_backings | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 38 | BSTParaStEph.rs | clone_elem | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 4 | 38 | BSTParaStEph.rs | filter_inner | none | W n lg n, S same | W n h²+Σ, S same | no cost; old wrong [3] |
| 5 | 38 | BSTParaStEph.rs | reduce_inner | none | W n, S n | W n h+Σ, S same | no cost; old wrong [1] |
| 6 | 38 | BSTParaStEph.rs | new (T,I) | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 7 | 38 | BSTParaStEph.rs | singleton (T,I) | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 8 | 38 | BSTParaStEph.rs | expose (T,I) | 38.11: 1,1 | W 1, S 1 | W n, S n | not textbook; old wrong [1] |
| 9 | 38 | BSTParaStEph.rs | join_mid (T,I) | 38.11 join | W 1, S 1 | W 1, S 1 | not textbook: no rebalance [2] |
| 10 | 38 | BSTParaStEph.rs | join_m (T,I) | 38.11 join | W 1, S 1 | W 1, S 1 | not textbook: no rebalance [2] |
| 11 | 38 | BSTParaStEph.rs | size, is_empty (T,I) | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 12 | 38 | BSTParaStEph.rs | insert, delete (T,I) | 38.11: lg,lg | W lg, S lg | W n h, S n h | not textbook; old wrong [1] |
| 13 | 38 | BSTParaStEph.rs | find, split (T,I) | 38.11: lg,lg | W lg, S lg | W n h, S n h | not textbook; old wrong [1] |
| 14 | 38 | BSTParaStEph.rs | min_key, max_key (T,I) | 38.11: lg,lg | W lg, S lg | W n h, S n h | not textbook; old wrong [1] |
| 15 | 38 | BSTParaStEph.rs | join_pair (T,I) | 38.11 join | T lg; I n | W t1+t2·h, S same | not textbook; old wrong [4] |
| 16 | 38 | BSTParaStEph.rs | union (T,I) | 38.11 | T m lg(n/m); I n lg n | W n h², S n h² | not textbook; old wrong [5] |
| 17 | 38 | BSTParaStEph.rs | intersect (T,I) | 38.11 | T m lg(n/m); I n lg n | W n h², S n h² | not textbook; old wrong [5] |
| 18 | 38 | BSTParaStEph.rs | difference (T,I) | 38.11 | T m lg(n/m); I n lg n | W n h², S n h² | not textbook; old wrong [5] |
| 19 | 38 | BSTParaStEph.rs | filter (T,I) | Σ, lg+maxS | T Σ, n+maxS; I n lg n | W n h²+Σ, S same | not textbook; old wrong [3] |
| 20 | 38 | BSTParaStEph.rs | reduce (T,I) | Σ, lg+maxS | T Σ, n+maxS; I n | W n h+Σ, S same | not textbook; old wrong [1] |
| 21 | 38 | BSTParaStEph.rs | collect_in_order (T,I) | none | W n, S n | W n h, S n h | no cost; old wrong [1] |
| 22 | 38 | BSTParaStEph.rs | in_order (T,I) | \|t\|, \|t\| | W n, S n | W n h, S n h | not textbook; old wrong [1] |
| 23 | 38 | BSTParaStEph.rs | iter | none | W n, S n | W n h, S n h | no cost; old wrong [1] |

### 2b. BSTParaMtEph.rs

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 38 | BSTParaMtEph.rs | new_param_bst, clone_elem | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 38 | BSTParaMtEph.rs | new_leaf | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 38 | BSTParaMtEph.rs | new, singleton (T,I) | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 4 | 38 | BSTParaMtEph.rs | expose (T,I) | 38.11: 1,1 | W 1, S 1 | W n, S n | not textbook; old wrong [1] |
| 5 | 38 | BSTParaMtEph.rs | expose_internal | none | W 1, S 1 | W n, S n | not textbook; old wrong [1] |
| 6 | 38 | BSTParaMtEph.rs | join_mid, join_m (T,I) | 38.11 join | W 1, S 1 | W 1, S 1 | not textbook: no rebalance [2] |
| 7 | 38 | BSTParaMtEph.rs | size, is_empty (T,I) | 38.11: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 8 | 38 | BSTParaMtEph.rs | insert, delete (T,I) | 38.11: lg,lg | W lg, S lg | W n h, S n h | not textbook; old wrong [1] |
| 9 | 38 | BSTParaMtEph.rs | find, split (T,I) | 38.11: lg,lg | W lg, S lg | W n h, S n h | not textbook; old wrong [1] |
| 10 | 38 | BSTParaMtEph.rs | split_inner, find_recursive | none (38.3) | W lg, S lg | W n h, S n h | not textbook; old wrong [1] |
| 11 | 38 | BSTParaMtEph.rs | min_key (T,I), min_key_inner | 38.11: lg | W lg, S lg | W n h, S n h | not textbook; old wrong [1] |
| 12 | 38 | BSTParaMtEph.rs | join_pair (T,I) | Alg 38.4 | W lg, S lg | W t1+t2·h, S same | not textbook; old wrong [4] |
| 13 | 38 | BSTParaMtEph.rs | join_pair_inner (T,I) | 38.11 join | T lg; I n | W t1+t2·h, S same | not textbook; old wrong [4] |
| 14 | 38 | BSTParaMtEph.rs | union (T,I), union_inner | 38.11 | m lg(n/m) or n lg n; S lg² | W n h², S n h² | not textbook; old wrong [5] |
| 15 | 38 | BSTParaMtEph.rs | intersect (T), intersect_inner | 38.11 | m lg(n/m) or n lg n; S lg² | W n h², S n h² | not textbook; old wrong [5] |
| 16 | 38 | BSTParaMtEph.rs | difference (T,I), difference_inner | 38.11 | m lg(n/m) or n lg n; S lg² | W n h², S n h² | not textbook; old wrong [5] |
| 17 | 38 | BSTParaMtEph.rs | filter (T,I) | Σ, lg+maxS | T Σ, n+maxS; I n lg n | W n h²+Σ, S same | not textbook; old wrong [6] |
| 18 | 38 | BSTParaMtEph.rs | filter_inner, filter_parallel | none (38.9) | W n lg n, S same | W n h²+Σ, S same | not textbook; old wrong [6] |
| 19 | 38 | BSTParaMtEph.rs | reduce (T,I) | Σ, lg+maxS | T Σ, lg+maxS; I n, lg n | W n h+Σ, S n h+h·maxS | not textbook; old wrong [7] |
| 20 | 38 | BSTParaMtEph.rs | reduce_inner, reduce_parallel | none (38.10) | W n, S lg n | W n h+Σ, S n h+h·maxS | not textbook; old wrong [7] |
| 21 | 38 | BSTParaMtEph.rs | collect_in_order (T,I), _inner | none | W n, S n | W n h, S n h | no cost; old wrong [1] |
| 22 | 38 | BSTParaMtEph.rs | in_order (T,I) | \|t\|, \|t\| | W n, S n | W n h, S n h | not textbook; old wrong [1] |
| 23 | 38 | BSTParaMtEph.rs | Clone for ParamBST (impl) | none | W n, S n | W n, S n | no textbook cost |

"none (38.x)" means the function has no APAS line but implements the named
textbook algorithm; its verdict compares against that algorithm's cost.

### Footnotes

1. `expose` (St) and `expose_internal` (Mt) call `node.left.clone()` and
   `node.right.clone()`. `ParamBST::clone` is `expose` followed by
   `join_mid`, so it recursively copies every node: C(n) = C(n_l) + C(n_r) +
   O(1) = O(n), and `expose` = C(n_l) + C(n_r) = Θ(n). Every operation that
   exposes at each node of a search path pays the sum of the subtree sizes on
   that path, O(n h); every full traversal (`reduce`, `collect_in_order`)
   pays the sum of all subtree sizes, which is also O(n h). The Mt file's own
   `Clone` annotation says "Work O(n) — clones entire tree", which
   contradicts its O(1) `expose` lines.
2. `join_mid` builds `NodeInner { key, size, left, right }` directly; there is
   no priority, color, or rotation. It is O(1), but h is unbounded: the
   parametric trees in this chapter are unbalanced BSTs. CS 38.11 assumes a
   balancing `join`.
3. St `filter_inner` exposes at every node (O(n h) total) and, for each
   rejected key, calls `join_pair`, which costs O(|t1| + |t2| h) (footnote
   4); summed over the recursion this is O(n h²). The recursion is
   sequential (two plain calls), so Span equals Work.
4. `join_pair` / `join_pair_inner` are not Algorithm 38.4 (minKey + split +
   joinM). They descend the left spine of t2 (one Θ(size) `expose` per spine
   node) and clone t1 in full at the bottom, then rebuild with O(1)
   `join_mid`. Cost O(|t1| + |t2| h(t2)); the resulting tree is also taller
   (t1 hangs below t2's left spine).
5. Per recursion node of t1: an O(|a_i|) `expose` of a, an O(|b_i|) `expose`
   of b whose result is discarded (St only; Mt tests `b.is_empty()`), and
   an O(|b_i| h) `split` of b.
   Summed over h(t1) levels, Work O(n h²). In St the recursion is sequential;
   in Mt the two recursive calls run under `ParaPair!`, but the per-node
   expose and split are sequential, so the critical path is still
   O(n h²) in the worst case (O(n h) when subtree sizes halve). The old lines
   claimed CS 38.11 bounds or O(n lg n) work with O(lg² n) span.
6. `filter_parallel` wraps an `Arc` around the predicate and calls
   `filter_inner`, which recurses with two plain sequential calls. The name
   is misleading; there is no fork.
7. `reduce_inner` does fork with `ParaPair!`, but each node first pays a
   Θ(size) `expose_internal`; the critical path sums subtree sizes along one
   root-to-leaf path, O(n h), plus h applications of `op`.

## 3. Counts (per annotation site)

105 new lines were added: 48 in `BSTParaStEph.rs` and 57 in
`BSTParaMtEph.rs`. `BSTParaSpecsAndLemmas.rs` has no annotations.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 16 |
| 2 | does not match textbook | 74 |
| 3 | does not match old analysis | 74 |
| 4 | no textbook cost | 15 |
| 5 | unannotated functions | 19 |
| 6 | malformed annotations | 1 |

Rows 1, 2, and 4 partition the 105 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 38 | BSTParaStEph.rs | 48 | 8 | 32 | 33 | 8 |
| 2 | 38 | BSTParaMtEph.rs | 57 | 8 | 42 | 41 | 7 |

## 4. Unannotated functions (19)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 38 | BSTParaStEph.rs | clone (Exposed, NodeInner, ParamBST) |
| 2 | 38 | BSTParaStEph.rs | fmt x6 (Inv, Exposed, NodeInner, ParamBST) |
| 3 | 38 | BSTParaMtEph.rs | clone (Exposed, NodeInner) |
| 4 | 38 | BSTParaMtEph.rs | fmt x8 (Inv, Exposed, NodeInner, ParamBST) |

`ParamBST::clone` in `BSTParaStEph.rs` is unannotated and is the O(n)
function that makes `expose` O(n); it deserves an annotation.

## 5. Malformed annotations (1)

| # | Chap | File | Line | Function | Problem |
|---|---|---|---|---|---|
| 1 | 38 | BSTParaMtEph.rs | 669 | intersect (impl) | glued onto `// Veracity: UNNEEDED` |

The line is a `//` comment, not a doc comment, so no review line was added.
Review result for the record: Work O(n h²), Span O(n h²) (footnote 5),
which does not match its old text (Work O(n lg n), Span O(lg² n)).

## 6. Notable findings

1. **`expose` is Θ(n), not O(1), in both files.** The children are cloned
   through a recursive deep copy. This single defect turns every
   logarithmic operation (find, split, insert, delete, min, max, join_pair)
   into at least linear work, and every traversal into O(n h). Every old
   line in the chapter except the Mt `Clone` line assumed O(1) `expose`.
2. **The parametric trees are unbalanced.** `join_mid` never rebalances,
   and `insert` = split + join_m puts each new key at the root, so h can
   reach n and the "O(lg |t|)" bounds hold for no input distribution.
   Combined with finding 1, find is Θ(n²) on a tree built from sorted
   inserts.
3. **`join_pair` is not Algorithm 38.4.** It hangs a full clone of t1 under
   the left spine of t2 instead of splitting out the minimum key of t2.
4. **Mt parallelism is partial.** union/intersect/difference and reduce fork
   with `ParaPair!`, but filter (`filter_parallel`) is sequential despite its
   name, and the sequential Θ(size) exposes and splits at each node dominate
   the span of the forked functions.
5. The St `union` recursion computes `other.expose()` at every node and
   discards all but its Leaf test, another Θ(|b_i|) per node.
</content>
</invoke>
