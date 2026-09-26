# r228: algorithmic-analysis review of APAS-VERUS

Round r228, 2026-09-26. Branch `r228/alg-review` (from main `fc9b61116`).
Review only: no code, spec or proof changed, and no existing annotation was
edited. Fixes are a later round.

## Method

Every function with a `/// - Alg Analysis:` annotation in `src/ChapNN/` was
read, together with its callees. Each one got one new line directly under its
existing annotations:

```
/// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(...), Span O(...) — <verdict>
```

The verdict is one or more of:

- `matches textbook`;
- `does not match textbook: <reason>`;
- `does not match old analysis: <old> vs new; <reason>`;
- `no textbook cost`.

"Textbook" means the APAS prose (`prompts/ChapNN.txt`, or the book PDF where
the prose cites cost tables from other chapters), not the existing `APAS` lines.
Callee costs come from the reviewed cost, not the old annotation. For example,
the Chap43 reviews use the real Chap38 and Chap41 costs.

- Fourteen reviewer agents ran, three at a time, one batch of chapters each.
- After each batch the main session checked that the diff added only review
  lines, verified the batch's chapters in isolation, and committed.
- At the end, the full crate verified with 5840 verified, 0 errors
  (`logs/validate.20260926-124941.log`), the same count as main.

Per-chapter details, with one table row per function, are in
`plans/r228-alg-review/ChapNN.md`.

## Totals

The counts below are taken from the review lines in the source, not from the
reviewers' reports.

- The first three columns partition the lines: match + does not match + no
  textbook cost = lines.
- "Old wrong" overlaps them: it counts the sites where the existing Code-review
  line is wrong.

| # | Chap | Lines | Matches textbook | Does not match textbook | Old wrong | No textbook cost |
|---|---|---|---|---|---|---|
| 1 | 02 | 11 | 2 | 0 | 2 | 9 |
| 2 | 03 | 1 | 1 | 0 | 0 | 0 |
| 3 | 05 | 132 | 44 | 44 | 5 | 44 |
| 4 | 06 | 512 | 0 | 0 | 106 | 512 |
| 5 | 11 | 6 | 5 | 1 | 0 | 0 |
| 6 | 12 | 21 | 0 | 0 | 2 | 21 |
| 7 | 17 | 36 | 0 | 0 | 0 | 36 |
| 8 | 18 | 348 | 133 | 133 | 48 | 82 |
| 9 | 19 | 188 | 63 | 84 | 17 | 41 |
| 10 | 21 | 22 | 6 | 9 | 0 | 7 |
| 11 | 23 | 66 | 16 | 28 | 9 | 22 |
| 12 | 26 | 58 | 0 | 46 | 31 | 12 |
| 13 | 27 | 13 | 0 | 13 | 3 | 0 |
| 14 | 28 | 34 | 2 | 24 | 12 | 8 |
| 15 | 30 | 21 | 0 | 0 | 0 | 21 |
| 16 | 35 | 16 | 0 | 16 | 10 | 0 |
| 17 | 36 | 36 | 0 | 18 | 12 | 18 |
| 18 | 37 | 905 | 228 | 127 | 154 | 550 |
| 19 | 38 | 105 | 16 | 74 | 74 | 15 |
| 20 | 39 | 246 | 99 | 95 | 114 | 52 |
| 21 | 40 | 210 | 132 | 6 | 7 | 72 |
| 22 | 41 | 251 | 34 | 152 | 179 | 65 |
| 23 | 42 | 118 | 18 | 74 | 17 | 26 |
| 24 | 43 | 620 | 74 | 526 | 542 | 20 |
| 25 | 44 | 54 | 2 | 18 | 40 | 34 |
| 26 | 45 | 225 | 49 | 50 | 103 | 126 |
| 27 | 47 | 76 | 61 | 0 | 5 | 15 |
| 28 | 49 | 130 | 0 | 24 | 12 | 106 |
| 29 | 50 | 198 | 36 | 28 | 18 | 134 |
| 30 | 51 | 144 | 14 | 14 | 8 | 116 |
| 31 | 52 | 321 | 100 | 166 | 127 | 55 |
| 32 | 53 | 55 | 0 | 0 | 52 | 55 |
| 33 | 54 | 40 | 0 | 16 | 16 | 24 |
| 34 | 55 | 35 | 8 | 14 | 18 | 13 |
| 35 | 56 | 133 | 0 | 0 | 0 | 133 |
| 36 | 57 | 18 | 0 | 4 | 4 | 14 |
| 37 | 58 | 8 | 0 | 4 | 4 | 4 |
| 38 | 59 | 30 | 0 | 12 | 12 | 18 |
| 39 | 61 | 19 | 4 | 13 | 8 | 2 |
| 40 | 62 | 30 | 0 | 12 | 25 | 18 |
| 41 | 63 | 18 | 0 | 16 | 16 | 2 |
| 42 | 64 | 20 | 2 | 10 | 13 | 8 |
| 43 | 65 | 10 | 0 | 3 | 3 | 7 |
| 44 | 66 | 27 | 0 | 16 | 14 | 11 |
| | | **5567** | **1149 (21%)** | **1890 (34%)** | **1842 (33%)** | **2528 (45%)** |

