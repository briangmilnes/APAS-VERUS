# Proposed Work (Chap50–Chap66)

| # | Chapter | Holed Files | Proposed Work |
|---|---------|-------------|---------------|
| 1 | Chap50 | MatrixChainMtEph, MtPer | struct out, dummy RwLockPred |
| 2 | Chap50 | MatrixChainStEph, StPer | struct out (HashMap) |
| 3 | Chap50 | OptBinSearchTreeMtEph, MtPer | struct out, dummy RwLockPred |
| 4 | Chap50 | OptBinSearchTreeStEph, StPer | struct out (HashMap) |
| 5 | Chap50 | Probability | f64 ext_body (15) |
| 6 | Chap51 | BottomUpDPMtEph, MtPer | dummy RwLockPred |
| 7 | Chap51 | TopDownDPMtEph, MtPer | struct out, dummy RwLockPred |
| 8 | Chap51 | TopDownDPStEph, StPer | struct out (HashMap) |
| 9 | Chap52 | AdjTableGraphMtPer | ext_body delete_vertex |
| 10 | Chap52 | EdgeSetGraphMtPer | ext_body out_neighbors |
| 11 | Chap53 | GraphSearchStEph, StPer, MtPer | ext_body select, search |
| 12 | Chap53 | PQMinStEph, PQMinStPer | ext_body new, pq_min |
| 13 | Chap56 | Example56_1, Example56_3 | ext_body (5 total) |
| 14 | Chap56 | PathWeightUtilsStEph, StPer | ext_body f64_approx_eq |
| 15 | Chap57 | DijkstraStEphI64 | ext_body dijkstra fn |
| 16 | Chap64 | SpanTreeMtEph | dummy RwLockPred (2) |
| 17 | Chap65 | UnionFindStEph | struct out (HashMap) |
| 18 | Chap65 | PrimStEph | struct out PQEntry |

**Note:** Chap52, Chap53 modules commented out in lib.rs. Chap57 DijkstraStEphF64, Chap58 BellmanFordStEphF64, Chap59 Johnson*, Chap64 TSPApproxStEph, Chap65 PrimStEph, Chap66 Boruvka* commented out or use ordered_float/rand.

**Chapters clean:** Chap54, Chap55, Chap61, Chap62, Chap63
