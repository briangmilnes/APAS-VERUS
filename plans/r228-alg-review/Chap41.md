# r228 Alg Analysis Review: Chap41

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap41.txt` (Sets) gives two cost specifications for the set ADT
and one exercise for fromSeq.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | CS 41.3 (array, enumerable) | size, singleton, toSeq | u | 1 |
| 2 | CS 41.3 | filter | u + Σ W(f) | 1 + max S(f) |
| 3 | CS 41.3 | intersection, union, difference | u | 1 |
| 4 | CS 41.3 | find | 1 | 1 |
| 5 | CS 41.3 | insert, delete | u | 1 |
| 6 | CS 41.4 (balanced tree) | size, singleton | 1 | 1 |
| 7 | CS 41.4 | toSeq | \|a\| | lg \|a\| |
| 8 | CS 41.4 | filter | Σ W(f) | lg \|a\| + max S(f) |
| 9 | CS 41.4 | intersection, union, difference | m lg(1 + n/m) | lg n |
| 10 | CS 41.4 | find, insert, delete | lg \|a\| | lg \|a\| |
| 11 | Ex 41.3 | fromSeq (reduce of unions) | n lg n | lg² n |

`empty` has no row in either table; where the file's own legacy line cites
"APAS Cost Spec 41.4: Work 1, Span 1" (the AVLTreeSet files), O(1) is
counted as a match; in `ArraySetStEph.rs`, which has no such line, `empty`
is "no textbook cost". In `ArraySetEnumMtEph.rs` the old APAS lines cite
CS 41.3 O(u) for `new`/`empty`, which the zero-fill of u/w words matches.

`OrdKeyMap.rs` is an ordered key-value map (the backing store of the
Chap43 ordered tables). Chapter 41 states no cost for any of its
operations, and none of its annotations carries an APAS line, so every
OrdKeyMap line is "no textbook cost".

Notation: n = |a| (or |self|), m = |b| (or |other|), u = universe size,
w = 64 (bits per word), h(T) = height of tree T, h = max height of the
trees involved. St files are sequential: Span = Work.

## 2. The ParamBST cost base

Every `AVLTreeSet*` file and `OrdKeyMap.rs` stores its elements in the
Chap38 parametric BST (`BSTParaStEph` or `BSTParaMtEph`), reviewed in
batch 7. Two facts there set every cost here:

- `expose` clones both subtrees (`node.left.clone()`, `node.right.clone()`),
  and `ParamBST::clone` is a recursive expose plus join_mid copy, so one
  expose costs O(size of the subtree).
- `join_mid`/`join_m` wrap a node without rebalancing, so h(T) is
  unbounded (a sorted insertion sequence builds a path of height n).

Hence find, insert, delete, split, min_key, max_key cost O(n h(T));
in_order costs O(n h(T)); union, intersect, difference cost O(n h²);
filter costs O(n h(T)² + Σ W(f)). The names `AVLTreeSet*` are misleading:
no AVL balancing is present in any of the four files.

## 3. Reviewed functions

Every trait function has an annotation on the trait declaration (T) and
another on the impl (I); each got its own line. Rows merge T and I when
both verdicts agree; "old wrong" means "does not match old analysis".

### 3a. ArraySetEnumMtEph.rs (26 lines)

A bit array over universe u, `Vec<u64>` of u/w words. All loops are
sequential even though the file is Mt.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 41 | ArraySetEnumMtEph.rs | new (T,I) | u, 1 | W u/w | W u/w, S u/w | matches textbook |
| 2 | 41 | ArraySetEnumMtEph.rs | empty (T) | u, 1 | W u/w | W u/w, S u/w | matches textbook |
| 3 | 41 | ArraySetEnumMtEph.rs | empty (I) | u, 1 | W 1, S 1 | W u/w, S u/w | matches; old wrong |
| 4 | 41 | ArraySetEnumMtEph.rs | find (T) | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 5 | 41 | ArraySetEnumMtEph.rs | find (I) | 1, 1 | W n, S n | W 1, S 1 | matches; old wrong |
| 6 | 41 | ArraySetEnumMtEph.rs | size (T) | u, 1 | W u, S u | W u, S u | not textbook [1] |
| 7 | 41 | ArraySetEnumMtEph.rs | size (I) | u, 1 | W 1, S 1 | W u, S u | not textbook; old wrong [1] |
| 8 | 41 | ArraySetEnumMtEph.rs | to_seq (T,I) | u, 1 | W u, S u | W u, S u | not textbook (span) |
| 9 | 41 | ArraySetEnumMtEph.rs | singleton (T) | u, 1 | W u/w | W u/w, S u/w | not textbook (span) |
| 10 | 41 | ArraySetEnumMtEph.rs | singleton (I) | u, 1 | W 1, S 1 | W u/w, S u/w | not textbook; old wrong |
| 11 | 41 | ArraySetEnumMtEph.rs | from_seq (T) | n lg n, lg² n | W u/w+n | W u/w+n, S u/w+n | not textbook [2] |
| 12 | 41 | ArraySetEnumMtEph.rs | from_seq (I) | n lg n, lg² n | W n lg n | W u/w+n, S u/w+n | not textbook; old wrong [2] |
| 13 | 41 | ArraySetEnumMtEph.rs | filter (T,I) | u+ΣW, 1+maxS | W u+ΣW | W u+ΣW, S u+ΣS | not textbook [3] |
| 14 | 41 | ArraySetEnumMtEph.rs | inter/diff/union (T) | u, 1 | W u/w | W u/w, S u/w | not textbook (span) |
| 15 | 41 | ArraySetEnumMtEph.rs | inter/diff/union (I) | u, 1 | W n·m, S n·m | W u/w, S u/w | not textbook; old wrong |
| 16 | 41 | ArraySetEnumMtEph.rs | insert, delete (T) | u, 1 | W 1, S 1 | W 1, S 1 | not textbook [4] |
| 17 | 41 | ArraySetEnumMtEph.rs | insert, delete (I) | u, 1 | W n, S n | W 1, S 1 | not textbook; old wrong [4] |

### 3b. ArraySetStEph.rs (25 lines)

An unsorted, duplicate-free `Vec`. The chapter's array-set cost spec is for
the enumerable bit array, so this file is compared with CS 41.4 (the
general set ADT).

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 41 | ArraySetStEph.rs | size, singleton (T,I) | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 41 | ArraySetStEph.rs | empty (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 41 | ArraySetStEph.rs | iter (T) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 4 | 41 | ArraySetStEph.rs | to_seq (T,I) | \|a\|, lg \|a\| | W n, S n | W n, S n | not textbook (span) |
| 5 | 41 | ArraySetStEph.rs | from_seq (T) | n lg n, lg² n | W n², S n² | W n², S n² | not textbook |
| 6 | 41 | ArraySetStEph.rs | from_seq (I) | n lg n, lg² n | W n lg n | W n², S n² | not textbook; old wrong |
| 7 | 41 | ArraySetStEph.rs | filter (T,I) | ΣW, lg n+maxS | W n+ΣW | W n+ΣW, S n+ΣS | not textbook (span) |
| 8 | 41 | ArraySetStEph.rs | inter/diff/union (T,I) | m lg(1+n/m) | W n·m, S n·m | W n·m, S n·m | not textbook [5] |
| 9 | 41 | ArraySetStEph.rs | find, insert, delete | lg \|a\| | W n, S n | W n, S n | not textbook [5] |

### 3c. AVLTreeSetStEph.rs (52 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 41 | AVLTreeSetStEph.rs | size, empty, singleton | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 41 | AVLTreeSetStEph.rs | to_seq (T,I) | \|a\|, lg \|a\| | W n, S n | W n h, S n h | not textbook; old wrong |
| 3 | 41 | AVLTreeSetStEph.rs | from_seq (T,I) | n lg n, lg² n | W n lg n | W n² h, S n² h | not textbook; old wrong [6] |
| 4 | 41 | AVLTreeSetStEph.rs | filter (T,I) | ΣW, lg n+maxS | W n / ΣW | W n h²+ΣW | not textbook; old wrong |
| 5 | 41 | AVLTreeSetStEph.rs | inter/diff/union | m lg(1+n/m) | W m lg(1+n/m) | W n h², S n h² | not textbook; old wrong |
| 6 | 41 | AVLTreeSetStEph.rs | find/insert/delete | lg \|a\| | W lg n | W n h, S n h | not textbook; old wrong |
| 7 | 41 | AVLTreeSetStEph.rs | *_iter (7 fns, T,I) | as base op | as base op | as base op | not textbook; old wrong [7] |
| 8 | 41 | AVLTreeSetStEph.rs | *_sorted (6 fns, T,I) | as base op | as base op | as base op | not textbook; old wrong [7] |
| 9 | 41 | AVLTreeSetStEph.rs | iter (T) | none | W n, S n | W n h, S n h | no cost; old wrong |
| 10 | 41 | AVLTreeSetStEph.rs | clone_wf (I) | none | W n, S n | W n, S n | no textbook cost |

Rows 2–8 are 44 lines. Row 3's impl line and the impl `to_seq` also note
that `AVLTreeSeqStEph::from_vec` inserts one element at a time
(O(n lg n)).

### 3d. AVLTreeSetStPer.rs (42 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 41 | AVLTreeSetStPer.rs | size, empty, singleton | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 41 | AVLTreeSetStPer.rs | to_seq (T,I) | \|a\|, lg \|a\| | W n, S n | W n h, S n h | not textbook; old wrong |
| 3 | 41 | AVLTreeSetStPer.rs | from_seq (T,I) | n lg n, lg² n | W n lg n | W n² h, S n² h | not textbook; old wrong [6] |
| 4 | 41 | AVLTreeSetStPer.rs | filter (T,I) | ΣW, lg n+maxS | W n / ΣW | W n h²+ΣW | not textbook; old wrong |
| 5 | 41 | AVLTreeSetStPer.rs | inter/diff/union | m lg(1+n/m) | W m lg(1+n/m) | W n h², S n h² | not textbook; old wrong |
| 6 | 41 | AVLTreeSetStPer.rs | find (T,I) | lg \|a\| | W lg n | W n h, S n h | not textbook; old wrong |
| 7 | 41 | AVLTreeSetStPer.rs | insert, delete (T,I) | lg \|a\| | W lg n | W n h, S n h | not textbook; old wrong [8] |
| 8 | 41 | AVLTreeSetStPer.rs | *_iter (7 fns, T,I) | as base op | as base op | as base op | not textbook; old wrong [7] |
| 9 | 41 | AVLTreeSetStPer.rs | insert_sorted_per | lg \|a\| | W n (T) | W n h, S n h | not textbook; old wrong |
| 10 | 41 | AVLTreeSetStPer.rs | iter (T) | none | W n, S n | W n h, S n h | no cost; old wrong |
| 11 | 41 | AVLTreeSetStPer.rs | clone_wf (I) | none | W n, S n | W n, S n | no textbook cost |

Rows 2–9 are 34 lines, all also "does not match old analysis"; with
`iter` that gives 35 "old wrong" lines.

### 3e. AVLTreeSetMtEph.rs (24 lines) and AVLTreeSetMtPer.rs (24 lines)

Both files sit on `BSTParaMtEph`: union, intersect, and difference fork with
`ParaPair`, but each level still pays an O(n) deep-copy expose and a
sequential O(n h) split, so the span equals the work.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 41 | AVLTreeSetMtEph.rs | size, empty, singleton | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 41 | AVLTreeSetMtEph.rs | to_seq (T,I) | \|a\|, lg \|a\| | W n, S n | W n h, S n h | not textbook; old wrong |
| 3 | 41 | AVLTreeSetMtEph.rs | from_seq (T,I) | n lg n, lg² n | W n lg n, lg² n | W n h² lg n, S n h² | not textbook; old wrong [9] |
| 4 | 41 | AVLTreeSetMtEph.rs | filter (T,I) | ΣW, lg n+maxS | W n / ΣW | W n h²+ΣW | not textbook; old wrong |
| 5 | 41 | AVLTreeSetMtEph.rs | inter/diff/union | m lg(1+n/m), lg n | W m lg(1+n/m) | W n h², S n h² | not textbook; old wrong |
| 6 | 41 | AVLTreeSetMtEph.rs | find/insert/delete | lg \|a\| | W lg n | W n h, S n h | not textbook; old wrong |
| 7 | 41 | AVLTreeSetMtPer.rs | size, empty, singleton | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 8 | 41 | AVLTreeSetMtPer.rs | to_seq (T,I) | \|a\|, lg \|a\| | W n, S n | W n h, S n h | not textbook; old wrong |
| 9 | 41 | AVLTreeSetMtPer.rs | from_seq (T,I) | n lg n, lg² n | W n lg n, lg² n | W n h² lg n, S n h² | not textbook; old wrong [9] |
| 10 | 41 | AVLTreeSetMtPer.rs | filter (T,I) | ΣW, lg n+maxS | W ΣW, S n / n | W n h²+ΣW | not textbook; old wrong |
| 11 | 41 | AVLTreeSetMtPer.rs | inter/diff/union | m lg(1+n/m), lg n | W m lg(1+n/m) | W n h², S n h² | not textbook; old wrong |
| 12 | 41 | AVLTreeSetMtPer.rs | find (T,I) | lg \|a\| | W lg n | W n h, S n h | not textbook; old wrong |
| 13 | 41 | AVLTreeSetMtPer.rs | insert, delete (T,I) | lg \|a\| | W lg n | W n h, S n h | not textbook; old wrong [8] |

### 3f. OrdKeyMap.rs (58 lines)

All 58 lines are "no textbook cost" (Section 1). The table records the new
cost and whether the old line holds.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 41 | OrdKeyMap.rs | new, size, is_empty (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 41 | OrdKeyMap.rs | find, insert, delete | none | W lg n | W n h, S n h | no cost; old wrong |
| 3 | 41 | OrdKeyMap.rs | split (T,I) | none | W lg n | W n h, S n h | no cost; old wrong |
| 4 | 41 | OrdKeyMap.rs | next/prev_key (T,I) | none | W lg n | W n h, S n h | no cost; old wrong |
| 5 | 41 | OrdKeyMap.rs | rank/select_key (T,I) | none | W lg n | W n h, S n h | no cost; old wrong |
| 6 | 41 | OrdKeyMap.rs | first/last_key (T,I) | none | W lg n | W n h, S n h | no cost; old wrong |
| 7 | 41 | OrdKeyMap.rs | get_key_range (T,I) | none | W lg n | W n h, S n h | no cost; old wrong |
| 8 | 41 | OrdKeyMap.rs | split_rank_key (T,I) | none | W n, S n | W n h, S n h | no cost; old wrong |
| 9 | 41 | OrdKeyMap.rs | union, union_with | none | W n·m, S n·m | W (n+m)² h | no cost; old wrong [10] |
| 10 | 41 | OrdKeyMap.rs | intersect(_with) | none | W n·m, S n·m | W n m h + n h | no cost; old wrong [10] |
| 11 | 41 | OrdKeyMap.rs | difference (T,I) | none | W n·m, S n·m | W n m h + n² | no cost; old wrong [10] |
| 12 | 41 | OrdKeyMap.rs | collect, iter (T,I) | none | W n, S n | W n h, S n h | no cost; old wrong |
| 13 | 41 | OrdKeyMap.rs | filter (T,I) | none | W n lg n | W n h² + ΣW | no cost; old wrong [11] |
| 14 | 41 | OrdKeyMap.rs | map_values (T,I) | none | W n lg n | W n² + ΣW | no cost; old wrong [10] |
| 15 | 41 | OrdKeyMap.rs | reduce (T,I) | none | W n, S n | W n h + ΣW | no cost; old wrong |
| 16 | 41 | OrdKeyMap.rs | domain (T,I) | none | W n, S n | W n², S n² | no cost; old wrong [12] |
| 17 | 41 | OrdKeyMap.rs | tabulate (T,I) | none | W n lg n | W n² h + ΣW | no cost; old wrong [10] |
| 18 | 41 | OrdKeyMap.rs | restrict, subtract | none | W n·m, S n·m | W n² + n m | no cost; old wrong [10] |

### Footnotes

1. `size` scans every bit of the universe; there is no stored count. The
   work matches CS 41.3 (u) but the scan is a sequential loop, so the span
   is u instead of 1. The impl line claimed O(1).
2. `from_seq` sets one bit per element after an O(u/w) zero-fill: less work
   than the Ex 41.3 reduce-of-unions (n lg n) when u/w is small, but a
   sequential loop, so the span is O(u/w + n), not lg² n.
3. `filter` in an Mt file is a sequential loop calling f on each member in
   turn; CS 41.3 asks for span 1 + max S(f).
4. `insert` and `delete` update the bit in place (ephemeral), O(1); CS 41.3
   charges u for a persistent copy. The impl lines claimed O(n).
5. The array is unsorted, so find is a linear scan, insert copies the
   array to append, and each bulk operation does a linear find per
   element: n·m.
6. `from_seq` is n sequential inserts into the ParamBST, each O(n h(T)).
7. The `*_iter` functions are "iterative alternatives" in name only; each
   impl calls the base operation (find, insert, delete, filter,
   intersection, union, difference). The `*_sorted` functions likewise
   delegate. Each line gets the base operation's cost.
8. The persistent `insert` and `delete` clone the whole tree first (O(n)),
   then run the ephemeral BSTParaStEph/MtEph operation.
9. `from_vec_dc` / `from_vec_dc_per` have the Ex 41.3 shape: split the Vec,
   recurse on both halves with `join`, then union. But each level copies
   its halves sequentially, each BSTParaMtEph union costs O(n h²) work and
   span, and `from_seq` first fills the Vec sequentially. Work
   O(n h² lg n), span O(n h²).
10. These build their result by inserting entries one at a time into a
    fresh ParamBST. When the entries arrive in key order (from `in_order`),
    each insert sees a key larger than every key in the tree: split exposes
    (copies) only the root, O(i), and `join_m` hangs the old tree as the
    left child. The total is O(N²) and the result is a path of height N,
    which makes every later operation on it O(N²). `union` and `tabulate`
    insert in non-sorted order, bounded by O(N² h).
11. The impl line of `filter` says "in_order + conditional BST inserts",
    but the body delegates to `BSTParaStEph::filter`.
12. `domain` inserts each key into an `ArraySetStEph`, whose insert is an
    O(i) linear find plus copy.

## 4. Counts (per annotation site)

251 new lines were added, one per annotated site (trait and impl counted
separately), in 7 files. `Example41_3.rs` has no annotations and is an
Example file (skipped).

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 34 |
| 2 | does not match textbook | 152 |
| 3 | does not match old analysis | 179 |
| 4 | no textbook cost | 65 |
| 5 | unannotated functions | 46 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 251 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 41 | ArraySetEnumMtEph.rs | 26 | 6 | 20 | 10 | 0 |
| 2 | 41 | ArraySetStEph.rs | 25 | 4 | 18 | 1 | 3 |
| 3 | 41 | AVLTreeSetStEph.rs | 52 | 6 | 44 | 45 | 2 |
| 4 | 41 | AVLTreeSetStPer.rs | 42 | 6 | 34 | 35 | 2 |
| 5 | 41 | AVLTreeSetMtEph.rs | 24 | 6 | 18 | 18 | 0 |
| 6 | 41 | AVLTreeSetMtPer.rs | 24 | 6 | 18 | 18 | 0 |
| 7 | 41 | OrdKeyMap.rs | 58 | 0 | 0 | 52 | 58 |

## 5. Unannotated functions (46)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 41 | ArraySetEnumMtEph.rs | eq, clone, fmt x2 |
| 2 | 41 | ArraySetStEph.rs | default, eq, clone, fmt x2 |
| 3 | 41 | AVLTreeSetMtEph.rs | iter (T,I), from_vec_dc |
| 4 | 41 | AVLTreeSetMtEph.rs | default, clone, fmt x2 |
| 5 | 41 | AVLTreeSetMtPer.rs | assert_..._always_wf, _bounded_size |
| 6 | 41 | AVLTreeSetMtPer.rs | from_vec_dc_per, into_iter |
| 7 | 41 | AVLTreeSetMtPer.rs | partial_cmp, cmp, default, eq |
| 8 | 41 | AVLTreeSetMtPer.rs | clone, fmt x2 |
| 9 | 41 | AVLTreeSetStEph.rs | default, eq, clone, fmt x2 |
| 10 | 41 | AVLTreeSetStPer.rs | default, eq, clone, fmt x2 |
| 11 | 41 | OrdKeyMap.rs | ordkeymap_find, _split, _next, _prev |
| 12 | 41 | OrdKeyMap.rs | ordkeymap_rank, _select |
| 13 | 41 | OrdKeyMap.rs | clone, fmt x2 |

The commented-out `IntoIterator::into_iter` blocks (form C, r212) were not
counted. `AVLTreeSetMtPer.rs` still has a live `into_iter`, counted above.
The 14 functions of `Example41_3.rs` were not counted.

## 6. Malformed annotations (0)

No `/// - Alg Analysis:` line is malformed. The AVLTreeSet files carry
legacy cost lines in other formats, left in place:

