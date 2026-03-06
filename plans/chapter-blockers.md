<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter Blocking Report

Generated: 2026-03-06

| # | Chap | File | Verusified | Specs | Verified | Holes | Blocked By | Blocks |
|---|------|------|------------|-------|----------|-------|------------|--------|
| 1 | 02 | HFSchedulerMtEph | yes | med | yes | 20 | — | 02/FibonacciHFScheduler |
| | | | | | | | — | 05/SetMtEph |
| | | | | | | | — | 18/ArraySeqMtEph |
| | | | | | | | — | 18/ArraySeqMtPer |
| | | | | | | | — | 19/ArraySeqMtEph |
| | | | | | | | — | 26/MergeSortMtPer |
| | | | | | | | — | 26/ScanDCMtPer |
| | | | | | | | — | 27/ReduceContractMtEph |
| | | | | | | | — | 35/OrderStatSelectMtEph |
| | | | | | | | — | 35/OrderStatSelectMtPer |
| | | | | | | | — | 52/EdgeSetGraphMtPer |
| | | | | | | | — | 54/BFSMtEph |
| | | | | | | | — | 54/BFSMtPer |
| 2 | 02 | FibonacciHFScheduler | yes | high | yes | 0 | 02/HFSchedulerMtEph | — |
| 3 | 03 | InsertionSortStEph | yes | low | yes | 0 | — | — |
| 4 | 05 | SetStEph | yes | med | yes | 2 | — | 05/KleeneStPer |
| | | | | | | | — | 05/MappingStEph |
| | | | | | | | — | 05/RelationStEph |
| | | | | | | | — | 06/DirGraphMtEph |
| | | | | | | | — | 06/DirGraphStEph |
| | | | | | | | — | 06/LabDirGraphMtEph |
| | | | | | | | — | 06/LabDirGraphStEph |
| | | | | | | | — | 06/LabUnDirGraphMtEph |
| | | | | | | | — | 06/LabUnDirGraphStEph |
| | | | | | | | — | 06/UnDirGraphMtEph |
| | | | | | | | — | 06/UnDirGraphStEph |
| | | | | | | | — | 06/WeightedDirGraphStEphI128 |
| | | | | | | | — | 06/WeightedDirGraphStEphI16 |
| | | | | | | | — | 06/WeightedDirGraphStEphI32 |
| | | | | | | | — | 06/WeightedDirGraphStEphI64 |
| | | | | | | | — | 06/WeightedDirGraphStEphI8 |
| | | | | | | | — | 06/WeightedDirGraphStEphIsize |
| | | | | | | | — | 06/WeightedDirGraphStEphU128 |
| | | | | | | | — | 06/WeightedDirGraphStEphU16 |
| | | | | | | | — | 06/WeightedDirGraphStEphU32 |
| | | | | | | | — | 06/WeightedDirGraphStEphU64 |
| | | | | | | | — | 06/WeightedDirGraphStEphU8 |
| | | | | | | | — | 06/WeightedDirGraphStEphUsize |
| | | | | | | | — | 58/BellmanFordStEphI64 |
| | | | | | | | — | 62/StarContractionStEph |
| | | | | | | | — | 62/StarPartitionStEph |
| | | | | | | | — | 63/ConnectivityStEph |
| | | | | | | | — | 64/SpanTreeMtEph |
| | | | | | | | — | 64/SpanTreeStEph |
| | | | | | | | — | 64/TSPApproxStEph |
| | | | | | | | — | 65/KruskalStEph |
| | | | | | | | — | 65/PrimStEph |
| | | | | | | | — | 66/BoruvkaMtEph |
| | | | | | | | — | 66/BoruvkaStEph |
| 5 | 05 | SetMtEph | yes | med | yes | 2 | 02/HFSchedulerMtEph | — |
| 6 | 05 | RelationStEph | yes | med | yes | 0 | 05/SetStEph | 05/MappingStEph |
| 7 | 05 | MappingStEph | yes | med | yes | 2 | 05/RelationStEph | — |
| | | | | | | | 05/SetStEph | — |
| 8 | 05 | KleeneStPer | yes | med | yes | 0 | 05/SetStEph | — |
| 9 | 06 | DirGraphStEph | yes | med | yes | 0 | 05/SetStEph | — |
| 10 | 06 | UnDirGraphStEph | yes | med | yes | 0 | 05/SetStEph | 62/StarContractionStEph |
| | | | | | | | — | 62/StarPartitionStEph |
| | | | | | | | — | 63/ConnectivityStEph |
| | | | | | | | — | 64/SpanTreeStEph |
| 11 | 06 | LabDirGraphStEph | yes | med | yes | 0 | 05/SetStEph | 06/WeightedDirGraphStEphI128 |
| | | | | | | | — | 06/WeightedDirGraphStEphI16 |
| | | | | | | | — | 06/WeightedDirGraphStEphI32 |
| | | | | | | | — | 06/WeightedDirGraphStEphI64 |
| | | | | | | | — | 06/WeightedDirGraphStEphI8 |
| | | | | | | | — | 06/WeightedDirGraphStEphIsize |
| | | | | | | | — | 06/WeightedDirGraphStEphU128 |
| | | | | | | | — | 06/WeightedDirGraphStEphU16 |
| | | | | | | | — | 06/WeightedDirGraphStEphU32 |
| | | | | | | | — | 06/WeightedDirGraphStEphU64 |
| | | | | | | | — | 06/WeightedDirGraphStEphU8 |
| | | | | | | | — | 06/WeightedDirGraphStEphUsize |
| | | | | | | | — | 58/BellmanFordStEphI64 |
| 12 | 06 | LabUnDirGraphStEph | yes | med | yes | 0 | 05/SetStEph | 64/TSPApproxStEph |
| | | | | | | | — | 65/KruskalStEph |
| | | | | | | | — | 65/PrimStEph |
| 13 | 06 | WeightedDirGraphStEphU32 | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 14 | 06 | DirGraphMtEph | yes | med | yes | 0 | 05/SetStEph | — |
| 15 | 06 | UnDirGraphMtEph | yes | med | yes | 0 | 05/SetStEph | 64/SpanTreeMtEph |
| 16 | 06 | LabDirGraphMtEph | yes | med | yes | 0 | 05/SetStEph | — |
| 17 | 06 | LabUnDirGraphMtEph | yes | med | yes | 0 | 05/SetStEph | — |
| 18 | 06 | WeightedDirGraphStEphU8 | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 19 | 06 | WeightedDirGraphStEphU16 | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 20 | 06 | WeightedDirGraphStEphU64 | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 21 | 06 | WeightedDirGraphStEphU128 | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 22 | 06 | WeightedDirGraphStEphUsize | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 23 | 06 | WeightedDirGraphStEphI8 | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 24 | 06 | WeightedDirGraphStEphI16 | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 25 | 06 | WeightedDirGraphStEphI32 | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 26 | 06 | WeightedDirGraphStEphI64 | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 27 | 06 | WeightedDirGraphStEphI128 | yes | med | yes | 0 | 05/SetStEph | 58/BellmanFordStEphI64 |
| | | | | | | | 06/LabDirGraphStEph | — |
| 28 | 06 | WeightedDirGraphStEphIsize | yes | med | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| 29 | 11 | FibonacciStEph | yes | high | yes | 0 | — | 11/FibonacciMtEph2Threads |
| | | | | | | | — | 11/FibonacciMtEphRecomputes |
| | | | | | | | — | 11/FibonacciMtPerAllThreads |
| | | | | | | | — | 11/FibonacciMtPerTSM |
| 30 | 11 | FibonacciMtPerAllThreads | yes | high | yes | 0 | 11/FibonacciStEph | — |
| 31 | 11 | FibonacciMtPerTSM | yes | med | yes | 4 | 11/FibonacciStEph | — |
| 32 | 11 | FibonacciMtEph2Threads | yes | med | yes | 4 | 11/FibonacciStEph | — |
| 33 | 11 | FibonacciMtEphRecomputes | yes | med | yes | 4 | 11/FibonacciStEph | — |
| 34 | 12 | Exercise12_1 | yes | low | yes | 12 | — | — |
| 35 | 12 | Exercise12_2 | yes | low | yes | 0 | — | — |
| 36 | 12 | Exercise12_5 | yes | low | yes | 24 | — | — |
| 37 | 17 | MathSeq | yes | low | yes | 4 | — | — |
| 38 | 18 | ArraySeq | yes | med | yes | 6 | — | — |
| 39 | 18 | ArraySeqStPer | yes | med | yes | 2 | — | 21/Algorithm21_1 |
| | | | | | | | — | 21/Algorithm21_2 |
| | | | | | | | — | 21/Problem21_1 |
| | | | | | | | — | 26/DivConReduceStPer |
| | | | | | | | — | 26/MergeSortStPer |
| | | | | | | | — | 26/ScanDCStPer |
| | | | | | | | — | 35/OrderStatSelectStPer |
| | | | | | | | — | 37/AVLTreeSeqMtPer |
| | | | | | | | — | 37/AVLTreeSeqStPer |
| | | | | | | | — | 37/BSTRBMtEph |
| | | | | | | | — | 37/BSTSetAVLMtEph |
| | | | | | | | — | 37/BSTSetBBAlphaMtEph |
| | | | | | | | — | 37/BSTSetPlainMtEph |
| | | | | | | | — | 37/BSTSetRBMtEph |
| | | | | | | | — | 37/BSTSetSplayMtEph |
| | | | | | | | — | 37/BSTSplayMtEph |
| | | | | | | | — | 37/BSTSplayStEph |
| | | | | | | | — | 38/BSTParaMtEph |
| | | | | | | | — | 38/BSTParaStEph |
| | | | | | | | — | 39/BSTParaTreapMtEph |
| | | | | | | | — | 39/BSTSetTreapMtEph |
| | | | | | | | — | 39/BSTTreapMtEph |
| | | | | | | | — | 39/BSTTreapStEph |
| | | | | | | | — | 40/BSTKeyValueStEph |
| | | | | | | | — | 40/BSTReducedStEph |
| | | | | | | | — | 40/BSTSizeStEph |
| | | | | | | | — | 43/OrderedSetMtEph |
| | | | | | | | — | 43/OrderedTableMtPer |
| | | | | | | | — | 51/BottomUpDPStPer |
| | | | | | | | — | 51/TopDownDPStPer |
| | | | | | | | — | 52/AdjMatrixGraphStPer |
| | | | | | | | — | 52/AdjSeqGraphStPer |
| | | | | | | | — | 64/SpanTreeMtEph |
| 40 | 18 | ArraySeqStEph | yes | med | yes | 2 | — | 21/Exercise21_8 |
| 41 | 18 | LinkedListStPer | yes | med | yes | 4 | — | — |
| 42 | 18 | LinkedListStEph | yes | med | yes | 4 | — | 47/LinkedListChainedHashTableStEph |
| 43 | 18 | ArraySeqMtEph | yes | med | yes | 0 | 02/HFSchedulerMtEph | — |
| 44 | 18 | ArraySeqMtPer | yes | med | yes | 2 | 02/HFSchedulerMtEph | 26/DivConReduceMtPer |
| | | | | | | | — | 26/MergeSortMtPer |
| | | | | | | | — | 26/ScanDCMtPer |
| | | | | | | | — | 35/OrderStatSelectMtPer |
| | | | | | | | — | 49/MinEditDistMtPer |
| | | | | | | | — | 49/SubsetSumMtPer |
| | | | | | | | — | 51/BottomUpDPMtPer |
| | | | | | | | — | 51/TopDownDPMtPer |
| | | | | | | | — | 52/AdjMatrixGraphMtPer |
| | | | | | | | — | 52/AdjSeqGraphMtPer |
| | | | | | | | — | 54/BFSMtPer |
| 45 | 19 | ArraySeqStPer | yes | med | yes | 2 | — | 21/Algorithm21_5 |
| | | | | | | | — | 21/Algorithm21_6 |
| | | | | | | | — | 21/Exercise21_5 |
| | | | | | | | — | 21/Exercise21_7 |
| | | | | | | | — | 21/Problem21_4 |
| | | | | | | | — | 42/TableStPer |
| | | | | | | | — | 43/OrderedSetStEph |
| | | | | | | | — | 43/OrderedSetStPer |
| | | | | | | | — | 43/OrderedTableStPer |
| | | | | | | | — | 45/BinaryHeapPQ |
| | | | | | | | — | 45/HeapsortExample |
| | | | | | | | — | 45/LeftistHeapPQ |
| | | | | | | | — | 45/SortedListPQ |
| | | | | | | | — | 45/UnsortedListPQ |
| | | | | | | | — | 49/MinEditDistStPer |
| | | | | | | | — | 49/SubsetSumStPer |
| | | | | | | | — | 54/BFSStPer |
| | | | | | | | — | 56/AllPairsResultStEphF64 |
| | | | | | | | — | 56/AllPairsResultStEphI64 |
| | | | | | | | — | 56/AllPairsResultStPerF64 |
| | | | | | | | — | 56/AllPairsResultStPerI64 |
| | | | | | | | — | 56/PathWeightUtilsStEph |
| | | | | | | | — | 56/PathWeightUtilsStPer |
| | | | | | | | — | 56/SSSPResultStEphF64 |
| | | | | | | | — | 56/SSSPResultStEphI64 |
| | | | | | | | — | 56/SSSPResultStPerF64 |
| | | | | | | | — | 56/SSSPResultStPerI64 |
| 46 | 19 | ArraySeqStEph | yes | med | yes | 2 | — | 27/ReduceContractStEph |
| | | | | | | | — | 27/ScanContractStEph |
| | | | | | | | — | 28/MaxContigSubSumBruteStEph |
| | | | | | | | — | 28/MaxContigSubSumDivConOptStEph |
| | | | | | | | — | 28/MaxContigSubSumDivConStEph |
| | | | | | | | — | 28/MaxContigSubSumIterStEph |
| | | | | | | | — | 28/MaxContigSubSumReducedMcsseStEph |
| | | | | | | | — | 28/MaxContigSubSumReducedStEph |
| | | | | | | | — | 35/OrderStatSelectStEph |
| | | | | | | | — | 36/QuickSortStEph |
| | | | | | | | — | 37/AVLTreeSeq |
| | | | | | | | — | 37/AVLTreeSeqStEph |
| | | | | | | | — | 41/ArraySetStEph |
| | | | | | | | — | 41/Example41_3 |
| | | | | | | | — | 42/TableMtEph |
| | | | | | | | — | 42/TableStEph |
| | | | | | | | — | 42/TableStPer |
| | | | | | | | — | 43/OrderedTableStEph |
| | | | | | | | — | 49/MinEditDistStEph |
| | | | | | | | — | 49/SubsetSumStEph |
| | | | | | | | — | 51/BottomUpDPStEph |
| | | | | | | | — | 51/TopDownDPStEph |
| | | | | | | | — | 52/AdjMatrixGraphStEph |
| | | | | | | | — | 52/AdjSeqGraphStEph |
| | | | | | | | — | 54/BFSStEph |
| | | | | | | | — | 56/AllPairsResultStEphF64 |
| | | | | | | | — | 56/AllPairsResultStEphI64 |
| | | | | | | | — | 56/PathWeightUtilsStEph |
| | | | | | | | — | 56/SSSPResultStEphF64 |
| | | | | | | | — | 56/SSSPResultStEphI64 |
| 47 | 19 | ArraySeqMtEph | yes | med | yes | 0 | 02/HFSchedulerMtEph | 27/ReduceContractMtEph |
| | | | | | | | — | 27/ScanContractMtEph |
| | | | | | | | — | 28/MaxContigSubSumDivConMtEph |
| | | | | | | | — | 28/MaxContigSubSumDivConOptMtEph |
| | | | | | | | — | 35/OrderStatSelectMtEph |
| | | | | | | | — | 36/QuickSortMtEph |
| | | | | | | | — | 42/TableMtEph |
| | | | | | | | — | 43/AugOrderedTableMtEph |
| | | | | | | | — | 43/OrderedTableMtEph |
| | | | | | | | — | 49/MinEditDistMtEph |
| | | | | | | | — | 49/SubsetSumMtEph |
| | | | | | | | — | 51/BottomUpDPMtEph |
| | | | | | | | — | 51/TopDownDPMtEph |
| | | | | | | | — | 52/AdjMatrixGraphMtEph |
| | | | | | | | — | 52/AdjSeqGraphMtEph |
| | | | | | | | — | 54/BFSMtEph |
| 48 | 19 | ArraySeqMtEphSlice | yes | med | yes | 0 | — | — |
| 49 | 21 | Algorithm21_1 | yes | high | yes | 0 | 18/ArraySeqStPer | — |
| 50 | 21 | Algorithm21_2 | yes | high | yes | 0 | 18/ArraySeqStPer | — |
| 51 | 21 | Algorithm21_5 | yes | high | yes | 0 | 19/ArraySeqStPer | — |
| | | | | | | | 21/Exercise21_8 | — |
| 52 | 21 | Algorithm21_6 | yes | high | yes | 0 | 19/ArraySeqStPer | — |
| | | | | | | | 21/Exercise21_8 | — |
| 53 | 21 | Exercise21_5 | yes | high | yes | 0 | 19/ArraySeqStPer | — |
| 54 | 21 | Exercise21_6 | yes | — | yes | 0 | — | — |
| 55 | 21 | Exercise21_7 | yes | high | yes | 0 | 19/ArraySeqStPer | — |
| 56 | 21 | Exercise21_8 | yes | high | yes | 0 | 18/ArraySeqStEph | 21/Algorithm21_5 |
| | | | | | | | — | 21/Algorithm21_6 |
| 57 | 21 | Exercise21_9 | yes | high | yes | 0 | — | — |
| 58 | 21 | Problem21_1 | yes | high | yes | 0 | 18/ArraySeqStPer | — |
| 59 | 21 | Problem21_3 | no | — | no | — | — | — |
| 60 | 21 | Problem21_4 | yes | high | yes | 0 | 19/ArraySeqStPer | — |
| 61 | 23 | PrimTreeSeqStPer | yes | med | yes | 10 | — | — |
| 62 | 23 | BalBinTreeStEph | yes | low | yes | 8 | — | 37/BSTAVLStEph |
| | | | | | | | — | 37/BSTBBAlphaStEph |
| | | | | | | | — | 37/BSTPlainStEph |
| | | | | | | | — | 37/BSTRBStEph |
| 63 | 26 | DivConReduceStPer | yes | med | yes | 0 | 18/ArraySeqStPer | 26/ScanDCStPer |
| 64 | 26 | MergeSortStPer | yes | med | yes | 0 | 18/ArraySeqStPer | — |
| 65 | 26 | ScanDCStPer | yes | med | yes | 0 | 18/ArraySeqStPer | — |
| | | | | | | | 26/DivConReduceStPer | — |
| 66 | 26 | ETSPStEph | no | — | no | — | — | — |
| 67 | 26 | ETSPMtEph | no | — | no | — | — | — |
| 68 | 26 | DivConReduceMtPer | yes | med | yes | 0 | 18/ArraySeqMtPer | — |
| 69 | 26 | MergeSortMtPer | yes | med | yes | 0 | 02/HFSchedulerMtEph | — |
| | | | | | | | 18/ArraySeqMtPer | — |
| 70 | 26 | ScanDCMtPer | yes | med | yes | 0 | 02/HFSchedulerMtEph | — |
| | | | | | | | 18/ArraySeqMtPer | — |
| 71 | 27 | ReduceContractStEph | yes | high | yes | 0 | 19/ArraySeqStEph | — |
| 72 | 27 | ReduceContractMtEph | yes | high | yes | 0 | 02/HFSchedulerMtEph | 27/ScanContractMtEph |
| | | | | | | | 19/ArraySeqMtEph | — |
| 73 | 27 | ScanContractStEph | yes | high | yes | 0 | 19/ArraySeqStEph | — |
| 74 | 27 | ScanContractMtEph | yes | high | yes | 0 | 19/ArraySeqMtEph | — |
| | | | | | | | 27/ReduceContractMtEph | — |
| 75 | 28 | MCSSSpec | yes | med | yes | 0 | — | 28/MaxContigSubSumBruteStEph |
| | | | | | | | — | 28/MaxContigSubSumDivConMtEph |
| | | | | | | | — | 28/MaxContigSubSumDivConOptMtEph |
| | | | | | | | — | 28/MaxContigSubSumDivConOptStEph |
| | | | | | | | — | 28/MaxContigSubSumDivConStEph |
| | | | | | | | — | 28/MaxContigSubSumIterStEph |
| | | | | | | | — | 28/MaxContigSubSumReducedMcsseStEph |
| | | | | | | | — | 28/MaxContigSubSumReducedStEph |
| 76 | 28 | MaxContigSubSumBruteStEph | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| | | | | | | | 28/MCSSSpec | — |
| 77 | 28 | MaxContigSubSumReducedStEph | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| | | | | | | | 28/MCSSSpec | — |
| 78 | 28 | MaxContigSubSumDivConStEph | yes | med | yes | 0 | 19/ArraySeqStEph | 28/MaxContigSubSumDivConMtEph |
| | | | | | | | 28/MCSSSpec | — |
| 79 | 28 | MaxContigSubSumIterStEph | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| | | | | | | | 28/MCSSSpec | — |
| 80 | 28 | MaxContigSubSumReducedMcsseStEph | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| | | | | | | | 28/MCSSSpec | — |
| 81 | 28 | MaxContigSubSumDivConOptStEph | yes | med | yes | 0 | 19/ArraySeqStEph | 28/MaxContigSubSumDivConOptMtEph |
| | | | | | | | 28/MCSSSpec | — |
| 82 | 28 | MaxContigSubSumOptStEph | no | — | no | — | — | — |
| 83 | 28 | MaxContigSubSumOptMtEph | no | — | no | — | — | — |
| 84 | 28 | MaxContigSubSumDivConMtEph | yes | med | yes | 0 | 19/ArraySeqMtEph | — |
| | | | | | | | 28/MaxContigSubSumDivConStEph | — |
| | | | | | | | 28/MCSSSpec | — |
| 85 | 28 | MaxContigSubSumDivConOptMtEph | yes | med | yes | 0 | 19/ArraySeqMtEph | — |
| | | | | | | | 28/MaxContigSubSumDivConOptStEph | — |
| | | | | | | | 28/MCSSSpec | — |
| 86 | 30 | Probability | yes | low | yes | 22 | — | 50/OptBinSearchTreeMtEph |
| | | | | | | | — | 50/OptBinSearchTreeMtPer |
| | | | | | | | — | 50/OptBinSearchTreeStEph |
| | | | | | | | — | 50/OptBinSearchTreeStPer |
| 87 | 35 | OrderStatSelectStEph | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| 88 | 35 | OrderStatSelectStPer | yes | med | yes | 0 | 18/ArraySeqStPer | — |
| 89 | 35 | OrderStatSelectMtEph | yes | med | yes | 0 | 02/HFSchedulerMtEph | — |
| | | | | | | | 19/ArraySeqMtEph | — |
| 90 | 35 | OrderStatSelectMtPer | yes | med | yes | 0 | 02/HFSchedulerMtEph | — |
| | | | | | | | 18/ArraySeqMtPer | — |
| 91 | 36 | QuickSortStEph | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| 92 | 36 | QuickSortMtEph | yes | med | yes | 0 | 19/ArraySeqMtEph | — |
| 93 | 36 | QuickSortMtEphSlice | no | — | no | — | — | — |
| 94 | 37 | AVLTreeSeq | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| 95 | 37 | AVLTreeSeqStEph | yes | med | yes | 0 | 19/ArraySeqStEph | 41/AVLTreeSetMtEph |
| | | | | | | | — | 41/AVLTreeSetStEph |
| | | | | | | | — | 41/Example41_3 |
| | | | | | | | — | 43/OrderedSetStEph |
| | | | | | | | — | 53/GraphSearchStEph |
| | | | | | | | — | 53/PQMinStEph |
| 96 | 37 | AVLTreeSeqStPer | yes | med | yes | 0 | 18/ArraySeqStPer | 41/AVLTreeSetStPer |
| | | | | | | | — | 43/AugOrderedTableMtEph |
| | | | | | | | — | 43/AugOrderedTableStEph |
| | | | | | | | — | 43/AugOrderedTableStPer |
| | | | | | | | — | 43/Example43_1 |
| | | | | | | | — | 43/OrderedSetStEph |
| | | | | | | | — | 43/OrderedSetStPer |
| | | | | | | | — | 43/OrderedTableMtEph |
| | | | | | | | — | 43/OrderedTableStEph |
| | | | | | | | — | 43/OrderedTableStPer |
| | | | | | | | — | 45/BalancedTreePQ |
| | | | | | | | — | 45/HeapsortExample |
| | | | | | | | — | 53/GraphSearchStPer |
| | | | | | | | — | 53/PQMinStPer |
| 97 | 37 | AVLTreeSeqMtPer | yes | med | yes | 0 | 18/ArraySeqStPer | 41/AVLTreeSetMtPer |
| | | | | | | | — | 52/EdgeSetGraphMtPer |
| 98 | 37 | BSTPlainStEph | yes | med | yes | 0 | 23/BalBinTreeStEph | 37/BSTAVLStEph |
| | | | | | | | — | 37/BSTBBAlphaStEph |
| | | | | | | | — | 37/BSTRBStEph |
| 99 | 37 | BSTPlainMtEph | no | — | no | — | — | — |
| 100 | 37 | BSTAVLStEph | yes | med | yes | 0 | 23/BalBinTreeStEph | 37/BSTRBStEph |
| | | | | | | | 37/BSTPlainStEph | — |
| 101 | 37 | BSTAVLMtEph | no | — | no | — | — | — |
| 102 | 37 | BSTRBStEph | yes | med | yes | 0 | 23/BalBinTreeStEph | — |
| | | | | | | | 37/BSTAVLStEph | — |
| | | | | | | | 37/BSTPlainStEph | — |
| 103 | 37 | BSTRBMtEph | yes | low | yes | 2 | 18/ArraySeqStPer | 37/BSTSetRBMtEph |
| 104 | 37 | BSTSplayStEph | yes | low | yes | 0 | 18/ArraySeqStPer | — |
| 105 | 37 | BSTSplayMtEph | yes | low | yes | 2 | 18/ArraySeqStPer | 37/BSTSetSplayMtEph |
| 106 | 37 | BSTBBAlphaStEph | yes | med | yes | 0 | 23/BalBinTreeStEph | — |
| | | | | | | | 37/BSTPlainStEph | — |
| 107 | 37 | BSTBBAlphaMtEph | no | — | no | — | — | — |
| 108 | 37 | BSTSetPlainMtEph | yes | low | yes | 0 | 18/ArraySeqStPer | — |
| | | | | | | | 37/BSTPlainMtEph | — |
| 109 | 37 | BSTSetAVLMtEph | yes | low | yes | 0 | 18/ArraySeqStPer | — |
| | | | | | | | 37/BSTAVLMtEph | — |
| 110 | 37 | BSTSetRBMtEph | yes | low | yes | 0 | 18/ArraySeqStPer | — |
| | | | | | | | 37/BSTRBMtEph | — |
| 111 | 37 | BSTSetSplayMtEph | yes | low | yes | 0 | 18/ArraySeqStPer | — |
| | | | | | | | 37/BSTSplayMtEph | — |
| 112 | 37 | BSTSetBBAlphaMtEph | yes | low | yes | 0 | 18/ArraySeqStPer | — |
| | | | | | | | 37/BSTBBAlphaMtEph | — |
| 113 | 38 | BSTParaStEph | yes | med | yes | 4 | 18/ArraySeqStPer | — |
| 114 | 38 | BSTParaMtEph | yes | med | yes | 6 | 18/ArraySeqStPer | — |
| 115 | 39 | BSTTreapStEph | yes | low | yes | 0 | 18/ArraySeqStPer | — |
| 116 | 39 | BSTTreapMtEph | yes | med | yes | 8 | 18/ArraySeqStPer | — |
| 117 | 39 | BSTParaTreapMtEph | yes | low | yes | 2 | 18/ArraySeqStPer | 39/BSTSetTreapMtEph |
| | | | | | | | — | 43/OrderedSetMtEph |
| | | | | | | | — | 43/OrderedTableMtPer |
| 118 | 39 | BSTSetTreapMtEph | yes | med | yes | 0 | 18/ArraySeqStPer | — |
| | | | | | | | 39/BSTParaTreapMtEph | — |
| 119 | 40 | BSTKeyValueStEph | yes | med | yes | 0 | 18/ArraySeqStPer | — |
| 120 | 40 | BSTSizeStEph | yes | med | yes | 0 | 18/ArraySeqStPer | — |
| 121 | 40 | BSTReducedStEph | yes | med | yes | 6 | 18/ArraySeqStPer | — |
| 122 | 41 | ArraySetStEph | yes | med | yes | 0 | 19/ArraySeqStEph | 41/Example41_3 |
| | | | | | | | — | 42/Example42_1 |
| | | | | | | | — | 42/TableMtEph |
| | | | | | | | — | 42/TableStEph |
| | | | | | | | — | 42/TableStPer |
| | | | | | | | — | 43/AugOrderedTableMtEph |
| | | | | | | | — | 43/AugOrderedTableStEph |
| | | | | | | | — | 43/AugOrderedTableStPer |
| | | | | | | | — | 43/OrderedTableMtEph |
| | | | | | | | — | 43/OrderedTableStEph |
| | | | | | | | — | 43/OrderedTableStPer |
| 123 | 41 | ArraySetEnumMtEph | no | — | no | — | — | — |
| 124 | 41 | AVLTreeSetStEph | yes | low | yes | 2 | 37/AVLTreeSeqStEph | 41/AVLTreeSetMtEph |
| | | | | | | | — | 41/Example41_3 |
| | | | | | | | — | 43/OrderedSetStEph |
| | | | | | | | — | 53/GraphSearchStEph |
| | | | | | | | — | 53/PQMinStEph |
| 125 | 41 | AVLTreeSetStPer | yes | med | yes | 2 | 37/AVLTreeSeqStPer | 43/OrderedSetStPer |
| | | | | | | | — | 53/GraphSearchStPer |
| | | | | | | | — | 53/PQMinStPer |
| 126 | 41 | AVLTreeSetMtEph | yes | med | yes | 2 | 37/AVLTreeSeqStEph | — |
| | | | | | | | 41/AVLTreeSetStEph | — |
| 127 | 41 | AVLTreeSetMtPer | yes | med | yes | 2 | 37/AVLTreeSeqMtPer | 52/AdjTableGraphMtPer |
| | | | | | | | — | 52/EdgeSetGraphMtPer |
| 128 | 41 | Example41_3 | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| | | | | | | | 37/AVLTreeSeqStEph | — |
| | | | | | | | 41/ArraySetStEph | — |
| | | | | | | | 41/AVLTreeSetStEph | — |
| 129 | 42 | TableStEph | yes | med | yes | 0 | 19/ArraySeqStEph | 42/Example42_1 |
| | | | | | | | 41/ArraySetStEph | 43/AugOrderedTableStEph |
| | | | | | | | — | 43/OrderedTableStEph |
| 130 | 42 | TableStPer | yes | med | yes | 0 | 19/ArraySeqStEph | 42/Example42_1 |
| | | | | | | | 19/ArraySeqStPer | 43/AugOrderedTableStPer |
| | | | | | | | 41/ArraySetStEph | 43/OrderedTableStPer |
| 131 | 42 | TableMtEph | yes | med | yes | 0 | 19/ArraySeqMtEph | 42/Example42_1 |
| | | | | | | | 19/ArraySeqStEph | 43/AugOrderedTableMtEph |
| | | | | | | | 41/ArraySetStEph | 43/OrderedTableMtEph |
| 132 | 42 | Example42_1 | yes | low | yes | 0 | 41/ArraySetStEph | — |
| | | | | | | | 42/TableMtEph | — |
| | | | | | | | 42/TableStEph | — |
| | | | | | | | 42/TableStPer | — |
| 133 | 43 | OrderedTableStEph | yes | med | yes | 0 | 19/ArraySeqStEph | 43/AugOrderedTableStEph |
| | | | | | | | 37/AVLTreeSeqStPer | — |
| | | | | | | | 41/ArraySetStEph | — |
| | | | | | | | 42/TableStEph | — |
| 134 | 43 | OrderedTableMtEph | yes | med | yes | 0 | 19/ArraySeqMtEph | 43/AugOrderedTableMtEph |
| | | | | | | | 37/AVLTreeSeqStPer | — |
| | | | | | | | 41/ArraySetStEph | — |
| | | | | | | | 42/TableMtEph | — |
| 135 | 43 | AugOrderedTableStEph | yes | med | yes | 0 | 37/AVLTreeSeqStPer | — |
| | | | | | | | 41/ArraySetStEph | — |
| | | | | | | | 42/TableStEph | — |
| | | | | | | | 43/OrderedTableStEph | — |
| 136 | 43 | AugOrderedTableMtEph | yes | med | yes | 0 | 19/ArraySeqMtEph | — |
| | | | | | | | 37/AVLTreeSeqStPer | — |
| | | | | | | | 41/ArraySetStEph | — |
| | | | | | | | 42/TableMtEph | — |
| | | | | | | | 43/OrderedTableMtEph | — |
| 137 | 43 | OrderedTableStPer | yes | med | yes | 2 | 19/ArraySeqStPer | 43/AugOrderedTableStPer |
| | | | | | | | 37/AVLTreeSeqStPer | — |
| | | | | | | | 41/ArraySetStEph | — |
| | | | | | | | 42/TableStPer | — |
| 138 | 43 | AugOrderedTableStPer | yes | med | yes | 0 | 37/AVLTreeSeqStPer | — |
| | | | | | | | 41/ArraySetStEph | — |
| | | | | | | | 42/TableStPer | — |
| | | | | | | | 43/OrderedTableStPer | — |
| 139 | 43 | OrderedSetStEph | yes | med | yes | 0 | 19/ArraySeqStPer | — |
| | | | | | | | 37/AVLTreeSeqStEph | — |
| | | | | | | | 37/AVLTreeSeqStPer | — |
| | | | | | | | 41/AVLTreeSetStEph | — |
| 140 | 43 | OrderedSetStPer | yes | med | yes | 0 | 19/ArraySeqStPer | 43/Example43_1 |
| | | | | | | | 37/AVLTreeSeqStPer | — |
| | | | | | | | 41/AVLTreeSetStPer | — |
| 141 | 43 | OrderedSetMtEph | yes | med | yes | 0 | 18/ArraySeqStPer | 43/OrderedTableMtPer |
| | | | | | | | 39/BSTParaTreapMtEph | 52/AdjTableGraphMtPer |
| 142 | 43 | OrderedTableMtPer | yes | med | yes | 0 | 18/ArraySeqStPer | 52/AdjTableGraphMtPer |
| | | | | | | | 39/BSTParaTreapMtEph | — |
| | | | | | | | 43/OrderedSetMtEph | — |
| 143 | 43 | Example43_1 | yes | low | yes | 0 | 37/AVLTreeSeqStPer | — |
| | | | | | | | 43/OrderedSetStPer | — |
| 144 | 44 | DocumentIndex | no | — | no | — | — | — |
| 145 | 44 | Example44_1 | no | — | no | — | — | — |
| 146 | 45 | UnsortedListPQ | yes | med | yes | 20 | 19/ArraySeqStPer | 45/HeapsortExample |
| 147 | 45 | SortedListPQ | yes | med | yes | 18 | 19/ArraySeqStPer | 45/HeapsortExample |
| 148 | 45 | BinaryHeapPQ | yes | med | yes | 20 | 19/ArraySeqStPer | 45/HeapsortExample |
| | | | | | | | — | 65/PrimStEph |
| 149 | 45 | BalancedTreePQ | yes | low | yes | 2 | 37/AVLTreeSeqStPer | 45/HeapsortExample |
| 150 | 45 | LeftistHeapPQ | yes | med | yes | 22 | 19/ArraySeqStPer | 45/HeapsortExample |
| 151 | 45 | HeapsortExample | yes | low | yes | 4 | 19/ArraySeqStPer | 45/Example45_2 |
| | | | | | | | 37/AVLTreeSeqStPer | — |
| | | | | | | | 45/BalancedTreePQ | — |
| | | | | | | | 45/BinaryHeapPQ | — |
| | | | | | | | 45/LeftistHeapPQ | — |
| | | | | | | | 45/SortedListPQ | — |
| | | | | | | | 45/UnsortedListPQ | — |
| 152 | 45 | Example45_2 | yes | low | yes | 0 | 45/HeapsortExample | — |
| 153 | 47 | ChainedHashTable | yes | med | yes | 2 | 47/ParaHashTableStEph | 47/LinkedListChainedHashTableStEph |
| | | | | | | | — | 47/StructChainedHashTable |
| | | | | | | | — | 47/VecChainedHashTableStEph |
| 154 | 47 | StructChainedHashTable | yes | low | yes | 8 | 47/ChainedHashTable | — |
| | | | | | | | 47/ParaHashTableStEph | — |
| 155 | 47 | VecChainedHashTableStEph | yes | low | yes | 0 | 47/ChainedHashTable | — |
| | | | | | | | 47/ParaHashTableStEph | — |
| 156 | 47 | LinkedListChainedHashTableStEph | yes | low | yes | 0 | 18/LinkedListStEph | — |
| | | | | | | | 47/ChainedHashTable | — |
| | | | | | | | 47/ParaHashTableStEph | — |
| 157 | 47 | FlatHashTable | yes | med | yes | 0 | 47/ParaHashTableStEph | 47/DoubleHashFlatHashTableStEph |
| | | | | | | | — | 47/LinProbFlatHashTableStEph |
| | | | | | | | — | 47/QuadProbFlatHashTableStEph |
| 158 | 47 | LinProbFlatHashTableStEph | yes | low | yes | 0 | 47/FlatHashTable | — |
| | | | | | | | 47/ParaHashTableStEph | — |
| 159 | 47 | QuadProbFlatHashTableStEph | yes | low | yes | 0 | 47/FlatHashTable | — |
| | | | | | | | 47/ParaHashTableStEph | — |
| 160 | 47 | DoubleHashFlatHashTableStEph | yes | low | yes | 0 | 47/FlatHashTable | — |
| | | | | | | | 47/ParaHashTableStEph | — |
| 161 | 47 | ParaHashTableStEph | yes | med | yes | 0 | — | 47/ChainedHashTable |
| | | | | | | | — | 47/DoubleHashFlatHashTableStEph |
| | | | | | | | — | 47/FlatHashTable |
| | | | | | | | — | 47/LinkedListChainedHashTableStEph |
| | | | | | | | — | 47/LinProbFlatHashTableStEph |
| | | | | | | | — | 47/QuadProbFlatHashTableStEph |
| | | | | | | | — | 47/StructChainedHashTable |
| | | | | | | | — | 47/VecChainedHashTableStEph |
| 162 | 49 | SubsetSumStEph | yes | low | yes | 0 | 19/ArraySeqStEph | — |
| 163 | 49 | SubsetSumStPer | yes | low | yes | 0 | 19/ArraySeqStPer | — |
| 164 | 49 | SubsetSumMtEph | yes | low | yes | 4 | 19/ArraySeqMtEph | — |
| 165 | 49 | SubsetSumMtPer | yes | low | yes | 4 | 18/ArraySeqMtPer | — |
| 166 | 49 | MinEditDistStEph | yes | low | yes | 0 | 19/ArraySeqStEph | — |
| 167 | 49 | MinEditDistStPer | yes | low | yes | 0 | 19/ArraySeqStPer | — |
| 168 | 49 | MinEditDistMtEph | yes | low | yes | 4 | 19/ArraySeqMtEph | — |
| 169 | 49 | MinEditDistMtPer | yes | low | yes | 4 | 18/ArraySeqMtPer | — |
| 170 | 50 | MatrixChainStEph | yes | med | yes | 2 | — | — |
| 171 | 50 | MatrixChainStPer | yes | med | yes | 2 | — | — |
| 172 | 50 | MatrixChainMtEph | yes | low | yes | 4 | — | — |
| 173 | 50 | MatrixChainMtPer | yes | low | yes | 4 | — | — |
| 174 | 50 | OptBinSearchTreeStEph | yes | low | yes | 0 | 30/Probability | — |
| 175 | 50 | OptBinSearchTreeStPer | yes | low | yes | 0 | 30/Probability | — |
| 176 | 50 | OptBinSearchTreeMtEph | yes | low | yes | 4 | 30/Probability | — |
| 177 | 50 | OptBinSearchTreeMtPer | yes | low | yes | 2 | 30/Probability | — |
| 178 | 51 | BottomUpDPStEph | yes | low | yes | 0 | 19/ArraySeqStEph | — |
| 179 | 51 | BottomUpDPStPer | yes | low | yes | 0 | 18/ArraySeqStPer | — |
| 180 | 51 | BottomUpDPMtEph | yes | low | yes | 2 | 19/ArraySeqMtEph | — |
| 181 | 51 | BottomUpDPMtPer | yes | low | yes | 2 | 18/ArraySeqMtPer | — |
| 182 | 51 | TopDownDPStEph | yes | low | yes | 0 | 19/ArraySeqStEph | — |
| 183 | 51 | TopDownDPStPer | yes | low | yes | 0 | 18/ArraySeqStPer | — |
| 184 | 51 | TopDownDPMtEph | yes | low | yes | 2 | 19/ArraySeqMtEph | — |
| 185 | 51 | TopDownDPMtPer | yes | low | yes | 2 | 18/ArraySeqMtPer | — |
| 186 | 52 | AdjSeqGraphStEph | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| 187 | 52 | AdjSeqGraphStPer | yes | med | yes | 0 | 18/ArraySeqStPer | — |
| 188 | 52 | AdjSeqGraphMtEph | yes | med | yes | 0 | 19/ArraySeqMtEph | — |
| 189 | 52 | AdjSeqGraphMtPer | yes | med | yes | 0 | 18/ArraySeqMtPer | — |
| 190 | 52 | AdjMatrixGraphStEph | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| 191 | 52 | AdjMatrixGraphStPer | yes | med | yes | 0 | 18/ArraySeqStPer | — |
| 192 | 52 | AdjMatrixGraphMtEph | yes | med | yes | 0 | 19/ArraySeqMtEph | — |
| 193 | 52 | AdjMatrixGraphMtPer | yes | med | yes | 0 | 18/ArraySeqMtPer | — |
| 194 | 52 | AdjTableGraphStEph | no | — | no | — | — | — |
| 195 | 52 | AdjTableGraphStPer | no | — | no | — | — | — |
| 196 | 52 | AdjTableGraphMtPer | yes | med | yes | 0 | 41/AVLTreeSetMtPer | — |
| | | | | | | | 43/OrderedSetMtEph | — |
| | | | | | | | 43/OrderedTableMtPer | — |
| 197 | 52 | EdgeSetGraphStEph | no | — | no | — | — | — |
| 198 | 52 | EdgeSetGraphStPer | no | — | no | — | — | — |
| 199 | 52 | EdgeSetGraphMtPer | yes | low | yes | 0 | 02/HFSchedulerMtEph | — |
| | | | | | | | 37/AVLTreeSeqMtPer | — |
| | | | | | | | 41/AVLTreeSetMtPer | — |
| 200 | 53 | PQMinStEph | yes | low | yes | 0 | 37/AVLTreeSeqStEph | — |
| | | | | | | | 41/AVLTreeSetStEph | — |
| 201 | 53 | PQMinStPer | yes | low | yes | 0 | 37/AVLTreeSeqStPer | — |
| | | | | | | | 41/AVLTreeSetStPer | — |
| 202 | 53 | GraphSearchStEph | yes | low | yes | 0 | 37/AVLTreeSeqStEph | — |
| | | | | | | | 41/AVLTreeSetStEph | — |
| 203 | 53 | GraphSearchStPer | yes | low | yes | 0 | 37/AVLTreeSeqStPer | — |
| | | | | | | | 41/AVLTreeSetStPer | — |
| 204 | 53 | GraphSearchMtPer | no | — | no | — | — | — |
| 205 | 54 | BFSStEph | yes | med | yes | 0 | 19/ArraySeqStEph | — |
| 206 | 54 | BFSStPer | yes | med | yes | 0 | 19/ArraySeqStPer | — |
| 207 | 54 | BFSMtEph | yes | high | yes | 0 | 02/HFSchedulerMtEph | — |
| | | | | | | | 19/ArraySeqMtEph | — |
| 208 | 54 | BFSMtPer | yes | high | yes | 0 | 02/HFSchedulerMtEph | — |
| | | | | | | | 18/ArraySeqMtPer | — |
| 209 | 55 | DFSStEph | no | — | no | — | — | — |
| 210 | 55 | DFSStPer | no | — | no | — | — | — |
| 211 | 55 | TopoSortStEph | no | — | no | — | — | — |
| 212 | 55 | TopoSortStPer | no | — | no | — | — | — |
| 213 | 55 | CycleDetectStEph | no | — | no | — | — | — |
| 214 | 55 | CycleDetectStPer | no | — | no | — | — | — |
| 215 | 55 | SCCStEph | no | — | no | — | — | — |
| 216 | 55 | SCCStPer | no | — | no | — | — | — |
| 217 | 56 | SSSPResultStEphI64 | yes | low | yes | 0 | 19/ArraySeqStEph | 58/BellmanFordStEphI64 |
| | | | | | | | 19/ArraySeqStPer | — |
| 218 | 56 | SSSPResultStPerI64 | yes | low | yes | 0 | 19/ArraySeqStPer | — |
| 219 | 56 | AllPairsResultStEphI64 | yes | low | yes | 0 | 19/ArraySeqStEph | — |
| | | | | | | | 19/ArraySeqStPer | — |
| 220 | 56 | AllPairsResultStPerI64 | yes | low | yes | 0 | 19/ArraySeqStPer | — |
| 221 | 56 | PathWeightUtilsStEph | yes | low | yes | 0 | 19/ArraySeqStEph | — |
| | | | | | | | 19/ArraySeqStPer | — |
| 222 | 56 | PathWeightUtilsStPer | yes | low | yes | 0 | 19/ArraySeqStPer | — |
| 223 | 56 | SSSPResultStEphF64 | yes | low | yes | 0 | 19/ArraySeqStEph | — |
| | | | | | | | 19/ArraySeqStPer | — |
| 224 | 56 | SSSPResultStPerF64 | yes | low | yes | 0 | 19/ArraySeqStPer | — |
| 225 | 56 | AllPairsResultStEphF64 | yes | low | yes | 0 | 19/ArraySeqStEph | — |
| | | | | | | | 19/ArraySeqStPer | — |
| 226 | 56 | AllPairsResultStPerF64 | yes | low | yes | 0 | 19/ArraySeqStPer | — |
| 227 | 56 | Example56_1 | no | — | no | — | — | — |
| 228 | 56 | Example56_3 | no | — | no | — | — | — |
| 229 | 57 | StackStEph | yes | med | yes | 0 | — | — |
| 230 | 57 | DijkstraStEphI64 | no | — | no | — | — | — |
| 231 | 57 | DijkstraStEphF64 | no | — | no | — | — | — |
| 232 | 58 | BellmanFordStEphI64 | yes | low | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabDirGraphStEph | — |
| | | | | | | | 06/WeightedDirGraphStEphI128 | — |
| | | | | | | | 56/SSSPResultStEphI64 | — |
| 233 | 58 | BellmanFordStEphF64 | no | — | no | — | — | — |
| 234 | 59 | JohnsonStEphI64 | no | — | no | — | — | — |
| 235 | 59 | JohnsonMtEphI64 | no | — | no | — | — | — |
| 236 | 59 | JohnsonStEphF64 | no | — | no | — | — | — |
| 237 | 59 | JohnsonMtEphF64 | no | — | no | — | — | — |
| 238 | 61 | EdgeContractionStEph | no | — | no | — | — | — |
| 239 | 61 | EdgeContractionMtEph | no | — | no | — | — | — |
| 240 | 61 | VertexMatchingStEph | no | — | no | — | — | — |
| 241 | 61 | VertexMatchingMtEph | no | — | no | — | — | — |
| 242 | 62 | StarPartitionStEph | yes | low | yes | 0 | 05/SetStEph | 62/StarContractionStEph |
| | | | | | | | 06/UnDirGraphStEph | 63/ConnectivityStEph |
| 243 | 62 | StarPartitionMtEph | no | — | no | — | — | — |
| 244 | 62 | StarContractionStEph | yes | low | yes | 0 | 05/SetStEph | 63/ConnectivityStEph |
| | | | | | | | 06/UnDirGraphStEph | 64/SpanTreeStEph |
| | | | | | | | 62/StarPartitionStEph | — |
| 245 | 62 | StarContractionMtEph | no | — | no | — | — | — |
| 246 | 63 | ConnectivityStEph | yes | low | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/UnDirGraphStEph | — |
| | | | | | | | 62/StarContractionStEph | — |
| | | | | | | | 62/StarPartitionStEph | — |
| 247 | 63 | ConnectivityMtEph | no | — | no | — | — | — |
| 248 | 64 | SpanTreeStEph | yes | low | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/UnDirGraphStEph | — |
| | | | | | | | 62/StarContractionStEph | — |
| 249 | 64 | SpanTreeMtEph | yes | low | yes | 4 | 05/SetStEph | — |
| | | | | | | | 06/UnDirGraphMtEph | — |
| | | | | | | | 18/ArraySeqStPer | — |
| | | | | | | | 62/StarContractionMtEph | — |
| 250 | 64 | TSPApproxStEph | yes | low | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabUnDirGraphStEph | — |
| 251 | 65 | UnionFindStEph | yes | med | yes | 0 | — | 65/KruskalStEph |
| 252 | 65 | KruskalStEph | yes | low | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabUnDirGraphStEph | — |
| | | | | | | | 65/UnionFindStEph | — |
| 253 | 65 | PrimStEph | yes | low | yes | 0 | 05/SetStEph | — |
| | | | | | | | 06/LabUnDirGraphStEph | — |
| | | | | | | | 45/BinaryHeapPQ | — |
| 254 | 66 | BoruvkaStEph | yes | low | yes | 2 | 05/SetStEph | — |
| 255 | 66 | BoruvkaMtEph | yes | low | yes | 2 | 05/SetStEph | — |

**Verusified**: code inside verus! blocks with specs

**Specs**: high (>1.5 spec lines/fn), med (0.5-1.5), low (<0.5)

**Blocked By / Blocks**: NN/Module format (chapter/file)
