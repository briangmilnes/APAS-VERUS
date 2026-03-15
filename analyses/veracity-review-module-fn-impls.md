<style>
  body { max-width: 98%; margin: auto; font-size: 16px; }
  table { width: 100%; border-collapse: collapse; }
  th, td { padding: 4px 8px; }
</style>

# Module Function Implementations Review

## Specification Summary by Module

| Abbr | Meaning |
|------|---------|
| Tr | declared in a `trait` block |
| IT | in `impl Trait for Type` |
| IBI | in bare `impl Type` |
| ML | module-level free fn |
| V! | inside `verus!` macro |
| -V! | outside `verus!` macro |
| Unk | has requires/ensures (strength not assessed) |
| Hole | contains `assume()`, `admit()`, or `#[verifier::external_body]` |
| NoSpec | no spec |

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap02 | FibonacciHFScheduler | 0 | 0 | 0 | 7 | 7 | 0 | 7 | 0 | 0 |
| 2 | Chap02 | HFSchedulerMtEph | 0 | 0 | 0 | 9 | 5 | 4 | 0 | 5 | 4 |
| 3 | Chap03 | InsertionSortStEph | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 4 | Chap05 | KleeneStPer | 4 | 4 | 0 | 9 | 13 | 0 | 13 | 0 | 0 |
| 5 | Chap05 | MappingStEph | 13 | 16 | 0 | 0 | 16 | 0 | 14 | 1 | 1 |
| 6 | Chap05 | RelationStEph | 9 | 12 | 0 | 0 | 12 | 0 | 11 | 0 | 1 |
| 7 | Chap05 | SetMtEph | 17 | 20 | 0 | 1 | 21 | 0 | 20 | 0 | 1 |
| 8 | Chap05 | SetStEph | 18 | 21 | 0 | 1 | 22 | 0 | 21 | 0 | 1 |
| 9 | Chap06 | DirGraphMtEph | 23 | 24 | 0 | 0 | 24 | 0 | 24 | 0 | 0 |
| 10 | Chap06 | DirGraphStEph | 17 | 18 | 2 | 0 | 20 | 0 | 20 | 0 | 0 |
| 11 | Chap06 | LabDirGraphMtEph | 14 | 14 | 0 | 0 | 14 | 0 | 14 | 0 | 0 |
| 12 | Chap06 | LabDirGraphStEph | 11 | 11 | 0 | 0 | 11 | 0 | 11 | 0 | 0 |
| 13 | Chap06 | LabUnDirGraphMtEph | 11 | 11 | 0 | 0 | 11 | 0 | 11 | 0 | 0 |
| 14 | Chap06 | LabUnDirGraphStEph | 10 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 15 | Chap06 | UnDirGraphMtEph | 14 | 15 | 0 | 0 | 15 | 0 | 15 | 0 | 0 |
| 16 | Chap06 | UnDirGraphStEph | 11 | 12 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 17 | Chap06 | WeightedDirGraphStEphI128 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 18 | Chap06 | WeightedDirGraphStEphI16 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 19 | Chap06 | WeightedDirGraphStEphI32 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 20 | Chap06 | WeightedDirGraphStEphI64 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 21 | Chap06 | WeightedDirGraphStEphI8 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 22 | Chap06 | WeightedDirGraphStEphIsize | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 23 | Chap06 | WeightedDirGraphStEphU128 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 24 | Chap06 | WeightedDirGraphStEphU16 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 25 | Chap06 | WeightedDirGraphStEphU32 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 26 | Chap06 | WeightedDirGraphStEphU64 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 27 | Chap06 | WeightedDirGraphStEphU8 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 28 | Chap06 | WeightedDirGraphStEphUsize | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 29 | Chap11 | FibonacciMtEph2Threads | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 30 | Chap11 | FibonacciMtEphRecomputes | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 31 | Chap11 | FibonacciMtPerAllThreads | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 32 | Chap11 | FibonacciMtPerTSM | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 33 | Chap11 | FibonacciStEph | 0 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |
| 34 | Chap12 | Exercise12_1 | 4 | 5 | 0 | 1 | 6 | 0 | 0 | 5 | 1 |
| 35 | Chap12 | Exercise12_2 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 36 | Chap12 | Exercise12_5 | 5 | 7 | 0 | 0 | 7 | 0 | 0 | 6 | 1 |
| 37 | Chap17 | MathSeq | 18 | 20 | 1 | 0 | 20 | 1 | 20 | 0 | 1 |
| 38 | Chap18 | ArraySeq | 23 | 25 | 3 | 13 | 41 | 0 | 40 | 0 | 1 |
| 39 | Chap18 | ArraySeqMtEph | 22 | 24 | 7 | 3 | 34 | 0 | 34 | 0 | 0 |
| 40 | Chap18 | ArraySeqMtPer | 19 | 21 | 6 | 0 | 27 | 0 | 27 | 0 | 0 |
| 41 | Chap18 | ArraySeqStEph | 21 | 23 | 2 | 0 | 25 | 0 | 25 | 0 | 0 |
| 42 | Chap18 | ArraySeqStPer | 20 | 22 | 2 | 0 | 24 | 0 | 24 | 0 | 0 |
| 43 | Chap18 | LinkedListStEph | 19 | 21 | 2 | 0 | 23 | 0 | 23 | 0 | 0 |
| 44 | Chap18 | LinkedListStPer | 18 | 20 | 2 | 0 | 22 | 0 | 22 | 0 | 0 |
| 45 | Chap19 | ArraySeqMtEph | 25 | 27 | 6 | 4 | 37 | 0 | 37 | 0 | 0 |
| 46 | Chap19 | ArraySeqMtEphSlice | 8 | 8 | 1 | 0 | 9 | 0 | 9 | 0 | 0 |
| 47 | Chap19 | ArraySeqStEph | 24 | 26 | 3 | 2 | 31 | 0 | 31 | 0 | 0 |
| 48 | Chap19 | ArraySeqStPer | 23 | 25 | 2 | 2 | 29 | 0 | 29 | 0 | 0 |
| 49 | Chap21 | Algorithm21_1 | 0 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 50 | Chap21 | Algorithm21_2 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 51 | Chap21 | Algorithm21_5 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 52 | Chap21 | Algorithm21_6 | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 53 | Chap21 | Exercise21_5 | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 54 | Chap21 | Exercise21_7 | 0 | 0 | 0 | 3 | 3 | 0 | 3 | 0 | 0 |
| 55 | Chap21 | Exercise21_8 | 0 | 0 | 0 | 7 | 7 | 0 | 7 | 0 | 0 |
| 56 | Chap21 | Exercise21_9 | 0 | 0 | 0 | 3 | 3 | 0 | 3 | 0 | 0 |
| 57 | Chap21 | Problem21_1 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 58 | Chap21 | Problem21_3 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 59 | Chap21 | Problem21_4 | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 60 | Chap23 | BalBinTreeStEph | 8 | 10 | 3 | 3 | 16 | 0 | 16 | 0 | 0 |
| 61 | Chap23 | PrimTreeSeqStPer | 17 | 19 | 1 | 0 | 20 | 0 | 20 | 0 | 0 |
| 62 | Chap26 | DivConReduceMtPer | 5 | 5 | 0 | 3 | 8 | 0 | 8 | 0 | 0 |
| 63 | Chap26 | DivConReduceStPer | 5 | 5 | 0 | 0 | 5 | 0 | 5 | 0 | 0 |
| 64 | Chap26 | ETSPMtEph | 2 | 2 | 0 | 10 | 8 | 4 | 8 | 0 | 4 |
| 65 | Chap26 | ETSPStEph | 2 | 2 | 0 | 8 | 7 | 3 | 7 | 0 | 3 |
| 66 | Chap26 | MergeSortMtPer | 2 | 2 | 0 | 6 | 8 | 0 | 8 | 0 | 0 |
| 67 | Chap26 | MergeSortStPer | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |
| 68 | Chap26 | ScanDCMtPer | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 69 | Chap26 | ScanDCStPer | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |
| 70 | Chap27 | ReduceContractMtEph | 1 | 1 | 0 | 5 | 6 | 0 | 6 | 0 | 0 |
| 71 | Chap27 | ReduceContractStEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 72 | Chap27 | ScanContractMtEph | 2 | 2 | 0 | 8 | 10 | 0 | 10 | 0 | 0 |
| 73 | Chap27 | ScanContractStEph | 2 | 2 | 0 | 8 | 10 | 0 | 10 | 0 | 0 |
| 74 | Chap28 | MCSSSpec | 0 | 0 | 0 | 10 | 10 | 0 | 10 | 0 | 0 |
| 75 | Chap28 | MaxContigSubSumBruteStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 76 | Chap28 | MaxContigSubSumDivConMtEph | 1 | 1 | 0 | 3 | 4 | 0 | 4 | 0 | 0 |
| 77 | Chap28 | MaxContigSubSumDivConOptMtEph | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 78 | Chap28 | MaxContigSubSumDivConOptStEph | 1 | 1 | 0 | 3 | 4 | 0 | 4 | 0 | 0 |
| 79 | Chap28 | MaxContigSubSumDivConStEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 80 | Chap28 | MaxContigSubSumIterStEph | 1 | 1 | 0 | 3 | 4 | 0 | 4 | 0 | 0 |
| 81 | Chap28 | MaxContigSubSumOptMtEph | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 0 | 0 |
| 82 | Chap28 | MaxContigSubSumOptStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 83 | Chap28 | MaxContigSubSumReducedMcsseStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 84 | Chap28 | MaxContigSubSumReducedStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 85 | Chap30 | Probability | 4 | 14 | 0 | 0 | 14 | 0 | 0 | 10 | 4 |
| 86 | Chap35 | OrderStatSelectMtEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 87 | Chap35 | OrderStatSelectMtPer | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 88 | Chap35 | OrderStatSelectStEph | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 89 | Chap35 | OrderStatSelectStPer | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 90 | Chap36 | QuickSortMtEph | 6 | 6 | 0 | 2 | 8 | 0 | 8 | 0 | 0 |
| 91 | Chap36 | QuickSortMtEphSlice | 6 | 6 | 0 | 3 | 9 | 0 | 9 | 0 | 0 |
| 92 | Chap36 | QuickSortStEph | 6 | 6 | 0 | 2 | 8 | 0 | 8 | 0 | 0 |
| 93 | Chap37 | AVLTreeSeq | 20 | 23 | 0 | 14 | 37 | 0 | 34 | 2 | 1 |
| 94 | Chap37 | AVLTreeSeqMtPer | 11 | 14 | 0 | 14 | 28 | 0 | 23 | 3 | 2 |
| 95 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 0 | 14 | 35 | 0 | 31 | 3 | 1 |
| 96 | Chap37 | AVLTreeSeqStPer | 13 | 16 | 0 | 14 | 30 | 0 | 24 | 5 | 1 |
| 97 | Chap37 | BSTAVLMtEph | 11 | 11 | 0 | 8 | 19 | 0 | 19 | 0 | 0 |
| 98 | Chap37 | BSTAVLStEph | 7 | 7 | 0 | 10 | 17 | 0 | 17 | 0 | 0 |
| 99 | Chap37 | BSTBBAlphaMtEph | 11 | 11 | 0 | 5 | 16 | 0 | 16 | 0 | 0 |
| 100 | Chap37 | BSTBBAlphaStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 101 | Chap37 | BSTPlainMtEph | 11 | 11 | 0 | 5 | 16 | 0 | 16 | 0 | 0 |
| 102 | Chap37 | BSTPlainStEph | 10 | 10 | 0 | 10 | 20 | 0 | 20 | 0 | 0 |
| 103 | Chap37 | BSTRBMtEph | 14 | 15 | 0 | 21 | 36 | 0 | 35 | 0 | 1 |
| 104 | Chap37 | BSTRBStEph | 7 | 7 | 0 | 8 | 15 | 0 | 15 | 0 | 0 |
| 105 | Chap37 | BSTSetAVLMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 26 | 0 | 0 |
| 106 | Chap37 | BSTSetBBAlphaMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 26 | 0 | 0 |
| 107 | Chap37 | BSTSetPlainMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 26 | 0 | 0 |
| 108 | Chap37 | BSTSetRBMtEph | 21 | 22 | 0 | 3 | 25 | 0 | 25 | 0 | 0 |
| 109 | Chap37 | BSTSetSplayMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 26 | 0 | 0 |
| 110 | Chap37 | BSTSplayMtEph | 14 | 15 | 0 | 18 | 33 | 0 | 32 | 0 | 1 |
| 111 | Chap37 | BSTSplayStEph | 11 | 12 | 0 | 12 | 24 | 0 | 17 | 6 | 1 |
| 112 | Chap38 | BSTParaMtEph | 17 | 17 | 0 | 16 | 18 | 14 | 9 | 7 | 16 |
| 113 | Chap38 | BSTParaStEph | 20 | 20 | 0 | 8 | 28 | 0 | 25 | 3 | 0 |
| 114 | Chap39 | BSTParaTreapMtEph | 17 | 17 | 0 | 16 | 18 | 15 | 9 | 9 | 15 |
| 115 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 22 | 0 | 0 |
| 116 | Chap39 | BSTTreapMtEph | 12 | 13 | 0 | 20 | 33 | 0 | 24 | 7 | 2 |
| 117 | Chap39 | BSTTreapStEph | 34 | 35 | 0 | 0 | 35 | 0 | 33 | 2 | 0 |
| 118 | Chap40 | BSTKeyValueStEph | 26 | 28 | 0 | 9 | 37 | 0 | 32 | 5 | 0 |
| 119 | Chap40 | BSTReducedStEph | 36 | 38 | 0 | 3 | 41 | 0 | 34 | 7 | 0 |
| 120 | Chap40 | BSTSizeStEph | 31 | 33 | 0 | 5 | 38 | 0 | 34 | 4 | 0 |
| 121 | Chap41 | AVLTreeSetMtEph | 13 | 15 | 0 | 0 | 15 | 0 | 7 | 7 | 1 |
| 122 | Chap41 | AVLTreeSetMtPer | 12 | 16 | 0 | 0 | 16 | 0 | 4 | 10 | 2 |
| 123 | Chap41 | AVLTreeSetStEph | 12 | 14 | 0 | 1 | 15 | 0 | 11 | 3 | 1 |
| 124 | Chap41 | AVLTreeSetStPer | 12 | 14 | 0 | 0 | 14 | 0 | 11 | 2 | 1 |
| 125 | Chap41 | ArraySetEnumMtEph | 13 | 14 | 0 | 7 | 21 | 0 | 20 | 1 | 0 |
| 126 | Chap41 | ArraySetStEph | 12 | 14 | 0 | 7 | 21 | 0 | 18 | 2 | 1 |
| 127 | Chap41 | Example41_3 | 3 | 3 | 0 | 8 | 9 | 0 | 5 | 4 | 0 |
| 128 | Chap42 | Example42_1 | 2 | 0 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| 129 | Chap42 | TableMtEph | 16 | 17 | 0 | 11 | 28 | 0 | 24 | 4 | 0 |
| 130 | Chap42 | TableStEph | 16 | 18 | 0 | 9 | 27 | 0 | 20 | 6 | 1 |
| 131 | Chap42 | TableStPer | 16 | 17 | 0 | 14 | 31 | 0 | 26 | 5 | 0 |
| 132 | Chap43 | AugOrderedTableMtEph | 32 | 33 | 1 | 3 | 36 | 1 | 28 | 8 | 1 |
| 133 | Chap43 | AugOrderedTableStEph | 31 | 32 | 1 | 2 | 34 | 1 | 25 | 9 | 1 |
| 134 | Chap43 | AugOrderedTableStPer | 28 | 28 | 2 | 2 | 32 | 0 | 24 | 8 | 0 |
| 135 | Chap43 | Example43_1 | 2 | 0 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| 136 | Chap43 | OrderedSetMtEph | 22 | 22 | 0 | 1 | 23 | 0 | 10 | 13 | 0 |
| 137 | Chap43 | OrderedSetStEph | 22 | 24 | 1 | 1 | 24 | 2 | 13 | 11 | 2 |
| 138 | Chap43 | OrderedSetStPer | 22 | 24 | 1 | 1 | 25 | 1 | 17 | 8 | 1 |
| 139 | Chap43 | OrderedTableMtEph | 29 | 31 | 1 | 1 | 32 | 1 | 25 | 7 | 1 |
| 140 | Chap43 | OrderedTableMtPer | 19 | 20 | 0 | 1 | 20 | 1 | 11 | 8 | 2 |
| 141 | Chap43 | OrderedTableStEph | 29 | 31 | 1 | 1 | 32 | 1 | 20 | 12 | 1 |
| 142 | Chap43 | OrderedTableStPer | 26 | 28 | 1 | 1 | 30 | 0 | 20 | 10 | 0 |
| 143 | Chap44 | DocumentIndex | 15 | 16 | 0 | 3 | 2 | 17 | 1 | 0 | 18 |
| 144 | Chap44 | Example44_1 | 0 | 1 | 12 | 9 | 1 | 21 | 0 | 0 | 22 |
| 145 | Chap45 | BalancedTreePQ | 26 | 28 | 0 | 1 | 29 | 0 | 27 | 1 | 1 |
| 146 | Chap45 | BinaryHeapPQ | 18 | 20 | 0 | 12 | 32 | 0 | 29 | 2 | 1 |
| 147 | Chap45 | Example45_2 | 8 | 8 | 0 | 9 | 9 | 0 | 8 | 0 | 1 |
| 148 | Chap45 | HeapsortExample | 2 | 3 | 0 | 22 | 5 | 20 | 2 | 0 | 23 |
| 149 | Chap45 | LeftistHeapPQ | 24 | 27 | 0 | 7 | 31 | 3 | 29 | 0 | 5 |
| 150 | Chap45 | SortedListPQ | 19 | 21 | 0 | 1 | 22 | 0 | 20 | 0 | 2 |
| 151 | Chap45 | UnsortedListPQ | 15 | 17 | 0 | 1 | 18 | 0 | 16 | 0 | 2 |
| 152 | Chap47 | ChainedHashTable | 4 | 1 | 0 | 1 | 6 | 0 | 5 | 0 | 1 |
| 153 | Chap47 | DoubleHashFlatHashTableStEph | 0 | 0 | 7 | 0 | 7 | 0 | 1 | 4 | 2 |
| 154 | Chap47 | FlatHashTable | 4 | 5 | 0 | 0 | 9 | 0 | 8 | 0 | 1 |
| 155 | Chap47 | LinProbFlatHashTableStEph | 0 | 0 | 6 | 0 | 6 | 0 | 0 | 4 | 2 |
| 156 | Chap47 | LinkedListChainedHashTableStEph | 0 | 5 | 5 | 1 | 8 | 0 | 0 | 4 | 4 |
| 157 | Chap47 | ParaHashTableStEph | 9 | 0 | 0 | 7 | 16 | 0 | 13 | 2 | 1 |
| 158 | Chap47 | QuadProbFlatHashTableStEph | 0 | 0 | 6 | 0 | 6 | 0 | 0 | 4 | 2 |
| 159 | Chap47 | StructChainedHashTable | 0 | 7 | 5 | 4 | 13 | 0 | 4 | 4 | 5 |
| 160 | Chap47 | VecChainedHashTableStEph | 0 | 5 | 5 | 1 | 8 | 0 | 1 | 4 | 3 |
| 161 | Chap49 | MinEditDistMtEph | 11 | 12 | 0 | 3 | 12 | 3 | 11 | 0 | 4 |
| 162 | Chap49 | MinEditDistMtPer | 6 | 7 | 0 | 3 | 9 | 1 | 8 | 0 | 2 |
| 163 | Chap49 | MinEditDistStEph | 11 | 11 | 0 | 1 | 10 | 2 | 9 | 0 | 3 |
| 164 | Chap49 | MinEditDistStPer | 6 | 7 | 0 | 1 | 7 | 1 | 6 | 0 | 2 |
| 165 | Chap49 | SubsetSumMtEph | 8 | 9 | 0 | 3 | 10 | 2 | 9 | 0 | 3 |
| 166 | Chap49 | SubsetSumMtPer | 5 | 6 | 0 | 3 | 8 | 1 | 6 | 0 | 3 |
| 167 | Chap49 | SubsetSumStEph | 8 | 8 | 0 | 1 | 8 | 1 | 7 | 0 | 2 |
| 168 | Chap49 | SubsetSumStPer | 5 | 6 | 0 | 1 | 6 | 1 | 4 | 0 | 3 |
| 169 | Chap50 | MatrixChainMtEph | 13 | 14 | 0 | 0 | 14 | 0 | 12 | 0 | 2 |
| 170 | Chap50 | MatrixChainMtPer | 10 | 11 | 0 | 0 | 11 | 0 | 9 | 0 | 2 |
| 171 | Chap50 | MatrixChainStEph | 12 | 13 | 0 | 0 | 13 | 0 | 13 | 0 | 0 |
| 172 | Chap50 | MatrixChainStPer | 9 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 173 | Chap50 | OptBinSearchTreeMtEph | 10 | 11 | 0 | 1 | 12 | 0 | 10 | 0 | 2 |
| 174 | Chap50 | OptBinSearchTreeMtPer | 7 | 8 | 0 | 1 | 9 | 0 | 6 | 0 | 3 |
| 175 | Chap50 | OptBinSearchTreeStEph | 10 | 11 | 0 | 1 | 11 | 1 | 10 | 0 | 2 |
| 176 | Chap50 | OptBinSearchTreeStPer | 7 | 8 | 0 | 1 | 8 | 1 | 7 | 0 | 2 |
| 177 | Chap51 | BottomUpDPMtEph | 8 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 178 | Chap51 | BottomUpDPMtPer | 6 | 8 | 0 | 0 | 8 | 0 | 8 | 0 | 0 |
| 179 | Chap51 | BottomUpDPStEph | 10 | 12 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 180 | Chap51 | BottomUpDPStPer | 8 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 181 | Chap51 | TopDownDPMtEph | 9 | 11 | 0 | 3 | 14 | 0 | 14 | 0 | 0 |
| 182 | Chap51 | TopDownDPMtPer | 7 | 9 | 0 | 3 | 12 | 0 | 12 | 0 | 0 |
| 183 | Chap51 | TopDownDPStEph | 14 | 16 | 0 | 1 | 17 | 0 | 17 | 0 | 0 |
| 184 | Chap51 | TopDownDPStPer | 12 | 14 | 0 | 1 | 15 | 0 | 15 | 0 | 0 |
| 185 | Chap52 | AdjMatrixGraphMtEph | 9 | 9 | 0 | 3 | 12 | 0 | 12 | 0 | 0 |
| 186 | Chap52 | AdjMatrixGraphMtPer | 7 | 7 | 0 | 3 | 10 | 0 | 10 | 0 | 0 |
| 187 | Chap52 | AdjMatrixGraphStEph | 9 | 9 | 0 | 3 | 12 | 0 | 12 | 0 | 0 |
| 188 | Chap52 | AdjMatrixGraphStPer | 9 | 10 | 0 | 3 | 13 | 0 | 13 | 0 | 0 |
| 189 | Chap52 | AdjSeqGraphMtEph | 7 | 7 | 0 | 2 | 9 | 0 | 9 | 0 | 0 |
| 190 | Chap52 | AdjSeqGraphMtPer | 6 | 6 | 0 | 2 | 8 | 0 | 8 | 0 | 0 |
| 191 | Chap52 | AdjSeqGraphStEph | 9 | 9 | 0 | 2 | 11 | 0 | 11 | 0 | 0 |
| 192 | Chap52 | AdjSeqGraphStPer | 9 | 10 | 0 | 2 | 12 | 0 | 12 | 0 | 0 |
| 193 | Chap52 | AdjTableGraphMtPer | 10 | 11 | 0 | 0 | 11 | 0 | 10 | 0 | 1 |
| 194 | Chap52 | AdjTableGraphStEph | 12 | 12 | 0 | 1 | 13 | 0 | 13 | 0 | 0 |
| 195 | Chap52 | AdjTableGraphStPer | 12 | 12 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 196 | Chap52 | EdgeSetGraphMtPer | 13 | 14 | 0 | 0 | 14 | 0 | 11 | 2 | 1 |
| 197 | Chap52 | EdgeSetGraphStEph | 13 | 13 | 0 | 0 | 13 | 0 | 12 | 1 | 0 |
| 198 | Chap52 | EdgeSetGraphStPer | 13 | 13 | 0 | 0 | 13 | 0 | 11 | 2 | 0 |
| 199 | Chap53 | GraphSearchMtPer | 4 | 4 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 200 | Chap53 | GraphSearchStEph | 4 | 4 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 201 | Chap53 | GraphSearchStPer | 4 | 4 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 202 | Chap53 | PQMinStEph | 2 | 2 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 203 | Chap53 | PQMinStPer | 2 | 2 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 204 | Chap54 | BFSMtEph | 4 | 4 | 0 | 11 | 15 | 0 | 15 | 0 | 0 |
| 205 | Chap54 | BFSMtPer | 4 | 4 | 0 | 11 | 15 | 0 | 15 | 0 | 0 |
| 206 | Chap54 | BFSStEph | 4 | 4 | 0 | 4 | 8 | 0 | 8 | 0 | 0 |
| 207 | Chap54 | BFSStPer | 4 | 4 | 0 | 4 | 8 | 0 | 8 | 0 | 0 |
| 208 | Chap55 | CycleDetectStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 209 | Chap55 | CycleDetectStPer | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 210 | Chap55 | DFSStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 211 | Chap55 | DFSStPer | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 212 | Chap55 | SCCStEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 213 | Chap55 | SCCStPer | 1 | 1 | 0 | 5 | 6 | 0 | 6 | 0 | 0 |
| 214 | Chap55 | TopoSortStEph | 1 | 1 | 0 | 7 | 8 | 0 | 8 | 0 | 0 |
| 215 | Chap55 | TopoSortStPer | 1 | 1 | 0 | 3 | 4 | 0 | 4 | 0 | 0 |
| 216 | Chap56 | AllPairsResultStEphF64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 217 | Chap56 | AllPairsResultStEphI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 218 | Chap56 | AllPairsResultStPerF64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 219 | Chap56 | AllPairsResultStPerI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 220 | Chap56 | Example56_1 | 3 | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 3 |
| 221 | Chap56 | Example56_3 | 2 | 2 | 0 | 0 | 2 | 0 | 0 | 0 | 2 |
| 222 | Chap56 | PathWeightUtilsStEph | 4 | 4 | 0 | 0 | 4 | 0 | 2 | 0 | 2 |
| 223 | Chap56 | PathWeightUtilsStPer | 4 | 4 | 0 | 0 | 4 | 0 | 2 | 0 | 2 |
| 224 | Chap56 | SSSPResultStEphF64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 225 | Chap56 | SSSPResultStEphI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 226 | Chap56 | SSSPResultStPerF64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 227 | Chap56 | SSSPResultStPerI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 228 | Chap57 | DijkstraStEphF64 | 0 | 2 | 0 | 0 | 0 | 2 | 0 | 0 | 2 |
| 229 | Chap57 | DijkstraStEphI64 | 1 | 2 | 0 | 2 | 4 | 0 | 2 | 0 | 2 |
| 230 | Chap57 | StackStEph | 6 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 231 | Chap58 | BellmanFordStEphI64 | 1 | 0 | 0 | 2 | 1 | 1 | 0 | 0 | 2 |
| 232 | Chap59 | JohnsonMtEphI64 | 1 | 0 | 0 | 5 | 1 | 4 | 0 | 0 | 5 |
| 233 | Chap59 | JohnsonStEphI64 | 1 | 0 | 0 | 4 | 1 | 3 | 0 | 0 | 4 |
| 234 | Chap61 | EdgeContractionMtEph | 2 | 0 | 0 | 3 | 2 | 1 | 2 | 0 | 1 |
| 235 | Chap61 | EdgeContractionStEph | 2 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 236 | Chap61 | VertexMatchingMtEph | 1 | 0 | 0 | 5 | 1 | 4 | 1 | 0 | 4 |
| 237 | Chap61 | VertexMatchingStEph | 2 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 238 | Chap62 | StarContractionMtEph | 2 | 0 | 0 | 4 | 2 | 2 | 2 | 0 | 2 |
| 239 | Chap62 | StarContractionStEph | 2 | 0 | 0 | 3 | 2 | 1 | 2 | 0 | 1 |
| 240 | Chap62 | StarPartitionMtEph | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 241 | Chap62 | StarPartitionStEph | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 242 | Chap63 | ConnectivityMtEph | 4 | 0 | 0 | 7 | 4 | 3 | 4 | 0 | 3 |
| 243 | Chap63 | ConnectivityStEph | 4 | 0 | 0 | 5 | 4 | 1 | 4 | 0 | 1 |
| 244 | Chap64 | SpanTreeMtEph | 2 | 0 | 0 | 3 | 3 | 0 | 3 | 0 | 0 |
| 245 | Chap64 | SpanTreeStEph | 2 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 246 | Chap64 | TSPApproxStEph | 4 | 0 | 0 | 7 | 4 | 3 | 3 | 0 | 4 |
| 247 | Chap65 | KruskalStEph | 3 | 0 | 0 | 3 | 3 | 0 | 2 | 0 | 1 |
| 248 | Chap65 | PrimStEph | 2 | 2 | 0 | 3 | 2 | 3 | 1 | 0 | 4 |
| 249 | Chap65 | UnionFindStEph | 6 | 6 | 0 | 0 | 6 | 0 | 6 | 0 | 0 |
| 250 | Chap66 | BoruvkaMtEph | 5 | 4 | 0 | 12 | 6 | 10 | 2 | 0 | 14 |
| 251 | Chap66 | BoruvkaStEph | 5 | 9 | 0 | 1 | 7 | 3 | 6 | 0 | 4 |
| 252 | src | Concurrency | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 0 |
| 253 | src | ParaPairs | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 254 | src | Types | 0 | 4 | 0 | 15 | 15 | 4 | 2 | 13 | 4 |
| 255 | standards | arc_usage_standard | 2 | 2 | 0 | 1 | 3 | 0 | 1 | 1 | 1 |
| 256 | standards | deep_view_standard | 0 | 1 | 1 | 2 | 4 | 0 | 4 | 0 | 0 |
| 257 | standards | finite_sets_standard | 3 | 3 | 0 | 0 | 3 | 0 | 3 | 0 | 0 |
| 258 | standards | hfscheduler_standard | 0 | 0 | 0 | 6 | 6 | 0 | 1 | 2 | 3 |
| 259 | standards | iterators_standard | 1 | 2 | 1 | 0 | 3 | 0 | 3 | 0 | 0 |
| 260 | standards | mod_standard | 2 | 3 | 1 | 0 | 4 | 0 | 4 | 0 | 0 |
| 261 | standards | multi_struct_standard | 2 | 2 | 0 | 0 | 2 | 0 | 2 | 0 | 0 |
| 262 | standards | mut_standard | 6 | 6 | 0 | 7 | 13 | 0 | 9 | 0 | 4 |
| 263 | standards | partial_eq_eq_clone_standard | 0 | 1 | 0 | 0 | 1 | 0 | 1 | 0 | 0 |
| 264 | standards | spec_naming_convention | 4 | 4 | 0 | 0 | 4 | 0 | 1 | 3 | 0 |
| 265 | standards | spec_wf_standard | 5 | 5 | 0 | 0 | 5 | 0 | 5 | 0 | 0 |
| 266 | standards | table_of_contents_standard | 4 | 6 | 1 | 1 | 8 | 0 | 7 | 1 | 0 |
| 267 | standards | toplevel_coarse_rwlocks_for_mt_modules | 4 | 4 | 0 | 0 | 4 | 0 | 4 | 0 | 0 |
| 268 | standards | tsm_standard | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 269 | standards | using_closures_standard | 3 | 3 | 0 | 0 | 3 | 0 | 3 | 0 | 0 |
| 270 | standards | view_standard | 4 | 5 | 1 | 0 | 6 | 0 | 6 | 0 | 0 |
| 271 | standards | wrapping_iterators_standard | 1 | 2 | 1 | 0 | 3 | 0 | 3 | 0 | 0 |

