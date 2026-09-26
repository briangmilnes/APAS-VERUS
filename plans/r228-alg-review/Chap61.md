# r228 Alg Analysis Review: Chap61

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap61.txt` (Edge contraction) states few costs explicitly.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 61.3 | greedy vertex matching (sequential, one pass) | \|E\| (file APAS line) | \|E\| |
| 2 | Alg 61.4 | parallel vertex matching, one round | \|E\| (file APAS line) | lg \|V\| (file APAS line) |
| 3 | Alg 61.6, analysis | edge contraction of a cycle, all rounds | n | lg² n |
| 4 | Sec. 1 | maximum vertex matching (not implemented) | √\|V\| \|E\| | — |

The prose gives the cycle-graph bound (row 3) and says only that the
greedy matching is sequential. The per-round costs in rows 1–2, and
O(|V| + |E|) work with O(lg |V|) span for one round of contraction, are
the files' own APAS lines; they are the natural per-round costs of Alg
61.4 (a coin per edge, each edge inspects only its incident edges) and
are used as the textbook costs below.

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 61 | VertexMatchingStEph.rs | greedy_matching (T,I) | E, E | W E, S E | W E exp, S E | matches textbook |
| 2 | 61 | VertexMatchingStEph.rs | parallel_matching_st (T,I) | E, E | W E², S E² | W E², S E² | not textbook [1] |
| 3 | 61 | EdgeContractionStEph.rs | edge_contract (T,I) | n, lg² n | W V+E, S V+E | W V+E, S V+E | not textbook [2] |
| 4 | 61 | EdgeContractionStEph.rs | contract_round (T,I) | V+E, V+E | W V+E, S V+E | W V+E, S V+E | matches textbook |
| 5 | 61 | VertexMatchingMtEph.rs | parallel_matching_mt (T) | E, lg V | W E, S lg V | W E², S V+E | not textbook; old wrong [3] |
| 6 | 61 | VertexMatchingMtEph.rs | parallel_matching_mt (I) | E, lg V | W E², S E | W E², S V+E | not textbook; old wrong [3] |
| 7 | 61 | VertexMatchingMtEph.rs | flip_coins_parallel | E, 1 | W E, S E | W E, S E | not textbook [4] |
| 8 | 61 | VertexMatchingMtEph.rs | select_edges_parallel | E, lg V | W E², S lg E+E | W E², S V+E | not textbook; old wrong [3] |
| 9 | 61 | VertexMatchingMtEph.rs | select_edges_recursive | none | W k E, S lg k+E | W kE+k lg k, S E+k | no textbook cost |
| 10 | 61 | VertexMatchingMtEph.rs | should_select_edge | deg u+deg v | W E, S E | W E, S E | not textbook [1] |
| 11 | 61 | EdgeContractionMtEph.rs | edge_contract_mt (T) | V+E, lg V | W V+E, S lg V | W V+E lg E, S V+E | not textbook; old wrong [5] |
| 12 | 61 | EdgeContractionMtEph.rs | edge_contract_mt (I) | V+E, lg V | W V+E, S V+E | W V+E lg E, S V+E | not textbook; old wrong [5] |
| 13 | 61 | EdgeContractionMtEph.rs | build_edges_parallel | none | W E, S lg E | W k lg k, S k | no cost; old wrong [5] |
| 14 | 61 | EdgeContractionMtEph.rs | contract_round_mt (T) | V+E, lg V | W V+E, S lg V | W V+E², S V+E | not textbook; old wrong |
| 15 | 61 | EdgeContractionMtEph.rs | contract_round_mt (I) | V+E, lg V | W E², S E | W V+E², S V+E | not textbook; old wrong |

E = |E|, V = |V|; k = end − start. Hash-set and hash-map costs are
expected costs.

### Footnotes

1. The Alg 61.4 selection test asks whether every edge incident on u or v
   flipped tails. `parallel_matching_st` and `should_select_edge` answer
   it by scanning the whole edge list (with `incident`, O(1) per edge)
   for every edge whose coin is heads: O(|E|) per edge, O(|E|²) per
   round, against O(deg u + deg v) per edge in the textbook.
2. `edge_contract` performs one contraction step for a given matching
   (block map, new vertex set, rerouted edges), sequentially, O(|V| + |E|).
   The APAS line cites the whole recursive contraction of a cycle
   (Work O(n), Span O(lg² n)); the file has no recursive driver.
3. `parallel_matching_mt` = `to_seq` O(|E|) + `flip_coins_parallel`
   O(|E|) sequential + `select_edges_parallel`. The latter builds the coin
   map sequentially (O(|E|)), clones the graph (O(|V| + |E|)), runs
   `select_edges_recursive` (O(|E|²) work, O(|E|) span: leaves call
   `should_select_edge`, O(|E|) each; merges copy O(k)), and rebuilds the
   result set sequentially. Work O(|E|²), span O(|V| + |E|). The old impl
   line had O(|E|) span; the graph clone adds |V|, which dominates when
   |V| > |E|. The trait line claimed the textbook cost.
4. `flip_coins_parallel` draws from one seeded RNG in a sequential loop.
5. `build_edges_parallel` forks with `ParaPair!` and merges the two
   halves with `SetStEph::union`, which copies both sets sequentially:
   W(k) = 2W(k/2) + O(k) = O(k lg k), S(k) = S(k/2) + O(k) = O(k).
   `edge_contract_mt` adds the sequential block map and vertex set,
   O(|V| + |E|).

## 3. Counts (per annotation site)

19 new lines were added in 4 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 4 |
| 2 | does not match textbook | 13 |
| 3 | does not match old analysis | 8 |
| 4 | no textbook cost | 2 |
| 5 | unannotated functions | 4 |
| 6 | malformed annotations | 0 |

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 61 | VertexMatchingStEph.rs | 4 | 2 | 2 | 0 | 0 |
| 2 | 61 | EdgeContractionStEph.rs | 4 | 2 | 2 | 0 | 0 |
| 3 | 61 | VertexMatchingMtEph.rs | 6 | 0 | 5 | 3 | 1 |
| 4 | 61 | EdgeContractionMtEph.rs | 5 | 0 | 4 | 5 | 1 |

## 4. Unannotated functions (4)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 61 | VertexMatchingStEph.rs | fmt (Display) |
| 2 | 61 | VertexMatchingMtEph.rs | fmt (Display) |
| 3 | 61 | EdgeContractionStEph.rs | fmt (Display) |
| 4 | 61 | EdgeContractionMtEph.rs | fmt (Display) |

## 5. Malformed annotations (0)

None. Three impl annotations continue on a second `///   ` line; the new
line was placed after the continuation, at the end of that Alg Analysis
entry.

## 6. Notable findings

1. **The matching test scans every edge.** Both the St and Mt parallel
   matchings check "all incident edges flipped tails" by iterating over
   the whole edge set, so one round is O(|E|²) instead of O(|E|). An
   incidence table (edges by endpoint) would give the textbook bound.
2. **Mt matching and contraction are mostly sequential.** Coin flips use
   one RNG in a loop, the coin map, block map, and result sets are built
   sequentially, and the edge-rebuild merges hash sets with a sequential
   union: span O(|V| + |E|) instead of O(lg |V|).
3. **No recursive contraction driver exists.** Chap61 implements one
   round (`contract_round`, `contract_round_mt`); the APAS Alg 61.6 bound
   (O(n) work, O(lg² n) span over O(lg n) rounds on a cycle) is cited on
   the single-step `edge_contract`, which it does not describe.
