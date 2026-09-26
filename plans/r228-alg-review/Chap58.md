# r228 Alg Analysis Review: Chap58

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap58.txt` (Bellman-Ford) analyzes Alg 58.2 under two
representations.

| # | Spec | Representation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 58.2, Sec. 3 | tree tables (graph, distances) | n m lg n | n lg n |
| 2 | Alg 58.2, Sec. 3 | array sequences (enumerable graph) | n m | n lg n |

Per round the textbook tabulates over the vertices in parallel and, for
each vertex, maps over its in-neighbors: O(n + m) work and O(lg n) span
with sequences; at most n rounds.

Notation: n = |V|, m = |E|.

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 58 | BellmanFordStEphI64.rs | bellman_ford (T, free fn) | nm, n lg n | W nm, S nm | W n² m, S n² m | not textbook; old wrong [1] |
| 2 | 58 | BellmanFordStEphI64.rs | clamp_weight, add_distance | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 58 | BellmanFordStEphI64.rs | reconstruct_predecessors | none | W nm, S nm | W n m, S n m | no textbook cost |
| 4 | 58 | BellmanFordStEphF64.rs | bellman_ford (T, free fn) | nm, n lg n | W nm, S nm | W n² m, S n² m | not textbook; old wrong [1] |
| 5 | 58 | BellmanFordStEphF64.rs | reconstruct_predecessors | none | W nm, S nm | W n m, S n m | no textbook cost |

### Footnotes

1. Each round loops over the n vertices and calls
   `WeightedDirGraphStEph*::in_neighbors_weighed(&v)`, which iterates over
   the whole labeled-arc set to collect v's in-arcs: O(m) per call
   (Chap06 review). A round is therefore O(n m), not O(n + m), and n
   rounds give O(n² m). All loops are sequential (`Vec::push`), so the
   span equals the work instead of the textbook's O(n lg n). The old line
   was right about the round count but assumed O(m) per round.
   `reconstruct_predecessors` repeats the same scan once: O(n m).

## 3. Counts (per annotation site)

8 new lines were added in 2 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 4 |
| 3 | does not match old analysis | 4 |
| 4 | no textbook cost | 4 |
| 5 | unannotated functions | 4 |
| 6 | malformed annotations | 0 |

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 58 | BellmanFordStEphI64.rs | 5 | 0 | 2 | 2 | 3 |
| 2 | 58 | BellmanFordStEphF64.rs | 3 | 0 | 2 | 2 | 1 |

## 4. Unannotated functions (4)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 58 | BellmanFordStEphI64.rs | fmt x2 (BellmanFordError) |
| 2 | 58 | BellmanFordStEphF64.rs | fmt x2 (BellmanFordError) |

## 5. Malformed annotations (0)

None. Each `bellman_ford` site carries two APAS lines (the table and the
sequence costs); the Code-review line is compared with the sequence cost
(W n m, S n lg n), which is the representation the code aims at.

## 6. Notable findings

1. **The graph representation costs a factor of n.** `in_neighbors_weighed`
   scans every arc, so each Bellman-Ford round is O(n m) and the whole
   algorithm O(n² m), not O(n m). An in-neighbor table (the textbook's
   adjacency sequence) would restore O(n m).
2. **Bellman-Ford is sequential.** The textbook round is a parallel
   tabulate with O(lg n) span; here both the vertex loop and the
   neighbor loop are sequential, so span = work.
3. The early exit (`!changed`) and the negative-cycle test after n rounds
   follow Alg 58.2.
