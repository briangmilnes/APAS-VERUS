# r228 Alg Analysis Review: Chap23

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications (`prompts/Chap23.txt`)

| # | Item | Cost stated |
|---|---|---|
| 1 | DT 23.1 primitive tree sequences (length, expose, join) | see CS 23.2 |
| 2 | CS 23.2 length a | W 1, S 1 |
| 3 | CS 23.2 expose a | W 1, S 1 |
| 4 | CS 23.2 join(Zero), join(One(x)) | W 1, S 1 |
| 5 | CS 23.2 join(Two(L, R)) | W = S = 1 + \|r(L) − r(R)\| |
| 6 | Alg 23.3 empty, singleton, append, nth, map, tabulate, filter, drop, update, subseq, flatten | "left as an exercise"; bounds of Section 3 (tree-sequence CS, cited by the APAS lines as Ch20 CS 20.6) |

The chapter proves no cost for the Alg 23.3 functions. The existing APAS lines
supply them (CS 20.6: nth/subseq/update O(lg |a|), append
O(|lg(|a|/|b|)|); Alg 23.3 lines: map/tabulate W O(n), S O(lg n), filter S
O(lg² n), drop O(lg² n)); this review compares against those lines.

`BalBinTreeStEph.rs` cites "APAS (Ch23 DT 23.1)" with O(1) for leaf/node/
is_leaf and O(n) for size/height/traversals. DT 23.1 defines only length,
expose, and join. The O(1) lines agree with CS 23.2 (join Zero/Two, expose).
The O(n) size line contradicts CS 23.2 length O(1). The O(n) height and
traversal lines have no source in the prose; they are compared as given.

## 2. Reviewed functions

W = Work, S = Span, n = tree size, h = tree height. T = trait site, I = impl
site.

### BalBinTreeStEph.rs (26 sites)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 23 | BalBinTreeStEph.rs | leaf (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 23 | BalBinTreeStEph.rs | node (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 23 | BalBinTreeStEph.rs | is_leaf (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 4 | 23 | BalBinTreeStEph.rs | size (T,I) | W n [1] | W n, S n | W n, S n | not tb: no stored size [1] |
| 5 | 23 | BalBinTreeStEph.rs | height (T,I) | W n, S n | W n, S n | W n, S n | matches textbook |
| 6 | 23 | BalBinTreeStEph.rs | in_order (T,I) | W n, S n | W n, S n | W n·h, S n·h | not tb; not old [2] |
| 7 | 23 | BalBinTreeStEph.rs | pre_order (T,I) | W n, S n | W n, S n | W n·h, S n·h | not tb; not old [2] |
| 8 | 23 | BalBinTreeStEph.rs | post_order (T,I) | W n, S n | W n, S n | W n·h, S n·h | not tb; not old [2] |
| 9 | 23 | BalBinTreeStEph.rs | iter_in_order | W n, S n | W n, S n | W n·h, S n·h | not tb; not old [2] |
| 10 | 23 | BalBinTreeStEph.rs | iter_pre_order | W n, S n | W n, S n | W n·h, S n·h | not tb; not old [2] |
| 11 | 23 | BalBinTreeStEph.rs | iter_post_order | W n, S n | W n, S n | W n·h, S n·h | not tb; not old [2] |
| 12 | 23 | BalBinTreeStEph.rs | lemma_in_order_pre_order_perm. | none | N/A (proof) | W 0, S 0 | no textbook cost |
| 13 | 23 | BalBinTreeStEph.rs | lemma_pre_order_post_order_perm. | none | N/A (proof) | W 0, S 0 | no textbook cost |
| 14 | 23 | BalBinTreeStEph.rs | clone_tree | none | W Θ(n), S Θ(n) | W n, S n | no textbook cost |
| 15 | 23 | BalBinTreeStEph.rs | BalBinTree::eq | none | W Θ(n), S Θ(n) | W n, S n | no textbook cost |
| 16 | 23 | BalBinTreeStEph.rs | BalBinTree::clone | none | W Θ(n), S Θ(n) | W n, S n | no textbook cost |
| 17 | 23 | BalBinTreeStEph.rs | BalBinNode::eq | none | W Θ(n), S Θ(n) | W n, S n | no textbook cost |
| 18 | 23 | BalBinTreeStEph.rs | BalBinNode::clone | none | W Θ(n), S Θ(n) | W n, S n | no textbook cost |

### PrimTreeSeqStPer.rs (40 sites)

