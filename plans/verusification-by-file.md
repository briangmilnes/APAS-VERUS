# Verusification List by File (Chap50–Chap66)

Includes files with no verus! block and types declared outside verus!.

## Files with no verus! block (not verusified)

*None.* All previously unverusified files now have placeholder verus! blocks.

## Files with types outside verus!

| File | Chapter | Types outside verus! |
|------|---------|----------------------|
| MatrixChainMtEph.rs | Chap50 | MatrixChainMtEphS (HashMap) |
| MatrixChainMtPer.rs | Chap50 | MatrixChainMtPerS (HashMap) |
| MatrixChainStEph.rs | Chap50 | MatrixChainStEphS (HashMap) |
| MatrixChainStPer.rs | Chap50 | MatrixChainStPerS (HashMap) |
| OptBinSearchTreeMtEph.rs | Chap50 | KeyProb, OBSTMtEphS |
| OptBinSearchTreeMtPer.rs | Chap50 | KeyProb, OBSTMtPerS |
| OptBinSearchTreeStEph.rs | Chap50 | KeyProb, OBSTStEphS |
| OptBinSearchTreeStPer.rs | Chap50 | KeyProb, OBSTStPerS |
| TopDownDPMtEph.rs | Chap51 | TopDownDPMtEphS |
| TopDownDPMtPer.rs | Chap51 | TopDownDPMtPerS |
| TopDownDPStEph.rs | Chap51 | TopDownDPStEphS |
| TopDownDPStPer.rs | Chap51 | TopDownDPStPerS |
| DijkstraStEphF64.rs | Chap57 | PQEntry (F64Dist) |
| PrimStEph.rs | Chap65 | PQEntry (OrderedFloat) |
| UnionFindStEph.rs | Chap65 | UnionFindStEph |
| BoruvkaMtEph.rs | Chap66 | LabeledEdge |
| BoruvkaStEph.rs | Chap66 | LabeledEdge |

## Files with holes (external_body, etc.) but types in verus!

| File | Chapter | Issue |
|------|---------|-------|
| Probability.rs | Chap50 | 15 external_body (f64 impls) |
| BottomUpDPMtEph.rs | Chap51 | dummy RwLockPredicate |
| BottomUpDPMtPer.rs | Chap51 | dummy RwLockPredicate |
| AdjTableGraphMtPer.rs | Chap52 | external_body delete_vertex |
| EdgeSetGraphMtPer.rs | Chap52 | external_body out_neighbors |
| GraphSearchMtPer.rs | Chap53 | 4 external_body |
| GraphSearchStEph.rs | Chap53 | 4 external_body |
| GraphSearchStPer.rs | Chap53 | 4 external_body |
| PQMinStEph.rs | Chap53 | 4 external_body |
| PQMinStPer.rs | Chap53 | 4 external_body |
| Example56_1.rs | Chap56 | 3 external_body |
| Example56_3.rs | Chap56 | 2 external_body |
| PathWeightUtilsStEph.rs | Chap56 | 1 external_body f64_approx_eq |
| PathWeightUtilsStPer.rs | Chap56 | 1 external_body f64_approx_eq |
| DijkstraStEphI64.rs | Chap57 | 1 external_body dijkstra fn |
| SpanTreeMtEph.rs | Chap64 | 2 dummy RwLockPredicate |

## Files fully verusified (clean)

| File | Chapter |
|------|---------|
| BottomUpDPStEph.rs | Chap51 |
| BottomUpDPStPer.rs | Chap51 |
| AdjMatrixGraphMtEph.rs | Chap52 |
| AdjMatrixGraphMtPer.rs | Chap52 |
| AdjMatrixGraphStEph.rs | Chap52 |
| AdjMatrixGraphStPer.rs | Chap52 |
| AdjSeqGraphMtEph.rs | Chap52 |
| AdjSeqGraphMtPer.rs | Chap52 |
| AdjSeqGraphStEph.rs | Chap52 |
| AdjSeqGraphStPer.rs | Chap52 |
| AdjTableGraphStEph.rs | Chap52 |
| AdjTableGraphStPer.rs | Chap52 |
| EdgeSetGraphStEph.rs | Chap52 |
| EdgeSetGraphStPer.rs | Chap52 |
| BFSMtEph.rs | Chap54 |
| BFSMtPer.rs | Chap54 |
| BFSStEph.rs | Chap54 |
| BFSStPer.rs | Chap54 |
| CycleDetectStEph.rs | Chap55 |
| CycleDetectStPer.rs | Chap55 |
| DFSStEph.rs | Chap55 |
| DFSStPer.rs | Chap55 |
| SCCStEph.rs | Chap55 |
| SCCStPer.rs | Chap55 |
| TopoSortStEph.rs | Chap55 |
| TopoSortStPer.rs | Chap55 |
| AllPairsResultStEphF64.rs | Chap56 |
| AllPairsResultStEphI64.rs | Chap56 |
| AllPairsResultStPerF64.rs | Chap56 |
| AllPairsResultStPerI64.rs | Chap56 |
| SSSPResultStEphF64.rs | Chap56 |
| SSSPResultStEphI64.rs | Chap56 |
| SSSPResultStPerF64.rs | Chap56 |
| SSSPResultStPerI64.rs | Chap56 |
| StackStEph.rs | Chap57 |
| BellmanFordStEphF64.rs | Chap58 |
| BellmanFordStEphI64.rs | Chap58 |
| JohnsonMtEphF64.rs | Chap59 |
| JohnsonStEphF64.rs | Chap59 |
| JohnsonMtEphI64.rs | Chap59 |
| JohnsonStEphI64.rs | Chap59 |
| EdgeContractionMtEph.rs | Chap61 |
| EdgeContractionStEph.rs | Chap61 |
| VertexMatchingMtEph.rs | Chap61 |
| VertexMatchingStEph.rs | Chap61 |
| StarContractionMtEph.rs | Chap62 |
| StarContractionStEph.rs | Chap62 |
| StarPartitionMtEph.rs | Chap62 |
| StarPartitionStEph.rs | Chap62 |
| ConnectivityMtEph.rs | Chap63 |
| ConnectivityStEph.rs | Chap63 |
| SpanTreeStEph.rs | Chap64 |
| TSPApproxStEph.rs | Chap64 |
| KruskalStEph.rs | Chap65 |
