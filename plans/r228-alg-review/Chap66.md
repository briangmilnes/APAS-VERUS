# r228 Alg Analysis Review: Chap66

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap66.txt` (Parallel MST: Boruvka).

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 66.1 with tree contraction | Boruvka | m lg n | lg³ n |
| 2 | Alg 66.3 (star contraction on bridges) | Boruvka MST | m lg n | lg² n |
| 3 | Alg 66.3, vertexBridges | reduce of singleton tables | m | lg m |
| 4 | Alg 66.2/66.3, bridgeStarPartition | coin flips + filter | n | lg n |
| 5 | Lemma 66.1 | expected vertices removed per round | ≥ n/4 | — |

Notation: n = |V|, m = |E|, k = end − start of a divide-and-conquer
range. Hash-map and hash-set costs are expected costs. The O(lg n) round
bound (Lemma 66.1) needs fair independent coins.

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 66 | BoruvkaStEph.rs | coin_flip | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 66 | BoruvkaStEph.rs | vertex_bridges (T,I) | m, lg m | W m, S m | W m, S m | not textbook (span) |
| 3 | 66 | BoruvkaStEph.rs | bridge_star_partition (T,I) | n, lg n | W n, S n | W n, S n | not textbook (span) |
| 4 | 66 | BoruvkaStEph.rs | boruvka_mst (T,I) | m lg n, lg³/lg² n | W m lg n, S m lg n | W (n+m) lg n, same S | not textbook (span) [1] |
| 5 | 66 | BoruvkaStEph.rs | boruvka_mst_with_seed (T,I) | m lg n, lg² n | W m lg n, S m lg n | W (n+m) lg n, same S | not textbook (span) [1] |
| 6 | 66 | BoruvkaStEph.rs | mst_weight (T) | none | W m, S m | W m, S m | no textbook cost |
| 7 | 66 | BoruvkaMtEph.rs | vertex_bridges_mt (T,I) | m, lg m | W m, S lg m | W m lg m, S m | not textbook; old wrong [2] |
| 8 | 66 | BoruvkaMtEph.rs | bridge_star_partition_mt (T,I) | n, lg n | W n, S lg n | W n lg n, S n | not textbook; old wrong [2] |
| 9 | 66 | BoruvkaMtEph.rs | boruvka_mst_mt (T,I) | m lg n, lg² n | W m lg n, S lg² n | W (n+m) lg² n, S (n+m) lg n | not textbook; old wrong [3] |
| 10 | 66 | BoruvkaMtEph.rs | boruvka_mst_mt_with_seed (T,I) | m lg n, lg² n | W m lg n, S lg² n | W (n+m) lg² n, S (n+m) lg n | not textbook; old wrong [3] |
| 11 | 66 | BoruvkaMtEph.rs | mst_weight (T,I) | none | W m, S m | W m, S m | no textbook cost |
| 12 | 66 | BoruvkaMtEph.rs | hash_coin | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 13 | 66 | BoruvkaMtEph.rs | hash_coin_flips_mt, compute_remaining_mt | none | W n, S lg n | W k lg k, S k | no cost; old wrong [2] |
| 14 | 66 | BoruvkaMtEph.rs | collect_mst_labels_mt, build_partition_map_mt | none | W n, S lg n | W k lg k, S k | no cost; old wrong [2] |
| 15 | 66 | BoruvkaMtEph.rs | filter_tail_to_head_mt | none | W n, S lg n | W k lg k, S k | no cost; old wrong [2] |
| 16 | 66 | BoruvkaMtEph.rs | reroute_edges_mt | none | W m, S lg m | W k lg k, S k | no cost; old wrong [2] |

### Footnotes

1. The St Boruvka rounds cost O(n + m) each (bridges O(m), partition
   O(n), label and partition-map loops O(n), edge reroute O(m)), all
   sequential. With O(lg n) rounds the work is O((n + m) lg n), which is
   the textbook's O(m lg n) for connected graphs; the span equals the
   work. The coins are `(seed ^ index) & 1`, the parity of each vertex's
   position in the set's iteration order, with the same seed every
   round; Lemma 66.1's expectation needs independent fair coins, so the
   O(lg n) round bound rests on the hash-set iteration order behaving
   randomly. The Mt file hashes (seed, round, index) instead.
2. Every Mt divide-and-conquer helper forks with `ParaPair!` and then
   merges the right result into the left one with a sequential loop (map
   inserts or Vec pushes), O(k) per level: W(k) = O(k lg k), S(k) = O(k).
   `vertex_bridges_mt` therefore costs O(m lg m) work and O(m) span, and
   `bridge_star_partition_mt` O(n lg n) work and O(n) span (it also
   builds the remaining set and copies the partition map sequentially).
3. Per round `boruvka_mst_mt` costs O(n lg n + m lg m) work and O(n + m)
   span [2], plus sequential O(n) loops for partition keys and labels.
   Over O(lg n) expected rounds: Work O((n + m) lg² n), Span
   O((n + m) lg n). Parallel edges between the same pair of super-vertices
   are kept (only self-loops are dropped), as the textbook's multigraph
   variant allows.

## 3. Counts (per annotation site)

27 new lines were added in 2 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 16 |
| 3 | does not match old analysis | 14 |
| 4 | no textbook cost | 11 |
| 5 | unannotated functions | 17 |
| 6 | malformed annotations | 0 |

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 66 | BoruvkaStEph.rs | 10 | 0 | 8 | 0 | 2 |
| 2 | 66 | BoruvkaMtEph.rs | 17 | 0 | 8 | 14 | 9 |

## 4. Unannotated functions (17)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 66 | BoruvkaStEph.rs | mst_weight (impl; has only a "- Sequential:" line) |
| 2 | 66 | BoruvkaStEph.rs | clone, eq, partial_cmp, cmp, hash, fmt x3 |
| 3 | 66 | BoruvkaMtEph.rs | clone, eq, partial_cmp, cmp, hash, fmt x3 |

## 5. Malformed annotations (0)

None. The St impl functions carry an extra `/// - Sequential: Work ...,
Span ...` bullet after their Alg Analysis lines; it is not in Alg Analysis
form. The new line was placed after the last Alg Analysis line, before
that bullet. The St impl Code-review lines give work only ("Work O(m) —
ACCEPTED DIFFERENCE"); the span comparison used the "Sequential" bullet.

## 6. Notable findings

1. **The Mt Boruvka helpers merge sequentially.** All eight
   divide-and-conquer helpers have linear span and an extra lg factor of
   work because the two halves are combined with a sequential loop. The
   whole algorithm is O((n + m) lg² n) work and O((n + m) lg n) span,
   against O(m lg n) and O(lg² n). The same pattern appears in Chap62's
   star partition.
2. **The St coins are positional parities.** `coin_flip` returns the
   parity of `seed ^ index` with a fixed seed, so the expected-rounds
   argument of Lemma 66.1 does not apply directly; it relies on the hash
   set's iteration order. The Mt version hashes the round number into
   each coin.
3. The St Boruvka is otherwise faithful to Alg 66.3: O(n + m) per round
   and the textbook's O(m lg n) work for connected graphs, sequential.