The type is a `Vec<T>` wrapper, not a tree; `expose` and `join` split and
concatenate Vecs.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 19 | 23 | PrimTreeSeqStPer.rs | iter (inherent) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 20 | 23 | PrimTreeSeqStPer.rs | empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 21 | 23 | PrimTreeSeqStPer.rs | singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 22 | 23 | PrimTreeSeqStPer.rs | from_vec (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 23 | 23 | PrimTreeSeqStPer.rs | length (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 24 | 23 | PrimTreeSeqStPer.rs | nth (T,I) | lg a / 1 [3] | W 1, S 1 | W 1, S 1 | matches textbook [3] |
| 25 | 23 | PrimTreeSeqStPer.rs | expose (T,I) | W 1, S 1 | W a, S a | W a, S a | not tb: Vec copy |
| 26 | 23 | PrimTreeSeqStPer.rs | join (T,I) | 1+\|rL−rR\| | W L+R, S L+R | W L+R, S L+R | not tb: Vec append |
| 27 | 23 | PrimTreeSeqStPer.rs | append (T,I) | lg(a/b) | W a+b, S a+b | W a+b, S a+b | not tb: Vec copy |
| 28 | 23 | PrimTreeSeqStPer.rs | subseq (T,I) | W lg a | W len, S len | W len, S len | not tb: Vec copy |
| 29 | 23 | PrimTreeSeqStPer.rs | update (T,I) | lg a / 1 | W a, S a | W a, S a | not tb: Vec copy |
| 30 | 23 | PrimTreeSeqStPer.rs | map (T,I) | W n, S lg n | W n, S n | W n+ΣW, S n+ΣS | not tb: sequential |
| 31 | 23 | PrimTreeSeqStPer.rs | tabulate (T,I) | W n, S lg n | W n, S n | W n+ΣW, S n+ΣS | not tb: sequential |
| 32 | 23 | PrimTreeSeqStPer.rs | filter (T,I) | W n, S lg² n | W n, S n | W n+ΣW, S n+ΣS | not tb: sequential |
| 33 | 23 | PrimTreeSeqStPer.rs | drop (T) | W lg² n | W a−n, S a−n | W a−n, S a−n | not tb: Vec copy |
| 34 | 23 | PrimTreeSeqStPer.rs | flatten (T,I) | no cost [4] | W Σ, S Σ | W a+Σ, S a+Σ | no textbook cost |
| 35 | 23 | PrimTreeSeqStPer.rs | as_slice (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 36 | 23 | PrimTreeSeqStPer.rs | into_vec (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 37 | 23 | PrimTreeSeqStPer.rs | into_iter (owned, &) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 38 | 23 | PrimTreeSeqStPer.rs | PrimTreeSeqStS clone, eq | none | W Θ(n), S Θ(n) | W a, S a | no textbook cost |
| 39 | 23 | PrimTreeSeqStPer.rs | PrimTreeSeqStTree clone, eq | none | W Θ(n), S Θ(n) | W L+R, S L+R | no textbook cost |

Footnotes:

1. `size` recurses over every node because the node stores no size. CS 23.2
   specifies length as O(1) and the prose says so explicitly ("keeping track of
   the length just requires storing the size of the subtree in each node").
   The APAS line's O(n) is not from the prose.
2. The traversals build Vecs bottom-up and join them with `Vec::append`
   (`in_order`, `post_order`: right into left; `pre_order`: both children
   into a fresh Vec). Each element is copied once per ancestor on its root
   path, so Work = Span = O(n·h): O(n lg n) for a balanced tree, O(n²) for a
   path. Nothing in the type enforces balance (`spec_balbintreesteph_wf`
   relates only Leaf to size 0). The old lines state O(n). A single
   accumulator Vec passed down the recursion would give O(n).
3. `nth` has two APAS lines: CS 20.6 O(lg |a|) (tree sequences) and CS 22.2
   O(1). The Vec index is O(1), at or below both.
4. The flatten APAS line says "flatten = reduce append empty. Tree-based cost
   depends on reduce+append" and states no cost; the prose leaves it as an
   exercise.

## 3. Counts (per annotation site = lines added)

| # | Measure | Count |
|---|---|---|
| 1 | lines added | 66 |
| 2 | matches textbook | 16 |
| 3 | does not match textbook | 28 |
| 4 | does not match old analysis | 9 |
| 5 | no textbook cost | 22 |
| 6 | unannotated functions | 9 |
| 7 | malformed annotations | 0 |

Rows 2, 3, 5 partition the 66 sites; the 9 "does not match old analysis"
sites all also carry "does not match textbook".

Per file: BalBinTreeStEph 26 (8 match, 11 not tb, 7 no tb, 9 not old);
PrimTreeSeqStPer 40 (8 match, 17 not tb, 15 no tb, 0 not old).

Unannotated exec functions:

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 23 | BalBinTreeStEph.rs | 4 fmt (Debug/Display for BalBinTree, BalBinNode) |
| 2 | 23 | PrimTreeSeqStPer.rs | drop (impl; the trait site is annotated) |
| 3 | 23 | PrimTreeSeqStPer.rs | 4 fmt (Debug/Display for PrimTreeSeqStS, PrimTreeSeqStTree) |

Malformed: none. Placement note: in `BalBinTreeStEph.rs`, `clone_tree` has a
`// veracity: no_requires` line between its doc block and the `fn`; the new
line was placed inside the doc block after the last `Alg Analysis` line.

## 4. Notable findings

- Wrong old analysis: the three `BalBinTree` traversals and their iterator
  wrappers (9 sites) are O(n·h), not O(n), because every recursion level
  copies its children's Vecs with `Vec::append`. For an unbalanced tree this
  is O(n²).
- Missing representation: `BalBinTree::size` is O(n) because nodes carry no
  size field; CS 23.2 requires O(1) length.
- `PrimTreeSeqStPer` is Vec-backed, so every tree primitive (expose, join,
  append, subseq, update, drop) is linear instead of O(1)/O(lg n); map,
  tabulate, and filter are sequential loops, so none reaches the O(lg n) or
  O(lg² n) spans of Alg 23.3. The module implements the interface of Chapter
  23 but not its cost model.
- `PrimTreeSeqStPer::nth` is O(1), better than the tree bound O(lg |a|).
- All old Code-review lines in `PrimTreeSeqStPer.rs` agree with the new
  analysis.