## Function-by-Function Detail

### Chap02/FibonacciHFScheduler.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_pow2_mono` |  |  |  | Y | Y |  |  | unknown | 24&#8209;26 |
| 2 | `lemma_pow2_46_lt_u64_max` |  |  |  | Y | Y |  |  | unknown | 31&#8209;32 |
| 3 | `lemma_fib_bound` |  |  |  | Y | Y |  |  | unknown | 37&#8209;39 |
| 4 | `lemma_fib_fits_u64` |  |  |  | Y | Y |  |  | unknown | 54&#8209;56 |
| 5 | `lemma_fib_sum_fits_u64` |  |  |  | Y | Y |  |  | unknown | 63&#8209;65 |
| 6 | `fib_seq` |  |  |  | Y | Y |  |  | unknown | 72&#8209;75 |
| 7 | `fib_par` |  |  |  | Y | Y |  |  | unknown | 87&#8209;90 |

### Chap02/HFSchedulerMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `set_parallelism` |  |  |  | Y | Y |  |  | hole | 88 |
| 9 | `join` |  |  |  | Y | Y |  |  | hole | 98&#8209;109 |
| 10 | `spawn_join` |  |  |  | Y | Y |  |  | hole | 125&#8209;136 |
| 11 | `spawn` |  |  |  | Y | Y |  |  | hole | 156&#8209;163 |
| 12 | `wait` |  |  |  | Y | Y |  |  | hole | 178&#8209;180 |
| 13 | `init_pool` |  |  |  | Y |  | Y | Y |  | 33&#8209;45 |
| 14 | `try_acquire` |  |  |  | Y |  | Y | Y |  | 49&#8209;57 |
| 15 | `acquire` |  |  |  | Y |  | Y | Y |  | 59&#8209;65 |
| 16 | `release` |  |  |  | Y |  | Y | Y |  | 67&#8209;71 |

### Chap03/InsertionSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `insertion_sort` |  |  |  | Y | Y |  |  | unknown | 38&#8209;43 |

### Chap05/KleeneStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 18 | `lemma_star_closed_under_concat` |  |  |  | Y | Y |  |  | unknown | 83&#8209;88 |
| 19 | `lemma_plus_closed_under_concat` |  |  |  | Y | Y |  |  | unknown | 101&#8209;106 |
| 20 | `ptt_star_contains_empty` |  |  |  | Y | Y |  |  | unknown | 113&#8209;114 |
| 21 | `ptt_plus_rejects_empty` |  |  |  | Y | Y |  |  | unknown | 119&#8209;120 |
| 22 | `ptt_singleton_in_star_and_plus` |  |  |  | Y | Y |  |  | unknown | 125&#8209;129 |
| 23 | `ptt_plus_subset_of_star` |  |  |  | Y | Y |  |  | unknown | 134&#8209;136 |
| 24 | `ptt_star_property_transfer` |  |  |  | Y | Y |  |  | unknown | 142&#8209;151 |
| 25 | `ptt_star_concat_plus_is_plus` |  |  |  | Y | Y |  |  | unknown | 157&#8209;162 |
| 26 | `ptt_plus_concat_star_is_plus` |  |  |  | Y | Y |  |  | unknown | 171&#8209;176 |
| 27 | `new` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;197 |
| 28 | `mem_star` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;204 |
| 29 | `mem_plus` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;211 |
| 30 | `alphabet` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;218 |

### Chap05/MappingStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `is_functional_vec` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 32 | `is_functional_vec_at` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 33 | `is_functional_SetStEph_at` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 34 | `is_functional_SetStEph` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 35 | `is_functional_RelationStEph` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;169 |
| 36 | `empty` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;177 |
| 37 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;186 |
| 38 | `from_relation` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;195 |
| 39 | `size` | Y | Y |  |  | Y |  |  | hole | 199&#8209;201 |
| 40 | `domain` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;207 |
| 41 | `range` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;217 |
| 42 | `mem` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;226 |
| 43 | `iter` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;236 |
| 44 | `next` |  | Y |  |  | Y |  |  | unknown | 478&#8209;494 |
| 45 | `hash` |  | Y |  |  | Y |  | Y |  | 589 |
| 46 | `eq` |  | Y |  |  | Y |  |  | unknown | 595&#8209;596 |

### Chap05/RelationStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 47 | `empty` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 48 | `from_set` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 49 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 50 | `size` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 51 | `domain` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 52 | `range` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 53 | `mem` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;140 |
| 54 | `relates` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;146 |
| 55 | `iter` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;155 |
| 56 | `next` |  | Y |  |  | Y |  |  | unknown | 300&#8209;316 |
| 57 | `hash` |  | Y |  |  | Y |  | Y |  | 410 |
| 58 | `eq` |  | Y |  |  | Y |  |  | unknown | 416&#8209;417 |

### Chap05/SetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 59 | `lemma_singleton_choose` |  |  |  | Y | Y |  |  | unknown | 114&#8209;120 |
| 60 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;146 |
| 61 | `iter` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;155 |
| 62 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;163 |
| 63 | `empty` x3 | Y | Y |  |  | Y |  |  | unknown | 1011&#8209;1013 |
| 64 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;175 |
| 65 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 1015&#8209;1016 |
| 66 | `mem` x3 | Y | Y |  |  | Y |  |  | unknown | 1018&#8209;1019 |
| 67 | `insert` x3 | Y | Y |  |  | Y |  |  | unknown | 1021&#8209;1025 |
| 68 | `union` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;204 |
| 69 | `disjoint_union` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;217 |
| 70 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;225 |
| 71 | `elt_cross_set` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;236 |
| 72 | `cartesian_product` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;248 |
| 73 | `all_nonempty` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;257 |
| 74 | `partition_on_elt` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;272 |
| 75 | `partition` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;290 |
| 76 | `choose` x3 | Y | Y |  |  | Y |  |  | unknown | 1027&#8209;1029 |
| 77 | `next` |  | Y |  |  | Y |  |  | unknown | 874&#8209;890 |
| 78 | `hash` |  | Y |  |  | Y |  | Y |  | 1089 |
| 79 | `eq` |  | Y |  |  | Y |  |  | unknown | 1095&#8209;1096 |

### Chap05/SetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 80 | `lemma_singleton_choose` |  |  |  | Y | Y |  |  | unknown | 106&#8209;112 |
| 81 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 82 | `iter` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;142 |
| 83 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;150 |
| 84 | `empty` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 85 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;162 |
| 86 | `size` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 87 | `mem` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;174 |
| 88 | `insert` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;183 |
| 89 | `union` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;191 |
| 90 | `disjoint_union` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;204 |
| 91 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;212 |
| 92 | `elt_cross_set` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;223 |
| 93 | `cartesian_product` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;234 |
| 94 | `all_nonempty` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;243 |
| 95 | `partition_on_elt` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;258 |
| 96 | `partition` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;276 |
| 97 | `split` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;293 |
| 98 | `choose` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;304 |
| 99 | `next` |  | Y |  |  | Y |  |  | unknown | 790&#8209;806 |
| 100 | `hash` |  | Y |  |  | Y |  | Y |  | 885 |
| 101 | `eq` |  | Y |  |  | Y |  |  | unknown | 891&#8209;892 |

### Chap06/DirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 102 | `empty` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;106 |
| 103 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;120 |
| 104 | `vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 742&#8209;744 |
| 105 | `arcs` x3 | Y | Y |  |  | Y |  |  | unknown | 746&#8209;748 |
| 106 | `sizeV` x3 | Y | Y |  |  | Y |  |  | unknown | 750&#8209;752 |
| 107 | `sizeA` x3 | Y | Y |  |  | Y |  |  | unknown | 754&#8209;756 |
| 108 | `neighbor` x3 | Y | Y |  |  | Y |  |  | unknown | 758&#8209;763 |
| 109 | `incident` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 110 | `n_plus` x3 | Y | Y |  |  | Y |  |  | unknown | 765&#8209;772 |
| 111 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;193 |
| 112 | `n_minus` x3 | Y | Y |  |  | Y |  |  | unknown | 774&#8209;781 |
| 113 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;228 |
| 114 | `ng` x3 | Y | Y |  |  | Y |  |  | unknown | 783&#8209;789 |
| 115 | `degree` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;261 |
| 116 | `n_plus_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 791&#8209;797 |
| 117 | `n_minus_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 799&#8209;805 |
| 118 | `ng_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 807&#8209;813 |
| 119 | `n_plus_par` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;346 |
| 120 | `n_minus_par` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;359 |
| 121 | `n_plus_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;372 |
| 122 | `n_minus_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 375&#8209;385 |
| 123 | `ng_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;398 |
| 124 | `new` | Y | Y |  |  | Y |  |  | unknown | 730&#8209;740 |
| 125 | `eq` |  | Y |  |  | Y |  |  | unknown | 943&#8209;944 |

### Chap06/DirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 126 | `empty` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;119 |
| 127 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;131 |
| 128 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;136 |
| 129 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 130 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 131 | `sizeA` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 132 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 133 | `ng` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;165 |
| 134 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 135 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;177 |
| 136 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 137 | `n_plus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 138 | `n_minus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;195 |
| 139 | `incident` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;201 |
| 140 | `degree` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;207 |
| 141 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 142 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;219 |
| 143 | `iter_vertices` |  |  | Y |  | Y |  |  | unknown | 227&#8209;229 |
| 144 | `iter_arcs` |  |  | Y |  | Y |  |  | unknown | 233&#8209;235 |
| 145 | `eq` |  | Y |  |  | Y |  |  | unknown | 594&#8209;595 |

### Chap06/LabDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 146 | `empty` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;105 |
| 147 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;119 |
| 148 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 149 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 150 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;136 |
| 151 | `add_vertex` x3 | Y | Y |  |  | Y |  |  | unknown | 658&#8209;665 |
| 152 | `add_labeled_arc` x3 | Y | Y |  |  | Y |  |  | unknown | 667&#8209;675 |
| 153 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;160 |
| 154 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;166 |
| 155 | `n_plus` x3 | Y | Y |  |  | Y |  |  | unknown | 677&#8209;683 |
| 156 | `n_minus` x3 | Y | Y |  |  | Y |  |  | unknown | 685&#8209;691 |
| 157 | `n_plus_par` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;233 |
| 158 | `n_minus_par` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;246 |
| 159 | `new` | Y | Y |  |  | Y |  |  | unknown | 646&#8209;656 |

### Chap06/LabDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 160 | `empty` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;75 |
| 161 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;87 |
| 162 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 163 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 164 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 165 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 166 | `add_labeled_arc` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;117 |
| 167 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;125 |
| 168 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 169 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 170 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |

### Chap06/LabUnDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 171 | `empty` x3 | Y | Y |  |  | Y |  |  | unknown | 603&#8209;608 |
| 172 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;127 |
| 173 | `vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 610&#8209;611 |
| 174 | `labeled_edges` x3 | Y | Y |  |  | Y |  |  | unknown | 613&#8209;614 |
| 175 | `edges` x3 | Y | Y |  |  | Y |  |  | unknown | 616&#8209;619 |
| 176 | `add_vertex` x3 | Y | Y |  |  | Y |  |  | unknown | 633&#8209;637 |
| 177 | `add_labeled_edge` x3 | Y | Y |  |  | Y |  |  | unknown | 639&#8209;640 |
| 178 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;165 |
| 179 | `has_edge` x3 | Y | Y |  |  | Y |  |  | unknown | 621&#8209;625 |
| 180 | `ng` x3 | Y | Y |  |  | Y |  |  | unknown | 627&#8209;631 |
| 181 | `ng_par` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;206 |

### Chap06/LabUnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 182 | `empty` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;71 |
| 183 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;83 |
| 184 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 185 | `labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 186 | `edges` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;100 |
| 187 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 188 | `add_labeled_edge` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;115 |
| 189 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;125 |
| 190 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;132 |
| 191 | `ng` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |

### Chap06/UnDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 192 | `empty` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;118 |
| 193 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;132 |
| 194 | `vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 499&#8209;501 |
| 195 | `edges` x3 | Y | Y |  |  | Y |  |  | unknown | 503&#8209;505 |
| 196 | `sizeV` x3 | Y | Y |  |  | Y |  |  | unknown | 507&#8209;509 |
| 197 | `sizeE` x3 | Y | Y |  |  | Y |  |  | unknown | 511&#8209;513 |
| 198 | `neighbor` x3 | Y | Y |  |  | Y |  |  | unknown | 515&#8209;520 |
| 199 | `ng` x3 | Y | Y |  |  | Y |  |  | unknown | 522&#8209;528 |
| 200 | `ng_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 530&#8209;536 |
| 201 | `incident` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;206 |
| 202 | `degree` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;215 |
| 203 | `ng_par` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;228 |
| 204 | `ng_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;241 |
| 205 | `new` | Y | Y |  |  | Y |  |  | unknown | 487&#8209;497 |
| 206 | `eq` |  | Y |  |  | Y |  |  | unknown | 630&#8209;631 |

### Chap06/UnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 207 | `empty` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 208 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;110 |
| 209 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 210 | `edges` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 211 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 212 | `sizeE` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;132 |
| 213 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;141 |
| 214 | `ng` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;151 |
| 215 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;161 |
| 216 | `incident` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;166 |
| 217 | `degree` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;174 |
| 218 | `eq` |  | Y |  |  | Y |  |  | unknown | 358&#8209;359 |

### Chap06/WeightedDirGraphStEphI128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 219 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 220 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 221 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 222 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 223 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 224 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 225 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 226 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 227 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphI16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 228 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 229 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 230 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 231 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 232 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 233 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 234 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 235 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 236 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphI32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 237 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 238 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 239 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 240 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 241 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 242 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 243 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 244 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 245 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 246 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 247 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 248 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 249 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 250 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 251 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 252 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 253 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 254 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphI8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 255 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 256 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 257 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 258 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 259 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 260 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 261 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 262 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 263 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphIsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 264 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 265 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 266 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 267 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 268 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 269 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 270 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 271 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 272 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphU128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 273 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 274 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 275 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 276 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 277 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 278 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 279 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 280 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 281 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphU16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 282 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 283 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 284 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 285 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 286 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 287 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 288 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 289 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 290 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphU32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 291 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 292 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 293 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 294 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 295 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 296 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 297 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 298 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 299 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphU64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 300 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 301 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 302 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 303 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 304 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 305 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 306 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 307 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 308 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphU8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 309 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 310 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 311 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 312 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 313 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 314 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 315 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 316 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 317 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphUsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 318 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 319 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 320 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 321 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 322 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 323 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 324 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 325 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 326 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap11/FibonacciMtEph2Threads.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 327 | `fib_2threads` |  |  |  | Y | Y |  |  | unknown | 101&#8209;103 |

### Chap11/FibonacciMtEphRecomputes.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 328 | `fib_recomputes` |  |  |  | Y | Y |  |  | unknown | 93&#8209;96 |

### Chap11/FibonacciMtPerAllThreads.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 329 | `fib` |  |  |  | Y | Y |  |  | unknown | 21&#8209;26 |

### Chap11/FibonacciMtPerTSM.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 330 | `fib` |  |  |  | Y | Y |  |  | unknown | 87&#8209;90 |

### Chap11/FibonacciStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 331 | `lemma_fib_bound` |  |  |  | Y | Y |  |  | unknown | 48&#8209;50 |
| 332 | `lemma_fib_fits_u64` |  |  |  | Y | Y |  |  | unknown | 66&#8209;68 |
| 333 | `lemma_fib_sum_fits_u64` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 334 | `fib` |  |  |  | Y | Y |  |  | unknown | 89&#8209;93 |
| 335 | `fib_recursive` |  |  |  | Y | Y |  |  | unknown | 127&#8209;132 |

### Chap12/Exercise12_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 336 | `new` | Y | Y |  |  | Y |  |  | hole | 33&#8209;34 |
| 337 | `lock` | Y | Y |  |  | Y |  |  | hole | 39&#8209;40 |
| 338 | `unlock` | Y | Y |  |  | Y |  |  | hole | 45&#8209;47 |
| 339 | `with_lock` | Y | Y |  |  | Y |  |  | hole | 54 |
| 340 | `parallel_increment` |  |  |  | Y | Y |  |  | hole | 94&#8209;95 |
| 341 | `default` |  | Y |  |  | Y |  | Y |  | 121 |

### Chap12/Exercise12_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 342 | `fetch_add_cas` | Y | Y |  |  | Y |  | Y |  | 22 |

### Chap12/Exercise12_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 343 | `new` | Y | Y |  |  | Y |  |  | hole | 58&#8209;59 |
| 344 | `push` | Y | Y |  |  | Y |  |  | hole | 64&#8209;65 |
| 345 | `pop` | Y | Y |  |  | Y |  |  | hole | 72&#8209;73 |
| 346 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 347 | `drain` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 348 | `default` |  | Y |  |  | Y |  | Y |  | 148 |
| 349 | `drop` |  | Y |  |  | Y |  |  | hole | 155&#8209;157 |

