# r228 Alg Analysis Review: Chap65

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap65.txt` (Sequential MST algorithms).

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 65.1, "Cost of Prim's" | Prim with a binary-heap PQ | m lg n | m lg n |
| 2 | Alg 65.1 | Prim with a Fibonacci heap | m + n lg n | same |
| 3 | Alg 65.2, "Cost of Kruskal's" | sort edges + union/find | m lg n | m lg n (no parallelism) |
| 4 | Alg 65.2 | union, find each | lg n | lg n |

Notation: n = |V|, m = |E|, q = entries in the priority queue, |MST| =
number of tree edges (≤ n − 1). Callee costs:

- Chap45 `BinaryHeapPQ`: `delete_min` O(q²), `insert` O(q) (persistent
  array rebuilt by one-element appends; Chap45 review).
- Chap06 `LabUnDirGraphStEph::ng` and `get_edge_label`: O(m) each (scan of
  all labeled edges).
- `UnionFindPCStEph` (this chapter, union by rank and path compression):
  find and union O(lg n).

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 65 | PrimStEph.rs | prim_mst (T) | m lg n | W m lg n, S m lg n | W n m + m³ | not textbook; old wrong [1] |
| 2 | 65 | PrimStEph.rs | prim_mst (free fn) | m lg n | W m² lg n | W n m + m³ | not textbook; old wrong [1] |
| 3 | 65 | PrimStEph.rs | mst_weight (T, free fn) | none | W m / \|MST\| | W \|MST\|, S \|MST\| | no textbook cost |
| 4 | 65 | PrimStEph.rs | pq_entry_new, cmp, partial_cmp | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 5 | 65 | KruskalStEph.rs | kruskal_mst | m lg n | W m lg n, S m lg n | W m² + m lg n | not textbook; old wrong [2] |
| 6 | 65 | KruskalStEph.rs | mst_weight | none | "matches APAS" | W \|MST\|, S \|MST\| | no textbook cost |
| 7 | 65 | KruskalStEph.rs | verify_mst_size | none | "matches APAS" | W 1, S 1 | no textbook cost |

### Footnotes

1. `prim_mst` is Alg 65.1 with lazy deletion (one PQ insert per directed
   edge, up to 2m + 1 entries, no decrease-key). Costs: up to 2m + 1
   `delete_min` calls at O(q²) each, O(m³); up to 2m `insert` calls at
   O(q), O(m²); for each of the n visited vertices one `ng` (O(m)) and one
   `get_edge_label` for the parent edge (O(m)), O(n m); for each neighbor
   a `get_edge_label` (O(m)), O(m²) summed over all neighbors. Total
   O(n m + m³), sequential. The old impl line found the O(m) graph
   lookups (O(m² lg n)) but kept the textbook's O(lg n) heap operations.
2. `sort_edges_by_weight` is a selection sort (O(m²)); the textbook sorts
   in O(m lg n). The rest (building the union-find, copying the edges,
   and the greedy phase with O(lg n) find/union per edge) is
   O(n + m lg n).

## 3. Counts (per annotation site)

10 new lines were added in 2 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 3 |
| 3 | does not match old analysis | 3 |
| 4 | no textbook cost | 7 |
| 5 | unannotated functions | 49 |
| 6 | malformed annotations | 0 |

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 65 | PrimStEph.rs | 7 | 0 | 2 | 2 | 5 |
| 2 | 65 | KruskalStEph.rs | 3 | 0 | 1 | 1 | 2 |
| 3 | 65 | UnionFindPCStEph.rs | 0 | 0 | 0 | 0 | 0 |
| 4 | 65 | UnionFindNoPCStEph.rs | 0 | 0 | 0 | 0 | 0 |
| 5 | 65 | UnionFindArrayStEph.rs | 0 | 0 | 0 | 0 | 0 |

## 4. Unannotated functions (49)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 65 | PrimStEph.rs | TotalOrder cmp, clone, fmt x3 |
| 2 | 65 | KruskalStEph.rs | kruskal_mst, mst_weight, verify_mst_size (trait decls) |
| 3 | 65 | KruskalStEph.rs | kruskal_process_edge, kruskal_greedy_phase, sort_edges_by_weight |
| 4 | 65 | UnionFindPCStEph.rs | new, insert, find_root, find, union, equals, size (T,I); fmt x2 |
| 5 | 65 | UnionFindNoPCStEph.rs | new, insert, find, union_sets, equals, size (T,I); fmt x2 |
| 6 | 65 | UnionFindArrayStEph.rs | new, find, union, num_sets, size (T,I) |

The three union-find files carry no Alg Analysis annotations at all,
although union and find are the operations whose O(lg n) cost the
textbook's Kruskal bound depends on. The Kruskal trait declarations carry
free-text "APAS: ..." lines but no Alg Analysis lines.

## 5. Malformed annotations (0)

None is malformed in the plan's sense. Nonstandard lines, left in place:

- `KruskalStEph.rs` has three legacy `/// - Claude-Opus-4.6: ...` cost
  lines (on `kruskal_mst`, `mst_weight`, `verify_mst_size`). The new line
  was placed after the last Alg Analysis line, before the legacy line.
- `KruskalStEph.rs` `mst_weight` and `verify_mst_size` have
  `Alg Analysis: APAS: (no cost stated)` lines, which Standard 25 says
  not to write, and Code-review lines that say only "matches APAS".

## 6. Notable findings

1. **Kruskal sorts with a selection sort.** `sort_edges_by_weight` is
   O(m²), so Kruskal is O(m²) instead of O(m lg n). The old line said
   "matches APAS". Any O(m lg m) sort would restore the textbook bound.
2. **Prim is O(n m + m³).** It uses the persistent `BinaryHeapPQ`
   (delete_min O(q²)) and the edge-list graph (O(m) lookups). The old
   impl line saw the graph cost but not the heap cost.
3. **The union-find structures are unannotated.** `UnionFindPCStEph`
   (used by Kruskal), `UnionFindNoPCStEph`, and `UnionFindArrayStEph` have
   no cost annotations; the PC version uses union by rank with path
   compression, so its find and union are O(lg n) as the textbook
   assumes.
