# r228 Alg Analysis Review: Chap53

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap53.txt` (Graph Search) defines generic graph search
(Algorithm 53.4), reachability (Problem 53.2), multi-source search
(Exercise 53.3), the graph-search tree, and priority-first search. It
states no cost specification. The only quantitative statement is a bound
on rounds, not a cost.

| # | Source | Algorithm | Work | Span |
|---|---|---|---|---|
| 1 | Alg 53.4 | graphSearch (generic) | none stated | none stated |
| 2 | Thm 53.1 | graphSearch rounds | ≤ \|V\| rounds | — |
| 3 | Ex 53.3 | multi-source search | none stated | none stated |
| 4 | §4 PFS | priority-first search | none stated | none stated |

The prose says BFS (select the whole frontier) is parallel and DFS
(select one vertex) is sequential, but it attaches no cost. Every line in
this chapter is therefore "no textbook cost"; only the old-analysis
comparison applies.

Notation: h = height of the `AVLTreeSet*` trees involved (≤ |V|); F =
frontier; d(v) = out-degree. All files are sequential, so Span = Work.

## 2. Cost base

All five files store vertex sets in the Chap41 `AVLTreeSetStEph`,
`AVLTreeSetStPer`, or `AVLTreeSetMtPer`, which sit on the Chap38
ParamBST. From the Chap41 review: union and difference cost O(n h²)
(n = total size), find O(n h), to_seq O(n h), clone O(n), singleton and
size O(1). The Mt set operations have span equal to work. Chap37
`AVLTreeSeq*::nth` is O(lg n).

## 3. Reviewed functions

T = trait declaration, I = impl, F = free function. Rows merge sites with
the same verdict. Each GraphSearch file has 13 annotated sites (17 lines,
of which 4 are "N/A" first lines on the same functions); each PQMin file
has 8 sites.

### 3a. GraphSearchStEph.rs, GraphSearchStPer.rs, GraphSearchMtPer.rs (13 lines each)

The three files have the same body. `graph_search_explore` loops over
rounds; in each round it unions the frontier into the visited set,
converts the frontier to a sequence, and for each frontier vertex unions
graph(v) into `new_neighbors`, then takes the difference with the visited
set. The MtPer file has no `join`; its loops are sequential.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 53 | GraphSearch{StEph,StPer,MtPer} | graph_search (T,I,F) | none | (V+E) lg V | (V²+E) h² | no cost; old wrong [1] |
| 2 | 53 | GraphSearch{StEph,StPer,MtPer} | graph_search_multi (T,I,F) | none | (V+E) lg V | (V²+E) h² | no cost; old wrong [1] |
| 3 | 53 | GraphSearch{StEph,StPer,MtPer} | reachable (T,I,F) | none | (V+E) lg V | (V²+E) h² | no cost; old wrong [1] |
| 4 | 53 | GraphSearch{StEph,StPer,MtPer} | graph_search_explore (F) | none | (V+E) lg V | (V²+E) h² | no cost; old wrong [1][2] |
| 5 | 53 | GraphSearch{StEph,StPer,MtPer} | SelectAll::select (I) | none | \|F\|, \|F\| | \|F\|, \|F\| | no textbook cost |
| 6 | 53 | GraphSearch{StEph,StPer,MtPer} | SelectOne::select (I) | none | lg \|F\| | \|F\| h | no cost; old wrong [3] |
| 7 | 53 | GraphSearch{StEph,StPer,MtPer} | SelectionStrategy::select (T) | none | varies | \|F\| h | no cost; old wrong [4] |

Per file: rows 1–3 are 9 lines, row 4 is 1, rows 5–7 are 3; 13 lines,
of which 12 are "old wrong".

### 3b. PQMinStEph.rs, PQMinStPer.rs (8 lines each)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 53 | PQMin{StEph,StPer} | pq_min (T,I,F) | none | V² + E lg V | V(V+E) h² | no cost; old wrong [5] |
| 2 | 53 | PQMin{StEph,StPer} | pq_min_multi (T,I,F) | none | V² + E lg V | V(V+E) h² | no cost; old wrong [5] |
| 3 | 53 | PQMin{StEph,StPer} | pq_explore (F) | none | V² + E lg V | V(V+E) h² | no cost; old wrong [5] |
| 4 | 53 | PQMin{StEph,StPer} | pq_find_min_priority (F) | none | lg \|F\| | \|F\| h | no cost; old wrong [3][6] |

### Footnotes

1. Each round does an O(|V| h²) visited union and an O(|V| h²)
   difference; with at most |V| rounds that is O(|V|² h²). Each frontier
   vertex (each vertex is in the frontier once) does an
   O((|new_neighbors| + d(v)) h²) union, O((|V| + d(v)) h²), summing to
   O((|V|² + |E|) h²). The cost of the caller's `graph` closure is extra.
   The old line assumed an O(lg |V|) balanced set with O(1)-per-element
   union.
2. `graph_search_explore` takes a `strategy` argument and never calls
   `strategy.select`: every round visits the whole frontier, so every
   strategy behaves as SelectAll (BFS). SelectOne is dead code.
3. `to_seq` materializes the whole set by an in-order traversal
   (O(n h) with the deep-copy expose) to read element 0. The old line
   assumed O(lg |F|).
4. The trait declaration said "varies by strategy"; the new line gives
   the bound over both impls.
5. `pq_explore` pops one vertex per round: an O(|F| h) to_seq for the
   minimum and O(|V| h²) difference and union. Each out-edge of the
   popped vertex does an O(|V| h) `find` and an O(|V| h²) singleton
   union into the frontier, so the edges cost O(|E| |V| h²). The
   closing priorities loop is |V| singleton unions, O(|V|² h²), and
   `pq_min_multi` builds the initial frontier the same way. Total
   O(|V| (|V| + |E|) h²). The old line charged the edges O(lg |V|) each.
6. `pq_find_min_priority` is never called; `pq_explore` inlines the same
   to_seq + nth(0).

## 4. Counts (per annotation site)

55 new lines were added, one per annotated site, in 5 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 0 |
| 3 | does not match old analysis | 52 |
| 4 | no textbook cost | 55 |
| 5 | unannotated functions | 21 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 55 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 53 | GraphSearchStEph.rs | 13 | 0 | 0 | 12 | 13 |
| 2 | 53 | GraphSearchStPer.rs | 13 | 0 | 0 | 12 | 13 |
| 3 | 53 | GraphSearchMtPer.rs | 13 | 0 | 0 | 12 | 13 |
| 4 | 53 | PQMinStEph.rs | 8 | 0 | 0 | 8 | 8 |
| 5 | 53 | PQMinStPer.rs | 8 | 0 | 0 | 8 | 8 |

## 5. Unannotated functions (21)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 53 | GraphSearchStEph.rs | SearchResult clone, fmt x2; SelectAll fmt; SelectOne fmt |
| 2 | 53 | GraphSearchStPer.rs | SearchResult clone, fmt x2; SelectAll fmt; SelectOne fmt |
| 3 | 53 | GraphSearchMtPer.rs | SearchResult clone, fmt x2; SelectAll fmt; SelectOne fmt |
| 4 | 53 | PQMinStEph.rs | PQMinResult clone, fmt x2 |
| 5 | 53 | PQMinStPer.rs | PQMinResult clone, fmt x2 |

Spec functions (`spec_pqmin*_wf*`) were not counted.

## 6. Malformed annotations (0)

None is malformed. Eleven old lines read "no explicit cost in APAS — N/A"
or "no explicit PFS cost in APAS — N/A" (GraphSearch: 4 per file;
PQMin: 3 in StEph, 2 in StPer). Standard 25 forbids such lines; each sits
above a second old Code-review line that gives a cost, and the new line
was compared against that second line.

## 7. Notable findings

1. **The selection strategy is ignored.** `graph_search_explore` in all
   three GraphSearch files never calls `strategy.select`; every run is a
   whole-frontier (BFS) search, so `SelectOne` (the DFS-like selection)
   is dead code and Algorithm 53.4's "choose U ⊆ F" is not implemented.
2. **Costs are quadratic in |V| times h², not (|V|+|E|) lg |V|.** The
   AVLTreeSet backing (ParamBST with deep-copy expose and no
   rebalancing) makes every union O(n h²), and both searches union into a
   set that grows to |V| once per frontier vertex (GraphSearch) or per
   edge (PQMin). All 52 cost-bearing old lines understated this.
3. **PQMin finds the minimum by materializing the frontier.** Each round
   converts the whole frontier to a sequence to read its first element,
   and each edge does a singleton union; PFS here costs
   O(|V| (|V| + |E|) h²). The GraphSearchMtPer file is sequential
   despite its name.