Of the 3039 sites where the textbook states a cost, 1149 (38%) meet it.
One site in three has a wrong existing analysis.

Per the chapter reports:

- **1046 functions carry no Alg Analysis annotation.** Most are `Clone`,
  `PartialEq`, `Debug`, `Display` and `IntoIterator` impls. The algorithmic
  ones are the three Chap65 union-find files and some Chap37 and Chap45
  helpers.
- **115 existing annotations are malformed:**
  - 47 truncated "Work O(m log(n/m + 1)" lines in Chap43;
  - 19 bare `O(...)` lines in Chap17 and 10 in Chap12;
  - 13 Chap47 lines with no Span;
  - about 12 sites where an annotation is glued onto a `// Veracity:` line and
    so is a plain comment, not a doc line (Chap06, 18, 37, 38, 43, 45, 50,
    52). These got no review line; the chapter reports give their costs.

`scripts/check-alg-analysis.sh` before and after the review: errors 241 → 237,
warnings 286 → 286, info 1211 → 1215. The checker does not read the new line's
verdict, so these counts barely move. Logs:
`plans/r228-alg-review/check-alg-analysis-{before,after}.log`.

## Findings by cause

The 1890 "does not match textbook" sites come from a small number of causes.
Most of them sit in a few shared building blocks, so a fix there repairs many
chapters at once.

### 1. The Chap38 parametric BST: `expose` deep-copies, and `join_mid` never rebalances

This is the root cause with the most downstream effects.

- **Chap38.** `BSTParaStEph`, `BSTParaMtEph` and `BSTParaTreapMtEph` implement
  `expose` by cloning both subtrees, which is Θ(n) where the textbook's
  interface is O(1). `join_mid` wraps a node without rebalancing, so the height
  h can reach n.
  - find, split, insert and delete cost O(n h), up to O(n²).
  - union, intersect and difference cost O(n h²).
  - `join_pair` is not Algorithm 38.4: it grafts a full copy, or runs a union.
- **Chap39.** The treap `BSTTreapStEph` keeps a min-heap on priority for the
  rotation operations and a max-heap for the parametric ones, on the same
  nodes. That voids the O(lg n) expected bounds.
- **Chap41.** All four AVLTreeSet files sit on ParamBST. They are neither AVL
  trees nor O(lg n); 116 of their 142 old lines claimed the textbook cost.
  OrdKeyMap bulk operations insert keys one at a time into a tree that never
  rebalances, which builds paths in O(N²).
- **Chap43.** OrderedSet and OrderedTable inherit all of this, and no ordered
  operation meets O(lg n); 542 old lines were wrong.
