# r228 Alg Analysis Review: Chap52

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap52.txt` (Graph Representations) gives four cost tables, one
per representation, plus Example 52.3 for adjacency sequences.

| # | Source | Operation | Work | Span |
|---|---|---|---|---|
| 1 | CS 52.1 Edge Sets | map over vertices | n | lg n |
| 2 | CS 52.1 | map over edges | m | lg n |
| 3 | CS 52.1 | map over (out-)neighbors | m | lg n |
| 4 | CS 52.1 | (out-)degree | m | lg n |
| 5 | CS 52.1 | is-edge (u, v) | lg n | lg n |
| 6 | CS 52.1 | insert / delete vertex | lg n | lg n |
| 7 | CS 52.1 | insert / delete edge | lg n | lg n |
| 8 | CS 52.3 Adj Tables | map over vertices | n | lg n |
| 9 | CS 52.3 | map over edges | m | lg n |
| 10 | CS 52.3 | map over neighbors of v | lg n + d(v) | lg n |
| 11 | CS 52.3 | degree | lg n | lg n |
| 12 | CS 52.3 | is-edge | lg n | lg n |
| 13 | CS 52.3 | insert / delete vertex, edge | lg n | lg n |
| 14 | CS 52.5 Adj Sequence | map over vertices | n | 1 |
| 15 | CS 52.5 | map over edges | n + m | 1 |
| 16 | CS 52.5 | map over neighbors of v | d(v) | 1 |
| 17 | CS 52.5 | degree | 1 | 1 |
| 18 | CS 52.5 | is-edge (u, v) | d(u) | lg d(u) |
| 19 | CS 52.5 | insert / delete vertex (persistent) | n | 1 |
| 20 | CS 52.5 | insert / delete edge (persistent) | n | 1 |
| 21 | Ex 52.3 | out-neighbors of v (adj seq) | Θ(1) | Θ(1) |
| 22 | CS 52.6 Adj Matrix | map over vertices | n | 1 |
| 23 | CS 52.6 | map over edges | n² | 1 |
| 24 | CS 52.6 | map over neighbors of v | n | 1 |
| 25 | CS 52.6 | degree | n | lg n |
| 26 | CS 52.6 | is-edge | 1 | 1 |
| 27 | CS 52.6 | insert / delete vertex | n² | 1 |
| 28 | CS 52.6 | insert / delete edge (persistent) | n | 1 |

The prose notes that ephemeral (single-threaded) sequences improve the
adjacency-sequence and adjacency-matrix edge updates to O(d(u)) and O(1).
Many files' APAS lines cite costs the tables do not give (EdgeSet
delete_vertex O(m lg m); AdjTable num_edges O(1); AdjSeq from_seq O(1);
EdgeSetGraphMtEph out_neighbors Span O(m); AdjMatrix set_edge O(1)). The
verdicts compare against the file's APAS line where it exists, otherwise
against the CS row, otherwise "no textbook cost".

Notation: |V|, |E| = vertex and edge counts; d = out-degree of the vertex
in question; h_V, h_E, h_T, h_u = heights of the vertex set, edge set,
ordered table and u's neighbor set (the Chap41 ParamBST sets do not
rebalance, so heights are unbounded by lg). Callee costs used:

- Chap41 AVLTreeSet (ParamBST): find, insert, delete O(size·height)
  (expose deep-copies subtrees); filter O(n h² + ΣW); to_seq O(n h);
  persistent insert/delete add an O(n) clone; keys inserted in sorted
  order cost O(i) each.
- Chap42 TableStEph/StPer: unsorted Vec of pairs; find, find_ref O(|a|);
  insert and delete rebuild the array, cloning every pair (including the
  neighbor sets); domain O(|a|²).
- Chap43 OrderedTableMtPer: find, insert_wf, delete, first_key O(n h),
  and every expose also copies the stored neighbor sets.
- Chap18/19 ArraySeq tabulate: sequential O(n) push loop, including the
  Mt variants; nth, length, set O(1).

No Chap52 file calls `join`, spawns a thread, or uses HFScheduler. Every
Mt file is sequential, so every Span equals its Work.

## 2. Reviewed functions

T = trait declaration, I = impl; both got a line. Rows merge T and I
(and files) where the costs and verdicts agree. "old ✗" means the line
also carries "does not match old analysis".

### 2a. EdgeSetGraph* (4 files, 105 lines)

Vertices and edges are Chap41 AVLTreeSets of V and Pair<V,V>.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 52 | EdgeSetGraphStEph.rs, MtEph.rs | empty, from_vertices_and_edges, vertices, edges (T,I) | 1, 1 | 1, 1 | 1, 1 | matches textbook |
| 2 | 52 | EdgeSetGraphStPer.rs, MtPer.rs | empty, from_vertices_and_edges, vertices, edges (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 3 | 52 | EdgeSetGraph (all 4) | num_vertices, num_edges (T,I) | 1, 1 | 1, 1 | 1, 1 | matches textbook |
| 4 | 52 | EdgeSetGraph (all 4) | has_edge (T,I) | lg n, lg n | lg, lg | \|E\|h_E, same | not textbook; old ✗ |
| 5 | 52 | EdgeSetGraphStEph.rs, StPer.rs | out_neighbors, out_degree (T,I) | m, lg n | m lg, … | \|E\|h_E² + d², same | not textbook; old ✗ |
| 6 | 52 | EdgeSetGraphMtEph.rs, MtPer.rs | out_neighbors, out_degree (T,I) | m, lg n (MtEph: m) | m lg, … | \|E\|h_E + \|E\|lg\|E\| + d², same | not textbook; old ✗ [1] |
| 7 | 52 | EdgeSetGraphStEph.rs, MtEph.rs | insert_vertex (T,I) | lg n, lg n | lg, lg | \|V\|h_V, same | not textbook; old ✗ |
| 8 | 52 | EdgeSetGraphStPer.rs, MtPer.rs | insert_vertex (T,I) | lg n, lg n | lg, lg | \|V\|h_V + \|E\|, same | not textbook; old ✗ [2] |
| 9 | 52 | EdgeSetGraphStEph.rs, MtEph.rs | delete_vertex (T,I) | m lg m | m lg m | \|V\|h_V + \|E\|lg\|E\| + (1+deg v)\|E\|h_E | not textbook; old ✗ |
| 10 | 52 | EdgeSetGraphStPer.rs | delete_vertex (T,I) | m lg m | m lg m | \|V\|h_V + \|E\|h_E², same | not textbook; old ✗ |
| 11 | 52 | EdgeSetGraphMtPer.rs | delete_vertex (T,I) | m lg m | m lg m | \|V\|h_V + \|E\|², same | not textbook; old ✗ [3] |
| 12 | 52 | EdgeSetGraph (all 4) | insert_edge (T,I) | lg n, lg n | lg, lg | \|V\|h_V + \|E\|h_E, same | not textbook; old ✗ |
| 13 | 52 | EdgeSetGraphStEph.rs, MtEph.rs | delete_edge (T,I) | lg n, lg n | lg, lg | \|E\|h_E, same | not textbook; old ✗ |
| 14 | 52 | EdgeSetGraphStPer.rs, MtPer.rs | delete_edge (T,I) | lg n, lg n | lg, lg | \|V\| + \|E\|h_E, same | not textbook; old ✗ |
| 15 | 52 | EdgeSetGraphStPer.rs | Debug fmt | — | "depends on size" | \|V\|h_V + \|E\|h_E | no textbook cost |

[1] The Mt out_neighbors converts the whole edge set to a sequence and
scans it sequentially instead of filtering; the neighbor set is then built
by d inserts of sorted keys, O(i) each, so O(d²).
[2] Persistent vertex insert also clones the edge set, O(|E|).
[3] MtPer delete_vertex removes incident edges one at a time, each a
persistent delete that clones the edge set.

### 2b. AdjTableGraph* (3 files, 68 lines)

StEph/StPer store a Chap42 TableStEph/StPer<V, AVLTreeSet<V>>; MtPer
stores a Chap43 OrderedTableMtPer<V, AVLTreeSetMtPer<V>> with a cached
edge count.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 16 | 52 | AdjTableGraphStEph.rs | empty, from_table (T,I) | 1, 1 | 1, 1 | 1, 1 | matches textbook |
| 17 | 52 | AdjTableGraphStPer.rs, MtPer.rs | empty, from_table (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 18 | 52 | AdjTableGraph (all 3) | num_vertices (T,I) | 1, 1 | 1, 1 | 1, 1 | matches textbook |
| 19 | 52 | AdjTableGraphStEph.rs, StPer.rs | num_edges (T,I) | 1 (not in CS) | n, … | \|V\|², same | not textbook; old ✗ |
| 20 | 52 | AdjTableGraphMtPer.rs | num_edges (T,I) | 1 | 1 | 1, 1 (cached) | matches textbook |
| 21 | 52 | AdjTableGraphStEph.rs | vertices (T,I) | n, lg n | n lg n | \|V\|²h_V, same | not textbook; old ✗ |
| 22 | 52 | AdjTableGraphStPer.rs | vertices (T,I) | — | n lg n | \|V\|²h_V, same | no textbook cost; old ✗ |
| 23 | 52 | AdjTableGraphStEph.rs, StPer.rs | has_edge (T,I) | lg n, lg n | lg, lg | \|V\| + d h_u, same | not textbook; old ✗ |
| 24 | 52 | AdjTableGraphStEph.rs, StPer.rs | out_neighbors (T,I) | lg n + d, lg n | lg n + d | \|V\| + d, same | not textbook; old ✗ |
| 25 | 52 | AdjTableGraphStEph.rs, StPer.rs | out_degree (T,I) | lg n, lg n | lg, lg | \|V\|, \|V\| | not textbook; old ✗ |
| 26 | 52 | AdjTableGraphStEph.rs, StPer.rs | insert_vertex, insert_edge, delete_edge (T,I) | lg n, lg n | lg, lg | \|V\| + \|E\|, same | not textbook; old ✗ |
| 27 | 52 | AdjTableGraphStEph.rs, StPer.rs | delete_vertex (T,I) | lg n (isolated) | (n + m) … | \|V\|² + \|V\|\|E\|, same | not textbook; old ✗ |
| 28 | 52 | AdjTableGraphMtPer.rs | has_edge, out_neighbors, out_degree (T,I) | lg n (+d), lg n | lg n (+d) | (\|V\|+\|E\|)h_T, same | not textbook; old ✗ |
| 29 | 52 | AdjTableGraphMtPer.rs | insert_vertex, insert_edge, delete_edge (T,I) | lg n, lg n | lg, lg | (\|V\|+\|E\|)h_T, same | not textbook; old ✗ |
| 30 | 52 | AdjTableGraphMtPer.rs | delete_vertex (T,I) | lg n (isolated) | n lg n … | \|V\|(\|V\|+\|E\|)h_T, same | not textbook; old ✗ [4] |

[4] MtPer delete_vertex deletes v from the table and then, sequentially
for every remaining vertex, finds and rewrites its neighbor set; it also
recounts edges through the unannotated `count_table_edges`.

### 2c. AdjSeqGraph* (4 files, 76 lines)

A sequence of neighbor sequences (Chap19 ArraySeqStEph/MtEph for the Eph
files, Chap18 ArraySeqStPer/MtPer for the Per files) with a cached edge
count.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 31 | 52 | AdjSeqGraphStEph.rs | new (T,I) | n, 1 | n, n | \|V\|, \|V\| | matches textbook |
| 32 | 52 | AdjSeqGraphMtEph.rs, StPer.rs, MtPer.rs | new (T,I) | — | n, n | \|V\|, \|V\| | no textbook cost |
| 33 | 52 | AdjSeqGraphStEph.rs | from_seq (T,I) | 1, 1 | Θ(1) (T) | \|V\|, \|V\| | not textbook; old ✗ (T) |
| 34 | 52 | AdjSeqGraphMtEph.rs | from_seq (T,I) | — | n, n | \|V\|, \|V\| | no textbook cost |
| 35 | 52 | AdjSeqGraphStPer.rs, MtPer.rs | from_seq (T,I) | — | n + m | \|V\|, \|V\| | no textbook cost; old ✗ [5] |
| 36 | 52 | AdjSeqGraph (all 4) | num_vertices, num_edges, out_degree (T,I) | 1, 1 | 1, 1 | 1, 1 | matches textbook |
| 37 | 52 | AdjSeqGraph (all 4) | has_edge (T,I) | d, lg d | d, d | d, d | not textbook (span) |
| 38 | 52 | AdjSeqGraphStEph.rs, MtEph.rs | out_neighbors (T,I) | d, 1 | d, d | d, d | not textbook (tabulate copy) |
| 39 | 52 | AdjSeqGraphStPer.rs, MtPer.rs | out_neighbors (T,I) | d, 1 | 1 (T); d (I) | 1, 1 | matches textbook (Ex 52.3); old ✗ (I) |
| 40 | 52 | AdjSeqGraphStEph.rs | set_neighbors (T,I) | 1, 1 | 1, 1 | 1, 1 | matches textbook |
| 41 | 52 | AdjSeqGraphMtEph.rs | set_neighbors (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 42 | 52 | AdjSeqGraphStEph.rs | insert_edge, delete_edge (T,I) | n, 1 | n + deg(u) (T); d (I) | d, d | not textbook; old ✗ (T) [6] |
| 43 | 52 | AdjSeqGraphStEph.rs | set_edge (T) | n, 1 | n + d, n + d | d, d | not textbook; old ✗ [6] |
| 43a | 52 | AdjSeqGraphStEph.rs (I), MtEph.rs (T,I) | set_edge | n, 1 | d, d | d, d | not textbook [6] |
| 44 | 52 | AdjSeqGraphStPer.rs, MtPer.rs | insert_edge, delete_edge (T,I) | n, 1 | d, d | \|V\|+\|E\|, same | not textbook; old ✗ [7] |

[5] from_seq reads each row's length, not its elements.
[6] The ephemeral update rebuilds only u's row: work below the persistent
Θ(n), but span O(d) instead of O(1), because the rebuild is sequential.
[7] The persistent update rebuilds the whole outer sequence with a nested
sequential tabulate that deep-copies every row, not only row u.

### 2d. AdjMatrixGraph* (4 files, 72 lines)

A sequence of boolean rows plus cached n and edge count. All four files
share the same bodies up to the sequence type.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 45 | 52 | AdjMatrixGraph (all 4) | new, from_matrix (T,I) | — ("N/A" in StEph) | n², n² | \|V\|², \|V\|² | no textbook cost |
| 46 | 52 | AdjMatrixGraph (all 4) | num_vertices, num_edges, has_edge (T,I) | 1, 1 | 1, 1 | 1, 1 | matches textbook |
| 47 | 52 | AdjMatrixGraph (all 4) | out_neighbors (T,I) | n, 1 | n, n | \|V\|, \|V\| | not textbook (span) |
| 48 | 52 | AdjMatrixGraph (all 4) | out_degree (T,I) | n, lg n | n, n | \|V\|, \|V\| | not textbook (span) |
| 49 | 52 | AdjMatrixGraphStEph.rs, MtEph.rs | set_edge (T) | 1, 1 | 1, 1 "in-place" | \|V\|², \|V\|² | not textbook; old ✗ [8] |
| 50 | 52 | AdjMatrixGraphStEph.rs, MtEph.rs | set_edge (I) | 1, 1 | n², n² | \|V\|², \|V\|² | not textbook [8] |
| 51 | 52 | AdjMatrixGraphStPer.rs | set_edge (T) | 1, 1 | n, n | \|V\|², \|V\|² | not textbook; old ✗ [8] |
| 52 | 52 | AdjMatrixGraphStPer.rs | set_edge (I) | 1, 1 | n², n² | \|V\|², \|V\|² | not textbook [8] |
| 53 | 52 | AdjMatrixGraphMtPer.rs | set_edge (T,I) | n, 1 | n², n² | \|V\|², \|V\|² | not textbook [8] |
| 54 | 52 | AdjMatrixGraph (all 4) | complement (T,I) | n², 1 | n², n² | \|V\|², \|V\|² | not textbook (span) |

[8] set_edge rebuilds the entire matrix with a nested sequential
tabulate, copying all |V|² entries, in both the ephemeral and the
persistent files. The textbook's ephemeral cost is O(1) and its persistent
cost O(n). The ephemeral trait lines that say "in-place matrix update" are
wrong.

## 3. Counts (per annotation site)

| # | Chap | File | Lines added | Matches | Not textbook | No textbook cost | Old ✗ |
|---|---|---|---|---|---|---|---|
| 1 | 52 | EdgeSetGraphStEph.rs | 26 | 12 | 14 | 0 | 14 |
| 2 | 52 | EdgeSetGraphStPer.rs | 27 | 4 | 14 | 9 | 14 |
| 3 | 52 | EdgeSetGraphMtEph.rs | 26 | 12 | 14 | 0 | 14 |
| 4 | 52 | EdgeSetGraphMtPer.rs | 26 | 4 | 14 | 8 | 14 |
| 5 | 52 | AdjTableGraphStEph.rs | 24 | 6 | 18 | 0 | 18 |
| 6 | 52 | AdjTableGraphStPer.rs | 24 | 2 | 16 | 6 | 18 |
| 7 | 52 | AdjTableGraphMtPer.rs | 20 | 4 | 14 | 2 | 14 |
| 8 | 52 | AdjSeqGraphStEph.rs | 22 | 10 | 12 | 0 | 4 |
| 9 | 52 | AdjSeqGraphStPer.rs | 18 | 8 | 6 | 4 | 7 |
| 10 | 52 | AdjSeqGraphMtEph.rs | 18 | 6 | 6 | 6 | 0 |
| 11 | 52 | AdjSeqGraphMtPer.rs | 18 | 8 | 6 | 4 | 7 |
| 12 | 52 | AdjMatrixGraphStEph.rs | 18 | 6 | 8 | 4 | 1 |
| 13 | 52 | AdjMatrixGraphStPer.rs | 18 | 6 | 8 | 4 | 1 |
| 14 | 52 | AdjMatrixGraphMtEph.rs | 18 | 6 | 8 | 4 | 1 |
| 15 | 52 | AdjMatrixGraphMtPer.rs | 18 | 6 | 8 | 4 | 0 |
| | | **Total** | **321** | **100** | **166** | **55** | **127** |

Matches + not textbook + no textbook cost = 321 = lines added. "Old ✗"
overlaps the other three columns. `AdjTableGraphSpecsAndLemmas.rs` has no
Alg Analysis annotations. `git diff --numstat src/Chap52` shows only
additions, and every added line is a `///` comment.