### Chap17/MathSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 350 | `new` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;142 |
| 351 | `set` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;152 |
| 352 | `length` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;157 |
| 353 | `nth` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 354 | `empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 355 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;175 |
| 356 | `add_last` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;183 |
| 357 | `delete_last` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;193 |
| 358 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;198 |
| 359 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;203 |
| 360 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;208 |
| 361 | `with_len` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;215 |
| 362 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;226 |
| 363 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;235 |
| 364 | `domain` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;242 |
| 365 | `range` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;250 |
| 366 | `multiset_range` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;259 |
| 367 | `iter` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;268 |
| 368 | `next` |  | Y |  |  | Y |  |  | unknown | 615&#8209;631 |
| 369 | `eq` |  | Y |  |  | Y |  |  | unknown | 722&#8209;723 |
| 370 | `iter_mut` |  |  | Y |  |  | Y | Y |  | 736&#8209;741 |

### Chap18/ArraySeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 371 | `lemma_deep_view_len` |  |  |  | Y | Y |  |  | unknown | 162&#8209;164 |
| 372 | `lemma_deep_view_key` |  |  |  | Y | Y |  |  | unknown | 169&#8209;174 |
| 373 | `lemma_find_key_index_bounds` |  |  |  | Y | Y |  |  | unknown | 179&#8209;184 |
| 374 | `lemma_find_key_index_found` |  |  |  | Y | Y |  |  | unknown | 194&#8209;205 |
| 375 | `lemma_find_key_index_not_found` |  |  |  | Y | Y |  |  | unknown | 214&#8209;222 |
| 376 | `lemma_spec_collect_step_some` |  |  |  | Y | Y |  |  | unknown | 231&#8209;243 |
| 377 | `lemma_spec_collect_step_none` |  |  |  | Y | Y |  |  | unknown | 254&#8209;265 |
| 378 | `lemma_find_key_some` |  |  |  | Y | Y |  |  | unknown | 275&#8209;282 |
| 379 | `lemma_find_key_none` |  |  |  | Y | Y |  |  | unknown | 293&#8209;298 |
| 380 | `new` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;327 |
| 381 | `set` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;337 |
| 382 | `length` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;343 |
| 383 | `nth` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;350 |
| 384 | `empty` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;356 |
| 385 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 361&#8209;364 |
| 386 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;377 |
| 387 | `append` | Y | Y |  |  | Y |  |  | unknown | 382&#8209;390 |
| 388 | `filter` | Y | Y |  |  | Y |  |  | unknown | 397&#8209;412 |
| 389 | `update` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;425 |
| 390 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;431 |
| 391 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 436&#8209;437 |
| 392 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 443&#8209;448 |
| 393 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 454&#8209;462 |
| 394 | `scan` | Y | Y |  |  | Y |  |  | unknown | 468&#8209;482 |
| 395 | `inject` | Y | Y |  |  | Y |  |  | unknown | 488&#8209;497 |
| 396 | `scan_inclusive` | Y | Y |  |  | Y |  |  | unknown | 503&#8209;513 |
| 397 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 518&#8209;526 |
| 398 | `remove` | Y | Y |  |  | Y |  |  | unknown | 531&#8209;538 |
| 399 | `insert` | Y | Y |  |  | Y |  |  | unknown | 543&#8209;550 |
| 400 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 555&#8209;558 |
| 401 | `find_key` | Y | Y |  |  | Y |  |  | unknown | 561&#8209;573 |
| 402 | `collect` | Y | Y |  |  | Y |  |  | unknown | 579&#8209;591 |
| 403 | `map` |  |  |  | Y | Y |  |  | unknown | 1254&#8209;1258 |
| 404 | `tabulate` |  |  |  | Y | Y |  |  | unknown | 1285&#8209;1291 |
| 405 | `flatten` |  |  |  | Y | Y |  |  | unknown | 1312&#8209;1316 |
| 406 | `iterate_prefixes` |  |  |  | Y | Y |  |  | unknown | 1370&#8209;1385 |
| 407 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1447&#8209;1449 |
| 408 | `iter` |  |  | Y |  | Y |  |  | unknown | 1453&#8209;1457 |
| 409 | `iter_mut` |  |  | Y |  | Y |  | Y |  | 1465 |
| 410 | `next` |  | Y |  |  | Y |  |  | unknown | 1514&#8209;1530 |
| 411 | `eq` |  | Y |  |  | Y |  |  | unknown | 1618&#8209;1619 |

### Chap18/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 412 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 123&#8209;132 |
| 413 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 163&#8209;165 |
| 414 | `apply_ninject_updates` |  |  |  | Y | Y |  |  | unknown | 197&#8209;205 |
| 415 | `new` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;279 |
| 416 | `set` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;289 |
| 417 | `length` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;295 |
| 418 | `nth` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;302 |
| 419 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;316 |
| 420 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;330 |
| 421 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;339 |
| 422 | `empty` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;349 |
| 423 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;358 |
| 424 | `append` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;372 |
| 425 | `filter` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;394 |
| 426 | `update` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;408 |
| 427 | `inject` | Y | Y |  |  | Y |  |  | unknown | 414&#8209;424 |
| 428 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;439 |
| 429 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 444&#8209;445 |
| 430 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 450&#8209;451 |
| 431 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 456&#8209;461 |
| 432 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 466&#8209;474 |
| 433 | `scan` | Y | Y |  |  | Y |  |  | unknown | 479&#8209;491 |
| 434 | `map` | Y | Y |  |  | Y |  |  | unknown | 496&#8209;501 |
| 435 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 506&#8209;512 |
| 436 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 517&#8209;522 |
| 437 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1087&#8209;1089 |
| 438 | `iter` |  |  | Y |  | Y |  |  | unknown | 1093&#8209;1097 |
| 439 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1104&#8209;1115 |
| 440 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1156&#8209;1166 |
| 441 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1210&#8209;1213 |
| 442 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1238&#8209;1253 |
| 443 | `ninject_par` |  |  | Y |  | Y |  |  | unknown | 1326&#8209;1336 |
| 444 | `next` |  | Y |  |  | Y |  |  | unknown | 1463&#8209;1479 |
| 445 | `eq` |  | Y |  |  | Y |  |  | unknown | 1582&#8209;1583 |

### Chap18/ArraySeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 446 | `new` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;114 |
| 447 | `length` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 448 | `nth` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;127 |
| 449 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;141 |
| 450 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;155 |
| 451 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;164 |
| 452 | `empty` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;174 |
| 453 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;183 |
| 454 | `append` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;197 |
| 455 | `filter` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;219 |
| 456 | `update` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;233 |
| 457 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;239 |
| 458 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;245 |
| 459 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;255 |
| 460 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;268 |
| 461 | `scan` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;285 |
| 462 | `map` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;295 |
| 463 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;306 |
| 464 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;316 |
| 465 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 776&#8209;778 |
| 466 | `iter` |  |  | Y |  | Y |  |  | unknown | 782&#8209;786 |
| 467 | `map_par` |  |  | Y |  | Y |  |  | unknown | 794&#8209;804 |
| 468 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 852&#8209;861 |
| 469 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 909&#8209;912 |
| 470 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 949&#8209;964 |
| 471 | `next` |  | Y |  |  | Y |  |  | unknown | 1068&#8209;1084 |
| 472 | `eq` |  | Y |  |  | Y |  |  | unknown | 1187&#8209;1188 |

### Chap18/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 473 | `new` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;123 |
| 474 | `set` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;133 |
| 475 | `length` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;139 |
| 476 | `nth` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;146 |
| 477 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;160 |
| 478 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;174 |
| 479 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;183 |
| 480 | `empty` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;193 |
| 481 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;202 |
| 482 | `append` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;215 |
| 483 | `filter` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;236 |
| 484 | `update` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;249 |
| 485 | `inject` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;264 |
| 486 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;270 |
| 487 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;276 |
| 488 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;286 |
| 489 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;299 |
| 490 | `scan` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;316 |
| 491 | `map` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;326 |
| 492 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;337 |
| 493 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;347 |
| 494 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 888&#8209;890 |
| 495 | `iter` |  |  | Y |  | Y |  |  | unknown | 894&#8209;898 |
| 496 | `next` |  | Y |  |  | Y |  |  | unknown | 945&#8209;961 |
| 497 | `eq` |  | Y |  |  | Y |  |  | unknown | 1051&#8209;1052 |

### Chap18/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 498 | `new` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;123 |
| 499 | `length` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 500 | `nth` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 501 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;150 |
| 502 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;164 |
| 503 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;173 |
| 504 | `empty` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;183 |
| 505 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;192 |
| 506 | `append` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;206 |
| 507 | `filter` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;228 |
| 508 | `update` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;242 |
| 509 | `inject` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;258 |
| 510 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;264 |
| 511 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;270 |
| 512 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;280 |
| 513 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;293 |
| 514 | `scan` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;310 |
| 515 | `map` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;320 |
| 516 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 325&#8209;331 |
| 517 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;341 |
| 518 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 873&#8209;875 |
| 519 | `iter` |  |  | Y |  | Y |  |  | unknown | 879&#8209;883 |
| 520 | `next` |  | Y |  |  | Y |  |  | unknown | 917&#8209;933 |
| 521 | `eq` |  | Y |  |  | Y |  |  | unknown | 1017&#8209;1018 |

### Chap18/LinkedListStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 522 | `new` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;110 |
| 523 | `set` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;119 |
| 524 | `length` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 525 | `nth` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 526 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;143 |
| 527 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;151 |
| 528 | `empty` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 529 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;168 |
| 530 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;178 |
| 531 | `map` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;187 |
| 532 | `append` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;200 |
| 533 | `filter` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;221 |
| 534 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;230 |
| 535 | `update` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;243 |
| 536 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;248 |
| 537 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;253 |
| 538 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;262 |
| 539 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;274 |
| 540 | `scan` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;290 |
| 541 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 732&#8209;734 |
| 542 | `iter` |  |  | Y |  | Y |  |  | unknown | 740&#8209;744 |
| 543 | `next` |  | Y |  |  | Y |  |  | unknown | 780&#8209;796 |
| 544 | `eq` |  | Y |  |  | Y |  |  | unknown | 888&#8209;889 |

### Chap18/LinkedListStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 545 | `new` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;111 |
| 546 | `length` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 547 | `nth` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 548 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;135 |
| 549 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;143 |
| 550 | `empty` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 551 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;160 |
| 552 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;170 |
| 553 | `map` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;179 |
| 554 | `append` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;192 |
| 555 | `filter` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;213 |
| 556 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;222 |
| 557 | `update` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;235 |
| 558 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;240 |
| 559 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;245 |
| 560 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;254 |
| 561 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;266 |
| 562 | `scan` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;282 |
| 563 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 715&#8209;717 |
| 564 | `iter` |  |  | Y |  | Y |  |  | unknown | 723&#8209;727 |
| 565 | `next` |  | Y |  |  | Y |  |  | unknown | 763&#8209;779 |
| 566 | `eq` |  | Y |  |  | Y |  |  | unknown | 871&#8209;872 |

### Chap19/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 567 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 123&#8209;132 |
| 568 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 163&#8209;165 |
| 569 | `new` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;197 |
| 570 | `set` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;208 |
| 571 | `length` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;214 |
| 572 | `nth` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;221 |
| 573 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;235 |
| 574 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;249 |
| 575 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;258 |
| 576 | `empty` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;264 |
| 577 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;275 |
| 578 | `append` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;289 |
| 579 | `filter` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;307 |
| 580 | `update` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;321 |
| 581 | `inject` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;337 |
| 582 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;352 |
| 583 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 357&#8209;358 |
| 584 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;364 |
| 585 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;374 |
| 586 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;386 |
| 587 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 391&#8209;398 |
| 588 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 403&#8209;411 |
| 589 | `scan` | Y | Y |  |  | Y |  |  | unknown | 416&#8209;426 |
| 590 | `map` | Y | Y |  |  | Y |  |  | unknown | 431&#8209;436 |
| 591 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 441&#8209;447 |
| 592 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 452&#8209;457 |
| 593 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 462&#8209;471 |
| 594 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 1060&#8209;1063 |
| 595 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 1075&#8209;1080 |
| 596 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1093&#8209;1095 |
| 597 | `iter` |  |  | Y |  | Y |  |  | unknown | 1099&#8209;1103 |
| 598 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1111&#8209;1122 |
| 599 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1164&#8209;1174 |
| 600 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1218&#8209;1221 |
| 601 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1247&#8209;1262 |
| 602 | `next` |  | Y |  |  | Y |  |  | unknown | 1358&#8209;1374 |
| 603 | `eq` |  | Y |  |  | Y |  |  | unknown | 1477&#8209;1478 |

### Chap19/ArraySeqMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 604 | `length` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 605 | `nth_cloned` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;99 |
| 606 | `slice` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;111 |
| 607 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;123 |
| 608 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;130 |
| 609 | `empty` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;135 |
| 610 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;142 |
| 611 | `new` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;152 |
| 612 | `iter` |  |  | Y |  | Y |  |  | unknown | 249&#8209;255 |

### Chap19/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 613 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 125&#8209;128 |
| 614 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 140&#8209;145 |
| 615 | `new` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;184 |
| 616 | `set` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;195 |
| 617 | `length` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;201 |
| 618 | `nth` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;208 |
| 619 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;222 |
| 620 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;236 |
| 621 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;245 |
| 622 | `empty` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;251 |
| 623 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;262 |
| 624 | `append` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;276 |
| 625 | `filter` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;294 |
| 626 | `update` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;308 |
| 627 | `inject` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;324 |
| 628 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;330 |
| 629 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;336 |
| 630 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 341&#8209;346 |
| 631 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;358 |
| 632 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;370 |
| 633 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 375&#8209;383 |
| 634 | `scan` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;398 |
| 635 | `map` | Y | Y |  |  | Y |  |  | unknown | 403&#8209;408 |
| 636 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 413&#8209;419 |
| 637 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 424&#8209;429 |
| 638 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 434&#8209;443 |
| 639 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1019&#8209;1021 |
| 640 | `iter` |  |  | Y |  | Y |  |  | unknown | 1025&#8209;1029 |
| 641 | `lemma_view_index` |  |  | Y |  | Y |  |  | unknown | 1037&#8209;1039 |
| 642 | `next` |  | Y |  |  | Y |  |  | unknown | 1073&#8209;1089 |
| 643 | `eq` |  | Y |  |  | Y |  |  | unknown | 1197&#8209;1198 |

### Chap19/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 644 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 124&#8209;127 |
| 645 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 139&#8209;144 |
| 646 | `new` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;181 |
| 647 | `length` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 648 | `nth` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 649 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;208 |
| 650 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;222 |
| 651 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;231 |
| 652 | `empty` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;237 |
| 653 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;248 |
| 654 | `append` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;262 |
| 655 | `filter` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;280 |
| 656 | `update` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;294 |
| 657 | `inject` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;310 |
| 658 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;316 |
| 659 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;322 |
| 660 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;332 |
| 661 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;344 |
| 662 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;356 |
| 663 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 361&#8209;369 |
| 664 | `scan` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;384 |
| 665 | `map` | Y | Y |  |  | Y |  |  | unknown | 389&#8209;394 |
| 666 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;405 |
| 667 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 410&#8209;415 |
| 668 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 420&#8209;429 |
| 669 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1027&#8209;1029 |
| 670 | `iter` |  |  | Y |  | Y |  |  | unknown | 1033&#8209;1037 |
| 671 | `next` |  | Y |  |  | Y |  |  | unknown | 1073&#8209;1089 |
| 672 | `eq` |  | Y |  |  | Y |  |  | unknown | 1197&#8209;1198 |

### Chap21/Algorithm21_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 673 | `lemma_sum_inner_lens_mono` |  |  |  | Y | Y |  |  | unknown | 52&#8209;55 |
| 674 | `lemma_sum_inner_lens_uniform` |  |  |  | Y | Y |  |  | unknown | 65&#8209;71 |
| 675 | `flatten_inner` |  |  |  | Y | Y |  |  | unknown | 92&#8209;96 |
| 676 | `points2d_tab_flat` |  |  |  | Y | Y |  |  | unknown | 152&#8209;158 |

### Chap21/Algorithm21_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 677 | `points3d_tab_flat` |  |  |  | Y | Y |  |  | unknown | 46&#8209;53 |

### Chap21/Algorithm21_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 678 | `primes_bf` |  |  |  | Y | Y |  |  | unknown | 46&#8209;55 |

### Chap21/Algorithm21_6.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 679 | `lemma_product_not_prime` |  |  |  | Y | Y |  |  | unknown | 37&#8209;39 |
| 680 | `prime_sieve` |  |  |  | Y | Y |  |  | unknown | 62&#8209;68 |

### Chap21/Exercise21_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 681 | `lemma_inner_lens_sum_triangular` |  |  |  | Y | Y |  |  | unknown | 44&#8209;51 |
| 682 | `all_contiguous_subseqs` |  |  |  | Y | Y |  |  | unknown | 72&#8209;76 |

### Chap21/Exercise21_7.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 683 | `is_even` |  |  |  | Y | Y |  |  | unknown | 47&#8209;49 |
| 684 | `is_vowel` |  |  |  | Y | Y |  |  | unknown | 61&#8209;63 |
| 685 | `pair_even_with_vowels` |  |  |  | Y | Y |  |  | unknown | 77&#8209;86 |

### Chap21/Exercise21_8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 686 | `lemma_zero_count_means_no_divisors` |  |  |  | Y | Y |  |  | unknown | 55&#8209;62 |
| 687 | `lemma_no_divisors_means_zero_count` |  |  |  | Y | Y |  |  | unknown | 81&#8209;88 |
| 688 | `lemma_divisor_count_nonneg` |  |  |  | Y | Y |  |  | unknown | 99&#8209;101 |
| 689 | `lemma_filter_len_eq_divisor_count` |  |  |  | Y | Y |  |  | unknown | 109&#8209;115 |
| 690 | `lemma_divisor_count_split_last` |  |  |  | Y | Y |  |  | unknown | 135&#8209;140 |
| 691 | `is_divisible` |  |  |  | Y | Y |  |  | unknown | 164&#8209;166 |
| 692 | `is_prime` |  |  |  | Y | Y |  |  | unknown | 176&#8209;178 |

### Chap21/Exercise21_9.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 693 | `lemma_div_exact` |  |  |  | Y | Y |  |  | unknown | 28&#8209;30 |
| 694 | `lemma_composite_has_small_divisor` |  |  |  | Y | Y |  |  | unknown | 38&#8209;43 |
| 695 | `lemma_composites_covered_by_small_multiples` |  |  |  | Y | Y |  |  | unknown | 75&#8209;82 |

### Chap21/Problem21_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 696 | `points2d` |  |  |  | Y | Y |  |  | unknown | 34&#8209;43 |

### Chap21/Problem21_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 697 | `points3d_loops` |  |  |  | Y | Y |  |  | unknown | 37&#8209;48 |

### Chap21/Problem21_4.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 698 | `cartesian_loops` |  |  |  | Y | Y |  |  | unknown | 40&#8209;47 |
| 699 | `cartesian_tab_flat` |  |  |  | Y | Y |  |  | unknown | 92&#8209;103 |

### Chap23/BalBinTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 700 | `lemma_in_order_pre_order_permutation` |  |  |  | Y | Y |  |  | unknown | 96&#8209;98 |
| 701 | `lemma_pre_order_post_order_permutation` |  |  |  | Y | Y |  |  | unknown | 141&#8209;143 |
| 702 | `leaf` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;200 |
| 703 | `node` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;211 |
| 704 | `is_leaf` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;217 |
| 705 | `size` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;224 |
| 706 | `height` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;231 |
| 707 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;241 |
| 708 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;251 |
| 709 | `post_order` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;261 |
| 710 | `iter_in_order` |  |  | Y |  | Y |  |  | unknown | 445&#8209;451 |
| 711 | `iter_pre_order` |  |  | Y |  | Y |  |  | unknown | 459&#8209;465 |
| 712 | `iter_post_order` |  |  | Y |  | Y |  |  | unknown | 473&#8209;479 |
| 713 | `next` x3 |  | Y |  |  | Y |  |  | unknown | 563&#8209;579 |
| 714 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 778&#8209;780 |
| 715 | `clone_tree` |  |  |  | Y | Y |  |  | unknown | 807&#8209;810 |

### Chap23/PrimTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 716 | `empty` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 717 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;145 |
| 718 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;154 |
| 719 | `length` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 720 | `nth` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;169 |
| 721 | `expose` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;182 |
| 722 | `join` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;202 |
| 723 | `append` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;218 |
| 724 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;233 |
| 725 | `update` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;248 |
| 726 | `map` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;258 |
| 727 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;270 |
| 728 | `filter` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;287 |
| 729 | `drop` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;302 |
| 730 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;312 |
| 731 | `as_slice` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;319 |
| 732 | `into_vec` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;326 |
| 733 | `iter` |  |  | Y |  | Y |  |  | unknown | 336&#8209;340 |
| 734 | `next` |  | Y |  |  | Y |  |  | unknown | 783&#8209;799 |
| 735 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 885&#8209;886 |

### Chap26/DivConReduceMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 736 | `lemma_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 84&#8209;88 |
| 737 | `lemma_max_fold_left_bound` |  |  |  | Y | Y |  |  | unknown | 97&#8209;102 |
| 738 | `lemma_max_fold_left_achievable` |  |  |  | Y | Y |  |  | unknown | 122&#8209;127 |
| 739 | `max_element_parallel` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;163 |
| 740 | `sum_parallel` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;181 |
| 741 | `product_parallel` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;193 |
| 742 | `any_parallel` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;205 |
| 743 | `all_parallel` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;217 |

### Chap26/DivConReduceStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 744 | `max_element` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;67 |
| 745 | `sum` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;85 |
| 746 | `product` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;97 |
| 747 | `any` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;109 |
| 748 | `all` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;121 |

### Chap26/ETSPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 749 | `lemma_point_in_seq_transitive` |  |  |  | Y | Y |  |  | unknown | 128&#8209;134 |
| 750 | `lemma_edge_valid_transitive` |  |  |  | Y | Y |  |  | unknown | 144&#8209;156 |
| 751 | `lemma_mod_successor` |  |  |  | Y | Y |  |  | unknown | 163&#8209;165 |
| 752 | `lemma_combined_cycle` |  |  |  | Y | Y |  |  | unknown | 173&#8209;197 |
| 753 | `etsp_parallel` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;311 |
| 754 | `etsp_parallel_inner` |  |  |  | Y | Y |  |  | unknown | 323&#8209;328 |
| 755 | `sort_and_split` |  |  |  | Y | Y |  |  | unknown | 507&#8209;518 |
| 756 | `find_best_swap` |  |  |  | Y | Y |  |  | unknown | 568&#8209;574 |
| 757 | `distance` | Y | Y |  |  |  | Y | Y |  | 599 |
| 758 | `sort_and_split_impl` |  |  |  | Y |  | Y | Y |  | 610&#8209;632 |
| 759 | `find_best_swap_impl` |  |  |  | Y |  | Y | Y |  | 634&#8209;642 |
| 760 | `find_best_swap_par` |  |  |  | Y |  | Y | Y |  | 644&#8209;678 |

### Chap26/ETSPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 761 | `lemma_point_in_seq_transitive` |  |  |  | Y | Y |  |  | unknown | 121&#8209;127 |
| 762 | `lemma_edge_valid_transitive` |  |  |  | Y | Y |  |  | unknown | 137&#8209;149 |
| 763 | `lemma_combined_cycle` |  |  |  | Y | Y |  |  | unknown | 156&#8209;180 |
| 764 | `etsp` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;299 |
| 765 | `etsp_inner` |  |  |  | Y | Y |  |  | unknown | 312&#8209;317 |
| 766 | `sort_and_split` |  |  |  | Y | Y |  |  | unknown | 482&#8209;493 |
| 767 | `find_best_swap` |  |  |  | Y | Y |  |  | unknown | 543&#8209;549 |
| 768 | `distance` | Y | Y |  |  |  | Y | Y |  | 572 |
| 769 | `sort_and_split_impl` |  |  |  | Y |  | Y | Y |  | 583&#8209;605 |
| 770 | `find_best_swap_impl` |  |  |  | Y |  | Y | Y |  | 607&#8209;626 |

### Chap26/MergeSortMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 771 | `lemma_multiset_count_positive_implies_exists` |  |  |  | Y | Y |  |  | unknown | 89&#8209;92 |
| 772 | `lemma_all_le_preserved_by_permutation` |  |  |  | Y | Y |  |  | unknown | 108&#8209;113 |
| 773 | `lemma_all_ge_preserved_by_permutation` |  |  |  | Y | Y |  |  | unknown | 125&#8209;130 |
| 774 | `lemma_sorted_concat_pivot` |  |  |  | Y | Y |  |  | unknown | 143&#8209;150 |
| 775 | `merge_parallel` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;188 |
| 776 | `merge_sort_parallel` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;198 |
| 777 | `binary_search_upper_bound` |  |  |  | Y | Y |  |  | unknown | 207&#8209;214 |
| 778 | `merge_dc` |  |  |  | Y | Y |  |  | unknown | 264&#8209;274 |

### Chap26/MergeSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 779 | `lemma_push_sorted` |  |  |  | Y | Y |  |  | unknown | 85&#8209;90 |
| 780 | `merge` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;125 |
| 781 | `merge_sort` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;135 |

### Chap26/ScanDCMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 782 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 75&#8209;78 |
| 783 | `prefix_sums_dc_parallel` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;102 |
| 784 | `prefix_sums_dc_inner` |  |  |  | Y | Y |  |  | unknown | 108&#8209;116 |

### Chap26/ScanDCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 785 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 80&#8209;83 |
| 786 | `scan_dc` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;114 |
| 787 | `prefix_sums_dc` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;127 |

### Chap27/ReduceContractMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 788 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 49&#8209;52 |
| 789 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 62&#8209;64 |
| 790 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 73&#8209;75 |
| 791 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 83&#8209;93 |
| 792 | `reduce_contract_parallel` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;165 |
| 793 | `contract_parallel` |  |  |  | Y | Y |  |  | unknown | 175&#8209;190 |

### Chap27/ReduceContractStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 794 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 44&#8209;47 |
| 795 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 60&#8209;62 |
| 796 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 797 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 88&#8209;98 |
| 798 | `reduce_contract` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;180 |

### Chap27/ScanContractMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 799 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 48&#8209;51 |
| 800 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 62&#8209;64 |
| 801 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 73&#8209;75 |
| 802 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 83&#8209;93 |
| 803 | `lemma_prefix_contraction` |  |  |  | Y | Y |  |  | unknown | 149&#8209;157 |
| 804 | `lemma_expand_even` |  |  |  | Y | Y |  |  | unknown | 169&#8209;177 |
| 805 | `lemma_expand_odd` |  |  |  | Y | Y |  |  | unknown | 189&#8209;195 |
| 806 | `lemma_expand_odd_tail` |  |  |  | Y | Y |  |  | unknown | 205&#8209;216 |
| 807 | `scan_contract_parallel` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;250 |
| 808 | `expand_scan_parallel` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;273 |

### Chap27/ScanContractStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 809 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 42&#8209;45 |
| 810 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 56&#8209;58 |
| 811 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 67&#8209;69 |
| 812 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 77&#8209;87 |
| 813 | `lemma_prefix_contraction` |  |  |  | Y | Y |  |  | unknown | 143&#8209;151 |
| 814 | `lemma_expand_even` |  |  |  | Y | Y |  |  | unknown | 163&#8209;171 |
| 815 | `lemma_expand_odd` |  |  |  | Y | Y |  |  | unknown | 183&#8209;189 |
| 816 | `lemma_expand_odd_tail` |  |  |  | Y | Y |  |  | unknown | 199&#8209;210 |
| 817 | `scan_contract` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;243 |
| 818 | `expand_scan` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;266 |

### Chap28/MCSSSpec.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 819 | `lemma_range_sum_snoc` |  |  |  | Y | Y |  |  | unknown | 144&#8209;151 |
| 820 | `lemma_range_sum_single` |  |  |  | Y | Y |  |  | unknown | 160&#8209;164 |
| 821 | `lemma_range_sum_empty` |  |  |  | Y | Y |  |  | unknown | 170&#8209;172 |
| 822 | `lemma_range_sum_split` |  |  |  | Y | Y |  |  | unknown | 177&#8209;182 |
| 823 | `lemma_range_sum_via_prefix` |  |  |  | Y | Y |  |  | unknown | 191&#8209;195 |
| 824 | `lemma_min_prefix_sum_is_min` |  |  |  | Y | Y |  |  | unknown | 201&#8209;206 |
| 825 | `lemma_min_prefix_sum_achieved` |  |  |  | Y | Y |  |  | unknown | 215&#8209;220 |
| 826 | `lemma_range_sum_subseq` |  |  |  | Y | Y |  |  | unknown | 237&#8209;245 |
| 827 | `lemma_crossing_decompose` |  |  |  | Y | Y |  |  | unknown | 256&#8209;260 |
| 828 | `lemma_sums_fit_subseq` |  |  |  | Y | Y |  |  | unknown | 266&#8209;273 |

### Chap28/MaxContigSubSumBruteStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 829 | `max_contig_sub_sum_brute` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;59 |
| 830 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 69&#8209;72 |

### Chap28/MaxContigSubSumDivConMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 831 | `max_contig_sub_sum_divcon_mt` | Y | Y |  |  | Y |  |  | unknown | 48&#8209;55 |
| 832 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 61&#8209;63 |
| 833 | `max_suffix_sum` |  |  |  | Y | Y |  |  | unknown | 73&#8209;75 |
| 834 | `max_prefix_sum` |  |  |  | Y | Y |  |  | unknown | 123&#8209;125 |

### Chap28/MaxContigSubSumDivConOptMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 835 | `max_contig_sub_sum_divcon_opt_mt` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;61 |
| 836 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 67&#8209;69 |
| 837 | `max_contig_sub_sum_aux` |  |  |  | Y | Y |  |  | unknown | 79&#8209;87 |

### Chap28/MaxContigSubSumDivConOptStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 838 | `lemma_strength_combine` |  |  |  | Y | Y |  |  | unknown | 69&#8209;118 |
| 839 | `max_contig_sub_sum_divcon_opt` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;269 |
| 840 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 277&#8209;279 |
| 841 | `max_contig_sub_sum_aux` |  |  |  | Y | Y |  |  | unknown | 292&#8209;302 |

### Chap28/MaxContigSubSumDivConStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 842 | `lemma_divcon_combine` |  |  |  | Y | Y |  |  | unknown | 61&#8209;88 |
| 843 | `max_contig_sub_sum_divcon` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;160 |
| 844 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 168&#8209;170 |
| 845 | `max_suffix_sum` |  |  |  | Y | Y |  |  | unknown | 185&#8209;190 |
| 846 | `max_prefix_sum` |  |  |  | Y | Y |  |  | unknown | 261&#8209;266 |

### Chap28/MaxContigSubSumIterStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 847 | `lemma_max_ending_at_is_max` |  |  |  | Y | Y |  |  | unknown | 74&#8209;80 |
| 848 | `lemma_max_ending_at_achieved` |  |  |  | Y | Y |  |  | unknown | 97&#8209;104 |
| 849 | `max_contig_sub_sum_iter` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;139 |
| 850 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 147&#8209;149 |

### Chap28/MaxContigSubSumOptMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 851 | `max_contig_sub_sum_opt_mt` | Y | Y |  |  | Y |  |  | unknown | 22&#8209;29 |

### Chap28/MaxContigSubSumOptStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 852 | `lemma_prefix_opt_is_mcss` |  |  |  | Y | Y |  |  | unknown | 30&#8209;46 |
| 853 | `max_contig_sub_sum_opt` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;86 |

### Chap28/MaxContigSubSumReducedMcsseStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 854 | `max_contig_sub_sum_reduced_mcsse` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;58 |
| 855 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 68&#8209;70 |

### Chap28/MaxContigSubSumReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 856 | `max_contig_sub_sum_reduced` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;55 |
| 857 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 65&#8209;67 |

### Chap30/Probability.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 858 | `new` | Y | Y |  |  | Y |  | Y |  | 32 |
| 859 | `value` | Y | Y |  |  | Y |  | Y |  | 36 |
| 860 | `infinity` | Y | Y |  |  | Y |  |  | hole | 40 |
| 861 | `zero` | Y | Y |  |  | Y |  | Y |  | 44 |
| 862 | `default` |  | Y |  |  | Y |  | Y |  | 58 |
| 863 | `eq` |  | Y |  |  | Y |  |  | hole | 66 |
| 864 | `partial_cmp` |  | Y |  |  | Y |  |  | hole | 75 |
| 865 | `cmp` |  | Y |  |  | Y |  |  | hole | 83 |
| 866 | `hash` |  | Y |  |  | Y |  |  | hole | 106 |
| 867 | `from` x2 |  | Y |  |  | Y |  |  | hole | 113 |
| 868 | `add` |  | Y |  |  | Y |  |  | hole | 127 |
| 869 | `sub` |  | Y |  |  | Y |  |  | hole | 134 |
| 870 | `mul` |  | Y |  |  | Y |  |  | hole | 141 |
| 871 | `div` |  | Y |  |  | Y |  |  | hole | 148 |

### Chap35/OrderStatSelectMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 872 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 68&#8209;69 |
| 873 | `lemma_const_seq_multiset` |  |  |  | Y | Y |  |  | unknown | 93&#8209;98 |
| 874 | `select` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 875 | `parallel_three_way_partition` |  |  |  | Y | Y |  |  | unknown | 124&#8209;146 |
| 876 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 399&#8209;408 |

### Chap35/OrderStatSelectMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 877 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 68&#8209;69 |
| 878 | `lemma_const_seq_multiset` |  |  |  | Y | Y |  |  | unknown | 93&#8209;98 |
| 879 | `select` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 880 | `parallel_three_way_partition` |  |  |  | Y | Y |  |  | unknown | 124&#8209;146 |
| 881 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 399&#8209;408 |

### Chap35/OrderStatSelectStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 882 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 60&#8209;61 |
| 883 | `select` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;95 |
| 884 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 113&#8209;120 |

### Chap35/OrderStatSelectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 885 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 59&#8209;60 |
| 886 | `select` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;94 |
| 887 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 112&#8209;119 |

### Chap36/QuickSortMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 888 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 64&#8209;65 |
| 889 | `lemma_partition_sort_concat` |  |  |  | Y | Y |  |  | unknown | 90&#8209;113 |
| 890 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;190 |
| 891 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;200 |
| 892 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;210 |
| 893 | `median_of_three` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;216 |
| 894 | `median3_pivot_idx` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;224 |
| 895 | `concat_three` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;233 |

### Chap36/QuickSortMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 896 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 71&#8209;72 |
| 897 | `lemma_partition_sort_concat` |  |  |  | Y | Y |  |  | unknown | 97&#8209;120 |
| 898 | `lemma_elements_from_vec` |  |  |  | Y | Y |  |  | unknown | 187&#8209;193 |
| 899 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;218 |
| 900 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;232 |
| 901 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;246 |
| 902 | `median_of_three` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;252 |
| 903 | `median3_pivot_idx` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;264 |
| 904 | `concat_three_vecs` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;273 |