- **Chap52 and Chap53 inherit it too.**
  - Chap52: edge-set `has_edge` is O(|E| h_E).
  - Chap53: GraphSearch costs O((|V|²+|E|) h²), and PQMin
    O(|V|(|V|+|E|) h²).

Fix candidate: make `expose` move or share its subtrees (for example with Arc)
and make `join_mid` rebalance. That one change would repair most of Chap38,
39, 41, 43 and 52.

### 2. Divide-and-conquer "parallel" helpers that copy sequentially at every level

A recurring pattern: the code forks correctly, but it splits with
`subseq_copy` or rejoins with `append`, a push loop, or `union`, each a
sequential O(n) copy per level. The result is Work O(n lg n) and Span O(n)
against the textbook's O(n) and O(lg n).

| # | Chap | Where |
|---|---|---|
| 1 | 18 | ArraySeqMtEph/MtPer `map_dc`, `filter_dc`, `reduce_dc`, `*_par`, `*_inner` |
| 2 | 18, 19 | ArraySeqMtEphSlice map, filter, tabulate, scan, flatten |
| 3 | 19 | ArraySeqMtEph `reduce_par` (old line said Span O(lg n)) |
| 4 | 23 | BalBinTreeStEph traversals: O(n·h), not O(n) |
| 5 | 26 | MergeSortMtPer `merge_dc` |
| 6 | 35 | OrderStatSelectMt `partition_three_dc` |
| 7 | 36 | QuickSortMtEphSlice partition |
| 8 | 37 | BSTRBMtEph r225 fork-join traversals (Work and Span O(n lg n)) |
| 9 | 42 | TableMtEph `map_table_dc`, `tabulate_table_dc` |
| 10 | 62, 66 | StarContractionMtEph and Boruvka merge steps |

In row 3, the ArraySeqMtEphSlice `reduce` splits in O(1) and meets the textbook
bound, which shows the fix: split by slice, not by copy. The remaining cost is
the rejoin, which needs a sequence type whose append has sublinear span.

### 3. "Mt" code that runs sequentially

Many Mt functions and files never fork. Their span equals their work, and
their old lines claimed the textbook span.

| # | Chap | Where |
|---|---|---|
| 1 | 05 | SetMtEph: every bulk operation except `cartesian_product` |
| 2 | 06 | none (the Mt graphs fork, but see cause 4) |
| 3 | 18 | ArraySeqMtPer `reduce`, `map`, `tabulate`, `scan` (parallel versions unused); MtEph `ninject` |
| 4 | 21 | built entirely on St sequences |
| 5 | 26 | DivConReduceMtPer; ScanDCMtPer split and concatenation |
| 6 | 27 | contraction forks once and clones its input |
| 7 | 28 | MaxContigSubSum D&C Mt files: no `join` at all |
| 8 | 37 | splay `*_parallel`; BSTSet*MtEph set operations |
| 9 | 38 | BSTParaMtEph `filter_parallel` |
| 10 | 41 | ArraySetEnumMtEph (every loop sequential) |
| 11 | 43 | every Mt file takes one lock and calls the St code |
| 12 | 44, 45 | nothing parallel |
| 13 | 50 | MatrixChainMtEph/MtPer: no `join`; Span O(n³) against O(n lg n) |
| 14 | 51, 52 | no Mt file forks |
| 15 | 62 | `route_edges_parallel` (no fork despite its name) |
| 16 | 63 | `compose_maps_parallel` (sequential and unused) |

### 4. Whole-structure copies on every update or fork

