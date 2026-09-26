# r228 Alg Analysis Review: Chap40

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap40.txt` (Augmenting Binary Search Trees) states costs only for
the augmentation primitives and rank/select. The treap costs the files rely
on for find/insert/delete come from Cost Specification 38.11 (Chap38) and the
treap analysis of Chap39 (expected, with random priorities).

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Ch40 §2 | size T (stored size field) | 1 | 1 |
| 2 | Ch40 §2 | makeNode (size = \|L\|+\|R\|+1) | 1 | 1 |
| 3 | Alg 40.1 | rank T k (size-augmented) | lg n | lg n |
| 4 | Alg 40.1 | select T i (size-augmented) | lg n | lg n |
| 5 | Alg 40.1 | rank/select without augmentation | n | lg n |
| 6 | Ex 40.1 | splitRank (t, i) | lg n (one path) | lg n |
| 7 | Ch40 §3 | reducedVal T | 1 | 1 |
| 8 | Ch40 §3 | makeNode with reduced values | 1 | 1 |
| 9 | CS 38.11 | empty, singleton | 1 | 1 |
| 10 | CS 38.11 | find, insert, delete, split | lg n | lg n |
| 11 | CS 38.11 | join t1 t2 | lg(\|t1\|+\|t2\|) | same |
| 12 | Ch39 | treap height | lg n w.h.p. | — |

The existing APAS lines cite "Ch40 ref" for operations the chapter does not
cost (height, keys, values, in_order, minimum, maximum, range_reduce); the
review compares against those lines where they exist, and against CS 38.11
for find/insert/delete.

## 2. Reviewed functions

All three files are sequential (St) treaps: Span = Work everywhere. Every
trait function has an annotation on the trait declaration (T) and another on
the impl (I); each got its own line. Rows merge T and I when both verdicts
agree. "exp" = expected over random priorities (O(n) worst case).

### 2a. BSTKeyValueStEph.rs (59 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 40 | BSTKeyValueStEph.rs | Node::new (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 40 | BSTKeyValueStEph.rs | new, size, is_empty (T,I) | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 40 | BSTKeyValueStEph.rs | height (T,I) | n, n | W n, S n | W n, S n | matches textbook |
| 4 | 40 | BSTKeyValueStEph.rs | insert (T,I) | lg n exp | W lg n exp | W lg n, S lg n exp | matches textbook |
| 5 | 40 | BSTKeyValueStEph.rs | delete (T) | n, n [1] | W n, S n | W lg n, S lg n exp | matches; old wrong [1] |
| 6 | 40 | BSTKeyValueStEph.rs | delete (I) | (T: n) | W lg n exp | W lg n, S lg n exp | matches textbook |
| 7 | 40 | BSTKeyValueStEph.rs | find, contains, get (T,I) | lg n exp | W lg n exp | W lg n, S lg n exp | matches textbook |
| 8 | 40 | BSTKeyValueStEph.rs | keys, values (T,I) | n, n | W n, S n | W n, S n | matches textbook |
| 9 | 40 | BSTKeyValueStEph.rs | minimum_key, maximum_key | lg n exp | W lg n exp | W lg n, S lg n exp | matches textbook |
| 10 | 40 | BSTKeyValueStEph.rs | height_link (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 11 | 40 | BSTKeyValueStEph.rs | rotate_left/right (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 12 | 40 | BSTKeyValueStEph.rs | insert/delete/find_link | none (38.11) | W lg n exp | W lg n, S lg n exp | matches textbook |
| 13 | 40 | BSTKeyValueStEph.rs | min/max_key_link (T,I) | none | W lg n exp | W lg n, S lg n exp | matches textbook |
| 14 | 40 | BSTKeyValueStEph.rs | collect_keys/values (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 15 | 40 | BSTKeyValueStEph.rs | collect_in_order_kvp [2] | none | W n, S n | W n, S n | no textbook cost |
| 16 | 40 | BSTKeyValueStEph.rs | find_min_priority_idx_kvp [2] | none | W n, S n | W n, S n | no textbook cost |
| 17 | 40 | BSTKeyValueStEph.rs | build_treap_from_vec [2] | none | W n lg n exp | W n lg n exp, same S | no textbook cost |
| 18 | 40 | BSTKeyValueStEph.rs | filter_by_key_kvp [2] | none | W n, S n | W n, S n | no textbook cost |
| 19 | 40 | BSTKeyValueStEph.rs | clone_link, compare_kv_links | none | W n, S n | W n, S n | no textbook cost |
| 20 | 40 | BSTKeyValueStEph.rs | iter | none | W n, S n | W n, S n | no textbook cost |

### 2b. BSTSizeStEph.rs (69 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 40 | BSTSizeStEph.rs | Node::new (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 40 | BSTSizeStEph.rs | new, size, is_empty (T,I) | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 40 | BSTSizeStEph.rs | height (T,I) | n, n | W n, S n | W n, S n | matches textbook |
| 4 | 40 | BSTSizeStEph.rs | insert (T,I) | lg n exp | W lg n exp | W lg n, S lg n exp | matches textbook |
| 5 | 40 | BSTSizeStEph.rs | delete (T) | n, n [1] | W n, S n | W lg n, S lg n exp | matches; old wrong [1] |
| 6 | 40 | BSTSizeStEph.rs | delete (I) | (T: n) | W lg n exp | W lg n, S lg n exp | matches textbook |
| 7 | 40 | BSTSizeStEph.rs | find, contains (T,I) | lg n exp | W lg n exp | W lg n, S lg n exp | matches textbook |
| 8 | 40 | BSTSizeStEph.rs | minimum, maximum (T,I) | lg n exp | W lg n exp | W lg n, S lg n exp | matches textbook |
| 9 | 40 | BSTSizeStEph.rs | in_order (T,I) | n, n | W n, S n | W n, S n | matches textbook |
| 10 | 40 | BSTSizeStEph.rs | rank, select (T,I) | 40.1: lg n | W lg n, S lg n | W lg n, S lg n exp | matches textbook |
| 11 | 40 | BSTSizeStEph.rs | split_rank (T) | Ex 40.1: lg n | W lg n, S lg n | W n lg n exp, same S | not textbook; old wrong [3] |
| 12 | 40 | BSTSizeStEph.rs | split_rank (I) | (T: lg n) | W n lg n exp | W n lg n exp, same S | not textbook [3] |
| 13 | 40 | BSTSizeStEph.rs | size_link (T,I) | none (§2 size) | W 1, S 1 | W 1, S 1 | matches textbook |
| 14 | 40 | BSTSizeStEph.rs | update_size, make_node | none (makeNode) | W 1, S 1 | W 1, S 1 | matches textbook |
| 15 | 40 | BSTSizeStEph.rs | rotate_left/right (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 16 | 40 | BSTSizeStEph.rs | insert/delete/find_link | none (38.11) | W lg n exp | W lg n, S lg n exp | matches textbook |
| 17 | 40 | BSTSizeStEph.rs | min_link, max_link (T,I) | none | W lg n exp | W lg n, S lg n exp | matches textbook |
| 18 | 40 | BSTSizeStEph.rs | rank_link, select_link | none (40.1) | W lg n exp | W lg n, S lg n exp | matches textbook |
| 19 | 40 | BSTSizeStEph.rs | height_link (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 20 | 40 | BSTSizeStEph.rs | in_order_collect(+_w_prio) | none | W n, S n | W n, S n | no textbook cost |
| 21 | 40 | BSTSizeStEph.rs | find_min_priority_idx | none | W n, S n | W n, S n | no textbook cost |
| 22 | 40 | BSTSizeStEph.rs | build_treap_from_vec | none | W n lg n exp | W n lg n exp, same S | no textbook cost |
| 23 | 40 | BSTSizeStEph.rs | filter_by_key [2] | none | W n, S n | W n, S n | no textbook cost |
| 24 | 40 | BSTSizeStEph.rs | compare_links, clone_link | none | W n, S n | W n, S n | no textbook cost |
| 25 | 40 | BSTSizeStEph.rs | iter | none | W n, S n | W n, S n | no textbook cost |

### 2c. BSTReducedStEph.rs (82 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 40 | BSTReducedStEph.rs | Node::new (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 40 | BSTReducedStEph.rs | SumOp identity/combine/lift | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 40 | BSTReducedStEph.rs | CountOp identity/combine/lift | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 4 | 40 | BSTReducedStEph.rs | ReduceOp trait (3 fns) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 5 | 40 | BSTReducedStEph.rs | new, size, is_empty (T,I) | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 6 | 40 | BSTReducedStEph.rs | height (T,I) | n, n | W n, S n | W n, S n | matches textbook |
| 7 | 40 | BSTReducedStEph.rs | insert, delete (T,I) | lg n exp | W lg n exp | W lg n, S lg n exp | matches textbook |
| 8 | 40 | BSTReducedStEph.rs | find, contains, get (T,I) | lg n exp | W lg n exp | W lg n, S lg n exp | matches textbook |
| 9 | 40 | BSTReducedStEph.rs | keys, values (T,I) | n, n | W n, S n | W n, S n | matches textbook |
| 10 | 40 | BSTReducedStEph.rs | minimum_key, maximum_key | lg n exp | W lg n exp | W lg n, S lg n exp | matches textbook |
| 11 | 40 | BSTReducedStEph.rs | reduced_value (T,I) | §3: 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 12 | 40 | BSTReducedStEph.rs | range_reduce (T,I) | lg n, lg n | W lg n, S lg n | W lg n+k, S lg n+k exp | not textbook; old wrong [4] |
| 13 | 40 | BSTReducedStEph.rs | size_link (T,I) | none (§2 size) | W 1, S 1 | W 1, S 1 | matches textbook |
| 14 | 40 | BSTReducedStEph.rs | reduced_value_link (T,I) | §3: 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 15 | 40 | BSTReducedStEph.rs | update_node, make_node | §3: 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 16 | 40 | BSTReducedStEph.rs | rotate_left/right (T,I) | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 17 | 40 | BSTReducedStEph.rs | insert/delete/find_link | lg n exp | W lg n exp | W lg n, S lg n exp | matches textbook |
| 18 | 40 | BSTReducedStEph.rs | min/max_key_link (T,I) | none | W lg n exp | W lg n, S lg n exp | matches textbook |
| 19 | 40 | BSTReducedStEph.rs | collect_keys/values/kvp | none | W n, S n | W n, S n | no textbook cost [2] |
| 20 | 40 | BSTReducedStEph.rs | height_link (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 21 | 40 | BSTReducedStEph.rs | filter/find_min/build [2] | none | n; n; n lg n exp | same | no textbook cost |
| 22 | 40 | BSTReducedStEph.rs | range_reduce_link (T,I) | lg n, lg n | W lg n, S lg n | W lg n+k, S lg n+k exp | not textbook; old wrong [4] |
| 23 | 40 | BSTReducedStEph.rs | clone_link, compare_links | none | W n, S n | W n, S n | no textbook cost |
| 24 | 40 | BSTReducedStEph.rs | iter | none | W n, S n | W n, S n | no textbook cost |

### Footnotes

1. The trait `delete` in `BSTKeyValueStEph.rs` and `BSTSizeStEph.rs` has both
   an APAS line and a Code-review line of O(n) ("filter + rebuild"). The impl
   calls `delete_link`, which descends to the key, rotates it down toward a
   leaf by priority, and removes it: expected O(lg n). CS 38.11 gives delete
   O(lg n), so the old APAS line misstates the textbook as well. The impl
   lines already said O(lg n) expected. `BSTReducedStEph.rs` has the correct
   trait line.
2. Dead helpers. In `BSTKeyValueStEph.rs` and `BSTReducedStEph.rs`,
   `collect_in_order_kvp`, `find_min_priority_idx_kvp`,
   `build_treap_from_vec`, and `filter_by_key_kvp` are called only by each
   other; nothing public reaches them. In `BSTSizeStEph.rs`, `filter_by_key`
   is dead; the rest are used by `split_rank`. They look like leftovers of the
   old filter-and-rebuild delete.
3. `split_rank` collects every (key, priority) pair in order (O(n)), then
   calls `build_treap_from_vec` on both halves. That builder scans each range
   linearly for the minimum priority; because the input is the in-order
   sequence of a treap, the recursion reproduces a treap shape and the total
   work is Σ subtree sizes = O(n · depth) = O(n lg n) expected, O(n²) worst.
   The edge cases (`rank == 0`, `rank >= size`) clone the whole tree, O(n).
   Exercise 40.1 asks for a split along one root-to-leaf path using the size
   field: O(lg n). The trait line claimed O(log n); the impl line already
   said O(n log n) expected.
4. `range_reduce_link` never reads the stored `reduced_value` fields. At each
   node it recurses left if `key > low`, adds the node if it lies in
   [low, high], and recurses right if `key < high`. Every in-range node is
   visited, so with k = keys in [low, high] the work is O(lg n + k) expected
   (the two boundary paths plus the in-range nodes), and O(n) when the range
   covers the tree. The augmentation this chapter builds exists precisely to
   answer such queries in O(lg n) by combining whole-subtree reduced values
   once the recursion is inside the range. The APAS and old lines both claim
   O(log n).

## 3. Counts (per annotation site)

210 new lines were added, one per annotated site (trait and impl counted
separately), in 3 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 132 |
| 2 | does not match textbook | 6 |
| 3 | does not match old analysis | 7 |
| 4 | no textbook cost | 72 |
| 5 | unannotated functions | 27 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 210 lines; row 3 overlaps them (2 lines are
"matches textbook; does not match old analysis", 5 lines are "does not match
textbook; does not match old analysis").

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 40 | BSTKeyValueStEph.rs | 59 | 36 | 0 | 1 | 23 |
| 2 | 40 | BSTSizeStEph.rs | 69 | 46 | 2 | 2 | 21 |
| 3 | 40 | BSTReducedStEph.rs | 82 | 50 | 4 | 4 | 28 |

## 4. Unannotated functions (27)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 40 | BSTKeyValueStEph.rs | Node::clone, default, clone, eq, fmt x5 |
| 2 | 40 | BSTSizeStEph.rs | Node::clone, default, clone, eq, fmt x5 |
| 3 | 40 | BSTReducedStEph.rs | Node::clone, default, clone, eq |
| 4 | 40 | BSTReducedStEph.rs | SumOp::clone, CountOp::clone, fmt x9 |

The commented-out `IntoIterator::into_iter` blocks (form C, r212) were not
counted.

## 5. Malformed annotations (0)

None. Two formatting oddities, left in place, where the new line follows the
existing indentation:

- `clone_link` and `compare_*_links` in all three files: the annotation is
  indented eight spaces while the free `fn` is indented four.
- `SumOp::identity`, `SumOp::combine`, `CountOp::combine` in
  `BSTReducedStEph.rs`: the doc comment follows the
  `#[verifier::external_body]` attribute rather than preceding it.