### Chap36/QuickSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 905 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 62&#8209;63 |
| 906 | `lemma_partition_sort_concat` |  |  |  | Y | Y |  |  | unknown | 88&#8209;111 |
| 907 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;188 |
| 908 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;198 |
| 909 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;208 |
| 910 | `median_of_three` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;214 |
| 911 | `median3_pivot_idx` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;222 |
| 912 | `concat_three` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;231 |

### Chap37/AVLTreeSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 913 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 184&#8209;187 |
| 914 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 198&#8209;200 |
| 915 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 211&#8209;214 |
| 916 | `empty` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;233 |
| 917 | `new` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;236 |
| 918 | `length` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;240 |
| 919 | `nth` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;244 |
| 920 | `set` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;251 |
| 921 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;257 |
| 922 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;261 |
| 923 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;265 |
| 924 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;270 |
| 925 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;273 |
| 926 | `update` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;281 |
| 927 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;289 |
| 928 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;296 |
| 929 | `iter` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;303 |
| 930 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;307 |
| 931 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;312 |
| 932 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;316 |
| 933 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;329 |
| 934 | `is_tree_empty` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;333 |
| 935 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;337 |
| 936 | `cached_height` |  |  |  | Y | Y |  |  | unknown | 345&#8209;347 |
| 937 | `cached_size` |  |  |  | Y | Y |  |  | unknown | 355&#8209;357 |
| 938 | `update_size_height` |  |  |  | Y | Y |  |  | unknown | 367&#8209;385 |
| 939 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 401&#8209;408 |
| 940 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 450&#8209;457 |
| 941 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 499&#8209;510 |
| 942 | `insert_at_link` |  |  |  | Y | Y |  |  | unknown | 554&#8209;565 |
| 943 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 630&#8209;633 |
| 944 | `set_link` |  |  |  | Y | Y |  |  | unknown | 648&#8209;658 |
| 945 | `push_inorder` |  |  |  | Y | Y |  |  | unknown | 675&#8209;680 |
| 946 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 710&#8209;715 |
| 947 | `next` |  | Y |  |  | Y |  |  | hole | 1118&#8209;1134 |
| 948 | `default` |  | Y |  |  | Y |  | Y |  | 1189 |
| 949 | `eq` |  | Y |  |  | Y |  |  | hole | 1222&#8209;1223 |

### Chap37/AVLTreeSeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 950 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 172&#8209;175 |
| 951 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 187&#8209;190 |
| 952 | `empty` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;211 |
| 953 | `new` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;214 |
| 954 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;217 |
| 955 | `length` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;221 |
| 956 | `nth` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;225 |
| 957 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;229 |
| 958 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;233 |
| 959 | `set` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;240 |
| 960 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 242&#8209;243 |
| 961 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;248 |
| 962 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;251 |
| 963 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 259&#8209;261 |
| 964 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 269&#8209;271 |
| 965 | `mk` |  |  |  | Y | Y |  |  | unknown | 279&#8209;290 |
| 966 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 299&#8209;304 |
| 967 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 333&#8209;338 |
| 968 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 367&#8209;372 |
| 969 | `nth_ref` |  |  |  | Y | Y |  |  | unknown | 433&#8209;436 |
| 970 | `set_rec` |  |  |  | Y | Y |  |  | unknown | 451&#8209;460 |
| 971 | `inorder_collect` |  |  |  | Y | Y |  |  | unknown | 497&#8209;499 |
| 972 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 509&#8209;512 |
| 973 | `rec` |  |  |  | Y | Y |  | Y |  | 514 |
| 974 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 528&#8209;533 |
| 975 | `default` |  | Y |  |  | Y |  | Y |  | 667 |
| 976 | `next` |  | Y |  |  | Y |  |  | unknown | 674&#8209;675 |
| 977 | `eq` |  | Y |  |  | Y |  |  | hole | 711&#8209;712 |

### Chap37/AVLTreeSeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 978 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 180&#8209;183 |
| 979 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 195&#8209;198 |
| 980 | `empty` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;219 |
| 981 | `new` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;222 |
| 982 | `length` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;226 |
| 983 | `nth` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;230 |
| 984 | `set` | Y | Y |  |  | Y |  |  | hole | 232&#8209;237 |
| 985 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;243 |
| 986 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;247 |
| 987 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;251 |
| 988 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;255 |
| 989 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;258 |
| 990 | `update` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;266 |
| 991 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;275 |
| 992 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;282 |
| 993 | `iter` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;285 |
| 994 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;293 |
| 995 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;298 |
| 996 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;306 |
| 997 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;319 |
| 998 | `h_fn` |  |  |  | Y | Y |  |  | unknown | 327&#8209;329 |
| 999 | `size_link_fn` |  |  |  | Y | Y |  |  | unknown | 337&#8209;339 |
| 1000 | `update_meta` |  |  |  | Y | Y |  |  | unknown | 349&#8209;363 |
| 1001 | `rotate_right_fn` |  |  |  | Y | Y |  |  | unknown | 380&#8209;387 |
| 1002 | `rotate_left_fn` |  |  |  | Y | Y |  |  | unknown | 418&#8209;425 |
| 1003 | `rebalance_fn` |  |  |  | Y | Y |  |  | unknown | 456&#8209;465 |
| 1004 | `insert_at_link` |  |  |  | Y | Y |  |  | unknown | 514&#8209;525 |
| 1005 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 578&#8209;581 |
| 1006 | `set_link` |  |  |  | Y | Y |  |  | unknown | 596&#8209;605 |
| 1007 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 631&#8209;636 |
| 1008 | `clone_link` |  |  |  | Y | Y |  |  | hole | 673&#8209;679 |
| 1009 | `default` |  | Y |  |  | Y |  | Y |  | 1033 |
| 1010 | `push_left_iter` |  |  |  | Y | Y |  |  | unknown | 1038&#8209;1040 |
| 1011 | `next` |  | Y |  |  | Y |  |  | unknown | 1060&#8209;1061 |
| 1012 | `eq` |  | Y |  |  | Y |  |  | hole | 1100&#8209;1101 |

### Chap37/AVLTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1013 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 136&#8209;139 |
| 1014 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 151&#8209;154 |
| 1015 | `empty` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;172 |
| 1016 | `new` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;175 |
| 1017 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;178 |
| 1018 | `length` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;182 |
| 1019 | `nth` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;186 |
| 1020 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;190 |
| 1021 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 1022 | `set` | Y | Y |  |  | Y |  |  | hole | 196&#8209;204 |
| 1023 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 206&#8209;208 |
| 1024 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;213 |
| 1025 | `values_in_order` | Y | Y |  |  | Y |  |  | hole | 215&#8209;217 |
| 1026 | `to_arrayseq` | Y | Y |  |  | Y |  |  | hole | 219&#8209;222 |
| 1027 | `iter` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;225 |
| 1028 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 230&#8209;232 |
| 1029 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 240&#8209;242 |
| 1030 | `mk` |  |  |  | Y | Y |  |  | unknown | 250&#8209;261 |
| 1031 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 270&#8209;275 |
| 1032 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 312&#8209;317 |
| 1033 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 349&#8209;354 |
| 1034 | `nth_ref` |  |  |  | Y | Y |  |  | unknown | 417&#8209;420 |
| 1035 | `set_rec` |  |  |  | Y | Y |  |  | unknown | 435&#8209;444 |
| 1036 | `inorder_collect` |  |  |  | Y | Y |  |  | unknown | 481&#8209;483 |
| 1037 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | unknown | 492&#8209;496 |
| 1038 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 536&#8209;541 |
| 1039 | `default` |  | Y |  |  | Y |  | Y |  | 691 |
| 1040 | `push_left_iter_stper` |  |  |  | Y | Y |  |  | unknown | 704&#8209;706 |
| 1041 | `next` |  | Y |  |  | Y |  |  | unknown | 730&#8209;731 |
| 1042 | `eq` |  | Y |  |  | Y |  |  | hole | 753&#8209;754 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1043 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 40&#8209;72 |
| 1044 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 93&#8209;97 |
| 1045 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 159&#8209;163 |
| 1046 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 226&#8209;235 |
| 1047 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 318&#8209;321 |
| 1048 | `find_node` |  |  |  | Y | Y |  |  | unknown | 343&#8209;348 |
| 1049 | `min_node` |  |  |  | Y | Y |  |  | unknown | 370&#8209;375 |
| 1050 | `max_node` |  |  |  | Y | Y |  |  | unknown | 386&#8209;391 |
| 1051 | `new` | Y | Y |  |  | Y |  |  | unknown | 444&#8209;446 |
| 1052 | `insert` | Y | Y |  |  | Y |  |  | unknown | 448&#8209;456 |
| 1053 | `contains` | Y | Y |  |  | Y |  |  | unknown | 458&#8209;460 |
| 1054 | `size` | Y | Y |  |  | Y |  |  | unknown | 462&#8209;464 |
| 1055 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 466&#8209;468 |
| 1056 | `height` | Y | Y |  |  | Y |  |  | unknown | 470&#8209;472 |
| 1057 | `find` | Y | Y |  |  | Y |  |  | unknown | 474&#8209;475 |
| 1058 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 476&#8209;477 |
| 1059 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 478&#8209;479 |
| 1060 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 480&#8209;481 |
| 1061 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 482&#8209;483 |

### Chap37/BSTAVLStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1062 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 67&#8209;99 |
| 1063 | `lemma_max_plus_one` |  |  |  | Y | Y |  |  | unknown | 117&#8209;119 |
| 1064 | `new` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;133 |
| 1065 | `size` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 1066 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 1067 | `height` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;146 |
| 1068 | `insert` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;157 |
| 1069 | `contains` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;162 |
| 1070 | `find` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;169 |
| 1071 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 174&#8209;212 |
| 1072 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 330&#8209;368 |
| 1073 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 491&#8209;519 |
| 1074 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 777&#8209;788 |
| 1075 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 931&#8209;934 |
| 1076 | `find_node` |  |  |  | Y | Y |  |  | unknown | 964&#8209;969 |
| 1077 | `min_node` |  |  |  | Y | Y |  |  | unknown | 999&#8209;1005 |
| 1078 | `max_node` |  |  |  | Y | Y |  |  | unknown | 1019&#8209;1025 |

### Chap37/BSTBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1079 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 41&#8209;50 |
| 1080 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 133&#8209;136 |
| 1081 | `find_node` |  |  |  | Y | Y |  |  | unknown | 158&#8209;163 |
| 1082 | `min_node` |  |  |  | Y | Y |  |  | unknown | 185&#8209;190 |
| 1083 | `max_node` |  |  |  | Y | Y |  |  | unknown | 201&#8209;206 |
| 1084 | `new` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;261 |
| 1085 | `insert` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;271 |
| 1086 | `contains` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;275 |
| 1087 | `size` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;279 |
| 1088 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;283 |
| 1089 | `height` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;287 |
| 1090 | `find` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;290 |
| 1091 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;292 |
| 1092 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;294 |
| 1093 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;296 |
| 1094 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;298 |

### Chap37/BSTBBAlphaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1095 | `new` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 1096 | `size` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 1097 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 1098 | `height` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 1099 | `insert` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;99 |
| 1100 | `contains` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;104 |
| 1101 | `find` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;111 |
| 1102 | `delete` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;121 |
| 1103 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;129 |
| 1104 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;137 |
| 1105 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 187&#8209;194 |
| 1106 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 322&#8209;325 |
| 1107 | `find_node` |  |  |  | Y | Y |  |  | unknown | 355&#8209;360 |
| 1108 | `min_node` |  |  |  | Y | Y |  |  | unknown | 390&#8209;396 |
| 1109 | `max_node` |  |  |  | Y | Y |  |  | unknown | 410&#8209;416 |
| 1110 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 431&#8209;442 |
| 1111 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 547&#8209;554 |

### Chap37/BSTPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1112 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 40&#8209;49 |
| 1113 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 132&#8209;135 |
| 1114 | `find_node` |  |  |  | Y | Y |  |  | unknown | 157&#8209;162 |
| 1115 | `min_node` |  |  |  | Y | Y |  |  | unknown | 184&#8209;189 |
| 1116 | `max_node` |  |  |  | Y | Y |  |  | unknown | 200&#8209;205 |
| 1117 | `new` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;260 |
| 1118 | `insert` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;270 |
| 1119 | `contains` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;274 |
| 1120 | `size` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;278 |
| 1121 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;282 |
| 1122 | `height` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;286 |
| 1123 | `find` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;289 |
| 1124 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;291 |
| 1125 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;293 |
| 1126 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;295 |
| 1127 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;297 |

### Chap37/BSTPlainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1128 | `lemma_node_contains` |  |  |  | Y | Y |  |  | unknown | 40&#8209;46 |
| 1129 | `lemma_bst_left` |  |  |  | Y | Y |  |  | unknown | 50&#8209;59 |
| 1130 | `lemma_bst_right` |  |  |  | Y | Y |  |  | unknown | 63&#8209;72 |
| 1131 | `new` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;91 |
| 1132 | `size` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 1133 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 1134 | `height` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;104 |
| 1135 | `insert` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;114 |
| 1136 | `contains` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;119 |
| 1137 | `find` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;126 |
| 1138 | `delete` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;136 |
| 1139 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;144 |
| 1140 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;152 |
| 1141 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 243&#8209;250 |
| 1142 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 378&#8209;381 |
| 1143 | `find_node` |  |  |  | Y | Y |  |  | unknown | 411&#8209;416 |
| 1144 | `min_node` |  |  |  | Y | Y |  |  | unknown | 446&#8209;452 |
| 1145 | `max_node` |  |  |  | Y | Y |  |  | unknown | 466&#8209;472 |
| 1146 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 487&#8209;498 |
| 1147 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 605&#8209;612 |

### Chap37/BSTRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1148 | `new_node` |  |  |  | Y | Y |  |  | unknown | 99&#8209;105 |
| 1149 | `is_red` |  |  |  | Y | Y |  |  | unknown | 116&#8209;119 |
| 1150 | `size_link` |  |  |  | Y | Y |  |  | unknown | 127&#8209;130 |
| 1151 | `update` |  |  |  | Y | Y |  |  | unknown | 138&#8209;144 |
| 1152 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 153&#8209;155 |
| 1153 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 179&#8209;181 |
| 1154 | `flip_colors` |  |  |  | Y | Y |  |  | unknown | 205&#8209;207 |
| 1155 | `fix_up` |  |  |  | Y | Y |  |  | unknown | 229&#8209;231 |
| 1156 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 268&#8209;271 |
| 1157 | `find_link` |  |  |  | Y | Y |  |  | unknown | 288&#8209;292 |
| 1158 | `min_link` |  |  |  | Y | Y |  |  | unknown | 308&#8209;313 |
| 1159 | `max_link` |  |  |  | Y | Y |  |  | unknown | 324&#8209;329 |
| 1160 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 340&#8209;343 |
| 1161 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 352&#8209;355 |
| 1162 | `in_order_parallel` |  |  |  | Y | Y |  |  | unknown | 364&#8209;367 |
| 1163 | `pre_order_parallel` |  |  |  | Y | Y |  |  | unknown | 385&#8209;388 |
| 1164 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 406&#8209;409 |
| 1165 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 435&#8209;440 |
| 1166 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 464&#8209;469 |
| 1167 | `height_rec` |  |  |  | Y | Y |  |  | unknown | 490&#8209;494 |
| 1168 | `compute_link_spec_size` |  |  |  | Y | Y |  |  | unknown | 503&#8209;506 |
| 1169 | `new` | Y | Y |  |  | Y |  |  | unknown | 556&#8209;558 |
| 1170 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 560&#8209;561 |
| 1171 | `insert` | Y | Y |  |  | Y |  |  | unknown | 563&#8209;569 |
| 1172 | `contains` | Y | Y |  |  | Y |  |  | unknown | 571&#8209;573 |
| 1173 | `size` | Y | Y |  |  | Y |  |  | unknown | 575&#8209;577 |
| 1174 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 579&#8209;581 |
| 1175 | `height` | Y | Y |  |  | Y |  |  | unknown | 583&#8209;585 |
| 1176 | `find` | Y | Y |  |  | Y |  |  | unknown | 587&#8209;588 |
| 1177 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 589&#8209;590 |
| 1178 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 591&#8209;592 |
| 1179 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 593&#8209;594 |
| 1180 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 595&#8209;596 |
| 1181 | `filter` | Y | Y |  |  | Y |  |  | unknown | 597&#8209;600 |
| 1182 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 601&#8209;604 |
| 1183 | `default` |  | Y |  |  | Y |  | Y |  | 743 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1184 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 51&#8209;83 |
| 1185 | `new` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 1186 | `size` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;115 |
| 1187 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 1188 | `height` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;123 |
| 1189 | `insert` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;133 |
| 1190 | `contains` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 1191 | `find` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;145 |
| 1192 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 151&#8209;157 |
| 1193 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 249&#8209;255 |
| 1194 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 346&#8209;353 |
| 1195 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 475&#8209;478 |
| 1196 | `find_node` |  |  |  | Y | Y |  |  | unknown | 508&#8209;513 |
| 1197 | `min_node` |  |  |  | Y | Y |  |  | unknown | 543&#8209;549 |
| 1198 | `max_node` |  |  |  | Y | Y |  |  | unknown | 563&#8209;569 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1199 | `empty` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 1200 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 1201 | `size` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 1202 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;86 |
| 1203 | `find` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 1204 | `contains` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 1205 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 1206 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 1207 | `insert` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 1208 | `delete` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 1209 | `union` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 1210 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 1211 | `difference` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 1212 | `split` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 1213 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 1214 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 1215 | `filter` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 1216 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 1217 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 1218 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 1219 | `iter` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 1220 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 142&#8209;144 |
| 1221 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 149&#8209;151 |
| 1222 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 160&#8209;164 |
| 1223 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 173&#8209;175 |
| 1224 | `next` |  | Y |  |  | Y |  |  | unknown | 384&#8209;400 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1225 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 1226 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 1227 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 1228 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 1229 | `find` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 1230 | `contains` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 1231 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 1232 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 1233 | `insert` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 1234 | `delete` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 1235 | `union` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 1236 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 1237 | `difference` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 1238 | `split` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 1239 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 1240 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 1241 | `filter` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 1242 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 1243 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 1244 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 1245 | `iter` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 1246 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 124&#8209;126 |
| 1247 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 130&#8209;132 |
| 1248 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 141&#8209;145 |
| 1249 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 154&#8209;156 |
| 1250 | `next` |  | Y |  |  | Y |  |  | unknown | 310&#8209;326 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1251 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 1252 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 1253 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 1254 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 1255 | `find` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 1256 | `contains` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 1257 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 1258 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 1259 | `insert` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 1260 | `delete` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 1261 | `union` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 1262 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 1263 | `difference` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 1264 | `split` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 1265 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 1266 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 1267 | `filter` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 1268 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 1269 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 1270 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 1271 | `iter` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 1272 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 124&#8209;126 |
| 1273 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 130&#8209;132 |
| 1274 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 141&#8209;145 |
| 1275 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 154&#8209;156 |
| 1276 | `next` |  | Y |  |  | Y |  |  | unknown | 366&#8209;382 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1277 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 1278 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 1279 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 1280 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 1281 | `find` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 1282 | `contains` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 1283 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 1284 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 1285 | `insert` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 1286 | `delete` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 1287 | `union` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 1288 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 1289 | `difference` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 1290 | `split` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 1291 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 1292 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 1293 | `filter` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 1294 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 1295 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 1296 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 1297 | `iter` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 1298 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 124&#8209;126 |
| 1299 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 131&#8209;133 |
| 1300 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 139&#8209;141 |
| 1301 | `next` |  | Y |  |  | Y |  |  | unknown | 386&#8209;402 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1302 | `empty` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 1303 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 1304 | `size` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 1305 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;86 |
| 1306 | `find` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 1307 | `contains` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 1308 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 1309 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 1310 | `insert` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 1311 | `delete` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 1312 | `union` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 1313 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 1314 | `difference` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 1315 | `split` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 1316 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 1317 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 1318 | `filter` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 1319 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 1320 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 1321 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 1322 | `iter` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 1323 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 142&#8209;144 |
| 1324 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 149&#8209;151 |
| 1325 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 160&#8209;164 |
| 1326 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 173&#8209;175 |
| 1327 | `next` |  | Y |  |  | Y |  |  | unknown | 384&#8209;400 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1328 | `new_node` |  |  |  | Y | Y |  |  | unknown | 92&#8209;98 |
| 1329 | `size_link` |  |  |  | Y | Y |  |  | unknown | 108&#8209;111 |
| 1330 | `update` |  |  |  | Y | Y |  |  | unknown | 119&#8209;124 |
| 1331 | `splay` |  |  |  | Y | Y |  |  | unknown | 135&#8209;138 |
| 1332 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 252&#8209;255 |
| 1333 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 276&#8209;278 |
| 1334 | `find_link` |  |  |  | Y | Y |  |  | unknown | 290&#8209;294 |
| 1335 | `min_link` |  |  |  | Y | Y |  |  | unknown | 310&#8209;315 |
| 1336 | `max_link` |  |  |  | Y | Y |  |  | unknown | 326&#8209;331 |
| 1337 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 342&#8209;345 |
| 1338 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 354&#8209;357 |
| 1339 | `in_order_parallel` |  |  |  | Y | Y |  |  | unknown | 366&#8209;369 |
| 1340 | `pre_order_parallel` |  |  |  | Y | Y |  |  | unknown | 387&#8209;390 |
| 1341 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 408&#8209;411 |
| 1342 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 436&#8209;441 |
| 1343 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 465&#8209;470 |
| 1344 | `height_rec` |  |  |  | Y | Y |  |  | unknown | 491&#8209;495 |
| 1345 | `compute_link_spec_size` |  |  |  | Y | Y |  |  | unknown | 504&#8209;507 |
| 1346 | `new` | Y | Y |  |  | Y |  |  | unknown | 557&#8209;559 |
| 1347 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 561&#8209;562 |
| 1348 | `insert` | Y | Y |  |  | Y |  |  | unknown | 564&#8209;570 |
| 1349 | `contains` | Y | Y |  |  | Y |  |  | unknown | 572&#8209;574 |
| 1350 | `size` | Y | Y |  |  | Y |  |  | unknown | 576&#8209;578 |
| 1351 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 580&#8209;582 |
| 1352 | `height` | Y | Y |  |  | Y |  |  | unknown | 584&#8209;586 |
| 1353 | `find` | Y | Y |  |  | Y |  |  | unknown | 588&#8209;589 |
| 1354 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 590&#8209;591 |
| 1355 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 592&#8209;593 |
| 1356 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 594&#8209;595 |
| 1357 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 596&#8209;597 |
| 1358 | `filter` | Y | Y |  |  | Y |  |  | unknown | 598&#8209;601 |
| 1359 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 602&#8209;605 |
| 1360 | `default` |  | Y |  |  | Y |  | Y |  | 741 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1361 | `new` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;146 |
| 1362 | `size` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;149 |
| 1363 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 1364 | `height` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;157 |
| 1365 | `insert` | Y | Y |  |  | Y |  |  | hole | 158&#8209;163 |
| 1366 | `find` | Y | Y |  |  | Y |  |  | hole | 164&#8209;168 |
| 1367 | `contains` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 1368 | `minimum` | Y | Y |  |  | Y |  |  | hole | 172&#8209;177 |
| 1369 | `maximum` | Y | Y |  |  | Y |  |  | hole | 178&#8209;183 |
| 1370 | `in_order` | Y | Y |  |  | Y |  |  | hole | 184&#8209;186 |
| 1371 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 187&#8209;189 |
| 1372 | `new_node` |  |  |  | Y | Y |  |  | unknown | 195&#8209;201 |
| 1373 | `size_link` |  |  |  | Y | Y |  |  | unknown | 211&#8209;213 |
| 1374 | `height_link` |  |  |  | Y | Y |  |  | unknown | 222&#8209;225 |
| 1375 | `update` |  |  |  | Y | Y |  |  | unknown | 239&#8209;244 |
| 1376 | `splay` |  |  |  | Y | Y |  |  | unknown | 256&#8209;259 |
| 1377 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 379&#8209;382 |
| 1378 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 403&#8209;405 |
| 1379 | `find_link` |  |  |  | Y | Y |  |  | unknown | 417&#8209;421 |
| 1380 | `min_link` |  |  |  | Y | Y |  |  | unknown | 435&#8209;440 |
| 1381 | `max_link` |  |  |  | Y | Y |  |  | unknown | 451&#8209;456 |
| 1382 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 467&#8209;470 |
| 1383 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 479&#8209;482 |
| 1384 | `default` |  | Y |  |  | Y |  | Y |  | 540 |

### Chap38/BSTParaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1385 | `new_param_bst_arc` |  |  |  | Y | Y |  |  | unknown | 68&#8209;72 |
| 1386 | `new` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 1387 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;117 |
| 1388 | `expose` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 1389 | `join_mid` | Y | Y |  | Y | Y |  |  | unknown | 122&#8209;123 |
| 1390 | `size` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 1391 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 1392 | `insert` | Y | Y |  |  | Y |  | Y |  | 132 |
| 1393 | `delete` | Y | Y |  |  | Y |  | Y |  | 135 |
| 1394 | `find` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 1395 | `split` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;144 |
| 1396 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 146&#8209;147 |
| 1397 | `union` | Y | Y |  |  | Y |  |  | hole | 149&#8209;150 |
| 1398 | `intersect` | Y | Y |  |  | Y |  |  | hole | 152&#8209;153 |
| 1399 | `difference` | Y | Y |  |  | Y |  |  | hole | 155&#8209;156 |
| 1400 | `filter` | Y | Y |  |  | Y |  |  | hole | 158&#8209;173 |
| 1401 | `reduce` | Y | Y |  |  | Y |  |  | hole | 176 |
| 1402 | `in_order` | Y | Y |  |  | Y |  |  | hole | 178&#8209;179 |
| 1403 | `new_leaf` |  |  |  | Y |  | Y | Y |  | 432&#8209;434 |
| 1404 | `expose_internal` |  |  |  | Y |  | Y | Y |  | 436&#8209;444 |
| 1405 | `split_inner` |  |  |  | Y |  | Y | Y |  | 458&#8209;475 |
| 1406 | `join_m` |  |  |  | Y |  | Y | Y |  | 477&#8209;479 |
| 1407 | `min_key` |  |  |  | Y |  | Y | Y |  | 481&#8209;489 |
| 1408 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 491&#8209;500 |
| 1409 | `union_inner` |  |  |  | Y |  | Y | Y |  | 502&#8209;513 |
| 1410 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 515&#8209;529 |
| 1411 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 531&#8209;547 |
| 1412 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 549&#8209;569 |
| 1413 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 571&#8209;577 |
| 1414 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 579&#8209;600 |
| 1415 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 602&#8209;609 |
| 1416 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 611&#8209;620 |