| # | Chap | Where | Cost |
|---|---|---|---|
| 1 | 05 | `cartesian_product` (St and Mt) | Work O(\|a\|²·\|b\|), against O(\|a\|·\|b\|) |
| 2 | 06 | Mt `n_plus_par`, `ng_par`, `*_of_vertices_par` clone the graph twice per fork | Work O(\|A\|(\|V\|+\|A\|)) |
| 3 | 45 | every priority queue copies itself per update; BinaryHeapPQ `swap_elements` O(n²) | heapify, heapsort O(n³) |
| 4 | 45 | LeftistHeapPQ wrapper deep-clones before `meld` (though `meld_nodes` meets Thm 45.2) | O(m + n) |
| 5 | 49 | Mt subset sum and edit distance clone their inputs twice per branch | Work up to O(\|S\|\|T\|(\|S\|+\|T\|)) |
| 6 | 52 | AdjMatrix `set_edge` rebuilds \|V\|² (including the "in-place O(1)" files) | O(\|V\|²) |
| 7 | 54 | BFS Mt `process_frontier_*` deep-copies the graph per fork | |
| 8 | 56 | AllPairs `set_distance`/`set_predecessor` clone a row | O(n) |

### 5. Representation choices far from the textbook's

- **Chap18.** The "linked lists" are Vec-backed: `nth` is O(1) rather than
  O(i), and `append` is O(|a|+|b|) rather than O(|a|).
- **Chap23.** `PrimTreeSeqStPer` is Vec-backed, so expose, join, append and
  subseq are linear. `BalBinTreeStEph::size` is O(n) because nodes store no
  size.
- **Chap37.** `BSTAVLStEph` and `BSTAVLMtEph` store no heights, so each
  rebalance re-walks subtrees and insert costs O(n). The five BSTSet*MtEph set
  operations peel one key per level and rebuild the tree: O(n³) for the plain
  and AVL trees, O(n² lg n) for BB[α] and red-black.
- **Chap42.** The tables are unsorted arrays: point operations cost O(|a|), bulk
  operations O(|a|·|b|), and `domain` O(|a|²). Chap44 `make_index` and `find`
  inherit this.
- **Chap55.** SCC `transpose_graph` does a Vec remove and insert per edge,
  O(|V||E|).
- **Chap58, 59.** `in_neighbors_weighed` scans all m arcs, so Bellman-Ford is
  O(n² m).
- **Chap61.** Vertex matching scans every edge per vertex: O(|E|²) per round.
- **Chap64.** The TSP approximation's Euler tour scans the graph per step: O(n m).
- **Chap65.** Kruskal sorts its edges with a selection sort, O(m²).

### 6. Algorithms that are not the textbook algorithm

These go beyond cost: the code implements something else.

| # | Chap | File | Finding |
|---|---|---|---|
| 1 | 26 | ETSPStEph, ETSPMtEph | `find_best_swap` is a stub returning (0,0); split at index midpoint, not longest dimension. Not Alg 26.7. |
| 2 | 38 | BSTPara* | `join_pair` is not Alg 38.4 (grafts a copy or runs a union) |
| 3 | 40 | BSTReducedStEph | `range_reduce` never reads the stored reduced values: O(lg n + k), not O(lg n) |
| 4 | 40 | BSTSizeStEph | `split_rank` flattens and rebuilds: O(n lg n), not O(lg n) |
| 5 | 43 | AugOrderedTable* | one cached reduction per table, not per node (not Def 43.3); `reduce_range` O(n h) |
| 6 | 43 | Ordered* | `join`/`join_key` call union, not a BST join |
| 7 | 44 | DocumentIndex | `make_index` does not follow Alg 44.2 |
| 8 | 45 | LeftistHeapPQ | `from_seq` inserts one at a time (O(n²)) instead of reducing with meld |
| 9 | 53 | GraphSearch | `graph_search_explore` never calls `strategy.select` (SelectOne is dead code) |
| 10 | 54 | BFSMtEph, BFSMtPer | `next_vertices` not deduplicated: the frontier grows to the number of shortest paths, which can be exponential |
| 11 | 37 | BSTSplay* | `find`/`contains` never splay, so there is no amortized bound |
| 12 | 57, 65 | Dijkstra, Prim | use Chap45 BinaryHeapPQ with an O(q²) `delete_min`: O(n m + m³), against O(m lg n) |

### 7. Correctness and rule issues found along the way

