# R197 Benchmark Coverage Inventory

Generated: 2026-04-11. All columns are per the prompt spec.
Benched = Y if a `[[bench]]` entry exists in Cargo.toml as of R197.
Skipped: `Example*.rs`, `Problem*.rs`, Mt-only files with no St counterpart, pure-spec files.

| # | Chap | File | Benched? | Asymptotic class | Priority |
|---|------|------|----------|-----------------|----------|
| 1 | 02 | HFSchedulerMtEph | N | O(W/P + D) fork-join | skip (Mt scheduler infra) |
| 2 | 02 | FibonacciHFScheduler | N | O(n/P) parallel fib | low (Mt, needs threads) |
| 3 | 03 | InsertionSortStEph | **Y** | O(n²) | done |
| 4 | 05 | SetStEph | N | O(n) insert/contains | medium |
| 5 | 05 | MappingStEph | N | O(n) insert/find | medium |
| 6 | 06 | DirGraphStEph | N | O(V+E) construction | low (data structure, not algorithm) |
| 7 | 06 | WeightedDirGraphStEphI64 | N | O(V+E) | low |
| 8 | 11 | FibonacciStEph | **Y** | O(n) iterative, O(φⁿ) recursive | done |
| 9 | 11 | FibonacciMtEph2Threads | N | O(n/P) | low (Mt) |
| 10 | 17 | MathSeq | N | pure spec | skip |
| 11 | 18 | ArraySeqStPer | N | O(n) append/subseq | medium |
| 12 | 18 | LinkedListStEph | N | O(n) traverse | low |
| 13 | 19 | ArraySeqStEph | **Y** | O(n) append/subseq | done |
| 14 | 19 | ArraySeqStPer | N | O(n) | medium |
| 15 | 21 | Algorithm21_1 | N | O(n) scan | medium |
| 16 | 21 | Algorithm21_2 | N | O(n) reduce | medium |
| 17 | 23 | BalBinTreeStEph | N | O(n) tree ops | low |
| 18 | 26 | MergeSortStPer | **Y** | O(n log n) | done |
| 19 | 26 | ETSPStEph | N | O(n log n) ETSP | high |
| 20 | 26 | ScanDCStPer | N | O(n log n) scan D&C | medium |
| 21 | 26 | DivConReduceStPer | N | O(n) D&C reduce | medium |
| 22 | 27 | ReduceContractStEph | N | O(n) | low |
| 23 | 27 | ScanContractStEph | N | O(n) | low |
| 24 | 28 | MaxContigSubSumOptStEph | **Y** | O(n) Kadane | done |
| 25 | 28 | MaxContigSubSumBruteStEph | N | O(n³) brute | medium |
| 26 | 28 | MaxContigSubSumIterStEph | N | O(n) iterative | medium |
| 27 | 28 | MaxContigSubSumDivConStEph | N | O(n log n) D&C | medium |
| 28 | 35 | OrderStatSelectStEph | **Y** | O(n) expected | done |
| 29 | 36 | QuickSortStEph | **Y** | O(n log n) expected | done |
| 30 | 37 | BSTPlainStEph | **Y** | O(n log n) insert, O(log n) find | done |
| 31 | 37 | BSTAVLStEph | N | O(log n) balanced insert | high |
| 32 | 37 | BSTSplayStEph | N | O(log n) amortized | high |
| 33 | 37 | BSTBBAlphaStEph | N | O(log n) weight-balanced | high |
| 34 | 37 | BSTRBStEph | N | O(log n) red-black | high |
| 35 | 38 | BSTParaStEph | N | O(log² n) parallel BST | medium |
| 36 | 39 | BSTTreapStEph | N | O(log n) expected treap | high |
| 37 | 40 | BSTKeyValueStEph | N | O(log n) map | medium |
| 38 | 41 | ArraySetStEph | N | O(n) set ops | medium |
| 39 | 41 | AVLTreeSetStEph | N | O(log n) set | high |
| 40 | 42 | TableStEph | N | O(n) table | medium |
| 41 | 43 | OrderedTableStEph | N | O(log n) ordered table | high |
| 42 | 43 | OrderedSetStEph | N | O(log n) ordered set | high |
| 43 | 44 | DocumentIndex | N | O(n) text indexing | medium |
| 44 | 45 | BinaryHeapPQ | N | O(log n) PQ | high |
| 45 | 45 | LeftistHeapPQ | N | O(log n) PQ | high |
| 46 | 47 | VecChainedHashTableStEph | N | O(1) amortized hash | high |
| 47 | 47 | LinProbFlatHashTableStEph | N | O(1) amortized hash | high |
| 48 | 49 | SubsetSumStEph | **Y** | O(k·n) DP | done |
| 49 | 49 | MinEditDistStEph | **Y** | O(m·n) DP | done |
| 50 | 50 | MatrixChainStEph | N | O(n³) DP | medium |
| 51 | 50 | OptBinSearchTreeStEph | N | O(n²) DP | medium |
| 52 | 51 | TopDownDPStEph | N | O(varies) | low |
| 53 | 51 | BottomUpDPStEph | N | O(varies) | low |
| 54 | 52 | AdjSeqGraphStEph | N | O(V+E) | low |
| 55 | 53 | GraphSearchStEph | N | O(V+E) BFS/DFS | medium |
| 56 | 53 | PQMinStEph | N | O(log n) PQ | medium |
| 57 | 54 | BFSStEph | N | O(V+E) BFS | medium |
| 58 | 55 | DFSStEph | N | O(V+E) DFS | medium |
| 59 | 55 | TopoSortStEph | N | O(V+E) | medium |
| 60 | 55 | CycleDetectStEph | N | O(V+E) | medium |
| 61 | 57 | DijkstraStEphU64 | N | O((V+E) log V) | high |
| 62 | 58 | BellmanFordStEphI64 | N | O(VE) | high |
| 63 | 61 | EdgeContractionStEph | N | O(V+E) | medium |
| 64 | 62 | StarContractionStEph | N | O(V+E) | medium |
| 65 | 63 | ConnectivityStEph | N | O(V log V) | medium |
| 66 | 64 | SpanTreeStEph | N | O(E log V) MST | high |
| 67 | 65 | KruskalStEph | N | O(E log E) MST | high |
| 68 | 65 | PrimStEph | N | O(E log V) MST | high |
| 69 | 65 | UnionFindPCStEph | **Y** | O(α(n)) amortized | done |
| 70 | 66 | BoruvkaStEph | N | O(E log V) MST | high |
