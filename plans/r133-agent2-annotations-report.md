# R133 Agent 2 — Alg Analysis Annotations Report

## Summary

Added `/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(...), Span O(...)` annotations to functions missing them in Chap06, Chap43, and Chap50.

**Every annotation was written after reading the actual function implementation** to determine the correct work/span complexity. No annotations were guessed by function name.

## Progress

| # | Chap | File | Missing Start | Annotated | Remaining |
|---|------|------|--------------|-----------|-----------|
| 1 | 06 | WeightedDirGraphStEph*.rs (12 files) | 24 | 24 | 0 |
| 2 | 06 | DirGraphStEph.rs | 19 | 19 | 0 |
| 3 | 06 | DirGraphMtEph.rs | 51 | 51 | 0 |
| 4 | 06 | UnDirGraphStEph.rs | 11 | 11 | 0 |
| 5 | 06 | UnDirGraphMtEph.rs | 31 | 31 | 0 |
| 6 | 06 | LabDirGraphStEph.rs | 11 | 11 | 0 |
| 7 | 06 | LabDirGraphMtEph.rs | 25 | 25 | 0 |
| 8 | 06 | LabUnDirGraphStEph.rs | 10 | 10 | 0 |
| 9 | 06 | LabUnDirGraphMtEph.rs | 28 | 28 | 0 |
| 10 | 43 | OrderedTableStEph.rs | 53 | 53 | 0 |
| 11 | 43 | OrderedTableStPer.rs | 54 | 54 | 0 |
| 12 | 43 | OrderedTableMtEph.rs | 31 | 31 | 0 |
| 13 | 43 | OrderedTableMtPer.rs | 24 | 24 | 0 |
| 14 | 43 | OrderedSetStEph.rs | 40 | 40 | 0 |
| 15 | 43 | OrderedSetStPer.rs | 40 | 40 | 0 |
| 16 | 43 | OrderedSetMtEph.rs | 22 | 22 | 0 |
| 17 | 43 | AugOrderedTableStEph.rs | 32 | 32 | 0 |
| 18 | 43 | AugOrderedTableStPer.rs | 29 | 29 | 0 |
| 19 | 43 | AugOrderedTableMtEph.rs | 34 | 34 | 0 |
| 20 | 50 | MatrixChainStEph.rs | 24 | 24 | 0 |
| 21 | 50 | MatrixChainStPer.rs | 18 | 18 | 0 |
| 22 | 50 | MatrixChainMtEph.rs | 26 | 26 | 0 |
| 23 | 50 | MatrixChainMtPer.rs | 20 | 20 | 0 |
| 24 | 50 | OptBinSearchTreeStEph.rs | 20 | 20 | 0 |
| 25 | 50 | OptBinSearchTreeStPer.rs | 14 | 14 | 0 |
| 26 | 50 | OptBinSearchTreeMtEph.rs | 20 | 20 | 0 |
| 27 | 50 | OptBinSearchTreeMtPer.rs | 14 | 14 | 0 |
| **Total** | | | **725** | **725** | **0** |

## Key Complexity Findings

### Chap06 Graphs
- **St files**: All sequential. Work = Span.
  - Accessors (vertices, arcs, sizeV, sizeA, neighbor, incident): O(1)
  - Neighbor queries (n_plus, n_minus, ng): O(|A|) or O(|E|) — iterate all arcs/edges
  - Multi-vertex queries (n_plus_of_vertices, ng_of_vertices): O(|S| * |A|)
- **Mt files**: Parallel via ParaPair! split.
  - Single-vertex _par: Work O(|A|), Span O(log |A|) — binary split on arcs
  - Multi-vertex _par: Work O(|S| * |A|), Span O(log |S| * log |A|)
  - Locked wrappers: same complexity + O(1) RwLock

### Chap43 OrderedTable
- **Critical finding**: `bst_find_by_key` (used by OrderedTable) does `tree.in_order()` + linear scan = **O(n)**, not O(log n). This makes find, insert, delete all O(n).
- `tabulate`, `map`, `filter`: O(n log n) — n iterations with O(log n) treap inserts
- `intersection`, `union`, `difference`: O(n * m) — iterate one, find (O(m)) per element
- `split_key`, `split_rank_key`, `get_key_range`: O(n log n) — in_order + rebuild with BST inserts
- `collect`, `domain`, `reduce`: O(n) — linear traversal
- `from_sorted_entries`: O(n log n) — n treap inserts

### Chap43 OrderedSet (different from OrderedTable!)
- **Key difference**: OrderedSet uses direct ParamBST operations, NOT bst_find_by_key.
  - `find`: O(log n) — proper BST search
  - `insert`, `delete`: O(log n) — treap split + join
  - `first`, `last`, `previous`, `next`: O(log n) — BST traversal
  - `split`, `rank`, `select`, `split_rank`, `get_range`: O(log n) — BST split-based
  - `filter`, `intersection`, `union`, `difference`: O(n log n) — recursive BST operations
  - `join`: O(n log n) — delegates to union

### Chap43 AugOrderedTable
- Wraps OrderedTableStEph with cached reduction value
- Mutating operations add O(n) recalculation via `calculate_reduction`
- `reduce_val`: O(1) — returns cached clone
- `reduce_range`: O(n log n) — get_key_range + cached clone

### Chap50 MatrixChain
- `optimal_cost`: O(n^3) — memoized DP, n^2 subproblems each O(n) inner loop
- `matrix_chain_rec`: O(n^3) — same memoized DP
- `multiply_cost`: O(1) — three array lookups + two multiplications
- Constructors: O(1) or O(n)
- `set_dimension`/`update_dimension`/`clear_memo`: O(n) — clears/rebuilds memo
- Mt variants: same Work complexity, wrap under RwLock/Arc

### Chap50 OptBinSearchTree
- `optimal_cost`: O(n^3) — memoized DP, n^2 subproblems each O(n)
- `obst_rec`: O(n^3) — recursive memoized DP helper
- Constructors: O(1) or O(n)
- `set_key_prob`/`update_prob`: O(1) for St, O(n) for Mt (clone under lock)
- Mt variants: RwLock wrappers, same Work complexity
