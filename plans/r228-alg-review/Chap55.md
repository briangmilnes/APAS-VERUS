# r228 Alg Analysis Review: Chap55

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap55.txt` (Depth-First Search).

| # | Source | Algorithm | Work | Span |
|---|---|---|---|---|
| 1 | Lemma 55.2 | DFSAll | n + m DFS calls; n visit/finish; m revisit | — |
| 2 | CS 55.8 (trees) | DFS, adjacency table + tree sets | (m + n) lg n | (m + n) lg n |
| 3 | CS 55.8 (arrays) | DFS, adjacency seqs + ephemeral array X (Alg 55.7) | m + n | m + n |
| 4 | Alg 55.10 | Directed cycle detection (DFSAll) | as CS 55.8 [1] | as CS 55.8 |
| 5 | Alg 55.13, Ex 55.6 | Topological sort (decreasingFinish), enumerable | \|V\| + \|E\| | \|V\| + \|E\| |
| 6 | Alg 55.18, Ex 55.8 | SCC | exercise; \|V\| + \|E\| with arrays [2] | same |

[1] Alg 55.10 is DFSAll with O(1) visit/finish/revisit when the ancestor
set is an array, so CS 55.8's array bound applies; the file's APAS line
cites CS 55.8.
[2] Ex 55.8 asks for the SCC cost. With the array representation it is two
DFSAll passes plus a transpose, O(|V| + |E|); `SCCStEph.rs` carries that
as an APAS line citing CS 55.8, and `SCCStPer.rs` has a free-form
"APAS: Work O(|V| + |E|)" line.

DFS is sequential in the textbook (P-complete remark), so Span = Work
throughout, and all files are St.

Notation: h = height of an `AVLTreeSet*` result tree (≤ its size);
c = size of one SCC component; E_c = out-edges of its vertices.

## 2. Cost base

- `ArraySeqStEph::set`/`nth`, `Vec<bool>` index/set: O(1).
- Chap41 `AVLTreeSetStEph::insert` and `AVLTreeSetStPer::insert`: O(n h)
  (ParamBST deep-copy expose, no rebalancing; the persistent insert first
  copies the tree).
- Chap37 `AVLTreeSeqStEph::from_vec`: O(n lg n) (one insert per element);
  `AVLTreeSeqStPer::from_vec`: O(n) (balanced build).
- `Vec::remove(i)` / `Vec::insert(i, x)`: O(len − i) element shifts.

## 3. Reviewed functions

T = trait declaration, I = impl.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 55 | DFSStEph.rs | dfs (T) | CS 55.8 both | (m+n) lg n | V+E+V² h | not textbook; old wrong [3] |
| 2 | 55 | DFSStEph.rs | dfs_recursive | none | V+E | V+E+V² h | not textbook; old wrong [3] |
| 3 | 55 | DFSStEph.rs | dfs (I) | CS 55.8 | V+E | V+E+V² h | not textbook; old wrong [3] |
| 4 | 55 | DFSStPer.rs | dfs (T) | CS 55.8 both | (m+n) lg n | V+E+V² h | not textbook; old wrong [3] |
| 5 | 55 | DFSStPer.rs | dfs_recursive | none | V+E | V+E+V² h | not textbook; old wrong [3] |
| 6 | 55 | DFSStPer.rs | dfs (I) | CS 55.8 | V+E | V+E+V² h | not textbook; old wrong [3] |
| 7 | 55 | CycleDetectStEph.rs | has_cycle (T,I) | CS 55.8: V+E | V+E | V+E | matches textbook |
| 8 | 55 | CycleDetectStEph.rs | dfs_check_cycle | none | V+E | V+E | no textbook cost |
| 9 | 55 | CycleDetectStPer.rs | has_cycle (T,I) | CS 55.8: V+E | V+E | V+E | matches textbook |
| 10 | 55 | CycleDetectStPer.rs | dfs_check_cycle | none | V+E | V+E | no textbook cost |
| 11 | 55 | TopoSortStEph.rs | topo_sort (T,I) | Ex 55.6: V+E | V+E | V lg V + E | not textbook; old wrong [4] |
| 12 | 55 | TopoSortStEph.rs | topological_sort_opt | Ex 55.6: V+E | V+E | V lg V + E | not textbook; old wrong [4] |
| 13 | 55 | TopoSortStEph.rs | dfs_finish_order | none | V+E | V+E | no textbook cost |
| 14 | 55 | TopoSortStEph.rs | dfs_finish_order_cycle_det. | none | V+E | V+E | no textbook cost [5] |
| 15 | 55 | TopoSortStPer.rs | topo_sort (T,I) | Ex 55.6: V+E | V+E | V+E | matches textbook |
| 16 | 55 | TopoSortStPer.rs | topological_sort_opt | Ex 55.6: V+E | V+E | V+E | matches textbook |
| 17 | 55 | TopoSortStPer.rs | dfs_finish_order | none | V+E | V+E | no textbook cost |
| 18 | 55 | TopoSortStPer.rs | dfs_finish_order_cycle_det. | none | V+E | V+E | no textbook cost [5] |
| 19 | 55 | SCCStEph.rs | scc (T,I) | CS 55.8: V+E | V+E | V(E + V h) | not textbook; old wrong [6] |
| 20 | 55 | SCCStEph.rs | compute_finish_order | Ex 55.6: V+E | V+E | V lg V + E | not textbook; old wrong [4] |
| 21 | 55 | SCCStEph.rs | transpose_graph | none | V+E | V + V E | no cost; old wrong [7] |
| 22 | 55 | SCCStEph.rs | check_wf_adj_list_eph | none | V+E | V+E | no textbook cost [8] |
| 23 | 55 | SCCStEph.rs | dfs_reach | none | V+E | c + E_c + c² h | no cost; old wrong [3] |
| 24 | 55 | SCCStPer.rs | scc (T,I) | "V+E" [2] | V+E | V(E + V h) | not textbook; old wrong [6] |
| 25 | 55 | SCCStPer.rs | dfs_finish_order | none | V+E | V+E | no textbook cost |
| 26 | 55 | SCCStPer.rs | compute_finish_order | Ex 55.6: V+E | V+E | V+E | matches textbook |
| 27 | 55 | SCCStPer.rs | transpose_graph | none | V+E | V + V E | no cost; old wrong [7] |
| 28 | 55 | SCCStPer.rs | check_wf_adj_list_per | none | V+E | V+E | no textbook cost [8] |
| 29 | 55 | SCCStPer.rs | dfs_reach | none | V+E | c + E_c + c² h | no cost; old wrong [3] |

Rows 7, 9, 11, 15, 19, 24 are 2 lines each (T and I); the rest are 1.

### Footnotes

3. The visited set is an O(1) array, but every reached vertex is also
   inserted into an `AVLTreeSet*` result, O(n h) per insert on the
   ParamBST. Over R reached vertices that is O(R² h), so DFS costs
   O(|V| + |E| + |V|² h), neither CS 55.8 bound.
4. The finish-order DFS and Vec reversal are linear, but the result is
   built with `AVLTreeSeqStEph::from_vec`, which inserts one element at a
   time, O(|V| lg |V|). The StPer files use `AVLTreeSeqStPer::from_vec`
   (balanced build, O(n)) and meet Ex 55.6.
5. `dfs_finish_order_cycle_detect` is never called outside its own
   recursion (dead code in both TopoSort files).
6. `scc` = compute_finish_order + transpose_graph + one dfs_reach per
   component + an `AVLTreeSeq*::from_vec` of the components. The transpose
   (footnote 7) and the AVLTreeSet component inserts dominate:
   O(|V| |E| + Σ c² h) ⊆ O(|V| (|E| + |V| h)).
7. `transpose_graph` appends u to v's list by
   `adj_vecs.remove(v); temp.push(u); adj_vecs.insert(v, temp)`. Each
   remove and insert shifts up to |V| entries of the outer Vec, so each
   edge costs O(|V|). Indexing `adj_vecs[v].push(u)` would be O(1).
8. `check_wf_adj_list_eph` and `check_wf_adj_list_per` are never called.

## 4. Counts (per annotation site)

35 new lines were added, one per annotated site, in 8 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 8 |
| 2 | does not match textbook | 14 |
| 3 | does not match old analysis | 18 |
| 4 | no textbook cost | 13 |
| 5 | unannotated functions | 8 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 35 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 55 | DFSStEph.rs | 3 | 0 | 3 | 3 | 0 |
| 2 | 55 | DFSStPer.rs | 3 | 0 | 3 | 3 | 0 |
| 3 | 55 | CycleDetectStEph.rs | 3 | 2 | 0 | 0 | 1 |
| 4 | 55 | CycleDetectStPer.rs | 3 | 2 | 0 | 0 | 1 |
| 5 | 55 | TopoSortStEph.rs | 5 | 0 | 3 | 3 | 2 |
| 6 | 55 | TopoSortStPer.rs | 5 | 3 | 0 | 0 | 2 |
| 7 | 55 | SCCStEph.rs | 6 | 0 | 3 | 5 | 3 |
| 8 | 55 | SCCStPer.rs | 7 | 1 | 2 | 4 | 4 |

## 5. Unannotated functions (8)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 55 | DFSStEph.rs, DFSStPer.rs | fmt (one each) |
| 2 | 55 | CycleDetectStEph.rs, CycleDetectStPer.rs | fmt (one each) |
| 3 | 55 | TopoSortStEph.rs, TopoSortStPer.rs | fmt (one each) |
| 4 | 55 | SCCStEph.rs, SCCStPer.rs | fmt (one each) |

`DFSSpecsAndLemmas.rs` has only spec and proof functions. Spec and proof
functions were not counted.

## 6. Malformed annotations (0)

None is malformed. `SCCStPer.rs` `scc` (trait) has a free-form doc line
"APAS: Work O(|V| + |E|), Span O(|V| + |E|)" that is not in
`/// - Alg Analysis: APAS (...)` form; it was left in place. The DFS
trait `dfs` in both files carries two APAS lines (tree and array variants
of CS 55.8) and two old Code-review lines; the new line follows the last.

