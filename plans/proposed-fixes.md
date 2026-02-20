# Proposed Code Fixes (Chap50–Chap66)

Generated from veracity proof holes report. Priority: verusification (no verus!) > struct outside verus! > external_body > other.

## Execution Report (AFK run)

**Completed:** 1–3 (P1 Chap50 verusification)
- Probability.rs: Added verus! block, external_body on f64 impls
- MatrixChainStEph, MatrixChainStPer: Added verus! with ExMatrixDim, ExMatrixChain*S
- OptBinSearchTreeStEph, OptBinSearchTreeStPer: Added verus! with ExKeyProb, ExOBST*S
- Removed ExProbability from OptBinSearchTreeMtEph (Probability now in verus!)

**Cancelled:** 4–7 (modules commented out in lib.rs)
**Cancelled:** 8–23 (require HashMap replacement or blocked dependencies)

| # | Pri | Chapter | File(s) | Proposed Work |
|---|-----|---------|---------|---------------|
| 1 | P1 | Chap50 | Probability.rs | Add verus! block — not verusified |
| 2 | P1 | Chap50 | MatrixChainStEph, MatrixChainStPer | Add verus! block — not verusified |
| 3 | P1 | Chap50 | OptBinSearchTreeStEph, OptBinSearchTreeStPer | Add verus! block — not verusified |
| 4 | P1 | Chap57 | DijkstraStEphF64 | Add verus! block — not verusified |
| 5 | P1 | Chap58 | BellmanFordStEphF64 | Add verus! block — not verusified |
| 6 | P1 | Chap59 | JohnsonMtEphF64, JohnsonStEphF64 | Add verus! block — not verusified |
| 7 | P1 | Chap64 | TSPApproxStEph | Add verus! block — not verusified |
| 8 | P2 | Chap50 | MatrixChainMtEph, MatrixChainMtPer | Move MatrixDim, MatrixChain*S inside verus! |
| 9 | P2 | Chap50 | OptBinSearchTreeMtEph, OptBinSearchTreeMtPer | Move KeyProb, OBSTMt*S inside verus! |
| 10 | P2 | Chap51 | TopDownDPStPer | Move TopDownDPStPerS inside verus!; Clone in verus! |
| 11 | P2 | Chap65 | UnionFindStEph | Move UnionFindStEph struct inside verus! |
| 12 | P2 | Chap65 | KruskalStEph | Move PQEntry inside verus!; Clone in verus! |
| 13 | P2 | Chap66 | BoruvkaStEph | Move LabeledEdge inside verus!; Clone in verus! |
| 14 | P3 | Chap50 | MatrixChainMtEph, OptBinSearchTreeMtEph | Replace dummy RwLockPredicate (inv true) |
| 15 | P3 | Chap52 | AdjSeqGraphMtEph | Verify delete_vertex (ext_body) |
| 16 | P3 | Chap52 | EdgeSetGraphMtPer | Verify out_neighbors (ext_body) |
| 17 | P3 | Chap53 | PQMinStEph, PQMinStPer | Verify new, priority, pq_min, pq_min_multi |
| 18 | P3 | Chap53 | GraphSearchStEph, GraphSearchStPer, MtPer | Verify select, graph_search, reachable |
| 19 | P3 | Chap56 | PathWeightUtilsStEph, PathWeightUtilsStPer | Verify f64_approx_eq (ext_body) |
| 20 | P3 | Chap56 | Example56_1, Example56_3 | Verify example fns (5 ext_body) |
| 21 | P3 | Chap57 | DijkstraStEphI64 | Verify dijkstra fn (ext_body) |
| 22 | P4 | Chap50 | MatrixChain*, OptBinSearchTree* | Replace Clone derive with spec impl |
| 23 | P4 | Chap64 | SpanTreeMtEph | RwLock ext_body, dummy predicate |

## Priority Legend

- **P1 (critical)**: File has no verus! block — not verusified. Highest priority.
- **P2 (high)**: Struct/enum outside verus! or Clone derived outside. Move inside verus! with specs.
- **P3 (medium)**: external_body hiding algorithmic logic. Verify or document as accepted boundary.
- **P4 (low)**: RwLock/dummy predicate, Clone derive. Improve specs.

## Chapters Clean (no proposed work)

- Chap54 (BFS): 4 modules, 22 proof fns, 0 holes
- Chap55 (DFS, SCC, TopoSort): 8 modules, 1 proof fn, 0 holes
- Chap61 (EdgeContraction, VertexMatching): 4 modules, 0 holes
- Chap62 (StarContraction, StarPartition): 4 modules, 0 holes
- Chap63 (Connectivity): 2 modules, 0 holes

## Note

Chap60 does not exist in the codebase.
