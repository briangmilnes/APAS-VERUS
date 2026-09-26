# r228 Alg Analysis Review: Chap62

Reviewer: Claude Opus 5.5, 2026-09-26. The star-contraction files were
reviewed as they stand after round r224 (the progress fallback with
`find_non_loop_edge` and `single_edge_partition`).

## 1. Textbook cost specifications

`prompts/Chap62.txt` (Star contraction).

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Thm 62.1 | starPartition (Alg 62.3, array sequences) | n + m | lg n |
| 2 | Lemma 62.2 | expected satellites removed per round | ≥ n•/4 | — |
| 3 | Thm 62.3 | star contraction to isolated vertices (Alg 62.5) | (n + m) lg n | lg² n |

Theorem 62.3 assumes `base` has linear work and constant span, and
`expand` linear work and logarithmic span. The St files carry APAS lines
with span O(n + m) (partition) and O((n + m) lg n) (contraction); those
are the sequential versions of rows 1 and 3.

Notation: n = |V|, m = |E|, c = number of centers a partition produces
(c ≤ n), k = end − start of a divide-and-conquer range. Hash-map and
hash-set costs are expected costs. SetStEph `union` copies both inputs
sequentially, O(|a| + |b|) (Chap05 review).

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 62 | StarPartitionStEph.rs | sequential_star_partition (T,I) | n+m, n+m | W n+m, S n+m | W n + c m | not textbook; old wrong [1] |
| 2 | 62 | StarPartitionMtEph.rs | parallel_star_partition (T,I) | n+m, lg n | W (n+m)lg, S lg | W n lg n+m lg m, S n+m | not textbook; old wrong [2] |
| 3 | 62 | StarPartitionMtEph.rs | hash_coin | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 4 | 62 | StarPartitionMtEph.rs | hash_coin_flips_mt | none | W n, S lg n | W k lg k, S k | no cost; old wrong [2] |
| 5 | 62 | StarPartitionMtEph.rs | build_th_edges_mt | none | W m, S lg m | W k lg k, S k | no cost; old wrong [2] |
| 6 | 62 | StarPartitionMtEph.rs | build_p_vec_mt | none | W n, S lg n | W k lg k, S k | no cost; old wrong [2] |
| 7 | 62 | StarPartitionMtEph.rs | build_vertex_to_index_mt | none | W n lg n, S lg n | W k lg k, S k | no cost; old wrong [2] |
| 8 | 62 | StarPartitionMtEph.rs | build_satellite_map_mt | none | W m lg m, S lg m | W k lg k, S k | no cost; old wrong [2] |
| 9 | 62 | StarPartitionMtEph.rs | build_p_vec_with_inject_mt | none | W n, S lg n | W k lg k, S k | no cost; old wrong [2] |
| 10 | 62 | StarPartitionMtEph.rs | build_partition_map_mt | none | W n lg n, S lg n | W k lg k, S k | no cost; old wrong [2] |
| 11 | 62 | StarPartitionMtEph.rs | build_centers_mt | none | W n, S lg n | W k lg k, S k | no cost; old wrong [2] |
| 12 | 62 | StarContractionStEph.rs | star_contract (T,I) | (n+m)lg n, lg² n | W (n+m) lg n | W n² m, S n² m | not textbook; old wrong [3] |
| 13 | 62 | StarContractionStEph.rs | contract_to_vertices (T,I) | (n+m)lg n | W (n+m) lg n | W n² m, S n² m | not textbook; old wrong [3] |
| 14 | 62 | StarContractionStEph.rs | star_contract_rec | none | W (n+m) lg n | W n² m, S n² m | no cost; old wrong [3] |
| 15 | 62 | StarContractionStEph.rs | build_quotient_graph | none | W m, S m | W n+m, S n+m | no cost; old wrong [4] |
| 16 | 62 | StarContractionStEph.rs | find_non_loop_edge | none | W m, S m | W m, S m | no textbook cost |
| 17 | 62 | StarContractionStEph.rs | single_edge_partition | none | W n, S n | W n, S n | no textbook cost |
| 18 | 62 | StarContractionMtEph.rs | star_contract_mt (T,I) | (n+m)lg n, lg² n | W (n+m)lg n | [5] | not textbook; old wrong [5] |
| 19 | 62 | StarContractionMtEph.rs | contract_to_vertices_mt (T,I) | (n+m)lg n, lg² n | W (n+m)lg n | [5] | not textbook; old wrong [5] |
| 20 | 62 | StarContractionMtEph.rs | star_contract_mt_rec | none | W (n+m)lg n, S lg² n | [5] | no cost; old wrong [5] |
| 21 | 62 | StarContractionMtEph.rs | build_quotient_graph_parallel | none | W m, S lg m | W n+m lg m, same S | no cost; old wrong [6] |
| 22 | 62 | StarContractionMtEph.rs | find_non_loop_edge | none | W m, S m | W m, S m | no textbook cost |
| 23 | 62 | StarContractionMtEph.rs | single_edge_partition | none | W n, S n | W n, S n | no textbook cost |
| 24 | 62 | StarContractionMtEph.rs | route_edges_parallel | none | W k, S lg k | W k lg k, S k lg k | no cost; old wrong [6] |