### Chap38/BSTParaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1417 | `new_param_bst` |  |  |  | Y | Y |  |  | unknown | 87&#8209;96 |
| 1418 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 146&#8209;151 |
| 1419 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 158&#8209;164 |
| 1420 | `lemma_cmp_eq_subst` |  |  |  | Y | Y |  |  | unknown | 171&#8209;178 |
| 1421 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 189&#8209;195 |
| 1422 | `lemma_cmp_equal_congruent_right` |  |  |  | Y | Y |  |  | unknown | 209&#8209;215 |
| 1423 | `new` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;229 |
| 1424 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;235 |
| 1425 | `expose` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;241 |
| 1426 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;255 |
| 1427 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;277 |
| 1428 | `size` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;280 |
| 1429 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;283 |
| 1430 | `insert` | Y | Y |  |  | Y |  |  | hole | 285&#8209;292 |
| 1431 | `delete` | Y | Y |  |  | Y |  |  | hole | 294&#8209;301 |
| 1432 | `find` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;307 |
| 1433 | `split` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;322 |
| 1434 | `min_key` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;332 |
| 1435 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;342 |
| 1436 | `union` | Y | Y |  |  | Y |  |  | hole | 344&#8209;348 |
| 1437 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;354 |
| 1438 | `difference` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;360 |
| 1439 | `filter` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;380 |
| 1440 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 383&#8209;385 |
| 1441 | `collect_in_order` | Y | Y |  |  | Y |  |  | unknown | 387&#8209;389 |
| 1442 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 391&#8209;394 |
| 1443 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 1387&#8209;1406 |
| 1444 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 1461&#8209;1470 |

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1445 | `new_param_treap_arc` |  |  |  | Y | Y |  |  | unknown | 86&#8209;90 |
| 1446 | `new` | Y | Y |  |  | Y |  |  | unknown | 370&#8209;371 |
| 1447 | `expose` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;385 |
| 1448 | `expose_with_priority` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;399 |
| 1449 | `join_mid` | Y | Y |  |  | Y |  |  | hole | 402&#8209;406 |
| 1450 | `size` | Y | Y |  |  | Y |  |  | unknown | 409&#8209;410 |
| 1451 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 413&#8209;414 |
| 1452 | `insert` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;418 |
| 1453 | `delete` | Y | Y |  |  | Y |  |  | unknown | 421&#8209;422 |
| 1454 | `find` | Y | Y |  |  | Y |  |  | unknown | 425&#8209;428 |
| 1455 | `split` | Y | Y |  |  | Y |  |  | hole | 431&#8209;439 |
| 1456 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 442&#8209;443 |
| 1457 | `union` | Y | Y |  |  | Y |  |  | hole | 446&#8209;447 |
| 1458 | `intersect` | Y | Y |  |  | Y |  |  | hole | 450&#8209;451 |
| 1459 | `difference` | Y | Y |  |  | Y |  |  | hole | 454&#8209;455 |
| 1460 | `filter` | Y | Y |  |  | Y |  |  | hole | 458&#8209;473 |
| 1461 | `reduce` | Y | Y |  |  | Y |  |  | hole | 476&#8209;479 |
| 1462 | `in_order` | Y | Y |  |  | Y |  |  | hole | 482&#8209;483 |
| 1463 | `priority_for` |  |  |  | Y |  | Y | Y |  | 134&#8209;142 |
| 1464 | `tree_priority` |  |  |  | Y |  | Y | Y |  | 144&#8209;151 |
| 1465 | `tree_size` |  |  |  | Y |  | Y | Y |  | 153&#8209;160 |
| 1466 | `make_node` |  |  |  | Y |  | Y | Y |  | 162&#8209;171 |
| 1467 | `join_with_priority` |  |  |  | Y |  | Y | Y |  | 173&#8209;195 |
| 1468 | `split_inner` |  |  |  | Y |  | Y | Y |  | 197&#8209;217 |
| 1469 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 219&#8209;232 |
| 1470 | `union_inner` |  |  |  | Y |  | Y | Y |  | 234&#8209;247 |
| 1471 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 249&#8209;266 |
| 1472 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 268&#8209;285 |
| 1473 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 287&#8209;305 |
| 1474 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 307&#8209;313 |
| 1475 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 315&#8209;336 |
| 1476 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 338&#8209;345 |
| 1477 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 347&#8209;359 |

### Chap39/BSTSetTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1478 | `empty` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;55 |
| 1479 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 1480 | `size` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 1481 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 1482 | `find` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 1483 | `contains` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 1484 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 1485 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;84 |
| 1486 | `insert` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 1487 | `delete` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 1488 | `union` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 1489 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 1490 | `difference` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 1491 | `split` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;109 |
| 1492 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 1493 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 1494 | `filter` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;126 |
| 1495 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;131 |
| 1496 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 1497 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 1498 | `minimum_inner` |  |  |  | Y | Y |  |  | unknown | 143&#8209;147 |
| 1499 | `maximum_inner` |  |  |  | Y | Y |  |  | unknown | 165&#8209;169 |

### Chap39/BSTTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1500 | `lemma_bst_decompose` |  |  |  | Y | Y |  |  | unknown | 108&#8209;118 |
| 1501 | `lemma_contains_left` |  |  |  | Y | Y |  |  | unknown | 122&#8209;124 |
| 1502 | `lemma_contains_right` |  |  |  | Y | Y |  |  | unknown | 128&#8209;130 |
| 1503 | `lemma_contains_root` |  |  |  | Y | Y |  |  | unknown | 134&#8209;135 |
| 1504 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 139&#8209;144 |
| 1505 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 156&#8209;169 |
| 1506 | `lemma_wf_assemble_node` |  |  |  | Y | Y |  |  | unknown | 180&#8209;185 |
| 1507 | `new` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;210 |
| 1508 | `insert` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;214 |
| 1509 | `delete` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;218 |
| 1510 | `find` | Y | Y |  |  | Y |  |  | hole | 220&#8209;221 |
| 1511 | `contains` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;224 |
| 1512 | `size` | Y | Y |  |  | Y |  |  | hole | 226&#8209;227 |
| 1513 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;230 |
| 1514 | `height` | Y | Y |  |  | Y |  | Y |  | 232 |
| 1515 | `minimum` | Y | Y |  |  | Y |  |  | hole | 234&#8209;235 |
| 1516 | `maximum` | Y | Y |  |  | Y |  |  | hole | 237&#8209;238 |
| 1517 | `in_order` | Y | Y |  |  | Y |  |  | hole | 240&#8209;241 |
| 1518 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 243&#8209;244 |
| 1519 | `clone_link` |  |  |  | Y | Y |  |  | hole | 301&#8209;306 |
| 1520 | `size_link` |  |  |  | Y | Y |  |  | unknown | 337&#8209;339 |
| 1521 | `update` |  |  |  | Y | Y |  |  | unknown | 347&#8209;353 |
| 1522 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 362&#8209;370 |
| 1523 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 436&#8209;444 |
| 1524 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 512&#8209;524 |
| 1525 | `delete_link` |  |  |  | Y | Y |  |  | unknown | 617&#8209;627 |
| 1526 | `find_link` |  |  |  | Y | Y |  |  | unknown | 801&#8209;804 |
| 1527 | `min_link` |  |  |  | Y | Y |  |  | unknown | 831&#8209;834 |
| 1528 | `max_link` |  |  |  | Y | Y |  |  | unknown | 856&#8209;859 |
| 1529 | `height_link` |  |  |  | Y | Y |  |  | unknown | 879&#8209;884 |
| 1530 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 908&#8209;911 |
| 1531 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 922&#8209;925 |
| 1532 | `default` |  | Y |  |  | Y |  | Y |  | 1061 |

### Chap39/BSTTreapStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1533 | `lemma_height_le_size` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 1534 | `lemma_size_wf_child_bounded` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;109 |
| 1535 | `lemma_wf_decompose` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;120 |
| 1536 | `lemma_wf_assemble_node` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;127 |
| 1537 | `lemma_contains_left` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 1538 | `lemma_contains_right` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 1539 | `lemma_bst_decompose` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;147 |
| 1540 | `lemma_contains_root` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 1541 | `new` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;160 |
| 1542 | `size` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 1543 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 1544 | `height` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;175 |
| 1545 | `insert` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;189 |
| 1546 | `delete` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;200 |
| 1547 | `find` | Y | Y |  |  | Y |  |  | hole | 203&#8209;209 |
| 1548 | `contains` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;216 |
| 1549 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;224 |
| 1550 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;232 |
| 1551 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;236 |
| 1552 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;240 |
| 1553 | `new_node` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;246 |
| 1554 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;250 |
| 1555 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;259 |
| 1556 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;269 |
| 1557 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;279 |
| 1558 | `clone_link` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;284 |
| 1559 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 286&#8209;290 |
| 1560 | `insert_link` | Y | Y |  |  | Y |  |  | hole | 292&#8209;304 |
| 1561 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;314 |
| 1562 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;317 |
| 1563 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;324 |
| 1564 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;331 |
| 1565 | `in_order_vec` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;334 |
| 1566 | `pre_order_vec` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;337 |
| 1567 | `default` |  | Y |  |  | Y |  |  | unknown | 1108&#8209;1109 |

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1568 | `lemma_content_left_contains_key` |  |  |  | Y | Y |  |  | unknown | 99&#8209;103 |
| 1569 | `lemma_content_right_contains_key` |  |  |  | Y | Y |  |  | unknown | 107&#8209;111 |
| 1570 | `lemma_rotate_left_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 116&#8209;127 |
| 1571 | `lemma_rotate_right_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 132&#8209;143 |
| 1572 | `lemma_left_key_in_link` |  |  |  | Y | Y |  |  | unknown | 148&#8209;157 |
| 1573 | `lemma_right_key_in_link` |  |  |  | Y | Y |  |  | unknown | 162&#8209;171 |
| 1574 | `lemma_node_key_in_link` |  |  |  | Y | Y |  |  | unknown | 176&#8209;184 |
| 1575 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 216&#8209;219 |
| 1576 | `size` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;222 |
| 1577 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;225 |
| 1578 | `height` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;229 |
| 1579 | `insert` | Y | Y |  |  | Y |  |  | hole | 231&#8209;237 |
| 1580 | `delete` | Y | Y |  |  | Y |  |  | hole | 239&#8209;243 |
| 1581 | `find` | Y | Y |  |  | Y |  |  | hole | 245&#8209;249 |
| 1582 | `contains` | Y | Y |  |  | Y |  |  | hole | 251&#8209;253 |
| 1583 | `get` | Y | Y |  |  | Y |  |  | hole | 255&#8209;259 |
| 1584 | `keys` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;263 |
| 1585 | `values` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;267 |
| 1586 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;279 |
| 1587 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;291 |
| 1588 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;298 |
| 1589 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;302 |
| 1590 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;306 |
| 1591 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;311 |
| 1592 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;316 |
| 1593 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;326 |
| 1594 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;336 |
| 1595 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;339 |
| 1596 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;342 |
| 1597 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;345 |
| 1598 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 346&#8209;350 |
| 1599 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;356 |
| 1600 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 357&#8209;360 |
| 1601 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 365&#8209;370 |
| 1602 | `compare_kv_links` |  |  |  | Y | Y |  |  | unknown | 486&#8209;492 |
| 1603 | `default` |  | Y |  |  | Y |  |  | unknown | 824&#8209;825 |
| 1604 | `eq` |  | Y |  |  | Y |  |  | unknown | 865&#8209;866 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1605 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 80&#8209;89 |
| 1606 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 1607 | `identity` x2 | Y | Y |  |  | Y |  |  | hole | 120&#8209;121 |
| 1608 | `combine` x2 | Y | Y |  |  | Y |  |  | hole | 122&#8209;123 |
| 1609 | `lift` x2 | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 1610 | `size` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 1611 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;144 |
| 1612 | `height` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;148 |
| 1613 | `insert` | Y | Y |  |  | Y |  |  | hole | 150&#8209;158 |
| 1614 | `delete` | Y | Y |  |  | Y |  |  | hole | 160&#8209;165 |
| 1615 | `find` | Y | Y |  |  | Y |  |  | hole | 167&#8209;171 |
| 1616 | `contains` | Y | Y |  |  | Y |  |  | hole | 173&#8209;175 |
| 1617 | `get` | Y | Y |  |  | Y |  |  | hole | 177&#8209;181 |
| 1618 | `keys` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;185 |
| 1619 | `values` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 1620 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;196 |
| 1621 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;203 |
| 1622 | `reduced_value` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;208 |
| 1623 | `range_reduce` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 1624 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;218 |
| 1625 | `reduced_value_link` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;222 |
| 1626 | `update_node` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;236 |
| 1627 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;249 |
| 1628 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;257 |
| 1629 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;265 |
| 1630 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;275 |
| 1631 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;278 |
| 1632 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;283 |
| 1633 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;288 |
| 1634 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;292 |
| 1635 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;296 |
| 1636 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;300 |
| 1637 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;304 |
| 1638 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;308 |
| 1639 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;313 |
| 1640 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;321 |
| 1641 | `range_reduce_link` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;326 |
| 1642 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 379&#8209;385 |
| 1643 | `compare_reduced_links` |  |  |  | Y | Y |  |  | unknown | 480&#8209;486 |
| 1644 | `default` |  | Y |  |  | Y |  |  | unknown | 870&#8209;871 |
| 1645 | `eq` |  | Y |  |  | Y |  |  | unknown | 916&#8209;917 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1646 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 68&#8209;73 |
| 1647 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 85&#8209;98 |
| 1648 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 108&#8209;118 |
| 1649 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 149&#8209;153 |
| 1650 | `size` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;156 |
| 1651 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;159 |
| 1652 | `height` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;166 |
| 1653 | `insert` | Y | Y |  |  | Y |  |  | hole | 168&#8209;176 |
| 1654 | `delete` | Y | Y |  |  | Y |  |  | hole | 178&#8209;183 |
| 1655 | `find` | Y | Y |  |  | Y |  |  | hole | 185&#8209;189 |
| 1656 | `contains` | Y | Y |  |  | Y |  |  | hole | 191&#8209;193 |
| 1657 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;199 |
| 1658 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;205 |
| 1659 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;209 |
| 1660 | `rank` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;216 |
| 1661 | `select` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;219 |
| 1662 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;225 |
| 1663 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;230 |
| 1664 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;239 |
| 1665 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;245 |
| 1666 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;252 |
| 1667 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;259 |
| 1668 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;268 |
| 1669 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;271 |
| 1670 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;276 |
| 1671 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;281 |
| 1672 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;287 |
| 1673 | `in_order_collect` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;291 |
| 1674 | `in_order_collect_with_priority` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;295 |
| 1675 | `find_min_priority_idx` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;298 |
| 1676 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;304 |
| 1677 | `filter_by_key` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;306 |
| 1678 | `rank_link` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;312 |
| 1679 | `select_link` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;315 |
| 1680 | `compare_links` |  |  |  | Y | Y |  |  | unknown | 417&#8209;423 |
| 1681 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 823&#8209;829 |
| 1682 | `default` |  | Y |  |  | Y |  |  | unknown | 848&#8209;849 |
| 1683 | `eq` |  | Y |  |  | Y |  |  | unknown | 886&#8209;887 |

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1684 | `size` | Y | Y |  |  | Y |  |  | hole | 116&#8209;118 |
| 1685 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 121&#8209;126 |
| 1686 | `empty` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;132 |
| 1687 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;139 |
| 1688 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;145 |
| 1689 | `filter` | Y | Y |  |  | Y |  |  | hole | 148&#8209;165 |
| 1690 | `intersection` | Y | Y |  |  | Y |  |  | hole | 168&#8209;173 |
| 1691 | `difference` | Y | Y |  |  | Y |  |  | hole | 176&#8209;181 |
| 1692 | `union` | Y | Y |  |  | Y |  |  | hole | 184&#8209;189 |
| 1693 | `find` | Y | Y |  |  | Y |  |  | hole | 192&#8209;194 |
| 1694 | `delete` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;202 |
| 1695 | `insert` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;210 |
| 1696 | `iter` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 1697 | `default` |  | Y |  |  | Y |  | Y |  | 511 |
| 1698 | `next` |  | Y |  |  | Y |  |  | unknown | 519&#8209;535 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1699 | `size` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 1700 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;101 |
| 1701 | `empty` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 1702 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 1703 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 1704 | `filter` | Y | Y |  |  | Y |  |  | hole | 115&#8209;131 |
| 1705 | `intersection` | Y | Y |  |  | Y |  |  | hole | 134&#8209;135 |
| 1706 | `difference` | Y | Y |  |  | Y |  |  | hole | 138&#8209;139 |
| 1707 | `union` | Y | Y |  |  | Y |  |  | hole | 142&#8209;143 |
| 1708 | `find` | Y | Y |  |  | Y |  |  | hole | 146&#8209;148 |
| 1709 | `delete` | Y | Y |  |  | Y |  |  | hole | 151&#8209;152 |
| 1710 | `insert` | Y | Y |  |  | Y |  |  | hole | 155&#8209;156 |
| 1711 | `default` |  | Y |  |  | Y |  | Y |  | 522 |
| 1712 | `eq` |  | Y |  |  | Y |  |  | hole | 534&#8209;535 |
| 1713 | `partial_cmp` |  | Y |  |  | Y |  | Y |  | 567 |
| 1714 | `cmp` |  | Y |  |  | Y |  |  | hole | 575 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1715 | `lemma_wf_implies_len_bound` |  |  |  | Y | Y |  |  | unknown | 67&#8209;72 |
| 1716 | `size` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 1717 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;103 |
| 1718 | `empty` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 1719 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;115 |
| 1720 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;121 |
| 1721 | `filter` | Y | Y |  |  | Y |  |  | hole | 124&#8209;140 |
| 1722 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;147 |
| 1723 | `difference` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;154 |
| 1724 | `union` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;161 |
| 1725 | `find` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;166 |
| 1726 | `delete` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;173 |
| 1727 | `insert` | Y | Y |  |  | Y |  |  | hole | 176&#8209;180 |
| 1728 | `default` |  | Y |  |  | Y |  | Y |  | 985 |
| 1729 | `eq` |  | Y |  |  | Y |  |  | hole | 997&#8209;998 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1730 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 1731 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 1732 | `empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;79 |
| 1733 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;85 |
| 1734 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;91 |
| 1735 | `filter` | Y | Y |  |  | Y |  |  | hole | 94&#8209;110 |
| 1736 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;119 |
| 1737 | `difference` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;128 |
| 1738 | `union` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;137 |
| 1739 | `find` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;142 |
| 1740 | `delete` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;149 |
| 1741 | `insert` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;156 |
| 1742 | `default` |  | Y |  |  | Y |  | Y |  | 942 |
| 1743 | `eq` |  | Y |  |  | Y |  |  | hole | 952&#8209;953 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1744 | `zero_bit_false` |  |  |  | Y | Y |  |  | unknown | 102&#8209;104 |
| 1745 | `set_bit64_proof` |  |  |  | Y | Y |  |  | unknown | 108&#8209;115 |
| 1746 | `bit_or_64_proof` |  |  |  | Y | Y |  |  | unknown | 119&#8209;122 |
| 1747 | `bit_and_64_proof` |  |  |  | Y | Y |  |  | unknown | 126&#8209;129 |
| 1748 | `bit_andnot_64_proof` |  |  |  | Y | Y |  |  | unknown | 133&#8209;136 |
| 1749 | `lemma_bounded_usize_set_finite` |  |  |  | Y | Y |  |  | unknown | 140&#8209;142 |
| 1750 | `lemma_view_finite` |  |  |  | Y | Y |  |  | unknown | 154&#8209;159 |
| 1751 | `new` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;182 |
| 1752 | `size` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;187 |
| 1753 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;195 |
| 1754 | `empty` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;202 |
| 1755 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;211 |
| 1756 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;218 |
| 1757 | `filter` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;229 |
| 1758 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;241 |
| 1759 | `difference` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;253 |
| 1760 | `union` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;265 |
| 1761 | `find` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;270 |
| 1762 | `delete` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;279 |
| 1763 | `insert` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;289 |
| 1764 | `eq` |  | Y |  |  | Y |  |  | hole | 936&#8209;937 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1765 | `lemma_filter_remove` |  |  |  | Y | Y |  |  | unknown | 80&#8209;83 |
| 1766 | `lemma_push_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 119&#8209;124 |
| 1767 | `lemma_filter_in_original` |  |  |  | Y | Y |  |  | unknown | 144&#8209;146 |
| 1768 | `lemma_filter_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 180&#8209;183 |
| 1769 | `lemma_filter_to_set_intersect` |  |  |  | Y | Y |  |  | unknown | 214&#8209;217 |
| 1770 | `lemma_filter_to_set_difference` |  |  |  | Y | Y |  |  | unknown | 249&#8209;252 |
| 1771 | `lemma_subseq_no_dups_subset` |  |  |  | Y | Y |  |  | unknown | 285&#8209;291 |
| 1772 | `size` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;311 |
| 1773 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;318 |
| 1774 | `empty` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;321 |
| 1775 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;324 |
| 1776 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;330 |
| 1777 | `filter` | Y | Y |  |  | Y |  |  | hole | 332&#8209;350 |
| 1778 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 352&#8209;361 |
| 1779 | `difference` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;372 |
| 1780 | `union` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;383 |
| 1781 | `find` | Y | Y |  |  | Y |  |  | unknown | 385&#8209;387 |
| 1782 | `delete` | Y | Y |  |  | Y |  |  | unknown | 389&#8209;396 |
| 1783 | `insert` | Y | Y |  |  | Y |  |  | unknown | 398&#8209;405 |
| 1784 | `default` |  | Y |  |  | Y |  | Y |  | 1146 |
| 1785 | `eq` |  | Y |  |  | Y |  |  | hole | 1158&#8209;1159 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1786 | `example_41_1_array_set` | Y | Y |  | Y | Y |  |  | unknown | 20&#8209;21 |
| 1787 | `example_41_1_avl_set` | Y | Y |  | Y | Y |  |  | unknown | 24&#8209;25 |
| 1788 | `demonstrate_set_operations` | Y | Y |  |  | Y |  |  | unknown | 28&#8209;29 |
| 1789 | `example_41_1_array_set_impl` |  |  |  | Y | Y |  |  | hole | 33&#8209;34 |
| 1790 | `example_41_1_avl_set_impl` |  |  |  | Y | Y |  |  | hole | 86&#8209;87 |
| 1791 | `example_41_3_from_seq_demonstration_impl` |  |  |  | Y | Y |  |  | hole | 139&#8209;140 |
| 1792 | `additional_set_operations_impl` |  |  |  | Y | Y |  |  | hole | 188&#8209;189 |
| 1793 | `example_41_3_from_seq_demonstration` |  |  |  | Y | Y |  |  | unknown | 250&#8209;251 |
| 1794 | `additional_set_operations` |  |  |  | Y | Y |  |  | unknown | 253&#8209;254 |

### Chap42/Example42_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1795 | `_example_42_1_verified` |  |  |  | Y | Y |  | Y |  | 11 |
| 1796 | `example_42_1` | Y |  |  | Y |  | Y | Y |  | 21&#8209;23 |
| 1797 | `demonstrate_table_operations` | Y |  |  |  |  | Y | Y |  | 25&#8209;27 |
| 1798 | `performance_comparison` |  |  |  | Y |  | Y | Y |  | 151&#8209;202 |

### Chap42/TableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1799 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 120&#8209;122 |
| 1800 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 130&#8209;133 |
| 1801 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 149&#8209;152 |
| 1802 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 163&#8209;166 |
| 1803 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 202&#8209;204 |
| 1804 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 212&#8209;219 |
| 1805 | `lemma_entries_to_map_subseq_value` |  |  |  | Y | Y |  |  | unknown | 252&#8209;271 |
| 1806 | `lemma_entries_to_map_skip_prefix` |  |  |  | Y | Y |  |  | unknown | 396&#8209;407 |
| 1807 | `lemma_entries_to_map_ignore_suffix` |  |  |  | Y | Y |  |  | unknown | 449&#8209;459 |
| 1808 | `lemma_entries_to_map_agree_on_key` |  |  |  | Y | Y |  |  | unknown | 479&#8209;492 |
| 1809 | `size` | Y | Y |  |  | Y |  |  | unknown | 531&#8209;533 |
| 1810 | `empty` | Y | Y |  |  | Y |  |  | unknown | 535&#8209;536 |
| 1811 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 538&#8209;539 |
| 1812 | `domain` | Y | Y |  |  | Y |  |  | unknown | 541&#8209;542 |
| 1813 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 544&#8209;546 |
| 1814 | `map` | Y | Y |  |  | Y |  |  | hole | 548&#8209;549 |
| 1815 | `filter` | Y | Y |  |  | Y |  |  | hole | 551&#8209;564 |
| 1816 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 566&#8209;569 |
| 1817 | `union` | Y | Y |  |  | Y |  |  | unknown | 571&#8209;579 |
| 1818 | `difference` | Y | Y |  |  | Y |  |  | unknown | 581&#8209;584 |
| 1819 | `find` | Y | Y |  |  | Y |  |  | unknown | 586&#8209;592 |
| 1820 | `delete` | Y | Y |  |  | Y |  |  | unknown | 594&#8209;595 |
| 1821 | `insert` | Y | Y |  |  | Y |  |  | hole | 597&#8209;604 |
| 1822 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 606&#8209;610 |
| 1823 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 612&#8209;616 |
| 1824 | `entries` | Y | Y |  |  | Y |  |  | unknown | 618&#8209;619 |
| 1825 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1929&#8209;1931 |
| 1826 | `eq` |  | Y |  |  | Y |  |  | unknown | 1954&#8209;1955 |

### Chap42/TableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1827 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 123&#8209;126 |
| 1828 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 142&#8209;145 |
| 1829 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 156&#8209;159 |
| 1830 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 195&#8209;197 |
| 1831 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 205&#8209;212 |
| 1832 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 243&#8209;249 |
| 1833 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 262&#8209;271 |
| 1834 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 280&#8209;282 |
| 1835 | `size` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;301 |
| 1836 | `empty` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;304 |
| 1837 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;308 |
| 1838 | `domain` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;312 |
| 1839 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 314&#8209;322 |
| 1840 | `map` | Y | Y |  |  | Y |  |  | hole | 324&#8209;332 |
| 1841 | `filter` | Y | Y |  |  | Y |  |  | hole | 334&#8209;349 |
| 1842 | `intersection` | Y | Y |  |  | Y |  |  | hole | 351&#8209;362 |
| 1843 | `union` | Y | Y |  |  | Y |  |  | hole | 364&#8209;380 |
| 1844 | `difference` | Y | Y |  |  | Y |  |  | unknown | 382&#8209;389 |
| 1845 | `find` | Y | Y |  |  | Y |  |  | unknown | 391&#8209;397 |
| 1846 | `delete` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;401 |
| 1847 | `insert` | Y | Y |  |  | Y |  |  | hole | 403&#8209;416 |
| 1848 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 418&#8209;425 |
| 1849 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 427&#8209;434 |
| 1850 | `entries` | Y | Y |  |  | Y |  |  | unknown | 437&#8209;438 |
| 1851 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1715&#8209;1719 |
| 1852 | `default` |  | Y |  |  | Y |  | Y |  | 1734 |
| 1853 | `eq` |  | Y |  |  | Y |  |  | unknown | 1748&#8209;1749 |

### Chap42/TableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1854 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 150&#8209;152 |
| 1855 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 163&#8209;170 |
| 1856 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 204&#8209;210 |
| 1857 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 223&#8209;226 |
| 1858 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 239&#8209;241 |
| 1859 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 249&#8209;252 |
| 1860 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 268&#8209;271 |
| 1861 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 313&#8209;322 |
| 1862 | `lemma_spec_collect_domain_step` |  |  |  | Y | Y |  |  | unknown | 332&#8209;335 |
| 1863 | `lemma_spec_collect_key_step` |  |  |  | Y | Y |  |  | unknown | 341&#8209;348 |
| 1864 | `lemma_spec_collect_key_not_in_domain` |  |  |  | Y | Y |  |  | unknown | 354&#8209;357 |
| 1865 | `lemma_spec_collect_key_len_bound` |  |  |  | Y | Y |  |  | unknown | 365&#8209;367 |
| 1866 | `size` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;386 |
| 1867 | `empty` | Y | Y |  |  | Y |  |  | unknown | 389&#8209;390 |
| 1868 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 393&#8209;395 |
| 1869 | `domain` | Y | Y |  |  | Y |  |  | unknown | 398&#8209;400 |
| 1870 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 403&#8209;411 |
| 1871 | `map` | Y | Y |  |  | Y |  |  | hole | 414&#8209;426 |
| 1872 | `filter` | Y | Y |  |  | Y |  |  | hole | 429&#8209;445 |
| 1873 | `intersection` | Y | Y |  |  | Y |  |  | hole | 448&#8209;461 |
| 1874 | `union` | Y | Y |  |  | Y |  |  | hole | 464&#8209;482 |
| 1875 | `difference` | Y | Y |  |  | Y |  |  | unknown | 485&#8209;490 |
| 1876 | `find` | Y | Y |  |  | Y |  |  | unknown | 493&#8209;499 |
| 1877 | `delete` | Y | Y |  |  | Y |  |  | unknown | 502&#8209;508 |
| 1878 | `insert` | Y | Y |  |  | Y |  |  | unknown | 511&#8209;525 |
| 1879 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 528&#8209;533 |
| 1880 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 536&#8209;541 |
| 1881 | `collect` | Y | Y |  |  | Y |  |  | unknown | 544&#8209;545 |
| 1882 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1840&#8209;1844 |
| 1883 | `collect_by_key` |  |  |  | Y | Y |  |  | unknown | 1858&#8209;1872 |
| 1884 | `eq` |  | Y |  |  | Y |  |  | unknown | 2012&#8209;2013 |

