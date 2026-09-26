# r228 Alg Analysis Review: Chap57

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap57.txt` (Dijkstra's algorithm) gives one cost analysis.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 57.2, Sec. 3 | Dijkstra with PQ (insert, deleteMin O(lg n)), tree tables | m lg n | m lg n (sequential) |
| 2 | Sec. 3 remark | Dijkstra with decreaseKey PQ | m + n lg n | same as work |

The prose assumes `PQ.insert` and `PQ.deleteMin` cost O(lg n), and that
the out-neighbors of a vertex are found in O(lg n + d(v)) from a tree
table. The stack (`StackStEph.rs`) has no textbook cost.

Notation: n = |V|, m = |E|, q = number of entries in the priority queue
(q ≤ m + 1 here).

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 57 | StackStEph.rs | new, peek, is_empty, size (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 57 | StackStEph.rs | push (T,I) | none | W 1 am, S 1 am | W 1 am, S 1 am | no textbook cost |
| 3 | 57 | StackStEph.rs | pop (T,I) | none | W 1 (am on T) | W 1, S 1 | no textbook cost |
| 4 | 57 | DijkstraStEphU64.rs | dijkstra (T, free fn) | m lg n, m lg n | W m lg n | W n m + m³ | not textbook; old wrong [1] |
| 5 | 57 | DijkstraStEphU64.rs | pq_entry_new | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 6 | 57 | DijkstraStEphF64.rs | dijkstra (T, free fn) | m lg n, m lg n | W m lg n | W n m + m³ | not textbook; old wrong [1] |
| 7 | 57 | DijkstraStEphF64.rs | pq_entry_new | none | W 1, S 1 | W 1, S 1 | no textbook cost |

The Dijkstra trait is declared but the algorithm is the free function
`dijkstra`; both carry annotations, and each got a line.

### Footnotes

1. The algorithm's shape is Alg 57.2 (lazy deletion: at most one PQ
   insert per edge, skip already-visited vertices). The cost comes from
   the callees:
   - `BinaryHeapPQ::delete_min` is O(q²) and `insert` O(q) (Chap45 review:
     one-element `ArraySeqStPer::append` copies the array). With up to
     m + 1 deleteMin calls on a queue of up to m + 1 entries, the
     deleteMins cost O(m³); the inserts cost O(m²).
   - `WeightedDirGraphStEph*::out_neighbors_weighed` scans the whole arc
     set, O(m), for each of up to n visited vertices: O(n m).
   - `SetStEph` (hash set) mem/insert and the SSSP result array reads and
     writes are O(1).
   Total Work O(n m + m³); the algorithm is sequential, so the span equals
   the work.

## 3. Counts (per annotation site)

18 new lines were added in 3 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 4 |
| 3 | does not match old analysis | 4 |
| 4 | no textbook cost | 14 |
| 5 | unannotated functions | 14 |
| 6 | malformed annotations | 0 |

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 57 | StackStEph.rs | 12 | 0 | 0 | 0 | 12 |
| 2 | 57 | DijkstraStEphU64.rs | 3 | 0 | 2 | 2 | 1 |
| 3 | 57 | DijkstraStEphF64.rs | 3 | 0 | 2 | 2 | 1 |

## 4. Unannotated functions (14)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 57 | DijkstraStEphU64.rs | Ord cmp, partial_cmp, TotalOrder cmp, fmt x2 |
| 2 | 57 | DijkstraStEphF64.rs | Ord cmp, partial_cmp, TotalOrder cmp, fmt x2 |
| 3 | 57 | StackStEph.rs | clone, default, fmt x2 |

## 5. Malformed annotations (0)

None. In both Dijkstra files `pq_entry_new` has a `// veracity:
no_requires` line after its doc block; the new line was placed directly
after the existing Alg Analysis line.

## 6. Notable findings

1. **Dijkstra is O(n m + m³), not O(m lg n).** The algorithm follows
   Alg 57.2, but it uses `BinaryHeapPQ`, whose persistent array-backed
   `delete_min` is O(q²), and a graph whose `out_neighbors_weighed` scans
   every arc. Replacing the PQ with the in-place heap operations
   (`bubble_down_heap` is O(lg n)) and giving the graph an adjacency table
   would restore O(m lg n).
2. The old lines on `dijkstra` restated the textbook cost without
   checking the callees.
3. `StackStEph` is a Vec wrapper; its O(1) annotations are correct.
