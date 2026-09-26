# r228 Alg Analysis Review: Chap59

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap59.txt` (Johnson's algorithm) states one cost.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 59.1 (cost stated after it) | Johnson APSP: 1 Bellman-Ford + n parallel Dijkstras | m n lg n | m lg n |

The Bellman-Ford phase (O(n m) work, O(n lg n) span with sequences) and
the reweighting are subsumed in this bound. The files also carry an APAS
line "Alg 59.1: Work O(m), Span O(m)" on `reweight_graph`; the prose gives
no separate reweighting cost, but the line is counted as the textbook
cost for that function.

Notation: n = |V|, m = |E|, k = end − start (vertex range of one
`parallel_dijkstra_all` call). Callee costs used:

- Chap57 `dijkstra`: O(n m + m³) work and span (this batch's Chap57
  review: BinaryHeapPQ delete_min O(q²), arc-scanning neighbor lookup).
- Chap58 `bellman_ford`: O(n² m) on a graph with n vertices and m arcs;
  on the augmented graph (n + 1 vertices, m + n arcs) O(n² m + n³).
- Chap56 `AllPairsResultStEph*::set_distance` and `set_predecessor`
  clone the whole row, O(n) per cell; `new(n)` is O(n²).
- Chap19 `ArraySeqStEph::append` and `tabulate` are sequential; append
  clones every element.

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 59 | JohnsonStEphI64.rs | johnson_apsp (T, free fn) | mn lg n, m lg n | W mn lg n, S mn lg n | W n³+n²m+nm³ | not textbook; old wrong [1] |
| 2 | 59 | JohnsonStEphI64.rs | adjust_distance, reweight_edge | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 59 | JohnsonStEphI64.rs | build_vertex_set | none | W max_val | W max_val exp | no textbook cost |
| 4 | 59 | JohnsonStEphI64.rs | add_dummy_source | none | W n+m, S n+m | W n m, S n m | no cost; old wrong [2] |
| 5 | 59 | JohnsonStEphI64.rs | reweight_graph | m, m | W n+m, S n+m | W n+m, S n+m | not textbook [3] |
| 6 | 59 | JohnsonStEphI64.rs | create_negative_cycle_result | none | W n², S n² | W n², S n² | no textbook cost |
| 7 | 59 | JohnsonStEphF64.rs | (same 8 sites as rows 1–6) | as above | as above | as above | as above |
| 8 | 59 | JohnsonMtEphI64.rs | johnson_apsp (T, free fn) | mn lg n, m lg n | W mn lg n, S m lg n | W n³+n²m+nm³, S n³+n²m+m³ | not textbook; old wrong [4] |
| 9 | 59 | JohnsonMtEphI64.rs | adjust_distance | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 10 | 59 | JohnsonMtEphI64.rs | parallel_dijkstra_all | none | W k m lg n, S m lg n | [5] | no cost; old wrong [5] |
| 11 | 59 | JohnsonMtEphI64.rs | add_dummy_source | none | W n+m, S n+m | W n+m, S n+m | no textbook cost |
| 12 | 59 | JohnsonMtEphI64.rs | reweight_graph | m, m | W n+m, S n+m | W n+m, S n+m | not textbook [3] |
| 13 | 59 | JohnsonMtEphI64.rs | create_negative_cycle_result | none | W n², S n² | W n², S n² | no textbook cost |
| 14 | 59 | JohnsonMtEphF64.rs | (same 7 sites as rows 8–13) | as above | as above | as above | as above |

### Footnotes

1. St `johnson_apsp`: `add_dummy_source` O(n m) [2], Bellman-Ford on the
   augmented graph O(n² m + n³), `reweight_graph` O(n + m), then n
   sequential Dijkstras at O(n m + m³) each (O(n² m + n m³)), and n²
   `set_distance`/`set_predecessor` calls at O(n) each (O(n³)). Total
   O(n³ + n² m + n m³), sequential, so the span equals the work.
2. St `add_dummy_source` calls `out_neighbors_weighed(&u)` for every
   vertex u, and each call scans all m arcs: O(n m). The Mt version
   iterates `labeled_arcs()` once and is O(n + m).
3. `reweight_graph` also rebuilds the vertex set {0..n−1} (O(n) hash
   inserts) and inserts the arcs in a sequential loop, so O(n + m) work
   and span against the APAS line's O(m). The latest old line already
   said O(n + m).
4. Mt `johnson_apsp`: Bellman-Ford (sequential, O(n² m + n³)) runs before
   the parallel phase and dominates the span. The parallel phase is
   `parallel_dijkstra_all(0, n)` [5]. Work O(n³ + n² m + n m³) (the n³
   term is Bellman-Ford's), span O(n³ + n² m + m³). The old line claimed
   the textbook's O(m lg n) span.
5. `parallel_dijkstra_all` forks with `ParaPair!` over the vertex range.
   Each leaf runs one Dijkstra (O(n m + m³)) and a sequential tabulate of
   its row (O(n)). Each internal node clones the graph twice (O(n + m))
   and the potentials (O(n)), and `append` clones every n-length row of
   both halves (O(k n)). Work O(k (n m + m³) + k n lg k); span
   O(n m + m³ + (n + m) lg k + k n). The structure is the right
   divide-and-conquer shape; the costs come from Dijkstra and the row
   copies.

## 3. Counts (per annotation site)

30 new lines were added in 4 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 12 |
| 3 | does not match old analysis | 12 |
| 4 | no textbook cost | 18 |
| 5 | unannotated functions | 0 |
| 6 | malformed annotations | 0 |

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 59 | JohnsonStEphI64.rs | 8 | 0 | 3 | 3 | 5 |
| 2 | 59 | JohnsonStEphF64.rs | 8 | 0 | 3 | 3 | 5 |
| 3 | 59 | JohnsonMtEphI64.rs | 7 | 0 | 3 | 3 | 4 |
| 4 | 59 | JohnsonMtEphF64.rs | 7 | 0 | 3 | 3 | 4 |

## 4. Unannotated functions (0)

Every function in the four files has an annotation.

## 5. Malformed annotations (0)

None. `reweight_graph` carries an APAS line and two older Code-review
lines; the new line compares with the last one (O(n + m)). Several
helpers have a `// veracity: no_requires` line after the doc block; the
new line was placed directly after the last Alg Analysis line.

## 6. Notable findings

1. **Johnson inherits the defects of Chap57 and Chap58.** With
   Dijkstra at O(n m + m³) and Bellman-Ford at O(n² m), Johnson is
   O(n³ + n² m + n m³) instead of O(m n lg n). Fixing the PQ and the
   graph's neighbor lookup in those chapters would fix this one.
2. **The Mt span is set by the sequential Bellman-Ford.** The parallel
   Dijkstra phase has the right fork-join structure, but it runs after a
   sequential O(n² m + n³) Bellman-Ford, so the span cannot reach
   O(m lg n).
3. **The all-pairs result costs O(n) per cell write.** St `johnson_apsp`
   fills the n × n result with `set_distance`, which clones a row per
   call: O(n³) just to store the answer. The Mt version builds rows with
   tabulate and avoids this, but its `append` still copies rows at every
   level of the recursion.
