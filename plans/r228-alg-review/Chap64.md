# r228 Alg Analysis Review: Chap64

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap64.txt` (Minimum spanning trees: definitions, the spanning
tree by star contraction, and the MST-based TSP approximation) states no
cost explicitly. The costs below are the ones the chapter implies and the
files' APAS lines cite.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Ex 64.2 | spanning tree by star contraction (Thm 62.3 costs) | (n + m) lg n | lg² n |
| 2 | Sec. 4 (TSP) | Euler tour of a tree (DFS) | n | n |
| 3 | Sec. 4 | shortcut the Euler tour | n | n |
| 4 | Sec. 4 | tour weight | n | n |
| 5 | Sec. 4 | approximate metric TSP given the tree | n + m | n + m |

The St spanning-tree APAS line uses span O((n + m) lg n) (sequential).

Notation: n = |V|, m = |E|, c = centers per round. Callee costs:

- Chap62 St `star_contract`: O(n² m) worst case; Mt `star_contract_mt`:
  Work O((n + m) lg² n), Span O(n lg n + m lg² n), expected (this batch).
- Chap06 `LabUnDirGraphStEph::ng` and `get_edge_label` scan all labeled
  edges: O(m) each.

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 64 | SpanTreeStEph.rs | spanning_tree_star_contraction (T,I) | (n+m)lg n | W (n+m) lg n | W n² m, S n² m | not textbook; old wrong [1] |
| 2 | 64 | SpanTreeStEph.rs | verify_spanning_tree (T) | none | W V+E, S V+E | W V, S V | no cost; old wrong [2] |
| 3 | 64 | SpanTreeStEph.rs | verify_spanning_tree (I) | none | W V+E_tree | W V, S V | no textbook cost |
| 4 | 64 | SpanTreeMtEph.rs | spanning_tree_star_contraction_mt (T,I) | (n+m)lg n, lg² n | W (n+m) lg n | W nm+(n+m)lg² n | not textbook; old wrong [1] |
| 5 | 64 | SpanTreeMtEph.rs | verify_spanning_tree (T) | none | W V+E, S V+E | W V, S V | no cost; old wrong [2] |
| 6 | 64 | SpanTreeMtEph.rs | verify_spanning_tree (I) | none | W V+E_tree | W V, S V | no textbook cost |
| 7 | 64 | TSPApproxStEph.rs | euler_tour (T,I) | n, n | W n, S n | W n m, S n m | not textbook; old wrong [3] |
| 8 | 64 | TSPApproxStEph.rs | euler_tour_dfs | none | W n m_tree | W n m, S n m | no cost; old wrong [3] |
| 9 | 64 | TSPApproxStEph.rs | shortcut_tour (T,I) | n, n | W n, S n | W n exp, S n | matches textbook |
| 10 | 64 | TSPApproxStEph.rs | tour_weight (T,I) | n, n | W n, S n | W n m, S n m | not textbook; old wrong [4] |
| 11 | 64 | TSPApproxStEph.rs | approx_metric_tsp (T,I) | n+m, n+m | W n / n+m | W n m, S n m | not textbook; old wrong |
| 12 | 64 | TSPApproxStEph.rs | vec_contains_pair | none | W n, S n | W \|v\|, S \|v\| | no textbook cost |
| 13 | 64 | TSPApproxStEph.rs | get_neighbors, get_edge_weight | none | W m, S m | W m, S m | no textbook cost |

### Footnotes

1. The spanning tree is `star_contract` with an `expand` closure that
   (a) adds the edge (v, center) for each partition-map entry, O(n), and
   (b) for each edge of the quotient's spanning tree scans the whole
   edge list for an original edge between the two stars: O(q m) for q
   quotient-tree edges. Summed over rounds this is O(m Σ nᵢ): O(n m) when
   the vertex count shrinks geometrically (Mt, in expectation) and
   O(n² m) in the St worst case. The expand loop is sequential in both
   files, so its O(n m) also lands on the Mt span.
2. `verify_spanning_tree` checks the edge count (O(1)) and then that each
   tree edge is a graph edge (hash lookups). The trait lines claim a
   "connectivity check" costing O(|V| + |E|); there is none, and the loop
   runs over at most |V| − 1 edges.
3. `euler_tour_dfs` is called once per tree vertex. Each call runs `ng`
   (O(m)) and, for each graph neighbor, scans the visited-edge `Vec`
   (O(n)) and the tree-edge set (O(n)): O(m + deg(v) n) per vertex,
   O(n m) in total. On the complete graphs TSP assumes this is O(n³),
   against the textbook's O(n) DFS on the tree. Restricting the DFS to
   the tree's own adjacency would give O(n).
4. `tour_weight` calls `get_edge_label` once per tour step; each call
   scans all labeled edges: O(n m).

## 3. Counts (per annotation site)

20 new lines were added in 3 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 2 |
| 2 | does not match textbook | 10 |
| 3 | does not match old analysis | 13 |
| 4 | no textbook cost | 8 |
| 5 | unannotated functions | 3 |
| 6 | malformed annotations | 2 |

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 64 | SpanTreeStEph.rs | 4 | 0 | 2 | 3 | 2 |
| 2 | 64 | SpanTreeMtEph.rs | 4 | 0 | 2 | 3 | 2 |
| 3 | 64 | TSPApproxStEph.rs | 12 | 2 | 6 | 7 | 4 |

## 4. Unannotated functions (3)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 64 | SpanTreeStEph.rs | fmt (Display) |
| 2 | 64 | SpanTreeMtEph.rs | fmt (Display) |
| 3 | 64 | TSPApproxStEph.rs | fmt (Display) |

## 5. Malformed annotations (2)

| # | Chap | File | Function | Problem |
|---|---|---|---|---|
| 1 | 64 | SpanTreeStEph.rs | spanning_tree_star_contraction (I) | line `Code review (Claude Opus 4.6): Work O((n+m)` is truncated (no closing parenthesis, no span) |
| 2 | 64 | SpanTreeMtEph.rs | spanning_tree_star_contraction_mt (I) | same truncated line |

Both were left in place; the new line follows the last Alg Analysis line
of each block (after the continuation line in the Mt file).

## 6. Notable findings

1. **The TSP approximation is O(n m), which is O(n³) on the complete
   graphs it targets.** The Euler-tour DFS walks the whole graph's
   neighbor list (an O(m) scan per call) instead of the tree's adjacency,
   and `tour_weight` looks up each edge weight by scanning all edges. The
   textbook steps are O(n) each once the tree is given.
2. **The spanning-tree expand step is O(n m) per round.** It finds the
   original edge behind each quotient-tree edge by scanning the edge list;
   recording the original edge when the quotient graph is built (as the
   textbook's labeled edges do) would make it O(n).
3. The trait-level `verify_spanning_tree` annotations describe a
   connectivity check the code does not perform.