## 4. Unannotated functions (48)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 52 | EdgeSetGraphStEph.rs | clone, fmt (Debug), fmt (Display) |
| 2 | 52 | EdgeSetGraphStPer.rs | clone, eq, fmt (Display) |
| 3 | 52 | EdgeSetGraphMtEph.rs | clone, default, fmt ×2 |
| 4 | 52 | EdgeSetGraphMtPer.rs | default, fmt ×2 |
| 5 | 52 | AdjTableGraphStEph.rs | fmt ×2 |
| 6 | 52 | AdjTableGraphStPer.rs | clone, fmt ×2 |
| 7 | 52 | AdjTableGraphMtPer.rs | count_table_edges, clone, fmt ×2 |
| 8 | 52 | AdjSeqGraphStEph.rs | clone, fmt ×2 |
| 9 | 52 | AdjSeqGraphStPer.rs | clone, eq, fmt ×2 |
| 10 | 52 | AdjSeqGraphMtEph.rs | clone, fmt ×2 |
| 11 | 52 | AdjSeqGraphMtPer.rs | clone, fmt ×2 |
| 12 | 52 | AdjMatrixGraphStEph.rs | clone, fmt ×2 |
| 13 | 52 | AdjMatrixGraphStPer.rs | clone, eq, fmt ×2 |
| 14 | 52 | AdjMatrixGraphMtEph.rs | clone, fmt ×2 |
| 15 | 52 | AdjMatrixGraphMtPer.rs | clone, fmt ×2 |