### Footnotes

1. `sequential_star_partition` makes each unprocessed vertex a center and
   then scans the whole edge list to find its unprocessed neighbors:
   O(m) per center, O(n + c m) in total, O(n m) in the worst case (on a
   path about half the vertices become centers).
2. Every divide-and-conquer helper in `StarPartitionMtEph.rs` forks both
   halves with `ParaPair!`, but then merges the right result into the left
   one with a sequential loop (HashMap inserts, Vec pushes) or with
   `SetStEph::union`: O(k) per level. Hence W(k) = 2W(k/2) + O(k) =
   O(k lg k) and S(k) = S(k/2) + O(k) = O(k). `parallel_star_partition`
   chains six of them: Work O(n lg n + m lg m), Span O(n + m), against
   Thm 62.1's O(n + m) and O(lg n). The old lines had logarithmic spans.
3. The St contraction uses the greedy `sequential_star_partition`, which
   guarantees progress (at least one vertex per round) but no constant
   fraction: on a star whose satellites come first in the vertex order,
   each satellite becomes its own center and one round removes one
   vertex, so up to n rounds are possible. Each round costs O(n + c m)
   [1] plus the quotient build O(n + m), so the worst case is O(n² m)
   work, all sequential. The costs of `base` and `expand` are extra.
4. `build_quotient_graph` also clones the center set, O(n).
5. The Mt contraction uses the randomized `parallel_star_partition`, so
   Lemma 62.2 gives O(lg n) expected rounds. Per round: partition
   O(n lg n + m lg m) work and O(n + m) span [2]; quotient build
   O(n + m lg m) work and span [6]. Isolated vertices stay in the graph,
   so n does not shrink. Total Work O((n + m) lg² n) expected, Span
   O(n lg n + m lg² n) expected (plus base and expand), against Thm 62.3's
   O((n + m) lg n) and O(lg² n). The r224 line on `star_contract_mt_rec`
   (a round without progress adds O(n + m)) is correct and still applies.
6. `route_edges_parallel` is named parallel and its old line says
   "binary fork-join via ParaPair", but the body calls itself on the left
   half, then on the right half, with no `ParaPair!`, and merges with
   `SetStEph::union`: O(k lg k) work and span. `build_quotient_graph_parallel`
   adds `to_seq`, a clone of the partition map, and a clone of the center
   set: O(n + m lg m) work and span.

## 3. Counts (per annotation site)

30 new lines were added in 4 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 12 |
| 3 | does not match old analysis | 25 |
| 4 | no textbook cost | 18 |
| 5 | unannotated functions | 4 |
| 6 | malformed annotations | 0 |

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 62 | StarPartitionStEph.rs | 2 | 0 | 2 | 2 | 0 |
| 2 | 62 | StarPartitionMtEph.rs | 11 | 0 | 2 | 10 | 9 |
| 3 | 62 | StarContractionStEph.rs | 8 | 0 | 4 | 6 | 4 |
| 4 | 62 | StarContractionMtEph.rs | 9 | 0 | 4 | 7 | 5 |

## 4. Unannotated functions (4)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 62 | StarPartitionStEph.rs | fmt (Display) |
| 2 | 62 | StarPartitionMtEph.rs | fmt (Display) |
| 3 | 62 | StarContractionStEph.rs | fmt (Display) |
| 4 | 62 | StarContractionMtEph.rs | fmt (Display) |

## 5. Malformed annotations (0)

None. The r224 lines ("Code review (Claude Opus 5.5): ...") have no date;
they are well formed and were treated as the latest old analysis where
they are the last line.

## 6. Notable findings

1. **`route_edges_parallel` is sequential.** Despite its name and its old
   annotation ("binary fork-join via ParaPair"), it makes its two
   recursive calls one after the other. With the set-union merge the
   quotient build is O(m lg m) span per round, which alone rules out the
   O(lg² n) contraction span.
2. **The parallel star partition merges sequentially.** All eight
   divide-and-conquer helpers fork correctly but merge their halves with
   sequential loops, so each has linear span and an extra lg factor of
   work. A parallel merge (or building the maps from a tabulated array,
   as Alg 62.3 does with `inject`) would give Thm 62.1's O(n + m) work
   and O(lg n) span.
3. **The St contraction has no round bound.** The greedy partition only
   guarantees one vertex of progress per round, and each round scans the
   edge list once per center, so the worst case is O(n² m) rather than
   O((n + m) lg n). The old lines assumed the vertices halve each round.
