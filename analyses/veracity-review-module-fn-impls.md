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
| 2 | Chap02 | HFSchedulerMtEph | 0 | 0 | 0 | 8 | 5 | 3 | 0 | 5 | 3 |
| 3 | Chap03 | InsertionSortStEph | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 4 | Chap05 | KleeneStPer | 4 | 4 | 0 | 9 | 13 | 0 | 13 | 0 | 0 |
| 5 | Chap05 | MappingStEph | 13 | 16 | 0 | 0 | 16 | 0 | 14 | 1 | 1 |
| 6 | Chap05 | RelationStEph | 9 | 12 | 0 | 0 | 12 | 0 | 11 | 0 | 1 |
| 7 | Chap05 | SetMtEph | 17 | 20 | 0 | 1 | 21 | 0 | 19 | 1 | 1 |
| 8 | Chap05 | SetStEph | 18 | 21 | 0 | 1 | 22 | 0 | 20 | 1 | 1 |
| 9 | Chap06 | DirGraphMtEph | 17 | 18 | 0 | 5 | 23 | 0 | 23 | 0 | 0 |
| 10 | Chap06 | DirGraphStEph | 17 | 18 | 2 | 0 | 20 | 0 | 20 | 0 | 0 |
| 11 | Chap06 | LabDirGraphMtEph | 11 | 11 | 0 | 2 | 13 | 0 | 13 | 0 | 0 |
| 12 | Chap06 | LabDirGraphStEph | 11 | 11 | 0 | 0 | 11 | 0 | 11 | 0 | 0 |
| 13 | Chap06 | LabUnDirGraphMtEph | 10 | 10 | 0 | 1 | 11 | 0 | 11 | 0 | 0 |
| 14 | Chap06 | LabUnDirGraphStEph | 10 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 15 | Chap06 | UnDirGraphMtEph | 11 | 12 | 0 | 2 | 14 | 0 | 14 | 0 | 0 |
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
| 29 | Chap11 | FibonacciMtEph2Threads | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 0 |
| 30 | Chap11 | FibonacciMtEphRecomputes | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 0 |
| 31 | Chap11 | FibonacciMtPerAllThreads | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 32 | Chap11 | FibonacciMtPerTSM | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 0 |
| 33 | Chap11 | FibonacciStEph | 0 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |
| 34 | Chap12 | Exercise12_1 | 4 | 5 | 0 | 1 | 6 | 0 | 0 | 5 | 1 |
| 35 | Chap12 | Exercise12_2 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 36 | Chap12 | Exercise12_5 | 5 | 7 | 0 | 0 | 7 | 0 | 0 | 6 | 1 |
| 37 | Chap17 | MathSeq | 18 | 20 | 1 | 0 | 20 | 1 | 19 | 1 | 1 |
| 38 | Chap18 | ArraySeq | 23 | 25 | 3 | 13 | 41 | 0 | 39 | 1 | 1 |
| 39 | Chap18 | ArraySeqMtEph | 22 | 24 | 7 | 3 | 34 | 0 | 33 | 1 | 0 |
| 40 | Chap18 | ArraySeqMtPer | 19 | 21 | 6 | 0 | 27 | 0 | 26 | 1 | 0 |
| 41 | Chap18 | ArraySeqStEph | 21 | 23 | 2 | 0 | 25 | 0 | 24 | 1 | 0 |
| 42 | Chap18 | ArraySeqStPer | 20 | 22 | 2 | 0 | 24 | 0 | 23 | 1 | 0 |
| 43 | Chap18 | LinkedListStEph | 19 | 21 | 2 | 0 | 23 | 0 | 22 | 1 | 0 |
| 44 | Chap18 | LinkedListStPer | 18 | 20 | 2 | 0 | 22 | 0 | 21 | 1 | 0 |
| 45 | Chap19 | ArraySeqMtEph | 25 | 27 | 6 | 4 | 37 | 0 | 36 | 1 | 0 |
| 46 | Chap19 | ArraySeqMtEphSlice | 8 | 8 | 1 | 0 | 9 | 0 | 9 | 0 | 0 |
| 47 | Chap19 | ArraySeqStEph | 24 | 26 | 2 | 2 | 30 | 0 | 29 | 1 | 0 |
| 48 | Chap19 | ArraySeqStPer | 23 | 25 | 2 | 2 | 29 | 0 | 28 | 1 | 0 |
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
| 60 | Chap23 | BalBinTreeStEph | 8 | 10 | 3 | 3 | 16 | 0 | 14 | 2 | 0 |
| 61 | Chap23 | PrimTreeSeqStPer | 17 | 19 | 1 | 0 | 20 | 0 | 19 | 1 | 0 |
| 62 | Chap26 | DivConReduceMtPer | 5 | 5 | 0 | 3 | 8 | 0 | 8 | 0 | 0 |
| 63 | Chap26 | DivConReduceStPer | 5 | 5 | 0 | 0 | 5 | 0 | 5 | 0 | 0 |
| 64 | Chap26 | ETSPMtEph | 1 | 1 | 1 | 10 | 8 | 4 | 5 | 3 | 4 |
| 65 | Chap26 | ETSPStEph | 1 | 1 | 1 | 8 | 7 | 3 | 4 | 3 | 3 |
| 66 | Chap26 | MergeSortMtPer | 2 | 2 | 0 | 6 | 8 | 0 | 8 | 0 | 0 |
| 67 | Chap26 | MergeSortStPer | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |
| 68 | Chap26 | ScanDCMtPer | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 69 | Chap26 | ScanDCStPer | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |
| 70 | Chap27 | ReduceContractMtEph | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 71 | Chap27 | ReduceContractStEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 72 | Chap27 | ScanContractMtEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 73 | Chap27 | ScanContractStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
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
| 85 | Chap35 | OrderStatSelectMtEph | 1 | 1 | 0 | 2 | 3 | 0 | 2 | 1 | 0 |
| 86 | Chap35 | OrderStatSelectMtPer | 1 | 1 | 0 | 2 | 3 | 0 | 2 | 1 | 0 |
| 87 | Chap35 | OrderStatSelectStEph | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 88 | Chap35 | OrderStatSelectStPer | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 89 | Chap36 | QuickSortMtEph | 0 | 0 | 0 | 9 | 9 | 0 | 9 | 0 | 0 |
| 90 | Chap36 | QuickSortMtEphSlice | 6 | 6 | 0 | 0 | 6 | 0 | 0 | 1 | 5 |
| 91 | Chap36 | QuickSortStEph | 0 | 0 | 0 | 9 | 9 | 0 | 9 | 0 | 0 |
| 92 | Chap37 | AVLTreeSeq | 20 | 23 | 0 | 13 | 33 | 3 | 10 | 23 | 3 |
| 93 | Chap37 | AVLTreeSeqMtPer | 11 | 14 | 0 | 13 | 25 | 2 | 11 | 12 | 4 |
| 94 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 0 | 12 | 30 | 3 | 10 | 20 | 3 |
| 95 | Chap37 | AVLTreeSeqStPer | 13 | 16 | 0 | 14 | 27 | 3 | 11 | 14 | 5 |
| 96 | Chap37 | BSTAVLMtEph | 0 | 0 | 6 | 8 | 14 | 0 | 6 | 0 | 8 |
| 97 | Chap37 | BSTAVLStEph | 0 | 0 | 0 | 17 | 17 | 0 | 15 | 0 | 2 |
| 98 | Chap37 | BSTBBAlphaMtEph | 0 | 0 | 6 | 5 | 11 | 0 | 3 | 0 | 8 |
| 99 | Chap37 | BSTBBAlphaStEph | 0 | 0 | 0 | 12 | 12 | 0 | 10 | 0 | 2 |
| 100 | Chap37 | BSTPlainMtEph | 0 | 0 | 6 | 5 | 11 | 0 | 3 | 0 | 8 |
| 101 | Chap37 | BSTPlainStEph | 0 | 0 | 0 | 12 | 12 | 0 | 10 | 0 | 2 |
| 102 | Chap37 | BSTRBMtEph | 14 | 16 | 0 | 20 | 1 | 35 | 0 | 1 | 35 |
| 103 | Chap37 | BSTRBStEph | 0 | 0 | 0 | 15 | 15 | 0 | 13 | 0 | 2 |
| 104 | Chap37 | BSTSetAVLMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 105 | Chap37 | BSTSetBBAlphaMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 106 | Chap37 | BSTSetPlainMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 107 | Chap37 | BSTSetRBMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 0 | 0 | 22 |
| 108 | Chap37 | BSTSetSplayMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 0 | 0 | 22 |
| 109 | Chap37 | BSTSplayMtEph | 14 | 16 | 0 | 17 | 1 | 32 | 0 | 1 | 32 |
| 110 | Chap37 | BSTSplayStEph | 11 | 12 | 0 | 12 | 24 | 0 | 1 | 6 | 17 |
| 111 | Chap38 | BSTParaMtEph | 16 | 16 | 0 | 16 | 1 | 30 | 0 | 1 | 30 |
| 112 | Chap38 | BSTParaStEph | 12 | 12 | 0 | 10 | 1 | 20 | 0 | 1 | 20 |
| 113 | Chap39 | BSTParaTreapMtEph | 17 | 17 | 0 | 16 | 1 | 32 | 0 | 1 | 32 |
| 114 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 0 | 4 | 24 | 0 | 0 | 12 | 12 |
| 115 | Chap39 | BSTTreapMtEph | 11 | 12 | 0 | 13 | 25 | 0 | 0 | 4 | 21 |
| 116 | Chap39 | BSTTreapStEph | 11 | 12 | 0 | 12 | 24 | 0 | 1 | 7 | 16 |
| 117 | Chap40 | BSTKeyValueStEph | 13 | 14 | 0 | 12 | 26 | 0 | 0 | 8 | 18 |
| 118 | Chap40 | BSTReducedStEph | 18 | 19 | 0 | 16 | 35 | 0 | 0 | 10 | 25 |
| 119 | Chap40 | BSTSizeStEph | 14 | 15 | 0 | 15 | 30 | 0 | 0 | 8 | 22 |
| 120 | Chap41 | AVLTreeSetMtEph | 12 | 13 | 0 | 1 | 13 | 1 | 0 | 13 | 1 |
| 121 | Chap41 | AVLTreeSetMtPer | 12 | 16 | 0 | 0 | 12 | 4 | 0 | 12 | 4 |
| 122 | Chap41 | AVLTreeSetStEph | 12 | 14 | 0 | 0 | 12 | 2 | 0 | 12 | 2 |
| 123 | Chap41 | AVLTreeSetStPer | 12 | 14 | 0 | 0 | 12 | 2 | 0 | 12 | 2 |
| 124 | Chap41 | ArraySetEnumMtEph | 13 | 14 | 0 | 0 | 13 | 1 | 0 | 13 | 1 |
| 125 | Chap41 | ArraySetStEph | 12 | 14 | 0 | 3 | 15 | 2 | 7 | 8 | 2 |
| 126 | Chap41 | Example41_3 | 3 | 3 | 0 | 8 | 9 | 0 | 0 | 4 | 5 |
| 127 | Chap42 | Example42_1 | 2 | 0 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| 128 | Chap42 | TableMtEph | 16 | 17 | 0 | 2 | 18 | 1 | 3 | 14 | 2 |
| 129 | Chap42 | TableStEph | 16 | 18 | 0 | 2 | 18 | 2 | 3 | 14 | 3 |
| 130 | Chap42 | TableStPer | 16 | 17 | 0 | 1 | 0 | 18 | 0 | 0 | 18 |
| 131 | Chap43 | AugOrderedTableMtEph | 32 | 33 | 0 | 3 | 35 | 1 | 29 | 4 | 3 |
| 132 | Chap43 | AugOrderedTableStEph | 31 | 32 | 0 | 2 | 33 | 1 | 29 | 4 | 1 |
| 133 | Chap43 | AugOrderedTableStPer | 28 | 29 | 0 | 2 | 30 | 1 | 27 | 3 | 1 |
| 134 | Chap43 | Example43_1 | 2 | 0 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| 135 | Chap43 | OrderedSetMtEph | 22 | 22 | 0 | 0 | 22 | 0 | 0 | 22 | 0 |
| 136 | Chap43 | OrderedSetStEph | 22 | 24 | 0 | 1 | 23 | 2 | 11 | 12 | 2 |
| 137 | Chap43 | OrderedSetStPer | 22 | 24 | 0 | 1 | 23 | 2 | 13 | 10 | 2 |
| 138 | Chap43 | OrderedTableMtEph | 29 | 30 | 0 | 1 | 30 | 1 | 12 | 16 | 3 |
| 139 | Chap43 | OrderedTableMtPer | 19 | 20 | 0 | 0 | 19 | 1 | 0 | 19 | 1 |
| 140 | Chap43 | OrderedTableStEph | 29 | 30 | 0 | 1 | 30 | 1 | 12 | 16 | 3 |
| 141 | Chap43 | OrderedTableStPer | 26 | 27 | 0 | 1 | 27 | 1 | 0 | 27 | 1 |
| 142 | Chap44 | DocumentIndex | 15 | 16 | 0 | 3 | 2 | 17 | 1 | 0 | 18 |
| 143 | Chap44 | Example44_1 | 0 | 1 | 12 | 9 | 1 | 21 | 0 | 0 | 22 |
| 144 | Chap45 | BalancedTreePQ | 26 | 28 | 0 | 1 | 2 | 27 | 0 | 1 | 28 |
| 145 | Chap45 | BinaryHeapPQ | 18 | 20 | 0 | 9 | 2 | 27 | 0 | 1 | 28 |
| 146 | Chap45 | Example45_2 | 8 | 0 | 0 | 9 | 1 | 8 | 0 | 0 | 9 |
| 147 | Chap45 | HeapsortExample | 22 | 4 | 0 | 21 | 2 | 22 | 0 | 1 | 23 |
| 148 | Chap45 | LeftistHeapPQ | 25 | 25 | 0 | 3 | 1 | 27 | 0 | 0 | 28 |
| 149 | Chap45 | SortedListPQ | 18 | 20 | 0 | 1 | 2 | 19 | 0 | 1 | 20 |
| 150 | Chap45 | UnsortedListPQ | 15 | 17 | 0 | 1 | 2 | 16 | 0 | 1 | 17 |
| 151 | Chap47 | ChainedHashTable | 4 | 1 | 0 | 1 | 2 | 4 | 0 | 1 | 5 |
| 152 | Chap47 | DoubleHashFlatHashTableStEph | 0 | 6 | 1 | 0 | 0 | 7 | 0 | 0 | 7 |
| 153 | Chap47 | FlatHashTable | 4 | 4 | 0 | 0 | 4 | 4 | 0 | 0 | 8 |
| 154 | Chap47 | LinProbFlatHashTableStEph | 0 | 6 | 0 | 0 | 0 | 6 | 0 | 0 | 6 |
| 155 | Chap47 | LinkedListChainedHashTableStEph | 0 | 6 | 0 | 1 | 1 | 6 | 0 | 0 | 7 |
| 156 | Chap47 | ParaHashTableStEph | 8 | 0 | 0 | 0 | 4 | 4 | 0 | 0 | 8 |
| 157 | Chap47 | QuadProbFlatHashTableStEph | 0 | 6 | 0 | 0 | 0 | 6 | 0 | 0 | 6 |
| 158 | Chap47 | StructChainedHashTable | 0 | 7 | 0 | 1 | 1 | 7 | 0 | 0 | 8 |
| 159 | Chap47 | VecChainedHashTableStEph | 0 | 6 | 0 | 1 | 1 | 6 | 0 | 0 | 7 |
| 160 | Chap49 | MinEditDistMtEph | 11 | 12 | 0 | 2 | 1 | 13 | 0 | 1 | 13 |
| 161 | Chap49 | MinEditDistMtPer | 6 | 7 | 0 | 2 | 1 | 8 | 0 | 1 | 8 |
| 162 | Chap49 | MinEditDistStEph | 11 | 11 | 0 | 2 | 1 | 12 | 0 | 0 | 13 |
| 163 | Chap49 | MinEditDistStPer | 6 | 6 | 0 | 2 | 1 | 7 | 0 | 0 | 8 |
| 164 | Chap49 | SubsetSumMtEph | 8 | 9 | 0 | 2 | 1 | 10 | 0 | 1 | 10 |
| 165 | Chap49 | SubsetSumMtPer | 5 | 6 | 0 | 2 | 1 | 7 | 0 | 1 | 7 |
| 166 | Chap49 | SubsetSumStEph | 8 | 8 | 0 | 2 | 1 | 9 | 0 | 0 | 10 |
| 167 | Chap49 | SubsetSumStPer | 5 | 5 | 0 | 2 | 1 | 6 | 0 | 0 | 7 |
| 168 | Chap50 | MatrixChainMtEph | 10 | 11 | 0 | 5 | 3 | 13 | 0 | 3 | 13 |
| 169 | Chap50 | MatrixChainMtPer | 7 | 8 | 0 | 4 | 2 | 10 | 0 | 2 | 10 |
| 170 | Chap50 | MatrixChainStEph | 11 | 12 | 0 | 2 | 1 | 13 | 0 | 1 | 13 |
| 171 | Chap50 | MatrixChainStPer | 7 | 8 | 0 | 2 | 1 | 9 | 0 | 1 | 9 |
| 172 | Chap50 | OptBinSearchTreeMtEph | 10 | 11 | 0 | 4 | 2 | 13 | 0 | 2 | 13 |
| 173 | Chap50 | OptBinSearchTreeMtPer | 7 | 8 | 0 | 3 | 1 | 10 | 0 | 1 | 10 |
| 174 | Chap50 | OptBinSearchTreeStEph | 11 | 11 | 0 | 1 | 0 | 12 | 0 | 0 | 12 |
| 175 | Chap50 | OptBinSearchTreeStPer | 7 | 7 | 0 | 1 | 0 | 8 | 0 | 0 | 8 |
| 176 | Chap50 | Probability | 4 | 14 | 0 | 0 | 14 | 0 | 0 | 14 | 0 |
| 177 | Chap51 | BottomUpDPMtEph | 10 | 12 | 0 | 1 | 1 | 12 | 0 | 1 | 12 |
| 178 | Chap51 | BottomUpDPMtPer | 8 | 10 | 0 | 1 | 1 | 10 | 0 | 1 | 10 |
| 179 | Chap51 | BottomUpDPStEph | 10 | 12 | 0 | 0 | 0 | 12 | 0 | 0 | 12 |
| 180 | Chap51 | BottomUpDPStPer | 8 | 10 | 0 | 0 | 0 | 10 | 0 | 0 | 10 |
| 181 | Chap51 | TopDownDPMtEph | 15 | 17 | 0 | 1 | 1 | 17 | 0 | 1 | 17 |
| 182 | Chap51 | TopDownDPMtPer | 13 | 15 | 0 | 1 | 1 | 15 | 0 | 1 | 15 |
| 183 | Chap51 | TopDownDPStEph | 13 | 14 | 0 | 0 | 0 | 14 | 0 | 0 | 14 |
| 184 | Chap51 | TopDownDPStPer | 11 | 12 | 0 | 0 | 0 | 12 | 0 | 0 | 12 |
| 185 | Chap52 | AdjMatrixGraphMtEph | 9 | 9 | 0 | 3 | 12 | 0 | 12 | 0 | 0 |
| 186 | Chap52 | AdjMatrixGraphMtPer | 7 | 7 | 0 | 3 | 10 | 0 | 10 | 0 | 0 |
| 187 | Chap52 | AdjMatrixGraphStEph | 9 | 9 | 0 | 3 | 12 | 0 | 12 | 0 | 0 |
| 188 | Chap52 | AdjMatrixGraphStPer | 9 | 9 | 0 | 3 | 12 | 0 | 12 | 0 | 0 |
| 189 | Chap52 | AdjSeqGraphMtEph | 7 | 7 | 0 | 2 | 9 | 0 | 9 | 0 | 0 |
| 190 | Chap52 | AdjSeqGraphMtPer | 6 | 6 | 0 | 2 | 8 | 0 | 8 | 0 | 0 |
| 191 | Chap52 | AdjSeqGraphStEph | 9 | 9 | 0 | 2 | 11 | 0 | 11 | 0 | 0 |
| 192 | Chap52 | AdjSeqGraphStPer | 9 | 9 | 0 | 2 | 11 | 0 | 11 | 0 | 0 |
| 193 | Chap52 | AdjTableGraphMtPer | 10 | 11 | 0 | 0 | 11 | 0 | 6 | 1 | 4 |
| 194 | Chap52 | AdjTableGraphStEph | 12 | 12 | 0 | 1 | 13 | 0 | 9 | 0 | 4 |
| 195 | Chap52 | AdjTableGraphStPer | 12 | 12 | 0 | 0 | 12 | 0 | 8 | 0 | 4 |
| 196 | Chap52 | EdgeSetGraphMtPer | 13 | 14 | 0 | 0 | 14 | 0 | 1 | 1 | 12 |
| 197 | Chap52 | EdgeSetGraphStEph | 13 | 13 | 0 | 0 | 13 | 0 | 2 | 0 | 11 |
| 198 | Chap52 | EdgeSetGraphStPer | 13 | 13 | 0 | 0 | 13 | 0 | 2 | 0 | 11 |
| 199 | Chap53 | GraphSearchMtPer | 4 | 1 | 0 | 4 | 5 | 0 | 0 | 4 | 1 |
| 200 | Chap53 | GraphSearchStEph | 4 | 1 | 0 | 4 | 5 | 0 | 0 | 4 | 1 |
| 201 | Chap53 | GraphSearchStPer | 4 | 1 | 0 | 4 | 5 | 0 | 0 | 4 | 1 |
| 202 | Chap53 | PQMinStEph | 4 | 0 | 2 | 4 | 6 | 0 | 0 | 4 | 2 |
| 203 | Chap53 | PQMinStPer | 4 | 0 | 2 | 4 | 6 | 0 | 0 | 4 | 2 |
| 204 | Chap54 | BFSMtEph | 4 | 2 | 0 | 13 | 15 | 0 | 15 | 0 | 0 |
| 205 | Chap54 | BFSMtPer | 4 | 2 | 0 | 13 | 15 | 0 | 15 | 0 | 0 |
| 206 | Chap54 | BFSStEph | 4 | 2 | 0 | 6 | 8 | 0 | 8 | 0 | 0 |
| 207 | Chap54 | BFSStPer | 4 | 2 | 0 | 6 | 8 | 0 | 8 | 0 | 0 |
| 208 | Chap55 | CycleDetectStEph | 1 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 209 | Chap55 | CycleDetectStPer | 1 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 210 | Chap55 | DFSStEph | 1 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 211 | Chap55 | DFSStPer | 1 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 212 | Chap55 | SCCStEph | 1 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |
| 213 | Chap55 | SCCStPer | 1 | 0 | 0 | 6 | 6 | 0 | 6 | 0 | 0 |
| 214 | Chap55 | TopoSortStEph | 1 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |
| 215 | Chap55 | TopoSortStPer | 1 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 216 | Chap56 | AllPairsResultStEphF64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |
| 217 | Chap56 | AllPairsResultStEphI64 | 7 | 7 | 0 | 0 | 7 | 0 | 5 | 0 | 2 |
| 218 | Chap56 | AllPairsResultStPerF64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |
| 219 | Chap56 | AllPairsResultStPerI64 | 7 | 7 | 0 | 0 | 7 | 0 | 5 | 0 | 2 |
| 220 | Chap56 | Example56_1 | 3 | 0 | 0 | 3 | 3 | 0 | 0 | 3 | 0 |
| 221 | Chap56 | Example56_3 | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 2 | 0 |
| 222 | Chap56 | PathWeightUtilsStEph | 4 | 0 | 0 | 5 | 5 | 0 | 0 | 1 | 4 |
| 223 | Chap56 | PathWeightUtilsStPer | 4 | 0 | 0 | 5 | 5 | 0 | 0 | 1 | 4 |
| 224 | Chap56 | SSSPResultStEphF64 | 0 | 0 | 7 | 0 | 6 | 1 | 1 | 0 | 6 |
| 225 | Chap56 | SSSPResultStEphI64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |
| 226 | Chap56 | SSSPResultStPerF64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |
| 227 | Chap56 | SSSPResultStPerI64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |
| 228 | Chap57 | DijkstraStEphF64 | 0 | 3 | 0 | 0 | 0 | 3 | 0 | 0 | 3 |
| 229 | Chap57 | DijkstraStEphI64 | 1 | 2 | 0 | 2 | 4 | 0 | 1 | 1 | 2 |
| 230 | Chap57 | StackStEph | 6 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 231 | Chap58 | BellmanFordStEphI64 | 1 | 0 | 0 | 2 | 1 | 1 | 0 | 0 | 2 |
| 232 | Chap59 | JohnsonMtEphI64 | 1 | 0 | 0 | 5 | 1 | 4 | 0 | 0 | 5 |
| 233 | Chap59 | JohnsonStEphI64 | 1 | 0 | 0 | 4 | 1 | 3 | 0 | 0 | 4 |
| 234 | Chap61 | EdgeContractionMtEph | 2 | 0 | 0 | 3 | 2 | 1 | 0 | 0 | 3 |
| 235 | Chap61 | EdgeContractionStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 236 | Chap61 | VertexMatchingMtEph | 1 | 0 | 0 | 5 | 1 | 4 | 0 | 0 | 5 |
| 237 | Chap61 | VertexMatchingStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 238 | Chap62 | StarContractionMtEph | 2 | 0 | 0 | 4 | 2 | 2 | 0 | 0 | 4 |
| 239 | Chap62 | StarContractionStEph | 2 | 0 | 0 | 3 | 2 | 1 | 0 | 0 | 3 |
| 240 | Chap62 | StarPartitionMtEph | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 241 | Chap62 | StarPartitionStEph | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 242 | Chap63 | ConnectivityMtEph | 4 | 0 | 0 | 7 | 4 | 3 | 0 | 0 | 7 |
| 243 | Chap63 | ConnectivityStEph | 4 | 0 | 0 | 5 | 4 | 1 | 0 | 0 | 5 |
| 244 | Chap64 | SpanTreeMtEph | 2 | 0 | 0 | 4 | 4 | 0 | 0 | 2 | 2 |
| 245 | Chap64 | SpanTreeStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 246 | Chap64 | TSPApproxStEph | 4 | 0 | 0 | 7 | 0 | 7 | 0 | 0 | 7 |
| 247 | Chap65 | KruskalStEph | 3 | 0 | 0 | 3 | 3 | 0 | 0 | 0 | 3 |
| 248 | Chap65 | PrimStEph | 2 | 2 | 0 | 3 | 2 | 3 | 0 | 0 | 5 |
| 249 | Chap65 | UnionFindStEph | 6 | 7 | 0 | 0 | 6 | 1 | 0 | 0 | 7 |
| 250 | Chap66 | BoruvkaMtEph | 5 | 0 | 0 | 7 | 5 | 2 | 0 | 0 | 7 |
| 251 | Chap66 | BoruvkaStEph | 5 | 0 | 0 | 5 | 5 | 0 | 0 | 0 | 5 |
| 252 | arithmetic | power2_plus | 0 | 0 | 0 | 6 | 6 | 0 | 6 | 0 | 0 |
| 253 | experiments | ArrayVal | 0 | 0 | 0 | 4 | 3 | 1 | 3 | 0 | 1 |
| 254 | experiments | ArrayVecSet | 0 | 0 | 0 | 4 | 3 | 1 | 0 | 1 | 3 |
| 255 | experiments | CheckedI32 | 0 | 0 | 12 | 0 | 12 | 0 | 6 | 6 | 0 |
| 256 | experiments | ForFor | 0 | 0 | 0 | 4 | 3 | 1 | 2 | 1 | 1 |
| 257 | experiments | ForLoops | 0 | 0 | 0 | 3 | 2 | 1 | 0 | 0 | 3 |
| 258 | experiments | HashCheckedU32 | 0 | 2 | 0 | 2 | 4 | 0 | 0 | 4 | 0 |
| 259 | experiments | SetLoops | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 260 | experiments | ToVecProof | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 261 | experiments | VSTDLoopProofs | 0 | 0 | 0 | 9 | 9 | 0 | 7 | 2 | 0 |
| 262 | experiments | WhileWhile | 0 | 0 | 0 | 4 | 3 | 1 | 2 | 1 | 1 |
| 263 | experiments | abstract_set_iter | 5 | 5 | 1 | 1 | 7 | 0 | 6 | 0 | 1 |
| 264 | experiments | accept | 0 | 1 | 0 | 2 | 3 | 0 | 0 | 2 | 1 |
| 265 | experiments | accept_external_body | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 266 | experiments | arc_clone_deref | 0 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 267 | experiments | arc_rwlock_ninject | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 268 | experiments | assume_spec_test | 0 | 1 | 0 | 1 | 2 | 0 | 0 | 0 | 2 |
| 269 | experiments | baseviewtypes | 0 | 0 | 0 | 7 | 7 | 0 | 1 | 0 | 6 |
| 270 | experiments | biconditional_spec_fun | 0 | 0 | 0 | 10 | 10 | 0 | 1 | 0 | 9 |
| 271 | experiments | boxing_fns | 0 | 0 | 0 | 12 | 12 | 0 | 5 | 0 | 7 |
| 272 | experiments | checked_signed_int | 16 | 0 | 0 | 0 | 16 | 0 | 16 | 0 | 0 |
| 273 | experiments | checked_u32 | 0 | 8 | 3 | 0 | 11 | 0 | 6 | 3 | 2 |
| 274 | experiments | checked_unsigned_int | 15 | 0 | 0 | 0 | 15 | 0 | 15 | 0 | 0 |
| 275 | experiments | clone | 0 | 0 | 0 | 11 | 11 | 0 | 4 | 6 | 1 |
| 276 | experiments | clone_fn | 0 | 0 | 0 | 5 | 5 | 0 | 0 | 5 | 0 |
| 277 | experiments | clone_plus | 0 | 0 | 0 | 4 | 4 | 0 | 2 | 1 | 1 |
| 278 | experiments | clone_plus_vs_deep_clone | 0 | 0 | 0 | 3 | 3 | 0 | 1 | 0 | 2 |
| 279 | experiments | collect | 0 | 0 | 0 | 7 | 7 | 0 | 7 | 0 | 0 |
| 280 | experiments | collect2 | 0 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |
| 281 | experiments | collect_deep_view | 0 | 0 | 0 | 11 | 11 | 0 | 10 | 1 | 0 |
| 282 | experiments | deep_view_2_tuple | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 283 | experiments | deep_view_struct | 1 | 1 | 0 | 1 | 2 | 0 | 1 | 0 | 1 |
| 284 | experiments | eq_rel | 9 | 9 | 0 | 0 | 9 | 0 | 5 | 1 | 3 |
| 285 | experiments | executable_use_of_int | 0 | 0 | 0 | 2 | 2 | 0 | 0 | 1 | 1 |
| 286 | experiments | f64_bits_sort | 0 | 0 | 0 | 5 | 5 | 0 | 2 | 3 | 0 |
| 287 | experiments | f64_float_cmp_sort | 0 | 0 | 0 | 4 | 4 | 0 | 0 | 4 | 0 |
| 288 | experiments | f64_sort | 0 | 0 | 0 | 4 | 4 | 0 | 1 | 3 | 0 |
| 289 | experiments | ghost_type_invariant | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 290 | experiments | hash_set_iter | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 291 | experiments | hash_set_modern_pattern | 0 | 0 | 0 | 5 | 5 | 0 | 0 | 1 | 4 |
| 292 | experiments | hash_set_with_view_plus_loops | 0 | 0 | 0 | 7 | 7 | 0 | 4 | 1 | 2 |
| 293 | experiments | invariant_proof_test | 0 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 294 | experiments | minimal_iter | 2 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 2 |
| 295 | experiments | modify_a_ghost_struct | 0 | 0 | 0 | 6 | 6 | 0 | 0 | 0 | 6 |
| 296 | experiments | mut_refs_and_mut_returns | 0 | 0 | 3 | 2 | 5 | 0 | 3 | 0 | 2 |
| 297 | experiments | parapair_closure_ensures | 0 | 0 | 0 | 3 | 3 | 0 | 2 | 0 | 1 |
| 298 | experiments | parapair_move_closure_ensures | 0 | 0 | 0 | 3 | 3 | 0 | 2 | 0 | 1 |
| 299 | experiments | parapair_named_closure | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 300 | experiments | parapair_toplevel_closure | 0 | 0 | 0 | 3 | 3 | 0 | 2 | 0 | 1 |
| 301 | experiments | pervasives | 0 | 0 | 0 | 8 | 8 | 0 | 3 | 3 | 2 |
| 302 | experiments | possession | 2 | 2 | 0 | 0 | 2 | 0 | 2 | 0 | 0 |
| 303 | experiments | proof_fn_in_trait | 2 | 2 | 0 | 1 | 3 | 0 | 2 | 0 | 1 |
| 304 | experiments | proven_partialeq | 5 | 5 | 0 | 2 | 7 | 0 | 7 | 0 | 0 |
| 305 | experiments | pub_crate_test | 0 | 0 | 1 | 3 | 0 | 4 | 0 | 0 | 4 |
| 306 | experiments | seq_array_equality | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 307 | experiments | seq_for_basic_proofs | 0 | 0 | 0 | 27 | 27 | 0 | 25 | 2 | 0 |
| 308 | experiments | seq_loop_basic_proofs | 0 | 0 | 0 | 27 | 27 | 0 | 25 | 2 | 0 |
| 309 | experiments | seq_set_exec | 0 | 0 | 0 | 6 | 6 | 0 | 4 | 2 | 0 |
| 310 | experiments | seq_vec_equality | 0 | 0 | 0 | 2 | 2 | 0 | 1 | 1 | 0 |
| 311 | experiments | seq_while_basic_proofs | 0 | 0 | 0 | 27 | 27 | 0 | 25 | 2 | 0 |
| 312 | experiments | set_len_empty_both_ways | 0 | 0 | 0 | 4 | 4 | 0 | 3 | 0 | 1 |
| 313 | experiments | sigma_pi | 4 | 4 | 0 | 0 | 4 | 0 | 1 | 3 | 0 |
| 314 | experiments | signed_int | 6 | 0 | 0 | 0 | 6 | 0 | 6 | 0 | 0 |
| 315 | experiments | simple_hash_set_iter | 5 | 5 | 0 | 1 | 6 | 0 | 3 | 2 | 1 |
| 316 | experiments | simple_seq_iter | 4 | 5 | 0 | 5 | 10 | 0 | 8 | 1 | 1 |
| 317 | experiments | simple_set_iter | 5 | 6 | 0 | 4 | 10 | 0 | 7 | 2 | 1 |
| 318 | experiments | spec_fun_argument | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 0 |
| 319 | experiments | spec_loop | 2 | 2 | 0 | 0 | 2 | 0 | 2 | 0 | 0 |
| 320 | experiments | struct_construction_test | 0 | 0 | 4 | 3 | 0 | 7 | 0 | 0 | 7 |
| 321 | experiments | supertrait | 2 | 2 | 0 | 0 | 2 | 0 | 2 | 0 | 0 |
| 322 | experiments | tcb_foul | 0 | 0 | 0 | 5 | 5 | 0 | 0 | 3 | 2 |
| 323 | experiments | test_feq | 0 | 0 | 0 | 35 | 35 | 0 | 20 | 4 | 11 |
| 324 | experiments | test_feq_insertion_sort | 0 | 0 | 0 | 5 | 5 | 0 | 2 | 3 | 0 |
| 325 | experiments | test_test | 0 | 0 | 0 | 3 | 3 | 0 | 3 | 0 | 0 |
| 326 | experiments | total_ord_gen | 10 | 10 | 0 | 4 | 14 | 0 | 1 | 9 | 4 |
| 327 | experiments | total_ord_gen_axioms | 0 | 0 | 0 | 11 | 11 | 0 | 0 | 7 | 4 |
| 328 | experiments | trait_decreases | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |
| 329 | experiments | triangle | 0 | 0 | 0 | 3 | 3 | 0 | 3 | 0 | 0 |
| 330 | experiments | unsigned_int | 7 | 0 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 331 | experiments | use_proven_partialeq | 0 | 0 | 1 | 5 | 6 | 0 | 5 | 0 | 1 |
| 332 | experiments | vec_clone_in_verus | 0 | 0 | 1 | 1 | 2 | 0 | 1 | 0 | 1 |
| 333 | experiments | vec_filter | 0 | 0 | 0 | 3 | 3 | 0 | 2 | 1 | 0 |
| 334 | experiments | vec_if | 0 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 335 | experiments | vec_length_while_rust | 0 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 2 |
| 336 | experiments | vec_length_while_verus | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 337 | experiments | vec_remove_duplicates | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 338 | experiments | verus_iterator | 2 | 0 | 0 | 0 | 2 | 0 | 2 | 0 | 0 |
| 339 | experiments | verus_keep_ghost_and_test | 0 | 0 | 6 | 0 | 6 | 0 | 6 | 0 | 0 |
| 340 | experiments | verus_pub_crate_test | 0 | 0 | 1 | 1 | 2 | 0 | 1 | 0 | 1 |
| 341 | experiments | verus_sum_loops_iterators | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 342 | experiments | verus_vec_iterator | 2 | 2 | 0 | 0 | 2 | 0 | 2 | 0 | 0 |
| 343 | experiments | verus_vec_iterator_for_basic_proofs | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 344 | experiments | verus_vec_iterator_loop_basic_proofs | 0 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 345 | experiments | verus_vec_iterator_while_basic_proofs | 0 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 346 | experiments | verus_wrapped_iter_loops | 0 | 0 | 0 | 7 | 7 | 0 | 5 | 2 | 0 |
| 347 | src | Concurrency | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 348 | src | ParaPairs | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 349 | src | Types | 0 | 4 | 0 | 14 | 14 | 4 | 2 | 12 | 4 |
| 350 | vstdplus | VecQueue | 5 | 5 | 0 | 0 | 5 | 0 | 5 | 0 | 0 |
| 351 | vstdplus | accept | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 0 |
| 352 | vstdplus | checked_int | 12 | 0 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 353 | vstdplus | checked_nat | 11 | 0 | 0 | 0 | 11 | 0 | 11 | 0 | 0 |
| 354 | vstdplus | checked_nat_with_checked_view | 4 | 0 | 0 | 0 | 4 | 0 | 4 | 0 | 0 |
| 355 | vstdplus | clone_plus | 1 | 1 | 0 | 3 | 4 | 0 | 0 | 4 | 0 |
| 356 | vstdplus | feq | 0 | 0 | 0 | 6 | 5 | 1 | 2 | 3 | 1 |
| 357 | vstdplus | float | 5 | 11 | 6 | 20 | 31 | 5 | 21 | 10 | 5 |
| 358 | vstdplus | hash_set_with_view_plus | 1 | 3 | 5 | 1 | 9 | 0 | 0 | 9 | 0 |
| 359 | vstdplus | hashed_checked_u32 | 0 | 2 | 0 | 2 | 4 | 0 | 0 | 4 | 0 |
| 360 | vstdplus | multiset | 0 | 0 | 0 | 6 | 6 | 0 | 6 | 0 | 0 |
| 361 | vstdplus | partial_order | 4 | 4 | 0 | 0 | 4 | 0 | 0 | 4 | 0 |
| 362 | vstdplus | rand | 0 | 0 | 0 | 2 | 1 | 1 | 0 | 1 | 1 |
| 363 | vstdplus | seq | 0 | 0 | 0 | 10 | 10 | 0 | 9 | 1 | 0 |
| 364 | vstdplus | seq_set | 0 | 0 | 0 | 60 | 60 | 0 | 60 | 0 | 0 |
| 365 | vstdplus | smart_ptrs | 0 | 0 | 0 | 3 | 3 | 0 | 1 | 2 | 0 |
| 366 | vstdplus | threads_plus | 0 | 0 | 7 | 3 | 10 | 0 | 5 | 4 | 1 |
| 367 | vstdplus | total_order | 5 | 5 | 0 | 0 | 5 | 0 | 5 | 0 | 0 |

## Function-by-Function Detail

### Chap02/FibonacciHFScheduler.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_pow2_mono` |  |  |  | Y | Y |  |  | unknown | 23&#8209;25 |
| 2 | `lemma_pow2_46_lt_u64_max` |  |  |  | Y | Y |  |  | unknown | 30&#8209;31 |
| 3 | `lemma_fib_bound` |  |  |  | Y | Y |  |  | unknown | 36&#8209;38 |
| 4 | `lemma_fib_fits_u64` |  |  |  | Y | Y |  |  | unknown | 53&#8209;55 |
| 5 | `lemma_fib_sum_fits_u64` |  |  |  | Y | Y |  |  | unknown | 62&#8209;64 |
| 6 | `fib_seq` |  |  |  | Y | Y |  |  | unknown | 71&#8209;74 |
| 7 | `fib_par` |  |  |  | Y | Y |  |  | unknown | 86&#8209;89 |

### Chap02/HFSchedulerMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `set_parallelism` |  |  |  | Y | Y |  |  | hole | 85 |
| 9 | `join` |  |  |  | Y | Y |  |  | hole | 95&#8209;106 |
| 10 | `spawn_join` |  |  |  | Y | Y |  |  | hole | 122&#8209;133 |
| 11 | `spawn` |  |  |  | Y | Y |  |  | hole | 153&#8209;160 |
| 12 | `wait` |  |  |  | Y | Y |  |  | hole | 175&#8209;177 |
| 13 | `try_acquire` |  |  |  | Y |  | Y | Y |  | 46&#8209;54 |
| 14 | `acquire` |  |  |  | Y |  | Y | Y |  | 56&#8209;62 |
| 15 | `release` |  |  |  | Y |  | Y | Y |  | 64&#8209;68 |

### Chap03/InsertionSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `insertion_sort` |  |  |  | Y | Y |  |  | unknown | 34&#8209;38 |

### Chap05/KleeneStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `lemma_star_closed_under_concat` |  |  |  | Y | Y |  |  | unknown | 78&#8209;83 |
| 18 | `lemma_plus_closed_under_concat` |  |  |  | Y | Y |  |  | unknown | 96&#8209;101 |
| 19 | `ptt_star_contains_empty` |  |  |  | Y | Y |  |  | unknown | 108&#8209;109 |
| 20 | `ptt_plus_rejects_empty` |  |  |  | Y | Y |  |  | unknown | 114&#8209;115 |
| 21 | `ptt_singleton_in_star_and_plus` |  |  |  | Y | Y |  |  | unknown | 120&#8209;124 |
| 22 | `ptt_plus_subset_of_star` |  |  |  | Y | Y |  |  | unknown | 129&#8209;131 |
| 23 | `ptt_star_property_transfer` |  |  |  | Y | Y |  |  | unknown | 137&#8209;146 |
| 24 | `ptt_star_concat_plus_is_plus` |  |  |  | Y | Y |  |  | unknown | 152&#8209;157 |
| 25 | `ptt_plus_concat_star_is_plus` |  |  |  | Y | Y |  |  | unknown | 166&#8209;171 |
| 26 | `new` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 27 | `mem_star` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;196 |
| 28 | `mem_plus` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;203 |
| 29 | `alphabet` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;209 |

### Chap05/MappingStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `is_functional_vec` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 31 | `is_functional_vec_at` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 32 | `is_functional_SetStEph_at` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 33 | `is_functional_SetStEph` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 34 | `is_functional_RelationStEph` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 35 | `empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;172 |
| 36 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 37 | `from_relation` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;184 |
| 38 | `size` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;189 |
| 39 | `domain` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;195 |
| 40 | `range` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;205 |
| 41 | `mem` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;214 |
| 42 | `iter` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;224 |
| 43 | `next` |  | Y |  |  | Y |  |  | unknown | 449&#8209;465 |
| 44 | `hash` |  | Y |  |  | Y |  | Y |  | 560 |
| 45 | `eq` |  | Y |  |  | Y |  |  | hole | 566&#8209;567 |

### Chap05/RelationStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 46 | `empty` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 47 | `from_set` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 48 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 49 | `size` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 50 | `domain` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 51 | `range` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 52 | `mem` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 53 | `relates` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;142 |
| 54 | `iter` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;151 |
| 55 | `next` |  | Y |  |  | Y |  |  | unknown | 287&#8209;303 |
| 56 | `hash` |  | Y |  |  | Y |  | Y |  | 397 |
| 57 | `eq` |  | Y |  |  | Y |  |  | unknown | 403&#8209;404 |

### Chap05/SetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 58 | `lemma_singleton_choose` |  |  |  | Y | Y |  |  | unknown | 103&#8209;109 |
| 59 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;132 |
| 60 | `iter` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;141 |
| 61 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;149 |
| 62 | `empty` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 63 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 64 | `size` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;166 |
| 65 | `mem` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;172 |
| 66 | `insert` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;180 |
| 67 | `union` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;187 |
| 68 | `disjoint_union` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;199 |
| 69 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;205 |
| 70 | `elt_cross_set` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;216 |
| 71 | `cartesian_product` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;228 |
| 72 | `all_nonempty` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;237 |
| 73 | `partition_on_elt` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;252 |
| 74 | `partition` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;270 |
| 75 | `choose` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;280 |
| 76 | `next` |  | Y |  |  | Y |  |  | unknown | 850&#8209;866 |
| 77 | `hash` |  | Y |  |  | Y |  | Y |  | 959 |
| 78 | `eq` |  | Y |  |  | Y |  |  | hole | 965&#8209;966 |

### Chap05/SetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 79 | `lemma_singleton_choose` |  |  |  | Y | Y |  |  | unknown | 99&#8209;105 |
| 80 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 81 | `iter` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;137 |
| 82 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;145 |
| 83 | `empty` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 84 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 85 | `size` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;162 |
| 86 | `mem` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 87 | `insert` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;176 |
| 88 | `union` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;183 |
| 89 | `disjoint_union` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;195 |
| 90 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;201 |
| 91 | `elt_cross_set` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;212 |
| 92 | `cartesian_product` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;223 |
| 93 | `all_nonempty` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;232 |
| 94 | `partition_on_elt` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;247 |
| 95 | `partition` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;265 |
| 96 | `split` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;282 |
| 97 | `choose` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;293 |
| 98 | `next` |  | Y |  |  | Y |  |  | unknown | 772&#8209;788 |
| 99 | `hash` |  | Y |  |  | Y |  | Y |  | 867 |
| 100 | `eq` |  | Y |  |  | Y |  |  | hole | 873&#8209;874 |

### Chap06/DirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 101 | `empty` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;101 |
| 102 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;115 |
| 103 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 104 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 105 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 106 | `sizeA` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;135 |
| 107 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;145 |
| 108 | `incident` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 109 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;176 |
| 110 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;185 |
| 111 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;210 |
| 112 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;219 |
| 113 | `ng` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;236 |
| 114 | `degree` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;251 |
| 115 | `n_plus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;274 |
| 116 | `n_minus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;297 |
| 117 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;320 |
| 118 | `n_plus_par` |  |  |  | Y | Y |  |  | unknown | 327&#8209;337 |
| 119 | `n_minus_par` |  |  |  | Y | Y |  |  | unknown | 374&#8209;384 |
| 120 | `n_plus_of_vertices_par` |  |  |  | Y | Y |  |  | unknown | 421&#8209;433 |
| 121 | `n_minus_of_vertices_par` |  |  |  | Y | Y |  |  | unknown | 488&#8209;500 |
| 122 | `ng_of_vertices_par` |  |  |  | Y | Y |  |  | unknown | 555&#8209;567 |
| 123 | `eq` |  | Y |  |  | Y |  |  | unknown | 695&#8209;696 |

### Chap06/DirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 124 | `empty` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;117 |
| 125 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;129 |
| 126 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 127 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;139 |
| 128 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 129 | `sizeA` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 130 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 131 | `ng` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 132 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;169 |
| 133 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;175 |
| 134 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;181 |
| 135 | `n_plus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;187 |
| 136 | `n_minus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;193 |
| 137 | `incident` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;199 |
| 138 | `degree` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;205 |
| 139 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;211 |
| 140 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;217 |
| 141 | `iter_vertices` |  |  | Y |  | Y |  |  | unknown | 225&#8209;226 |
| 142 | `iter_arcs` |  |  | Y |  | Y |  |  | unknown | 230&#8209;231 |
| 143 | `eq` |  | Y |  |  | Y |  |  | unknown | 590&#8209;591 |

### Chap06/LabDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 144 | `empty` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 145 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;114 |
| 146 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 147 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 148 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;131 |
| 149 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 150 | `add_labeled_arc` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;146 |
| 151 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;155 |
| 152 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 153 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;201 |
| 154 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;213 |
| 155 | `n_plus_par` |  |  |  | Y | Y |  |  | unknown | 220&#8209;233 |
| 156 | `n_minus_par` |  |  |  | Y | Y |  |  | unknown | 335&#8209;348 |

### Chap06/LabDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 157 | `empty` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;75 |
| 158 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;87 |
| 159 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 160 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 161 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 162 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 163 | `add_labeled_arc` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;117 |
| 164 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;125 |
| 165 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 166 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 167 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |

### Chap06/LabUnDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 168 | `empty` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;108 |
| 169 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;122 |
| 170 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;127 |
| 171 | `labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 172 | `edges` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;139 |
| 173 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 174 | `add_labeled_edge` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 175 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;160 |
| 176 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;169 |
| 177 | `ng` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;186 |
| 178 | `ng_par` |  |  |  | Y | Y |  |  | unknown | 193&#8209;206 |

### Chap06/LabUnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 179 | `empty` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;69 |
| 180 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;81 |
| 181 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 182 | `labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 183 | `edges` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;98 |
| 184 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 185 | `add_labeled_edge` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;113 |
| 186 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;123 |
| 187 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;130 |
| 188 | `ng` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |

### Chap06/UnDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 189 | `empty` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;113 |
| 190 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;127 |
| 191 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 192 | `edges` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 193 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;142 |
| 194 | `sizeE` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 195 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;157 |
| 196 | `ng` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;174 |
| 197 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;191 |
| 198 | `incident` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;197 |
| 199 | `degree` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;206 |
| 200 | `ng_par` |  |  |  | Y | Y |  |  | unknown | 213&#8209;223 |
| 201 | `ng_of_vertices_par` |  |  |  | Y | Y |  |  | unknown | 310&#8209;322 |
| 202 | `eq` |  | Y |  |  | Y |  |  | unknown | 478&#8209;479 |

### Chap06/UnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 203 | `empty` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;98 |
| 204 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;108 |
| 205 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 206 | `edges` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 207 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 208 | `sizeE` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 209 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;139 |
| 210 | `ng` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;149 |
| 211 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;159 |
| 212 | `incident` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;164 |
| 213 | `degree` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;172 |
| 214 | `eq` |  | Y |  |  | Y |  |  | unknown | 356&#8209;357 |

### Chap06/WeightedDirGraphStEphI128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 215 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 216 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 217 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 218 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 219 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 220 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 221 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 222 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 223 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphI16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 224 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 225 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 226 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 227 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 228 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 229 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 230 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 231 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 232 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphI32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 233 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 234 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 235 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 236 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 237 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 238 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 239 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 240 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 241 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 242 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 243 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 244 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 245 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 246 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 247 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 248 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 249 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 250 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphI8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 251 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 252 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 253 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 254 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 255 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 256 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 257 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 258 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 259 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphIsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 260 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 261 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 262 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 263 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 264 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 265 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 266 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 267 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 268 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphU128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 269 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 270 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 271 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 272 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 273 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 274 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 275 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 276 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 277 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphU16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 278 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 279 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 280 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 281 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 282 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 283 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 284 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 285 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 286 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphU32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 287 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 288 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 289 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 290 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 291 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 292 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 293 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 294 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 295 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphU64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 296 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 297 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 298 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 299 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 300 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 301 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 302 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 303 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 304 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphU8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 305 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 306 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 307 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 308 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 309 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 310 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 311 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 312 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 313 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphUsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 314 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 315 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 316 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 317 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 318 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 319 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 320 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 321 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 322 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap11/FibonacciMtEph2Threads.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 323 | `fib_2threads` |  |  |  | Y | Y |  |  | hole | 100&#8209;102 |

### Chap11/FibonacciMtEphRecomputes.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 324 | `fib_recomputes` |  |  |  | Y | Y |  |  | hole | 92&#8209;95 |

### Chap11/FibonacciMtPerAllThreads.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 325 | `fib` |  |  |  | Y | Y |  |  | unknown | 21&#8209;26 |

### Chap11/FibonacciMtPerTSM.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 326 | `fib` |  |  |  | Y | Y |  |  | hole | 86&#8209;89 |

### Chap11/FibonacciStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 327 | `lemma_fib_bound` |  |  |  | Y | Y |  |  | unknown | 48&#8209;50 |
| 328 | `lemma_fib_fits_u64` |  |  |  | Y | Y |  |  | unknown | 66&#8209;68 |
| 329 | `lemma_fib_sum_fits_u64` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 330 | `fib` |  |  |  | Y | Y |  |  | unknown | 89&#8209;93 |
| 331 | `fib_recursive` |  |  |  | Y | Y |  |  | unknown | 127&#8209;132 |

### Chap12/Exercise12_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 332 | `new` | Y | Y |  |  | Y |  |  | hole | 32&#8209;33 |
| 333 | `lock` | Y | Y |  |  | Y |  |  | hole | 38&#8209;39 |
| 334 | `unlock` | Y | Y |  |  | Y |  |  | hole | 44&#8209;46 |
| 335 | `with_lock` | Y | Y |  |  | Y |  |  | hole | 53 |
| 336 | `parallel_increment` |  |  |  | Y | Y |  |  | hole | 93&#8209;94 |
| 337 | `default` |  | Y |  |  | Y |  | Y |  | 120 |

### Chap12/Exercise12_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 338 | `fetch_add_cas` | Y | Y |  |  | Y |  | Y |  | 22 |

### Chap12/Exercise12_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 339 | `new` | Y | Y |  |  | Y |  |  | hole | 58&#8209;59 |
| 340 | `push` | Y | Y |  |  | Y |  |  | hole | 64&#8209;65 |
| 341 | `pop` | Y | Y |  |  | Y |  |  | hole | 72&#8209;73 |
| 342 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 343 | `drain` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 344 | `default` |  | Y |  |  | Y |  | Y |  | 148 |
| 345 | `drop` |  | Y |  |  | Y |  |  | hole | 155&#8209;157 |

### Chap17/MathSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 346 | `new` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;144 |
| 347 | `set` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;154 |
| 348 | `length` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;159 |
| 349 | `nth` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;165 |
| 350 | `empty` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;170 |
| 351 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;177 |
| 352 | `add_last` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;185 |
| 353 | `delete_last` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;195 |
| 354 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;200 |
| 355 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;205 |
| 356 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;210 |
| 357 | `with_len` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;217 |
| 358 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;228 |
| 359 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;237 |
| 360 | `domain` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;242 |
| 361 | `range` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;250 |
| 362 | `multiset_range` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;259 |
| 363 | `iter` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;268 |
| 364 | `next` |  | Y |  |  | Y |  |  | unknown | 617&#8209;633 |
| 365 | `eq` |  | Y |  |  | Y |  |  | hole | 724&#8209;725 |
| 366 | `iter_mut` |  |  | Y |  |  | Y | Y |  | 738&#8209;743 |

### Chap18/ArraySeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 367 | `lemma_deep_view_len` |  |  |  | Y | Y |  |  | unknown | 177&#8209;179 |
| 368 | `lemma_deep_view_key` |  |  |  | Y | Y |  |  | unknown | 184&#8209;189 |
| 369 | `lemma_find_key_index_bounds` |  |  |  | Y | Y |  |  | unknown | 194&#8209;199 |
| 370 | `lemma_find_key_index_found` |  |  |  | Y | Y |  |  | unknown | 209&#8209;220 |
| 371 | `lemma_find_key_index_not_found` |  |  |  | Y | Y |  |  | unknown | 229&#8209;237 |
| 372 | `lemma_spec_collect_step_some` |  |  |  | Y | Y |  |  | unknown | 246&#8209;258 |
| 373 | `lemma_spec_collect_step_none` |  |  |  | Y | Y |  |  | unknown | 269&#8209;280 |
| 374 | `lemma_find_key_some` |  |  |  | Y | Y |  |  | unknown | 290&#8209;297 |
| 375 | `lemma_find_key_none` |  |  |  | Y | Y |  |  | unknown | 308&#8209;313 |
| 376 | `new` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;345 |
| 377 | `set` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;355 |
| 378 | `length` | Y | Y |  |  | Y |  |  | unknown | 360&#8209;361 |
| 379 | `nth` | Y | Y |  |  | Y |  |  | unknown | 366&#8209;368 |
| 380 | `empty` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;374 |
| 381 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;382 |
| 382 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 387&#8209;395 |
| 383 | `append` | Y | Y |  |  | Y |  |  | unknown | 400&#8209;408 |
| 384 | `filter` | Y | Y |  |  | Y |  |  | unknown | 415&#8209;430 |
| 385 | `update` | Y | Y |  |  | Y |  |  | unknown | 435&#8209;443 |
| 386 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 448&#8209;449 |
| 387 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 454&#8209;455 |
| 388 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 461&#8209;466 |
| 389 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 472&#8209;480 |
| 390 | `scan` | Y | Y |  |  | Y |  |  | unknown | 486&#8209;500 |
| 391 | `inject` | Y | Y |  |  | Y |  |  | unknown | 506&#8209;515 |
| 392 | `scan_inclusive` | Y | Y |  |  | Y |  |  | unknown | 521&#8209;531 |
| 393 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 536&#8209;544 |
| 394 | `remove` | Y | Y |  |  | Y |  |  | unknown | 549&#8209;556 |
| 395 | `insert` | Y | Y |  |  | Y |  |  | unknown | 561&#8209;568 |
| 396 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 573&#8209;576 |
| 397 | `find_key` | Y | Y |  |  | Y |  |  | unknown | 579&#8209;591 |
| 398 | `collect` | Y | Y |  |  | Y |  |  | unknown | 597&#8209;609 |
| 399 | `map` |  |  |  | Y | Y |  |  | unknown | 1272&#8209;1276 |
| 400 | `tabulate` |  |  |  | Y | Y |  |  | unknown | 1303&#8209;1309 |
| 401 | `flatten` |  |  |  | Y | Y |  |  | unknown | 1330&#8209;1334 |
| 402 | `iterate_prefixes` |  |  |  | Y | Y |  |  | unknown | 1388&#8209;1403 |
| 403 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1465&#8209;1467 |
| 404 | `iter` |  |  | Y |  | Y |  |  | unknown | 1471&#8209;1475 |
| 405 | `iter_mut` |  |  | Y |  | Y |  | Y |  | 1483 |
| 406 | `next` |  | Y |  |  | Y |  |  | unknown | 1535&#8209;1551 |
| 407 | `eq` |  | Y |  |  | Y |  |  | hole | 1642&#8209;1643 |

### Chap18/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 408 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 120&#8209;129 |
| 409 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 160&#8209;162 |
| 410 | `apply_ninject_updates` |  |  |  | Y | Y |  |  | unknown | 194&#8209;202 |
| 411 | `new` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;272 |
| 412 | `set` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;282 |
| 413 | `length` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;288 |
| 414 | `nth` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;295 |
| 415 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;308 |
| 416 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;321 |
| 417 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;329 |
| 418 | `empty` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;339 |
| 419 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 344&#8209;347 |
| 420 | `append` | Y | Y |  |  | Y |  |  | unknown | 352&#8209;360 |
| 421 | `filter` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;381 |
| 422 | `update` | Y | Y |  |  | Y |  |  | unknown | 386&#8209;394 |
| 423 | `inject` | Y | Y |  |  | Y |  |  | unknown | 400&#8209;409 |
| 424 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 415&#8209;423 |
| 425 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 428&#8209;429 |
| 426 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 434&#8209;435 |
| 427 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 440&#8209;445 |
| 428 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 450&#8209;458 |
| 429 | `scan` | Y | Y |  |  | Y |  |  | unknown | 463&#8209;475 |
| 430 | `map` | Y | Y |  |  | Y |  |  | unknown | 480&#8209;485 |
| 431 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 490&#8209;496 |
| 432 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 501&#8209;506 |
| 433 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1069&#8209;1071 |
| 434 | `iter` |  |  | Y |  | Y |  |  | unknown | 1075&#8209;1079 |
| 435 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1086&#8209;1097 |
| 436 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1138&#8209;1148 |
| 437 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1192&#8209;1195 |
| 438 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1220&#8209;1235 |
| 439 | `ninject_par` |  |  | Y |  | Y |  |  | unknown | 1308&#8209;1318 |
| 440 | `next` |  | Y |  |  | Y |  |  | unknown | 1445&#8209;1461 |
| 441 | `eq` |  | Y |  |  | Y |  |  | hole | 1564&#8209;1565 |

### Chap18/ArraySeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 442 | `new` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;108 |
| 443 | `length` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 444 | `nth` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 445 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;134 |
| 446 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;147 |
| 447 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;155 |
| 448 | `empty` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 449 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;173 |
| 450 | `append` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;186 |
| 451 | `filter` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;207 |
| 452 | `update` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;220 |
| 453 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;226 |
| 454 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;232 |
| 455 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;242 |
| 456 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;255 |
| 457 | `scan` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;272 |
| 458 | `map` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;282 |
| 459 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;293 |
| 460 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;303 |
| 461 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 761&#8209;763 |
| 462 | `iter` |  |  | Y |  | Y |  |  | unknown | 767&#8209;771 |
| 463 | `map_par` |  |  | Y |  | Y |  |  | unknown | 779&#8209;789 |
| 464 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 837&#8209;846 |
| 465 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 894&#8209;897 |
| 466 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 934&#8209;949 |
| 467 | `next` |  | Y |  |  | Y |  |  | unknown | 1053&#8209;1069 |
| 468 | `eq` |  | Y |  |  | Y |  |  | hole | 1172&#8209;1173 |

### Chap18/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 469 | `new` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;117 |
| 470 | `set` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;127 |
| 471 | `length` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;133 |
| 472 | `nth` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;140 |
| 473 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;153 |
| 474 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;166 |
| 475 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;174 |
| 476 | `empty` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;184 |
| 477 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;192 |
| 478 | `append` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;205 |
| 479 | `filter` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;226 |
| 480 | `update` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;239 |
| 481 | `inject` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;254 |
| 482 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;260 |
| 483 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;266 |
| 484 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;276 |
| 485 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;289 |
| 486 | `scan` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;306 |
| 487 | `map` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;316 |
| 488 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;327 |
| 489 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;337 |
| 490 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 876&#8209;878 |
| 491 | `iter` |  |  | Y |  | Y |  |  | unknown | 882&#8209;886 |
| 492 | `next` |  | Y |  |  | Y |  |  | unknown | 933&#8209;949 |
| 493 | `eq` |  | Y |  |  | Y |  |  | hole | 1039&#8209;1040 |

### Chap18/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 494 | `new` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;117 |
| 495 | `length` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 496 | `nth` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 497 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;143 |
| 498 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;156 |
| 499 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;164 |
| 500 | `empty` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;174 |
| 501 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;182 |
| 502 | `append` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;195 |
| 503 | `filter` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;216 |
| 504 | `update` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;229 |
| 505 | `inject` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;244 |
| 506 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;250 |
| 507 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;256 |
| 508 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;266 |
| 509 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;279 |
| 510 | `scan` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;296 |
| 511 | `map` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;306 |
| 512 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;317 |
| 513 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;327 |
| 514 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 857&#8209;859 |
| 515 | `iter` |  |  | Y |  | Y |  |  | unknown | 863&#8209;867 |
| 516 | `next` |  | Y |  |  | Y |  |  | unknown | 901&#8209;917 |
| 517 | `eq` |  | Y |  |  | Y |  |  | hole | 1001&#8209;1002 |

### Chap18/LinkedListStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 518 | `new` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;104 |
| 519 | `set` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;113 |
| 520 | `length` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 521 | `nth` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 522 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;136 |
| 523 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;143 |
| 524 | `empty` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 525 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;159 |
| 526 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;169 |
| 527 | `map` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;178 |
| 528 | `append` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;190 |
| 529 | `filter` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;210 |
| 530 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;219 |
| 531 | `update` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;231 |
| 532 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;236 |
| 533 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;241 |
| 534 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;250 |
| 535 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;262 |
| 536 | `scan` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;278 |
| 537 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 718&#8209;720 |
| 538 | `iter` |  |  | Y |  | Y |  |  | unknown | 726&#8209;730 |
| 539 | `next` |  | Y |  |  | Y |  |  | unknown | 766&#8209;782 |
| 540 | `eq` |  | Y |  |  | Y |  |  | hole | 874&#8209;875 |

### Chap18/LinkedListStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 541 | `new` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;105 |
| 542 | `length` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 543 | `nth` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 544 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;128 |
| 545 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;135 |
| 546 | `empty` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;144 |
| 547 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;151 |
| 548 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;161 |
| 549 | `map` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;170 |
| 550 | `append` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;182 |
| 551 | `filter` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;202 |
| 552 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;211 |
| 553 | `update` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;223 |
| 554 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;228 |
| 555 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;233 |
| 556 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;242 |
| 557 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;254 |
| 558 | `scan` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;270 |
| 559 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 701&#8209;703 |
| 560 | `iter` |  |  | Y |  | Y |  |  | unknown | 709&#8209;713 |
| 561 | `next` |  | Y |  |  | Y |  |  | unknown | 749&#8209;765 |
| 562 | `eq` |  | Y |  |  | Y |  |  | hole | 857&#8209;858 |

### Chap19/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 563 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 122&#8209;131 |
| 564 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 162&#8209;164 |
| 565 | `new` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;193 |
| 566 | `set` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;204 |
| 567 | `length` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;210 |
| 568 | `nth` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;217 |
| 569 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;230 |
| 570 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;243 |
| 571 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;251 |
| 572 | `empty` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;257 |
| 573 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;267 |
| 574 | `append` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;280 |
| 575 | `filter` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;297 |
| 576 | `update` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;310 |
| 577 | `inject` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;325 |
| 578 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;339 |
| 579 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 344&#8209;345 |
| 580 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;351 |
| 581 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;357 |
| 582 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;366 |
| 583 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 371&#8209;373 |
| 584 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 378&#8209;382 |
| 585 | `scan` | Y | Y |  |  | Y |  |  | unknown | 387&#8209;390 |
| 586 | `map` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;400 |
| 587 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 405&#8209;411 |
| 588 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 416&#8209;421 |
| 589 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 426&#8209;434 |
| 590 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 992&#8209;995 |
| 591 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 1007&#8209;1012 |
| 592 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1025&#8209;1027 |
| 593 | `iter` |  |  | Y |  | Y |  |  | unknown | 1031&#8209;1035 |
| 594 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1043&#8209;1054 |
| 595 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1096&#8209;1106 |
| 596 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1150&#8209;1153 |
| 597 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1179&#8209;1194 |
| 598 | `next` |  | Y |  |  | Y |  |  | unknown | 1290&#8209;1306 |
| 599 | `eq` |  | Y |  |  | Y |  |  | hole | 1409&#8209;1410 |

### Chap19/ArraySeqMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 600 | `length` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;90 |
| 601 | `nth_cloned` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;97 |
| 602 | `slice` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;109 |
| 603 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;121 |
| 604 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 605 | `empty` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;133 |
| 606 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;140 |
| 607 | `new` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;150 |
| 608 | `iter` |  |  | Y |  | Y |  |  | unknown | 247&#8209;253 |

### Chap19/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 609 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 116&#8209;119 |
| 610 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 131&#8209;136 |
| 611 | `new` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;171 |
| 612 | `set` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;182 |
| 613 | `length` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;188 |
| 614 | `nth` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;195 |
| 615 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;208 |
| 616 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;221 |
| 617 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;229 |
| 618 | `empty` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;235 |
| 619 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;245 |
| 620 | `append` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;258 |
| 621 | `filter` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;275 |
| 622 | `update` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;288 |
| 623 | `inject` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;303 |
| 624 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;309 |
| 625 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;315 |
| 626 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;321 |
| 627 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;330 |
| 628 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;337 |
| 629 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;346 |
| 630 | `scan` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;354 |
| 631 | `map` | Y | Y |  |  | Y |  |  | unknown | 359&#8209;364 |
| 632 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;375 |
| 633 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 380&#8209;385 |
| 634 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;398 |
| 635 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 943&#8209;945 |
| 636 | `iter` |  |  | Y |  | Y |  |  | unknown | 949&#8209;953 |
| 637 | `next` |  | Y |  |  | Y |  |  | unknown | 989&#8209;1005 |
| 638 | `eq` |  | Y |  |  | Y |  |  | hole | 1113&#8209;1114 |

### Chap19/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 639 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 115&#8209;118 |
| 640 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 130&#8209;135 |
| 641 | `new` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;169 |
| 642 | `length` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;175 |
| 643 | `nth` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;182 |
| 644 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;195 |
| 645 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;208 |
| 646 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;216 |
| 647 | `empty` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;222 |
| 648 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;232 |
| 649 | `append` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;245 |
| 650 | `filter` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;262 |
| 651 | `update` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;275 |
| 652 | `inject` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;290 |
| 653 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;296 |
| 654 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;302 |
| 655 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;308 |
| 656 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;317 |
| 657 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;324 |
| 658 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;333 |
| 659 | `scan` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;341 |
| 660 | `map` | Y | Y |  |  | Y |  |  | unknown | 346&#8209;351 |
| 661 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;362 |
| 662 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;372 |
| 663 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 377&#8209;385 |
| 664 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 952&#8209;954 |
| 665 | `iter` |  |  | Y |  | Y |  |  | unknown | 958&#8209;962 |
| 666 | `next` |  | Y |  |  | Y |  |  | unknown | 998&#8209;1014 |
| 667 | `eq` |  | Y |  |  | Y |  |  | hole | 1122&#8209;1123 |

### Chap21/Algorithm21_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 668 | `lemma_sum_inner_lens_mono` |  |  |  | Y | Y |  |  | unknown | 45&#8209;48 |
| 669 | `lemma_sum_inner_lens_uniform` |  |  |  | Y | Y |  |  | unknown | 58&#8209;64 |
| 670 | `flatten_inner` |  |  |  | Y | Y |  |  | unknown | 85&#8209;89 |
| 671 | `points2d_tab_flat` |  |  |  | Y | Y |  |  | unknown | 145&#8209;151 |

### Chap21/Algorithm21_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 672 | `points3d_tab_flat` |  |  |  | Y | Y |  |  | unknown | 46&#8209;53 |

### Chap21/Algorithm21_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 673 | `primes_bf` |  |  |  | Y | Y |  |  | unknown | 46&#8209;54 |

### Chap21/Algorithm21_6.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 674 | `lemma_product_not_prime` |  |  |  | Y | Y |  |  | unknown | 37&#8209;39 |
| 675 | `prime_sieve` |  |  |  | Y | Y |  |  | unknown | 62&#8209;68 |

### Chap21/Exercise21_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 676 | `lemma_inner_lens_sum_triangular` |  |  |  | Y | Y |  |  | unknown | 40&#8209;47 |
| 677 | `all_contiguous_subseqs` |  |  |  | Y | Y |  |  | unknown | 68&#8209;72 |

### Chap21/Exercise21_7.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 678 | `is_even` |  |  |  | Y | Y |  |  | unknown | 47&#8209;48 |
| 679 | `is_vowel` |  |  |  | Y | Y |  |  | unknown | 60&#8209;61 |
| 680 | `pair_even_with_vowels` |  |  |  | Y | Y |  |  | unknown | 75&#8209;84 |

### Chap21/Exercise21_8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 681 | `lemma_zero_count_means_no_divisors` |  |  |  | Y | Y |  |  | unknown | 55&#8209;62 |
| 682 | `lemma_no_divisors_means_zero_count` |  |  |  | Y | Y |  |  | unknown | 81&#8209;88 |
| 683 | `lemma_divisor_count_nonneg` |  |  |  | Y | Y |  |  | unknown | 99&#8209;101 |
| 684 | `lemma_filter_len_eq_divisor_count` |  |  |  | Y | Y |  |  | unknown | 109&#8209;115 |
| 685 | `lemma_divisor_count_split_last` |  |  |  | Y | Y |  |  | unknown | 135&#8209;140 |
| 686 | `is_divisible` |  |  |  | Y | Y |  |  | unknown | 164&#8209;166 |
| 687 | `is_prime` |  |  |  | Y | Y |  |  | unknown | 176&#8209;177 |

### Chap21/Exercise21_9.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 688 | `lemma_div_exact` |  |  |  | Y | Y |  |  | unknown | 28&#8209;30 |
| 689 | `lemma_composite_has_small_divisor` |  |  |  | Y | Y |  |  | unknown | 38&#8209;43 |
| 690 | `lemma_composites_covered_by_small_multiples` |  |  |  | Y | Y |  |  | unknown | 75&#8209;82 |

### Chap21/Problem21_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 691 | `points2d` |  |  |  | Y | Y |  |  | unknown | 34&#8209;43 |

### Chap21/Problem21_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 692 | `points3d_loops` |  |  |  | Y | Y |  |  | unknown | 37&#8209;48 |

### Chap21/Problem21_4.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 693 | `cartesian_loops` |  |  |  | Y | Y |  |  | unknown | 40&#8209;47 |
| 694 | `cartesian_tab_flat` |  |  |  | Y | Y |  |  | unknown | 92&#8209;103 |

### Chap23/BalBinTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 695 | `lemma_in_order_pre_order_permutation` |  |  |  | Y | Y |  |  | unknown | 92&#8209;94 |
| 696 | `lemma_pre_order_post_order_permutation` |  |  |  | Y | Y |  |  | unknown | 137&#8209;139 |
| 697 | `leaf` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;194 |
| 698 | `node` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;204 |
| 699 | `is_leaf` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;209 |
| 700 | `size` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;215 |
| 701 | `height` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;221 |
| 702 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;230 |
| 703 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;239 |
| 704 | `post_order` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;248 |
| 705 | `iter_in_order` |  |  | Y |  | Y |  |  | unknown | 428&#8209;434 |
| 706 | `iter_pre_order` |  |  | Y |  | Y |  |  | unknown | 442&#8209;448 |
| 707 | `iter_post_order` |  |  | Y |  | Y |  |  | unknown | 456&#8209;462 |
| 708 | `next` x3 |  | Y |  |  | Y |  |  | unknown | 546&#8209;562 |
| 709 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 761&#8209;763 |
| 710 | `clone_tree` |  |  |  | Y | Y |  |  | hole | 790&#8209;792 |

### Chap23/PrimTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 711 | `empty` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 712 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;133 |
| 713 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;141 |
| 714 | `length` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 715 | `nth` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;154 |
| 716 | `expose` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;166 |
| 717 | `join` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;185 |
| 718 | `append` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;198 |
| 719 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;211 |
| 720 | `update` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;224 |
| 721 | `map` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;234 |
| 722 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;245 |
| 723 | `filter` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;262 |
| 724 | `drop` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;275 |
| 725 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;285 |
| 726 | `as_slice` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;291 |
| 727 | `into_vec` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;297 |
| 728 | `iter` |  |  | Y |  | Y |  |  | unknown | 317&#8209;321 |
| 729 | `next` |  | Y |  |  | Y |  |  | unknown | 757&#8209;773 |
| 730 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 859&#8209;860 |

### Chap26/DivConReduceMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 731 | `lemma_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 80&#8209;84 |
| 732 | `lemma_max_fold_left_bound` |  |  |  | Y | Y |  |  | unknown | 93&#8209;98 |
| 733 | `lemma_max_fold_left_achievable` |  |  |  | Y | Y |  |  | unknown | 118&#8209;123 |
| 734 | `max_element_parallel` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;159 |
| 735 | `sum_parallel` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;177 |
| 736 | `product_parallel` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;189 |
| 737 | `any_parallel` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;201 |
| 738 | `all_parallel` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;213 |

### Chap26/DivConReduceStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 739 | `max_element` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;67 |
| 740 | `sum` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;85 |
| 741 | `product` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;97 |
| 742 | `any` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;109 |
| 743 | `all` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;121 |

### Chap26/ETSPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 744 | `lemma_point_in_seq_transitive` |  |  |  | Y | Y |  |  | unknown | 118&#8209;124 |
| 745 | `lemma_edge_valid_transitive` |  |  |  | Y | Y |  |  | unknown | 134&#8209;146 |
| 746 | `lemma_mod_successor` |  |  |  | Y | Y |  |  | unknown | 153&#8209;155 |
| 747 | `lemma_combined_cycle` |  |  |  | Y | Y |  |  | hole | 166&#8209;190 |
| 748 | `etsp_parallel` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;262 |
| 749 | `etsp_parallel_inner` |  |  |  | Y | Y |  |  | unknown | 274&#8209;279 |
| 750 | `sort_and_split` |  |  |  | Y | Y |  |  | hole | 460&#8209;471 |
| 751 | `find_best_swap` |  |  |  | Y | Y |  |  | hole | 478&#8209;484 |
| 752 | `distance` |  |  | Y |  |  | Y | Y |  | 508&#8209;515 |
| 753 | `sort_and_split_impl` |  |  |  | Y |  | Y | Y |  | 518&#8209;540 |
| 754 | `find_best_swap_impl` |  |  |  | Y |  | Y | Y |  | 542&#8209;548 |
| 755 | `find_best_swap_par` |  |  |  | Y |  | Y | Y |  | 550&#8209;584 |

### Chap26/ETSPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 756 | `lemma_point_in_seq_transitive` |  |  |  | Y | Y |  |  | unknown | 111&#8209;117 |
| 757 | `lemma_edge_valid_transitive` |  |  |  | Y | Y |  |  | unknown | 127&#8209;139 |
| 758 | `lemma_combined_cycle` |  |  |  | Y | Y |  |  | hole | 146&#8209;170 |
| 759 | `etsp` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;247 |
| 760 | `etsp_inner` |  |  |  | Y | Y |  |  | unknown | 262&#8209;267 |
| 761 | `sort_and_split` |  |  |  | Y | Y |  |  | hole | 434&#8209;445 |
| 762 | `find_best_swap` |  |  |  | Y | Y |  |  | hole | 452&#8209;458 |
| 763 | `distance` |  |  | Y |  |  | Y | Y |  | 482&#8209;489 |
| 764 | `sort_and_split_impl` |  |  |  | Y |  | Y | Y |  | 492&#8209;514 |
| 765 | `find_best_swap_impl` |  |  |  | Y |  | Y | Y |  | 516&#8209;535 |

### Chap26/MergeSortMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 766 | `lemma_multiset_count_positive_implies_exists` |  |  |  | Y | Y |  |  | unknown | 86&#8209;89 |
| 767 | `lemma_all_le_preserved_by_permutation` |  |  |  | Y | Y |  |  | unknown | 105&#8209;110 |
| 768 | `lemma_all_ge_preserved_by_permutation` |  |  |  | Y | Y |  |  | unknown | 122&#8209;127 |
| 769 | `lemma_sorted_concat_pivot` |  |  |  | Y | Y |  |  | unknown | 140&#8209;147 |
| 770 | `merge_parallel` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;185 |
| 771 | `merge_sort_parallel` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;195 |
| 772 | `binary_search_upper_bound` |  |  |  | Y | Y |  |  | unknown | 204&#8209;211 |
| 773 | `merge_dc` |  |  |  | Y | Y |  |  | unknown | 261&#8209;271 |

### Chap26/MergeSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 774 | `lemma_push_sorted` |  |  |  | Y | Y |  |  | unknown | 82&#8209;87 |
| 775 | `merge` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;122 |
| 776 | `merge_sort` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;132 |

### Chap26/ScanDCMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 777 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 71&#8209;74 |
| 778 | `prefix_sums_dc_parallel` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;98 |
| 779 | `prefix_sums_dc_inner` |  |  |  | Y | Y |  |  | unknown | 104&#8209;112 |

### Chap26/ScanDCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 780 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 76&#8209;79 |
| 781 | `scan_dc` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;110 |
| 782 | `prefix_sums_dc` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;123 |

### Chap27/ReduceContractMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 783 | `reduce_contract_parallel` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;72 |
| 784 | `contract_parallel` |  |  |  | Y | Y |  |  | unknown | 84&#8209;99 |
| 785 | `reduce_contract_verified` |  |  |  | Y | Y |  |  | unknown | 254&#8209;268 |

### Chap27/ReduceContractStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 786 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 40&#8209;43 |
| 787 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 56&#8209;58 |
| 788 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 72&#8209;74 |
| 789 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 84&#8209;94 |
| 790 | `reduce_contract` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;176 |

### Chap27/ScanContractMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 791 | `scan_contract_parallel` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;66 |
| 792 | `scan_contract_verified` |  |  |  | Y | Y |  |  | unknown | 75&#8209;92 |

### Chap27/ScanContractStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 793 | `lemma_prefix_contraction` |  |  |  | Y | Y |  |  | unknown | 42&#8209;50 |
| 794 | `scan_contract` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;83 |

### Chap28/MCSSSpec.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 795 | `lemma_range_sum_snoc` |  |  |  | Y | Y |  |  | unknown | 136&#8209;143 |
| 796 | `lemma_range_sum_single` |  |  |  | Y | Y |  |  | unknown | 152&#8209;156 |
| 797 | `lemma_range_sum_empty` |  |  |  | Y | Y |  |  | unknown | 162&#8209;164 |
| 798 | `lemma_range_sum_split` |  |  |  | Y | Y |  |  | unknown | 169&#8209;174 |
| 799 | `lemma_range_sum_via_prefix` |  |  |  | Y | Y |  |  | unknown | 183&#8209;187 |
| 800 | `lemma_min_prefix_sum_is_min` |  |  |  | Y | Y |  |  | unknown | 193&#8209;198 |
| 801 | `lemma_min_prefix_sum_achieved` |  |  |  | Y | Y |  |  | unknown | 207&#8209;212 |
| 802 | `lemma_range_sum_subseq` |  |  |  | Y | Y |  |  | unknown | 229&#8209;237 |
| 803 | `lemma_crossing_decompose` |  |  |  | Y | Y |  |  | unknown | 248&#8209;252 |
| 804 | `lemma_sums_fit_subseq` |  |  |  | Y | Y |  |  | unknown | 258&#8209;265 |

### Chap28/MaxContigSubSumBruteStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 805 | `max_contig_sub_sum_brute` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;59 |
| 806 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 69&#8209;71 |

### Chap28/MaxContigSubSumDivConMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 807 | `max_contig_sub_sum_divcon_mt` | Y | Y |  |  | Y |  |  | unknown | 48&#8209;55 |
| 808 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 61&#8209;62 |
| 809 | `max_suffix_sum` |  |  |  | Y | Y |  |  | unknown | 72&#8209;74 |
| 810 | `max_prefix_sum` |  |  |  | Y | Y |  |  | unknown | 122&#8209;124 |

### Chap28/MaxContigSubSumDivConOptMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 811 | `max_contig_sub_sum_divcon_opt_mt` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;61 |
| 812 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 67&#8209;68 |
| 813 | `max_contig_sub_sum_aux` |  |  |  | Y | Y |  |  | unknown | 78&#8209;86 |

### Chap28/MaxContigSubSumDivConOptStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 814 | `lemma_strength_combine` |  |  |  | Y | Y |  |  | unknown | 61&#8209;110 |
| 815 | `max_contig_sub_sum_divcon_opt` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;261 |
| 816 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 269&#8209;270 |
| 817 | `max_contig_sub_sum_aux` |  |  |  | Y | Y |  |  | unknown | 283&#8209;293 |

### Chap28/MaxContigSubSumDivConStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 818 | `lemma_divcon_combine` |  |  |  | Y | Y |  |  | unknown | 53&#8209;80 |
| 819 | `max_contig_sub_sum_divcon` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;152 |
| 820 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 160&#8209;161 |
| 821 | `max_suffix_sum` |  |  |  | Y | Y |  |  | unknown | 176&#8209;181 |
| 822 | `max_prefix_sum` |  |  |  | Y | Y |  |  | unknown | 252&#8209;257 |

### Chap28/MaxContigSubSumIterStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 823 | `lemma_max_ending_at_is_max` |  |  |  | Y | Y |  |  | unknown | 66&#8209;72 |
| 824 | `lemma_max_ending_at_achieved` |  |  |  | Y | Y |  |  | unknown | 89&#8209;96 |
| 825 | `max_contig_sub_sum_iter` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;131 |
| 826 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 139&#8209;140 |

### Chap28/MaxContigSubSumOptMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 827 | `max_contig_sub_sum_opt_mt` | Y | Y |  |  | Y |  |  | unknown | 22&#8209;29 |

### Chap28/MaxContigSubSumOptStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 828 | `lemma_prefix_opt_is_mcss` |  |  |  | Y | Y |  |  | unknown | 22&#8209;38 |
| 829 | `max_contig_sub_sum_opt` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;78 |

### Chap28/MaxContigSubSumReducedMcsseStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 830 | `max_contig_sub_sum_reduced_mcsse` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;58 |
| 831 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 68&#8209;69 |

### Chap28/MaxContigSubSumReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 832 | `max_contig_sub_sum_reduced` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;55 |
| 833 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 65&#8209;66 |

### Chap35/OrderStatSelectMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 834 | `select` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;54 |
| 835 | `parallel_three_way_partition` |  |  |  | Y | Y |  |  | hole | 62&#8209;84 |
| 836 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 125&#8209;134 |

### Chap35/OrderStatSelectMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 837 | `select` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;54 |
| 838 | `parallel_three_way_partition` |  |  |  | Y | Y |  |  | hole | 60&#8209;82 |
| 839 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 123&#8209;132 |

### Chap35/OrderStatSelectStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 840 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 57&#8209;58 |
| 841 | `select` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;92 |
| 842 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 110&#8209;117 |

### Chap35/OrderStatSelectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 843 | `select` | Y | Y |  |  | Y |  |  | unknown | 46&#8209;51 |
| 844 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 69&#8209;76 |

### Chap36/QuickSortMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 845 | `sort_vec` |  |  |  | Y | Y |  |  | unknown | 46&#8209;49 |
| 846 | `median_of_three` |  |  |  | Y | Y |  |  | unknown | 371&#8209;372 |
| 847 | `median3_pivot_idx` |  |  |  | Y | Y |  |  | unknown | 400&#8209;402 |
| 848 | `sort_vec_random` |  |  |  | Y | Y |  |  | unknown | 417&#8209;420 |
| 849 | `sort_vec_median3` |  |  |  | Y | Y |  |  | unknown | 430&#8209;433 |
| 850 | `sort_vec_with_idx` |  |  |  | Y | Y |  |  | unknown | 445&#8209;451 |
| 851 | `quick_sort_first` |  |  |  | Y | Y |  |  | unknown | 737&#8209;739 |
| 852 | `quick_sort_median3` |  |  |  | Y | Y |  |  | unknown | 746&#8209;748 |
| 853 | `quick_sort_random` |  |  |  | Y | Y |  |  | unknown | 755&#8209;757 |

### Chap36/QuickSortMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 854 | `pivot_mt_first` | Y | Y |  |  | Y |  | Y |  | 20 |
| 855 | `pivot_mt_median3` | Y | Y |  |  | Y |  | Y |  | 23 |
| 856 | `pivot_mt_random` | Y | Y |  |  | Y |  | Y |  | 26 |
| 857 | `quick_sort_mt_first` | Y | Y |  |  | Y |  | Y |  | 29 |
| 858 | `quick_sort_mt_median3` | Y | Y |  |  | Y |  | Y |  | 32 |
| 859 | `quick_sort_mt_random` | Y | Y |  |  | Y |  |  | hole | 35 |

### Chap36/QuickSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 860 | `sort_vec` |  |  |  | Y | Y |  |  | unknown | 44&#8209;47 |
| 861 | `quick_sort_first` |  |  |  | Y | Y |  |  | unknown | 367&#8209;369 |
| 862 | `median_of_three` |  |  |  | Y | Y |  |  | unknown | 388&#8209;389 |
| 863 | `median3_pivot_idx` |  |  |  | Y | Y |  |  | unknown | 419&#8209;421 |
| 864 | `sort_vec_random` |  |  |  | Y | Y |  |  | unknown | 436&#8209;439 |
| 865 | `sort_vec_median3` |  |  |  | Y | Y |  |  | unknown | 449&#8209;452 |
| 866 | `sort_vec_with_idx` |  |  |  | Y | Y |  |  | unknown | 464&#8209;470 |
| 867 | `quick_sort_median3` |  |  |  | Y | Y |  |  | unknown | 735&#8209;737 |
| 868 | `quick_sort_random` |  |  |  | Y | Y |  |  | unknown | 744&#8209;746 |

### Chap37/AVLTreeSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 869 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 125&#8209;128 |
| 870 | `empty` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;146 |
| 871 | `new` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;149 |
| 872 | `length` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 873 | `nth` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 874 | `set` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 875 | `singleton` | Y | Y |  |  | Y |  |  | hole | 162&#8209;163 |
| 876 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 877 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 878 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 173&#8209;174 |
| 879 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;177 |
| 880 | `update` | Y | Y |  |  | Y |  |  | hole | 179 |
| 881 | `from_vec` | Y | Y |  |  | Y |  |  | hole | 181 |
| 882 | `to_arrayseq` | Y | Y |  |  | Y |  |  | hole | 183 |
| 883 | `iter` | Y | Y |  |  | Y |  |  | hole | 185 |
| 884 | `push_back` | Y | Y |  |  | Y |  |  | hole | 187 |
| 885 | `contains_value` | Y | Y |  |  | Y |  |  | hole | 189 |
| 886 | `insert_value` | Y | Y |  |  | Y |  |  | hole | 191 |
| 887 | `delete_value` | Y | Y |  |  | Y |  |  | hole | 193 |
| 888 | `is_tree_empty` | Y | Y |  |  | Y |  |  | hole | 195 |
| 889 | `values_in_order` | Y | Y |  |  | Y |  |  | hole | 197 |
| 890 | `h_fn` |  |  |  | Y | Y |  |  | unknown | 202&#8209;203 |
| 891 | `size_link_fn` |  |  |  | Y | Y |  |  | hole | 211&#8209;212 |
| 892 | `update_meta` |  |  |  | Y | Y |  |  | hole | 224 |
| 893 | `rotate_right_fn` |  |  |  | Y | Y |  |  | hole | 233&#8209;237 |
| 894 | `rotate_left_fn` |  |  |  | Y | Y |  |  | hole | 250&#8209;254 |
| 895 | `rebalance_fn` |  |  |  | Y | Y |  |  | hole | 267&#8209;271 |
| 896 | `insert_at_link` |  |  |  | Y | Y |  |  | hole | 294 |
| 897 | `nth_link` |  |  |  | Y | Y |  |  | hole | 323&#8209;325 |
| 898 | `set_link` |  |  |  | Y | Y |  |  | hole | 339 |
| 899 | `push_inorder` |  |  |  | Y | Y |  |  | hole | 357 |
| 900 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 366 |
| 901 | `eq` |  | Y |  |  | Y |  |  | hole | 621&#8209;622 |
| 902 | `default` |  | Y |  |  |  | Y | Y |  | 647 |
| 903 | `push_left_iter` |  |  |  | Y |  | Y | Y |  | 672&#8209;678 |
| 904 | `next` |  | Y |  |  |  | Y | Y |  | 682&#8209;687 |

### Chap37/AVLTreeSeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 905 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 118&#8209;121 |
| 906 | `empty` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;139 |
| 907 | `new` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;142 |
| 908 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;145 |
| 909 | `length` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;149 |
| 910 | `nth` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 911 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 912 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 913 | `set` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 914 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 166&#8209;167 |
| 915 | `from_vec` | Y | Y |  |  | Y |  | Y |  | 169 |
| 916 | `values_in_order` | Y | Y |  |  | Y |  |  | hole | 171 |
| 917 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 176&#8209;177 |
| 918 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 185&#8209;186 |
| 919 | `mk` |  |  |  | Y | Y |  |  | hole | 195&#8209;204 |
| 920 | `rotate_right` |  |  |  | Y | Y |  |  | hole | 214&#8209;218 |
| 921 | `rotate_left` |  |  |  | Y | Y |  |  | hole | 227&#8209;231 |
| 922 | `rebalance` |  |  |  | Y | Y |  |  | hole | 240&#8209;244 |
| 923 | `nth_ref` |  |  |  | Y | Y |  |  | hole | 268&#8209;270 |
| 924 | `set_rec` |  |  |  | Y | Y |  |  | hole | 289&#8209;291 |
| 925 | `inorder_collect` |  |  |  | Y | Y |  |  | hole | 317 |
| 926 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 326&#8209;327 |
| 927 | `rec` |  |  |  | Y | Y |  | Y |  | 329 |
| 928 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 344 |
| 929 | `eq` |  | Y |  |  | Y |  |  | hole | 462&#8209;463 |
| 930 | `default` |  | Y |  |  |  | Y | Y |  | 487 |
| 931 | `next` |  | Y |  |  |  | Y | Y |  | 511&#8209;519 |

### Chap37/AVLTreeSeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 932 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 124&#8209;127 |
| 933 | `empty` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;145 |
| 934 | `new` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;148 |
| 935 | `length` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 936 | `nth` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 937 | `set` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;159 |
| 938 | `singleton` | Y | Y |  |  | Y |  |  | hole | 161&#8209;162 |
| 939 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;166 |
| 940 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;170 |
| 941 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 172&#8209;173 |
| 942 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;176 |
| 943 | `update` | Y | Y |  |  | Y |  |  | hole | 178 |
| 944 | `from_vec` | Y | Y |  |  | Y |  |  | hole | 180 |
| 945 | `to_arrayseq` | Y | Y |  |  | Y |  |  | hole | 182 |
| 946 | `iter` | Y | Y |  |  | Y |  |  | hole | 184 |
| 947 | `push_back` | Y | Y |  |  | Y |  |  | hole | 186 |
| 948 | `contains_value` | Y | Y |  |  | Y |  |  | hole | 188 |
| 949 | `insert_value` | Y | Y |  |  | Y |  |  | hole | 190 |
| 950 | `delete_value` | Y | Y |  |  | Y |  |  | hole | 192 |
| 951 | `h_fn` |  |  |  | Y | Y |  |  | unknown | 197&#8209;198 |
| 952 | `size_link_fn` |  |  |  | Y | Y |  |  | hole | 206&#8209;207 |
| 953 | `update_meta` |  |  |  | Y | Y |  |  | hole | 219 |
| 954 | `rotate_right_fn` |  |  |  | Y | Y |  |  | hole | 228&#8209;232 |
| 955 | `rotate_left_fn` |  |  |  | Y | Y |  |  | hole | 245&#8209;249 |
| 956 | `rebalance_fn` |  |  |  | Y | Y |  |  | hole | 262&#8209;266 |
| 957 | `insert_at_link` |  |  |  | Y | Y |  |  | hole | 289 |
| 958 | `nth_link` |  |  |  | Y | Y |  |  | hole | 318&#8209;320 |
| 959 | `set_link` |  |  |  | Y | Y |  |  | hole | 334 |
| 960 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 352 |
| 961 | `eq` |  | Y |  |  | Y |  |  | hole | 607&#8209;608 |
| 962 | `default` |  | Y |  |  |  | Y | Y |  | 633 |
| 963 | `push_left_iter` |  |  |  | Y |  | Y | Y |  | 638&#8209;644 |
| 964 | `next` |  | Y |  |  |  | Y | Y |  | 648&#8209;653 |

### Chap37/AVLTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 965 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 124&#8209;127 |
| 966 | `empty` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;145 |
| 967 | `new` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;148 |
| 968 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 969 | `length` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 970 | `nth` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 971 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 972 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 973 | `set` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;170 |
| 974 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 172&#8209;173 |
| 975 | `from_vec` | Y | Y |  |  | Y |  | Y |  | 175 |
| 976 | `values_in_order` | Y | Y |  |  | Y |  |  | hole | 177 |
| 977 | `to_arrayseq` | Y | Y |  |  | Y |  |  | hole | 179 |
| 978 | `iter` | Y | Y |  |  | Y |  |  | hole | 181 |
| 979 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 186&#8209;187 |
| 980 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 195&#8209;196 |
| 981 | `mk` |  |  |  | Y | Y |  |  | hole | 205&#8209;214 |
| 982 | `rotate_right` |  |  |  | Y | Y |  |  | hole | 224&#8209;228 |
| 983 | `rotate_left` |  |  |  | Y | Y |  |  | hole | 237&#8209;241 |
| 984 | `rebalance` |  |  |  | Y | Y |  |  | hole | 250&#8209;254 |
| 985 | `nth_ref` |  |  |  | Y | Y |  |  | hole | 278&#8209;280 |
| 986 | `set_rec` |  |  |  | Y | Y |  |  | hole | 299&#8209;301 |
| 987 | `inorder_collect` |  |  |  | Y | Y |  |  | hole | 327 |
| 988 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 336&#8209;337 |
| 989 | `rec` |  |  |  | Y | Y |  | Y |  | 339 |
| 990 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 352 |
| 991 | `eq` |  | Y |  |  | Y |  |  | hole | 478&#8209;479 |
| 992 | `default` |  | Y |  |  |  | Y | Y |  | 503 |
| 993 | `push_left_iter` |  |  |  | Y |  | Y | Y |  | 520&#8209;525 |
| 994 | `next` |  | Y |  |  |  | Y | Y |  | 529&#8209;538 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 995 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 53&#8209;85 |
| 996 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 100&#8209;104 |
| 997 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 166&#8209;170 |
| 998 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 233&#8209;242 |
| 999 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 325&#8209;328 |
| 1000 | `find_node` |  |  |  | Y | Y |  |  | unknown | 350&#8209;355 |
| 1001 | `min_node` |  |  |  | Y | Y |  | Y |  | 377&#8209;378 |
| 1002 | `max_node` |  |  |  | Y | Y |  | Y |  | 389&#8209;390 |
| 1003 | `new` |  |  | Y |  | Y |  | Y |  | 404 |
| 1004 | `insert` |  |  | Y |  | Y |  | Y |  | 414 |
| 1005 | `contains` |  |  | Y |  | Y |  | Y |  | 437 |
| 1006 | `size` |  |  | Y |  | Y |  | Y |  | 446 |
| 1007 | `is_empty` |  |  | Y |  | Y |  | Y |  | 456 |
| 1008 | `height` |  |  | Y |  | Y |  | Y |  | 465 |

### Chap37/BSTAVLStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1009 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 56&#8209;88 |
| 1010 | `lemma_max_plus_one` |  |  |  | Y | Y |  |  | unknown | 95&#8209;97 |
| 1011 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 108&#8209;146 |
| 1012 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 260&#8209;298 |
| 1013 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 419&#8209;447 |
| 1014 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 705&#8209;716 |
| 1015 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 859&#8209;862 |
| 1016 | `find_node` |  |  |  | Y | Y |  |  | unknown | 892&#8209;897 |
| 1017 | `min_node` |  |  |  | Y | Y |  | Y |  | 927&#8209;928 |
| 1018 | `max_node` |  |  |  | Y | Y |  | Y |  | 942&#8209;943 |
| 1019 | `avl_new` |  |  |  | Y | Y |  |  | unknown | 957&#8209;960 |
| 1020 | `avl_size` |  |  |  | Y | Y |  |  | unknown | 965&#8209;967 |
| 1021 | `avl_is_empty` |  |  |  | Y | Y |  |  | unknown | 972&#8209;973 |
| 1022 | `avl_height` |  |  |  | Y | Y |  |  | unknown | 978&#8209;980 |
| 1023 | `avl_insert` |  |  |  | Y | Y |  |  | unknown | 985&#8209;993 |
| 1024 | `avl_contains` |  |  |  | Y | Y |  |  | unknown | 998&#8209;1000 |
| 1025 | `avl_find` |  |  |  | Y | Y |  |  | unknown | 1005&#8209;1009 |

### Chap37/BSTBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1026 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 63&#8209;72 |
| 1027 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 155&#8209;158 |
| 1028 | `find_node` |  |  |  | Y | Y |  |  | unknown | 180&#8209;185 |
| 1029 | `min_node` |  |  |  | Y | Y |  | Y |  | 207&#8209;208 |
| 1030 | `max_node` |  |  |  | Y | Y |  | Y |  | 219&#8209;220 |
| 1031 | `new` |  |  | Y |  | Y |  | Y |  | 234 |
| 1032 | `insert` |  |  | Y |  | Y |  | Y |  | 244 |
| 1033 | `contains` |  |  | Y |  | Y |  | Y |  | 267 |
| 1034 | `size` |  |  | Y |  | Y |  | Y |  | 276 |
| 1035 | `is_empty` |  |  | Y |  | Y |  | Y |  | 286 |
| 1036 | `height` |  |  | Y |  | Y |  | Y |  | 295 |

### Chap37/BSTBBAlphaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1037 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 60&#8209;67 |
| 1038 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 189&#8209;192 |
| 1039 | `find_node` |  |  |  | Y | Y |  |  | unknown | 222&#8209;227 |
| 1040 | `min_node` |  |  |  | Y | Y |  | Y |  | 257&#8209;258 |
| 1041 | `max_node` |  |  |  | Y | Y |  | Y |  | 272&#8209;273 |
| 1042 | `bb_new` |  |  |  | Y | Y |  |  | unknown | 287&#8209;290 |
| 1043 | `bb_size` |  |  |  | Y | Y |  |  | unknown | 295&#8209;297 |
| 1044 | `bb_is_empty` |  |  |  | Y | Y |  |  | unknown | 302&#8209;303 |
| 1045 | `bb_height` |  |  |  | Y | Y |  |  | unknown | 308&#8209;310 |
| 1046 | `bb_insert` |  |  |  | Y | Y |  |  | unknown | 315&#8209;321 |
| 1047 | `bb_contains` |  |  |  | Y | Y |  |  | unknown | 326&#8209;328 |
| 1048 | `bb_find` |  |  |  | Y | Y |  |  | unknown | 333&#8209;337 |

### Chap37/BSTPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1049 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 58&#8209;67 |
| 1050 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 150&#8209;153 |
| 1051 | `find_node` |  |  |  | Y | Y |  |  | unknown | 175&#8209;180 |
| 1052 | `min_node` |  |  |  | Y | Y |  | Y |  | 202&#8209;203 |
| 1053 | `max_node` |  |  |  | Y | Y |  | Y |  | 214&#8209;215 |
| 1054 | `new` |  |  | Y |  | Y |  | Y |  | 229 |
| 1055 | `insert` |  |  | Y |  | Y |  | Y |  | 239 |
| 1056 | `contains` |  |  | Y |  | Y |  | Y |  | 262 |
| 1057 | `is_empty` |  |  | Y |  | Y |  | Y |  | 271 |
| 1058 | `size` |  |  | Y |  | Y |  | Y |  | 280 |
| 1059 | `height` |  |  | Y |  | Y |  | Y |  | 290 |

### Chap37/BSTPlainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1060 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 64&#8209;71 |
| 1061 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 193&#8209;196 |
| 1062 | `find_node` |  |  |  | Y | Y |  |  | unknown | 226&#8209;231 |
| 1063 | `min_node` |  |  |  | Y | Y |  | Y |  | 261&#8209;262 |
| 1064 | `max_node` |  |  |  | Y | Y |  | Y |  | 276&#8209;277 |
| 1065 | `bst_new` |  |  |  | Y | Y |  |  | unknown | 291&#8209;294 |
| 1066 | `bst_size` |  |  |  | Y | Y |  |  | unknown | 299&#8209;301 |
| 1067 | `bst_is_empty` |  |  |  | Y | Y |  |  | unknown | 306&#8209;307 |
| 1068 | `bst_height` |  |  |  | Y | Y |  |  | unknown | 312&#8209;314 |
| 1069 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 319&#8209;325 |
| 1070 | `bst_contains` |  |  |  | Y | Y |  |  | unknown | 330&#8209;332 |
| 1071 | `bst_find` |  |  |  | Y | Y |  |  | unknown | 337&#8209;341 |

### Chap37/BSTRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1072 | `new_rb_link_lock` |  |  |  | Y | Y |  |  | hole | 329 |
| 1073 | `new_node` |  |  |  | Y |  | Y | Y |  | 31&#8209;39 |
| 1074 | `is_red` |  |  |  | Y |  | Y | Y |  | 41 |
| 1075 | `size_link` |  |  |  | Y |  | Y | Y |  | 43 |
| 1076 | `update` |  |  |  | Y |  | Y | Y |  | 45 |
| 1077 | `rotate_left` |  |  |  | Y |  | Y | Y |  | 47&#8209;64 |
| 1078 | `rotate_right` |  |  |  | Y |  | Y | Y |  | 66&#8209;83 |
| 1079 | `flip_colors` |  |  |  | Y |  | Y | Y |  | 85&#8209;104 |
| 1080 | `fix_up` |  |  |  | Y |  | Y | Y |  | 106&#8209;140 |
| 1081 | `insert_link` |  |  |  | Y |  | Y | Y |  | 142&#8209;156 |
| 1082 | `find_link` |  |  |  | Y |  | Y | Y |  | 158&#8209;171 |
| 1083 | `min_link` |  |  |  | Y |  | Y | Y |  | 173&#8209;181 |
| 1084 | `max_link` |  |  |  | Y |  | Y | Y |  | 183&#8209;191 |
| 1085 | `in_order_collect` |  |  |  | Y |  | Y | Y |  | 193&#8209;199 |
| 1086 | `pre_order_collect` |  |  |  | Y |  | Y | Y |  | 201&#8209;207 |
| 1087 | `in_order_parallel` |  |  |  | Y |  | Y | Y |  | 211&#8209;226 |
| 1088 | `pre_order_parallel` |  |  |  | Y |  | Y | Y |  | 228&#8209;243 |
| 1089 | `build_balanced` |  |  |  | Y |  | Y | Y |  | 245&#8209;264 |
| 1090 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 266&#8209;291 |
| 1091 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 293&#8209;315 |
| 1092 | `new` | Y | Y |  |  |  | Y | Y |  | 348&#8209;349 |
| 1093 | `from_sorted_slice` | Y | Y |  |  |  | Y | Y |  | 350&#8209;351 |
| 1094 | `insert` | Y | Y |  |  |  | Y | Y |  | 352&#8209;353 |
| 1095 | `find` | Y | Y |  |  |  | Y | Y |  | 354&#8209;355 |
| 1096 | `contains` | Y | Y |  |  |  | Y | Y |  | 356 |
| 1097 | `size` | Y | Y |  |  |  | Y | Y |  | 357 |
| 1098 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 358 |
| 1099 | `height` | Y | Y |  |  |  | Y | Y |  | 359 |
| 1100 | `minimum` | Y | Y |  |  |  | Y | Y |  | 360 |
| 1101 | `maximum` | Y | Y |  |  |  | Y | Y |  | 361 |
| 1102 | `in_order` | Y | Y |  |  |  | Y | Y |  | 362&#8209;363 |
| 1103 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 364&#8209;365 |
| 1104 | `filter` | Y | Y |  |  |  | Y | Y |  | 366&#8209;369 |
| 1105 | `reduce` | Y | Y |  |  |  | Y | Y |  | 370&#8209;373 |
| 1106 | `height_rec` |  | Y |  |  |  | Y | Y |  | 411&#8209;416 |
| 1107 | `default` |  | Y |  |  |  | Y | Y |  | 481 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1108 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 41&#8209;73 |
| 1109 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 87&#8209;93 |
| 1110 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 185&#8209;191 |
| 1111 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 282&#8209;289 |
| 1112 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 411&#8209;414 |
| 1113 | `find_node` |  |  |  | Y | Y |  |  | unknown | 444&#8209;449 |
| 1114 | `min_node` |  |  |  | Y | Y |  | Y |  | 479&#8209;480 |
| 1115 | `max_node` |  |  |  | Y | Y |  | Y |  | 494&#8209;495 |
| 1116 | `rb_new` |  |  |  | Y | Y |  |  | unknown | 509&#8209;512 |
| 1117 | `rb_size` |  |  |  | Y | Y |  |  | unknown | 517&#8209;519 |
| 1118 | `rb_is_empty` |  |  |  | Y | Y |  |  | unknown | 524&#8209;525 |
| 1119 | `rb_height` |  |  |  | Y | Y |  |  | unknown | 530&#8209;532 |
| 1120 | `rb_insert` |  |  |  | Y | Y |  |  | unknown | 537&#8209;543 |
| 1121 | `rb_contains` |  |  |  | Y | Y |  |  | unknown | 548&#8209;550 |
| 1122 | `rb_find` |  |  |  | Y | Y |  |  | unknown | 555&#8209;559 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1123 | `values_vec` |  |  |  | Y | Y |  | Y |  | 24 |
| 1124 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 26 |
| 1125 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 34&#8209;36 |
| 1126 | `empty` | Y | Y |  |  | Y |  | Y |  | 47 |
| 1127 | `singleton` | Y | Y |  |  | Y |  | Y |  | 49 |
| 1128 | `size` | Y | Y |  |  | Y |  | Y |  | 51 |
| 1129 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 53 |
| 1130 | `find` | Y | Y |  |  | Y |  | Y |  | 55 |
| 1131 | `contains` | Y | Y |  |  | Y |  | Y |  | 57 |
| 1132 | `minimum` | Y | Y |  |  | Y |  | Y |  | 59 |
| 1133 | `maximum` | Y | Y |  |  | Y |  | Y |  | 61 |
| 1134 | `insert` | Y | Y |  |  | Y |  | Y |  | 63 |
| 1135 | `delete` | Y | Y |  |  | Y |  | Y |  | 65 |
| 1136 | `union` | Y | Y |  |  | Y |  | Y |  | 67 |
| 1137 | `intersection` | Y | Y |  |  | Y |  | Y |  | 69 |
| 1138 | `difference` | Y | Y |  |  | Y |  | Y |  | 71 |
| 1139 | `split` | Y | Y |  |  | Y |  | Y |  | 73 |
| 1140 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 75 |
| 1141 | `join_m` | Y | Y |  |  | Y |  | Y |  | 77 |
| 1142 | `filter` | Y | Y |  |  | Y |  | Y |  | 79 |
| 1143 | `reduce` | Y | Y |  |  | Y |  | Y |  | 81 |
| 1144 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 83 |
| 1145 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 85 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1146 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 1147 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 1148 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 1149 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 1150 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 1151 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 1152 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 1153 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 1154 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 1155 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 1156 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 1157 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 1158 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 1159 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 1160 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 1161 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 1162 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 1163 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 1164 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 1165 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 1166 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 1167 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 70 |
| 1168 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 75 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1169 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 1170 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 1171 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 1172 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 1173 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 1174 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 1175 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 1176 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 1177 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 1178 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 1179 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 1180 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 1181 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 1182 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 1183 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 1184 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 1185 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 1186 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 1187 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 1188 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 1189 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 1190 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 70 |
| 1191 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 75 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1192 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 1193 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 1194 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 1195 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 1196 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 1197 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 1198 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 1199 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 1200 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 1201 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 1202 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 1203 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 1204 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 1205 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 1206 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 1207 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 1208 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 1209 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 1210 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 1211 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 1212 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 1213 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 70 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1214 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 1215 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 1216 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 1217 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 1218 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 1219 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 1220 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 1221 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 1222 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 1223 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 1224 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 1225 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 1226 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 1227 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 1228 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 1229 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 1230 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 1231 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 1232 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 1233 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 1234 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 1235 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 71 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1236 | `new_splay_link_lock` |  |  |  | Y | Y |  |  | hole | 351 |
| 1237 | `new_node` |  |  |  | Y |  | Y | Y |  | 24&#8209;31 |
| 1238 | `size_link` |  |  |  | Y |  | Y | Y |  | 33 |
| 1239 | `update` |  |  |  | Y |  | Y | Y |  | 35 |
| 1240 | `splay` |  |  |  | Y |  | Y | Y |  | 37&#8209;148 |
| 1241 | `bst_insert` |  |  |  | Y |  | Y | Y |  | 150&#8209;168 |
| 1242 | `insert_link` |  |  |  | Y |  | Y | Y |  | 170&#8209;179 |
| 1243 | `find_link` |  |  |  | Y |  | Y | Y |  | 181&#8209;194 |
| 1244 | `min_link` |  |  |  | Y |  | Y | Y |  | 196&#8209;204 |
| 1245 | `max_link` |  |  |  | Y |  | Y | Y |  | 206&#8209;214 |
| 1246 | `in_order_collect` |  |  |  | Y |  | Y | Y |  | 216&#8209;222 |
| 1247 | `pre_order_collect` |  |  |  | Y |  | Y | Y |  | 224&#8209;230 |
| 1248 | `in_order_parallel` |  |  |  | Y |  | Y | Y |  | 234&#8209;249 |
| 1249 | `pre_order_parallel` |  |  |  | Y |  | Y | Y |  | 251&#8209;266 |
| 1250 | `build_balanced` |  |  |  | Y |  | Y | Y |  | 268&#8209;286 |
| 1251 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 288&#8209;313 |
| 1252 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 315&#8209;337 |
| 1253 | `new` | Y | Y |  |  |  | Y | Y |  | 370&#8209;371 |
| 1254 | `from_sorted_slice` | Y | Y |  |  |  | Y | Y |  | 372&#8209;373 |
| 1255 | `insert` | Y | Y |  |  |  | Y | Y |  | 374&#8209;375 |
| 1256 | `find` | Y | Y |  |  |  | Y | Y |  | 376&#8209;377 |
| 1257 | `contains` | Y | Y |  |  |  | Y | Y |  | 378&#8209;379 |
| 1258 | `size` | Y | Y |  |  |  | Y | Y |  | 380&#8209;381 |
| 1259 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 382&#8209;383 |
| 1260 | `height` | Y | Y |  |  |  | Y | Y |  | 384&#8209;385 |
| 1261 | `minimum` | Y | Y |  |  |  | Y | Y |  | 386&#8209;387 |
| 1262 | `maximum` | Y | Y |  |  |  | Y | Y |  | 388&#8209;389 |
| 1263 | `in_order` | Y | Y |  |  |  | Y | Y |  | 390&#8209;391 |
| 1264 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 392&#8209;393 |
| 1265 | `filter` | Y | Y |  |  |  | Y | Y |  | 394&#8209;397 |
| 1266 | `reduce` | Y | Y |  |  |  | Y | Y |  | 398&#8209;401 |
| 1267 | `height_rec` |  | Y |  |  |  | Y | Y |  | 436&#8209;441 |
| 1268 | `default` |  | Y |  |  |  | Y | Y |  | 506 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1269 | `new_node` |  |  |  | Y | Y |  | Y |  | 36 |
| 1270 | `size_link` |  |  |  | Y | Y |  | Y |  | 54 |
| 1271 | `height_link` |  |  |  | Y | Y |  |  | hole | 61&#8209;63 |
| 1272 | `update` |  |  |  | Y | Y |  |  | hole | 84 |
| 1273 | `splay` |  |  |  | Y | Y |  | Y |  | 90&#8209;91 |
| 1274 | `bst_insert` |  |  |  | Y | Y |  |  | hole | 212 |
| 1275 | `insert_link` |  |  |  | Y | Y |  |  | hole | 233&#8209;234 |
| 1276 | `find_link` |  |  |  | Y | Y |  | Y |  | 246&#8209;247 |
| 1277 | `min_link` |  |  |  | Y | Y |  | Y |  | 263&#8209;264 |
| 1278 | `max_link` |  |  |  | Y | Y |  | Y |  | 275&#8209;276 |
| 1279 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 287&#8209;288 |
| 1280 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 297&#8209;298 |
| 1281 | `new` | Y | Y |  |  | Y |  | Y |  | 325&#8209;327 |
| 1282 | `size` | Y | Y |  |  | Y |  | Y |  | 329 |
| 1283 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 331 |
| 1284 | `height` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;334 |
| 1285 | `insert` | Y | Y |  |  | Y |  | Y |  | 336 |
| 1286 | `find` | Y | Y |  |  | Y |  | Y |  | 338 |
| 1287 | `contains` | Y | Y |  |  | Y |  | Y |  | 340 |
| 1288 | `minimum` | Y | Y |  |  | Y |  | Y |  | 342 |
| 1289 | `maximum` | Y | Y |  |  | Y |  | Y |  | 344 |
| 1290 | `in_order` | Y | Y |  |  | Y |  |  | hole | 346 |
| 1291 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 348 |
| 1292 | `default` |  | Y |  |  | Y |  | Y |  | 390 |

### Chap38/BSTParaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1293 | `new_bst_para_lock` |  |  |  | Y | Y |  |  | hole | 42 |
| 1294 | `new_leaf` |  |  |  | Y |  | Y | Y |  | 80&#8209;82 |
| 1295 | `new` | Y | Y |  |  |  | Y | Y |  | 85&#8209;86 |
| 1296 | `expose` | Y | Y |  |  |  | Y | Y |  | 87&#8209;88 |
| 1297 | `join_mid` | Y | Y |  | Y |  | Y | Y |  | 89&#8209;90 |
| 1298 | `size` | Y | Y |  |  |  | Y | Y |  | 91&#8209;92 |
| 1299 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 93&#8209;94 |
| 1300 | `insert` | Y | Y |  |  |  | Y | Y |  | 95&#8209;96 |
| 1301 | `delete` | Y | Y |  |  |  | Y | Y |  | 97&#8209;98 |
| 1302 | `find` | Y | Y |  |  |  | Y | Y |  | 99&#8209;100 |
| 1303 | `split` | Y | Y |  |  |  | Y | Y |  | 101&#8209;102 |
| 1304 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 103&#8209;104 |
| 1305 | `union` | Y | Y |  |  |  | Y | Y |  | 105&#8209;106 |
| 1306 | `intersect` | Y | Y |  |  |  | Y | Y |  | 107&#8209;108 |
| 1307 | `difference` | Y | Y |  |  |  | Y | Y |  | 109&#8209;110 |
| 1308 | `filter` | Y | Y |  |  |  | Y | Y |  | 111&#8209;112 |
| 1309 | `reduce` | Y | Y |  |  |  | Y | Y |  | 113&#8209;114 |
| 1310 | `in_order` | Y | Y |  |  |  | Y | Y |  | 115&#8209;116 |
| 1311 | `expose_internal` |  |  |  | Y |  | Y | Y |  | 119&#8209;127 |
| 1312 | `split_inner` |  |  |  | Y |  | Y | Y |  | 141&#8209;158 |
| 1313 | `join_m` |  |  |  | Y |  | Y | Y |  | 160&#8209;162 |
| 1314 | `min_key` |  |  |  | Y |  | Y | Y |  | 164&#8209;172 |
| 1315 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 174&#8209;183 |
| 1316 | `union_inner` |  |  |  | Y |  | Y | Y |  | 185&#8209;196 |
| 1317 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 198&#8209;212 |
| 1318 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 214&#8209;230 |
| 1319 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 232&#8209;252 |
| 1320 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 254&#8209;260 |
| 1321 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 262&#8209;283 |
| 1322 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 285&#8209;292 |
| 1323 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 294&#8209;303 |

### Chap38/BSTParaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1324 | `new_bst_para_lock` |  |  |  | Y | Y |  |  | hole | 102 |
| 1325 | `new` | Y | Y |  |  |  | Y | Y |  | 109&#8209;111 |
| 1326 | `expose` | Y | Y |  |  |  | Y | Y |  | 112&#8209;114 |
| 1327 | `join_mid` | Y | Y |  | Y |  | Y | Y |  | 115&#8209;117 |
| 1328 | `size` | Y | Y |  |  |  | Y | Y |  | 118&#8209;120 |
| 1329 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 121&#8209;123 |
| 1330 | `insert` | Y | Y |  |  |  | Y | Y |  | 124&#8209;126 |
| 1331 | `delete` | Y | Y |  |  |  | Y | Y |  | 127&#8209;129 |
| 1332 | `find` | Y | Y |  |  |  | Y | Y |  | 130&#8209;132 |
| 1333 | `split` | Y | Y |  |  |  | Y | Y |  | 133&#8209;135 |
| 1334 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 136&#8209;138 |
| 1335 | `union` | Y | Y |  |  |  | Y | Y |  | 139&#8209;141 |
| 1336 | `in_order` | Y | Y |  |  |  | Y | Y |  | 142&#8209;144 |
| 1337 | `new_leaf` |  |  |  | Y |  | Y | Y |  | 147&#8209;149 |
| 1338 | `expose_internal` |  |  |  | Y |  | Y | Y |  | 151&#8209;160 |
| 1339 | `split_inner` |  |  |  | Y |  | Y | Y |  | 175&#8209;194 |
| 1340 | `join_m` |  |  |  | Y |  | Y | Y |  | 196&#8209;200 |
| 1341 | `min_key` |  |  |  | Y |  | Y | Y |  | 202&#8209;212 |
| 1342 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 214&#8209;225 |
| 1343 | `union_inner` |  |  |  | Y |  | Y | Y |  | 227&#8209;240 |
| 1344 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 242&#8209;253 |

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1345 | `new_treap_lock` |  |  |  | Y | Y |  |  | hole | 48 |
| 1346 | `priority_for` |  |  |  | Y |  | Y | Y |  | 85&#8209;93 |
| 1347 | `tree_priority` |  |  |  | Y |  | Y | Y |  | 95&#8209;102 |
| 1348 | `tree_size` |  |  |  | Y |  | Y | Y |  | 104&#8209;111 |
| 1349 | `make_node` |  |  |  | Y |  | Y | Y |  | 113&#8209;122 |
| 1350 | `join_with_priority` |  |  |  | Y |  | Y | Y |  | 124&#8209;146 |
| 1351 | `split_inner` |  |  |  | Y |  | Y | Y |  | 148&#8209;168 |
| 1352 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 170&#8209;183 |
| 1353 | `union_inner` |  |  |  | Y |  | Y | Y |  | 185&#8209;198 |
| 1354 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 200&#8209;217 |
| 1355 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 219&#8209;236 |
| 1356 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 238&#8209;256 |
| 1357 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 258&#8209;264 |
| 1358 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 266&#8209;287 |
| 1359 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 289&#8209;296 |
| 1360 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 298&#8209;310 |
| 1361 | `new` | Y | Y |  |  |  | Y | Y |  | 315&#8209;317 |
| 1362 | `expose` | Y | Y |  |  |  | Y | Y |  | 318&#8209;320 |
| 1363 | `expose_with_priority` | Y | Y |  |  |  | Y | Y |  | 321&#8209;323 |
| 1364 | `join_mid` | Y | Y |  |  |  | Y | Y |  | 324&#8209;326 |
| 1365 | `size` | Y | Y |  |  |  | Y | Y |  | 327&#8209;329 |
| 1366 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 330&#8209;332 |
| 1367 | `insert` | Y | Y |  |  |  | Y | Y |  | 333&#8209;335 |
| 1368 | `delete` | Y | Y |  |  |  | Y | Y |  | 336&#8209;338 |
| 1369 | `find` | Y | Y |  |  |  | Y | Y |  | 339&#8209;341 |
| 1370 | `split` | Y | Y |  |  |  | Y | Y |  | 342&#8209;344 |
| 1371 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 345&#8209;347 |
| 1372 | `union` | Y | Y |  |  |  | Y | Y |  | 348&#8209;350 |
| 1373 | `intersect` | Y | Y |  |  |  | Y | Y |  | 351&#8209;353 |
| 1374 | `difference` | Y | Y |  |  |  | Y | Y |  | 354&#8209;356 |
| 1375 | `filter` | Y | Y |  |  |  | Y | Y |  | 357&#8209;359 |
| 1376 | `reduce` | Y | Y |  |  |  | Y | Y |  | 360&#8209;364 |
| 1377 | `in_order` | Y | Y |  |  |  | Y | Y |  | 365&#8209;367 |

### Chap39/BSTSetTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1378 | `priority_for` |  |  |  | Y | Y |  |  | hole | 40 |
| 1379 | `empty` | Y | Y |  |  | Y |  | Y |  | 64 |
| 1380 | `singleton` | Y | Y |  |  | Y |  | Y |  | 67 |
| 1381 | `size` | Y | Y |  |  | Y |  | Y |  | 70 |
| 1382 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 73 |
| 1383 | `find` | Y | Y |  |  | Y |  | Y |  | 76 |
| 1384 | `contains` | Y | Y |  |  | Y |  | Y |  | 79 |
| 1385 | `minimum` | Y | Y |  |  | Y |  | Y |  | 82 |
| 1386 | `maximum` | Y | Y |  |  | Y |  | Y |  | 85 |
| 1387 | `insert` | Y | Y |  |  | Y |  | Y |  | 88 |
| 1388 | `delete` | Y | Y |  |  | Y |  |  | hole | 91 |
| 1389 | `union` | Y | Y |  |  | Y |  |  | hole | 94 |
| 1390 | `intersection` | Y | Y |  |  | Y |  |  | hole | 97 |
| 1391 | `difference` | Y | Y |  |  | Y |  |  | hole | 100 |
| 1392 | `split` | Y | Y |  |  | Y |  |  | hole | 103 |
| 1393 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 106 |
| 1394 | `join_m` | Y | Y |  |  | Y |  |  | hole | 109 |
| 1395 | `filter` | Y | Y |  |  | Y |  |  | hole | 112 |
| 1396 | `reduce` | Y | Y |  |  | Y |  |  | hole | 115 |
| 1397 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 118 |
| 1398 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 121 |
| 1399 | `values_vec` |  |  |  | Y | Y |  |  | hole | 125 |
| 1400 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 129 |
| 1401 | `from_sorted_iter` |  |  |  | Y | Y |  |  | hole | 139 |

### Chap39/BSTTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1402 | `new` x3 | Y | Y |  |  | Y |  | Y |  | 115 |
| 1403 | `clone_link` |  |  |  | Y | Y |  |  | hole | 44&#8209;45 |
| 1404 | `new_treap_link_lock` |  |  |  | Y | Y |  |  | hole | 92 |
| 1405 | `insert` | Y | Y |  |  | Y |  | Y |  | 118 |
| 1406 | `find` | Y | Y |  |  | Y |  | Y |  | 121 |
| 1407 | `contains` | Y | Y |  |  | Y |  | Y |  | 124 |
| 1408 | `size` | Y | Y |  |  | Y |  | Y |  | 127 |
| 1409 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 130 |
| 1410 | `height` | Y | Y |  |  | Y |  | Y |  | 133 |
| 1411 | `minimum` | Y | Y |  |  | Y |  | Y |  | 136 |
| 1412 | `maximum` | Y | Y |  |  | Y |  | Y |  | 139 |
| 1413 | `in_order` | Y | Y |  |  | Y |  | Y |  | 142 |
| 1414 | `pre_order` | Y | Y |  |  | Y |  | Y |  | 145 |
| 1415 | `size_link` |  |  |  | Y | Y |  | Y |  | 151 |
| 1416 | `update` |  |  |  | Y | Y |  |  | hole | 161 |
| 1417 | `rotate_left` |  |  |  | Y | Y |  | Y |  | 167 |
| 1418 | `rotate_right` |  |  |  | Y | Y |  | Y |  | 183 |
| 1419 | `insert_link` |  |  |  | Y | Y |  | Y |  | 199&#8209;200 |
| 1420 | `find_link` |  |  |  | Y | Y |  | Y |  | 237&#8209;238 |
| 1421 | `min_link` |  |  |  | Y | Y |  | Y |  | 256&#8209;257 |
| 1422 | `max_link` |  |  |  | Y | Y |  | Y |  | 270&#8209;271 |
| 1423 | `height_link` |  |  |  | Y | Y |  |  | hole | 283 |
| 1424 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 292&#8209;293 |
| 1425 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 304&#8209;305 |
| 1426 | `default` |  | Y |  |  | Y |  | Y |  | 384 |

### Chap39/BSTTreapStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1427 | `new_node` |  |  |  | Y | Y |  | Y |  | 40 |
| 1428 | `new` | Y | Y |  |  | Y |  | Y |  | 69&#8209;71 |
| 1429 | `size` | Y | Y |  |  | Y |  | Y |  | 74 |
| 1430 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 77 |
| 1431 | `height` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 1432 | `insert` | Y | Y |  |  | Y |  | Y |  | 84 |
| 1433 | `find` | Y | Y |  |  | Y |  | Y |  | 87 |
| 1434 | `contains` | Y | Y |  |  | Y |  | Y |  | 90 |
| 1435 | `minimum` | Y | Y |  |  | Y |  | Y |  | 93 |
| 1436 | `maximum` | Y | Y |  |  | Y |  | Y |  | 96 |
| 1437 | `in_order` | Y | Y |  |  | Y |  |  | hole | 99 |
| 1438 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 102 |
| 1439 | `size_link` |  |  |  | Y | Y |  |  | hole | 117 |
| 1440 | `height_link` |  |  |  | Y | Y |  |  | hole | 119&#8209;121 |
| 1441 | `update` |  |  |  | Y | Y |  |  | hole | 142 |
| 1442 | `rotate_left` |  |  |  | Y | Y |  | Y |  | 148 |
| 1443 | `rotate_right` |  |  |  | Y | Y |  | Y |  | 164 |
| 1444 | `insert_link` |  |  |  | Y | Y |  |  | hole | 181&#8209;182 |
| 1445 | `find_link` |  |  |  | Y | Y |  |  | hole | 207&#8209;208 |
| 1446 | `min_link` |  |  |  | Y | Y |  | Y |  | 226&#8209;227 |
| 1447 | `max_link` |  |  |  | Y | Y |  | Y |  | 240&#8209;241 |
| 1448 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 254&#8209;255 |
| 1449 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 266&#8209;267 |
| 1450 | `default` |  | Y |  |  | Y |  | Y |  | 317 |

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1451 | `new_node` |  |  |  | Y | Y |  | Y |  | 40 |
| 1452 | `new` | Y | Y |  |  | Y |  | Y |  | 70&#8209;72 |
| 1453 | `size` | Y | Y |  |  | Y |  | Y |  | 74 |
| 1454 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 76 |
| 1455 | `height` | Y | Y |  |  | Y |  | Y |  | 78 |
| 1456 | `insert` | Y | Y |  |  | Y |  |  | hole | 80 |
| 1457 | `delete` | Y | Y |  |  | Y |  |  | hole | 82 |
| 1458 | `find` | Y | Y |  |  | Y |  | Y |  | 84 |
| 1459 | `contains` | Y | Y |  |  | Y |  | Y |  | 86 |
| 1460 | `get` | Y | Y |  |  | Y |  | Y |  | 88 |
| 1461 | `keys` | Y | Y |  |  | Y |  |  | hole | 90 |
| 1462 | `values` | Y | Y |  |  | Y |  |  | hole | 92 |
| 1463 | `minimum_key` | Y | Y |  |  | Y |  | Y |  | 95 |
| 1464 | `maximum_key` | Y | Y |  |  | Y |  | Y |  | 98 |
| 1465 | `height_link` |  |  |  | Y | Y |  |  | hole | 101&#8209;102 |
| 1466 | `rotate_left` |  |  |  | Y | Y |  | Y |  | 116 |
| 1467 | `rotate_right` |  |  |  | Y | Y |  | Y |  | 130 |
| 1468 | `insert_link` |  |  |  | Y | Y |  |  | hole | 145&#8209;146 |
| 1469 | `find_link` |  |  |  | Y | Y |  |  | hole | 174 |
| 1470 | `min_key_link` |  |  |  | Y | Y |  | Y |  | 191&#8209;192 |
| 1471 | `max_key_link` |  |  |  | Y | Y |  | Y |  | 205&#8209;206 |
| 1472 | `collect_keys` |  |  |  | Y | Y |  | Y |  | 219&#8209;220 |
| 1473 | `collect_values` |  |  |  | Y | Y |  | Y |  | 231&#8209;232 |
| 1474 | `collect_in_order_kvp` |  |  |  | Y | Y |  | Y |  | 242&#8209;243 |
| 1475 | `build_treap_from_sorted` |  |  |  | Y | Y |  |  | hole | 254&#8209;256 |
| 1476 | `default` |  | Y |  |  | Y |  | Y |  | 329 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1477 | `new` x3 | Y | Y |  |  | Y |  | Y |  | 117&#8209;119 |
| 1478 | `identity` x2 | Y | Y |  |  | Y |  | Y |  | 64 |
| 1479 | `combine` x2 | Y | Y |  |  | Y |  |  | hole | 66 |
| 1480 | `lift` x2 | Y | Y |  |  | Y |  | Y |  | 68 |
| 1481 | `size` | Y | Y |  |  | Y |  | Y |  | 121 |
| 1482 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 123 |
| 1483 | `height` | Y | Y |  |  | Y |  | Y |  | 125 |
| 1484 | `insert` | Y | Y |  |  | Y |  | Y |  | 127 |
| 1485 | `delete` | Y | Y |  |  | Y |  |  | hole | 129 |
| 1486 | `find` | Y | Y |  |  | Y |  | Y |  | 131 |
| 1487 | `contains` | Y | Y |  |  | Y |  | Y |  | 133 |
| 1488 | `get` | Y | Y |  |  | Y |  | Y |  | 135 |
| 1489 | `keys` | Y | Y |  |  | Y |  |  | hole | 137 |
| 1490 | `values` | Y | Y |  |  | Y |  |  | hole | 139 |
| 1491 | `minimum_key` | Y | Y |  |  | Y |  | Y |  | 142 |
| 1492 | `maximum_key` | Y | Y |  |  | Y |  | Y |  | 145 |
| 1493 | `reduced_value` | Y | Y |  |  | Y |  | Y |  | 148 |
| 1494 | `range_reduce` | Y | Y |  |  | Y |  | Y |  | 151 |
| 1495 | `default` |  | Y |  |  | Y |  | Y |  | 155 |
| 1496 | `size_link` |  |  |  | Y | Y |  | Y |  | 160 |
| 1497 | `reduced_value_link` |  |  |  | Y | Y |  |  | hole | 170 |
| 1498 | `update_node` |  |  |  | Y | Y |  |  | hole | 180 |
| 1499 | `make_node` |  |  |  | Y | Y |  | Y |  | 193&#8209;199 |
| 1500 | `rotate_left` |  |  |  | Y | Y |  | Y |  | 210 |
| 1501 | `rotate_right` |  |  |  | Y | Y |  | Y |  | 226 |
| 1502 | `insert_link` |  |  |  | Y | Y |  |  | hole | 243&#8209;248 |
| 1503 | `find_link` |  |  |  | Y | Y |  | Y |  | 274&#8209;278 |
| 1504 | `min_key_link` |  |  |  | Y | Y |  | Y |  | 296&#8209;297 |
| 1505 | `max_key_link` |  |  |  | Y | Y |  | Y |  | 310&#8209;311 |
| 1506 | `collect_keys` |  |  |  | Y | Y |  | Y |  | 324&#8209;325 |
| 1507 | `collect_values` |  |  |  | Y | Y |  | Y |  | 336&#8209;337 |
| 1508 | `collect_in_order_kvp` |  |  |  | Y | Y |  | Y |  | 347&#8209;351 |
| 1509 | `height_link` |  |  |  | Y | Y |  |  | hole | 360&#8209;361 |
| 1510 | `build_treap_from_sorted` |  |  |  | Y | Y |  |  | hole | 375&#8209;377 |
| 1511 | `range_reduce_link` |  |  |  | Y | Y |  |  | hole | 398&#8209;402 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1512 | `new` x3 | Y | Y |  |  | Y |  | Y |  | 70&#8209;72 |
| 1513 | `size` | Y | Y |  |  | Y |  | Y |  | 74 |
| 1514 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 76 |
| 1515 | `height` | Y | Y |  |  | Y |  | Y |  | 78 |
| 1516 | `insert` | Y | Y |  |  | Y |  | Y |  | 80 |
| 1517 | `delete` | Y | Y |  |  | Y |  |  | hole | 82 |
| 1518 | `find` | Y | Y |  |  | Y |  | Y |  | 84 |
| 1519 | `contains` | Y | Y |  |  | Y |  | Y |  | 86 |
| 1520 | `minimum` | Y | Y |  |  | Y |  | Y |  | 88 |
| 1521 | `maximum` | Y | Y |  |  | Y |  | Y |  | 90 |
| 1522 | `in_order` | Y | Y |  |  | Y |  |  | hole | 92 |
| 1523 | `rank` | Y | Y |  |  | Y |  | Y |  | 94 |
| 1524 | `select` | Y | Y |  |  | Y |  | Y |  | 97 |
| 1525 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 100 |
| 1526 | `height_link` |  |  |  | Y | Y |  |  | hole | 103&#8209;104 |
| 1527 | `size_link` |  |  |  | Y | Y |  | Y |  | 118 |
| 1528 | `update_size` |  |  |  | Y | Y |  |  | hole | 128 |
| 1529 | `make_node` |  |  |  | Y | Y |  | Y |  | 134 |
| 1530 | `rotate_left` |  |  |  | Y | Y |  | Y |  | 144 |
| 1531 | `rotate_right` |  |  |  | Y | Y |  | Y |  | 160 |
| 1532 | `insert_link` |  |  |  | Y | Y |  |  | hole | 177&#8209;178 |
| 1533 | `find_link` |  |  |  | Y | Y |  | Y |  | 202&#8209;203 |
| 1534 | `min_link` |  |  |  | Y | Y |  | Y |  | 221&#8209;222 |
| 1535 | `max_link` |  |  |  | Y | Y |  | Y |  | 235&#8209;236 |
| 1536 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 249&#8209;250 |
| 1537 | `in_order_collect_with_priority` |  |  |  | Y | Y |  | Y |  | 261&#8209;265 |
| 1538 | `build_treap_from_sorted` |  |  |  | Y | Y |  |  | hole | 277 |
| 1539 | `rank_link` |  |  |  | Y | Y |  |  | hole | 298&#8209;299 |
| 1540 | `select_link` |  |  |  | Y | Y |  | Y |  | 318&#8209;319 |
| 1541 | `default` |  | Y |  |  | Y |  | Y |  | 405 |

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1542 | `new_set_mt_lock` |  |  |  | Y | Y |  |  | hole | 53 |
| 1543 | `size` | Y | Y |  |  | Y |  |  | hole | 80&#8209;81 |
| 1544 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 1545 | `empty` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 1546 | `singleton` | Y | Y |  |  | Y |  |  | hole | 92&#8209;93 |
| 1547 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 1548 | `filter` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 1549 | `intersection` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 1550 | `difference` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 1551 | `union` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 1552 | `find` | Y | Y |  |  | Y |  |  | hole | 115&#8209;116 |
| 1553 | `delete` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 1554 | `insert` | Y | Y |  |  | Y |  |  | hole | 123&#8209;124 |
| 1555 | `default` |  | Y |  |  |  | Y | Y |  | 401 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1556 | `size` | Y | Y |  |  | Y |  |  | hole | 69&#8209;70 |
| 1557 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 73&#8209;74 |
| 1558 | `empty` | Y | Y |  |  | Y |  |  | hole | 77&#8209;78 |
| 1559 | `singleton` | Y | Y |  |  | Y |  |  | hole | 81&#8209;82 |
| 1560 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 1561 | `filter` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 1562 | `intersection` | Y | Y |  |  | Y |  |  | hole | 92&#8209;93 |
| 1563 | `difference` | Y | Y |  |  | Y |  |  | hole | 96&#8209;97 |
| 1564 | `union` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 1565 | `find` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 1566 | `delete` | Y | Y |  |  | Y |  |  | hole | 108&#8209;109 |
| 1567 | `insert` | Y | Y |  |  | Y |  |  | hole | 112&#8209;113 |
| 1568 | `default` |  | Y |  |  |  | Y | Y |  | 467 |
| 1569 | `eq` |  | Y |  |  |  | Y | Y |  | 471&#8209;480 |
| 1570 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 486&#8209;488 |
| 1571 | `cmp` |  | Y |  |  |  | Y | Y |  | 492&#8209;510 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1572 | `size` | Y | Y |  |  | Y |  |  | hole | 59&#8209;60 |
| 1573 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 63&#8209;64 |
| 1574 | `empty` | Y | Y |  |  | Y |  |  | hole | 67&#8209;68 |
| 1575 | `singleton` | Y | Y |  |  | Y |  |  | hole | 71&#8209;72 |
| 1576 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 74&#8209;75 |
| 1577 | `filter` | Y | Y |  |  | Y |  |  | hole | 78&#8209;79 |
| 1578 | `intersection` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 1579 | `difference` | Y | Y |  |  | Y |  |  | hole | 86&#8209;87 |
| 1580 | `union` | Y | Y |  |  | Y |  |  | hole | 90&#8209;91 |
| 1581 | `find` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 1582 | `delete` | Y | Y |  |  | Y |  |  | hole | 98&#8209;99 |
| 1583 | `insert` | Y | Y |  |  | Y |  |  | hole | 102&#8209;103 |
| 1584 | `default` |  | Y |  |  |  | Y | Y |  | 435 |
| 1585 | `eq` |  | Y |  |  |  | Y | Y |  | 439&#8209;448 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1586 | `size` | Y | Y |  |  | Y |  |  | hole | 57&#8209;58 |
| 1587 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 61&#8209;62 |
| 1588 | `empty` | Y | Y |  |  | Y |  |  | hole | 65&#8209;66 |
| 1589 | `singleton` | Y | Y |  |  | Y |  |  | hole | 69&#8209;70 |
| 1590 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 72&#8209;73 |
| 1591 | `filter` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 1592 | `intersection` | Y | Y |  |  | Y |  |  | hole | 80&#8209;81 |
| 1593 | `difference` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 1594 | `union` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 1595 | `find` | Y | Y |  |  | Y |  |  | hole | 92&#8209;93 |
| 1596 | `delete` | Y | Y |  |  | Y |  |  | hole | 96&#8209;97 |
| 1597 | `insert` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 1598 | `default` |  | Y |  |  |  | Y | Y |  | 441 |
| 1599 | `eq` |  | Y |  |  |  | Y | Y |  | 445&#8209;454 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1600 | `new` | Y | Y |  |  | Y |  |  | hole | 48&#8209;49 |
| 1601 | `size` | Y | Y |  |  | Y |  |  | hole | 52&#8209;53 |
| 1602 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 56&#8209;57 |
| 1603 | `empty` | Y | Y |  |  | Y |  |  | hole | 59&#8209;60 |
| 1604 | `singleton` | Y | Y |  |  | Y |  |  | hole | 63&#8209;67 |
| 1605 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 69&#8209;70 |
| 1606 | `filter` | Y | Y |  |  | Y |  |  | hole | 73&#8209;74 |
| 1607 | `intersection` | Y | Y |  |  | Y |  |  | hole | 77&#8209;78 |
| 1608 | `difference` | Y | Y |  |  | Y |  |  | hole | 81&#8209;82 |
| 1609 | `union` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 1610 | `find` | Y | Y |  |  | Y |  |  | hole | 89&#8209;90 |
| 1611 | `delete` | Y | Y |  |  | Y |  |  | hole | 93&#8209;94 |
| 1612 | `insert` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 1613 | `eq` |  | Y |  |  |  | Y | Y |  | 292&#8209;301 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1614 | `lemma_filter_remove` |  |  |  | Y | Y |  |  | hole | 74&#8209;77 |
| 1615 | `lemma_push_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 97&#8209;102 |
| 1616 | `lemma_subseq_no_dups_subset` |  |  |  | Y | Y |  |  | unknown | 123&#8209;129 |
| 1617 | `size` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;148 |
| 1618 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 1619 | `empty` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;154 |
| 1620 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;157 |
| 1621 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 1622 | `filter` | Y | Y |  |  | Y |  |  | hole | 162&#8209;164 |
| 1623 | `intersection` | Y | Y |  |  | Y |  |  | hole | 166&#8209;168 |
| 1624 | `difference` | Y | Y |  |  | Y |  |  | hole | 170&#8209;172 |
| 1625 | `union` | Y | Y |  |  | Y |  |  | hole | 174&#8209;176 |
| 1626 | `find` | Y | Y |  |  | Y |  |  | hole | 178&#8209;179 |
| 1627 | `delete` | Y | Y |  |  | Y |  |  | hole | 181&#8209;183 |
| 1628 | `insert` | Y | Y |  |  | Y |  |  | hole | 185&#8209;187 |
| 1629 | `default` |  | Y |  |  |  | Y | Y |  | 492 |
| 1630 | `eq` |  | Y |  |  |  | Y | Y |  | 496&#8209;505 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1631 | `example_41_1_array_set` | Y | Y |  | Y | Y |  | Y |  | 21 |
| 1632 | `example_41_1_avl_set` | Y | Y |  | Y | Y |  | Y |  | 25 |
| 1633 | `demonstrate_set_operations` | Y | Y |  |  | Y |  | Y |  | 29 |
| 1634 | `example_41_1_array_set_impl` |  |  |  | Y | Y |  |  | hole | 33&#8209;34 |
| 1635 | `example_41_1_avl_set_impl` |  |  |  | Y | Y |  |  | hole | 86&#8209;87 |
| 1636 | `example_41_3_from_seq_demonstration_impl` |  |  |  | Y | Y |  |  | hole | 139&#8209;140 |
| 1637 | `additional_set_operations_impl` |  |  |  | Y | Y |  |  | hole | 188&#8209;189 |
| 1638 | `example_41_3_from_seq_demonstration` |  |  |  | Y | Y |  | Y |  | 246 |
| 1639 | `additional_set_operations` |  |  |  | Y | Y |  | Y |  | 247 |

### Chap42/Example42_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1640 | `_example_42_1_verified` |  |  |  | Y | Y |  | Y |  | 11 |
| 1641 | `example_42_1` | Y |  |  | Y |  | Y | Y |  | 21&#8209;23 |
| 1642 | `demonstrate_table_operations` | Y |  |  |  |  | Y | Y |  | 25&#8209;27 |
| 1643 | `performance_comparison` |  |  |  | Y |  | Y | Y |  | 151&#8209;202 |

### Chap42/TableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1644 | `size` | Y | Y |  |  | Y |  |  | hole | 64&#8209;65 |
| 1645 | `empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 1646 | `singleton` | Y | Y |  |  | Y |  |  | hole | 70&#8209;71 |
| 1647 | `domain` | Y | Y |  |  | Y |  |  | hole | 73&#8209;74 |
| 1648 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 1649 | `map` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 1650 | `filter` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 1651 | `intersection` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 1652 | `union` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 1653 | `difference` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 1654 | `find` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 1655 | `delete` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 1656 | `insert` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 1657 | `restrict` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 1658 | `subtract` | Y | Y |  |  | Y |  |  | hole | 106&#8209;107 |
| 1659 | `entries` | Y | Y |  |  | Y |  | Y |  | 109 |
| 1660 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 719&#8209;720 |
| 1661 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 729&#8209;731 |
| 1662 | `eq` |  | Y |  |  |  | Y | Y |  | 743&#8209;745 |

### Chap42/TableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1663 | `size` | Y | Y |  |  | Y |  |  | hole | 64&#8209;65 |
| 1664 | `empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 1665 | `singleton` | Y | Y |  |  | Y |  |  | hole | 70&#8209;71 |
| 1666 | `domain` | Y | Y |  |  | Y |  |  | hole | 73&#8209;74 |
| 1667 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 1668 | `map` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 1669 | `filter` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 1670 | `intersection` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 1671 | `union` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 1672 | `difference` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 1673 | `find` | Y | Y |  |  | Y |  |  | hole | 94&#8209;99 |
| 1674 | `delete` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 1675 | `insert` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 1676 | `restrict` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 1677 | `subtract` | Y | Y |  |  | Y |  |  | hole | 110&#8209;111 |
| 1678 | `entries` | Y | Y |  |  | Y |  | Y |  | 114 |
| 1679 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 414&#8209;417 |
| 1680 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 426&#8209;428 |
| 1681 | `default` |  | Y |  |  |  | Y | Y |  | 454&#8209;456 |
| 1682 | `eq` |  | Y |  |  |  | Y | Y |  | 460&#8209;462 |

### Chap42/TableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1683 | `eq` |  | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 1684 | `size` | Y | Y |  |  |  | Y | Y |  | 55&#8209;57 |
| 1685 | `empty` | Y | Y |  |  |  | Y | Y |  | 59&#8209;61 |
| 1686 | `singleton` | Y | Y |  |  |  | Y | Y |  | 63&#8209;65 |
| 1687 | `domain` | Y | Y |  |  |  | Y | Y |  | 67&#8209;69 |
| 1688 | `tabulate` | Y | Y |  |  |  | Y | Y |  | 71&#8209;73 |
| 1689 | `map` | Y | Y |  |  |  | Y | Y |  | 75&#8209;77 |
| 1690 | `filter` | Y | Y |  |  |  | Y | Y |  | 79&#8209;81 |
| 1691 | `intersection` | Y | Y |  |  |  | Y | Y |  | 83&#8209;85 |
| 1692 | `union` | Y | Y |  |  |  | Y | Y |  | 87&#8209;89 |
| 1693 | `difference` | Y | Y |  |  |  | Y | Y |  | 91&#8209;93 |
| 1694 | `find` | Y | Y |  |  |  | Y | Y |  | 95&#8209;97 |
| 1695 | `delete` | Y | Y |  |  |  | Y | Y |  | 99&#8209;101 |
| 1696 | `insert` | Y | Y |  |  |  | Y | Y |  | 103&#8209;105 |
| 1697 | `restrict` | Y | Y |  |  |  | Y | Y |  | 107&#8209;109 |
| 1698 | `subtract` | Y | Y |  |  |  | Y | Y |  | 111&#8209;113 |
| 1699 | `collect` | Y | Y |  |  |  | Y | Y |  | 115&#8209;117 |
| 1700 | `from_sorted_entries` |  |  |  | Y |  | Y | Y |  | 395&#8209;400 |

### Chap43/AugOrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1701 | `recalculate_reduction` |  |  |  | Y | Y |  |  | hole | 60&#8209;63 |
| 1702 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 69&#8209;74 |
| 1703 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 99&#8209;102 |
| 1704 | `size` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 1705 | `empty` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 1706 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 1707 | `find` | Y | Y |  |  | Y |  | Y |  | 116 |
| 1708 | `lookup` | Y | Y |  |  | Y |  | Y |  | 117 |
| 1709 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 1710 | `insert` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 1711 | `delete` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 1712 | `domain` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 1713 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;132 |
| 1714 | `map` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 1715 | `filter` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;136 |
| 1716 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 1717 | `union` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 1718 | `difference` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;142 |
| 1719 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;144 |
| 1720 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;146 |
| 1721 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;148 |
| 1722 | `collect` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 1723 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 1724 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;154 |
| 1725 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;156 |
| 1726 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;158 |
| 1727 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 1728 | `join_key` | Y | Y |  |  | Y |  |  | hole | 162&#8209;163 |
| 1729 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 1730 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 1731 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;169 |
| 1732 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;172 |
| 1733 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;174 |
| 1734 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;176 |
| 1735 | `reduce_range_parallel` | Y | Y |  |  | Y |  |  | hole | 177&#8209;178 |
| 1736 | `eq` |  | Y |  |  |  | Y | Y |  | 572&#8209;575 |

### Chap43/AugOrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1737 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 60&#8209;67 |
| 1738 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 92&#8209;95 |
| 1739 | `size` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 1740 | `empty` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 1741 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 1742 | `find` | Y | Y |  |  | Y |  |  | hole | 112&#8209;117 |
| 1743 | `lookup` | Y | Y |  |  | Y |  |  | hole | 118&#8209;123 |
| 1744 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 1745 | `insert` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;127 |
| 1746 | `delete` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 1747 | `domain` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 1748 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;133 |
| 1749 | `map` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;135 |
| 1750 | `filter` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 1751 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;139 |
| 1752 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 1753 | `union` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;143 |
| 1754 | `difference` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;145 |
| 1755 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 1756 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;149 |
| 1757 | `collect` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 1758 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;153 |
| 1759 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;155 |
| 1760 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;157 |
| 1761 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;159 |
| 1762 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;162 |
| 1763 | `join_key` | Y | Y |  |  | Y |  |  | hole | 163&#8209;164 |
| 1764 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;166 |
| 1765 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 1766 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;170 |
| 1767 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;173 |
| 1768 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;175 |
| 1769 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;177 |
| 1770 | `eq` |  | Y |  |  |  | Y | Y |  | 557&#8209;560 |

### Chap43/AugOrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1771 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 60&#8209;67 |
| 1772 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 92&#8209;95 |
| 1773 | `size` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 1774 | `empty` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 1775 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 1776 | `find` | Y | Y |  |  | Y |  |  | hole | 112&#8209;117 |
| 1777 | `insert` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 1778 | `delete` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 1779 | `domain` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 1780 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 1781 | `map` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;127 |
| 1782 | `filter` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 1783 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 1784 | `union` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;133 |
| 1785 | `difference` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;135 |
| 1786 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 1787 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;139 |
| 1788 | `collect` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 1789 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;143 |
| 1790 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;145 |
| 1791 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 1792 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;149 |
| 1793 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 1794 | `join_key` | Y | Y |  |  | Y |  |  | hole | 153&#8209;154 |
| 1795 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;156 |
| 1796 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;158 |
| 1797 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 1798 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 1799 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 1800 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 1801 | `eq` |  | Y |  |  |  | Y | Y |  | 576&#8209;579 |

### Chap43/Example43_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1802 | `_example_43_1_verified` |  |  |  | Y | Y |  | Y |  | 11 |
| 1803 | `run_example43_1` | Y |  |  | Y |  | Y | Y |  | 19&#8209;21 |
| 1804 | `demonstrate_ordered_operations` | Y |  |  |  |  | Y | Y |  | 23&#8209;25 |
| 1805 | `run_integer_example` |  |  |  | Y |  | Y | Y |  | 174&#8209;230 |

### Chap43/OrderedSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1806 | `size` | Y | Y |  |  | Y |  |  | hole | 47&#8209;48 |
| 1807 | `empty` | Y | Y |  |  | Y |  |  | hole | 50&#8209;51 |
| 1808 | `singleton` | Y | Y |  |  | Y |  |  | hole | 53&#8209;54 |
| 1809 | `find` | Y | Y |  |  | Y |  |  | hole | 56&#8209;57 |
| 1810 | `insert` | Y | Y |  |  | Y |  |  | hole | 59&#8209;60 |
| 1811 | `delete` | Y | Y |  |  | Y |  |  | hole | 62&#8209;63 |
| 1812 | `filter` | Y | Y |  |  | Y |  |  | hole | 65&#8209;66 |
| 1813 | `intersection` | Y | Y |  |  | Y |  |  | hole | 68&#8209;69 |
| 1814 | `union` | Y | Y |  |  | Y |  |  | hole | 71&#8209;72 |
| 1815 | `difference` | Y | Y |  |  | Y |  |  | hole | 74&#8209;75 |
| 1816 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 77&#8209;78 |
| 1817 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 80&#8209;81 |
| 1818 | `first` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 1819 | `last` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 1820 | `previous` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 1821 | `next` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 1822 | `split` | Y | Y |  |  | Y |  |  | hole | 97&#8209;99 |
| 1823 | `join` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 1824 | `get_range` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 1825 | `rank` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 1826 | `select` | Y | Y |  |  | Y |  |  | hole | 110&#8209;111 |
| 1827 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 113&#8209;115 |

### Chap43/OrderedSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1828 | `size` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;51 |
| 1829 | `empty` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;54 |
| 1830 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;57 |
| 1831 | `find` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 1832 | `insert` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 1833 | `delete` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 1834 | `filter` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 1835 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 1836 | `union` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 1837 | `difference` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 1838 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 80&#8209;81 |
| 1839 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 83&#8209;84 |
| 1840 | `first` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 1841 | `last` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 1842 | `previous` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 1843 | `next` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 1844 | `split` | Y | Y |  |  | Y |  |  | hole | 100&#8209;102 |
| 1845 | `join` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 1846 | `get_range` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 1847 | `rank` | Y | Y |  |  | Y |  |  | hole | 110&#8209;111 |
| 1848 | `select` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 1849 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 116&#8209;118 |
| 1850 | `from_sorted_elements` |  |  |  | Y | Y |  |  | hole | 401&#8209;402 |
| 1851 | `default` |  | Y |  |  |  | Y | Y |  | 426 |
| 1852 | `eq` |  | Y |  |  |  | Y | Y |  | 430&#8209;440 |

### Chap43/OrderedSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1853 | `size` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;50 |
| 1854 | `empty` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;53 |
| 1855 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;56 |
| 1856 | `find` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;59 |
| 1857 | `insert` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 1858 | `delete` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 1859 | `filter` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 1860 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 1861 | `union` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 1862 | `difference` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 1863 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 1864 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;83 |
| 1865 | `first` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 1866 | `last` | Y | Y |  |  | Y |  |  | hole | 90&#8209;91 |
| 1867 | `previous` | Y | Y |  |  | Y |  |  | hole | 93&#8209;94 |
| 1868 | `next` | Y | Y |  |  | Y |  |  | hole | 96&#8209;97 |
| 1869 | `split` | Y | Y |  |  | Y |  |  | hole | 99&#8209;101 |
| 1870 | `join` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 1871 | `get_range` | Y | Y |  |  | Y |  |  | hole | 106&#8209;107 |
| 1872 | `rank` | Y | Y |  |  | Y |  |  | hole | 109&#8209;110 |
| 1873 | `select` | Y | Y |  |  | Y |  |  | hole | 112&#8209;113 |
| 1874 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 115&#8209;117 |
| 1875 | `from_sorted_elements` |  |  |  | Y | Y |  |  | hole | 380&#8209;381 |
| 1876 | `default` |  | Y |  |  |  | Y | Y |  | 405 |
| 1877 | `eq` |  | Y |  |  |  | Y | Y |  | 409&#8209;419 |

### Chap43/OrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1878 | `size` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;53 |
| 1879 | `empty` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;56 |
| 1880 | `singleton` | Y | Y |  |  | Y |  |  | hole | 58&#8209;59 |
| 1881 | `find` | Y | Y |  |  | Y |  | Y |  | 61 |
| 1882 | `lookup` | Y | Y |  |  | Y |  | Y |  | 63 |
| 1883 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 1884 | `insert` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 1885 | `delete` | Y | Y |  |  | Y |  |  | hole | 71&#8209;72 |
| 1886 | `domain` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 1887 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 1888 | `map` | Y | Y |  |  | Y |  |  | hole | 80&#8209;81 |
| 1889 | `filter` | Y | Y |  |  | Y |  |  | hole | 83&#8209;84 |
| 1890 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 1891 | `union` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 1892 | `difference` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 1893 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 1894 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 1895 | `reduce` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 1896 | `collect` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 1897 | `first_key` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 1898 | `last_key` | Y | Y |  |  | Y |  |  | hole | 110&#8209;111 |
| 1899 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 1900 | `next_key` | Y | Y |  |  | Y |  |  | hole | 116&#8209;117 |
| 1901 | `split_key` | Y | Y |  |  | Y |  |  | hole | 119&#8209;121 |
| 1902 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 1903 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 126&#8209;127 |
| 1904 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 129&#8209;130 |
| 1905 | `select_key` | Y | Y |  |  | Y |  |  | hole | 132&#8209;133 |
| 1906 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 135&#8209;137 |
| 1907 | `from_sorted_entries` |  |  |  | Y | Y |  |  | hole | 505&#8209;506 |
| 1908 | `eq` |  | Y |  |  |  | Y | Y |  | 523&#8209;525 |

### Chap43/OrderedTableMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1909 | `size` | Y | Y |  |  | Y |  |  | hole | 53&#8209;54 |
| 1910 | `empty` | Y | Y |  |  | Y |  |  | hole | 56&#8209;57 |
| 1911 | `singleton` | Y | Y |  |  | Y |  |  | hole | 59&#8209;60 |
| 1912 | `find` | Y | Y |  |  | Y |  |  | hole | 62 |
| 1913 | `insert` | Y | Y |  |  | Y |  |  | hole | 64&#8209;65 |
| 1914 | `delete` | Y | Y |  |  | Y |  |  | hole | 67&#8209;68 |
| 1915 | `domain` | Y | Y |  |  | Y |  |  | hole | 70&#8209;71 |
| 1916 | `map` | Y | Y |  |  | Y |  |  | hole | 73&#8209;74 |
| 1917 | `filter` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 1918 | `first_key` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 1919 | `last_key` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 1920 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 1921 | `next_key` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 1922 | `split_key` | Y | Y |  |  | Y |  |  | hole | 91&#8209;93 |
| 1923 | `join_key` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 1924 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 98&#8209;99 |
| 1925 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 1926 | `select_key` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 1927 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 107&#8209;109 |
| 1928 | `default` |  | Y |  |  |  | Y | Y |  | 369 |

### Chap43/OrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1929 | `size` | Y | Y |  |  | Y |  |  | unknown | 48&#8209;49 |
| 1930 | `empty` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;51 |
| 1931 | `singleton` | Y | Y |  |  | Y |  |  | hole | 52&#8209;53 |
| 1932 | `find` | Y | Y |  |  | Y |  | Y |  | 54 |
| 1933 | `lookup` | Y | Y |  |  | Y |  | Y |  | 55 |
| 1934 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;57 |
| 1935 | `insert` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;59 |
| 1936 | `delete` | Y | Y |  |  | Y |  |  | hole | 60&#8209;61 |
| 1937 | `domain` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 1938 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 1939 | `map` | Y | Y |  |  | Y |  |  | hole | 66&#8209;67 |
| 1940 | `filter` | Y | Y |  |  | Y |  |  | hole | 68&#8209;69 |
| 1941 | `reduce` | Y | Y |  |  | Y |  |  | hole | 70&#8209;71 |
| 1942 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 1943 | `union` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 1944 | `difference` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 1945 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 1946 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 1947 | `collect` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 1948 | `first_key` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 1949 | `last_key` | Y | Y |  |  | Y |  |  | hole | 86&#8209;87 |
| 1950 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 1951 | `next_key` | Y | Y |  |  | Y |  |  | hole | 90&#8209;91 |
| 1952 | `split_key` | Y | Y |  |  | Y |  |  | hole | 92&#8209;94 |
| 1953 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 1954 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 1955 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 1956 | `select_key` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 1957 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 103&#8209;105 |
| 1958 | `from_sorted_entries` |  |  |  | Y | Y |  |  | hole | 466&#8209;468 |
| 1959 | `eq` |  | Y |  |  |  | Y | Y |  | 482&#8209;484 |

### Chap43/OrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1960 | `size` | Y | Y |  |  | Y |  |  | hole | 49&#8209;50 |
| 1961 | `empty` | Y | Y |  |  | Y |  |  | hole | 51&#8209;52 |
| 1962 | `singleton` | Y | Y |  |  | Y |  |  | hole | 53&#8209;54 |
| 1963 | `find` | Y | Y |  |  | Y |  |  | hole | 55 |
| 1964 | `insert` | Y | Y |  |  | Y |  |  | hole | 56&#8209;57 |
| 1965 | `delete` | Y | Y |  |  | Y |  |  | hole | 58&#8209;59 |
| 1966 | `domain` | Y | Y |  |  | Y |  |  | hole | 60&#8209;61 |
| 1967 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 62&#8209;63 |
| 1968 | `map` | Y | Y |  |  | Y |  |  | hole | 64&#8209;65 |
| 1969 | `filter` | Y | Y |  |  | Y |  |  | hole | 66&#8209;67 |
| 1970 | `intersection` | Y | Y |  |  | Y |  |  | hole | 68&#8209;69 |
| 1971 | `union` | Y | Y |  |  | Y |  |  | hole | 70&#8209;71 |
| 1972 | `difference` | Y | Y |  |  | Y |  |  | hole | 72&#8209;73 |
| 1973 | `restrict` | Y | Y |  |  | Y |  |  | hole | 74&#8209;75 |
| 1974 | `subtract` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 1975 | `collect` | Y | Y |  |  | Y |  |  | hole | 78&#8209;79 |
| 1976 | `first_key` | Y | Y |  |  | Y |  |  | hole | 80&#8209;81 |
| 1977 | `last_key` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 1978 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 1979 | `next_key` | Y | Y |  |  | Y |  |  | hole | 86&#8209;87 |
| 1980 | `split_key` | Y | Y |  |  | Y |  |  | hole | 88&#8209;90 |
| 1981 | `join_key` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 1982 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 93&#8209;94 |
| 1983 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 1984 | `select_key` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 1985 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 99&#8209;101 |
| 1986 | `from_sorted_entries` |  |  |  | Y | Y |  |  | hole | 433&#8209;435 |
| 1987 | `eq` |  | Y |  |  |  | Y | Y |  | 449&#8209;451 |

### Chap44/DocumentIndex.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1988 | `_document_index_verified` |  |  |  | Y | Y |  | Y |  | 25 |
| 1989 | `eq` |  | Y |  |  | Y |  |  | unknown | 41&#8209;42 |
| 1990 | `make_index` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 1991 | `find` x3 | Y | Y |  |  |  | Y | Y |  | 285&#8209;287 |
| 1992 | `query_and` | Y | Y |  |  |  | Y | Y |  | 61&#8209;63 |
| 1993 | `query_or` | Y | Y |  |  |  | Y | Y |  | 65&#8209;67 |
| 1994 | `query_and_not` | Y | Y |  |  |  | Y | Y |  | 69&#8209;71 |
| 1995 | `size` | Y | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 1996 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 77&#8209;79 |
| 1997 | `empty` | Y | Y |  |  |  | Y | Y |  | 81&#8209;83 |
| 1998 | `get_all_words` | Y | Y |  |  |  | Y | Y |  | 85&#8209;87 |
| 1999 | `word_count` | Y | Y |  |  |  | Y | Y |  | 89&#8209;91 |
| 2000 | `tokens` |  |  |  | Y |  | Y | Y |  | 195&#8209;222 |
| 2001 | `create_finder` |  |  |  | Y |  | Y | Y |  | 224&#8209;229 |
| 2002 | `new` | Y | Y |  |  |  | Y | Y |  | 281&#8209;283 |
| 2003 | `and` | Y | Y |  |  |  | Y | Y |  | 289&#8209;291 |
| 2004 | `or` | Y | Y |  |  |  | Y | Y |  | 293&#8209;295 |
| 2005 | `and_not` | Y | Y |  |  |  | Y | Y |  | 297&#8209;299 |
| 2006 | `complex_query` | Y | Y |  |  |  | Y | Y |  | 301&#8209;303 |

### Chap44/Example44_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2007 | `_example_44_1_verified` |  |  |  | Y | Y |  | Y |  | 14 |
| 2008 | `create_tweet_collection` |  |  |  | Y |  | Y | Y |  | 17&#8209;27 |
| 2009 | `create_tweet_index` |  |  |  | Y |  | Y | Y |  | 29&#8209;35 |
| 2010 | `create_tweet_finder` |  |  |  | Y |  | Y | Y |  | 37&#8209;44 |
| 2011 | `default` |  | Y |  |  |  | Y | Y |  | 56 |
| 2012 | `new` |  |  | Y |  |  | Y | Y |  | 60&#8209;68 |
| 2013 | `search_fun` |  |  | Y |  |  | Y | Y |  | 70&#8209;73 |
| 2014 | `search_club` |  |  | Y |  |  | Y | Y |  | 75&#8209;78 |
| 2015 | `search_food` |  |  | Y |  |  | Y | Y |  | 80&#8209;83 |
| 2016 | `search_chess` |  |  | Y |  |  | Y | Y |  | 85&#8209;88 |
| 2017 | `complex_query_fun_and_food_or_chess` |  |  | Y |  |  | Y | Y |  | 90&#8209;104 |
| 2018 | `count_fun_but_not_chess` |  |  | Y |  |  | Y | Y |  | 106&#8209;117 |
| 2019 | `search_food_or_fun` |  |  | Y |  |  | Y | Y |  | 119&#8209;127 |
| 2020 | `search_party_and_food` |  |  | Y |  |  | Y | Y |  | 129&#8209;137 |
| 2021 | `get_all_words` |  |  | Y |  |  | Y | Y |  | 139&#8209;142 |
| 2022 | `get_word_count` |  |  | Y |  |  | Y | Y |  | 144&#8209;147 |
| 2023 | `query_builder_example` |  |  | Y |  |  | Y | Y |  | 149&#8209;162 |
| 2024 | `doc_set_to_sorted_vec` |  |  |  | Y |  | Y | Y |  | 165&#8209;178 |
| 2025 | `verify_textbook_examples` |  |  |  | Y |  | Y | Y |  | 180&#8209;220 |
| 2026 | `performance_comparison_demo` |  |  |  | Y |  | Y | Y |  | 222&#8209;236 |
| 2027 | `tokenization_demo` |  |  |  | Y |  | Y | Y |  | 238&#8209;244 |
| 2028 | `index_statistics` |  |  |  | Y |  | Y | Y |  | 246&#8209;265 |

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2029 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 16 |
| 2030 | `eq` |  | Y |  |  | Y |  |  | hole | 43&#8209;44 |
| 2031 | `empty` | Y | Y |  |  |  | Y | Y |  | 63&#8209;64 |
| 2032 | `singleton` | Y | Y |  |  |  | Y | Y |  | 66&#8209;67 |
| 2033 | `find_min` | Y | Y |  |  |  | Y | Y |  | 69&#8209;71 |
| 2034 | `insert` | Y | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 2035 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 77&#8209;81 |
| 2036 | `meld` | Y | Y |  |  |  | Y | Y |  | 83&#8209;85 |
| 2037 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 87&#8209;89 |
| 2038 | `size` | Y | Y |  |  |  | Y | Y |  | 91 |
| 2039 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 92 |
| 2040 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 93 |
| 2041 | `find_max` | Y | Y |  |  |  | Y | Y |  | 94 |
| 2042 | `delete_max` | Y | Y |  |  |  | Y | Y |  | 95&#8209;97 |
| 2043 | `insert_all` | Y | Y |  |  |  | Y | Y |  | 98 |
| 2044 | `extract_all_sorted` | Y | Y |  |  |  | Y | Y |  | 99 |
| 2045 | `contains` | Y | Y |  |  |  | Y | Y |  | 100 |
| 2046 | `remove` | Y | Y |  |  |  | Y | Y |  | 101&#8209;103 |
| 2047 | `range` | Y | Y |  |  |  | Y | Y |  | 104 |
| 2048 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 105 |
| 2049 | `to_vec` | Y | Y |  |  |  | Y | Y |  | 106 |
| 2050 | `to_sorted_vec` | Y | Y |  |  |  | Y | Y |  | 107 |
| 2051 | `is_sorted` | Y | Y |  |  |  | Y | Y |  | 108 |
| 2052 | `height` | Y | Y |  |  |  | Y | Y |  | 109 |
| 2053 | `split` | Y | Y |  |  |  | Y | Y |  | 110&#8209;112 |
| 2054 | `join` | Y | Y |  |  |  | Y | Y |  | 113 |
| 2055 | `filter` | Y | Y |  |  |  | Y | Y |  | 114&#8209;116 |
| 2056 | `map` | Y | Y |  |  |  | Y | Y |  | 117&#8209;120 |
| 2057 | `default` |  | Y |  |  |  | Y | Y |  | 422 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2058 | `_binary_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 15 |
| 2059 | `eq` |  | Y |  |  | Y |  |  | hole | 43&#8209;44 |
| 2060 | `empty` | Y | Y |  |  |  | Y | Y |  | 63&#8209;64 |
| 2061 | `singleton` | Y | Y |  |  |  | Y | Y |  | 66&#8209;67 |
| 2062 | `find_min` | Y | Y |  |  |  | Y | Y |  | 69&#8209;71 |
| 2063 | `insert` | Y | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 2064 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 77&#8209;81 |
| 2065 | `meld` | Y | Y |  |  |  | Y | Y |  | 83&#8209;85 |
| 2066 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 87&#8209;89 |
| 2067 | `size` | Y | Y |  |  |  | Y | Y |  | 91 |
| 2068 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 92 |
| 2069 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 93 |
| 2070 | `insert_all` | Y | Y |  |  |  | Y | Y |  | 94 |
| 2071 | `extract_all_sorted` | Y | Y |  |  |  | Y | Y |  | 95 |
| 2072 | `is_valid_heap` | Y | Y |  |  |  | Y | Y |  | 96 |
| 2073 | `height` | Y | Y |  |  |  | Y | Y |  | 97 |
| 2074 | `level_elements` | Y | Y |  |  |  | Y | Y |  | 98 |
| 2075 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 99 |
| 2076 | `to_vec` | Y | Y |  |  |  | Y | Y |  | 100 |
| 2077 | `to_sorted_vec` | Y | Y |  |  |  | Y | Y |  | 101 |
| 2078 | `left_child` |  |  |  | Y |  | Y | Y |  | 104 |
| 2079 | `right_child` |  |  |  | Y |  | Y | Y |  | 105 |
| 2080 | `parent` |  |  |  | Y |  | Y | Y |  | 106 |
| 2081 | `swap_elements` |  |  |  | Y |  | Y | Y |  | 108&#8209;125 |
| 2082 | `bubble_up` |  |  |  | Y |  | Y | Y |  | 127&#8209;144 |
| 2083 | `bubble_down` |  |  |  | Y |  | Y | Y |  | 146&#8209;171 |
| 2084 | `heapify` |  |  |  | Y |  | Y | Y |  | 173&#8209;186 |
| 2085 | `is_heap` |  |  |  | Y |  | Y | Y |  | 188&#8209;202 |
| 2086 | `default` |  | Y |  |  |  | Y | Y |  | 375 |

### Chap45/Example45_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2087 | `_example_45_2_verified` |  |  |  | Y | Y |  | Y |  | 12 |
| 2088 | `example_45_2_textbook_example` | Y |  |  | Y |  | Y | Y |  | 17&#8209;19 |
| 2089 | `example_45_2_reverse_sorted` | Y |  |  | Y |  | Y | Y |  | 21&#8209;23 |
| 2090 | `example_45_2_already_sorted` | Y |  |  | Y |  | Y | Y |  | 25&#8209;27 |
| 2091 | `example_45_2_duplicates` | Y |  |  | Y |  | Y | Y |  | 29&#8209;31 |
| 2092 | `example_45_2_single_element` | Y |  |  | Y |  | Y | Y |  | 33&#8209;35 |
| 2093 | `example_45_2_empty` | Y |  |  | Y |  | Y | Y |  | 37&#8209;39 |
| 2094 | `example_45_2_efficiency_demonstration` | Y |  |  | Y |  | Y | Y |  | 41&#8209;43 |
| 2095 | `run_example_45_2` | Y |  |  | Y |  | Y | Y |  | 45&#8209;47 |

### Chap45/HeapsortExample.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2096 | `_heapsort_example_verified` |  |  |  | Y | Y |  | Y |  | 24 |
| 2097 | `eq` |  | Y |  |  | Y |  |  | hole | 81&#8209;82 |
| 2098 | `heapsort_unsorted_list` | Y |  |  | Y |  | Y | Y |  | 99&#8209;101 |
| 2099 | `heapsort_sorted_list` | Y |  |  | Y |  | Y | Y |  | 103&#8209;105 |
| 2100 | `heapsort_balanced_tree` | Y |  |  | Y |  | Y | Y |  | 107&#8209;109 |
| 2101 | `heapsort_binary_heap` | Y |  |  | Y |  | Y | Y |  | 111&#8209;113 |
| 2102 | `heapsort_leftist_heap` | Y |  |  | Y |  | Y | Y |  | 115&#8209;117 |
| 2103 | `compare_all_heapsorts` | Y |  |  | Y |  | Y | Y |  | 119&#8209;121 |
| 2104 | `textbook_example` | Y |  |  | Y |  | Y | Y |  | 240&#8209;241 |
| 2105 | `reverse_sorted_example` | Y |  |  | Y |  | Y | Y |  | 242&#8209;243 |
| 2106 | `already_sorted_example` | Y |  |  | Y |  | Y | Y |  | 244&#8209;245 |
| 2107 | `duplicates_example` | Y |  |  | Y |  | Y | Y |  | 246&#8209;247 |
| 2108 | `single_element_example` | Y |  |  | Y |  | Y | Y |  | 248&#8209;249 |
| 2109 | `empty_example` | Y |  |  | Y |  | Y | Y |  | 250&#8209;251 |
| 2110 | `large_example` | Y |  |  | Y |  | Y | Y |  | 252&#8209;253 |
| 2111 | `efficiency_demonstration` | Y |  |  | Y |  | Y | Y |  | 254&#8209;255 |
| 2112 | `all_results_match` | Y | Y |  |  |  | Y | Y |  | 259&#8209;260 |
| 2113 | `all_results_sorted` | Y | Y |  |  |  | Y | Y |  | 261&#8209;262 |
| 2114 | `complexity_analysis` | Y |  |  | Y |  | Y | Y |  | 266&#8209;267 |
| 2115 | `correctness_verification` | Y |  |  | Y |  | Y | Y |  | 268&#8209;269 |
| 2116 | `vec_to_array_seq` | Y |  |  | Y |  | Y | Y |  | 273&#8209;274 |
| 2117 | `vec_to_avl_seq` | Y |  |  | Y |  | Y | Y |  | 275&#8209;276 |
| 2118 | `is_sorted` | Y | Y |  | Y |  | Y | Y |  | 277&#8209;278 |
| 2119 | `generate_test_sequences` | Y |  |  | Y |  | Y | Y |  | 279&#8209;280 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2120 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 12 |
| 2121 | `empty` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 2122 | `singleton` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 2123 | `find_min` | Y | Y |  |  |  | Y | Y |  | 41&#8209;43 |
| 2124 | `insert` | Y | Y |  |  |  | Y | Y |  | 45&#8209;47 |
| 2125 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 49&#8209;53 |
| 2126 | `meld` | Y | Y |  |  |  | Y | Y |  | 55&#8209;57 |
| 2127 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 59&#8209;61 |
| 2128 | `size` x3 | Y | Y |  |  |  | Y | Y |  | 93 |
| 2129 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 64 |
| 2130 | `extract_all_sorted` | Y | Y |  |  |  | Y | Y |  | 65 |
| 2131 | `height` x3 | Y | Y |  |  |  | Y | Y |  | 94 |
| 2132 | `root_rank` | Y | Y |  |  |  | Y | Y |  | 67 |
| 2133 | `is_valid_leftist_heap` | Y | Y |  |  |  | Y | Y |  | 68 |
| 2134 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 69 |
| 2135 | `to_vec` x3 | Y | Y |  |  |  | Y | Y |  | 97 |
| 2136 | `to_sorted_vec` | Y | Y |  |  |  | Y | Y |  | 71 |
| 2137 | `meld_multiple` | Y | Y |  |  |  | Y | Y |  | 72&#8209;74 |
| 2138 | `split` | Y | Y |  |  |  | Y | Y |  | 75&#8209;77 |
| 2139 | `efficient_multi_way_merge` | Y |  |  | Y |  | Y | Y |  | 81&#8209;83 |
| 2140 | `parallel_heap_construction` | Y |  |  | Y |  | Y | Y |  | 84&#8209;85 |
| 2141 | `rank` | Y | Y |  |  |  | Y | Y |  | 89 |
| 2142 | `make_node` | Y | Y |  |  |  | Y | Y |  | 90 |
| 2143 | `meld_nodes` | Y | Y |  |  |  | Y | Y |  | 91&#8209;92 |
| 2144 | `is_leftist` | Y | Y |  |  |  | Y | Y |  | 95 |
| 2145 | `is_heap` | Y | Y |  |  |  | Y | Y |  | 96 |
| 2146 | `default` |  | Y |  |  |  | Y | Y |  | 380 |
| 2147 | `format_node` |  | Y |  |  |  | Y | Y |  | 385&#8209;396 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2148 | `_sorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 15 |
| 2149 | `eq` |  | Y |  |  | Y |  |  | hole | 43&#8209;44 |
| 2150 | `empty` | Y | Y |  |  |  | Y | Y |  | 63&#8209;64 |
| 2151 | `singleton` | Y | Y |  |  |  | Y | Y |  | 66&#8209;67 |
| 2152 | `find_min` | Y | Y |  |  |  | Y | Y |  | 69&#8209;71 |
| 2153 | `insert` | Y | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 2154 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 77&#8209;81 |
| 2155 | `meld` | Y | Y |  |  |  | Y | Y |  | 83&#8209;85 |
| 2156 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 87&#8209;89 |
| 2157 | `size` | Y | Y |  |  |  | Y | Y |  | 91 |
| 2158 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 92 |
| 2159 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 93 |
| 2160 | `insert_all` | Y | Y |  |  |  | Y | Y |  | 94 |
| 2161 | `extract_all_sorted` | Y | Y |  |  |  | Y | Y |  | 95 |
| 2162 | `find_max` | Y | Y |  |  |  | Y | Y |  | 96 |
| 2163 | `delete_max` | Y | Y |  |  |  | Y | Y |  | 97&#8209;99 |
| 2164 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 100 |
| 2165 | `to_vec` | Y | Y |  |  |  | Y | Y |  | 101 |
| 2166 | `to_sorted_vec` | Y | Y |  |  |  | Y | Y |  | 102 |
| 2167 | `is_sorted` | Y | Y |  |  |  | Y | Y |  | 103 |
| 2168 | `default` |  | Y |  |  |  | Y | Y |  | 322 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2169 | `_unsorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 15 |
| 2170 | `eq` |  | Y |  |  | Y |  |  | hole | 43&#8209;44 |
| 2171 | `empty` | Y | Y |  |  |  | Y | Y |  | 63&#8209;64 |
| 2172 | `singleton` | Y | Y |  |  |  | Y | Y |  | 66&#8209;67 |
| 2173 | `find_min` | Y | Y |  |  |  | Y | Y |  | 69&#8209;71 |
| 2174 | `insert` | Y | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 2175 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 77&#8209;81 |
| 2176 | `meld` | Y | Y |  |  |  | Y | Y |  | 83&#8209;85 |
| 2177 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 87&#8209;89 |
| 2178 | `size` | Y | Y |  |  |  | Y | Y |  | 91 |
| 2179 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 92 |
| 2180 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 93 |
| 2181 | `insert_all` | Y | Y |  |  |  | Y | Y |  | 94 |
| 2182 | `extract_all_sorted` | Y | Y |  |  |  | Y | Y |  | 95 |
| 2183 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 96 |
| 2184 | `to_vec` | Y | Y |  |  |  | Y | Y |  | 97 |
| 2185 | `to_sorted_vec` | Y | Y |  |  |  | Y | Y |  | 98 |
| 2186 | `default` |  | Y |  |  |  | Y | Y |  | 246 |

### Chap47/ChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2187 | `_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 24 |
| 2188 | `eq` |  | Y |  |  | Y |  |  | hole | 52&#8209;53 |
| 2189 | `hash_index` | Y |  |  |  |  | Y | Y |  | 72&#8209;75 |
| 2190 | `insert_chained` | Y |  |  |  |  | Y | Y |  | 77&#8209;89 |
| 2191 | `lookup_chained` | Y |  |  |  |  | Y | Y |  | 91&#8209;101 |
| 2192 | `delete_chained` | Y |  |  |  |  | Y | Y |  | 103&#8209;117 |

### Chap47/DoubleHashFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2193 | `second_hash` |  |  | Y |  |  | Y | Y |  | 36&#8209;62 |
| 2194 | `insert` |  | Y |  |  |  | Y | Y |  | 68&#8209;85 |
| 2195 | `lookup` |  | Y |  |  |  | Y | Y |  | 87&#8209;102 |
| 2196 | `delete` |  | Y |  |  |  | Y | Y |  | 104&#8209;123 |
| 2197 | `resize` |  | Y |  |  |  | Y | Y |  | 125&#8209;156 |
| 2198 | `probe` |  | Y |  |  |  | Y | Y |  | 162&#8209;168 |
| 2199 | `find_slot` |  | Y |  |  |  | Y | Y |  | 170&#8209;183 |

### Chap47/FlatHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2200 | `new` |  | Y |  |  | Y |  | Y |  | 53 |
| 2201 | `insert` |  | Y |  |  | Y |  | Y |  | 57 |
| 2202 | `lookup` |  | Y |  |  | Y |  | Y |  | 61 |
| 2203 | `delete` |  | Y |  |  | Y |  | Y |  | 72 |
| 2204 | `probe` | Y |  |  |  |  | Y | Y |  | 97&#8209;100 |
| 2205 | `find_slot` | Y |  |  |  |  | Y | Y |  | 102&#8209;105 |
| 2206 | `insert_with_probe` | Y |  |  |  |  | Y | Y |  | 107&#8209;115 |
| 2207 | `lookup_with_probe` | Y |  |  |  |  | Y | Y |  | 117&#8209;130 |

### Chap47/LinProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2208 | `insert` |  | Y |  |  |  | Y | Y |  | 35&#8209;52 |
| 2209 | `lookup` |  | Y |  |  |  | Y | Y |  | 54&#8209;69 |
| 2210 | `delete` |  | Y |  |  |  | Y | Y |  | 71&#8209;90 |
| 2211 | `resize` |  | Y |  |  |  | Y | Y |  | 92&#8209;123 |
| 2212 | `probe` |  | Y |  |  |  | Y | Y |  | 129&#8209;134 |
| 2213 | `find_slot` |  | Y |  |  |  | Y | Y |  | 136&#8209;149 |

### Chap47/LinkedListChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2214 | `_linked_list_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 23 |
| 2215 | `new` |  | Y |  |  |  | Y | Y |  | 27&#8209;29 |
| 2216 | `insert` x2 |  | Y |  |  |  | Y | Y |  | 31&#8209;41 |
| 2217 | `lookup` x2 |  | Y |  |  |  | Y | Y |  | 43&#8209;52 |
| 2218 | `delete` x2 |  | Y |  |  |  | Y | Y |  | 54&#8209;72 |
| 2219 | `resize` |  | Y |  |  |  | Y | Y |  | 101&#8209;132 |
| 2220 | `hash_index` |  | Y |  |  |  | Y | Y |  | 138&#8209;142 |

### Chap47/ParaHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2221 | `new` | Y |  |  |  | Y |  | Y |  | 49 |
| 2222 | `insert` x2 | Y |  |  |  | Y |  | Y |  | 100&#8209;103 |
| 2223 | `lookup` x2 | Y |  |  |  | Y |  | Y |  | 105&#8209;108 |
| 2224 | `delete` x2 | Y |  |  |  | Y |  | Y |  | 110&#8209;113 |
| 2225 | `createTable` | Y |  |  |  |  | Y | Y |  | 81&#8209;98 |
| 2226 | `metrics` | Y |  |  |  |  | Y | Y |  | 115&#8209;118 |
| 2227 | `loadAndSize` | Y |  |  |  |  | Y | Y |  | 120&#8209;134 |
| 2228 | `resize` | Y |  |  |  |  | Y | Y |  | 136&#8209;141 |

### Chap47/QuadProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2229 | `insert` |  | Y |  |  |  | Y | Y |  | 36&#8209;53 |
| 2230 | `lookup` |  | Y |  |  |  | Y | Y |  | 55&#8209;71 |
| 2231 | `delete` |  | Y |  |  |  | Y | Y |  | 73&#8209;93 |
| 2232 | `resize` |  | Y |  |  |  | Y | Y |  | 95&#8209;126 |
| 2233 | `probe` |  | Y |  |  |  | Y | Y |  | 132&#8209;137 |
| 2234 | `find_slot` |  | Y |  |  |  | Y | Y |  | 139&#8209;153 |

### Chap47/StructChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2235 | `_struct_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 24 |
| 2236 | `new` |  | Y |  |  |  | Y | Y |  | 44&#8209;46 |
| 2237 | `insert` x2 |  | Y |  |  |  | Y | Y |  | 48&#8209;65 |
| 2238 | `lookup` x2 |  | Y |  |  |  | Y | Y |  | 67&#8209;78 |
| 2239 | `delete` x2 |  | Y |  |  |  | Y | Y |  | 80&#8209;96 |
| 2240 | `default` |  | Y |  |  |  | Y | Y |  | 100&#8209;102 |
| 2241 | `resize` |  | Y |  |  |  | Y | Y |  | 131&#8209;164 |
| 2242 | `hash_index` |  | Y |  |  |  | Y | Y |  | 170&#8209;174 |

### Chap47/VecChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2243 | `_vec_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 22 |
| 2244 | `new` |  | Y |  |  |  | Y | Y |  | 28&#8209;30 |
| 2245 | `insert` x2 |  | Y |  |  |  | Y | Y |  | 32&#8209;42 |
| 2246 | `lookup` x2 |  | Y |  |  |  | Y | Y |  | 44&#8209;53 |
| 2247 | `delete` x2 |  | Y |  |  |  | Y | Y |  | 55&#8209;64 |
| 2248 | `resize` |  | Y |  |  |  | Y | Y |  | 95&#8209;126 |
| 2249 | `hash_index` |  | Y |  |  |  | Y | Y |  | 132&#8209;136 |

### Chap49/MinEditDistMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2250 | `new_min_edit_dist_eph_lock` |  |  |  | Y | Y |  |  | hole | 30 |
| 2251 | `new` | Y | Y |  |  |  | Y | Y |  | 48&#8209;53 |
| 2252 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 55&#8209;58 |
| 2253 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 60&#8209;64 |
| 2254 | `source` | Y | Y |  |  |  | Y | Y |  | 66&#8209;69 |
| 2255 | `target` | Y | Y |  |  |  | Y | Y |  | 71&#8209;74 |
| 2256 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 76&#8209;79 |
| 2257 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 81&#8209;84 |
| 2258 | `set_source` | Y | Y |  |  |  | Y | Y |  | 86&#8209;89 |
| 2259 | `set_target` | Y | Y |  |  |  | Y | Y |  | 91&#8209;94 |
| 2260 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 96&#8209;99 |
| 2261 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 101&#8209;104 |
| 2262 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 109&#8209;160 |
| 2263 | `eq` |  | Y |  |  |  | Y | Y |  | 237 |

### Chap49/MinEditDistMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2264 | `new_min_edit_dist_per_lock` |  |  |  | Y | Y |  |  | hole | 29 |
| 2265 | `new` | Y | Y |  |  |  | Y | Y |  | 47&#8209;52 |
| 2266 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 54&#8209;57 |
| 2267 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 59&#8209;63 |
| 2268 | `source` | Y | Y |  |  |  | Y | Y |  | 65&#8209;68 |
| 2269 | `target` | Y | Y |  |  |  | Y | Y |  | 70&#8209;73 |
| 2270 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 75&#8209;78 |
| 2271 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 83&#8209;130 |
| 2272 | `eq` |  | Y |  |  |  | Y | Y |  | 183 |

### Chap49/MinEditDistStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2273 | `_min_edit_dist_st_eph_verified` |  |  |  | Y | Y |  | Y |  | 23 |
| 2274 | `new` | Y | Y |  |  |  | Y | Y |  | 40&#8209;45 |
| 2275 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 47&#8209;50 |
| 2276 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 52&#8209;54 |
| 2277 | `source` | Y | Y |  |  |  | Y | Y |  | 56&#8209;59 |
| 2278 | `target` | Y | Y |  |  |  | Y | Y |  | 61&#8209;64 |
| 2279 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 66&#8209;69 |
| 2280 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 71&#8209;74 |
| 2281 | `set_source` | Y | Y |  |  |  | Y | Y |  | 76&#8209;79 |
| 2282 | `set_target` | Y | Y |  |  |  | Y | Y |  | 81&#8209;84 |
| 2283 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 86&#8209;89 |
| 2284 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 91&#8209;94 |
| 2285 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 99&#8209;126 |

### Chap49/MinEditDistStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2286 | `_min_edit_dist_st_per_verified` |  |  |  | Y | Y |  | Y |  | 22 |
| 2287 | `new` | Y | Y |  |  |  | Y | Y |  | 39&#8209;44 |
| 2288 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 46&#8209;49 |
| 2289 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 51&#8209;53 |
| 2290 | `source` | Y | Y |  |  |  | Y | Y |  | 55&#8209;58 |
| 2291 | `target` | Y | Y |  |  |  | Y | Y |  | 60&#8209;63 |
| 2292 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 65&#8209;68 |
| 2293 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 73&#8209;100 |

### Chap49/SubsetSumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2294 | `new_subset_sum_eph_lock` |  |  |  | Y | Y |  |  | hole | 30 |
| 2295 | `new` | Y | Y |  |  |  | Y | Y |  | 47&#8209;52 |
| 2296 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 54&#8209;57 |
| 2297 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 59&#8209;63 |
| 2298 | `multiset` | Y | Y |  |  |  | Y | Y |  | 65&#8209;68 |
| 2299 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 70&#8209;73 |
| 2300 | `set` | Y | Y |  |  |  | Y | Y |  | 75&#8209;78 |
| 2301 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 80&#8209;83 |
| 2302 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 85&#8209;88 |
| 2303 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 93&#8209;138 |
| 2304 | `eq` |  | Y |  |  |  | Y | Y |  | 204 |

### Chap49/SubsetSumMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2305 | `new_subset_sum_per_lock` |  |  |  | Y | Y |  |  | hole | 29 |
| 2306 | `new` | Y | Y |  |  |  | Y | Y |  | 46&#8209;51 |
| 2307 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 53&#8209;56 |
| 2308 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 58&#8209;62 |
| 2309 | `multiset` | Y | Y |  |  |  | Y | Y |  | 64&#8209;67 |
| 2310 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 69&#8209;72 |
| 2311 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 77&#8209;122 |
| 2312 | `eq` |  | Y |  |  |  | Y | Y |  | 173 |

### Chap49/SubsetSumStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2313 | `_subset_sum_st_eph_verified` |  |  |  | Y | Y |  | Y |  | 21 |
| 2314 | `new` | Y | Y |  |  |  | Y | Y |  | 37&#8209;42 |
| 2315 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 44&#8209;47 |
| 2316 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 49&#8209;53 |
| 2317 | `multiset` | Y | Y |  |  |  | Y | Y |  | 55&#8209;58 |
| 2318 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 60&#8209;63 |
| 2319 | `set` | Y | Y |  |  |  | Y | Y |  | 65&#8209;68 |
| 2320 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 70&#8209;73 |
| 2321 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 75&#8209;78 |
| 2322 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 83&#8209;105 |

### Chap49/SubsetSumStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2323 | `_subset_sum_st_per_verified` |  |  |  | Y | Y |  | Y |  | 21 |
| 2324 | `new` | Y | Y |  |  |  | Y | Y |  | 37&#8209;42 |
| 2325 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 44&#8209;47 |
| 2326 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 49&#8209;53 |
| 2327 | `multiset` | Y | Y |  |  |  | Y | Y |  | 55&#8209;58 |
| 2328 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 60&#8209;63 |
| 2329 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 68&#8209;90 |

### Chap50/MatrixChainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2330 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 54&#8209;55 |
| 2331 | `new_mceph_dim_lock` |  |  |  | Y | Y |  |  | hole | 68 |
| 2332 | `new_mceph_memo_lock` |  |  |  | Y | Y |  |  | hole | 77 |
| 2333 | `new` | Y | Y |  |  |  | Y | Y |  | 92&#8209;94 |
| 2334 | `from_dimensions` | Y | Y |  |  |  | Y | Y |  | 96&#8209;98 |
| 2335 | `from_dim_pairs` | Y | Y |  |  |  | Y | Y |  | 100&#8209;102 |
| 2336 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 104&#8209;106 |
| 2337 | `dimensions` | Y | Y |  |  |  | Y | Y |  | 108&#8209;110 |
| 2338 | `set_dimension` | Y | Y |  |  |  | Y | Y |  | 112&#8209;114 |
| 2339 | `update_dimension` | Y | Y |  |  |  | Y | Y |  | 116&#8209;118 |
| 2340 | `num_matrices` | Y | Y |  |  |  | Y | Y |  | 120&#8209;122 |
| 2341 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 124&#8209;126 |
| 2342 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 128&#8209;130 |
| 2343 | `multiply_cost_mt_eph` |  |  |  | Y |  | Y | Y |  | 135&#8209;142 |
| 2344 | `parallel_min_reduction` |  |  |  | Y |  | Y | Y |  | 144&#8209;166 |
| 2345 | `matrix_chain_rec_mt_eph` |  |  |  | Y |  | Y | Y |  | 168&#8209;200 |

### Chap50/MatrixChainMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2346 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 56&#8209;57 |
| 2347 | `new_mcper_memo_lock` |  |  |  | Y | Y |  |  | hole | 70 |
| 2348 | `new` | Y | Y |  |  |  | Y | Y |  | 85&#8209;87 |
| 2349 | `from_dimensions` | Y | Y |  |  |  | Y | Y |  | 89&#8209;91 |
| 2350 | `from_dim_pairs` | Y | Y |  |  |  | Y | Y |  | 93&#8209;95 |
| 2351 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 97&#8209;99 |
| 2352 | `dimensions` | Y | Y |  |  |  | Y | Y |  | 101&#8209;103 |
| 2353 | `num_matrices` | Y | Y |  |  |  | Y | Y |  | 105&#8209;107 |
| 2354 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 109&#8209;111 |
| 2355 | `multiply_cost_mt_per` |  |  |  | Y |  | Y | Y |  | 116&#8209;121 |
| 2356 | `parallel_min_reduction_mt_per` |  |  |  | Y |  | Y | Y |  | 123&#8209;145 |
| 2357 | `matrix_chain_rec_mt_per` |  |  |  | Y |  | Y | Y |  | 147&#8209;179 |

### Chap50/MatrixChainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2358 | `eq` |  | Y |  |  | Y |  |  | hole | 51&#8209;52 |
| 2359 | `new` | Y | Y |  |  |  | Y | Y |  | 75&#8209;77 |
| 2360 | `from_dimensions` | Y | Y |  |  |  | Y | Y |  | 79&#8209;81 |
| 2361 | `from_dim_pairs` | Y | Y |  |  |  | Y | Y |  | 83&#8209;85 |
| 2362 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 87&#8209;89 |
| 2363 | `dimensions` | Y | Y |  |  |  | Y | Y |  | 91&#8209;93 |
| 2364 | `dimensions_mut` | Y | Y |  |  |  | Y | Y |  | 95&#8209;97 |
| 2365 | `set_dimension` | Y | Y |  |  |  | Y | Y |  | 99&#8209;101 |
| 2366 | `update_dimension` | Y | Y |  |  |  | Y | Y |  | 103&#8209;105 |
| 2367 | `num_matrices` | Y | Y |  |  |  | Y | Y |  | 107&#8209;109 |
| 2368 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 111&#8209;113 |
| 2369 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 115&#8209;117 |
| 2370 | `multiply_cost_st_eph` |  |  |  | Y |  | Y | Y |  | 122&#8209;127 |
| 2371 | `matrix_chain_rec_st_eph` |  |  |  | Y |  | Y | Y |  | 129&#8209;150 |

### Chap50/MatrixChainStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2372 | `eq` |  | Y |  |  | Y |  |  | hole | 53&#8209;54 |
| 2373 | `new` | Y | Y |  |  |  | Y | Y |  | 77&#8209;79 |
| 2374 | `from_dimensions` | Y | Y |  |  |  | Y | Y |  | 81&#8209;83 |
| 2375 | `from_dim_pairs` | Y | Y |  |  |  | Y | Y |  | 85&#8209;87 |
| 2376 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 89&#8209;91 |
| 2377 | `dimensions` | Y | Y |  |  |  | Y | Y |  | 93&#8209;95 |
| 2378 | `num_matrices` | Y | Y |  |  |  | Y | Y |  | 97&#8209;99 |
| 2379 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 101&#8209;103 |
| 2380 | `multiply_cost_st_per` |  |  |  | Y |  | Y | Y |  | 108&#8209;113 |
| 2381 | `matrix_chain_rec_st_per` |  |  |  | Y |  | Y | Y |  | 115&#8209;136 |

### Chap50/OptBinSearchTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2382 | `new_obst_eph_keys_lock` |  |  |  | Y | Y |  |  | hole | 46 |
| 2383 | `new_obst_eph_memo_lock` |  |  |  | Y | Y |  |  | hole | 55 |
| 2384 | `new` | Y | Y |  |  |  | Y | Y |  | 63&#8209;65 |
| 2385 | `from_keys_probs` | Y | Y |  |  |  | Y | Y |  | 67&#8209;69 |
| 2386 | `from_key_probs` | Y | Y |  |  |  | Y | Y |  | 71&#8209;73 |
| 2387 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 75&#8209;79 |
| 2388 | `keys` | Y | Y |  |  |  | Y | Y |  | 81&#8209;83 |
| 2389 | `set_key_prob` | Y | Y |  |  |  | Y | Y |  | 85&#8209;87 |
| 2390 | `update_prob` | Y | Y |  |  |  | Y | Y |  | 89&#8209;91 |
| 2391 | `num_keys` | Y | Y |  |  |  | Y | Y |  | 93&#8209;95 |
| 2392 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 97&#8209;99 |
| 2393 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 101&#8209;103 |
| 2394 | `parallel_min_reduction` |  |  |  | Y |  | Y | Y |  | 106&#8209;132 |
| 2395 | `obst_rec` |  |  |  | Y |  | Y | Y |  | 134&#8209;178 |
| 2396 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 303&#8209;312 |

### Chap50/OptBinSearchTreeMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2397 | `new_obst_per_memo_lock` |  |  |  | Y | Y |  |  | hole | 44 |
| 2398 | `new` | Y | Y |  |  |  | Y | Y |  | 52&#8209;54 |
| 2399 | `from_keys_probs` | Y | Y |  |  |  | Y | Y |  | 56&#8209;58 |
| 2400 | `from_key_probs` | Y | Y |  |  |  | Y | Y |  | 60&#8209;62 |
| 2401 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 64&#8209;68 |
| 2402 | `keys` | Y | Y |  |  |  | Y | Y |  | 70&#8209;72 |
| 2403 | `num_keys` | Y | Y |  |  |  | Y | Y |  | 74&#8209;76 |
| 2404 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 78&#8209;80 |
| 2405 | `parallel_min_reduction` |  |  |  | Y |  | Y | Y |  | 83&#8209;109 |
| 2406 | `obst_rec` |  |  |  | Y |  | Y | Y |  | 111&#8209;149 |
| 2407 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 224&#8209;226 |

### Chap50/OptBinSearchTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2408 | `new` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 2409 | `from_keys_probs` | Y | Y |  |  |  | Y | Y |  | 54&#8209;56 |
| 2410 | `from_key_probs` | Y | Y |  |  |  | Y | Y |  | 58&#8209;60 |
| 2411 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 2412 | `keys` | Y | Y |  |  |  | Y | Y |  | 66&#8209;68 |
| 2413 | `keys_mut` | Y | Y |  |  |  | Y | Y |  | 70&#8209;72 |
| 2414 | `set_key_prob` | Y | Y |  |  |  | Y | Y |  | 74&#8209;76 |
| 2415 | `update_prob` | Y | Y |  |  |  | Y | Y |  | 78&#8209;80 |
| 2416 | `num_keys` | Y | Y |  |  |  | Y | Y |  | 82&#8209;84 |
| 2417 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 86&#8209;88 |
| 2418 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 90&#8209;92 |
| 2419 | `obst_rec_st_eph` |  |  |  | Y |  | Y | Y |  | 97&#8209;122 |

### Chap50/OptBinSearchTreeStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2420 | `new` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 2421 | `from_keys_probs` | Y | Y |  |  |  | Y | Y |  | 54&#8209;56 |
| 2422 | `from_key_probs` | Y | Y |  |  |  | Y | Y |  | 58&#8209;60 |
| 2423 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 2424 | `keys` | Y | Y |  |  |  | Y | Y |  | 66&#8209;68 |
| 2425 | `num_keys` | Y | Y |  |  |  | Y | Y |  | 70&#8209;72 |
| 2426 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 74&#8209;76 |
| 2427 | `obst_rec_st_per` |  |  |  | Y |  | Y | Y |  | 81&#8209;106 |

### Chap50/Probability.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2428 | `new` | Y | Y |  |  | Y |  |  | hole | 29 |
| 2429 | `value` | Y | Y |  |  | Y |  |  | hole | 33 |
| 2430 | `infinity` | Y | Y |  |  | Y |  |  | hole | 37 |
| 2431 | `zero` | Y | Y |  |  | Y |  |  | hole | 41 |
| 2432 | `default` |  | Y |  |  | Y |  |  | hole | 65 |
| 2433 | `eq` |  | Y |  |  | Y |  |  | hole | 72 |
| 2434 | `partial_cmp` |  | Y |  |  | Y |  |  | hole | 81 |
| 2435 | `cmp` |  | Y |  |  | Y |  |  | hole | 88 |
| 2436 | `hash` |  | Y |  |  | Y |  |  | hole | 110 |
| 2437 | `from` x2 |  | Y |  |  | Y |  |  | hole | 117 |
| 2438 | `add` |  | Y |  |  | Y |  |  | hole | 131 |
| 2439 | `sub` |  | Y |  |  | Y |  |  | hole | 138 |
| 2440 | `mul` |  | Y |  |  | Y |  |  | hole | 145 |
| 2441 | `div` |  | Y |  |  | Y |  |  | hole | 152 |

### Chap51/BottomUpDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2442 | `new_bu_eph_lock` |  |  |  | Y | Y |  |  | hole | 43 |
| 2443 | `new` | Y | Y |  |  |  | Y | Y |  | 52 |
| 2444 | `s_length` | Y | Y |  |  |  | Y | Y |  | 53 |
| 2445 | `t_length` | Y | Y |  |  |  | Y | Y |  | 54 |
| 2446 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 55 |
| 2447 | `set_s` | Y | Y |  |  |  | Y | Y |  | 56 |
| 2448 | `set_t` | Y | Y |  |  |  | Y | Y |  | 57 |
| 2449 | `med_bottom_up_parallel` | Y | Y |  |  |  | Y | Y |  | 58 |
| 2450 | `initialize_base_cases` | Y | Y |  |  |  | Y | Y |  | 59 |
| 2451 | `compute_diagonal_parallel` | Y | Y |  |  |  | Y | Y |  | 60 |
| 2452 | `compute_cell_value_static` | Y | Y |  |  |  | Y | Y |  | 61&#8209;67 |
| 2453 | `eq` |  | Y |  |  |  | Y | Y |  | 182&#8209;184 |
| 2454 | `default` |  | Y |  |  |  | Y | Y |  | 190&#8209;194 |

### Chap51/BottomUpDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2455 | `new_bu_per_lock` |  |  |  | Y | Y |  |  | hole | 43 |
| 2456 | `new` | Y | Y |  |  |  | Y | Y |  | 52 |
| 2457 | `s_length` | Y | Y |  |  |  | Y | Y |  | 53 |
| 2458 | `t_length` | Y | Y |  |  |  | Y | Y |  | 54 |
| 2459 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 55 |
| 2460 | `med_bottom_up_parallel` | Y | Y |  |  |  | Y | Y |  | 56 |
| 2461 | `initialize_base_cases` | Y | Y |  |  |  | Y | Y |  | 57 |
| 2462 | `compute_diagonal_parallel` | Y | Y |  |  |  | Y | Y |  | 58 |
| 2463 | `compute_cell_value_static` | Y | Y |  |  |  | Y | Y |  | 59&#8209;65 |
| 2464 | `eq` |  | Y |  |  |  | Y | Y |  | 178&#8209;180 |
| 2465 | `default` |  | Y |  |  |  | Y | Y |  | 186&#8209;190 |

### Chap51/BottomUpDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2466 | `new` | Y | Y |  |  |  | Y | Y |  | 39 |
| 2467 | `s_length` | Y | Y |  |  |  | Y | Y |  | 40 |
| 2468 | `t_length` | Y | Y |  |  |  | Y | Y |  | 41 |
| 2469 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 42 |
| 2470 | `set_s` | Y | Y |  |  |  | Y | Y |  | 43 |
| 2471 | `set_t` | Y | Y |  |  |  | Y | Y |  | 44 |
| 2472 | `med_bottom_up` | Y | Y |  |  |  | Y | Y |  | 45 |
| 2473 | `initialize_base_cases` | Y | Y |  |  |  | Y | Y |  | 46 |
| 2474 | `compute_diagonal` | Y | Y |  |  |  | Y | Y |  | 47 |
| 2475 | `compute_cell_value` | Y | Y |  |  |  | Y | Y |  | 48 |
| 2476 | `eq` |  | Y |  |  |  | Y | Y |  | 132&#8209;134 |
| 2477 | `default` |  | Y |  |  |  | Y | Y |  | 140&#8209;144 |

### Chap51/BottomUpDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2478 | `new` | Y | Y |  |  |  | Y | Y |  | 39 |
| 2479 | `s_length` | Y | Y |  |  |  | Y | Y |  | 40 |
| 2480 | `t_length` | Y | Y |  |  |  | Y | Y |  | 41 |
| 2481 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 42 |
| 2482 | `med_bottom_up` | Y | Y |  |  |  | Y | Y |  | 43 |
| 2483 | `initialize_base_cases` | Y | Y |  |  |  | Y | Y |  | 44 |
| 2484 | `compute_diagonal` | Y | Y |  |  |  | Y | Y |  | 45 |
| 2485 | `compute_cell_value` | Y | Y |  |  |  | Y | Y |  | 46 |
| 2486 | `eq` |  | Y |  |  |  | Y | Y |  | 131&#8209;133 |
| 2487 | `default` |  | Y |  |  |  | Y | Y |  | 139&#8209;143 |

### Chap51/TopDownDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2488 | `new_td_eph_lock` |  |  |  | Y | Y |  |  | hole | 37 |
| 2489 | `new` | Y | Y |  |  |  | Y | Y |  | 57 |
| 2490 | `med_memoized_concurrent` | Y | Y |  |  |  | Y | Y |  | 58 |
| 2491 | `med_memoized_parallel` | Y | Y |  |  |  | Y | Y |  | 59 |
| 2492 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 60 |
| 2493 | `is_memoized` | Y | Y |  |  |  | Y | Y |  | 61 |
| 2494 | `get_memoized` | Y | Y |  |  |  | Y | Y |  | 62 |
| 2495 | `insert_memo` | Y | Y |  |  |  | Y | Y |  | 63 |
| 2496 | `s_length` | Y | Y |  |  |  | Y | Y |  | 64 |
| 2497 | `t_length` | Y | Y |  |  |  | Y | Y |  | 65 |
| 2498 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 66 |
| 2499 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 67 |
| 2500 | `set_s` | Y | Y |  |  |  | Y | Y |  | 68 |
| 2501 | `set_t` | Y | Y |  |  |  | Y | Y |  | 69 |
| 2502 | `med_recursive_concurrent` | Y | Y |  |  |  | Y | Y |  | 70 |
| 2503 | `med_recursive_parallel` | Y | Y |  |  |  | Y | Y |  | 71 |
| 2504 | `eq` |  | Y |  |  |  | Y | Y |  | 225&#8209;235 |
| 2505 | `default` |  | Y |  |  |  | Y | Y |  | 239&#8209;245 |

### Chap51/TopDownDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2506 | `new_td_per_lock` |  |  |  | Y | Y |  |  | hole | 37 |
| 2507 | `new` | Y | Y |  |  |  | Y | Y |  | 57 |
| 2508 | `med_memoized_concurrent` | Y | Y |  |  |  | Y | Y |  | 58 |
| 2509 | `med_memoized_parallel` | Y | Y |  |  |  | Y | Y |  | 59 |
| 2510 | `with_memo_table` | Y | Y |  |  |  | Y | Y |  | 60 |
| 2511 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 61 |
| 2512 | `is_memoized` | Y | Y |  |  |  | Y | Y |  | 62 |
| 2513 | `get_memoized` | Y | Y |  |  |  | Y | Y |  | 63 |
| 2514 | `s_length` | Y | Y |  |  |  | Y | Y |  | 64 |
| 2515 | `t_length` | Y | Y |  |  |  | Y | Y |  | 65 |
| 2516 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 66 |
| 2517 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 67 |
| 2518 | `med_recursive_concurrent` | Y | Y |  |  |  | Y | Y |  | 68 |
| 2519 | `med_recursive_parallel` | Y | Y |  |  |  | Y | Y |  | 69 |
| 2520 | `eq` |  | Y |  |  |  | Y | Y |  | 217&#8209;227 |
| 2521 | `default` |  | Y |  |  |  | Y | Y |  | 231&#8209;237 |

### Chap51/TopDownDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2522 | `new` | Y | Y |  |  |  | Y | Y |  | 43 |
| 2523 | `med_memoized` | Y | Y |  |  |  | Y | Y |  | 44 |
| 2524 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 45 |
| 2525 | `is_memoized` | Y | Y |  |  |  | Y | Y |  | 46 |
| 2526 | `get_memoized` | Y | Y |  |  |  | Y | Y |  | 47 |
| 2527 | `insert_memo` | Y | Y |  |  |  | Y | Y |  | 48 |
| 2528 | `s_length` | Y | Y |  |  |  | Y | Y |  | 49 |
| 2529 | `t_length` | Y | Y |  |  |  | Y | Y |  | 50 |
| 2530 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 51 |
| 2531 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 52 |
| 2532 | `set_s` | Y | Y |  |  |  | Y | Y |  | 53 |
| 2533 | `set_t` | Y | Y |  |  |  | Y | Y |  | 54 |
| 2534 | `med_recursive` | Y | Y |  |  |  | Y | Y |  | 55 |
| 2535 | `default` |  | Y |  |  |  | Y | Y |  | 128&#8209;134 |

### Chap51/TopDownDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2536 | `new` | Y | Y |  |  |  | Y | Y |  | 43 |
| 2537 | `med_memoized` | Y | Y |  |  |  | Y | Y |  | 44 |
| 2538 | `with_memo_table` | Y | Y |  |  |  | Y | Y |  | 45 |
| 2539 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 46 |
| 2540 | `is_memoized` | Y | Y |  |  |  | Y | Y |  | 47 |
| 2541 | `get_memoized` | Y | Y |  |  |  | Y | Y |  | 48 |
| 2542 | `s_length` | Y | Y |  |  |  | Y | Y |  | 49 |
| 2543 | `t_length` | Y | Y |  |  |  | Y | Y |  | 50 |
| 2544 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 51 |
| 2545 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 52 |
| 2546 | `med_recursive` | Y | Y |  |  |  | Y | Y |  | 53 |
| 2547 | `default` |  | Y |  |  |  | Y | Y |  | 122&#8209;128 |

### Chap52/AdjMatrixGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2548 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 65&#8209;68 |
| 2549 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 75&#8209;78 |
| 2550 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 85&#8209;88 |
| 2551 | `new` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;109 |
| 2552 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;121 |
| 2553 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 2554 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;140 |
| 2555 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 2556 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;156 |
| 2557 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;164 |
| 2558 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;179 |
| 2559 | `complement` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;189 |

### Chap52/AdjMatrixGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2560 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 66&#8209;69 |
| 2561 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 76&#8209;79 |
| 2562 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 86&#8209;89 |
| 2563 | `new` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;110 |
| 2564 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 2565 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;129 |
| 2566 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;136 |
| 2567 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;150 |
| 2568 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;160 |
| 2569 | `complement` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;170 |

### Chap52/AdjMatrixGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2570 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 65&#8209;68 |
| 2571 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 75&#8209;78 |
| 2572 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 85&#8209;88 |
| 2573 | `new` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;109 |
| 2574 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;121 |
| 2575 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 2576 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;140 |
| 2577 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 2578 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;156 |
| 2579 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;164 |
| 2580 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;179 |
| 2581 | `complement` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;189 |

### Chap52/AdjMatrixGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2582 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 68&#8209;71 |
| 2583 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 78&#8209;81 |
| 2584 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 88&#8209;91 |
| 2585 | `new` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;112 |
| 2586 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;124 |
| 2587 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;129 |
| 2588 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;143 |
| 2589 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;148 |
| 2590 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;159 |
| 2591 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;167 |
| 2592 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;182 |
| 2593 | `complement` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;192 |

### Chap52/AdjSeqGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2594 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 55&#8209;58 |
| 2595 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 66&#8209;68 |
| 2596 | `new` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;85 |
| 2597 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;89 |
| 2598 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;102 |
| 2599 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;109 |
| 2600 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;117 |
| 2601 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 2602 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;142 |

### Chap52/AdjSeqGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2603 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 51&#8209;54 |
| 2604 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 61&#8209;63 |
| 2605 | `new` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;80 |
| 2606 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 2607 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;97 |
| 2608 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;104 |
| 2609 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;112 |
| 2610 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |

### Chap52/AdjSeqGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2611 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 53&#8209;56 |
| 2612 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 64&#8209;66 |
| 2613 | `new` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;83 |
| 2614 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;93 |
| 2615 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 2616 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;110 |
| 2617 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;117 |
| 2618 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;125 |
| 2619 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 2620 | `set_neighbors` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;145 |
| 2621 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;165 |

### Chap52/AdjSeqGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2622 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 54&#8209;57 |
| 2623 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 65&#8209;67 |
| 2624 | `new` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;84 |
| 2625 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;94 |
| 2626 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 2627 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;111 |
| 2628 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 2629 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;126 |
| 2630 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 2631 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;148 |
| 2632 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;163 |

### Chap52/AdjTableGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2633 | `empty` | Y | Y |  |  | Y |  | Y |  | 58 |
| 2634 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 60 |
| 2635 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;64 |
| 2636 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 2637 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;72 |
| 2638 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 74 |
| 2639 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 2640 | `delete_vertex` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 2641 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;86 |
| 2642 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;91 |
| 2643 | `default` |  | Y |  |  | Y |  | Y |  | 271 |

### Chap52/AdjTableGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2644 | `lemma_sum_adj_sizes_monotone` |  |  |  | Y | Y |  |  | unknown | 67&#8209;69 |
| 2645 | `empty` | Y | Y |  |  | Y |  | Y |  | 80 |
| 2646 | `from_table` | Y | Y |  |  | Y |  | Y |  | 82 |
| 2647 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 84 |
| 2648 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 2649 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 90 |
| 2650 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 92 |
| 2651 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 94 |
| 2652 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 96 |
| 2653 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 98 |
| 2654 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 100 |
| 2655 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 102 |
| 2656 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 104 |

### Chap52/AdjTableGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2657 | `empty` | Y | Y |  |  | Y |  | Y |  | 48 |
| 2658 | `from_table` | Y | Y |  |  | Y |  | Y |  | 50 |
| 2659 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 52 |
| 2660 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;56 |
| 2661 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;59 |
| 2662 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 2663 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;67 |
| 2664 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 69 |
| 2665 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 2666 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 2667 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 2668 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;86 |

### Chap52/EdgeSetGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2669 | `empty` | Y | Y |  |  | Y |  | Y |  | 49 |
| 2670 | `from_vertices_and_edges` | Y | Y |  |  | Y |  | Y |  | 51 |
| 2671 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 53 |
| 2672 | `num_edges` | Y | Y |  |  | Y |  | Y |  | 55 |
| 2673 | `vertices` | Y | Y |  |  | Y |  | Y |  | 57 |
| 2674 | `edges` | Y | Y |  |  | Y |  | Y |  | 59 |
| 2675 | `has_edge` | Y | Y |  |  | Y |  | Y |  | 61 |
| 2676 | `out_neighbors` | Y | Y |  |  | Y |  |  | hole | 63&#8209;64 |
| 2677 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 66 |
| 2678 | `insert_vertex` | Y | Y |  |  | Y |  | Y |  | 68 |
| 2679 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 2680 | `insert_edge` | Y | Y |  |  | Y |  | Y |  | 73 |
| 2681 | `delete_edge` | Y | Y |  |  | Y |  | Y |  | 75 |
| 2682 | `default` |  | Y |  |  | Y |  | Y |  | 187 |

### Chap52/EdgeSetGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2683 | `empty` | Y | Y |  |  | Y |  | Y |  | 46 |
| 2684 | `from_vertices_and_edges` | Y | Y |  |  | Y |  | Y |  | 48 |
| 2685 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 50 |
| 2686 | `num_edges` | Y | Y |  |  | Y |  | Y |  | 52 |
| 2687 | `vertices` | Y | Y |  |  | Y |  | Y |  | 54 |
| 2688 | `edges` | Y | Y |  |  | Y |  | Y |  | 56 |
| 2689 | `has_edge` | Y | Y |  |  | Y |  | Y |  | 58 |
| 2690 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 2691 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 63 |
| 2692 | `insert_vertex` | Y | Y |  |  | Y |  | Y |  | 65 |
| 2693 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 2694 | `insert_edge` | Y | Y |  |  | Y |  | Y |  | 70 |
| 2695 | `delete_edge` | Y | Y |  |  | Y |  | Y |  | 72 |

### Chap52/EdgeSetGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2696 | `empty` | Y | Y |  |  | Y |  | Y |  | 48 |
| 2697 | `from_vertices_and_edges` | Y | Y |  |  | Y |  | Y |  | 50 |
| 2698 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 52 |
| 2699 | `num_edges` | Y | Y |  |  | Y |  | Y |  | 54 |
| 2700 | `vertices` | Y | Y |  |  | Y |  | Y |  | 56 |
| 2701 | `edges` | Y | Y |  |  | Y |  | Y |  | 58 |
| 2702 | `has_edge` | Y | Y |  |  | Y |  | Y |  | 60 |
| 2703 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 2704 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 65 |
| 2705 | `insert_vertex` | Y | Y |  |  | Y |  | Y |  | 67 |
| 2706 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 2707 | `insert_edge` | Y | Y |  |  | Y |  | Y |  | 72 |
| 2708 | `delete_edge` | Y | Y |  |  | Y |  | Y |  | 74 |

### Chap53/GraphSearchMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2709 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 30 |
| 2710 | `graph_search` | Y |  |  | Y | Y |  |  | hole | 36&#8209;39 |
| 2711 | `graph_search_multi` | Y |  |  | Y | Y |  |  | hole | 43&#8209;46 |
| 2712 | `reachable` | Y |  |  | Y | Y |  |  | hole | 50&#8209;52 |
| 2713 | `explore` |  |  |  | Y | Y |  | Y |  | 99&#8209;108 |

### Chap53/GraphSearchStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2714 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 25 |
| 2715 | `graph_search` | Y |  |  | Y | Y |  |  | hole | 31&#8209;34 |
| 2716 | `graph_search_multi` | Y |  |  | Y | Y |  |  | hole | 38&#8209;41 |
| 2717 | `reachable` | Y |  |  | Y | Y |  |  | hole | 45&#8209;47 |
| 2718 | `explore` |  |  |  | Y | Y |  | Y |  | 91&#8209;100 |

### Chap53/GraphSearchStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2719 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 33 |
| 2720 | `graph_search` | Y |  |  | Y | Y |  |  | hole | 39&#8209;42 |
| 2721 | `graph_search_multi` | Y |  |  | Y | Y |  |  | hole | 46&#8209;49 |
| 2722 | `reachable` | Y |  |  | Y | Y |  |  | hole | 53&#8209;55 |
| 2723 | `explore` |  |  |  | Y | Y |  | Y |  | 103&#8209;112 |

### Chap53/PQMinStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2724 | `priority` | Y |  | Y |  | Y |  |  | hole | 30 |
| 2725 | `pq_min` | Y |  |  | Y | Y |  |  | hole | 36&#8209;39 |
| 2726 | `pq_min_multi` | Y |  |  | Y | Y |  |  | hole | 43&#8209;46 |
| 2727 | `new` | Y |  | Y |  | Y |  |  | hole | 50 |
| 2728 | `find_min_priority` |  |  |  | Y | Y |  | Y |  | 95 |
| 2729 | `explore` |  |  |  | Y | Y |  | Y |  | 104&#8209;114 |

### Chap53/PQMinStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2730 | `priority` | Y |  | Y |  | Y |  |  | hole | 29 |
| 2731 | `pq_min` | Y |  |  | Y | Y |  |  | hole | 42&#8209;45 |
| 2732 | `pq_min_multi` | Y |  |  | Y | Y |  |  | hole | 49&#8209;52 |
| 2733 | `new` | Y |  | Y |  | Y |  |  | hole | 56 |
| 2734 | `find_min_priority` |  |  |  | Y | Y |  | Y |  | 101 |
| 2735 | `explore` |  |  |  | Y | Y |  | Y |  | 112&#8209;122 |

### Chap54/BFSMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2736 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 55&#8209;60 |
| 2737 | `lemma_set_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 63&#8209;80 |
| 2738 | `lemma_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 94&#8209;105 |
| 2739 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 115&#8209;121 |
| 2740 | `lemma_set_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 125&#8209;142 |
| 2741 | `copy_distances` |  |  |  | Y | Y |  |  | unknown | 157&#8209;162 |
| 2742 | `copy_graph` |  |  |  | Y | Y |  |  | unknown | 175&#8209;183 |
| 2743 | `lemma_copy_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 208&#8209;221 |
| 2744 | `lemma_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 233&#8209;244 |
| 2745 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;264 |
| 2746 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;273 |
| 2747 | `bfs` | Y |  |  | Y | Y |  |  | unknown | 277&#8209;287 |
| 2748 | `bfs_tree` | Y |  |  | Y | Y |  |  | unknown | 291&#8209;305 |
| 2749 | `process_frontier_parallel` |  |  |  | Y | Y |  |  | unknown | 311&#8209;333 |
| 2750 | `process_frontier_tree_parallel` |  |  |  | Y | Y |  |  | unknown | 595&#8209;613 |

### Chap54/BFSMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2751 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 55&#8209;60 |
| 2752 | `lemma_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 63&#8209;80 |
| 2753 | `lemma_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 94&#8209;105 |
| 2754 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 115&#8209;121 |
| 2755 | `lemma_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 125&#8209;142 |
| 2756 | `copy_distances` |  |  |  | Y | Y |  |  | unknown | 157&#8209;162 |
| 2757 | `copy_graph` |  |  |  | Y | Y |  |  | unknown | 175&#8209;183 |
| 2758 | `lemma_copy_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 209&#8209;222 |
| 2759 | `lemma_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 235&#8209;246 |
| 2760 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;266 |
| 2761 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;275 |
| 2762 | `bfs` | Y |  |  | Y | Y |  |  | unknown | 279&#8209;289 |
| 2763 | `bfs_tree` | Y |  |  | Y | Y |  |  | unknown | 293&#8209;307 |
| 2764 | `process_frontier_parallel` |  |  |  | Y | Y |  |  | unknown | 314&#8209;336 |
| 2765 | `process_frontier_tree_parallel` |  |  |  | Y | Y |  |  | unknown | 612&#8209;630 |

### Chap54/BFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2766 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 57&#8209;62 |
| 2767 | `lemma_set_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 65&#8209;82 |
| 2768 | `bfs` | Y |  |  | Y | Y |  |  | unknown | 99&#8209;109 |
| 2769 | `bfs_tree` | Y |  |  | Y | Y |  |  | unknown | 113&#8209;127 |
| 2770 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;137 |
| 2771 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;144 |
| 2772 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 149&#8209;155 |
| 2773 | `lemma_set_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 160&#8209;177 |

### Chap54/BFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2774 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 57&#8209;62 |
| 2775 | `lemma_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 65&#8209;82 |
| 2776 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 96&#8209;102 |
| 2777 | `lemma_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 106&#8209;123 |
| 2778 | `bfs` | Y |  |  | Y | Y |  |  | unknown | 140&#8209;150 |
| 2779 | `bfs_tree` | Y |  |  | Y | Y |  |  | unknown | 154&#8209;168 |
| 2780 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;178 |
| 2781 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;185 |

### Chap55/CycleDetectStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2782 | `has_cycle` | Y |  |  | Y | Y |  |  | unknown | 28 |
| 2783 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 35&#8209;53 |

### Chap55/CycleDetectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2784 | `has_cycle` | Y |  |  | Y | Y |  |  | unknown | 38 |
| 2785 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 45&#8209;63 |

### Chap55/DFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2786 | `dfs` | Y |  |  | Y | Y |  |  | unknown | 21 |
| 2787 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 25&#8209;41 |

### Chap55/DFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2788 | `dfs` | Y |  |  | Y | Y |  |  | unknown | 22 |
| 2789 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 27&#8209;43 |

### Chap55/SCCStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2790 | `scc` | Y |  |  | Y | Y |  |  | unknown | 33&#8209;34 |
| 2791 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 40&#8209;41 |
| 2792 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 77&#8209;79 |
| 2793 | `check_wf_adj_list_eph` |  |  |  | Y | Y |  |  | unknown | 139&#8209;140 |
| 2794 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 219&#8209;235 |

### Chap55/SCCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2795 | `scc` | Y |  |  | Y | Y |  |  | unknown | 31&#8209;32 |
| 2796 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 38&#8209;54 |
| 2797 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 89&#8209;90 |
| 2798 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 134&#8209;136 |
| 2799 | `check_wf_adj_list_per` |  |  |  | Y | Y |  |  | unknown | 194&#8209;195 |
| 2800 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 238&#8209;254 |

### Chap55/TopoSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2801 | `lemma_set_true_decreases_num_false` |  |  |  | Y | Y |  |  | unknown | 46&#8209;52 |
| 2802 | `topo_sort` | Y |  |  | Y | Y |  |  | unknown | 68 |
| 2803 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 75&#8209;91 |
| 2804 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 128&#8209;147 |
| 2805 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 196&#8209;197 |

### Chap55/TopoSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2806 | `topo_sort` | Y |  |  | Y | Y |  |  | unknown | 39 |
| 2807 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 45&#8209;61 |
| 2808 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 97&#8209;116 |
| 2809 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 162&#8209;163 |

### Chap56/AllPairsResultStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2810 | `new` | Y | Y |  |  | Y |  |  | unknown | 36 |
| 2811 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 38 |
| 2812 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 40 |
| 2813 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 42 |
| 2814 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 44 |
| 2815 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 46 |
| 2816 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 48 |

### Chap56/AllPairsResultStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2817 | `new` | Y | Y |  |  | Y |  |  | unknown | 26 |
| 2818 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 28 |
| 2819 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 30 |
| 2820 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 32 |
| 2821 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 34 |
| 2822 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 36 |
| 2823 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 38 |

### Chap56/AllPairsResultStPerF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2824 | `new` | Y | Y |  |  | Y |  |  | unknown | 35 |
| 2825 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 37 |
| 2826 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 39 |
| 2827 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 41 |
| 2828 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 43 |
| 2829 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 45 |
| 2830 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 47 |

### Chap56/AllPairsResultStPerI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2831 | `new` | Y | Y |  |  | Y |  |  | unknown | 25 |
| 2832 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 27 |
| 2833 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 29 |
| 2834 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 31 |
| 2835 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 33 |
| 2836 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 35 |
| 2837 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 37 |

### Chap56/Example56_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2838 | `example_path_weight_int` | Y |  |  | Y | Y |  |  | hole | 28 |
| 2839 | `example_path_weight_float` | Y |  |  | Y | Y |  |  | hole | 32 |
| 2840 | `example_negative_weights` | Y |  |  | Y | Y |  |  | hole | 36 |

### Chap56/Example56_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2841 | `example_negative_cycle` | Y |  |  | Y | Y |  |  | hole | 28 |
| 2842 | `example_undefined_shortest_path` | Y |  |  | Y | Y |  |  | hole | 32 |

### Chap56/PathWeightUtilsStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2843 | `path_weight_int` | Y |  |  | Y | Y |  | Y |  | 37 |
| 2844 | `path_weight_float` | Y |  |  | Y | Y |  | Y |  | 39&#8209;42 |
| 2845 | `validate_subpath_property_int` | Y |  |  | Y | Y |  | Y |  | 44&#8209;48 |
| 2846 | `validate_subpath_property_float` | Y |  |  | Y | Y |  | Y |  | 50&#8209;54 |
| 2847 | `f64_approx_eq` |  |  |  | Y | Y |  |  | hole | 164 |

### Chap56/PathWeightUtilsStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2848 | `path_weight_int` | Y |  |  | Y | Y |  | Y |  | 36 |
| 2849 | `path_weight_float` | Y |  |  | Y | Y |  | Y |  | 38&#8209;41 |
| 2850 | `validate_subpath_property_int` | Y |  |  | Y | Y |  | Y |  | 43&#8209;47 |
| 2851 | `validate_subpath_property_float` | Y |  |  | Y | Y |  | Y |  | 49&#8209;53 |
| 2852 | `f64_approx_eq` |  |  |  | Y | Y |  |  | hole | 163 |

### Chap56/SSSPResultStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2853 | `new` |  |  | Y |  | Y |  |  | unknown | 36&#8209;37 |
| 2854 | `get_distance` |  |  | Y |  | Y |  | Y |  | 62 |
| 2855 | `set_distance` |  |  | Y |  | Y |  | Y |  | 69 |
| 2856 | `get_predecessor` |  |  | Y |  | Y |  | Y |  | 75 |
| 2857 | `set_predecessor` |  |  | Y |  | Y |  | Y |  | 83 |
| 2858 | `is_reachable` |  |  | Y |  | Y |  | Y |  | 89 |
| 2859 | `extract_path` |  |  | Y |  |  | Y | Y |  | 97&#8209;112 |

### Chap56/SSSPResultStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2860 | `new` | Y | Y |  |  | Y |  |  | unknown | 27&#8209;28 |
| 2861 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 30 |
| 2862 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 32 |
| 2863 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 34 |
| 2864 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 36 |
| 2865 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 38 |
| 2866 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 40 |

### Chap56/SSSPResultStPerF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2867 | `new` | Y | Y |  |  | Y |  |  | unknown | 45&#8209;46 |
| 2868 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 48 |
| 2869 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 50 |
| 2870 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 52 |
| 2871 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 54 |
| 2872 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 56 |
| 2873 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 58 |

### Chap56/SSSPResultStPerI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2874 | `new` | Y | Y |  |  | Y |  |  | unknown | 25&#8209;26 |
| 2875 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 28 |
| 2876 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 30 |
| 2877 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 32 |
| 2878 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 34 |
| 2879 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 36 |
| 2880 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 38 |

### Chap57/DijkstraStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2881 | `eq` |  | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 2882 | `cmp` |  | Y |  |  |  | Y | Y |  | 49&#8209;52 |
| 2883 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 56 |

### Chap57/DijkstraStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2884 | `dijkstra` | Y |  |  | Y | Y |  |  | hole | 62 |
| 2885 | `pq_entry_new` |  |  |  | Y | Y |  |  | unknown | 67&#8209;68 |
| 2886 | `cmp` |  | Y |  |  | Y |  | Y |  | 74 |
| 2887 | `partial_cmp` |  | Y |  |  | Y |  | Y |  | 86 |

### Chap57/StackStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2888 | `new` | Y | Y |  |  | Y |  |  | unknown | 55 |
| 2889 | `push` | Y | Y |  |  | Y |  |  | unknown | 57 |
| 2890 | `pop` | Y | Y |  |  | Y |  |  | unknown | 59 |
| 2891 | `peek` | Y | Y |  |  | Y |  |  | unknown | 61 |
| 2892 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 63 |
| 2893 | `size` | Y | Y |  |  | Y |  |  | unknown | 65 |
| 2894 | `default` |  | Y |  |  | Y |  |  | unknown | 116&#8209;117 |

### Chap58/BellmanFordStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2895 | `bellman_ford` | Y |  |  | Y | Y |  | Y |  | 26&#8209;27 |
| 2896 | `reconstruct_predecessors` |  |  |  | Y |  | Y | Y |  | 71&#8209;92 |

### Chap59/JohnsonMtEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2897 | `johnson_apsp` | Y |  |  | Y | Y |  | Y |  | 37 |
| 2898 | `parallel_dijkstra_all` |  |  |  | Y |  | Y | Y |  | 79&#8209;138 |
| 2899 | `add_dummy_source` |  |  |  | Y |  | Y | Y |  | 140&#8209;164 |
| 2900 | `reweight_graph` |  |  |  | Y |  | Y | Y |  | 166&#8209;190 |
| 2901 | `create_negative_cycle_result` |  |  |  | Y |  | Y | Y |  | 192&#8209;205 |

### Chap59/JohnsonStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2902 | `johnson_apsp` | Y |  |  | Y | Y |  | Y |  | 34 |
| 2903 | `add_dummy_source` |  |  |  | Y |  | Y | Y |  | 100&#8209;126 |
| 2904 | `reweight_graph` |  |  |  | Y |  | Y | Y |  | 128&#8209;152 |
| 2905 | `create_negative_cycle_result` |  |  |  | Y |  | Y | Y |  | 154&#8209;167 |

### Chap61/EdgeContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2906 | `edge_contract_mt` | Y |  |  | Y | Y |  | Y |  | 32&#8209;35 |
| 2907 | `contract_round_mt` | Y |  |  | Y | Y |  | Y |  | 39&#8209;42 |
| 2908 | `build_edges_parallel` |  |  |  | Y |  | Y | Y |  | 109&#8209;162 |

### Chap61/EdgeContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2909 | `edge_contract` | Y |  |  | Y | Y |  | Y |  | 30&#8209;33 |
| 2910 | `contract_round` | Y |  |  | Y | Y |  | Y |  | 37 |

### Chap61/VertexMatchingMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2911 | `parallel_matching_mt` | Y |  |  | Y | Y |  | Y |  | 30 |
| 2912 | `flip_coins_parallel` |  |  |  | Y |  | Y | Y |  | 78&#8209;100 |
| 2913 | `select_edges_parallel` |  |  |  | Y |  | Y | Y |  | 102&#8209;133 |
| 2914 | `select_edges_recursive` |  |  |  | Y |  | Y | Y |  | 135&#8209;178 |
| 2915 | `should_select_edge` |  |  |  | Y |  | Y | Y |  | 180&#8209;211 |

### Chap61/VertexMatchingStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2916 | `greedy_matching` | Y |  |  | Y | Y |  | Y |  | 26 |
| 2917 | `parallel_matching_st` | Y |  |  | Y | Y |  | Y |  | 30 |

### Chap62/StarContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2918 | `star_contract_mt` | Y |  |  | Y | Y |  | Y |  | 31&#8209;35 |
| 2919 | `contract_to_vertices_mt` | Y |  |  | Y | Y |  | Y |  | 39 |
| 2920 | `build_quotient_graph_parallel` |  |  |  | Y |  | Y | Y |  | 83&#8209;104 |
| 2921 | `route_edges_parallel` |  |  |  | Y |  | Y | Y |  | 106&#8209;156 |

### Chap62/StarContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2922 | `star_contract` | Y |  |  | Y | Y |  | Y |  | 26&#8209;30 |
| 2923 | `contract_to_vertices` | Y |  |  | Y | Y |  | Y |  | 34 |
| 2924 | `build_quotient_graph` |  |  |  | Y |  | Y | Y |  | 77&#8209;108 |

### Chap62/StarPartitionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2925 | `parallel_star_partition` | Y |  |  | Y | Y |  | Y |  | 26&#8209;29 |

### Chap62/StarPartitionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2926 | `sequential_star_partition` | Y |  |  | Y | Y |  | Y |  | 25 |

### Chap63/ConnectivityMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2927 | `count_components_mt` | Y |  |  | Y | Y |  | Y |  | 36 |
| 2928 | `connected_components_mt` | Y |  |  | Y | Y |  | Y |  | 40&#8209;43 |
| 2929 | `count_components_hof` | Y |  |  | Y | Y |  | Y |  | 47 |
| 2930 | `connected_components_hof` | Y |  |  | Y | Y |  | Y |  | 51&#8209;54 |
| 2931 | `build_quotient_edges_parallel` |  |  |  | Y |  | Y | Y |  | 127&#8209;143 |
| 2932 | `route_edges_parallel` |  |  |  | Y |  | Y | Y |  | 145&#8209;195 |
| 2933 | `compose_maps_parallel` |  |  |  | Y |  | Y | Y |  | 197&#8209;212 |

### Chap63/ConnectivityStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2934 | `count_components` | Y |  |  | Y | Y |  | Y |  | 31 |
| 2935 | `connected_components` | Y |  |  | Y | Y |  | Y |  | 35 |
| 2936 | `count_components_hof` | Y |  |  | Y | Y |  | Y |  | 39 |
| 2937 | `connected_components_hof` | Y |  |  | Y | Y |  | Y |  | 43 |
| 2938 | `build_quotient_edges` |  |  |  | Y |  | Y | Y |  | 118&#8209;146 |

### Chap64/SpanTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2939 | `new_spanning_edges_lock` |  |  |  | Y | Y |  |  | hole | 36&#8209;38 |
| 2940 | `new_valid_lock` |  |  |  | Y | Y |  |  | hole | 47 |
| 2941 | `spanning_tree_star_contraction_mt` | Y |  |  | Y | Y |  | Y |  | 54&#8209;56 |
| 2942 | `verify_spanning_tree` | Y |  |  | Y | Y |  | Y |  | 60 |

### Chap64/SpanTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2943 | `spanning_tree_star_contraction` | Y |  |  | Y | Y |  | Y |  | 27 |
| 2944 | `verify_spanning_tree` | Y |  |  | Y | Y |  | Y |  | 31 |

### Chap64/TSPApproxStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2945 | `euler_tour` | Y |  |  | Y |  | Y | Y |  | 40&#8209;42 |
| 2946 | `shortcut_tour` | Y |  |  | Y |  | Y | Y |  | 44&#8209;46 |
| 2947 | `tour_weight` | Y |  |  | Y |  | Y | Y |  | 48&#8209;53 |
| 2948 | `approx_metric_tsp` | Y |  |  | Y |  | Y | Y |  | 55&#8209;60 |
| 2949 | `euler_tour_dfs` |  |  |  | Y |  | Y | Y |  | 92&#8209;143 |
| 2950 | `get_neighbors` |  |  |  | Y |  | Y | Y |  | 208&#8209;222 |
| 2951 | `get_edge_weight` |  |  |  | Y |  | Y | Y |  | 224&#8209;239 |

### Chap65/KruskalStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2952 | `kruskal_mst` | Y |  |  | Y | Y |  | Y |  | 26&#8209;28 |
| 2953 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 32 |
| 2954 | `verify_mst_size` | Y |  |  | Y | Y |  | Y |  | 36&#8209;39 |

### Chap65/PrimStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2955 | `prim_mst` | Y |  |  | Y | Y |  | Y |  | 43&#8209;46 |
| 2956 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 50 |
| 2957 | `pq_entry_new` |  |  |  | Y |  | Y | Y |  | 59&#8209;69 |
| 2958 | `cmp` |  | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 2959 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 80&#8209;82 |

### Chap65/UnionFindStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2960 | `new` | Y | Y |  |  | Y |  | Y |  | 22 |
| 2961 | `insert` | Y | Y |  |  | Y |  | Y |  | 26 |
| 2962 | `find` | Y | Y |  |  | Y |  | Y |  | 30 |
| 2963 | `union` | Y | Y |  |  | Y |  | Y |  | 34 |
| 2964 | `equals` | Y | Y |  |  | Y |  | Y |  | 38 |
| 2965 | `num_sets` | Y | Y |  |  | Y |  | Y |  | 42 |
| 2966 | `default` |  | Y |  |  |  | Y | Y |  | 128&#8209;130 |

### Chap66/BoruvkaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2967 | `vertex_bridges_mt` | Y |  |  | Y | Y |  | Y |  | 36&#8209;40 |
| 2968 | `bridge_star_partition_mt` | Y |  |  | Y | Y |  | Y |  | 44&#8209;48 |
| 2969 | `boruvka_mst_mt` | Y |  |  | Y | Y |  | Y |  | 52&#8209;57 |
| 2970 | `boruvka_mst_mt_with_seed` | Y |  |  | Y | Y |  | Y |  | 61&#8209;65 |
| 2971 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 69&#8209;72 |
| 2972 | `filter_tail_to_head_mt` |  |  |  | Y |  | Y | Y |  | 181&#8209;232 |
| 2973 | `reroute_edges_mt` |  |  |  | Y |  | Y | Y |  | 285&#8209;328 |

### Chap66/BoruvkaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2974 | `vertex_bridges` | Y |  |  | Y | Y |  | Y |  | 35&#8209;37 |
| 2975 | `bridge_star_partition` | Y |  |  | Y | Y |  | Y |  | 41&#8209;45 |
| 2976 | `boruvka_mst` | Y |  |  | Y | Y |  | Y |  | 49&#8209;54 |
| 2977 | `boruvka_mst_with_seed` | Y |  |  | Y | Y |  | Y |  | 58&#8209;62 |
| 2978 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 66&#8209;69 |

### arithmetic/power2_plus.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2979 | `lemma_pow2_mono` |  |  |  | Y | Y |  |  | unknown | 13&#8209;15 |
| 2980 | `lemma_pow2_46_lt_u64_max` |  |  |  | Y | Y |  |  | unknown | 25&#8209;26 |
| 2981 | `lemma_pow2_63_lt_u64_max` |  |  |  | Y | Y |  |  | unknown | 33&#8209;34 |
| 2982 | `lemma_pow2_lt_u64_max` |  |  |  | Y | Y |  |  | unknown | 41&#8209;43 |
| 2983 | `lemma_pow2_31_lt_u32_max` |  |  |  | Y | Y |  |  | unknown | 53&#8209;54 |
| 2984 | `lemma_pow2_lt_u32_max` |  |  |  | Y | Y |  |  | unknown | 61&#8209;63 |

### experiments/ArrayVal.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2985 | `array_lit` |  |  |  | Y | Y |  |  | unknown | 12&#8209;13 |
| 2986 | `array_mod` |  |  |  | Y | Y |  |  | unknown | 19&#8209;20 |
| 2987 | `mut_array_ref_in_array_ref_out` |  |  |  | Y | Y |  |  | unknown | 38&#8209;41 |
| 2988 | `main` |  |  |  | Y |  | Y | Y |  | 49 |

### experiments/ArrayVecSet.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2989 | `for_on_array_with_use` |  |  |  | Y | Y |  | Y |  | 11 |
| 2990 | `for_on_array_with_sum` |  |  |  | Y | Y |  |  | hole | 22 |
| 2991 | `while_on_array_with_sum` |  |  |  | Y | Y |  | Y |  | 40 |
| 2992 | `main` |  |  |  | Y |  | Y | Y |  | 57 |

### experiments/CheckedI32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2993 | `new` |  |  | Y |  | Y |  |  | unknown | 60&#8209;61 |
| 2994 | `new_out_of_range` |  |  | Y |  | Y |  |  | unknown | 67&#8209;69 |
| 2995 | `is_normal` |  |  | Y |  | Y |  |  | unknown | 80&#8209;81 |
| 2996 | `is_out_of_range` |  |  | Y |  | Y |  |  | unknown | 103&#8209;104 |
| 2997 | `unwrap` |  |  | Y |  | Y |  |  | unknown | 111&#8209;113 |
| 2998 | `to_option` |  |  | Y |  | Y |  |  | unknown | 120&#8209;123 |
| 2999 | `add_value` |  |  | Y |  | Y |  |  | hole | 132&#8209;133 |
| 3000 | `add_checked` |  |  | Y |  | Y |  |  | hole | 145&#8209;146 |
| 3001 | `sub_value` |  |  | Y |  | Y |  |  | hole | 158&#8209;159 |
| 3002 | `sub_checked` |  |  | Y |  | Y |  |  | hole | 171&#8209;172 |
| 3003 | `mul_value` |  |  | Y |  | Y |  |  | hole | 184&#8209;185 |
| 3004 | `mul_checked` |  |  | Y |  | Y |  |  | hole | 203&#8209;204 |

### experiments/ForFor.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3005 | `provable_array_for` |  |  |  | Y | Y |  |  | unknown | 11&#8209;12 |
| 3006 | `unprovable_array_for` |  |  |  | Y | Y |  |  | hole | 27 |
| 3007 | `provable_for_for` |  |  |  | Y | Y |  |  | unknown | 45&#8209;46 |
| 3008 | `main` |  |  |  | Y |  | Y | Y |  | 71 |

### experiments/ForLoops.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3009 | `loop_bounds` |  |  |  | Y | Y |  | Y |  | 11 |
| 3010 | `loop_on_array` |  |  |  | Y | Y |  | Y |  | 21 |
| 3011 | `main` |  |  |  | Y |  | Y | Y |  | 32 |

### experiments/HashCheckedU32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3012 | `hash` |  | Y |  |  | Y |  |  | hole | 18 |
| 3013 | `eq` |  | Y |  |  | Y |  |  | hole | 26 |
| 3014 | `axiom_CheckedU32_feq` |  |  |  | Y | Y |  |  | hole | 36&#8209;38 |
| 3015 | `axiom_CheckedU32_key_model` |  |  |  | Y | Y |  |  | hole | 41&#8209;43 |

### experiments/SetLoops.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3016 | `walkvec_while` |  |  |  | Y | Y |  | Y |  | 19 |

### experiments/ToVecProof.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3017 | `vec_to_vec` |  |  |  | Y | Y |  |  | unknown | 15&#8209;18 |

### experiments/VSTDLoopProofs.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3018 | `vec_length_while` |  |  |  | Y | Y |  |  | unknown | 13&#8209;14 |
| 3019 | `vec_length_while_asserted` |  |  |  | Y | Y |  |  | unknown | 32&#8209;33 |
| 3020 | `vec_length_loop_return` |  |  |  | Y | Y |  |  | unknown | 59&#8209;60 |
| 3021 | `vec_length_loop_break` |  |  |  | Y | Y |  |  | hole | 86&#8209;87 |
| 3022 | `vec_length_loop_return_asserted` |  |  |  | Y | Y |  |  | unknown | 108&#8209;109 |
| 3023 | `vec_length_for_range` |  |  |  | Y | Y |  |  | unknown | 139&#8209;140 |
| 3024 | `vec_length_for_range_asserted` |  |  |  | Y | Y |  |  | unknown | 154&#8209;155 |
| 3025 | `vec_mem_for_vec` |  |  |  | Y | Y |  |  | hole | 177&#8209;178 |
| 3026 | `vec_mem_for_range` |  |  |  | Y | Y |  |  | unknown | 190&#8209;191 |

### experiments/WhileWhile.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3027 | `provable_array_while` |  |  |  | Y | Y |  |  | unknown | 10&#8209;11 |
| 3028 | `unprovable_array_while` |  |  |  | Y | Y |  |  | hole | 28 |
| 3029 | `provable_while_while` |  |  |  | Y | Y |  |  | unknown | 48&#8209;49 |
| 3030 | `main` |  |  |  | Y |  | Y | Y |  | 77 |

### experiments/abstract_set_iter.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3031 | `len` | Y | Y |  |  | Y |  | Y |  | 49 |
| 3032 | `new` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 3033 | `mem` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;56 |
| 3034 | `insert` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;65 |
| 3035 | `iter` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;83 |
| 3036 | `next` |  |  | Y |  | Y |  |  | unknown | 217&#8209;249 |
| 3037 | `abstract_set_copy_loop` |  |  |  | Y | Y |  |  | unknown | 284&#8209;288 |

### experiments/accept.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3038 | `accept` |  |  |  | Y | Y |  |  | hole | 26&#8209;27 |
| 3039 | `accept_propagates_like_assume` |  |  |  | Y | Y |  | Y |  | 32 |
| 3040 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 65&#8209;66 |

### experiments/accept_external_body.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3041 | `clone_ensures_propagates` |  |  |  | Y | Y |  |  | unknown | 56&#8209;57 |

### experiments/arc_clone_deref.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3042 | `test_arc_clone_u8` |  |  |  | Y | Y |  | Y |  | 14 |
| 3043 | `test_arc_clone_generic` |  |  |  | Y | Y |  | Y |  | 20 |

### experiments/arc_rwlock_ninject.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3044 | `apply_updates` |  |  |  | Y | Y |  |  | unknown | 44&#8209;51 |
| 3045 | `ninject_par` |  |  |  | Y | Y |  |  | unknown | 105&#8209;114 |

### experiments/assume_spec_test.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3046 | `next` |  | Y |  |  | Y |  | Y |  | 52 |
| 3047 | `test` |  |  |  | Y | Y |  | Y |  | 63 |

### experiments/baseviewtypes.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3048 | `test_u64_view` |  |  |  | Y | Y |  | Y |  | 16 |
| 3049 | `test_i64_view` |  |  |  | Y | Y |  | Y |  | 23 |
| 3050 | `test_bool_view` |  |  |  | Y | Y |  | Y |  | 29 |
| 3051 | `test_unit_view` |  |  |  | Y | Y |  | Y |  | 36 |
| 3052 | `test_explicit_conversion` |  |  |  | Y | Y |  | Y |  | 43 |
| 3053 | `test_vec_u64_view` |  |  |  | Y | Y |  |  | unknown | 55&#8209;56 |
| 3054 | `test_cross_type_comparison` |  |  |  | Y | Y |  | Y |  | 70 |

### experiments/biconditional_spec_fun.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3055 | `is_even` |  |  |  | Y | Y |  |  | unknown | 23&#8209;24 |
| 3056 | `test_forward_direction` |  |  |  | Y | Y |  | Y |  | 31 |
| 3057 | `test_backward_direction` |  |  |  | Y | Y |  | Y |  | 46 |
| 3058 | `test_biconditional` |  |  |  | Y | Y |  | Y |  | 61 |
| 3059 | `test_identity_biconditional` |  |  |  | Y | Y |  | Y |  | 76 |
| 3060 | `test_biconditional_inline` |  |  |  | Y | Y |  | Y |  | 90 |
| 3061 | `test_biconditional_with_by` |  |  |  | Y | Y |  | Y |  | 103 |
| 3062 | `test_forward_with_by` |  |  |  | Y | Y |  | Y |  | 118 |
| 3063 | `test_specific_call` |  |  |  | Y | Y |  | Y |  | 133 |
| 3064 | `test_call_ensures` |  |  |  | Y | Y |  | Y |  | 152 |

### experiments/boxing_fns.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3065 | `apply_impl_fn` |  |  |  | Y | Y |  |  | unknown | 14&#8209;18 |
| 3066 | `double` |  |  |  | Y | Y |  |  | unknown | 23&#8209;25 |
| 3067 | `test_impl_fn` |  |  |  | Y | Y |  | Y |  | 30 |
| 3068 | `test_impl_fn_closure` |  |  |  | Y | Y |  | Y |  | 36 |
| 3069 | `triple` |  |  |  | Y | Y |  |  | unknown | 49&#8209;51 |
| 3070 | `test_fn_item_as_impl_fn` |  |  |  | Y | Y |  | Y |  | 56 |
| 3071 | `test_spec_fn_direct` |  |  |  | Y | Y |  | Y |  | 66 |
| 3072 | `test_spec_fn_lambda` |  |  |  | Y | Y |  | Y |  | 71 |
| 3073 | `apply_fn_once` |  |  |  | Y | Y |  |  | unknown | 77&#8209;81 |
| 3074 | `test_fn_once` |  |  |  | Y | Y |  | Y |  | 86 |
| 3075 | `apply_fn_mut` |  |  |  | Y | Y |  |  | unknown | 98&#8209;102 |
| 3076 | `test_fn_mut` |  |  |  | Y | Y |  | Y |  | 107 |

### experiments/checked_signed_int.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3077 | `is_normal` | Y |  |  |  | Y |  |  | unknown | 18&#8209;19 |
| 3078 | `is_overflow` | Y |  |  |  | Y |  |  | unknown | 21&#8209;22 |
| 3079 | `is_underflow` | Y |  |  |  | Y |  |  | unknown | 24&#8209;25 |
| 3080 | `from_value` | Y |  |  |  | Y |  |  | unknown | 27&#8209;29 |
| 3081 | `from_int` | Y |  |  |  | Y |  |  | unknown | 31&#8209;32 |
| 3082 | `to_value` | Y |  |  |  | Y |  |  | unknown | 34&#8209;36 |
| 3083 | `add_checked` | Y |  |  |  | Y |  |  | unknown | 38&#8209;39 |
| 3084 | `sub_checked` | Y |  |  |  | Y |  |  | unknown | 41&#8209;42 |
| 3085 | `mul_checked` | Y |  |  |  | Y |  |  | unknown | 44&#8209;45 |
| 3086 | `clone_checked` | Y |  |  |  | Y |  |  | unknown | 47&#8209;48 |
| 3087 | `lemma_add_commutative_ghost` | Y |  |  |  | Y |  |  | unknown | 50&#8209;51 |
| 3088 | `lemma_mul_commutative_ghost` | Y |  |  |  | Y |  |  | unknown | 54&#8209;55 |
| 3089 | `lemma_add_associative_ghost` | Y |  |  |  | Y |  |  | unknown | 58&#8209;59 |
| 3090 | `lemma_mul_associative_ghost` | Y |  |  |  | Y |  |  | unknown | 62&#8209;63 |
| 3091 | `lemma_mul_distributes_over_add_ghost` | Y |  |  |  | Y |  |  | unknown | 66&#8209;67 |
| 3092 | `lemma_sub_anti_commutative_ghost` | Y |  |  |  | Y |  |  | unknown | 70&#8209;71 |

### experiments/checked_u32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3093 | `new` |  |  | Y |  | Y |  |  | unknown | 41&#8209;42 |
| 3094 | `new_overflow` |  |  | Y |  | Y |  |  | unknown | 47&#8209;49 |
| 3095 | `unwrap` |  |  | Y |  | Y |  |  | unknown | 54&#8209;56 |
| 3096 | `is_normal` |  | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 3097 | `is_overflow` |  | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 3098 | `from_value` |  | Y |  |  | Y |  | Y |  | 80 |
| 3099 | `from_int` |  | Y |  |  | Y |  |  | hole | 86&#8209;87 |
| 3100 | `to_value` |  | Y |  |  | Y |  | Y |  | 93 |
| 3101 | `add_checked` |  | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 3102 | `mul_checked` |  | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 3103 | `clone_checked` |  | Y |  |  | Y |  |  | unknown | 123&#8209;124 |

### experiments/checked_unsigned_int.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3104 | `is_normal` | Y |  |  |  | Y |  |  | unknown | 17&#8209;18 |
| 3105 | `is_overflow` | Y |  |  |  | Y |  |  | unknown | 20&#8209;21 |
| 3106 | `from_value` | Y |  |  |  | Y |  |  | unknown | 23&#8209;25 |
| 3107 | `from_int` | Y |  |  |  | Y |  |  | unknown | 27&#8209;28 |
| 3108 | `to_value` | Y |  |  |  | Y |  |  | unknown | 30&#8209;32 |
| 3109 | `add_checked` | Y |  |  |  | Y |  |  | unknown | 34&#8209;35 |
| 3110 | `mul_checked` | Y |  |  |  | Y |  |  | unknown | 37&#8209;38 |
| 3111 | `clone_checked` | Y |  |  |  | Y |  |  | unknown | 40&#8209;41 |
| 3112 | `lemma_add_commutative_ghost` | Y |  |  |  | Y |  |  | unknown | 43&#8209;44 |
| 3113 | `lemma_mul_commutative_ghost` | Y |  |  |  | Y |  |  | unknown | 47&#8209;48 |
| 3114 | `lemma_add_associative_ghost` | Y |  |  |  | Y |  |  | unknown | 51&#8209;52 |
| 3115 | `lemma_mul_associative_ghost` | Y |  |  |  | Y |  |  | unknown | 55&#8209;56 |
| 3116 | `lemma_mul_distributes_over_add_ghost` | Y |  |  |  | Y |  |  | unknown | 59&#8209;60 |
| 3117 | `lemma_add_monotonic` | Y |  |  |  | Y |  |  | unknown | 63&#8209;65 |
| 3118 | `lemma_partial_sums_fit` | Y |  |  |  | Y |  |  | unknown | 68&#8209;70 |

### experiments/clone.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3119 | `axiom_cloned_view_eq` |  |  |  | Y | Y |  |  | unknown | 23&#8209;26 |
| 3120 | `test_clone_no_assert_return_eq` |  |  |  | Y | Y |  |  | hole | 36&#8209;39 |
| 3121 | `test_clone_no_assert_return_eq_view` |  |  |  | Y | Y |  |  | hole | 47&#8209;50 |
| 3122 | `test_clone_assert_return` |  |  |  | Y | Y |  |  | hole | 59&#8209;61 |
| 3123 | `test_clone_with_assert` |  |  |  | Y | Y |  |  | hole | 78&#8209;79 |
| 3124 | `test_1_cloned_automatic` |  |  |  | Y | Y |  | Y |  | 87&#8209;88 |
| 3125 | `test_2_concrete_eq` |  |  |  | Y | Y |  |  | hole | 97&#8209;98 |
| 3126 | `test_3_extensional_equality` |  |  |  | Y | Y |  |  | hole | 110 |
| 3127 | `test_clone_no_assert_return_eq_u64` |  |  |  | Y | Y |  |  | unknown | 121&#8209;124 |
| 3128 | `test_clone_no_assert_return_eq_view_u64` |  |  |  | Y | Y |  |  | unknown | 130&#8209;133 |
| 3129 | `test_clone_assert_return_u64` |  |  |  | Y | Y |  |  | unknown | 139&#8209;141 |

### experiments/clone_fn.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3130 | `clone_fn1` |  |  |  | Y | Y |  |  | hole | 9&#8209;13 |
| 3131 | `clone_fn2` |  |  |  | Y | Y |  |  | hole | 21&#8209;26 |
| 3132 | `clone_fn_binary` |  |  |  | Y | Y |  |  | hole | 35&#8209;40 |
| 3133 | `clone_predicate` |  |  |  | Y | Y |  |  | hole | 49&#8209;54 |
| 3134 | `clone_fn_axiom` |  |  |  | Y | Y |  |  | hole | 64&#8209;67 |

### experiments/clone_plus.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3135 | `test_clone_gives_cloned` |  |  |  | Y | Y |  | Y |  | 12 |
| 3136 | `test_clone_view_no_feq` |  |  |  | Y | Y |  |  | hole | 19 |
| 3137 | `test_clone_view_with_feq` |  |  |  | Y | Y |  |  | unknown | 26&#8209;27 |
| 3138 | `test_clone_eq_with_feq` |  |  |  | Y | Y |  |  | unknown | 34&#8209;36 |

### experiments/clone_plus_vs_deep_clone.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3139 | `test_clone_u64` |  |  |  | Y | Y |  | Y |  | 58 |
| 3140 | `test_vstd_equality` |  |  |  | Y | Y |  |  | unknown | 69&#8209;70 |
| 3141 | `test_element_deep_clone` |  |  |  | Y | Y |  | Y |  | 87 |

### experiments/collect.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3142 | `vec_mem` |  |  |  | Y | Y |  |  | unknown | 26&#8209;31 |
| 3143 | `collect_keys_rec` |  |  |  | Y | Y |  |  | unknown | 171&#8209;179 |
| 3144 | `collect_keys_deduplicating` |  |  |  | Y | Y |  |  | unknown | 218&#8209;226 |
| 3145 | `collect_values_rec` |  |  |  | Y | Y |  |  | unknown | 267&#8209;280 |
| 3146 | `collect_values_deduplicating` |  |  |  | Y | Y |  |  | unknown | 339&#8209;352 |
| 3147 | `collect_rec` |  |  |  | Y | Y |  |  | unknown | 411&#8209;423 |
| 3148 | `collect_deduplicating` |  |  |  | Y | Y |  |  | unknown | 429&#8209;441 |

### experiments/collect2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3149 | `lemma_deep_view_key` |  |  |  | Y | Y |  |  | unknown | 61&#8209;66 |
| 3150 | `lemma_find_key_some` |  |  |  | Y | Y |  |  | unknown | 71&#8209;78 |
| 3151 | `lemma_find_key_none` |  |  |  | Y | Y |  |  | unknown | 90&#8209;95 |
| 3152 | `vec_find_key` |  |  |  | Y | Y |  |  | unknown | 105&#8209;117 |
| 3153 | `collect2` |  |  |  | Y | Y |  |  | unknown | 140&#8209;148 |

### experiments/collect_deep_view.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3154 | `lemma_find_key_index_bounds` |  |  |  | Y | Y |  |  | unknown | 53&#8209;58 |
| 3155 | `lemma_spec_collect_step_some` |  |  |  | Y | Y |  |  | unknown | 67&#8209;79 |
| 3156 | `lemma_spec_collect_step_none` |  |  |  | Y | Y |  |  | unknown | 90&#8209;101 |
| 3157 | `lemma_deep_view_len` |  |  |  | Y | Y |  |  | unknown | 111&#8209;113 |
| 3158 | `lemma_deep_view_key` |  |  |  | Y | Y |  |  | unknown | 118&#8209;123 |
| 3159 | `lemma_find_key_index_found` |  |  |  | Y | Y |  |  | unknown | 129&#8209;140 |
| 3160 | `lemma_find_key_index_not_found` |  |  |  | Y | Y |  |  | unknown | 149&#8209;157 |
| 3161 | `lemma_find_key_some` |  |  |  | Y | Y |  |  | unknown | 166&#8209;173 |
| 3162 | `lemma_find_key_none` |  |  |  | Y | Y |  |  | unknown | 187&#8209;192 |
| 3163 | `vec_find_key` |  |  |  | Y | Y |  |  | unknown | 202&#8209;214 |
| 3164 | `collect` |  |  |  | Y | Y |  |  | hole | 240&#8209;250 |

### experiments/deep_view_2_tuple.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3165 | `test_1_tuple_deep_view` |  |  |  | Y | Y |  |  | unknown | 12&#8209;14 |
| 3166 | `test_2_tuple_deep_view` |  |  |  | Y | Y |  |  | unknown | 18&#8209;20 |

### experiments/deep_view_struct.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3167 | `get_val` | Y | Y |  |  | Y |  |  | unknown | 34&#8209;35 |
| 3168 | `test_deep_view` |  |  |  | Y | Y |  | Y |  | 48 |

### experiments/eq_rel.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3169 | `eq_reflexive` | Y | Y |  |  | Y |  |  | unknown | 15 |
| 3170 | `eq_symmetric` | Y | Y |  |  | Y |  |  | unknown | 16 |
| 3171 | `eq_transitive` | Y | Y |  |  | Y |  |  | unknown | 17&#8209;18 |
| 3172 | `eq_view` | Y | Y |  |  | Y |  |  | unknown | 22&#8209;24 |
| 3173 | `axiom_cloned_implies_eq` | Y | Y |  |  | Y |  |  | hole | 30&#8209;32 |
| 3174 | `clone_view_eq` | Y | Y |  |  | Y |  |  | unknown | 35&#8209;38 |
| 3175 | `test_eq_rel` x2 | Y | Y |  |  | Y |  | Y |  | 58 |
| 3176 | `test_eq_rel_view` x2 | Y | Y |  |  | Y |  | Y |  | 59 |
| 3177 | `test_eq_rel_clone` x2 | Y | Y |  |  | Y |  | Y |  | 60 |

### experiments/executable_use_of_int.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3178 | `vec_int_succeeds` |  |  |  | Y | Y |  |  | hole | 20&#8209;21 |
| 3179 | `vec_int_mem_verifies` |  |  |  | Y | Y |  | Y |  | 31 |

### experiments/f64_bits_sort.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3180 | `axiom_f64_le_total_ordering` |  |  |  | Y | Y |  |  | hole | 61&#8209;63 |
| 3181 | `lemma_f64_le_transitive` |  |  |  | Y | Y |  |  | unknown | 79&#8209;84 |
| 3182 | `f64_le` |  |  |  | Y | Y |  |  | hole | 89&#8209;94 |
| 3183 | `insertion_sort_f64` |  |  |  | Y | Y |  |  | unknown | 100&#8209;106 |
| 3184 | `test_sort` |  |  |  | Y | Y |  |  | hole | 194 |

### experiments/f64_float_cmp_sort.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3185 | `insertion_sort_f64` |  |  |  | Y | Y |  |  | hole | 42&#8209;47 |
| 3186 | `test_comparison_bridge` |  |  |  | Y | Y |  |  | hole | 89 |
| 3187 | `test_strong_connectivity` |  |  |  | Y | Y |  |  | hole | 104 |
| 3188 | `test_sort` |  |  |  | Y | Y |  |  | hole | 120 |

### experiments/f64_sort.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3189 | `axiom_f64_le_total_ordering` |  |  |  | Y | Y |  |  | hole | 50&#8209;52 |
| 3190 | `f64_le` |  |  |  | Y | Y |  |  | hole | 73&#8209;78 |
| 3191 | `insertion_sort_f64` |  |  |  | Y | Y |  |  | unknown | 85&#8209;92 |
| 3192 | `test_sort` |  |  |  | Y | Y |  |  | hole | 181 |

### experiments/ghost_type_invariant.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3193 | `test_basic` |  |  |  | Y | Y |  | Y |  | 80 |

### experiments/hash_set_iter.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3194 | `test_hash_set_iter` |  |  |  | Y | Y |  | Y |  | 11 |

### experiments/hash_set_modern_pattern.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3195 | `test_hashset_new` |  |  |  | Y | Y |  | Y |  | 59 |
| 3196 | `test_hashset_insert` |  |  |  | Y | Y |  | Y |  | 68 |
| 3197 | `test_hashset_contains` |  |  |  | Y | Y |  | Y |  | 86 |
| 3198 | `test_hashset_remove` |  |  |  | Y | Y |  | Y |  | 102 |
| 3199 | `test_hashset_with_view_comparison` |  |  |  | Y | Y |  |  | hole | 119 |

### experiments/hash_set_with_view_plus_loops.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3200 | `vec_to_vec_copy_fails` |  |  |  | Y | Y |  |  | hole | 24&#8209;25 |
| 3201 | `vec_to_vec_copy` |  |  |  | Y | Y |  |  | unknown | 38&#8209;39 |
| 3202 | `vec_to_vec_copy_stepped` |  |  |  | Y | Y |  | Y |  | 57 |
| 3203 | `vec_add_one` |  |  |  | Y | Y |  |  | unknown | 161&#8209;163 |
| 3204 | `hash_set_copy` |  |  |  | Y | Y |  |  | unknown | 182&#8209;183 |
| 3205 | `hash_set_add_one` |  |  |  | Y | Y |  |  | unknown | 201&#8209;203 |
| 3206 | `hash_set_copy_stepped` |  |  |  | Y | Y |  | Y |  | 224 |

### experiments/invariant_proof_test.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3207 | `new_iter` |  |  |  | Y | Y |  |  | unknown | 29&#8209;32 |
| 3208 | `next_with_invariant` |  |  |  | Y | Y |  |  | unknown | 38&#8209;59 |
| 3209 | `lemma_iter_lengths` |  |  |  | Y | Y |  |  | unknown | 77&#8209;82 |
| 3210 | `test_copy` |  |  |  | Y | Y |  |  | unknown | 88&#8209;89 |

### experiments/minimal_iter.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3211 | `next` | Y |  |  |  | Y |  | Y |  | 8 |
| 3212 | `iter` | Y |  |  |  | Y |  | Y |  | 14 |

### experiments/modify_a_ghost_struct.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3213 | `test_create_ghost` |  |  |  | Y | Y |  | Y |  | 26 |
| 3214 | `test_functional_update` |  |  |  | Y | Y |  | Y |  | 35 |
| 3215 | `test_mutable_ghost_var` |  |  |  | Y | Y |  | Y |  | 46 |
| 3216 | `test_direct_field_modify` |  |  |  | Y | Y |  | Y |  | 62 |
| 3217 | `test_ghost_in_exec` |  |  |  | Y | Y |  | Y |  | 73 |
| 3218 | `test_modify_ghost_in_exec` |  |  |  | Y | Y |  | Y |  | 84 |

### experiments/mut_refs_and_mut_returns.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3219 | `new` |  |  | Y |  | Y |  |  | unknown | 29&#8209;30 |
| 3220 | `inc` |  |  | Y |  | Y |  |  | unknown | 36&#8209;40 |
| 3221 | `add` |  |  | Y |  | Y |  |  | unknown | 46&#8209;50 |
| 3222 | `proof_inc_spec` |  |  |  | Y | Y |  | Y |  | 76 |
| 3223 | `proof_add_spec` |  |  |  | Y | Y |  | Y |  | 87 |

### experiments/parapair_closure_ensures.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3224 | `return_42` |  |  |  | Y | Y |  |  | unknown | 10&#8209;11 |
| 3225 | `return_99` |  |  |  | Y | Y |  |  | unknown | 16&#8209;17 |
| 3226 | `test_parapair_closure_ensures` |  |  |  | Y | Y |  | Y |  | 23 |

### experiments/parapair_move_closure_ensures.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3227 | `return_42` |  |  |  | Y | Y |  |  | unknown | 11&#8209;12 |
| 3228 | `return_99` |  |  |  | Y | Y |  |  | unknown | 18&#8209;19 |
| 3229 | `test_parapair_literal_return` |  |  |  | Y | Y |  | Y |  | 25 |

### experiments/parapair_named_closure.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3230 | `test_parapair_named_closure` |  |  |  | Y | Y |  | Y |  | 11 |

### experiments/parapair_toplevel_closure.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3231 | `f42` |  |  |  | Y | Y |  |  | unknown | 10&#8209;11 |
| 3232 | `f99` |  |  |  | Y | Y |  |  | unknown | 14&#8209;15 |
| 3233 | `test_parapair_toplevel_closure` |  |  |  | Y | Y |  | Y |  | 18 |

### experiments/pervasives.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3234 | `test_unreached_in_match` |  |  |  | Y | Y |  | Y |  | 15 |
| 3235 | `test_unreached_with_proof` |  |  |  | Y | Y |  |  | unknown | 29&#8209;30 |
| 3236 | `assert_unchecked` |  |  |  | Y | Y |  |  | hole | 44&#8209;45 |
| 3237 | `test_assert_unchecked_proves_false` |  |  |  | Y | Y |  |  | unknown | 51&#8209;52 |
| 3238 | `claim_anything` |  |  |  | Y | Y |  |  | hole | 60&#8209;61 |
| 3239 | `test_claim_anything_proves_false` |  |  |  | Y | Y |  |  | unknown | 65&#8209;66 |
| 3240 | `example_with_spec_comments` |  |  |  | Y | Y |  | Y |  | 79 |
| 3241 | `using_assume_proves_false` |  |  |  | Y | Y |  |  | hole | 86&#8209;87 |

### experiments/possession.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3242 | `new_exorcised` | Y | Y |  |  | Y |  |  | unknown | 12 |
| 3243 | `new_possessed` | Y | Y |  |  | Y |  |  | unknown | 13 |

### experiments/proof_fn_in_trait.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3244 | `simple_proof_method` | Y | Y |  |  | Y |  |  | unknown | 14&#8209;15 |
| 3245 | `lemma_property_holds` | Y | Y |  |  | Y |  |  | unknown | 31&#8209;33 |
| 3246 | `test_call_proof_fn` |  |  |  | Y | Y |  | Y |  | 50 |

### experiments/proven_partialeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3247 | `eq` x2 | Y | Y |  |  | Y |  |  | unknown | 20&#8209;21 |
| 3248 | `ne` x2 | Y | Y |  |  | Y |  |  | unknown | 23&#8209;24 |
| 3249 | `proof_reflexivity` x2 | Y | Y |  |  | Y |  |  | unknown | 26&#8209;27 |
| 3250 | `proof_symmetry` x2 | Y | Y |  |  | Y |  |  | unknown | 29&#8209;31 |
| 3251 | `proof_transitivity` x2 | Y | Y |  |  | Y |  |  | unknown | 33&#8209;35 |
| 3252 | `test_use_i32` |  |  |  | Y | Y |  |  | unknown | 64&#8209;65 |
| 3253 | `test_use_myint` |  |  |  | Y | Y |  |  | unknown | 103&#8209;104 |

### experiments/pub_crate_test.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3254 | `_new_valid` x2 |  |  | Y |  |  | Y | Y |  | 12&#8209;14 |
| 3255 | `test_pub_crate` |  |  |  | Y |  | Y | Y |  | 33&#8209;37 |
| 3256 | `test_pub_super` |  |  |  | Y |  | Y | Y |  | 39&#8209;43 |
| 3257 | `main` |  |  |  | Y |  | Y | Y |  | 46&#8209;50 |

### experiments/seq_array_equality.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3258 | `array_copy_equality` |  |  |  | Y | Y |  |  | unknown | 12&#8209;15 |
| 3259 | `array_clone_equality` |  |  |  | Y | Y |  |  | unknown | 20&#8209;23 |

### experiments/seq_for_basic_proofs.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3260 | `fixed_length_i64_array` |  |  |  | Y | Y |  |  | unknown | 13&#8209;14 |
| 3261 | `length_i64_array` |  |  |  | Y | Y |  |  | unknown | 17&#8209;18 |
| 3262 | `length_i64_vec` |  |  |  | Y | Y |  |  | unknown | 21&#8209;22 |
| 3263 | `fixed_length_usize_array` |  |  |  | Y | Y |  |  | unknown | 28&#8209;29 |
| 3264 | `length_usize_array` |  |  |  | Y | Y |  |  | unknown | 32&#8209;33 |
| 3265 | `length_usize_vec` |  |  |  | Y | Y |  |  | unknown | 36&#8209;37 |
| 3266 | `fixed_length_t_array` |  |  |  | Y | Y |  |  | unknown | 44&#8209;45 |
| 3267 | `length_t_array` |  |  |  | Y | Y |  |  | unknown | 48&#8209;49 |
| 3268 | `length_t_vec` |  |  |  | Y | Y |  |  | unknown | 52&#8209;53 |
| 3269 | `i64_array_mem_for` |  |  |  | Y | Y |  |  | unknown | 65&#8209;66 |
| 3270 | `i64_vec_mem_for` |  |  |  | Y | Y |  |  | unknown | 79&#8209;80 |
| 3271 | `usize_array_mem_for` |  |  |  | Y | Y |  |  | unknown | 98&#8209;99 |
| 3272 | `usize_vec_mem_for` |  |  |  | Y | Y |  |  | unknown | 112&#8209;113 |
| 3273 | `i64_array_find_for` |  |  |  | Y | Y |  |  | unknown | 171&#8209;176 |
| 3274 | `i64_vec_find_for` |  |  |  | Y | Y |  |  | unknown | 189&#8209;194 |
| 3275 | `usize_array_find_for` |  |  |  | Y | Y |  |  | unknown | 216&#8209;221 |
| 3276 | `usize_vec_find_for` |  |  |  | Y | Y |  |  | unknown | 234&#8209;239 |
| 3277 | `i64_array_sum_non_negative_up_for` |  |  |  | Y | Y |  |  | unknown | 284&#8209;285 |
| 3278 | `i64_vec_sum_non_negative_up_for` |  |  |  | Y | Y |  |  | unknown | 303&#8209;304 |
| 3279 | `i64_array_sum_non_negative_down_for` |  |  |  | Y | Y |  |  | hole | 350&#8209;351 |
| 3280 | `i64_vec_sum_non_negative_down_for` |  |  |  | Y | Y |  |  | hole | 384&#8209;385 |
| 3281 | `array_length_up_for` |  |  |  | Y | Y |  |  | unknown | 419&#8209;420 |
| 3282 | `vec_length_up_for` |  |  |  | Y | Y |  |  | unknown | 432&#8209;433 |
| 3283 | `i64_array_count_up_for` |  |  |  | Y | Y |  |  | unknown | 467&#8209;468 |
| 3284 | `i64_vec_count_up_for` |  |  |  | Y | Y |  |  | unknown | 482&#8209;483 |
| 3285 | `seq_int_sum_equivalence` |  |  |  | Y | Y |  |  | unknown | 497&#8209;499 |
| 3286 | `seq_i64_sum_equivalence` |  |  |  | Y | Y |  |  | unknown | 516&#8209;518 |

### experiments/seq_loop_basic_proofs.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3287 | `fixed_length_i64_array` |  |  |  | Y | Y |  |  | unknown | 13&#8209;14 |
| 3288 | `length_i64_array` |  |  |  | Y | Y |  |  | unknown | 17&#8209;18 |
| 3289 | `length_i64_vec` |  |  |  | Y | Y |  |  | unknown | 21&#8209;22 |
| 3290 | `fixed_length_usize_array` |  |  |  | Y | Y |  |  | unknown | 28&#8209;29 |
| 3291 | `length_usize_array` |  |  |  | Y | Y |  |  | unknown | 32&#8209;33 |
| 3292 | `length_usize_vec` |  |  |  | Y | Y |  |  | unknown | 36&#8209;37 |
| 3293 | `fixed_length_t_array` |  |  |  | Y | Y |  |  | unknown | 44&#8209;45 |
| 3294 | `length_t_array` |  |  |  | Y | Y |  |  | unknown | 48&#8209;49 |
| 3295 | `length_t_vec` |  |  |  | Y | Y |  |  | unknown | 52&#8209;53 |
| 3296 | `i64_array_mem_loop` |  |  |  | Y | Y |  |  | unknown | 65&#8209;66 |
| 3297 | `i64_vec_mem_loop` |  |  |  | Y | Y |  |  | unknown | 85&#8209;86 |
| 3298 | `usize_array_mem_loop` |  |  |  | Y | Y |  |  | unknown | 110&#8209;111 |
| 3299 | `usize_vec_mem_loop` |  |  |  | Y | Y |  |  | unknown | 130&#8209;131 |
| 3300 | `i64_array_find_loop` |  |  |  | Y | Y |  |  | unknown | 195&#8209;200 |
| 3301 | `i64_vec_find_loop` |  |  |  | Y | Y |  |  | unknown | 219&#8209;224 |
| 3302 | `usize_array_find_loop` |  |  |  | Y | Y |  |  | unknown | 252&#8209;257 |
| 3303 | `usize_vec_find_loop` |  |  |  | Y | Y |  |  | unknown | 276&#8209;281 |
| 3304 | `i64_array_sum_non_negative_up_loop` |  |  |  | Y | Y |  |  | unknown | 332&#8209;333 |
| 3305 | `i64_vec_sum_non_negative_up_loop` |  |  |  | Y | Y |  |  | unknown | 357&#8209;358 |
| 3306 | `i64_array_sum_non_negative_down_loop` |  |  |  | Y | Y |  |  | hole | 410&#8209;411 |
| 3307 | `i64_vec_sum_non_negative_down_loop` |  |  |  | Y | Y |  |  | hole | 444&#8209;445 |
| 3308 | `array_length_up_loop` |  |  |  | Y | Y |  |  | unknown | 479&#8209;480 |
| 3309 | `vec_length_up_loop` |  |  |  | Y | Y |  |  | unknown | 498&#8209;499 |
| 3310 | `i64_array_count_up_loop` |  |  |  | Y | Y |  |  | unknown | 539&#8209;540 |
| 3311 | `i64_vec_count_up_loop` |  |  |  | Y | Y |  |  | unknown | 560&#8209;561 |
| 3312 | `seq_int_sum_equivalence` |  |  |  | Y | Y |  |  | unknown | 581&#8209;583 |
| 3313 | `seq_i64_sum_equivalence` |  |  |  | Y | Y |  |  | unknown | 600&#8209;602 |

### experiments/seq_set_exec.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3314 | `lemma_nat_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 33&#8209;37 |
| 3315 | `proof_sum_seq_nat` |  |  |  | Y | Y |  |  | unknown | 49&#8209;54 |
| 3316 | `proof_sum_seq_nat_full` |  |  |  | Y | Y |  |  | unknown | 67&#8209;69 |
| 3317 | `sum_seq_u32_no_overflow` |  |  |  | Y | Y |  |  | hole | 75&#8209;77 |
| 3318 | `sum_seq_CheckedU32` |  |  |  | Y | Y |  |  | unknown | 87&#8209;89 |
| 3319 | `sum_set_CheckedU32` |  |  |  | Y | Y |  |  | hole | 113&#8209;118 |

### experiments/seq_vec_equality.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3320 | `vec_copy_generic_equality` |  |  |  | Y | Y |  |  | hole | 8&#8209;11 |
| 3321 | `vec_clone_usize_equality` |  |  |  | Y | Y |  |  | unknown | 16&#8209;19 |

### experiments/seq_while_basic_proofs.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3322 | `fixed_length_i64_array` |  |  |  | Y | Y |  |  | unknown | 13&#8209;14 |
| 3323 | `length_i64_array` |  |  |  | Y | Y |  |  | unknown | 17&#8209;18 |
| 3324 | `length_i64_vec` |  |  |  | Y | Y |  |  | unknown | 21&#8209;22 |
| 3325 | `fixed_length_usize_array` |  |  |  | Y | Y |  |  | unknown | 28&#8209;29 |
| 3326 | `length_usize_array` |  |  |  | Y | Y |  |  | unknown | 32&#8209;33 |
| 3327 | `length_usize_vec` |  |  |  | Y | Y |  |  | unknown | 36&#8209;37 |
| 3328 | `fixed_length_t_array` |  |  |  | Y | Y |  |  | unknown | 44&#8209;45 |
| 3329 | `length_t_array` |  |  |  | Y | Y |  |  | unknown | 48&#8209;49 |
| 3330 | `length_t_vec` |  |  |  | Y | Y |  |  | unknown | 52&#8209;53 |
| 3331 | `i64_array_mem_while` |  |  |  | Y | Y |  |  | unknown | 65&#8209;66 |
| 3332 | `i64_vec_mem_while` |  |  |  | Y | Y |  |  | unknown | 83&#8209;84 |
| 3333 | `usize_array_mem_while` |  |  |  | Y | Y |  |  | unknown | 106&#8209;107 |
| 3334 | `usize_vec_mem_while` |  |  |  | Y | Y |  |  | unknown | 124&#8209;125 |
| 3335 | `i64_array_find_while` |  |  |  | Y | Y |  |  | unknown | 192&#8209;197 |
| 3336 | `i64_vec_find_while` |  |  |  | Y | Y |  |  | unknown | 214&#8209;219 |
| 3337 | `usize_array_find_while` |  |  |  | Y | Y |  |  | unknown | 245&#8209;250 |
| 3338 | `usize_vec_find_while` |  |  |  | Y | Y |  |  | unknown | 267&#8209;272 |
| 3339 | `i64_array_sum_non_negative_up_while` |  |  |  | Y | Y |  |  | unknown | 321&#8209;322 |
| 3340 | `i64_vec_sum_non_negative_up_while` |  |  |  | Y | Y |  |  | unknown | 344&#8209;345 |
| 3341 | `i64_array_sum_non_negative_down_while` |  |  |  | Y | Y |  |  | hole | 395&#8209;396 |
| 3342 | `i64_vec_sum_non_negative_down_while` |  |  |  | Y | Y |  |  | hole | 427&#8209;428 |
| 3343 | `array_length_up_while` |  |  |  | Y | Y |  |  | unknown | 460&#8209;461 |
| 3344 | `vec_length_up_while` |  |  |  | Y | Y |  |  | unknown | 477&#8209;478 |
| 3345 | `i64_array_count_up_while` |  |  |  | Y | Y |  |  | unknown | 516&#8209;517 |
| 3346 | `i64_vec_count_up_while` |  |  |  | Y | Y |  |  | unknown | 535&#8209;536 |
| 3347 | `seq_int_sum_equivalence` |  |  |  | Y | Y |  |  | unknown | 554&#8209;556 |
| 3348 | `seq_i64_sum_equivalence` |  |  |  | Y | Y |  |  | unknown | 573&#8209;575 |

### experiments/set_len_empty_both_ways.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3349 | `lemma_len0_implies_empty` |  |  |  | Y | Y |  |  | unknown | 18&#8209;20 |
| 3350 | `lemma_len_pos_implies_not_empty` |  |  |  | Y | Y |  |  | unknown | 26&#8209;28 |
| 3351 | `lemma_set_len_empty_both_ways` |  |  |  | Y | Y |  |  | unknown | 34&#8209;38 |
| 3352 | `test_both_directions` |  |  |  | Y | Y |  | Y |  | 44 |

### experiments/sigma_pi.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3353 | `add_commutative_at` | Y | Y |  |  | Y |  |  | unknown | 28&#8209;33 |
| 3354 | `lemma_add_commutative` | Y | Y |  |  | Y |  |  | hole | 35&#8209;36 |
| 3355 | `mul_commutative_at` | Y | Y |  |  | Y |  |  | hole | 38&#8209;44 |
| 3356 | `lemma_mul_commutative` | Y | Y |  |  | Y |  |  | hole | 46&#8209;47 |

### experiments/signed_int.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3357 | `lemma_add_commutative` | Y |  |  |  | Y |  |  | unknown | 20&#8209;21 |
| 3358 | `lemma_mul_commutative` | Y |  |  |  | Y |  |  | unknown | 24&#8209;25 |
| 3359 | `lemma_add_associative` | Y |  |  |  | Y |  |  | unknown | 28&#8209;29 |
| 3360 | `lemma_mul_associative` | Y |  |  |  | Y |  |  | unknown | 32&#8209;33 |
| 3361 | `lemma_mul_distributes_over_add` | Y |  |  |  | Y |  |  | unknown | 36&#8209;37 |
| 3362 | `lemma_sub_anti_commutative` | Y |  |  |  | Y |  |  | unknown | 40&#8209;41 |

### experiments/simple_hash_set_iter.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3363 | `len` | Y | Y |  |  | Y |  | Y |  | 49 |
| 3364 | `new` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;56 |
| 3365 | `contains` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;60 |
| 3366 | `insert` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;68 |
| 3367 | `iter` | Y | Y |  |  | Y |  |  | hole | 70&#8209;74 |
| 3368 | `simple_hash_set_copy_loop` |  |  |  | Y | Y |  |  | hole | 94&#8209;99 |

### experiments/simple_seq_iter.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3369 | `len` | Y | Y |  |  | Y |  | Y |  | 24 |
| 3370 | `new` | Y | Y |  |  | Y |  |  | unknown | 25 |
| 3371 | `push` | Y | Y |  |  | Y |  |  | unknown | 26 |
| 3372 | `iter` | Y | Y |  |  | Y |  |  | unknown | 27 |
| 3373 | `lemma_iter_invariant` |  |  |  | Y | Y |  |  | unknown | 74&#8209;78 |
| 3374 | `assumption_free_next` |  |  |  | Y | Y |  |  | unknown | 83&#8209;104 |
| 3375 | `next` |  | Y |  |  | Y |  |  | hole | 118&#8209;137 |
| 3376 | `simple_seq_copy_loop` |  |  |  | Y | Y |  |  | unknown | 242&#8209;244 |
| 3377 | `simple_seq_copy_for_iter` |  |  |  | Y | Y |  |  | unknown | 266&#8209;268 |
| 3378 | `simple_seq_copy_for_range` |  |  |  | Y | Y |  |  | unknown | 288&#8209;290 |

### experiments/simple_set_iter.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3379 | `len` | Y | Y |  |  | Y |  | Y |  | 48 |
| 3380 | `new` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;51 |
| 3381 | `mem` | Y | Y |  |  | Y |  |  | hole | 53&#8209;54 |
| 3382 | `insert` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;58 |
| 3383 | `iter` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;70 |
| 3384 | `lemma_iter_invariant` |  |  |  | Y | Y |  |  | unknown | 138&#8209;142 |
| 3385 | `assumption_free_next` |  |  |  | Y | Y |  |  | unknown | 147&#8209;168 |
| 3386 | `next` |  | Y |  |  | Y |  |  | hole | 182&#8209;201 |
| 3387 | `simple_set_copy_loop` |  |  |  | Y | Y |  |  | unknown | 261&#8209;263 |
| 3388 | `simple_set_copy_for` |  |  |  | Y | Y |  |  | unknown | 319&#8209;321 |

### experiments/spec_fun_argument.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3389 | `vec_filter_predicate_provenence_completeness_loop` |  |  |  | Y | Y |  |  | hole | 29&#8209;47 |

### experiments/spec_loop.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3390 | `filter` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 3391 | `map` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;55 |

### experiments/struct_construction_test.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3392 | `_new_valid` |  |  | Y |  |  | Y | Y |  | 13&#8209;15 |
| 3393 | `new` |  |  | Y |  |  | Y | Y |  | 25&#8209;27 |
| 3394 | `get_x` |  |  | Y |  |  | Y | Y |  | 29 |
| 3395 | `get_y` |  |  | Y |  |  | Y | Y |  | 30 |
| 3396 | `test_public_fields` |  |  |  | Y |  | Y | Y |  | 37&#8209;44 |
| 3397 | `test_private_fields` |  |  |  | Y |  | Y | Y |  | 46&#8209;54 |
| 3398 | `main` |  |  |  | Y |  | Y | Y |  | 57&#8209;60 |

### experiments/supertrait.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3399 | `foo` | Y | Y |  |  | Y |  |  | unknown | 8&#8209;9 |
| 3400 | `foo_strict` | Y | Y |  |  | Y |  |  | unknown | 13&#8209;14 |

### experiments/tcb_foul.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3401 | `lying_function` |  |  |  | Y | Y |  |  | hole | 33&#8209;34 |
| 3402 | `tcb_foul_lying_spec` |  |  |  | Y | Y |  | Y |  | 40 |
| 3403 | `lying_increment` |  |  |  | Y | Y |  |  | hole | 54&#8209;56 |
| 3404 | `tcb_foul_mutation` |  |  |  | Y | Y |  | Y |  | 61 |
| 3405 | `tcb_foul_assume` |  |  |  | Y | Y |  |  | hole | 73 |

### experiments/test_feq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3406 | `axiom_point_feq` |  |  |  | Y | Y |  |  | hole | 91&#8209;93 |
| 3407 | `axiom_color_feq` |  |  |  | Y | Y |  |  | hole | 96&#8209;98 |
| 3408 | `axiom_inttree_feq` |  |  |  | Y | Y |  |  | hole | 101&#8209;103 |
| 3409 | `test_generic_reflexive` |  |  |  | Y | Y |  |  | unknown | 113&#8209;114 |
| 3410 | `test_generic_symmetric` |  |  |  | Y | Y |  |  | unknown | 122&#8209;123 |
| 3411 | `test_generic_transitive` |  |  |  | Y | Y |  |  | unknown | 133&#8209;134 |
| 3412 | `test_generic_view` |  |  |  | Y | Y |  |  | unknown | 144&#8209;145 |
| 3413 | `test_generic_view_injective` |  |  |  | Y | Y |  |  | unknown | 154&#8209;155 |
| 3414 | `test_exec_eq_implies_view_eq` |  |  |  | Y | Y |  |  | hole | 167&#8209;168 |
| 3415 | `test_exec_feq_implies_eq_and_view` |  |  |  | Y | Y |  |  | unknown | 178&#8209;179 |
| 3416 | `test_generic_clone` |  |  |  | Y | Y |  |  | unknown | 189&#8209;190 |
| 3417 | `test_generic_no_requires` |  |  |  | Y | Y |  | Y |  | 200 |
| 3418 | `test_u64_reflexive` |  |  |  | Y | Y |  | Y |  | 209 |
| 3419 | `test_u64_symmetric` |  |  |  | Y | Y |  | Y |  | 216 |
| 3420 | `test_u64_transitive` |  |  |  | Y | Y |  | Y |  | 225 |
| 3421 | `test_u64_view` |  |  |  | Y | Y |  | Y |  | 234 |
| 3422 | `test_u64_view_injective` |  |  |  | Y | Y |  | Y |  | 243 |
| 3423 | `test_u64_clone` |  |  |  | Y | Y |  | Y |  | 252 |
| 3424 | `test_point_reflexive` |  |  |  | Y | Y |  |  | unknown | 262&#8209;263 |
| 3425 | `test_point_symmetric` |  |  |  | Y | Y |  |  | unknown | 268&#8209;269 |
| 3426 | `test_point_clone` |  |  |  | Y | Y |  |  | unknown | 274&#8209;275 |
| 3427 | `test_color_reflexive` |  |  |  | Y | Y |  |  | unknown | 281&#8209;282 |
| 3428 | `test_color_symmetric` |  |  |  | Y | Y |  |  | unknown | 287&#8209;288 |
| 3429 | `test_color_clone` |  |  |  | Y | Y |  |  | unknown | 293&#8209;294 |
| 3430 | `test_point_with_axiom` |  |  |  | Y | Y |  | Y |  | 300 |
| 3431 | `test_color_with_axiom` |  |  |  | Y | Y |  | Y |  | 309 |
| 3432 | `test_inttree_reflexive` |  |  |  | Y | Y |  |  | unknown | 319&#8209;320 |
| 3433 | `test_inttree_symmetric` |  |  |  | Y | Y |  |  | unknown | 325&#8209;326 |
| 3434 | `test_inttree_clone` |  |  |  | Y | Y |  |  | unknown | 331&#8209;332 |
| 3435 | `test_inttree_with_axiom` |  |  |  | Y | Y |  | Y |  | 337 |
| 3436 | `test_pair_reflexive` |  |  |  | Y | Y |  |  | unknown | 347&#8209;348 |
| 3437 | `test_pair_symmetric` |  |  |  | Y | Y |  |  | unknown | 353&#8209;354 |
| 3438 | `test_pair_view` |  |  |  | Y | Y |  |  | unknown | 359&#8209;360 |
| 3439 | `test_pair_view_injective` |  |  |  | Y | Y |  |  | unknown | 365&#8209;366 |
| 3440 | `test_pair_with_axiom` |  |  |  | Y | Y |  | Y |  | 371 |

### experiments/test_feq_insertion_sort.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3441 | `is_sorted` |  |  |  | Y | Y |  |  | hole | 20&#8209;22 |
| 3442 | `insertion_sort_while` |  |  |  | Y | Y |  |  | hole | 54&#8209;60 |
| 3443 | `insertion_sort_loop` |  |  |  | Y | Y |  |  | hole | 113&#8209;119 |
| 3444 | `test_insertion_sort_while` |  |  |  | Y | Y |  |  | unknown | 181&#8209;187 |
| 3445 | `test_insertion_sort_loop` |  |  |  | Y | Y |  |  | unknown | 194&#8209;200 |

### experiments/test_test.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3446 | `add_one` |  |  |  | Y | Y |  |  | unknown | 12&#8209;14 |
| 3447 | `multiply_by_two` |  |  |  | Y | Y |  |  | unknown | 20&#8209;22 |
| 3448 | `swap` |  |  |  | Y | Y |  |  | unknown | 28&#8209;29 |

### experiments/total_ord_gen.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3449 | `axiom_eq_reflexive` | Y | Y |  |  | Y |  |  | hole | 16 |
| 3450 | `axiom_eq_symmetric` | Y | Y |  |  | Y |  |  | hole | 17 |
| 3451 | `axiom_eq_transitive` | Y | Y |  |  | Y |  |  | hole | 18 |
| 3452 | `axiom_eq_view` | Y | Y |  |  | Y |  |  | hole | 19 |
| 3453 | `axiom_cloned_implies_eq` | Y | Y |  |  | Y |  |  | hole | 22&#8209;24 |
| 3454 | `axiom_cloned_view_eq` | Y | Y |  |  | Y |  |  | unknown | 26&#8209;28 |
| 3455 | `axiom_le_reflexive` | Y | Y |  |  | Y |  |  | hole | 35 |
| 3456 | `axiom_le_antisymmetric` | Y | Y |  |  | Y |  |  | hole | 36 |
| 3457 | `axiom_le_transitive` | Y | Y |  |  | Y |  |  | hole | 37&#8209;39 |
| 3458 | `axiom_le_total` | Y | Y |  |  | Y |  |  | hole | 40 |
| 3459 | `test_equality_axioms` |  |  |  | Y | Y |  | Y |  | 68 |
| 3460 | `test_clone_axioms` |  |  |  | Y | Y |  | Y |  | 86 |
| 3461 | `test_ordering_axioms` |  |  |  | Y | Y |  | Y |  | 94 |
| 3462 | `test_u64_uses_trait` |  |  |  | Y | Y |  | Y |  | 111 |

### experiments/total_ord_gen_axioms.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3463 | `axiom_eq_transitive` |  |  |  | Y | Y |  |  | hole | 27&#8209;29 |
| 3464 | `axiom_eq_view` |  |  |  | Y | Y |  |  | hole | 34&#8209;36 |
| 3465 | `axiom_cloned_implies_eq` |  |  |  | Y | Y |  |  | hole | 42&#8209;44 |
| 3466 | `axiom_le_reflexive` |  |  |  | Y | Y |  |  | hole | 50&#8209;52 |
| 3467 | `axiom_le_antisymmetric` |  |  |  | Y | Y |  |  | hole | 57&#8209;59 |
| 3468 | `axiom_le_transitive` |  |  |  | Y | Y |  |  | hole | 64&#8209;66 |
| 3469 | `axiom_le_total` |  |  |  | Y | Y |  |  | hole | 71&#8209;73 |
| 3470 | `test_equality_axioms` |  |  |  | Y | Y |  | Y |  | 90 |
| 3471 | `test_clone_axioms` |  |  |  | Y | Y |  | Y |  | 106 |
| 3472 | `test_ordering_axioms` |  |  |  | Y | Y |  | Y |  | 115 |
| 3473 | `test_u64_uses_broadcast` |  |  |  | Y | Y |  | Y |  | 130 |

### experiments/trait_decreases.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3474 | `test1_inherent_works` |  |  |  | Y | Y |  |  | unknown | 50&#8209;51 |
| 3475 | `is_leaf3` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 3476 | `is_leaf4` | Y | Y |  |  | Y |  |  | unknown | 115 |

### experiments/triangle.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3477 | `triangle_is_monotonic` |  |  |  | Y | Y |  |  | unknown | 16&#8209;21 |
| 3478 | `loop_triangle` |  |  |  | Y | Y |  |  | unknown | 28&#8209;32 |
| 3479 | `loop_triangle_with_vec` |  |  |  | Y | Y |  |  | unknown | 52&#8209;57 |

### experiments/unsigned_int.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3480 | `lemma_add_commutative` | Y |  |  |  | Y |  |  | unknown | 19&#8209;20 |
| 3481 | `lemma_mul_commutative` | Y |  |  |  | Y |  |  | unknown | 23&#8209;24 |
| 3482 | `lemma_add_associative` | Y |  |  |  | Y |  |  | unknown | 27&#8209;28 |
| 3483 | `lemma_mul_associative` | Y |  |  |  | Y |  |  | unknown | 31&#8209;32 |
| 3484 | `lemma_mul_distributes_over_add` | Y |  |  |  | Y |  |  | unknown | 35&#8209;36 |
| 3485 | `lemma_add_monotonic` | Y |  |  |  | Y |  |  | unknown | 39&#8209;41 |
| 3486 | `lemma_partial_sums_fit` | Y |  |  |  | Y |  |  | unknown | 44&#8209;50 |

### experiments/use_proven_partialeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3487 | `are_equal` |  |  |  | Y | Y |  |  | unknown | 14&#8209;15 |
| 3488 | `reflexivity_example` |  |  |  | Y | Y |  |  | unknown | 21&#8209;22 |
| 3489 | `symmetry_example` |  |  |  | Y | Y |  |  | unknown | 30&#8209;32 |
| 3490 | `transitivity_example` |  |  |  | Y | Y |  |  | unknown | 40&#8209;42 |
| 3491 | `test_with_i32` |  |  |  | Y | Y |  | Y |  | 50 |
| 3492 | `are_same` |  |  | Y |  | Y |  |  | unknown | 73&#8209;74 |

### experiments/vec_clone_in_verus.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3493 | `deep_clone_wrapper` |  |  | Y |  | Y |  |  | unknown | 64&#8209;65 |
| 3494 | `test_deep_clone` |  |  |  | Y | Y |  | Y |  | 71 |

### experiments/vec_filter.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3495 | `vec_filter_predicate_for` |  |  |  | Y | Y |  |  | unknown | 32&#8209;44 |
| 3496 | `vec_filter_anvil` |  |  |  | Y | Y |  |  | unknown | 71&#8209;81 |
| 3497 | `vec_filter_predicate_provenence_completeness_loop` |  |  |  | Y | Y |  |  | hole | 114&#8209;132 |

### experiments/vec_if.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3498 | `axiom_view_eq_auto_reveal` |  |  |  | Y | Y |  |  | unknown | 17&#8209;24 |
| 3499 | `vec_elem_equals` |  |  |  | Y | Y |  |  | unknown | 32&#8209;39 |
| 3500 | `vec_elem_equals_with_reveal` |  |  |  | Y | Y |  |  | unknown | 53&#8209;60 |
| 3501 | `vec_u64_equals` |  |  |  | Y | Y |  |  | unknown | 74&#8209;77 |

### experiments/vec_length_while_rust.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3502 | `vec_length_while` |  |  |  | Y |  | Y | Y |  | 1&#8209;11 |
| 3503 | `main` |  |  |  | Y |  | Y | Y |  | 13&#8209;17 |

### experiments/vec_length_while_verus.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3504 | `vec_length_while` |  |  |  | Y | Y |  |  | unknown | 7&#8209;8 |

### experiments/vec_remove_duplicates.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3505 | `vec_mem_from` |  |  |  | Y | Y |  |  | unknown | 23&#8209;29 |
| 3506 | `vec_remove_duplicates` |  |  |  | Y | Y |  |  | unknown | 60&#8209;66 |

### experiments/verus_iterator.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3507 | `iter` | Y |  |  |  | Y |  |  | unknown | 25&#8209;27 |
| 3508 | `next` | Y |  |  |  | Y |  |  | unknown | 29&#8209;45 |

### experiments/verus_keep_ghost_and_test.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3509 | `empty` |  |  | Y |  | Y |  |  | unknown | 68&#8209;69 |
| 3510 | `push` |  |  | Y |  | Y |  |  | unknown | 75&#8209;78 |
| 3511 | `len` |  |  | Y |  | Y |  |  | unknown | 84&#8209;85 |
| 3512 | `nth` |  |  | Y |  | Y |  |  | unknown | 91&#8209;93 |
| 3513 | `sum` |  |  | Y |  | Y |  |  | unknown | 100&#8209;103 |
| 3514 | `lemma_push_increases_len` |  |  | Y |  | Y |  |  | unknown | 123&#8209;128 |

### experiments/verus_pub_crate_test.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3515 | `new` |  |  | Y |  | Y |  |  | unknown | 19&#8209;21 |
| 3516 | `test` |  |  |  | Y | Y |  | Y |  | 27 |

### experiments/verus_sum_loops_iterators.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3517 | `stub` |  |  |  | Y | Y |  | Y |  | 9 |

### experiments/verus_vec_iterator.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3518 | `iter` | Y | Y |  |  | Y |  |  | unknown | 24&#8209;25 |
| 3519 | `next` x2 | Y | Y |  |  | Y |  |  | unknown | 26&#8209;38 |

### experiments/verus_vec_iterator_for_basic_proofs.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3520 | `test_for_semantics` |  |  |  | Y | Y |  | Y |  | 13 |

### experiments/verus_vec_iterator_loop_basic_proofs.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3521 | `usize_vec_mem_loop` |  |  |  | Y | Y |  |  | unknown | 12&#8209;13 |
| 3522 | `usize_vec_find_loop` |  |  |  | Y | Y |  |  | unknown | 49&#8209;54 |
| 3523 | `vec_length_up_loop` |  |  |  | Y | Y |  |  | unknown | 84&#8209;85 |
| 3524 | `usize_vec_count_up_loop` |  |  |  | Y | Y |  |  | unknown | 120&#8209;121 |

### experiments/verus_vec_iterator_while_basic_proofs.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3525 | `usize_vec_mem_while` |  |  |  | Y | Y |  |  | unknown | 12&#8209;13 |
| 3526 | `usize_vec_find_while` |  |  |  | Y | Y |  |  | unknown | 47&#8209;52 |
| 3527 | `vec_length_up_while` |  |  |  | Y | Y |  |  | unknown | 80&#8209;81 |
| 3528 | `usize_vec_count_up_while` |  |  |  | Y | Y |  |  | unknown | 113&#8209;114 |

### experiments/verus_wrapped_iter_loops.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3529 | `lemma_found_at_pos` |  |  |  | Y | Y |  |  | hole | 11&#8209;16 |
| 3530 | `lemma_not_found` |  |  |  | Y | Y |  |  | hole | 21&#8209;25 |
| 3531 | `i64_vec_mem_while` |  |  |  | Y | Y |  |  | unknown | 31&#8209;32 |
| 3532 | `i64_vec_mem_loop` |  |  |  | Y | Y |  |  | unknown | 50&#8209;51 |
| 3533 | `i64_vec_mem_for` |  |  |  | Y | Y |  |  | unknown | 71&#8209;72 |
| 3534 | `i64_vec_mem_for_no_auto` |  |  |  | Y | Y |  |  | unknown | 98&#8209;99 |
| 3535 | `i64_vec_mem_for_into_iter` |  |  |  | Y | Y |  |  | unknown | 177&#8209;178 |

### src/Concurrency.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3536 | `diverge` |  |  |  | Y | Y |  | Y |  | 13 |

### src/ParaPairs.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3537 | `para_pair` |  |  |  | Y | Y |  |  | unknown | 52&#8209;63 |
| 3538 | `para_pair_disjoint` |  |  |  | Y | Y |  |  | unknown | 72&#8209;85 |

### src/Types.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3539 | `lemma_wf_graph_view_subset_arcs` |  |  |  | Y | Y |  |  | unknown | 45&#8209;50 |
| 3540 | `lemma_wf_lab_graph_view_subset_arcs` |  |  |  | Y | Y |  |  | unknown | 72&#8209;77 |
| 3541 | `axiom_Pair_view_injective` |  |  |  | Y | Y |  |  | hole | 188&#8209;192 |
| 3542 | `axiom_Pair_feq` |  |  |  | Y | Y |  |  | hole | 204&#8209;206 |
| 3543 | `axiom_Edge_feq` |  |  |  | Y | Y |  |  | hole | 226&#8209;228 |
| 3544 | `axiom_Edge_key_model` |  |  |  | Y | Y |  |  | hole | 231&#8209;233 |
| 3545 | `axiom_LabEdge_feq` |  |  |  | Y | Y |  |  | hole | 253&#8209;255 |
| 3546 | `axiom_LabEdge_key_model` |  |  |  | Y | Y |  |  | hole | 258&#8209;260 |
| 3547 | `axiom_WeightedEdge_feq` |  |  |  | Y | Y |  |  | hole | 275&#8209;277 |
| 3548 | `axiom_WeightedEdge_key_model` |  |  |  | Y | Y |  |  | hole | 280&#8209;282 |
| 3549 | `axiom_WeightedLabEdge_feq` |  |  |  | Y | Y |  |  | hole | 302&#8209;304 |
| 3550 | `axiom_WeightedLabEdge_key_model` |  |  |  | Y | Y |  |  | hole | 307&#8209;309 |
| 3551 | `axiom_Triple_feq` |  |  |  | Y | Y |  |  | hole | 324&#8209;326 |
| 3552 | `axiom_Triple_key_model` |  |  |  | Y | Y |  |  | hole | 329&#8209;331 |
| 3553 | `from` x10 |  | Y |  |  |  | Y | Y |  | 431 |
| 3554 | `next` |  | Y |  |  |  | Y | Y |  | 502&#8209;504 |
| 3555 | `deref` |  | Y |  |  |  | Y | Y |  | 511&#8209;513 |
| 3556 | `deref_mut` |  | Y |  |  |  | Y | Y |  | 517&#8209;519 |

### vstdplus/VecQueue.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3557 | `new` | Y | Y |  |  | Y |  |  | unknown | 20&#8209;21 |
| 3558 | `len` | Y | Y |  |  | Y |  |  | unknown | 23&#8209;24 |
| 3559 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 26&#8209;27 |
| 3560 | `enqueue` | Y | Y |  |  | Y |  |  | unknown | 29&#8209;30 |
| 3561 | `dequeue` | Y | Y |  |  | Y |  |  | unknown | 32&#8209;35 |

### vstdplus/accept.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3562 | `accept` x2 |  |  |  | Y | Y |  |  | hole | 17&#8209;18 |

### vstdplus/checked_int.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3563 | `is_normal` | Y |  |  |  | Y |  |  | unknown | 88&#8209;89 |
| 3564 | `is_out_of_range` | Y |  |  |  | Y |  |  | unknown | 91&#8209;92 |
| 3565 | `add_checked` | Y |  |  |  | Y |  |  | unknown | 94&#8209;95 |
| 3566 | `sub_checked` | Y |  |  |  | Y |  |  | unknown | 97&#8209;98 |
| 3567 | `mul_checked` | Y |  |  |  | Y |  |  | unknown | 100&#8209;101 |
| 3568 | `lemma_add_commutative_ghost` | Y |  |  |  | Y |  |  | unknown | 105&#8209;106 |
| 3569 | `lemma_mul_commutative_ghost` | Y |  |  |  | Y |  |  | unknown | 109&#8209;111 |
| 3570 | `lemma_sub_anticommutative_ghost` | Y |  |  |  | Y |  |  | unknown | 113&#8209;114 |
| 3571 | `lemma_add_associative_ghost` | Y |  |  |  | Y |  |  | unknown | 118&#8209;119 |
| 3572 | `lemma_mul_associative_ghost` | Y |  |  |  | Y |  |  | unknown | 122&#8209;123 |
| 3573 | `lemma_mul_distributes_over_add_ghost` | Y |  |  |  | Y |  |  | unknown | 130&#8209;132 |
| 3574 | `lemma_mul_distributes_over_sub_ghost` | Y |  |  |  | Y |  |  | unknown | 134&#8209;135 |

### vstdplus/checked_nat.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3575 | `is_normal` | Y |  |  |  | Y |  |  | unknown | 87&#8209;88 |
| 3576 | `is_overflow` | Y |  |  |  | Y |  |  | unknown | 90&#8209;91 |
| 3577 | `add_checked` | Y |  |  |  | Y |  |  | unknown | 93&#8209;94 |
| 3578 | `mul_checked` | Y |  |  |  | Y |  |  | unknown | 96&#8209;97 |
| 3579 | `lemma_add_commutative_ghost` | Y |  |  |  | Y |  |  | unknown | 101&#8209;102 |
| 3580 | `lemma_mul_commutative_ghost` | Y |  |  |  | Y |  |  | unknown | 105&#8209;106 |
| 3581 | `lemma_add_associative_ghost` | Y |  |  |  | Y |  |  | unknown | 112&#8209;113 |
| 3582 | `lemma_mul_associative_ghost` | Y |  |  |  | Y |  |  | unknown | 115&#8209;116 |
| 3583 | `lemma_mul_distributes_over_add_ghost` | Y |  |  |  | Y |  |  | unknown | 121&#8209;122 |
| 3584 | `lemma_sum_monotonic` | Y |  |  |  | Y |  |  | unknown | 127&#8209;134 |
| 3585 | `lemma_add_normal_if_sum_fits` | Y |  |  |  | Y |  |  | unknown | 137&#8209;142 |

### vstdplus/checked_nat_with_checked_view.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3586 | `is_normal` | Y |  |  |  | Y |  |  | unknown | 62&#8209;63 |
| 3587 | `is_overflow` | Y |  |  |  | Y |  |  | unknown | 65&#8209;66 |
| 3588 | `add_checked` | Y |  |  |  | Y |  |  | unknown | 68&#8209;69 |
| 3589 | `mul_checked` | Y |  |  |  | Y |  |  | unknown | 71&#8209;72 |

### vstdplus/clone_plus.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3590 | `clone_plus` x3 | Y | Y |  |  | Y |  |  | hole | 62 |
| 3591 | `clone_fn` x2 |  |  |  | Y | Y |  |  | hole | 28&#8209;31 |
| 3592 | `clone_fn2` x2 |  |  |  | Y | Y |  |  | hole | 38&#8209;41 |
| 3593 | `clone_pred` x2 |  |  |  | Y | Y |  |  | hole | 48&#8209;51 |

### vstdplus/feq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3594 | `lemma_cloned_view_eq` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 3595 | `lemma_view_injective` |  |  |  | Y | Y |  |  | unknown | 86&#8209;88 |
| 3596 | `axiom_cloned_implies_eq` |  |  |  | Y | Y |  |  | hole | 163&#8209;165 |
| 3597 | `axiom_cloned_implies_eq_owned` |  |  |  | Y | Y |  |  | hole | 172&#8209;174 |
| 3598 | `feq` x2 |  |  |  | Y | Y |  |  | hole | 180&#8209;182 |
| 3599 | `obeys_feq_clone` |  |  |  | Y |  | Y | Y |  | 210&#8209;211 |

### vstdplus/float.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3600 | `reflexive` x2 | Y | Y |  |  | Y |  |  | unknown | 34&#8209;36 |
| 3601 | `antisymmetric` x2 | Y | Y |  |  | Y |  |  | unknown | 39&#8209;42 |
| 3602 | `transitive` x2 | Y | Y |  |  | Y |  |  | unknown | 45&#8209;48 |
| 3603 | `totality` x2 | Y | Y |  |  | Y |  |  | unknown | 51&#8209;53 |
| 3604 | `float_cmp` x2 | Y | Y |  |  | Y |  |  | hole | 56&#8209;63 |
| 3605 | `axiom_f64_le_functional` |  |  |  | Y | Y |  |  | unknown | 114&#8209;117 |
| 3606 | `axiom_f64_reflexive` |  |  |  | Y | Y |  |  | unknown | 119&#8209;121 |
| 3607 | `axiom_f64_antisymmetric` |  |  |  | Y | Y |  |  | unknown | 123&#8209;128 |
| 3608 | `axiom_f64_transitive` |  |  |  | Y | Y |  |  | unknown | 130&#8209;135 |
| 3609 | `axiom_f64_totality` |  |  |  | Y | Y |  |  | unknown | 137&#8209;141 |
| 3610 | `axiom_f32_le_functional` |  |  |  | Y | Y |  |  | unknown | 181&#8209;184 |
| 3611 | `axiom_f32_reflexive` |  |  |  | Y | Y |  |  | unknown | 186&#8209;188 |
| 3612 | `axiom_f32_antisymmetric` |  |  |  | Y | Y |  |  | unknown | 190&#8209;195 |
| 3613 | `axiom_f32_transitive` |  |  |  | Y | Y |  |  | unknown | 197&#8209;202 |
| 3614 | `axiom_f32_totality` |  |  |  | Y | Y |  |  | unknown | 204&#8209;208 |
| 3615 | `is_finite` |  |  | Y |  | Y |  |  | hole | 229&#8209;230 |
| 3616 | `eq` |  | Y | Y |  | Y |  |  | hole | 236&#8209;237 |
| 3617 | `dist_le` |  |  | Y |  | Y |  |  | hole | 243&#8209;245 |
| 3618 | `dist_lt` |  |  | Y |  | Y |  |  | hole | 251&#8209;253 |
| 3619 | `dist_add` |  |  | Y |  | Y |  |  | hole | 259&#8209;260 |
| 3620 | `dist_sub` |  |  | Y |  | Y |  |  | hole | 266&#8209;267 |
| 3621 | `f64_is_finite` |  |  |  | Y | Y |  |  | hole | 275&#8209;276 |
| 3622 | `axiom_f64_zero_is_finite` |  |  |  | Y | Y |  |  | unknown | 286&#8209;287 |
| 3623 | `axiom_f64_unreachable_not_finite` |  |  |  | Y | Y |  |  | unknown | 289&#8209;290 |
| 3624 | `unreachable_dist` |  |  |  | Y | Y |  |  | hole | 293&#8209;295 |
| 3625 | `zero_dist` |  |  |  | Y | Y |  |  | hole | 301&#8209;303 |
| 3626 | `finite_dist` |  |  |  | Y | Y |  |  | unknown | 308&#8209;311 |
| 3627 | `axiom_f64_add_zero_right` |  |  |  | Y | Y |  |  | unknown | 323&#8209;325 |
| 3628 | `axiom_f64_add_commutative` |  |  |  | Y | Y |  |  | unknown | 327&#8209;330 |
| 3629 | `axiom_f64_add_finite_preserves` |  |  |  | Y | Y |  |  | unknown | 332&#8209;336 |
| 3630 | `axiom_f64_add_monotone_left` |  |  |  | Y | Y |  |  | unknown | 338&#8209;345 |
| 3631 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 391&#8209;393 |
| 3632 | `cmp` |  | Y |  |  |  | Y | Y |  | 397&#8209;399 |
| 3633 | `hash` |  | Y |  |  |  | Y | Y |  | 403&#8209;405 |
| 3634 | `add` |  | Y |  |  |  | Y | Y |  | 410&#8209;412 |
| 3635 | `add_assign` |  | Y |  |  |  | Y | Y |  | 416&#8209;418 |

### vstdplus/hash_set_with_view_plus.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3636 | `axiom_hash_set_with_view_plus_finite` |  |  |  | Y | Y |  |  | hole | 38&#8209;40 |
| 3637 | `new` |  |  | Y |  | Y |  |  | hole | 60&#8209;65 |
| 3638 | `with_capacity` |  |  | Y |  | Y |  |  | hole | 71&#8209;76 |
| 3639 | `len` |  |  | Y |  | Y |  |  | hole | 82&#8209;84 |
| 3640 | `contains` |  |  | Y |  | Y |  |  | hole | 90&#8209;94 |
| 3641 | `insert` |  |  | Y |  | Y |  |  | hole | 100&#8209;106 |
| 3642 | `iter` | Y | Y |  |  | Y |  |  | hole | 113&#8209;117 |
| 3643 | `hash` |  | Y |  |  | Y |  |  | hole | 132 |
| 3644 | `eq` |  | Y |  |  | Y |  |  | hole | 141 |

### vstdplus/hashed_checked_u32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3645 | `hash` |  | Y |  |  | Y |  |  | hole | 31 |
| 3646 | `eq` |  | Y |  |  | Y |  |  | hole | 39 |
| 3647 | `axiom_CheckedU32_feq` |  |  |  | Y | Y |  |  | hole | 50&#8209;52 |
| 3648 | `axiom_CheckedU32_key_model` |  |  |  | Y | Y |  |  | hole | 56&#8209;58 |

### vstdplus/multiset.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3649 | `lemma_spec_filter_len_nonneg` |  |  |  | Y | Y |  |  | unknown | 31&#8209;33 |
| 3650 | `lemma_spec_filter_len_le_len` |  |  |  | Y | Y |  |  | unknown | 41&#8209;43 |
| 3651 | `lemma_flatten_01_eq_spec_filter_len` |  |  |  | Y | Y |  |  | unknown | 54&#8209;65 |
| 3652 | `lemma_multiset_insert_filter_pos` |  |  |  | Y | Y |  |  | unknown | 86&#8209;88 |
| 3653 | `lemma_multiset_insert_filter_neg` |  |  |  | Y | Y |  |  | unknown | 96&#8209;98 |
| 3654 | `lemma_flatten_01_multiset_eq_filter` |  |  |  | Y | Y |  |  | unknown | 109&#8209;121 |

### vstdplus/partial_order.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3655 | `reflexive` x14 | Y | Y |  |  | Y |  |  | hole | 17&#8209;20 |
| 3656 | `transitive` x14 | Y | Y |  |  | Y |  |  | hole | 23&#8209;29 |
| 3657 | `antisymmetric` x14 | Y | Y |  |  | Y |  |  | hole | 32&#8209;38 |
| 3658 | `compare` x14 | Y | Y |  |  | Y |  |  | hole | 40&#8209;48 |

### vstdplus/rand.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3659 | `random_usize_range` |  |  |  | Y | Y |  |  | hole | 21&#8209;23 |
| 3660 | `random_usize_range_exec` x2 |  |  |  | Y |  | Y | Y |  | 30&#8209;35 |

### vstdplus/seq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3661 | `seq_u32_to_CheckedU32` |  |  |  | Y | Y |  |  | hole | 32&#8209;34 |
| 3662 | `lemma_sum_u32_unfold_take` |  |  |  | Y | Y |  |  | unknown | 77&#8209;79 |
| 3663 | `lemma_sum_int_equiv` |  |  |  | Y | Y |  |  | unknown | 105&#8209;107 |
| 3664 | `lemma_sum_int_push` |  |  |  | Y | Y |  |  | unknown | 117&#8209;118 |
| 3665 | `lemma_sum_int_unfold_take` |  |  |  | Y | Y |  |  | unknown | 125&#8209;127 |
| 3666 | `lemma_sum_checked_u32_unfold_take` |  |  |  | Y | Y |  |  | unknown | 143&#8209;145 |
| 3667 | `lemma_flatten_uniform_len` |  |  |  | Y | Y |  |  | unknown | 160&#8209;165 |
| 3668 | `lemma_flatten_len_is_inner_lens_sum` |  |  |  | Y | Y |  |  | unknown | 190&#8209;192 |
| 3669 | `lemma_flatten_all` |  |  |  | Y | Y |  |  | unknown | 201&#8209;210 |
| 3670 | `lemma_flatten_contains` |  |  |  | Y | Y |  |  | unknown | 237&#8209;240 |

### vstdplus/seq_set.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3671 | `lemma_push_not_contains_to_set_subset` |  |  |  | Y | Y |  |  | unknown | 20&#8209;24 |
| 3672 | `lemma_push_not_contains_to_set_superset` |  |  |  | Y | Y |  |  | unknown | 42&#8209;46 |
| 3673 | `lemma_push_not_contains_to_set` |  |  |  | Y | Y |  |  | unknown | 62&#8209;66 |
| 3674 | `lemma_take_extends_set_subset` |  |  |  | Y | Y |  |  | unknown | 77&#8209;81 |
| 3675 | `lemma_take_extends_set_superset` |  |  |  | Y | Y |  |  | unknown | 102&#8209;106 |
| 3676 | `lemma_take_one_more_extends_the_seq_set` |  |  |  | Y | Y |  |  | unknown | 125&#8209;129 |
| 3677 | `lemma_set_contains_insert_idempotent` |  |  |  | Y | Y |  |  | unknown | 137&#8209;141 |
| 3678 | `lemma_take_one_more_extends_the_seq_set_with_view` |  |  |  | Y | Y |  |  | unknown | 151&#8209;155 |
| 3679 | `lemma_take_full_to_set_with_view` |  |  |  | Y | Y |  |  | unknown | 191&#8209;194 |
| 3680 | `lemma_seq_index_in_map_to_set` |  |  |  | Y | Y |  |  | unknown | 201&#8209;205 |
| 3681 | `lemma_map_to_set_contains_index` |  |  |  | Y | Y |  |  | unknown | 216&#8209;220 |
| 3682 | `lemma_map_not_contains_implies_all_ne` |  |  |  | Y | Y |  |  | unknown | 232&#8209;236 |
| 3683 | `lemma_seq_map_to_set_equality` |  |  |  | Y | Y |  |  | unknown | 246&#8209;252 |
| 3684 | `lemma_take_one_more_intersect` |  |  |  | Y | Y |  |  | unknown | 283&#8209;292 |
| 3685 | `lemma_spec_nat_seq_fold_equals_spec_set_fold` |  |  |  | Y | Y |  |  | unknown | 342&#8209;347 |
| 3686 | `lemma_to_seq_no_duplicates` |  |  |  | Y | Y |  |  | unknown | 382&#8209;387 |
| 3687 | `lemma_spec_nat_seq_sum_is_nat_set_sum` |  |  |  | Y | Y |  |  | unknown | 416&#8209;420 |
| 3688 | `lemma_nat_partial_sum_monotonic` |  |  |  | Y | Y |  |  | unknown | 433&#8209;438 |
| 3689 | `lemma_nat_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 447&#8209;451 |
| 3690 | `lemma_spec_nat_seq_sum_no_intermediate_overflow` |  |  |  | Y | Y |  |  | unknown | 474&#8209;478 |
| 3691 | `lemma_nat_any_order_no_overflow` |  |  |  | Y | Y |  |  | unknown | 490&#8209;498 |
| 3692 | `lemma_no_dup_same_set_implies_same_multiset` |  |  |  | Y | Y |  |  | unknown | 505&#8209;511 |
| 3693 | `lemma_spec_nat_seq_sum_permutation_invariant` |  |  |  | Y | Y |  |  | unknown | 533&#8209;539 |
| 3694 | `lemma_u32_view_identity` |  |  |  | Y | Y |  |  | unknown | 558&#8209;559 |
| 3695 | `lemma_to_seq_gives_same_set` |  |  |  | Y | Y |  |  | unknown | 567&#8209;572 |
| 3696 | `lemma_seq_map_to_set_eq_set_map` |  |  |  | Y | Y |  |  | unknown | 589&#8209;594 |
| 3697 | `lemma_set_contains_iff_to_seq_map_contains` |  |  |  | Y | Y |  |  | unknown | 614&#8209;619 |
| 3698 | `lemma_weighted_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 646&#8209;650 |
| 3699 | `lemma_weighted_seq_fold_equals_set_fold` |  |  |  | Y | Y |  |  | unknown | 660&#8209;665 |
| 3700 | `lemma_weighted_seq_sum_is_set_sum` |  |  |  | Y | Y |  |  | unknown | 703&#8209;707 |
| 3701 | `lemma_int_fold_equals_nat_fold_weighted` |  |  |  | Y | Y |  |  | unknown | 718&#8209;722 |
| 3702 | `lemma_seq_fold_left_plus_is_weighted_seq_sum` |  |  |  | Y | Y |  |  | unknown | 735&#8209;739 |
| 3703 | `lemma_fold_left_int_equals_nat_as_int` |  |  |  | Y | Y |  |  | unknown | 768&#8209;772 |
| 3704 | `lemma_seq_fold_left_plus_is_weighted_seq_sum_u8` |  |  |  | Y | Y |  |  | unknown | 804&#8209;806 |
| 3705 | `lemma_fold_left_int_equals_nat_as_int_u8` |  |  |  | Y | Y |  |  | unknown | 817&#8209;819 |
| 3706 | `lemma_weighted_seq_fold_equals_set_fold_u8` |  |  |  | Y | Y |  |  | unknown | 823&#8209;826 |
| 3707 | `lemma_seq_fold_left_plus_is_weighted_seq_sum_u16` |  |  |  | Y | Y |  |  | unknown | 854&#8209;856 |
| 3708 | `lemma_fold_left_int_equals_nat_as_int_u16` |  |  |  | Y | Y |  |  | unknown | 867&#8209;869 |
| 3709 | `lemma_weighted_seq_fold_equals_set_fold_u16` |  |  |  | Y | Y |  |  | unknown | 873&#8209;876 |
| 3710 | `lemma_seq_fold_left_plus_is_weighted_seq_sum_u64` |  |  |  | Y | Y |  |  | unknown | 902&#8209;904 |
| 3711 | `lemma_fold_left_int_equals_nat_as_int_u64` |  |  |  | Y | Y |  |  | unknown | 916&#8209;918 |
| 3712 | `lemma_weighted_seq_fold_equals_set_fold_u64` |  |  |  | Y | Y |  |  | unknown | 922&#8209;925 |
| 3713 | `lemma_seq_fold_left_plus_is_weighted_seq_sum_u128` |  |  |  | Y | Y |  |  | unknown | 952&#8209;954 |
| 3714 | `lemma_fold_left_int_equals_nat_as_int_u128` |  |  |  | Y | Y |  |  | unknown | 966&#8209;968 |
| 3715 | `lemma_weighted_seq_fold_equals_set_fold_u128` |  |  |  | Y | Y |  |  | unknown | 972&#8209;975 |
| 3716 | `lemma_seq_fold_left_plus_is_weighted_seq_sum_usize` |  |  |  | Y | Y |  |  | unknown | 1002&#8209;1004 |
| 3717 | `lemma_fold_left_int_equals_nat_as_int_usize` |  |  |  | Y | Y |  |  | unknown | 1016&#8209;1018 |
| 3718 | `lemma_weighted_seq_fold_equals_set_fold_usize` |  |  |  | Y | Y |  |  | unknown | 1022&#8209;1025 |
| 3719 | `lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i8` |  |  |  | Y | Y |  |  | unknown | 1054&#8209;1056 |
| 3720 | `lemma_signed_weighted_seq_fold_equals_set_fold_i8` |  |  |  | Y | Y |  |  | unknown | 1066&#8209;1069 |
| 3721 | `lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i16` |  |  |  | Y | Y |  |  | unknown | 1095&#8209;1097 |
| 3722 | `lemma_signed_weighted_seq_fold_equals_set_fold_i16` |  |  |  | Y | Y |  |  | unknown | 1108&#8209;1111 |
| 3723 | `lemma_signed_seq_fold_left_plus_is_weighted_seq_sum` |  |  |  | Y | Y |  |  | unknown | 1138&#8209;1140 |
| 3724 | `lemma_signed_weighted_seq_fold_equals_set_fold` |  |  |  | Y | Y |  |  | unknown | 1150&#8209;1154 |
| 3725 | `lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i64` |  |  |  | Y | Y |  |  | unknown | 1180&#8209;1182 |
| 3726 | `lemma_signed_weighted_seq_fold_equals_set_fold_i64` |  |  |  | Y | Y |  |  | unknown | 1192&#8209;1196 |
| 3727 | `lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i128` |  |  |  | Y | Y |  |  | unknown | 1222&#8209;1224 |
| 3728 | `lemma_signed_weighted_seq_fold_equals_set_fold_i128` |  |  |  | Y | Y |  |  | unknown | 1234&#8209;1238 |
| 3729 | `lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_isize` |  |  |  | Y | Y |  |  | unknown | 1264&#8209;1266 |
| 3730 | `lemma_signed_weighted_seq_fold_equals_set_fold_isize` |  |  |  | Y | Y |  |  | unknown | 1276&#8209;1279 |

### vstdplus/smart_ptrs.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3731 | `call_f` |  |  |  | Y | Y |  |  | unknown | 20&#8209;26 |
| 3732 | `arc_deref` |  |  |  | Y | Y |  |  | hole | 34&#8209;35 |
| 3733 | `arc_vec_as_slice` |  |  |  | Y | Y |  |  | hole | 43&#8209;45 |

### vstdplus/threads_plus.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3734 | `is_finished` |  |  | Y |  | Y |  |  | hole | 22 |
| 3735 | `join` |  |  | Y |  | Y |  |  | hole | 25&#8209;26 |
| 3736 | `spawn_plus` |  |  |  | Y | Y |  |  | hole | 40&#8209;43 |
| 3737 | `agrees` |  |  | Y |  | Y |  |  | unknown | 73&#8209;74 |
| 3738 | `thread_id_plus` |  |  |  | Y | Y |  |  | hole | 80&#8209;81 |
| 3739 | `ghost_thread_id_plus` |  |  |  | Y | Y |  | Y |  | 87 |
| 3740 | `into` |  |  | Y |  | Y |  |  | unknown | 96&#8209;99 |
| 3741 | `borrow` |  |  | Y |  | Y |  |  | unknown | 100&#8209;102 |
| 3742 | `send_into` |  |  | Y |  | Y |  |  | unknown | 106&#8209;107 |
| 3743 | `sync_borrow` |  |  | Y |  | Y |  |  | unknown | 111&#8209;112 |

### vstdplus/total_order.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3744 | `reflexive` x12 | Y | Y |  |  | Y |  |  | unknown | 13&#8209;16 |
| 3745 | `transitive` x12 | Y | Y |  |  | Y |  |  | unknown | 19&#8209;25 |
| 3746 | `antisymmetric` x12 | Y | Y |  |  | Y |  |  | unknown | 28&#8209;34 |
| 3747 | `total` x12 | Y | Y |  |  | Y |  |  | unknown | 37&#8209;40 |
| 3748 | `cmp` x12 | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