### Chap43/AugOrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1885 | `recalculate_reduction` |  |  |  | Y | Y |  |  | unknown | 70&#8209;73 |
| 1886 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 84&#8209;89 |
| 1887 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 114&#8209;117 |
| 1888 | `size` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;129 |
| 1889 | `empty` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 1890 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;133 |
| 1891 | `find` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;140 |
| 1892 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;147 |
| 1893 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;150 |
| 1894 | `insert` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 1895 | `delete` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;159 |
| 1896 | `domain` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;161 |
| 1897 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;169 |
| 1898 | `map` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;172 |
| 1899 | `filter` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;177 |
| 1900 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;180 |
| 1901 | `union` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 1902 | `difference` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;185 |
| 1903 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 1904 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;189 |
| 1905 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;192 |
| 1906 | `collect` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;194 |
| 1907 | `first_key` | Y | Y |  |  | Y |  |  | hole | 196&#8209;202 |
| 1908 | `last_key` | Y | Y |  |  | Y |  |  | hole | 204&#8209;210 |
| 1909 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 212&#8209;218 |
| 1910 | `next_key` | Y | Y |  |  | Y |  |  | hole | 220&#8209;226 |
| 1911 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;229 |
| 1912 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;231 |
| 1913 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;233 |
| 1914 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 235&#8209;240 |
| 1915 | `select_key` | Y | Y |  |  | Y |  |  | hole | 242&#8209;248 |
| 1916 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;251 |
| 1917 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;253 |
| 1918 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;255 |
| 1919 | `reduce_range_parallel` | Y | Y |  |  | Y |  |  | hole | 256&#8209;258 |
| 1920 | `iter` |  |  | Y |  | Y |  |  | unknown | 628&#8209;632 |
| 1921 | `eq` |  | Y |  |  |  | Y | Y |  | 673&#8209;676 |

### Chap43/AugOrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1922 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 72&#8209;79 |
| 1923 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 104&#8209;107 |
| 1924 | `size` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 1925 | `empty` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 1926 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;127 |
| 1927 | `find` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;134 |
| 1928 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;141 |
| 1929 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;144 |
| 1930 | `insert` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;157 |
| 1931 | `delete` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;164 |
| 1932 | `domain` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 1933 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;177 |
| 1934 | `map` | Y | Y |  |  | Y |  |  | hole | 178&#8209;180 |
| 1935 | `filter` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;190 |
| 1936 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;193 |
| 1937 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;206 |
| 1938 | `union` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;224 |
| 1939 | `difference` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;230 |
| 1940 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;236 |
| 1941 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;242 |
| 1942 | `collect` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;244 |
| 1943 | `first_key` | Y | Y |  |  | Y |  |  | hole | 246&#8209;252 |
| 1944 | `last_key` | Y | Y |  |  | Y |  |  | hole | 254&#8209;260 |
| 1945 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 262&#8209;268 |
| 1946 | `next_key` | Y | Y |  |  | Y |  |  | hole | 270&#8209;276 |
| 1947 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;286 |
| 1948 | `join_key` | Y | Y |  |  | Y |  |  | hole | 287&#8209;291 |
| 1949 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;296 |
| 1950 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 298&#8209;303 |
| 1951 | `select_key` | Y | Y |  |  | Y |  |  | hole | 305&#8209;311 |
| 1952 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;321 |
| 1953 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;323 |
| 1954 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;325 |
| 1955 | `iter` |  |  | Y |  | Y |  |  | unknown | 728&#8209;732 |
| 1956 | `eq` |  | Y |  |  |  | Y | Y |  | 778&#8209;781 |

### Chap43/AugOrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1957 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 75&#8209;82 |
| 1958 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 111&#8209;114 |
| 1959 | `size` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;129 |
| 1960 | `empty` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 1961 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 1962 | `find` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;141 |
| 1963 | `insert` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;147 |
| 1964 | `delete` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;154 |
| 1965 | `domain` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 1966 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;167 |
| 1967 | `map` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;178 |
| 1968 | `filter` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;191 |
| 1969 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;206 |
| 1970 | `union` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;226 |
| 1971 | `difference` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;233 |
| 1972 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;240 |
| 1973 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;247 |
| 1974 | `collect` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;249 |
| 1975 | `first_key` | Y | Y |  |  | Y |  |  | hole | 250&#8209;256 |
| 1976 | `last_key` | Y | Y |  |  | Y |  |  | hole | 257&#8209;263 |
| 1977 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 264&#8209;270 |
| 1978 | `next_key` | Y | Y |  |  | Y |  |  | hole | 271&#8209;277 |
| 1979 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;291 |
| 1980 | `join_key` | Y | Y |  |  | Y |  |  | hole | 292&#8209;301 |
| 1981 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;306 |
| 1982 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 307&#8209;312 |
| 1983 | `select_key` | Y | Y |  |  | Y |  |  | hole | 313&#8209;319 |
| 1984 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;329 |
| 1985 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;331 |
| 1986 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;333 |
| 1987 | `iter` |  |  | Y |  | Y |  |  | unknown | 771&#8209;775 |
| 1988 | `eq` |  |  | Y |  | Y |  |  | unknown | 805&#8209;806 |

### Chap43/Example43_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1989 | `_example_43_1_verified` |  |  |  | Y | Y |  | Y |  | 11 |
| 1990 | `run_example43_1` | Y |  |  | Y |  | Y | Y |  | 19&#8209;21 |
| 1991 | `demonstrate_ordered_operations` | Y |  |  |  |  | Y | Y |  | 23&#8209;25 |
| 1992 | `run_integer_example` |  |  |  | Y |  | Y | Y |  | 174&#8209;230 |

### Chap43/OrderedSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1993 | `from_st` |  |  |  | Y | Y |  |  | unknown | 68&#8209;70 |
| 1994 | `size` | Y | Y |  |  | Y |  |  | hole | 96&#8209;97 |
| 1995 | `empty` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 1996 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 1997 | `find` | Y | Y |  |  | Y |  |  | hole | 105&#8209;106 |
| 1998 | `insert` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 1999 | `delete` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 2000 | `filter` | Y | Y |  |  | Y |  |  | hole | 114&#8209;123 |
| 2001 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 2002 | `union` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 2003 | `difference` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 2004 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 134&#8209;138 |
| 2005 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 2006 | `first` | Y | Y |  |  | Y |  |  | hole | 145&#8209;151 |
| 2007 | `last` | Y | Y |  |  | Y |  |  | hole | 153&#8209;159 |
| 2008 | `previous` | Y | Y |  |  | Y |  |  | hole | 161&#8209;167 |
| 2009 | `next` | Y | Y |  |  | Y |  |  | hole | 169&#8209;175 |
| 2010 | `split` | Y | Y |  |  | Y |  |  | hole | 177&#8209;179 |
| 2011 | `join` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;182 |
| 2012 | `get_range` | Y | Y |  |  | Y |  |  | hole | 184&#8209;185 |
| 2013 | `rank` | Y | Y |  |  | Y |  |  | hole | 187&#8209;192 |
| 2014 | `select` | Y | Y |  |  | Y |  |  | hole | 194&#8209;200 |
| 2015 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 202&#8209;204 |

### Chap43/OrderedSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2016 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 2017 | `empty` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;72 |
| 2018 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 2019 | `find` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 2020 | `insert` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;89 |
| 2021 | `delete` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;96 |
| 2022 | `filter` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;110 |
| 2023 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;117 |
| 2024 | `union` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;124 |
| 2025 | `difference` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;131 |
| 2026 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 133&#8209;138 |
| 2027 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 140&#8209;143 |
| 2028 | `first` | Y | Y |  |  | Y |  |  | hole | 147&#8209;154 |
| 2029 | `last` | Y | Y |  |  | Y |  |  | hole | 156&#8209;163 |
| 2030 | `previous` | Y | Y |  |  | Y |  |  | hole | 165&#8209;171 |
| 2031 | `next` x2 | Y | Y |  |  | Y |  |  | hole | 173&#8209;179 |
| 2032 | `split` | Y | Y |  |  | Y |  |  | hole | 181&#8209;194 |
| 2033 | `join` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;198 |
| 2034 | `get_range` | Y | Y |  |  | Y |  |  | hole | 200&#8209;204 |
| 2035 | `rank` | Y | Y |  |  | Y |  |  | hole | 206&#8209;211 |
| 2036 | `select` | Y | Y |  |  | Y |  |  | hole | 213&#8209;220 |
| 2037 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 222&#8209;232 |
| 2038 | `iter` |  |  | Y |  | Y |  |  | unknown | 569&#8209;574 |
| 2039 | `from_sorted_elements` |  |  |  | Y | Y |  |  | unknown | 713&#8209;714 |
| 2040 | `default` |  | Y |  |  |  | Y | Y |  | 738 |
| 2041 | `eq` |  | Y |  |  |  | Y | Y |  | 742&#8209;752 |

### Chap43/OrderedSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2042 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 2043 | `empty` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 2044 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 2045 | `find` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;77 |
| 2046 | `insert` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 2047 | `delete` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 2048 | `filter` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;97 |
| 2049 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 2050 | `union` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 2051 | `difference` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 2052 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;115 |
| 2053 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 2054 | `first` | Y | Y |  |  | Y |  |  | hole | 123&#8209;130 |
| 2055 | `last` | Y | Y |  |  | Y |  |  | hole | 132&#8209;139 |
| 2056 | `previous` | Y | Y |  |  | Y |  |  | hole | 141&#8209;148 |
| 2057 | `next` x2 | Y | Y |  |  | Y |  |  | hole | 150&#8209;157 |
| 2058 | `split` | Y | Y |  |  | Y |  |  | hole | 159&#8209;172 |
| 2059 | `join` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |
| 2060 | `get_range` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;183 |
| 2061 | `rank` | Y | Y |  |  | Y |  |  | hole | 185&#8209;191 |
| 2062 | `select` | Y | Y |  |  | Y |  |  | hole | 193&#8209;200 |
| 2063 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 202&#8209;212 |
| 2064 | `from_sorted_elements` |  |  |  | Y | Y |  |  | unknown | 580&#8209;582 |
| 2065 | `iter` |  |  | Y |  | Y |  |  | unknown | 593&#8209;598 |
| 2066 | `default` |  | Y |  |  | Y |  |  | unknown | 738&#8209;739 |
| 2067 | `eq` |  | Y |  |  |  | Y | Y |  | 771&#8209;773 |

### Chap43/OrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2068 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 2069 | `empty` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 2070 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 2071 | `find` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;81 |
| 2072 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;89 |
| 2073 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;93 |
| 2074 | `insert` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 2075 | `delete` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;104 |
| 2076 | `domain` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 2077 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 2078 | `map` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 2079 | `filter` | Y | Y |  |  | Y |  |  | hole | 117&#8209;126 |
| 2080 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 2081 | `union` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 2082 | `difference` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 2083 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 2084 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;143 |
| 2085 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 2086 | `collect` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 2087 | `first_key` | Y | Y |  |  | Y |  |  | hole | 153&#8209;159 |
| 2088 | `last_key` | Y | Y |  |  | Y |  |  | hole | 162&#8209;168 |
| 2089 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 171&#8209;177 |
| 2090 | `next_key` | Y | Y |  |  | Y |  |  | hole | 180&#8209;186 |
| 2091 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;190 |
| 2092 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;193 |
| 2093 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;196 |
| 2094 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 199&#8209;204 |
| 2095 | `select_key` | Y | Y |  |  | Y |  |  | hole | 207&#8209;213 |
| 2096 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;217 |
| 2097 | `iter` |  |  | Y |  | Y |  |  | unknown | 666&#8209;670 |
| 2098 | `next` |  | Y |  |  | Y |  |  | unknown | 694&#8209;710 |
| 2099 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 801&#8209;803 |
| 2100 | `eq` |  | Y |  |  |  | Y | Y |  | 831&#8209;833 |

### Chap43/OrderedTableMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2101 | `from_st_table` |  |  |  | Y | Y |  |  | hole | 73&#8209;77 |
| 2102 | `size` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 2103 | `empty` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 2104 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 2105 | `find` | Y | Y |  |  | Y |  | Y |  | 112 |
| 2106 | `insert` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 2107 | `delete` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 2108 | `domain` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 2109 | `map` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 2110 | `filter` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;129 |
| 2111 | `first_key` | Y | Y |  |  | Y |  |  | hole | 132&#8209;138 |
| 2112 | `last_key` | Y | Y |  |  | Y |  |  | hole | 141&#8209;147 |
| 2113 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 150&#8209;156 |
| 2114 | `next_key` | Y | Y |  |  | Y |  |  | hole | 159&#8209;165 |
| 2115 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;169 |
| 2116 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;172 |
| 2117 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;175 |
| 2118 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 178&#8209;183 |
| 2119 | `select_key` | Y | Y |  |  | Y |  |  | hole | 186&#8209;192 |
| 2120 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;196 |
| 2121 | `default` |  | Y |  |  |  | Y | Y |  | 511 |

### Chap43/OrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2122 | `size` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;65 |
| 2123 | `empty` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 2124 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 2125 | `find` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;77 |
| 2126 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;84 |
| 2127 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 2128 | `insert` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;100 |
| 2129 | `delete` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;107 |
| 2130 | `domain` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 2131 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;120 |
| 2132 | `map` | Y | Y |  |  | Y |  |  | hole | 121&#8209;123 |
| 2133 | `filter` | Y | Y |  |  | Y |  |  | hole | 124&#8209;138 |
| 2134 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 2135 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;154 |
| 2136 | `union` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;172 |
| 2137 | `difference` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;178 |
| 2138 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;184 |
| 2139 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;190 |
| 2140 | `collect` | Y | Y |  |  | Y |  |  | hole | 191&#8209;192 |
| 2141 | `first_key` | Y | Y |  |  | Y |  |  | hole | 194&#8209;200 |
| 2142 | `last_key` | Y | Y |  |  | Y |  |  | hole | 202&#8209;208 |
| 2143 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 210&#8209;216 |
| 2144 | `next_key` | Y | Y |  |  | Y |  |  | hole | 218&#8209;224 |
| 2145 | `split_key` | Y | Y |  |  | Y |  |  | hole | 226&#8209;240 |
| 2146 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;246 |
| 2147 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 248&#8209;252 |
| 2148 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 254&#8209;259 |
| 2149 | `select_key` | Y | Y |  |  | Y |  |  | hole | 261&#8209;267 |
| 2150 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 269&#8209;279 |
| 2151 | `iter` |  |  | Y |  | Y |  |  | unknown | 698&#8209;702 |
| 2152 | `next` |  | Y |  |  | Y |  |  | unknown | 726&#8209;742 |
| 2153 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 829&#8209;833 |
| 2154 | `eq` |  | Y |  |  |  | Y | Y |  | 860&#8209;862 |

### Chap43/OrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2155 | `size` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;68 |
| 2156 | `empty` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 2157 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 2158 | `find` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;80 |
| 2159 | `insert` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;86 |
| 2160 | `delete` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;93 |
| 2161 | `domain` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 2162 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;106 |
| 2163 | `map` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;117 |
| 2164 | `filter` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;130 |
| 2165 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;145 |
| 2166 | `union` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;165 |
| 2167 | `difference` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;172 |
| 2168 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;179 |
| 2169 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;186 |
| 2170 | `collect` | Y | Y |  |  | Y |  |  | hole | 187&#8209;188 |
| 2171 | `first_key` | Y | Y |  |  | Y |  |  | hole | 190&#8209;196 |
| 2172 | `last_key` | Y | Y |  |  | Y |  |  | hole | 198&#8209;204 |
| 2173 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 206&#8209;212 |
| 2174 | `next_key` | Y | Y |  |  | Y |  |  | hole | 214&#8209;220 |
| 2175 | `split_key` | Y | Y |  |  | Y |  |  | hole | 222&#8209;235 |
| 2176 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;246 |
| 2177 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 248&#8209;252 |
| 2178 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 254&#8209;259 |
| 2179 | `select_key` | Y | Y |  |  | Y |  |  | hole | 261&#8209;267 |
| 2180 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 269&#8209;278 |
| 2181 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 625&#8209;629 |
| 2182 | `iter` |  |  | Y |  | Y |  |  | unknown | 653&#8209;657 |
| 2183 | `next` |  | Y |  |  | Y |  |  | unknown | 681&#8209;697 |
| 2184 | `eq` |  | Y |  |  | Y |  |  | unknown | 785&#8209;786 |

### Chap44/DocumentIndex.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2185 | `_document_index_verified` |  |  |  | Y | Y |  | Y |  | 25 |
| 2186 | `eq` |  | Y |  |  | Y |  |  | unknown | 41&#8209;42 |
| 2187 | `make_index` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 2188 | `find` x3 | Y | Y |  |  |  | Y | Y |  | 289&#8209;291 |
| 2189 | `query_and` | Y | Y |  |  |  | Y | Y |  | 61&#8209;63 |
| 2190 | `query_or` | Y | Y |  |  |  | Y | Y |  | 65&#8209;67 |
| 2191 | `query_and_not` | Y | Y |  |  |  | Y | Y |  | 69&#8209;71 |
| 2192 | `size` | Y | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 2193 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 77&#8209;79 |
| 2194 | `empty` | Y | Y |  |  |  | Y | Y |  | 85&#8209;87 |
| 2195 | `get_all_words` | Y | Y |  |  |  | Y | Y |  | 89&#8209;91 |
| 2196 | `word_count` | Y | Y |  |  |  | Y | Y |  | 93&#8209;95 |
| 2197 | `tokens` |  |  |  | Y |  | Y | Y |  | 199&#8209;226 |
| 2198 | `create_finder` |  |  |  | Y |  | Y | Y |  | 228&#8209;233 |
| 2199 | `new` | Y | Y |  |  |  | Y | Y |  | 285&#8209;287 |
| 2200 | `and` | Y | Y |  |  |  | Y | Y |  | 293&#8209;295 |
| 2201 | `or` | Y | Y |  |  |  | Y | Y |  | 297&#8209;299 |
| 2202 | `and_not` | Y | Y |  |  |  | Y | Y |  | 301&#8209;303 |
| 2203 | `complex_query` | Y | Y |  |  |  | Y | Y |  | 305&#8209;307 |

### Chap44/Example44_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2204 | `_example_44_1_verified` |  |  |  | Y | Y |  | Y |  | 14 |
| 2205 | `create_tweet_collection` |  |  |  | Y |  | Y | Y |  | 17&#8209;27 |
| 2206 | `create_tweet_index` |  |  |  | Y |  | Y | Y |  | 29&#8209;35 |
| 2207 | `create_tweet_finder` |  |  |  | Y |  | Y | Y |  | 37&#8209;44 |
| 2208 | `default` |  | Y |  |  |  | Y | Y |  | 56 |
| 2209 | `new` |  |  | Y |  |  | Y | Y |  | 60&#8209;68 |
| 2210 | `search_fun` |  |  | Y |  |  | Y | Y |  | 70&#8209;73 |
| 2211 | `search_club` |  |  | Y |  |  | Y | Y |  | 75&#8209;78 |
| 2212 | `search_food` |  |  | Y |  |  | Y | Y |  | 80&#8209;83 |
| 2213 | `search_chess` |  |  | Y |  |  | Y | Y |  | 85&#8209;88 |
| 2214 | `complex_query_fun_and_food_or_chess` |  |  | Y |  |  | Y | Y |  | 90&#8209;104 |
| 2215 | `count_fun_but_not_chess` |  |  | Y |  |  | Y | Y |  | 106&#8209;117 |
| 2216 | `search_food_or_fun` |  |  | Y |  |  | Y | Y |  | 119&#8209;127 |
| 2217 | `search_party_and_food` |  |  | Y |  |  | Y | Y |  | 129&#8209;137 |
| 2218 | `get_all_words` |  |  | Y |  |  | Y | Y |  | 139&#8209;142 |
| 2219 | `get_word_count` |  |  | Y |  |  | Y | Y |  | 144&#8209;147 |
| 2220 | `query_builder_example` |  |  | Y |  |  | Y | Y |  | 149&#8209;162 |
| 2221 | `doc_set_to_sorted_vec` |  |  |  | Y |  | Y | Y |  | 165&#8209;178 |
| 2222 | `verify_textbook_examples` |  |  |  | Y |  | Y | Y |  | 180&#8209;220 |
| 2223 | `performance_comparison_demo` |  |  |  | Y |  | Y | Y |  | 222&#8209;236 |
| 2224 | `tokenization_demo` |  |  |  | Y |  | Y | Y |  | 238&#8209;244 |
| 2225 | `index_statistics` |  |  |  | Y |  | Y | Y |  | 246&#8209;265 |

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2226 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 55 |
| 2227 | `empty` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 2228 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 2229 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;73 |
| 2230 | `insert` | Y | Y |  |  | Y |  |  | hole | 75&#8209;80 |
| 2231 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;89 |
| 2232 | `meld` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;93 |
| 2233 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 2234 | `size` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 2235 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 2236 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 2237 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 2238 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;125 |
| 2239 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;129 |
| 2240 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 2241 | `contains` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 2242 | `remove` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;144 |
| 2243 | `range` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;148 |
| 2244 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 2245 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 2246 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 2247 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 2248 | `height` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 2249 | `split` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;174 |
| 2250 | `join` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 2251 | `filter` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;184 |
| 2252 | `map` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;186 |
| 2253 | `default` |  | Y |  |  | Y |  |  | unknown | 558&#8209;559 |
| 2254 | `eq` |  | Y |  |  | Y |  |  | unknown | 610&#8209;611 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2255 | `lemma_log2_bound` |  |  |  | Y | Y |  |  | unknown | 77&#8209;84 |
| 2256 | `empty` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;117 |
| 2257 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;124 |
| 2258 | `find_min` | Y | Y |  |  | Y |  |  | hole | 126&#8209;132 |
| 2259 | `insert` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;140 |
| 2260 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;152 |
| 2261 | `meld` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;161 |
| 2262 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;168 |
| 2263 | `size` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;171 |
| 2264 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;174 |
| 2265 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 2266 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;185 |
| 2267 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | hole | 187&#8209;193 |
| 2268 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;196 |
| 2269 | `height` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;200 |
| 2270 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;206 |
| 2271 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;212 |
| 2272 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;216 |
| 2273 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;224 |
| 2274 | `left_child` |  |  |  | Y | Y |  |  | unknown | 236&#8209;238 |
| 2275 | `right_child` |  |  |  | Y | Y |  |  | unknown | 243&#8209;245 |
| 2276 | `parent` |  |  |  | Y | Y |  |  | unknown | 250&#8209;252 |
| 2277 | `lemma_swap_preserves_multiset` |  |  |  | Y | Y |  |  | unknown | 257&#8209;259 |
| 2278 | `swap_elements` |  |  |  | Y | Y |  |  | unknown | 284&#8209;292 |
| 2279 | `bubble_up` |  |  |  | Y | Y |  |  | unknown | 352&#8209;359 |
| 2280 | `bubble_down` |  |  |  | Y | Y |  |  | unknown | 404&#8209;412 |
| 2281 | `heapify` |  |  |  | Y | Y |  |  | unknown | 465&#8209;472 |
| 2282 | `is_heap` |  |  |  | Y | Y |  |  | unknown | 515&#8209;517 |
| 2283 | `exec_pow2` |  |  |  | Y | Y |  |  | unknown | 543&#8209;545 |
| 2284 | `exec_log2` |  |  |  | Y | Y |  |  | unknown | 570&#8209;573 |
| 2285 | `default` |  | Y |  |  | Y |  | Y |  | 1068 |
| 2286 | `eq` |  | Y |  |  | Y |  |  | unknown | 1093&#8209;1094 |

### Chap45/Example45_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2287 | `_example_45_2_verified` |  |  |  | Y | Y |  | Y |  | 27 |
| 2288 | `example_45_2_textbook_example` | Y | Y |  | Y | Y |  |  | unknown | 32 |
| 2289 | `example_45_2_reverse_sorted` | Y | Y |  | Y | Y |  |  | unknown | 33 |
| 2290 | `example_45_2_already_sorted` | Y | Y |  | Y | Y |  |  | unknown | 34 |
| 2291 | `example_45_2_duplicates` | Y | Y |  | Y | Y |  |  | unknown | 35 |
| 2292 | `example_45_2_single_element` | Y | Y |  | Y | Y |  |  | unknown | 36 |
| 2293 | `example_45_2_empty` | Y | Y |  | Y | Y |  |  | unknown | 37 |
| 2294 | `example_45_2_efficiency_demonstration` | Y | Y |  | Y | Y |  |  | unknown | 38 |
| 2295 | `run_example_45_2` | Y | Y |  | Y | Y |  |  | unknown | 39 |