These are not cost findings, but the reviews turned them up:

- **Chap12.** `Exercise12_5` `ConcurrentStackMt::pop` has no memory reclamation,
  so it can read freed memory and is exposed to ABA. It also uses `unsafe`,
  which violates the no-unsafe standard.
- **Chap11.** `fib_par` switches to sequential code for n ≤ 10, which violates
  the no-threshold rule. `fib_2threads` is not the recursive Ex 11.10.
- **Chap49, 50.** The Mt subset-sum, edit-distance and OBST memos are read
  before `join` and written after it. Parallel branches can recompute the same
  state; the result is correct, but the worst-case work is exponential.
- **Chap66.** St Boruvka's coins are `(seed ^ index) & 1`, with the same seed
  every round. Its O(lg n) round bound then depends on hash-set iteration order.
- **Chap62, 63, 64 St.** The greedy star partition can remove one vertex per
  round. Its worst case is O(n² m).

## Old analyses that were wrong

1842 sites (33%) have an existing Code-review line that the new review
contradicts. The largest groups:

- **Claimed textbook costs that the code doesn't have:**
  - Chap38, 41 and 43: 116 + 542 lines, cause 1;
  - Chap18 and 19 D&C helpers, cause 2.
- **Construction costs overstated:**
  - Chap06 `from_sets` and its variants move their inputs in O(1), not
    O(|V|+|A|);
  - Chap18 `from_vec` is O(1);
  - Chap50 Mt constructors are O(1).
- **Cached sizes claimed O(n):**
  - Chap39: five `size` lines;
  - Chap41: ArraySetEnumMtEph `size` is O(u), claimed O(1).
- **Off by a factor of n:**
  - Chap28: the "single-pass O(n)" MCSS files are O(n²), and brute-force MCSS
    is O(n²), not O(n³);
  - Chap50: `clear_memo` is O(n²), not O(1).
- **Invented textbook lines.** Several chapters' `APAS` lines cite costs, or
  algorithm numbers, that the text doesn't contain:
  - Chap05, 06 and 11: 302 lines in Chap06 alone;
  - Chap19: Alg 19.11 and 19.15;
  - Chap21: Span = Work on the Problem functions.

  Standard 25 says not to write an APAS line when the book gives no cost.
- **Stale descriptions** ("linear scan", "treap", "external_body") that no
  longer match the code, mostly in Chap43.

## Where the code matches the textbook

- Chap47 hash tables match everywhere the book states a cost. The one caveat is
  that deleted slots are never reused, so the load factor counts them.
- Chap40 BSTKeyValue and the size-augmented BSTs match everywhere except
  `range_reduce` and `split_rank`.
- The Chap37 red-black, BB[α] and AVLTreeSeq trees match for point operations.
- Chap39 rotation treaps match when callers pass random priorities.
- The Chap45 leftist `meld_nodes` meets its bound.
- Chap55 CycleDetect and TopoSortStPer match.
- Chap56 accessors match.

## Suggested fix order (not started)

1. **Chap38 `expose` and `join_mid`.** The fix is to share subtrees and to
   rebalance. It repairs most of Chap38, 39, 41, 43, 52 and 53.
2. **A sequence type with sublinear-span split and append** (or slice-based
   split). It repairs cause 2 across Chap18–66 and lets the r225 traversals
   reach O(lg² n) span.
3. **Chap45 priority queues** (in-place heap, `from_seq` by meld). Dijkstra,
   Prim and Johnson inherit the fix.
4. **Wire up the unused parallel versions** (Chap18 MtPer, Chap50
   MatrixChain), and add `join` where Mt files have none.
5. **Algorithm corrections from cause 6:**
   - ETSP swap search;
   - the Chap43 per-node augmentation;
   - GraphSearch `select`;
   - BFS Mt deduplication.
6. **Annotation cleanup:**
   - remove the invented APAS lines;
   - repair the 115 malformed lines;
   - replace the old Code-review lines with the reviewed ones.