- `/// - APAS Cost Spec 41.4: Work 1, Span 1` on `empty`, one per file
  (4 lines).
- `/// - claude-4-sonet: ...` lines (34 lines: MtEph 12, MtPer 12, StEph 5,
  StPer 5). Several are wrong (for example `to_seq`: "Work Θ(1)"). Where one
  follows the Alg Analysis block, the new line was placed after the last
  Alg Analysis line and before the sonet line.
- `from_vec_dc_per` has a free-form "Work O(n lg n), Span O(lg^2 n)" doc
  line, not in Alg Analysis form.

## 7. Notable findings

1. **The AVLTreeSet family is not AVL and not O(lg n).** All four
   `AVLTreeSet*` files sit on the Chap38 ParamBST, whose `expose`
   deep-copies both subtrees and whose `join_mid` never rebalances. Every
   non-O(1) operation costs O(n h(T)) or more (find/insert/delete
   O(n h), bulk operations O(n h²), filter O(n h² + ΣW)) against the CS 41.4
   lg n and m lg(1 + n/m). The old lines claimed the textbook costs; 116 of
   the 142 AVLTreeSet lines contradict the old analysis. Fixing expose
   (move or share subtrees instead of cloning) and adding balancing to
   join_mid in Chap38 would repair all four files at once.
2. **OrdKeyMap builds degenerate trees.** Its bulk operations (union,
   intersect, difference, map_values, restrict, subtract, tabulate) collect
   entries in order and insert them one at a time into a fresh
   never-rebalanced tree. Sorted insertion makes the result a path of
   height N at O(N²) cost, and every later operation on that map costs
   O(N²). The old lines (n·m, n lg n) understate this. Chap43's ordered
   tables inherit these costs.
3. **ArraySetEnumMtEph impl annotations were badly off, and its "Mt"
   operations are sequential.** The impl lines claimed size O(1) (it scans
   the universe, O(u)), bulk operations O(n·m) (they are O(u/w) word
   loops), find/insert/delete O(n) (they are O(1)). Every loop, including
   filter, is sequential in an Mt file, so no operation meets the CS 41.3
   span of 1.
4. The `*_iter` and `*_sorted` variants in AVLTreeSetStEph/StPer only
   delegate to the base operations; they add annotation sites but no
   distinct algorithm.
5. The Mt `from_seq` has the right Ex 41.3 divide-and-conquer shape
   (`from_vec_dc` with `join`), so it will reach O(n lg n) work and
   O(lg² n) span once the ParamBST union does.
