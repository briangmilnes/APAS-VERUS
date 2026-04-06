# R149 Agent 3 — Debug/Display Report

## Task

Add missing `impl Debug` and `impl Display` outside `verus!` for structs flagged
by veracity rule [14] in Chap44–Chap66.

## Summary

- **[14] warnings fixed**: 112 (56 Debug + 56 Display) across 47 files
- **[14] warnings skipped**: 8 (4 MatrixDim structs with `#[derive(Debug)]` inside verus!)
- **Example files skipped**: Example44_1.rs (per instructions)
- **Validation**: 5701 verified, 0 errors from new code
- **RTT**: 3690 passed, 0 failed

## Pre-existing issue

StarPartitionMtEph.rs line 1755 has a Z3 verification failure on a forall quantifier
inside `verus!`. This is unrelated to Debug/Display additions (which are outside `verus!`).
The failure is consistent across runs.

## Files modified by chapter

| # | Chap | File | Structs added |
|---|------|------|---------------|
| 1 | 44 | DocumentIndex.rs | QueryBuilder: Debug, Display |
| 2 | 49 | MinEditDistMtEph.rs | MinEditDistMtEphMemoInv: Debug, Display |
| 3 | 49 | MinEditDistMtPer.rs | MinEditDistMtPerMemoInv: Debug, Display |
| 4 | 49 | SubsetSumMtEph.rs | SubsetSumMtEphMemoInv: Debug, Display |
| 5 | 49 | SubsetSumMtPer.rs | SubsetSumMtPerMemoInv: Debug, Display |
| 6 | 50 | MatrixChainMtEph.rs | MatrixChainMtEphDimInv, MatrixChainMtEphMemoInv, MatrixChainMtEphV: Debug, Display each |
| 7 | 50 | MatrixChainMtPer.rs | MatrixChainMtPerV, MatrixChainMtPerMemoInv: Debug, Display each |
| 8 | 50 | MatrixChainStEph.rs | MatrixChainStEphV: Debug, Display; MatrixChainStEphS: Debug |
| 9 | 50 | MatrixChainStPer.rs | MatrixChainStPerV: Debug, Display; MatrixChainStPerS: Debug |
| 10 | 50 | OptBinSearchTreeMtEph.rs | OptBSTMtEphKeysInv, OptBSTMtEphMemoInv, OBSTMtEphV: Debug, Display each |
| 11 | 50 | OptBinSearchTreeMtPer.rs | OptBSTMtPerMemoInv, OBSTMtPerV: Debug, Display each |
| 12 | 50 | OptBinSearchTreeStEph.rs | OBSTStEphV: Debug, Display |
| 13 | 50 | OptBinSearchTreeStPer.rs | OBSTStPerV: Debug, Display |
| 14 | 52 | AdjMatrixGraphMtEph.rs | AdjMatrixGraphMtEph: Display |
| 15 | 52 | AdjMatrixGraphMtPer.rs | AdjMatrixGraphMtPer: Display |
| 16 | 52 | AdjMatrixGraphStEph.rs | AdjMatrixGraphStEph: Display |
| 17 | 52 | AdjMatrixGraphStPer.rs | AdjMatrixGraphStPer: Display |
| 18 | 52 | AdjSeqGraphMtEph.rs | AdjSeqGraphMtEph: Display |
| 19 | 52 | AdjSeqGraphMtPer.rs | AdjSeqGraphMtPer: Display |
| 20 | 52 | AdjSeqGraphStEph.rs | AdjSeqGraphStEph: Display |
| 21 | 52 | AdjSeqGraphStPer.rs | AdjSeqGraphStPer: Display |
| 22 | 52 | AdjTableGraphMtPer.rs | AdjTableGraphMtPer: Debug, Display |
| 23 | 52 | AdjTableGraphStEph.rs | AdjTableGraphStEph: Debug, Display |
| 24 | 52 | AdjTableGraphStPer.rs | AdjTableGraphStPer: Debug, Display |
| 25 | 52 | EdgeSetGraphMtPer.rs | EdgeSetGraphMtPer: Debug, Display |
| 26 | 52 | EdgeSetGraphStEph.rs | EdgeSetGraphStEph: Debug, Display |
| 27 | 52 | EdgeSetGraphStPer.rs | EdgeSetGraphStPer: Display |
| 28 | 54 | BFSMtEph.rs | BFSTreeS, BFSMtEph: Debug, Display each |
| 29 | 54 | BFSMtPer.rs | BFSTreeS, BFSMtPer: Debug, Display each |
| 30 | 54 | BFSStEph.rs | BFSTreeS, BFSStEph: Debug, Display each |
| 31 | 54 | BFSStPer.rs | BFSTreeS, BFSStPer: Debug, Display each |
| 32 | 55 | CycleDetectStEph.rs | CycleDetectStEph: Debug, Display |
| 33 | 55 | CycleDetectStPer.rs | CycleDetectStPer: Debug, Display |
| 34 | 55 | DFSStEph.rs | DFSStEph: Debug, Display |
| 35 | 55 | DFSStPer.rs | DFSStPer: Debug, Display |
| 36 | 55 | SCCStEph.rs | SCCStEph: Debug, Display |
| 37 | 55 | SCCStPer.rs | SCCStPer: Debug, Display |
| 38 | 55 | TopoSortStEph.rs | TopoSortStEph: Debug, Display |
| 39 | 55 | TopoSortStPer.rs | TopoSortStPer: Debug, Display |
| 40 | 57 | StackStEph.rs | StackStEph: Display |
| 41 | 61 | EdgeContractionMtEph.rs | EdgeContractionMtEph: Debug, Display |
| 42 | 61 | EdgeContractionStEph.rs | EdgeContractionStEph: Debug, Display |
| 43 | 61 | VertexMatchingMtEph.rs | VertexMatchingMtEph: Debug, Display |
| 44 | 61 | VertexMatchingStEph.rs | VertexMatchingStEph: Debug, Display |
| 45 | 62 | StarContractionMtEph.rs | StarContractionMtEph: Debug, Display |
| 46 | 62 | StarContractionStEph.rs | StarContractionStEph: Debug, Display |
| 47 | 62 | StarPartitionMtEph.rs | StarPartitionMtEph: Debug, Display |
| 48 | 62 | StarPartitionStEph.rs | StarPartitionStEph: Debug, Display |
| 49 | 63 | ConnectivityMtEph.rs | ConnectivityMtEph: Debug, Display |
| 50 | 63 | ConnectivityStEph.rs | ConnectivityStEph: Debug, Display |
| 51 | 64 | SpanTreeMtEph.rs | SpanTreeMtEph: Debug, Display |
| 52 | 64 | SpanTreeStEph.rs | SpanTreeStEph: Debug, Display |
| 53 | 64 | TSPApproxStEph.rs | TSPApproxStEph: Debug, Display |
| 54 | 65 | PrimStEph.rs | PrimStEph: Debug, Display |
| 55 | 65 | UnionFindStEph.rs | UnionFindStEph, UnionMergeInfo, UnionFindStEphV: Debug, Display each |
| 56 | 66 | BoruvkaMtEph.rs | BoruvkaMtEph: Debug, Display |
| 57 | 66 | BoruvkaStEph.rs | BoruvkaStEph: Debug, Display |

## Skipped MatrixDim Debug warnings

Four MatrixChain files (MtEph, MtPer, StEph, StPer) define `MatrixDim` with
`#[derive(Clone, Copy, PartialEq, Eq, Debug)]` inside `verus!`. Adding a manual
`impl Debug` outside `verus!` would create a duplicate impl conflict. The derive
already provides Debug.

## Patterns used

- **Unit structs** (BFSStEph, DFSStEph, etc.): print type name only.
- **Ghost/Inv structs** (MemoInv, DimInv, ghost structs): print type name only
  (ghost fields not accessible at runtime).
- **Data structs** (UnionFindStEph, BFSTreeS, MatrixChainStEphS): print key
  field summaries (lengths, counts).
- **Generic structs**: trait bounds match or exceed the struct's own bounds.