### Chap45/HeapsortExample.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2296 | `eq` |  | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 2297 | `_heapsort_example_verified` |  |  |  | Y | Y |  | Y |  | 126 |
| 2298 | `is_vec_sorted_exec` |  |  |  | Y | Y |  |  | unknown | 131&#8209;132 |
| 2299 | `all_results_match` | Y | Y |  |  | Y |  | Y |  | 153 |
| 2300 | `all_results_sorted` | Y | Y |  |  | Y |  | Y |  | 155 |
| 2301 | `heapsort_unsorted_list` |  |  |  | Y |  | Y | Y |  | 179&#8209;195 |
| 2302 | `heapsort_sorted_list` |  |  |  | Y |  | Y | Y |  | 197&#8209;213 |
| 2303 | `heapsort_balanced_tree` |  |  |  | Y |  | Y | Y |  | 215&#8209;231 |
| 2304 | `heapsort_binary_heap` |  |  |  | Y |  | Y | Y |  | 233&#8209;249 |
| 2305 | `heapsort_leftist_heap` |  |  |  | Y |  | Y | Y |  | 251&#8209;267 |
| 2306 | `compare_all_heapsorts` |  |  |  | Y |  | Y | Y |  | 269&#8209;279 |
| 2307 | `textbook_example` |  |  |  | Y |  | Y | Y |  | 306&#8209;310 |
| 2308 | `reverse_sorted_example` |  |  |  | Y |  | Y | Y |  | 312&#8209;316 |
| 2309 | `already_sorted_example` |  |  |  | Y |  | Y | Y |  | 318&#8209;322 |
| 2310 | `duplicates_example` |  |  |  | Y |  | Y | Y |  | 324&#8209;328 |
| 2311 | `single_element_example` |  |  |  | Y |  | Y | Y |  | 330&#8209;334 |
| 2312 | `empty_example` |  |  |  | Y |  | Y | Y |  | 336&#8209;340 |
| 2313 | `large_example` |  |  |  | Y |  | Y | Y |  | 342&#8209;352 |
| 2314 | `efficiency_demonstration` |  |  |  | Y |  | Y | Y |  | 354&#8209;363 |
| 2315 | `complexity_analysis` |  |  |  | Y |  | Y | Y |  | 365&#8209;394 |
| 2316 | `correctness_verification` |  |  |  | Y |  | Y | Y |  | 396&#8209;410 |
| 2317 | `vec_to_array_seq` |  |  |  | Y |  | Y | Y |  | 412&#8209;420 |
| 2318 | `vec_to_avl_seq` |  |  |  | Y |  | Y | Y |  | 422&#8209;423 |
| 2319 | `is_sorted` |  |  |  | Y |  | Y | Y |  | 425&#8209;426 |
| 2320 | `generate_test_sequences` |  |  |  | Y |  | Y | Y |  | 428&#8209;443 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2321 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 81 |
| 2322 | `total_order_le` |  |  |  | Y | Y |  |  | unknown | 84&#8209;86 |
| 2323 | `lemma_total_size_monotone` |  |  |  | Y | Y |  |  | unknown | 104&#8209;107 |
| 2324 | `lemma_heap_root_is_min` |  |  |  | Y | Y |  |  | unknown | 115&#8209;121 |
| 2325 | `lemma_rank_le_size` |  |  |  | Y | Y |  |  | unknown | 176&#8209;178 |
| 2326 | `rank` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;206 |
| 2327 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;222 |
| 2328 | `meld_nodes` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;236 |
| 2329 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 318&#8209;320 |
| 2330 | `height` x3 | Y | Y |  |  | Y |  |  | unknown | 330&#8209;332 |
| 2331 | `is_leftist` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;246 |
| 2332 | `is_heap` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;248 |
| 2333 | `is_rank_bounded` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;251 |
| 2334 | `to_vec` x3 | Y | Y |  |  | Y |  |  | unknown | 343&#8209;345 |
| 2335 | `empty` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;268 |
| 2336 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;273 |
| 2337 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;281 |
| 2338 | `insert` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;289 |
| 2339 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;303 |
| 2340 | `meld` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;312 |
| 2341 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;317 |
| 2342 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;322 |
| 2343 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;329 |
| 2344 | `root_rank` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;334 |
| 2345 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;337 |
| 2346 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;342 |
| 2347 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 346&#8209;352 |
| 2348 | `meld_multiple` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;362 |
| 2349 | `split` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;367 |
| 2350 | `default` |  | Y |  |  | Y |  | Y |  | 1108 |
| 2351 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 1134&#8209;1136 |
| 2352 | `format_node` x2 |  | Y |  |  |  | Y | Y |  | 1205&#8209;1215 |
| 2353 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 1246&#8209;1253 |
| 2354 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 1255&#8209;1258 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2355 | `_sorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 57 |
| 2356 | `lemma_push_preserves_sorted` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;72 |
| 2357 | `empty` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;79 |
| 2358 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;87 |
| 2359 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 2360 | `insert` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;103 |
| 2361 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;116 |
| 2362 | `meld` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;127 |
| 2363 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;134 |
| 2364 | `size` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 2365 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 2366 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;144 |
| 2367 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;154 |
| 2368 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;162 |
| 2369 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;168 |
| 2370 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;181 |
| 2371 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;188 |
| 2372 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;192 |
| 2373 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;200 |
| 2374 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;205 |
| 2375 | `default` |  | Y |  |  | Y |  | Y |  | 1023 |
| 2376 | `eq` |  | Y |  |  | Y |  |  | unknown | 1050&#8209;1051 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2377 | `_unsorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 57 |
| 2378 | `empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;71 |
| 2379 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;78 |
| 2380 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;86 |
| 2381 | `insert` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;94 |
| 2382 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;114 |
| 2383 | `meld` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;122 |
| 2384 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 2385 | `size` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 2386 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 2387 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 2388 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;144 |
| 2389 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;152 |
| 2390 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 2391 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;160 |
| 2392 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;168 |
| 2393 | `default` |  | Y |  |  | Y |  | Y |  | 629 |
| 2394 | `eq` |  | Y |  |  | Y |  |  | unknown | 656&#8209;657 |

### Chap47/ChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2395 | `_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 42 |
| 2396 | `hash_index` | Y |  |  |  | Y |  |  | unknown | 55&#8209;59 |
| 2397 | `insert_chained` | Y |  |  |  | Y |  |  | unknown | 64&#8209;71 |
| 2398 | `lookup_chained` | Y |  |  |  | Y |  |  | unknown | 92&#8209;96 |
| 2399 | `delete_chained` | Y |  |  |  | Y |  |  | unknown | 109&#8209;114 |
| 2400 | `eq` |  | Y |  |  | Y |  |  | unknown | 150&#8209;151 |

### Chap47/DoubleHashFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2401 | `second_hash` |  |  | Y |  | Y |  |  | unknown | 40&#8209;42 |
| 2402 | `insert` |  |  | Y |  | Y |  |  | hole | 55 |
| 2403 | `lookup` |  |  | Y |  | Y |  |  | hole | 86 |
| 2404 | `delete` |  |  | Y |  | Y |  |  | hole | 111 |
| 2405 | `resize` |  |  | Y |  | Y |  |  | hole | 141&#8209;144 |
| 2406 | `probe` |  |  | Y |  | Y |  | Y |  | 207 |
| 2407 | `find_slot` |  |  | Y |  | Y |  | Y |  | 215 |

### Chap47/FlatHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2408 | `probe` | Y |  |  |  | Y |  |  | unknown | 48&#8209;52 |
| 2409 | `find_slot` | Y |  |  |  | Y |  |  | unknown | 57&#8209;62 |
| 2410 | `insert_with_probe` | Y |  |  |  | Y |  |  | unknown | 67&#8209;74 |
| 2411 | `lookup_with_probe` | Y |  |  |  | Y |  |  | unknown | 91&#8209;95 |
| 2412 | `new` |  | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 2413 | `insert` |  | Y |  |  | Y |  |  | unknown | 134&#8209;135 |
| 2414 | `lookup` |  | Y |  |  | Y |  |  | unknown | 140&#8209;143 |
| 2415 | `delete` |  | Y |  |  | Y |  |  | unknown | 155&#8209;158 |
| 2416 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 174 |

### Chap47/LinProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2417 | `insert` |  |  | Y |  | Y |  |  | hole | 37 |
| 2418 | `lookup` |  |  | Y |  | Y |  |  | hole | 68 |
| 2419 | `delete` |  |  | Y |  | Y |  |  | hole | 93 |
| 2420 | `resize` |  |  | Y |  | Y |  |  | hole | 123&#8209;126 |
| 2421 | `probe` |  |  | Y |  | Y |  | Y |  | 192 |
| 2422 | `find_slot` |  |  | Y |  | Y |  | Y |  | 199 |

### Chap47/LinkedListChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2423 | `_linked_list_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 32 |
| 2424 | `new` |  | Y |  |  | Y |  | Y |  | 43 |
| 2425 | `insert` |  | Y | Y |  | Y |  |  | hole | 47&#8209;51 |
| 2426 | `lookup` |  | Y | Y |  | Y |  |  | hole | 72 |
| 2427 | `delete` |  | Y | Y |  | Y |  |  | hole | 87&#8209;88 |
| 2428 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 107 |
| 2429 | `resize` |  |  | Y |  | Y |  |  | hole | 202&#8209;205 |
| 2430 | `hash_index` |  |  | Y |  | Y |  | Y |  | 281 |

### Chap47/ParaHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2431 | `lemma_table_to_map_push_empty` |  |  |  | Y | Y |  |  | unknown | 84&#8209;92 |
| 2432 | `lemma_table_to_map_update_contains` |  |  |  | Y | Y |  |  | unknown | 104&#8209;115 |
| 2433 | `call_hash_fn` |  |  |  | Y | Y |  |  | hole | 147&#8209;149 |
| 2434 | `linear_probe` |  |  |  | Y | Y |  |  | unknown | 155&#8209;157 |
| 2435 | `quadratic_probe` |  |  |  | Y | Y |  |  | unknown | 164&#8209;166 |
| 2436 | `compute_second_hash` |  |  |  | Y | Y |  |  | hole | 175&#8209;177 |
| 2437 | `double_hash_probe` |  |  |  | Y | Y |  |  | unknown | 196&#8209;198 |
| 2438 | `new` | Y |  |  |  | Y |  |  | unknown | 214&#8209;215 |
| 2439 | `insert` x2 | Y |  |  |  | Y |  |  | unknown | 288&#8209;297 |
| 2440 | `lookup` x2 | Y |  |  |  | Y |  |  | unknown | 302&#8209;308 |
| 2441 | `delete` x2 | Y |  |  |  | Y |  |  | unknown | 313&#8209;321 |
| 2442 | `clone_entry` | Y |  |  |  | Y |  | Y |  | 228 |
| 2443 | `createTable` | Y |  |  |  | Y |  |  | unknown | 246&#8209;255 |
| 2444 | `metrics` | Y |  |  |  | Y |  |  | unknown | 326&#8209;328 |
| 2445 | `loadAndSize` | Y |  |  |  | Y |  |  | unknown | 335&#8209;339 |
| 2446 | `resize` | Y |  |  |  | Y |  |  | unknown | 352&#8209;359 |

### Chap47/QuadProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2447 | `insert` |  |  | Y |  | Y |  |  | hole | 39 |
| 2448 | `lookup` |  |  | Y |  | Y |  |  | hole | 70 |
| 2449 | `delete` |  |  | Y |  | Y |  |  | hole | 95 |
| 2450 | `resize` |  |  | Y |  | Y |  |  | hole | 125&#8209;128 |
| 2451 | `probe` |  |  | Y |  | Y |  | Y |  | 191 |
| 2452 | `find_slot` |  |  | Y |  | Y |  | Y |  | 198 |

### Chap47/StructChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2453 | `_struct_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 69 |
| 2454 | `default` |  | Y |  |  | Y |  | Y |  | 99 |
| 2455 | `chain_insert` |  |  |  | Y | Y |  |  | unknown | 103&#8209;112 |
| 2456 | `chain_lookup` |  |  |  | Y | Y |  |  | unknown | 137&#8209;144 |
| 2457 | `chain_delete` |  |  |  | Y | Y |  |  | unknown | 159&#8209;167 |
| 2458 | `new` |  | Y |  |  | Y |  | Y |  | 193 |
| 2459 | `insert` |  | Y | Y |  | Y |  |  | hole | 197&#8209;198 |
| 2460 | `lookup` |  | Y | Y |  | Y |  |  | hole | 205 |
| 2461 | `delete` |  | Y | Y |  | Y |  |  | hole | 212&#8209;214 |
| 2462 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 222 |
| 2463 | `resize` |  |  | Y |  | Y |  |  | hole | 272&#8209;275 |
| 2464 | `hash_index` |  |  | Y |  | Y |  | Y |  | 347 |
| 2465 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 373&#8209;375 |

### Chap47/VecChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2466 | `_vec_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 32 |
| 2467 | `new` |  | Y |  |  | Y |  |  | unknown | 43&#8209;44 |
| 2468 | `insert` |  | Y | Y |  | Y |  |  | hole | 49&#8209;53 |
| 2469 | `lookup` |  | Y | Y |  | Y |  |  | hole | 74 |
| 2470 | `delete` |  | Y | Y |  | Y |  |  | hole | 89&#8209;90 |
| 2471 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 109 |
| 2472 | `resize` |  |  | Y |  | Y |  |  | hole | 206&#8209;209 |
| 2473 | `hash_index` |  |  | Y |  | Y |  | Y |  | 282 |

### Chap49/MinEditDistMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2474 | `new` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 2475 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;103 |
| 2476 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;113 |
| 2477 | `source` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 2478 | `target` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 2479 | `set_source` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;131 |
| 2480 | `set_target` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;139 |
| 2481 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;146 |
| 2482 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 150 |
| 2483 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 156&#8209;160 |
| 2484 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 166&#8209;170 |
| 2485 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 177&#8209;191 |
| 2486 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 363&#8209;365 |
| 2487 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 367&#8209;369 |
| 2488 | `eq` |  | Y |  |  |  | Y | Y |  | 380 |

### Chap49/MinEditDistMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2489 | `new` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 2490 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 2491 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 2492 | `source` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 2493 | `target` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 2494 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 123 |
| 2495 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 129&#8209;133 |
| 2496 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 139&#8209;143 |
| 2497 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 150&#8209;164 |
| 2498 | `eq` |  | Y |  |  |  | Y | Y |  | 313 |

### Chap49/MinEditDistStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2499 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 2500 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;99 |
| 2501 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;107 |
| 2502 | `source` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 2503 | `target` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 2504 | `set_source` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;125 |
| 2505 | `set_target` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;133 |
| 2506 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;140 |
| 2507 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 144 |
| 2508 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 151&#8209;166 |
| 2509 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 270&#8209;272 |
| 2510 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 274&#8209;276 |

### Chap49/MinEditDistStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2511 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 2512 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;99 |
| 2513 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 2514 | `source` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 2515 | `target` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 2516 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 118 |
| 2517 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 125&#8209;140 |
| 2518 | `eq` |  | Y |  |  |  | Y | Y |  | 236&#8209;240 |

### Chap49/SubsetSumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2519 | `new` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 2520 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 2521 | `subset_sum` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;106 |
| 2522 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 2523 | `set` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |
| 2524 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 2525 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 126 |
| 2526 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 132&#8209;136 |
| 2527 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 142&#8209;146 |
| 2528 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 153&#8209;163 |
| 2529 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 305&#8209;307 |
| 2530 | `eq` |  | Y |  |  |  | Y | Y |  | 317 |

### Chap49/SubsetSumMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2531 | `new` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 2532 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 2533 | `subset_sum` | Y | Y |  |  | Y |  | Y |  | 102&#8209;104 |
| 2534 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 2535 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 113 |
| 2536 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 119&#8209;123 |
| 2537 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 129&#8209;133 |
| 2538 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 140&#8209;150 |
| 2539 | `eq` |  | Y |  |  |  | Y | Y |  | 278 |

### Chap49/SubsetSumStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2540 | `new` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 2541 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 2542 | `subset_sum` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;94 |
| 2543 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 2544 | `set` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 2545 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 2546 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 114 |
| 2547 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 121&#8209;130 |
| 2548 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 220&#8209;222 |

### Chap49/SubsetSumStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2549 | `new` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 2550 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 2551 | `subset_sum` | Y | Y |  |  | Y |  | Y |  | 91&#8209;93 |
| 2552 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 2553 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 102 |
| 2554 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 109&#8209;118 |
| 2555 | `eq` |  | Y |  |  |  | Y | Y |  | 204&#8209;206 |

### Chap50/MatrixChainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2556 | `new` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;154 |
| 2557 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;157 |
| 2558 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 2559 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;172 |
| 2560 | `dimensions` | Y | Y |  |  | Y |  | Y |  | 174 |
| 2561 | `set_dimension` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 2562 | `update_dimension` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;182 |
| 2563 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;186 |
| 2564 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;190 |
| 2565 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 192 |
| 2566 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;203 |
| 2567 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;214 |
| 2568 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;220 |
| 2569 | `eq` |  | Y |  |  | Y |  |  | unknown | 508&#8209;509 |

### Chap50/MatrixChainMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2570 | `new` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;146 |
| 2571 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;149 |
| 2572 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 2573 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;162 |
| 2574 | `dimensions` | Y | Y |  |  | Y |  | Y |  | 164 |
| 2575 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 2576 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 169 |
| 2577 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;179 |
| 2578 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;190 |
| 2579 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;196 |
| 2580 | `eq` |  | Y |  |  | Y |  |  | unknown | 420&#8209;421 |

### Chap50/MatrixChainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2581 | `new` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;139 |
| 2582 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;144 |
| 2583 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;149 |
| 2584 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;159 |
| 2585 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;162 |
| 2586 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 2587 | `set_dimension` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;171 |
| 2588 | `update_dimension` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;178 |
| 2589 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;183 |
| 2590 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;186 |
| 2591 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;196 |
| 2592 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;211 |
| 2593 | `eq` |  | Y |  |  | Y |  |  | unknown | 382&#8209;383 |

### Chap50/MatrixChainStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2594 | `new` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;139 |
| 2595 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;144 |
| 2596 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;149 |
| 2597 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;158 |
| 2598 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;161 |
| 2599 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 2600 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 2601 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;177 |
| 2602 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;192 |
| 2603 | `eq` |  | Y |  |  | Y |  |  | unknown | 351&#8209;352 |

### Chap50/OptBinSearchTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2604 | `new` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 2605 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 2606 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 2607 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 2608 | `keys` | Y | Y |  |  | Y |  | Y |  | 117 |
| 2609 | `set_key_prob` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 2610 | `update_prob` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 2611 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;129 |
| 2612 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 2613 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 135 |
| 2614 | `obst_rec` |  |  |  | Y | Y |  |  | unknown | 140&#8209;145 |
| 2615 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 370&#8209;371 |

### Chap50/OptBinSearchTreeMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2616 | `new` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 2617 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 2618 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 2619 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 105 |
| 2620 | `keys` | Y | Y |  |  | Y |  | Y |  | 107 |
| 2621 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 2622 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 112 |
| 2623 | `obst_rec` |  |  |  | Y | Y |  |  | unknown | 117&#8209;120 |
| 2624 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 275&#8209;276 |

### Chap50/OptBinSearchTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2625 | `new` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 2626 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;108 |
| 2627 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;113 |
| 2628 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 115 |
| 2629 | `keys` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 2630 | `set_key_prob` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;124 |
| 2631 | `update_prob` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;130 |
| 2632 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;133 |
| 2633 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;138 |
| 2634 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 2635 | `obst_rec_st_eph` |  |  |  | Y | Y |  |  | unknown | 146&#8209;153 |
| 2636 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 340&#8209;342 |

### Chap50/OptBinSearchTreeStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2637 | `new` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 2638 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;108 |
| 2639 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;113 |
| 2640 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 115 |
| 2641 | `keys` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 2642 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 2643 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 2644 | `obst_rec_st_per` |  |  |  | Y | Y |  |  | unknown | 129&#8209;136 |
| 2645 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 302&#8209;304 |

### Chap51/BottomUpDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2646 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;53 |
| 2647 | `new` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;60 |
| 2648 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 2649 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 2650 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 2651 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;74 |
| 2652 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;79 |
| 2653 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;89 |
| 2654 | `default` |  | Y |  |  | Y |  |  | unknown | 285&#8209;288 |
| 2655 | `eq` |  | Y |  |  | Y |  |  | unknown | 311&#8209;312 |

### Chap51/BottomUpDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2656 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;53 |
| 2657 | `new` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;60 |
| 2658 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 2659 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 2660 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 2661 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;77 |
| 2662 | `default` |  | Y |  |  | Y |  |  | unknown | 266&#8209;269 |
| 2663 | `eq` |  | Y |  |  | Y |  |  | unknown | 292&#8209;293 |

### Chap51/BottomUpDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2664 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 2665 | `new` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;59 |
| 2666 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 2667 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 2668 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 2669 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 2670 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;78 |
| 2671 | `med_bottom_up` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;88 |
| 2672 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;104 |
| 2673 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;127 |
| 2674 | `default` |  | Y |  |  | Y |  |  | unknown | 433&#8209;436 |
| 2675 | `eq` |  | Y |  |  | Y |  |  | unknown | 459&#8209;460 |

### Chap51/BottomUpDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2676 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;53 |
| 2677 | `new` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;60 |
| 2678 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 2679 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 2680 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 2681 | `med_bottom_up` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;77 |
| 2682 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;93 |
| 2683 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;116 |
| 2684 | `default` |  | Y |  |  | Y |  |  | unknown | 408&#8209;411 |
| 2685 | `eq` |  | Y |  |  | Y |  |  | unknown | 434&#8209;435 |

### Chap51/TopDownDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2686 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 99&#8209;101 |
| 2687 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 2688 | `new` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 2689 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 2690 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 2691 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 2692 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;142 |
| 2693 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;147 |
| 2694 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;157 |
| 2695 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;167 |
| 2696 | `med_recursive_sequential` |  |  |  | Y | Y |  |  | unknown | 173&#8209;190 |
| 2697 | `med_recursive_parallel` |  |  |  | Y | Y |  |  | unknown | 249&#8209;264 |
| 2698 | `default` |  | Y |  |  | Y |  |  | unknown | 439&#8209;442 |
| 2699 | `eq` |  | Y |  |  | Y |  |  | unknown | 465&#8209;466 |

### Chap51/TopDownDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2700 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 99&#8209;101 |
| 2701 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 2702 | `new` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 2703 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 2704 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 2705 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 2706 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 2707 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 2708 | `med_recursive_sequential` |  |  |  | Y | Y |  |  | unknown | 151&#8209;168 |
| 2709 | `med_recursive_parallel` |  |  |  | Y | Y |  |  | unknown | 227&#8209;242 |
| 2710 | `default` |  | Y |  |  | Y |  |  | unknown | 414&#8209;417 |
| 2711 | `eq` |  | Y |  |  | Y |  |  | unknown | 440&#8209;441 |

### Chap51/TopDownDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2712 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 71&#8209;73 |
| 2713 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 2714 | `new` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;102 |
| 2715 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 2716 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;108 |
| 2717 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 2718 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 2719 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 2720 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;125 |
| 2721 | `insert_memo` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;131 |
| 2722 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;137 |
| 2723 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;142 |
| 2724 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;147 |
| 2725 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;157 |
| 2726 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;172 |
| 2727 | `default` |  | Y |  |  | Y |  |  | unknown | 329&#8209;332 |
| 2728 | `eq` |  | Y |  |  | Y |  |  | unknown | 356&#8209;357 |

### Chap51/TopDownDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2729 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 71&#8209;73 |
| 2730 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 2731 | `new` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;102 |
| 2732 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 2733 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;108 |
| 2734 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 2735 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 2736 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 2737 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;125 |
| 2738 | `with_memo_table` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;130 |
| 2739 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;135 |
| 2740 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 2741 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;155 |
| 2742 | `default` |  | Y |  |  | Y |  |  | unknown | 303&#8209;306 |
| 2743 | `eq` |  | Y |  |  | Y |  |  | unknown | 330&#8209;331 |

### Chap52/AdjMatrixGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2744 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 72&#8209;75 |
| 2745 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 82&#8209;85 |
| 2746 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 92&#8209;95 |
| 2747 | `new` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 2748 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;128 |
| 2749 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 2750 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;147 |
| 2751 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 2752 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;163 |
| 2753 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;171 |
| 2754 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;186 |
| 2755 | `complement` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;196 |

### Chap52/AdjMatrixGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2756 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 73&#8209;76 |
| 2757 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 83&#8209;86 |
| 2758 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 93&#8209;96 |
| 2759 | `new` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;117 |
| 2760 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 2761 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;136 |
| 2762 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;143 |
| 2763 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;157 |
| 2764 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;167 |
| 2765 | `complement` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;177 |

### Chap52/AdjMatrixGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2766 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 72&#8209;75 |
| 2767 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 82&#8209;85 |
| 2768 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 92&#8209;95 |
| 2769 | `new` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 2770 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;128 |
| 2771 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 2772 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;147 |
| 2773 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 2774 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;163 |
| 2775 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;171 |
| 2776 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;186 |
| 2777 | `complement` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;196 |

### Chap52/AdjMatrixGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2778 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 78&#8209;81 |
| 2779 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 88&#8209;91 |
| 2780 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 98&#8209;101 |
| 2781 | `new` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;122 |
| 2782 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;134 |
| 2783 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 2784 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;153 |
| 2785 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 2786 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;169 |
| 2787 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;177 |
| 2788 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;192 |
| 2789 | `complement` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;202 |
| 2790 | `eq` |  | Y |  |  | Y |  |  | unknown | 474&#8209;475 |

### Chap52/AdjSeqGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2791 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 60&#8209;63 |
| 2792 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 71&#8209;73 |
| 2793 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 2794 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 2795 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;111 |
| 2796 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 2797 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;126 |
| 2798 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 2799 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;153 |

### Chap52/AdjSeqGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2800 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 58&#8209;61 |
| 2801 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 68&#8209;70 |
| 2802 | `new` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 2803 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 2804 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;108 |
| 2805 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;115 |
| 2806 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;123 |
| 2807 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |

### Chap52/AdjSeqGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2808 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 60&#8209;63 |
| 2809 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 71&#8209;73 |
| 2810 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 2811 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;108 |
| 2812 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 2813 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;127 |
| 2814 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;134 |
| 2815 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;142 |
| 2816 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 2817 | `set_neighbors` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;167 |
| 2818 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;189 |

### Chap52/AdjSeqGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2819 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 64&#8209;67 |
| 2820 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 75&#8209;77 |
| 2821 | `new` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 2822 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;112 |
| 2823 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |
| 2824 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;131 |
| 2825 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 2826 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;146 |
| 2827 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 2828 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;170 |
| 2829 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;186 |
| 2830 | `eq` |  | Y |  |  | Y |  |  | unknown | 510&#8209;511 |

### Chap52/AdjTableGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2831 | `empty` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 2832 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 2833 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;86 |
| 2834 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;90 |
| 2835 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 2836 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 2837 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 2838 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 2839 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;115 |
| 2840 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;122 |
| 2841 | `default` |  | Y |  |  | Y |  | Y |  | 262 |

### Chap52/AdjTableGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2842 | `lemma_sum_adj_sizes_monotone` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 2843 | `empty` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 2844 | `from_table` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;99 |
| 2845 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 2846 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 2847 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 2848 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 2849 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 2850 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 2851 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 2852 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 2853 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;134 |
| 2854 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;141 |

### Chap52/AdjTableGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2855 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 2856 | `from_table` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;68 |
| 2857 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 2858 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;75 |
| 2859 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 2860 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 2861 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 2862 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 2863 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 2864 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 2865 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;108 |
| 2866 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;115 |

### Chap52/EdgeSetGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2867 | `empty` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;55 |
| 2868 | `from_vertices_and_edges` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;62 |
| 2869 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 2870 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 2871 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 2872 | `edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 2873 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 2874 | `out_neighbors` | Y | Y |  |  | Y |  |  | hole | 79&#8209;81 |
| 2875 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 2876 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 2877 | `delete_vertex` | Y | Y |  |  | Y |  |  | hole | 90&#8209;92 |
| 2878 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 2879 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 2880 | `default` |  | Y |  |  | Y |  | Y |  | 217 |

### Chap52/EdgeSetGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2881 | `empty` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;56 |
| 2882 | `from_vertices_and_edges` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;63 |
| 2883 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 2884 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 2885 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 2886 | `edges` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 2887 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 2888 | `out_neighbors` | Y | Y |  |  | Y |  |  | hole | 80&#8209;82 |
| 2889 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 2890 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 2891 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;93 |
| 2892 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 2893 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |

### Chap52/EdgeSetGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2894 | `empty` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 2895 | `from_vertices_and_edges` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;65 |
| 2896 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 2897 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 2898 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 2899 | `edges` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 2900 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 2901 | `out_neighbors` | Y | Y |  |  | Y |  |  | hole | 82&#8209;84 |
| 2902 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 2903 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 2904 | `delete_vertex` | Y | Y |  |  | Y |  |  | hole | 93&#8209;95 |
| 2905 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 2906 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |

### Chap53/GraphSearchMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2907 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 36&#8209;38 |
| 2908 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 44&#8209;49 |
| 2909 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 53&#8209;60 |
| 2910 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 64&#8209;68 |
| 2911 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 116&#8209;129 |

### Chap53/GraphSearchStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2912 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 28&#8209;32 |
| 2913 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 38&#8209;45 |
| 2914 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 49&#8209;57 |
| 2915 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 61&#8209;67 |
| 2916 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 117&#8209;130 |

### Chap53/GraphSearchStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2917 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 37&#8209;41 |
| 2918 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 47&#8209;54 |
| 2919 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 58&#8209;66 |
| 2920 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 70&#8209;76 |
| 2921 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 125&#8209;138 |

### Chap53/PQMinStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2922 | `pq_min` | Y | Y |  | Y | Y |  |  | unknown | 39&#8209;49 |
| 2923 | `pq_min_multi` | Y | Y |  | Y | Y |  |  | unknown | 53&#8209;64 |
| 2924 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | unknown | 102&#8209;110 |
| 2925 | `pq_explore` |  |  |  | Y | Y |  |  | unknown | 129&#8209;144 |

### Chap53/PQMinStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2926 | `pq_min` | Y | Y |  | Y | Y |  |  | unknown | 43&#8209;53 |
| 2927 | `pq_min_multi` | Y | Y |  | Y | Y |  |  | unknown | 57&#8209;68 |
| 2928 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | unknown | 104&#8209;112 |
| 2929 | `pq_explore` |  |  |  | Y | Y |  |  | unknown | 125&#8209;140 |

### Chap54/BFSMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2930 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 59&#8209;64 |
| 2931 | `lemma_set_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 67&#8209;84 |
| 2932 | `lemma_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 98&#8209;109 |
| 2933 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 119&#8209;125 |
| 2934 | `lemma_set_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 129&#8209;146 |
| 2935 | `copy_distances` |  |  |  | Y | Y |  |  | unknown | 161&#8209;166 |
| 2936 | `copy_graph` |  |  |  | Y | Y |  |  | unknown | 179&#8209;187 |
| 2937 | `lemma_copy_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 212&#8209;225 |
| 2938 | `lemma_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 237&#8209;248 |
| 2939 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;268 |
| 2940 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;277 |
| 2941 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;296 |
| 2942 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;318 |
| 2943 | `process_frontier_parallel` |  |  |  | Y | Y |  |  | unknown | 324&#8209;346 |
| 2944 | `process_frontier_tree_parallel` |  |  |  | Y | Y |  |  | unknown | 482&#8209;500 |