`count_table_edges` in AdjTableGraphMtPer.rs is the only algorithmic
helper without an annotation; it is called by delete_vertex and costs
O((|V|+|E|) h_T) per call.

## 5. Malformed annotations (5 sites)

| # | Chap | File | Function | Problem |
|---|---|---|---|---|
| 1 | 52 | AdjTableGraphStEph.rs | delete_vertex (T), line 155 | truncated: "Work O((n + m)" |
| 2 | 52 | AdjSeqGraphStEph.rs | insert_edge (T), lines 187-188 | truncated "Work O(n + deg(u)", then "Delegates to set_edge(u, v, true)." with no cost |
| 3 | 52 | AdjSeqGraphStEph.rs | delete_edge (T), lines 210-211 | truncated "Work O(n + deg(u)", then "Delegates to set_edge(u, v, false)." with no cost |
| 4 | 52 | EdgeSetGraphMtPer.rs | delete_vertex (I), line 364 | a `// Veracity: NEEDED proof block` line splits the APAS line from the Code review lines |
| 5 | 52 | AdjMatrixGraphStEph.rs | new (T), line 83 | "Code review ... no explicit cost in APAS — N/A" states no cost |

The new line at each site was added after the last Alg Analysis line.

## 6. Notable findings

1. **Tree- and table-based representations are far above the textbook.**
   Every EdgeSet and AdjTable operation that the textbook prices at lg n
   costs O(size·height) or worse, because the Chap41 AVLTreeSet does not
   rebalance and deep-copies on expose, and the Chap42 Table is an
   unsorted array whose updates clone every entry, including every
   neighbor set. For example, AdjTable insert_edge is O(|V|+|E|) and
   delete_vertex O(|V|² + |V||E|) against lg n; EdgeSet has_edge is
   O(|E| h_E) against lg n. The old lines almost all repeat the textbook
   (127 of 321 new lines disagree with the old line).
2. **No Chap52 Mt file is parallel.** None of the eight Mt files forks.
   Scans, tabulates and filters are sequential, so every span equals its
   work: AdjSeq has_edge has span O(d) against lg d, AdjMatrix out_degree
   O(|V|) against lg n, complement O(|V|²) against 1.
3. **Edge updates copy far more than they should.**
   - AdjMatrix set_edge rebuilds the whole |V|×|V| matrix in all four
     files, including the ephemeral ones whose trait line says "in-place
     matrix update, O(1)".
   - The persistent AdjSeq insert_edge and delete_edge deep-copy every
     row, O(|V|+|E|) against the textbook's O(n); the old lines say O(d).
   - In the other direction, persistent AdjSeq out_neighbors returns a
     reference in O(1), which matches Ex 52.3; the old impl lines say O(d).