## 7. Notable findings

1. **SCC transpose is O(|V| |E|).** Both `transpose_graph` functions
   append to an adjacency list by removing it from the outer Vec and
   reinserting it at the same index, which shifts up to |V| entries per
   edge. With the AVLTreeSet component inserts, SCC costs
   O(|V| (|E| + |V| h)) against the textbook's O(|V| + |E|); every old
   line claimed O(|V| + |E|).
2. **DFS and dfs_reach pay for an AVLTreeSet result.** The DFS itself is
   the array-based Alg 55.7, but each reached vertex is inserted into a
   Chap41 AVLTreeSet (ParamBST with deep-copy expose and no rebalancing),
   O(n h) per insert, so reachability costs O(|V| + |E| + |V|² h).
3. **TopoSortStEph misses Ex 55.6 by a log factor; the Per variants
   match.** `AVLTreeSeqStEph::from_vec` builds the output by repeated
   insertion (O(|V| lg |V|)); TopoSortStPer and SCCStPer's finish order
   use the balanced `AVLTreeSeqStPer::from_vec` and meet O(|V| + |E|).
   CycleDetect (both files) meets CS 55.8. Dead code:
   `dfs_finish_order_cycle_detect` (both TopoSort files) and
   `check_wf_adj_list_*` (both SCC files).