### Chap54/BFSMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2945 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 59&#8209;64 |
| 2946 | `lemma_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 67&#8209;84 |
| 2947 | `lemma_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 98&#8209;109 |
| 2948 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 119&#8209;125 |
| 2949 | `lemma_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 129&#8209;146 |
| 2950 | `copy_distances` |  |  |  | Y | Y |  |  | unknown | 161&#8209;166 |
| 2951 | `copy_graph` |  |  |  | Y | Y |  |  | unknown | 179&#8209;187 |
| 2952 | `lemma_copy_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 213&#8209;226 |
| 2953 | `lemma_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 239&#8209;250 |
| 2954 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;270 |
| 2955 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;279 |
| 2956 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;298 |
| 2957 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;320 |
| 2958 | `process_frontier_parallel` |  |  |  | Y | Y |  |  | unknown | 327&#8209;349 |
| 2959 | `process_frontier_tree_parallel` |  |  |  | Y | Y |  |  | unknown | 494&#8209;512 |

### Chap54/BFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2960 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 59&#8209;64 |
| 2961 | `lemma_set_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 67&#8209;84 |
| 2962 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 98&#8209;104 |
| 2963 | `lemma_set_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 109&#8209;126 |
| 2964 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;157 |
| 2965 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;179 |
| 2966 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;189 |
| 2967 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;196 |

### Chap54/BFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2968 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 59&#8209;64 |
| 2969 | `lemma_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 67&#8209;84 |
| 2970 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 98&#8209;104 |
| 2971 | `lemma_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 108&#8209;125 |
| 2972 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;156 |
| 2973 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;178 |
| 2974 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;188 |
| 2975 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;195 |

### Chap55/CycleDetectStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2976 | `has_cycle` | Y | Y |  |  | Y |  |  | unknown | 32&#8209;37 |
| 2977 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 44&#8209;62 |

### Chap55/CycleDetectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2978 | `has_cycle` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;47 |
| 2979 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 54&#8209;72 |

### Chap55/DFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2980 | `dfs` | Y | Y |  |  | Y |  |  | unknown | 33&#8209;42 |
| 2981 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 48&#8209;64 |

### Chap55/DFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2982 | `dfs` | Y | Y |  |  | Y |  |  | unknown | 34&#8209;43 |
| 2983 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 50&#8209;66 |

### Chap55/SCCStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2984 | `scc` | Y | Y |  |  | Y |  |  | unknown | 37&#8209;42 |
| 2985 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 48&#8209;54 |
| 2986 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 107&#8209;109 |
| 2987 | `check_wf_adj_list_eph` |  |  |  | Y | Y |  |  | unknown | 169&#8209;171 |
| 2988 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 251&#8209;267 |

### Chap55/SCCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2989 | `scc` | Y | Y |  |  | Y |  |  | unknown | 39&#8209;44 |
| 2990 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 50&#8209;74 |
| 2991 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 116&#8209;122 |
| 2992 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 186&#8209;188 |
| 2993 | `check_wf_adj_list_per` |  |  |  | Y | Y |  |  | unknown | 246&#8209;248 |
| 2994 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 291&#8209;307 |

### Chap55/TopoSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2995 | `lemma_set_true_decreases_num_false` |  |  |  | Y | Y |  |  | unknown | 118&#8209;124 |
| 2996 | `lemma_set_true_num_false_eq` |  |  |  | Y | Y |  |  | unknown | 136&#8209;142 |
| 2997 | `lemma_all_true_num_false_zero` |  |  |  | Y | Y |  |  | unknown | 154&#8209;157 |
| 2998 | `lemma_all_false_num_false_eq_len` |  |  |  | Y | Y |  |  | unknown | 165&#8209;168 |
| 2999 | `topo_sort` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;186 |
| 3000 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 193&#8209;217 |
| 3001 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 261&#8209;280 |
| 3002 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 329&#8209;333 |

### Chap55/TopoSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3003 | `topo_sort` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;109 |
| 3004 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 115&#8209;131 |
| 3005 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 167&#8209;186 |
| 3006 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 232&#8209;236 |

### Chap56/AllPairsResultStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3007 | `new` | Y | Y |  |  | Y |  |  | unknown | 40&#8209;43 |
| 3008 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 45&#8209;46 |
| 3009 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 48&#8209;52 |
| 3010 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;55 |
| 3011 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;61 |
| 3012 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 3013 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |

### Chap56/AllPairsResultStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3014 | `new` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;63 |
| 3015 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;69 |
| 3016 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;93 |
| 3017 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 3018 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;124 |
| 3019 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;131 |
| 3020 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;147 |

### Chap56/AllPairsResultStPerF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3021 | `new` | Y | Y |  |  | Y |  |  | unknown | 39&#8209;42 |
| 3022 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 44&#8209;45 |
| 3023 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 47&#8209;51 |
| 3024 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;54 |
| 3025 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;60 |
| 3026 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 3027 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |

### Chap56/AllPairsResultStPerI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3028 | `new` | Y | Y |  |  | Y |  |  | unknown | 48&#8209;62 |
| 3029 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;68 |
| 3030 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;92 |
| 3031 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;99 |
| 3032 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;123 |
| 3033 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;130 |
| 3034 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;146 |

### Chap56/Example56_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3035 | `example_path_weight_int` | Y | Y |  |  | Y |  | Y |  | 32 |
| 3036 | `example_path_weight_float` | Y | Y |  |  | Y |  | Y |  | 35 |
| 3037 | `example_negative_weights` | Y | Y |  |  | Y |  | Y |  | 38 |

### Chap56/Example56_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3038 | `example_negative_cycle` | Y | Y |  |  | Y |  | Y |  | 32 |
| 3039 | `example_undefined_shortest_path` | Y | Y |  |  | Y |  | Y |  | 35 |

### Chap56/PathWeightUtilsStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3040 | `path_weight_int` | Y | Y |  |  | Y |  |  | unknown | 47&#8209;48 |
| 3041 | `path_weight_float` | Y | Y |  |  | Y |  | Y |  | 50&#8209;53 |
| 3042 | `validate_subpath_property_int` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;67 |
| 3043 | `validate_subpath_property_float` | Y | Y |  |  | Y |  | Y |  | 69&#8209;73 |

### Chap56/PathWeightUtilsStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3044 | `path_weight_int` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 3045 | `path_weight_float` | Y | Y |  |  | Y |  | Y |  | 54&#8209;57 |
| 3046 | `validate_subpath_property_int` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;64 |
| 3047 | `validate_subpath_property_float` | Y | Y |  |  | Y |  | Y |  | 66&#8209;70 |

### Chap56/SSSPResultStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3048 | `new` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;56 |
| 3049 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 3050 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;72 |
| 3051 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;79 |
| 3052 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;89 |
| 3053 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 3054 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |

### Chap56/SSSPResultStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3055 | `new` | Y | Y |  |  | Y |  |  | unknown | 41&#8209;50 |
| 3056 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;55 |
| 3057 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;63 |
| 3058 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;69 |
| 3059 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;77 |
| 3060 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;82 |
| 3061 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;95 |

### Chap56/SSSPResultStPerF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3062 | `new` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;68 |
| 3063 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;74 |
| 3064 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;84 |
| 3065 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;91 |
| 3066 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;101 |
| 3067 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;107 |
| 3068 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |

### Chap56/SSSPResultStPerI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3069 | `new` | Y | Y |  |  | Y |  |  | unknown | 40&#8209;49 |
| 3070 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;54 |
| 3071 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;61 |
| 3072 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;67 |
| 3073 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;74 |
| 3074 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;79 |
| 3075 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;92 |

### Chap57/DijkstraStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3076 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 33&#8209;35 |
| 3077 | `cmp` |  | Y |  |  |  | Y | Y |  | 39&#8209;42 |

### Chap57/DijkstraStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3078 | `dijkstra` | Y |  |  | Y | Y |  |  | unknown | 63 |
| 3079 | `pq_entry_new` |  |  |  | Y | Y |  |  | unknown | 68&#8209;70 |
| 3080 | `cmp` |  | Y |  |  | Y |  | Y |  | 76 |
| 3081 | `partial_cmp` |  | Y |  |  | Y |  | Y |  | 88 |

### Chap57/StackStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3082 | `new` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 3083 | `push` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;72 |
| 3084 | `pop` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 3085 | `peek` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 3086 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 3087 | `size` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 3088 | `default` |  | Y |  |  | Y |  |  | unknown | 140&#8209;141 |

### Chap58/BellmanFordStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3089 | `bellman_ford` | Y |  |  | Y | Y |  | Y |  | 26&#8209;27 |
| 3090 | `reconstruct_predecessors` |  |  |  | Y |  | Y | Y |  | 71&#8209;92 |

### Chap59/JohnsonMtEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3091 | `johnson_apsp` | Y |  |  | Y | Y |  | Y |  | 35 |
| 3092 | `parallel_dijkstra_all` |  |  |  | Y |  | Y | Y |  | 77&#8209;136 |
| 3093 | `add_dummy_source` |  |  |  | Y |  | Y | Y |  | 138&#8209;162 |
| 3094 | `reweight_graph` |  |  |  | Y |  | Y | Y |  | 164&#8209;188 |
| 3095 | `create_negative_cycle_result` |  |  |  | Y |  | Y | Y |  | 190&#8209;203 |

### Chap59/JohnsonStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3096 | `johnson_apsp` | Y |  |  | Y | Y |  | Y |  | 34 |
| 3097 | `add_dummy_source` |  |  |  | Y |  | Y | Y |  | 100&#8209;126 |
| 3098 | `reweight_graph` |  |  |  | Y |  | Y | Y |  | 128&#8209;152 |
| 3099 | `create_negative_cycle_result` |  |  |  | Y |  | Y | Y |  | 154&#8209;167 |

### Chap61/EdgeContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3100 | `edge_contract_mt` | Y |  |  | Y | Y |  |  | unknown | 45&#8209;49 |
| 3101 | `contract_round_mt` | Y |  |  | Y | Y |  |  | unknown | 53&#8209;57 |
| 3102 | `build_edges_parallel` |  |  |  | Y |  | Y | Y |  | 125&#8209;178 |

### Chap61/EdgeContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3103 | `edge_contract` | Y |  |  | Y | Y |  |  | unknown | 43&#8209;47 |
| 3104 | `contract_round` | Y |  |  | Y | Y |  |  | unknown | 51&#8209;52 |

### Chap61/VertexMatchingMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3105 | `parallel_matching_mt` | Y |  |  | Y | Y |  |  | unknown | 43&#8209;44 |
| 3106 | `flip_coins_parallel` |  |  |  | Y |  | Y | Y |  | 93&#8209;115 |
| 3107 | `select_edges_parallel` |  |  |  | Y |  | Y | Y |  | 117&#8209;148 |
| 3108 | `select_edges_recursive` |  |  |  | Y |  | Y | Y |  | 150&#8209;193 |
| 3109 | `should_select_edge` |  |  |  | Y |  | Y | Y |  | 195&#8209;226 |

### Chap61/VertexMatchingStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3110 | `greedy_matching` | Y |  |  | Y | Y |  |  | unknown | 39&#8209;40 |
| 3111 | `parallel_matching_st` | Y |  |  | Y | Y |  |  | unknown | 44&#8209;45 |

### Chap62/StarContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3112 | `star_contract_mt` | Y |  |  | Y | Y |  |  | unknown | 44&#8209;49 |
| 3113 | `contract_to_vertices_mt` | Y |  |  | Y | Y |  |  | unknown | 53&#8209;54 |
| 3114 | `build_quotient_graph_parallel` |  |  |  | Y |  | Y | Y |  | 99&#8209;120 |
| 3115 | `route_edges_parallel` |  |  |  | Y |  | Y | Y |  | 122&#8209;172 |

### Chap62/StarContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3116 | `star_contract` | Y |  |  | Y | Y |  |  | unknown | 39&#8209;44 |
| 3117 | `contract_to_vertices` | Y |  |  | Y | Y |  |  | unknown | 48&#8209;49 |
| 3118 | `build_quotient_graph` |  |  |  | Y |  | Y | Y |  | 93&#8209;124 |

### Chap62/StarPartitionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3119 | `parallel_star_partition` | Y |  |  | Y | Y |  |  | unknown | 39&#8209;43 |

### Chap62/StarPartitionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3120 | `sequential_star_partition` | Y |  |  | Y | Y |  |  | unknown | 38&#8209;39 |

### Chap63/ConnectivityMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3121 | `count_components_mt` | Y |  |  | Y | Y |  |  | unknown | 49&#8209;50 |
| 3122 | `connected_components_mt` | Y |  |  | Y | Y |  |  | unknown | 54&#8209;58 |
| 3123 | `count_components_hof` | Y |  |  | Y | Y |  |  | unknown | 62&#8209;63 |
| 3124 | `connected_components_hof` | Y |  |  | Y | Y |  |  | unknown | 67&#8209;71 |
| 3125 | `build_quotient_edges_parallel` |  |  |  | Y |  | Y | Y |  | 145&#8209;161 |
| 3126 | `route_edges_parallel` |  |  |  | Y |  | Y | Y |  | 163&#8209;213 |
| 3127 | `compose_maps_parallel` |  |  |  | Y |  | Y | Y |  | 215&#8209;230 |

### Chap63/ConnectivityStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3128 | `count_components` | Y |  |  | Y | Y |  |  | unknown | 44&#8209;45 |
| 3129 | `connected_components` | Y |  |  | Y | Y |  |  | unknown | 49&#8209;50 |
| 3130 | `count_components_hof` | Y |  |  | Y | Y |  |  | unknown | 54&#8209;55 |
| 3131 | `connected_components_hof` | Y |  |  | Y | Y |  |  | unknown | 59&#8209;60 |
| 3132 | `build_quotient_edges` |  |  |  | Y |  | Y | Y |  | 136&#8209;164 |

### Chap64/SpanTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3133 | `new_spanning_edges_arc` |  |  |  | Y | Y |  |  | unknown | 41&#8209;45 |
| 3134 | `spanning_tree_star_contraction_mt` | Y |  |  | Y | Y |  |  | unknown | 60&#8209;63 |
| 3135 | `verify_spanning_tree` | Y |  |  | Y | Y |  |  | unknown | 67&#8209;68 |

### Chap64/SpanTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3136 | `spanning_tree_star_contraction` | Y |  |  | Y | Y |  |  | unknown | 40&#8209;41 |
| 3137 | `verify_spanning_tree` | Y |  |  | Y | Y |  |  | unknown | 45&#8209;46 |

### Chap64/TSPApproxStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3138 | `euler_tour` | Y |  |  | Y | Y |  |  | unknown | 49&#8209;54 |
| 3139 | `shortcut_tour` | Y |  |  | Y | Y |  | Y |  | 58 |
| 3140 | `tour_weight` | Y |  |  | Y | Y |  |  | unknown | 62&#8209;66 |
| 3141 | `approx_metric_tsp` | Y |  |  | Y | Y |  |  | unknown | 70&#8209;75 |
| 3142 | `euler_tour_dfs` |  |  |  | Y |  | Y | Y |  | 109&#8209;160 |
| 3143 | `get_neighbors` |  |  |  | Y |  | Y | Y |  | 225&#8209;239 |
| 3144 | `get_edge_weight` |  |  |  | Y |  | Y | Y |  | 241&#8209;256 |

### Chap65/KruskalStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3145 | `kruskal_mst` | Y |  |  | Y | Y |  |  | unknown | 39&#8209;42 |
| 3146 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 46 |
| 3147 | `verify_mst_size` | Y |  |  | Y | Y |  |  | unknown | 50&#8209;54 |

### Chap65/PrimStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3148 | `prim_mst` | Y |  |  | Y | Y |  |  | unknown | 63&#8209;67 |
| 3149 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 71 |
| 3150 | `pq_entry_new` |  |  |  | Y |  | Y | Y |  | 76&#8209;86 |
| 3151 | `cmp` |  | Y |  |  |  | Y | Y |  | 90&#8209;92 |
| 3152 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 97&#8209;99 |

### Chap65/UnionFindStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3153 | `new` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;80 |
| 3154 | `insert` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;90 |
| 3155 | `find` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;112 |
| 3156 | `union` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;126 |
| 3157 | `equals` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;147 |
| 3158 | `num_sets` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;157 |

### Chap66/BoruvkaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3159 | `eq` |  | Y |  |  | Y |  |  | unknown | 48&#8209;49 |
| 3160 | `vertex_bridges_mt` | Y |  |  | Y | Y |  | Y |  | 85&#8209;89 |
| 3161 | `bridge_star_partition_mt` | Y |  |  | Y | Y |  | Y |  | 93&#8209;98 |
| 3162 | `boruvka_mst_mt` | Y |  |  | Y | Y |  | Y |  | 102&#8209;108 |
| 3163 | `boruvka_mst_mt_with_seed` | Y |  |  | Y | Y |  |  | unknown | 112&#8209;117 |
| 3164 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 121&#8209;124 |
| 3165 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 131&#8209;133 |
| 3166 | `cmp` |  | Y |  |  |  | Y | Y |  | 137&#8209;142 |
| 3167 | `hash` |  | Y |  |  |  | Y | Y |  | 146&#8209;151 |
| 3168 | `hash_coin` |  |  |  | Y |  | Y | Y |  | 166&#8209;177 |
| 3169 | `hash_coin_flips_mt` |  |  |  | Y |  | Y | Y |  | 179&#8209;212 |
| 3170 | `compute_remaining_mt` |  |  |  | Y |  | Y | Y |  | 214&#8209;250 |
| 3171 | `collect_mst_labels_mt` |  |  |  | Y |  | Y | Y |  | 252&#8209;287 |
| 3172 | `build_partition_map_mt` |  |  |  | Y |  | Y | Y |  | 289&#8209;329 |
| 3173 | `filter_tail_to_head_mt` |  |  |  | Y |  | Y | Y |  | 430&#8209;480 |
| 3174 | `reroute_edges_mt` |  |  |  | Y |  | Y | Y |  | 545&#8209;587 |

### Chap66/BoruvkaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3175 | `eq` |  | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 3176 | `coin_flip` |  |  |  | Y | Y |  |  | unknown | 100&#8209;102 |
| 3177 | `vertex_bridges` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;127 |
| 3178 | `bridge_star_partition` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;142 |
| 3179 | `boruvka_mst` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;158 |
| 3180 | `boruvka_mst_with_seed` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;173 |
| 3181 | `mst_weight` | Y | Y |  |  | Y |  | Y |  | 177&#8209;180 |
| 3182 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 505&#8209;507 |
| 3183 | `cmp` |  | Y |  |  |  | Y | Y |  | 511&#8209;516 |
| 3184 | `hash` |  | Y |  |  |  | Y | Y |  | 520&#8209;525 |

### src/Concurrency.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3185 | `diverge` |  |  |  | Y | Y |  |  | hole | 16 |

### src/ParaPairs.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3186 | `para_pair` |  |  |  | Y | Y |  |  | unknown | 59&#8209;70 |
| 3187 | `para_pair_disjoint` |  |  |  | Y | Y |  |  | unknown | 79&#8209;92 |

### src/Types.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3188 | `lemma_spec_graphview_wf_subset_arcs` |  |  |  | Y | Y |  |  | unknown | 53&#8209;58 |
| 3189 | `lemma_spec_labgraphview_wf_subset_arcs` |  |  |  | Y | Y |  |  | unknown | 80&#8209;85 |
| 3190 | `axiom_Pair_view_injective` |  |  |  | Y | Y |  |  | hole | 196&#8209;200 |
| 3191 | `axiom_Pair_feq` |  |  |  | Y | Y |  |  | hole | 212&#8209;214 |
| 3192 | `axiom_Pair_key_model` |  |  |  | Y | Y |  |  | hole | 217&#8209;219 |
| 3193 | `axiom_Edge_feq` |  |  |  | Y | Y |  |  | hole | 240&#8209;242 |
| 3194 | `axiom_Edge_key_model` |  |  |  | Y | Y |  |  | hole | 245&#8209;247 |
| 3195 | `axiom_LabEdge_feq` |  |  |  | Y | Y |  |  | hole | 267&#8209;269 |
| 3196 | `axiom_LabEdge_key_model` |  |  |  | Y | Y |  |  | hole | 272&#8209;274 |
| 3197 | `axiom_WeightedEdge_feq` |  |  |  | Y | Y |  |  | hole | 289&#8209;291 |
| 3198 | `axiom_WeightedEdge_key_model` |  |  |  | Y | Y |  |  | hole | 294&#8209;296 |
| 3199 | `axiom_WeightedLabEdge_feq` |  |  |  | Y | Y |  |  | hole | 316&#8209;318 |
| 3200 | `axiom_WeightedLabEdge_key_model` |  |  |  | Y | Y |  |  | hole | 321&#8209;323 |
| 3201 | `axiom_Triple_feq` |  |  |  | Y | Y |  |  | hole | 338&#8209;340 |
| 3202 | `axiom_Triple_key_model` |  |  |  | Y | Y |  |  | hole | 343&#8209;345 |
| 3203 | `from` x10 |  | Y |  |  |  | Y | Y |  | 445 |
| 3204 | `next` |  | Y |  |  |  | Y | Y |  | 516&#8209;518 |
| 3205 | `deref` |  | Y |  |  |  | Y | Y |  | 525&#8209;527 |
| 3206 | `deref_mut` |  | Y |  |  |  | Y | Y |  | 531&#8209;533 |

### standards/arc_usage_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3207 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 3208 | `get` x3 | Y | Y |  |  | Y |  |  | hole | 140&#8209;142 |
| 3209 | `example_arc_closure_sharing` |  |  |  | Y | Y |  | Y |  | 205 |

### standards/deep_view_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3210 | `lemma_deep_view_len` |  |  |  | Y | Y |  |  | unknown | 111&#8209;113 |
| 3211 | `test_tuple_deep_view` |  |  |  | Y | Y |  |  | unknown | 118&#8209;120 |
| 3212 | `iter` |  |  | Y |  | Y |  |  | unknown | 145&#8209;149 |
| 3213 | `next` |  | Y |  |  | Y |  |  | unknown | 160&#8209;177 |

### standards/finite_sets_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3214 | `new` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;106 |
| 3215 | `insert` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 3216 | `len` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |

### standards/hfscheduler_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3217 | `new_arc_counter` |  |  |  | Y | Y |  |  | hole | 47&#8209;50 |
| 3218 | `clone_arc_counter` |  |  |  | Y | Y |  |  | hole | 57&#8209;59 |
| 3219 | `parallel_reads` |  |  |  | Y | Y |  | Y |  | 64 |
| 3220 | `increment` |  |  |  | Y | Y |  |  | unknown | 105&#8209;106 |
| 3221 | `parallel_writes` |  |  |  | Y | Y |  | Y |  | 116 |
| 3222 | `parallel_reads_parapair` |  |  |  | Y | Y |  | Y |  | 144 |

### standards/iterators_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3223 | `new` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 3224 | `iter` |  |  | Y |  | Y |  |  | unknown | 142&#8209;146 |
| 3225 | `next` |  | Y |  |  | Y |  |  | unknown | 161&#8209;178 |

### standards/mod_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3226 | `new` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;95 |
| 3227 | `length` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;100 |
| 3228 | `iter` |  |  | Y |  | Y |  |  | unknown | 130&#8209;134 |
| 3229 | `next` |  | Y |  |  | Y |  |  | unknown | 145&#8209;162 |

### standards/multi_struct_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3230 | `new` x5 | Y | Y |  |  | Y |  |  | unknown | 109&#8209;113 |
| 3231 | `set_key` x3 | Y | Y |  |  | Y |  |  | unknown | 88&#8209;93 |

### standards/mut_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3232 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 85&#8209;88 |
| 3233 | `inc` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;70 |
| 3234 | `add` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;77 |
| 3235 | `set_fst` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;94 |
| 3236 | `set_snd` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;99 |
| 3237 | `swap` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;105 |
| 3238 | `increment_ref` |  |  |  | Y | Y |  |  | unknown | 176&#8209;180 |
| 3239 | `collect_range` |  |  |  | Y | Y |  |  | unknown | 187&#8209;192 |
| 3240 | `replace_item` |  |  |  | Y | Y |  |  | unknown | 209&#8209;211 |
| 3241 | `proof_sequential_mutations` |  |  |  | Y | Y |  | Y |  | 219 |
| 3242 | `proof_field_mutation` |  |  |  | Y | Y |  | Y |  | 233 |
| 3243 | `proof_ref_parameter` |  |  |  | Y | Y |  | Y |  | 244 |
| 3244 | `proof_collect_range` |  |  |  | Y | Y |  | Y |  | 253 |

### standards/partial_eq_eq_clone_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3245 | `eq` |  | Y |  |  | Y |  |  | unknown | 131&#8209;132 |

### standards/spec_naming_convention.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3246 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 137&#8209;140 |
| 3247 | `increment` x3 | Y | Y |  |  | Y |  |  | hole | 142&#8209;148 |
| 3248 | `value` x3 | Y | Y |  |  | Y |  |  | hole | 150&#8209;152 |
| 3249 | `full` x3 | Y | Y |  |  | Y |  |  | hole | 154&#8209;156 |

### standards/spec_wf_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3250 | `empty` x3 | Y | Y |  |  | Y |  |  | unknown | 222&#8209;225 |
| 3251 | `len` x3 | Y | Y |  |  | Y |  |  | unknown | 227&#8209;231 |
| 3252 | `push` x3 | Y | Y |  |  | Y |  |  | unknown | 233&#8209;238 |
| 3253 | `merge` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;163 |
| 3254 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;170 |

### standards/table_of_contents_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3255 | `lemma_view_len_nat` |  |  |  | Y | Y |  |  | unknown | 133&#8209;135 |
| 3256 | `new` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;158 |
| 3257 | `length` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;163 |
| 3258 | `nth` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;170 |
| 3259 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;175 |
| 3260 | `iter` |  |  | Y |  | Y |  |  | unknown | 218&#8209;222 |
| 3261 | `next` |  | Y |  |  | Y |  |  | unknown | 244&#8209;261 |
| 3262 | `eq` |  | Y |  |  | Y |  |  | hole | 371&#8209;372 |

### standards/toplevel_coarse_rwlocks_for_mt_modules.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3263 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 171&#8209;174 |
| 3264 | `count_down` x3 | Y | Y |  |  | Y |  |  | unknown | 178&#8209;184 |
| 3265 | `count` x3 | Y | Y |  |  | Y |  |  | unknown | 186&#8209;188 |
| 3266 | `done` x3 | Y | Y |  |  | Y |  |  | unknown | 190&#8209;192 |

### standards/tsm_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3267 | `parallel_add` |  |  |  | Y | Y |  |  | unknown | 117&#8209;119 |

### standards/using_closures_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3268 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;72 |
| 3269 | `map_apply` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;84 |
| 3270 | `filter` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;102 |

### standards/view_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3271 | `new` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;112 |
| 3272 | `get` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;117 |
| 3273 | `length` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;131 |
| 3274 | `nth` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;138 |
| 3275 | `iter` |  |  | Y |  | Y |  |  | unknown | 176&#8209;180 |
| 3276 | `next` |  | Y |  |  | Y |  |  | unknown | 191&#8209;208 |

### standards/wrapping_iterators_standard.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3277 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 89&#8209;92 |
| 3278 | `iter` x2 |  |  | Y |  | Y |  |  | unknown | 130&#8209;133 |
| 3279 | `next` x2 |  | Y |  |  | Y |  |  | unknown | 216&#8209;233 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