## 6. Notable findings

1. **`range_reduce` ignores the augmentation.** `BSTReducedStEph.rs`
   maintains a reduced value at every node, but `range_reduce_link` walks
   every in-range node and combines the lifted values one by one: Work
   O(lg n + k), O(n) for a full range. The four annotation sites (trait and
   impl, public and link) all claim O(log n). This is a true cost defect
   against the purpose of Section 3 of the chapter.
2. **`split_rank` (Exercise 40.1) is a rebuild, not a split.** It flattens
   the tree and rebuilds both halves with a quadratic-worst-case builder:
   O(n lg n) expected instead of O(lg n). The trait line claims O(log n).
3. **Trait `delete` lines are wrong in two files.** `BSTKeyValueStEph.rs`
   and `BSTSizeStEph.rs` annotate the trait `delete` as O(n) "filter +
   rebuild" in both the APAS and Code-review lines; the implementation is
   the O(lg n) rotate-down treap delete. The filter-and-rebuild helpers
   remain as dead code (footnote 2).
4. Rank and select (Algorithm 40.1) match the textbook exactly: both read
   the O(1) `size` field and descend one path.
5. The expected bounds assume random priorities, but every insert takes a
   caller-supplied `priority: u64` (the literal macros hash the key). With
   adversarial priorities every tree operation degrades to O(n); none of the
   annotations states that the bound depends on the caller.
