# r228 Alg Analysis Review: Chap54

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap54.txt` and `prompts/Chap54part2.txt` (Breadth-First Search).

| # | Source | Algorithm | Work | Span |
|---|---|---|---|---|
| 1 | Alg 54.3, §2.1 | Sequential BFS (queue, visited array) | \|V\| + \|E\| | = Work (sequential) |
| 2 | Alg 54.4, §3.1 | Parallel BFSReach (tree sets/tables) | \|E\| lg \|V\| | d lg² \|V\| |
| 3 | §3.1 per round | X ∪ F, N \ X | \|F\| lg n | lg n |
| 4 | §3.1 per round | N⁺(F) by reduce union | \|F\| lg n | lg² n |
| 5 | Alg 54.5 | BFSDistance | as Alg 54.4 | as Alg 54.4 |
| 6 | Alg 54.6, §4.1 | BFSTree with sequences | \|V\| + \|E\| | d lg \|V\| |

d = largest distance of a reachable vertex from the source. The prose
states that Alg 54.6 removes duplicate frontier vertices by an inject and
a select, so every vertex enters one frontier.

`BFSSpecsAndLemmas.rs` has only spec and proof functions and no
annotations.

Notation: σ = Σ over reachable v of the number of shortest s–v paths
(σ ≥ number of reachable vertices, and σ can be 2^Θ(|V|), e.g. a chain of
diamonds); F = frontier; r = output length.

## 2. Cost base

- Chap19 `ArraySeqStEph::set`, `ArraySeqMtEph::set`, `nth`, `length`:
  O(1). `from_vec`: O(1).
- `tabulate` in Chap19 `ArraySeqStEph`, `ArraySeqStPer`, `ArraySeqMtEph`
  and Chap18 `ArraySeqMtPer`: a sequential loop, Work = Span = O(n).
- `ArraySeqStPer::update` (Chap19) and `ArraySeqMtPer::update` (Chap18):
  a full tabulate copy, O(n) work and span.
- `std::collections::VecDeque` push/pop: O(1) amortized.

## 3. Reviewed functions

T = trait declaration, I = impl.

### 3a. BFSStEph.rs (8 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 54 | BFSStEph.rs | top_down_order (T,I) | none | 1, 1 | 1, 1 | no textbook cost |
| 2 | 54 | BFSStEph.rs | bottom_up_order (T,I) | none | n, n | n, n | no textbook cost |
| 3 | 54 | BFSStEph.rs | bfs (T) | 54.4: E lg V, d lg²V | V+E, V+E | V+E, V+E | not textbook [1] |
| 4 | 54 | BFSStEph.rs | bfs (I) | "54.6": V+E, V+E [2] | V+E, V+E | V+E, V+E | not textbook [1][2] |
| 5 | 54 | BFSStEph.rs | bfs_tree (T,I) | 54.6: V+E, d lg V | V+E, V+E | V+E, V+E | not textbook [1] |

### 3b. BFSStPer.rs (8 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 54 | BFSStPer.rs | top_down_order (T,I) | none | 1, 1 | 1, 1 | no textbook cost |
| 2 | 54 | BFSStPer.rs | bottom_up_order (T,I) | none | n, n | n, n | no textbook cost |
| 3 | 54 | BFSStPer.rs | bfs (T) | 54.4: E lg V, d lg²V | V+E, V+E | V²+E, V²+E | not textbook; old wrong [3] |
| 4 | 54 | BFSStPer.rs | bfs (I) | "54.6": V+E, V+E [2] | V+E, V+E | V²+E, V²+E | not textbook; old wrong [3] |
| 5 | 54 | BFSStPer.rs | bfs_tree (T,I) | 54.6: V+E, d lg V | V+E, V+E | V²+E, V²+E | not textbook; old wrong [3] |

### 3c. BFSMtEph.rs (12 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 54 | BFSMtEph.rs | top_down_order (T,I) | none | 1, 1 | 1, 1 | no textbook cost |
| 2 | 54 | BFSMtEph.rs | bottom_up_order (T,I) | none | n, n | n, n | no textbook cost |
| 3 | 54 | BFSMtEph.rs | copy_distances | none | V, V | V, V | no textbook cost |
| 4 | 54 | BFSMtEph.rs | copy_graph | none | V+E, V+E | V+E, V+E | no textbook cost |
| 5 | 54 | BFSMtEph.rs | process_frontier_parallel | none | F edges, deg lg F | F(V+E), (V+E) lg F | no cost; old wrong [4] |
| 6 | 54 | BFSMtEph.rs | process_frontier_tree_par. | none | F edges, deg lg F | F(V+E), (V+E) lg F | no cost; old wrong [4] |
| 7 | 54 | BFSMtEph.rs | bfs (T,I) | 54.4: E lg V, d lg²V | E lg V / V+E | σ(V+E), σ+d(V+E)lg σ | not textbook; old wrong [5] |
| 8 | 54 | BFSMtEph.rs | bfs_tree (T,I) | 54.6: V+E, d lg V | V+E, d lg V | V(V+E), d(V+E) lg V | not textbook; old wrong [6] |

### 3d. BFSMtPer.rs (12 lines)

Same structure as BFSMtEph, on Chap18 `ArraySeqMtPer`, whose `update`
copies the whole array.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 54 | BFSMtPer.rs | top_down_order (T,I) | none | 1, 1 | 1, 1 | no textbook cost |
| 2 | 54 | BFSMtPer.rs | bottom_up_order (T,I) | none | n, n | n, n | no textbook cost |
| 3 | 54 | BFSMtPer.rs | copy_distances | none | V, V | V, V | no textbook cost |
| 4 | 54 | BFSMtPer.rs | copy_graph | none | V+E, V+E | V+E, V+E | no textbook cost |
| 5 | 54 | BFSMtPer.rs | process_frontier_parallel | none | F edges, deg lg F | F(V+E), (V+E) lg F | no cost; old wrong [4] |
| 6 | 54 | BFSMtPer.rs | process_frontier_tree_par. | none | F edges, deg lg F | F(V+E), (V+E) lg F | no cost; old wrong [4] |
| 7 | 54 | BFSMtPer.rs | bfs (T,I) | 54.4: E lg V, d lg²V | E lg V / V+E | σ(V+E), σV+d(V+E)lg σ | not textbook; old wrong [5][7] |
| 8 | 54 | BFSMtPer.rs | bfs_tree (T,I) | 54.6: V+E, d lg V | V+E, d lg V | V(V+E), V²+d(V+E)lg V | not textbook; old wrong [6][7] |

### Footnotes

1. The St files run the §2.1 queue-based sequential BFS: a VecDeque
   frontier and an O(1)-`set` distance (or parent) array. That meets the
   §2.1 work bound O(|V| + |E|), but the functions are documented as Alg
   54.5 and Alg 54.6, whose spans are O(d lg² |V|) and O(d lg |V|).
2. The impl `bfs` carries its own APAS line "Ch54 Alg 54.6: Work
   O(|V| + |E|), Span O(|V| + |E|)". The function computes distances
   (Alg 54.5), and Alg 54.6's span is O(d lg |V|), not O(|V| + |E|). The
   APAS line misquotes the textbook.
3. Each distance or parent write is `ArraySeqStPerS::update`, a persistent
   copy of all |V| entries. Up to |V| writes cost O(|V|²).
4. At each of its |F| − 1 internal nodes, the divide-and-conquer helper
   calls `copy_graph` and `copy_distances` (sequential tabulates,
   O(|V| + |E|)) so that each branch owns a copy, then merges the halves
   with sequential push loops. Work O(|F|(|V| + |E|) + r lg |F|), span
   O((|V| + |E|) lg |F| + r). The old lines charged only the frontier's
   edges.
5. `process_frontier_parallel` emits every neighbor that was unreached at
   the start of the round, once per in-edge from the frontier, and `bfs`
   uses that list as the next frontier without deduplication. A vertex at
   distance i + 1 therefore appears once per shortest path, so the
   frontier sizes sum to σ, which is exponential in |V| on a chain of
   diamonds. Each frontier element also triggers an O(|V| + |E|) graph
   copy inside the D&C. The distances computed are still correct.
6. `bfs_tree` deduplicates (it checks the parent before accepting an
   update), so the frontiers sum to ≤ |V|; the per-internal-node graph
   copies still make the work O(|V| (|V| + |E|)).
7. In the MtPer file each distance or parent write is
   `ArraySeqMtPerS::update`, O(|V|), inside a sequential loop.

## 4. Counts (per annotation site)

40 new lines were added, one per annotated site, in 4 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 16 |
| 3 | does not match old analysis | 16 |
| 4 | no textbook cost | 24 |
| 5 | unannotated functions | 12 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 40 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 54 | BFSStEph.rs | 8 | 0 | 4 | 0 | 4 |
| 2 | 54 | BFSStPer.rs | 8 | 0 | 4 | 4 | 4 |
| 3 | 54 | BFSMtEph.rs | 12 | 0 | 4 | 6 | 8 |
| 4 | 54 | BFSMtPer.rs | 12 | 0 | 4 | 6 | 8 |

## 5. Unannotated functions (12)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 54 | BFSStEph.rs | BFSTreeS fmt x2; BFSStEph fmt |
| 2 | 54 | BFSStPer.rs | BFSTreeS fmt x2; BFSStPer fmt |
| 3 | 54 | BFSMtEph.rs | BFSTreeS fmt x2; BFSMtEph fmt |
| 4 | 54 | BFSMtPer.rs | BFSTreeS fmt x2; BFSMtPer fmt |

Spec and proof functions (including all of `BFSSpecsAndLemmas.rs`) were
not counted.

## 6. Malformed annotations (0)

None. The impl `bfs` APAS line in BFSStEph and BFSStPer misquotes the
textbook (footnote 2) but is well formed.

## 7. Notable findings

1. **Mt `bfs` does not deduplicate the frontier (true defect).** In
   BFSMtEph and BFSMtPer, a vertex enters the next frontier once per
   shortest path to it, so total frontier size is σ, exponential in |V|
   in the worst case (a chain of k diamonds gives 2^k copies of the last
   vertex). Alg 54.6's inject-then-select step, which the tree variant
   does implement, is missing from the distance variant.
2. **The Mt BFS copies the whole graph at every fork.** Both
   `process_frontier_*` helpers deep-copy the adjacency lists and the
   distance/parent array (sequential tabulates) at each internal node of
   the frontier split, and the outer loop copies them again per round.
   That makes `bfs_tree` O(|V|(|V| + |E|)) work and O(d (|V| + |E|) lg |V|)
   span against Alg 54.6's O(|V| + |E|) and O(d lg |V|). The old lines
   claimed the textbook costs.
3. **The Per files pay O(|V|) per write.** BFSStPer and BFSMtPer write
   distances and parents with persistent `update`, a full-array copy, so
   they are O(|V|²) even in the sequential case. BFSStEph matches the
   §2.1 sequential O(|V| + |E|) work; no file reaches the textbook span.
