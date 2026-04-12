# Algorithms List for Literature Search

Generated 2026-04-12 from `main`. Covers all `src/Chap*/` files
(excluding `Example*.rs` and `Problem*.rs` per CLAUDE.md).

**Columns:**
- **#** — row number (stable for referencing in research notes).
- **Chap** — APAS chapter number.
- **File** — source file base name. Each row corresponds to exactly
  one `src/ChapNN/<File>.rs`.
- **Mt?** — `Yes` if multi-threaded (file name contains `Mt`), `No`
  if sequential, `—` if neither applies (e.g., foundational files).
- **Per/Eph** — `Per` if persistent (file name contains `Per`), `Eph`
  if ephemeral, `—` if neither.
- **Description** — first non-copyright documentation line from the
  module's `//!` header.

For APAS-VERUS variant naming:
- **StEph** = sequential ephemeral (classic mutable data structure).
- **StPer** = sequential persistent (functional / copy-on-write).
- **MtEph** = multi-threaded ephemeral (thread-safe mutable, backed
  by RwLock).
- **MtPer** = multi-threaded persistent (thread-safe persistent,
  usually Arc-shared).

Total files: 263 across 44 chapters.

| # | Chap | File | Mt? | Per/Eph | Description |
|---|------|------|-----|---------|-------------|
| 1 | 02 | FibonacciHFScheduler | — | — | Parallel Fibonacci demonstrating bounded parallelism with global pool. |
| 2 | 02 | HFSchedulerMtEph | Yes | Eph | Help-first scheduler with bounded parallelism using a global pool. |
| 3 | 03 | InsertionSortStEph | No | Eph | Chapter 3 insertion sort over mutable slices - Generic version using TotalOrder trait. |
| 4 | 05 | KleeneStPer | No | Per | Persistent Kleene Star and Plus over a finite alphabet (Definition 5.4, Exercise 5.1). |
| 5 | 05 | MappingStEph | No | Eph | Chapter 5.5 ephemeral Mapping (Function) built on `RelationStEph<A,B>`. |
| 6 | 05 | RelationStEph | No | Eph | Chapter 5.2 ephemeral Relation built on `SetStEph<Pair<A,B>>`. |
| 7 | 05 | SetMtEph | Yes | Eph | Chapter 5.1 — Multi-threaded ephemeral Set built on `std::collections::HashSet`. |
| 8 | 05 | SetStEph | No | Eph | Ephemeral Set built on `std::collections::HashSet` as wrapped by vstd and vstdplus. |
| 9 | 06 | DirGraphMtEph | Yes | Eph | Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs - Multi-threaded version. |
| 10 | 06 | DirGraphStEph | No | Eph | Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs. |
| 11 | 06 | LabDirGraphMtEph | Yes | Eph | Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs - Multi-threaded version. |
| 12 | 06 | LabDirGraphStEph | No | Eph | Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs. |
| 13 | 06 | LabUnDirGraphMtEph | Yes | Eph | Chapter 6 Labeled Undirected Graph (ephemeral) using Set for vertices and labeled edges - Multi-threaded version. |
| 14 | 06 | LabUnDirGraphStEph | No | Eph | Chapter 6 Labeled Undirected Graph (ephemeral) using Set for vertices and labeled edges. |
| 15 | 06 | UnDirGraphMtEph | Yes | Eph | Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges - Multi-threaded version. |
| 16 | 06 | UnDirGraphStEph | No | Eph | Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges. |
| 17 | 06 | WeightedDirGraphStEphF64 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with f64 weights - Single-threaded version. |
| 18 | 06 | WeightedDirGraphStEphI128 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with i128 weights - Single-threaded version. |
| 19 | 06 | WeightedDirGraphStEphI16 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with i16 weights - Single-threaded version. |
| 20 | 06 | WeightedDirGraphStEphI32 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with i32 weights - Single-threaded version. |
| 21 | 06 | WeightedDirGraphStEphI64 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with i64 weights - Single-threaded version. |
| 22 | 06 | WeightedDirGraphStEphI8 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with i8 weights - Single-threaded version. |
| 23 | 06 | WeightedDirGraphStEphIsize | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with isize weights - Single-threaded version. |
| 24 | 06 | WeightedDirGraphStEphU128 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with u128 weights - Single-threaded version. |
| 25 | 06 | WeightedDirGraphStEphU16 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with u16 weights - Single-threaded version. |
| 26 | 06 | WeightedDirGraphStEphU32 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with u32 weights - Single-threaded version. |
| 27 | 06 | WeightedDirGraphStEphU64 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with u64 weights - Single-threaded version. |
| 28 | 06 | WeightedDirGraphStEphU8 | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with u8 weights - Single-threaded version. |
| 29 | 06 | WeightedDirGraphStEphUsize | No | Eph | Chapter 6 Weighted Directed Graph (ephemeral) with usize weights - Single-threaded version. |
| 30 | 11 | FibonacciMtEph2Threads | Yes | Eph | Chapter 11 — Parallel Fibonacci using Tokenized State Machine. |
| 31 | 11 | FibonacciMtEphRecomputes | Yes | Eph | Chapter 11 — Parallel Fibonacci with TSM at every recursive level. |
| 32 | 11 | FibonacciMtPerAllThreads | Yes | Per | Chapter 11 — Parallel Fibonacci (multi-threaded, persistent). |
| 33 | 11 | FibonacciMtPerTSM | Yes | Per | Chapter 11 — Parallel Fibonacci with TSM (Tokenized State Machine). |
| 34 | 11 | FibonacciStEph | No | Eph | Chapter 11 — Fibonacci. |
| 35 | 12 | Exercise12_1 | — | — | Chapter 12 — Exercise 12.1: spin-lock via fetch-and-add tickets. |
| 36 | 12 | Exercise12_2 | — | — | Chapter 12 — Exercise 12.2: implement fetch-and-add using compare-and-swap. |
| 37 | 12 | Exercise12_5 | — | — | Chapter 12 — Exercise 12.5: lock-free concurrent stack using compare-and-swap. |
| 38 | 17 | MathSeq | — | — | Mathematical sequence backed by a growable vector. Dense domain 0..len-1. |
| 39 | 18 | ArraySeqMtEph | Yes | Eph | Chapter 18 algorithms for ArraySeqMtEph multithreaded ephemeral. Verusified. |
| 40 | 18 | ArraySeqMtEphSlice | Yes | Eph | Chapter 18 slice-backed array sequence (multi-threaded ephemeral). |
| 41 | 18 | ArraySeqMtPer | Yes | Per | Chapter 18 algorithms for ArraySeqMtPer multithreaded persistent. Verusified. |
| 42 | 18 | ArraySeq | — | — | The simplest possible version, ignoring parallelism. Verusified. |
| 43 | 18 | ArraySeqSpecsAndLemmas | — | — | Shared spec functions and proof lemmas for the Chapter 18 ArraySeq and LinkedList modules. |
| 44 | 18 | ArraySeqStEph | No | Eph | Single-threaded ephemeral array sequence (mutable) implementation. Verusified. |
| 45 | 18 | ArraySeqStPer | No | Per | Chapter 18 persistent sequence implementation for array-backed sequences. Verusified. |
| 46 | 18 | LinkedListStEph | No | Eph | Chapter 18 algorithms for LinkedListStEph (ephemeral). Verusified using Vec internally. |
| 47 | 18 | LinkedListStPer | No | Per | Chapter 18 algorithms for LinkedListStPer. Verusified using Vec internally. |
| 48 | 19 | ArraySeqMtEph | Yes | Eph | Chapter 19 parametric sequence implementation for array-backed sequences (multi-threaded ephemeral). Verusified. |
| 49 | 19 | ArraySeqMtEphSlice | Yes | Eph | Chapter 19 slice-backed array sequence (multi-threaded ephemeral). |
| 50 | 19 | ArraySeqSpecsAndLemmas | — | — | Shared spec functions and proof lemmas for the Chapter 19 ArraySeq modules. |
| 51 | 19 | ArraySeqStEph | No | Eph | Chapter 19 parametric sequence implementation for array-backed sequences (ephemeral). Verusified. |
| 52 | 19 | ArraySeqStPer | No | Per | Chapter 19 parametric sequence implementation for array-backed sequences. Verusified. |
| 53 | 21 | Algorithm21_1 | — | — | Chapter 21 — Algorithm 21.1: 2D Points using ArraySeqPer via tabulate + flatten. |
| 54 | 21 | Algorithm21_2 | — | — | Chapter 21 — Algorithm 21.2: 3D Points using ArraySeqPer via flatten of nested tabulates. |
| 55 | 21 | Algorithm21_5 | — | — | Chapter 21 — Algorithm 21.5: Brute Force Solution to the Primes Problem. |
| 56 | 21 | Algorithm21_6 | — | — | Chapter 21 — Algorithm 21.6: Prime Sieve using ArraySeqPer and ninject. |
| 57 | 21 | Exercise21_5 | — | — | Chapter 21 — Exercise 21.5: All contiguous subsequences. |
| 58 | 21 | Exercise21_6 | — | — | Chapter 21 — Exercise 21.6: Cost analysis of all contiguous subsequences. |
| 59 | 21 | Exercise21_7 | — | — | Chapter 21 — Exercise 21.7: Comprehension with Conditionals - even elements paired with vowels. |
| 60 | 21 | Exercise21_8 | — | — | Chapter 21 — Exercise 21.8 / Algorithm 21.4: Brute Force Primality Test (isPrime). |
| 61 | 21 | Exercise21_9 | — | — | Chapter 21 — Exercise 21.9: Composite generation proof. |
| 62 | 23 | BalBinTreeStEph | No | Eph | Ephemeral balanced binary tree utilities (Chapter 23). Verusified. |
| 63 | 23 | PrimTreeSeqStPer | No | Per | Primitive tree sequence implementation for Chapter 23. Verusified. |
| 64 | 26 | DivConReduceMtPer | Yes | Per | Divide-and-conquer via reduce pattern - parallel implementation (Chapter 26, Section 5). |
| 65 | 26 | DivConReduceStPer | No | Per | Divide-and-conquer via reduce pattern - sequential implementation (Chapter 26, Section 5). |
| 66 | 26 | ETSPMtEph | Yes | Eph | Divide-and-conquer Euclidean Traveling Salesperson heuristic — parallel (Chapter 26, Section 4). |
| 67 | 26 | ETSPStEph | No | Eph | Divide-and-conquer Euclidean Traveling Salesperson heuristic (Chapter 26, Section 4). |
| 68 | 26 | MergeSortMtPer | Yes | Per | Parallel merge sort implementation (Chapter 26). |
| 69 | 26 | MergeSortStPer | No | Per | Sequential merge sort implementation (Chapter 26). |
| 70 | 26 | ScanDCMtPer | Yes | Per | Divide-and-conquer scan - parallel implementation (Chapter 26, Section 3). |
| 71 | 26 | ScanDCStPer | No | Per | Divide-and-conquer scan - sequential implementation (Chapter 26, Section 3). |
| 72 | 27 | ContractSpecsAndLemmas | — | — | Shared contraction lemmas for Reduce and Scan algorithms (Chapter 27). |
| 73 | 27 | ReduceContractMtEph | Yes | Eph | Parallel reduce using contraction technique (Chapter 27, Algorithm 27.2). |
| 74 | 27 | ReduceContractStEph | No | Eph | Sequential reduce using contraction technique (Chapter 27, Algorithm 27.2). |
| 75 | 27 | ScanContractMtEph | Yes | Eph | Parallel scan using contraction technique (Chapter 27, Algorithm 27.3). |
| 76 | 27 | ScanContractStEph | No | Eph | Sequential scan using contraction technique (Chapter 27, Algorithm 27.3). |
| 77 | 28 | MaxContigSubSumBruteStEph | No | Eph | Maximum Contiguous Subsequence Sum — Brute Force (Chapter 28, Algorithm 28.8). |
| 78 | 28 | MaxContigSubSumDivConMtEph | Yes | Eph | Maximum Contiguous Subsequence Sum — Parallel Divide and Conquer (Chapter 28, Algorithm 28.17). |
| 79 | 28 | MaxContigSubSumDivConOptMtEph | Yes | Eph | Maximum Contiguous Subsequence Sum — Parallel Strengthened D&C (Chapter 28, Algorithm 28.19). |
| 80 | 28 | MaxContigSubSumDivConOptStEph | No | Eph | Maximum Contiguous Subsequence Sum — Strengthened Divide and Conquer (Chapter 28, Algorithm 28.19). |
| 81 | 28 | MaxContigSubSumDivConStEph | No | Eph | Maximum Contiguous Subsequence Sum — Divide and Conquer (Chapter 28, Algorithm 28.17). |
| 82 | 28 | MaxContigSubSumIterStEph | No | Eph | Maximum Contiguous Subsequence Sum — Kadane's Iterative (Chapter 28, Algorithm 28.15). |
| 83 | 28 | MaxContigSubSumOptMtEph | Yes | Eph | Maximum Contiguous Subsequence Sum — Parallel Optimal (Chapter 28, Algorithm 28.16). |
| 84 | 28 | MaxContigSubSumOptStEph | No | Eph | Maximum Contiguous Subsequence Sum — Work Optimal (Chapter 28, Algorithm 28.16). |
| 85 | 28 | MaxContigSubSumReducedMcsseStEph | No | Eph | Maximum Contiguous Subsequence Sum — Reduction to MCSSE (Chapter 28, Algorithm 28.14). |
| 86 | 28 | MaxContigSubSumReducedStEph | No | Eph | Maximum Contiguous Subsequence Sum — Reduced Force (Chapter 28, Algorithm 28.13). |
| 87 | 28 | MCSSSpec | — | — | Shared specifications for Maximum Contiguous Subsequence Sum (Chapter 28). |
| 88 | 30 | Probability | — | — | Chapter 30: Probability wrapper type. |
| 89 | 35 | OrderStatSelectMtEph | Yes | Eph | Order Statistics - Parallel Ephemeral (Chapter 35, Algorithm 35.2). |
| 90 | 35 | OrderStatSelectMtPer | Yes | Per | Order Statistics - Parallel Persistent (Chapter 35, Algorithm 35.2). |
| 91 | 35 | OrderStatSelectStEph | No | Eph | Order Statistics - Sequential Ephemeral (Chapter 35, Algorithm 35.2). |
| 92 | 35 | OrderStatSelectStPer | No | Per | Order Statistics - Sequential Persistent (Chapter 35, Algorithm 35.2). |
| 93 | 36 | QuickSortMtEph | Yes | Eph | Chapter 36 (Multi-threaded): Quicksort over `ArraySeqMtEph`. |
| 94 | 36 | QuickSortMtEphSlice | Yes | Eph | Chapter 36 (Multi-threaded Slice): Quicksort over `ArraySeqMtEphSlice`. |
| 95 | 36 | QuickSortStEph | No | Eph | Chapter 36 (Single-threaded): Quicksort over `ArraySeqStEph`. |
| 96 | 37 | AVLTreeSeqMtPer | Yes | Per | MtPer (immutable, thread-safe, structurally shared) AVL tree sequence using Arc path-copying. |
| 97 | 37 | AVLTreeSeq | — | — | Implicit-order AVL tree providing O(lg(n)) nth and set by maintaining subtree sizes. |
| 98 | 37 | AVLTreeSeqStEph | No | Eph | StEphemeral (mutable) implicit-order AVL tree sequence. |
| 99 | 37 | AVLTreeSeqStPer | No | Per | StPer (immutable, structurally shared) AVL tree sequence using Arc path-copying. |
| 100 | 37 | BSTAVLMtEph | Yes | Eph | Ephemeral AVL-balanced binary search tree with coarse RwLock for multi-threaded access. |
| 101 | 37 | BSTAVLStEph | No | Eph | Ephemeral AVL-balanced binary search tree. |
| 102 | 37 | BSTBBAlphaMtEph | Yes | Eph | Ephemeral weight-balanced (BB[α]) binary search tree with coarse RwLock for multi-threaded access. |
| 103 | 37 | BSTBBAlphaStEph | No | Eph | Ephemeral weight-balanced (BB[α]) binary search tree. |
| 104 | 37 | BSTPlainMtEph | Yes | Eph | Ephemeral binary search tree with coarse RwLock for multi-threaded access. |
| 105 | 37 | BSTPlainStEph | No | Eph | Ephemeral binary search tree built on `BBTEph` primitives. |
| 106 | 37 | BSTRBMtEph | Yes | Eph | Ephemeral Red-Black balanced binary search tree with coarse RwLock for multi-threaded access. |
| 107 | 37 | BSTRBStEph | No | Eph | Ephemeral Red-Black balanced binary search tree. |
| 108 | 37 | BSTSetAVLMtEph | Yes | Eph | Set interface built atop the AVL multi-threaded BST implementation. |
| 109 | 37 | BSTSetBBAlphaMtEph | Yes | Eph | Set interface built atop the BB-Alpha multi-threaded BST implementation. |
| 110 | 37 | BSTSetPlainMtEph | Yes | Eph | Set interface built atop the Plain multi-threaded BST implementation. |
| 111 | 37 | BSTSetRBMtEph | Yes | Eph | Set interface built atop the Red-Black multi-threaded BST implementation. |
| 112 | 37 | BSTSetSplayMtEph | Yes | Eph | Set interface built atop the Splay multi-threaded BST implementation. |
| 113 | 37 | BSTSpecsAndLemmas | — | — | Shared proof lemmas for the BST modules in Chap37. |
| 114 | 37 | BSTSplayMtEph | Yes | Eph | Ephemeral splay-style binary search tree with coarse RwLock for multi-threaded access. |
| 115 | 37 | BSTSplayStEph | No | Eph | Ephemeral Splay Tree (standard BST semantics) with public methods. |
| 116 | 38 | BSTParaMtEph | Yes | Eph | Parametric multi-threaded BST built around a joinMid interface. |
| 117 | 38 | BSTParaSpecsAndLemmas | — | — | Shared spec functions and proof lemmas for the parametric BST modules in Chap38. |
| 118 | 38 | BSTParaStEph | No | Eph | Parametric single-threaded BST built around a joinMid interface. |
| 119 | 39 | BSTParaTreapMtEph | Yes | Eph | Parametric multi-threaded Treap (probabilistically balanced BST) with parallel operations. |
| 120 | 39 | BSTSetTreapMtEph | Yes | Eph | Set interface as a thin shim over BSTParaTreapMtEph. |
| 121 | 39 | BSTTreapMtEph | Yes | Eph | Ephemeral Treap (randomized heap-ordered BST) with interior locking for multi-threaded access. |
| 122 | 39 | BSTTreapSpecsAndLemmas | — | — | Shared spec functions and proof lemmas for the BST Treap modules (StEph, MtEph, ParaMtEph). |
| 123 | 39 | BSTTreapStEph | No | Eph | Ephemeral Treap (randomized heap-ordered BST) with full parametric BST interface. |
| 124 | 40 | BSTKeyValueStEph | No | Eph | Key-Value BST (dictionary/table) with ephemeral treap structure. |
| 125 | 40 | BSTReducedStEph | No | Eph | BST with general reduced values augmentation using associative functions. |
| 126 | 40 | BSTSizeStEph | No | Eph | Size-augmented BST with O(1) size queries and rank/select operations. |
| 127 | 41 | ArraySetEnumMtEph | Yes | Eph | Multi-threaded ephemeral enumerated set using bit array. |
| 128 | 41 | ArraySetStEph | No | Eph | Single-threaded ephemeral set implementation using ArraySeqStEph as backing store. |
| 129 | 41 | AVLTreeSetMtEph | Yes | Eph | Multi-threaded ephemeral set backed by BSTParaMtEph for parallel set operations. |
| 130 | 41 | AVLTreeSetMtPer | Yes | Per | Multi-threaded persistent set backed by BSTParaMtEph for parallel set operations. |
| 131 | 41 | AVLTreeSetStEph | No | Eph | Single-threaded ephemeral set implementation using BSTParaStEph (Ch38 parametric BST) |
| 132 | 41 | AVLTreeSetStPer | No | Per | Single-threaded persistent set implementation using BSTParaStEph (Ch38 parametric BST) |
| 133 | 41 | OrdKeyMap | — | — | Ordered key-value map backed by ParamBST<Pair<K,V>> with View = Map<K::V, V::V>. |
| 134 | 42 | TableMtEph | Yes | Eph | Chapter 42 multi-threaded ephemeral table implementation using ArraySeqMtEph as backing store. |
| 135 | 42 | TableSpecsAndLemmas | — | — | Shared spec functions and proof lemmas for the Table modules (StEph, StPer, MtEph). |
| 136 | 42 | TableStEph | No | Eph | Chapter 42 single-threaded ephemeral table implementation using ArraySeq as backing store. |
| 137 | 42 | TableStPer | No | Per | Chapter 42 single-threaded persistent table implementation using ArraySeq as backing store. |
| 138 | 43 | AugOrderedTableMtEph | Yes | Eph | Multi-threaded ephemeral reducer-augmented ordered table implementation. |
| 139 | 43 | AugOrderedTableStEph | No | Eph | Single-threaded ephemeral reducer-augmented ordered table implementation. |
| 140 | 43 | AugOrderedTableStPer | No | Per | Single-threaded persistent reducer-augmented ordered table implementation. |
| 141 | 43 | OrderedSetMtEph | Yes | Eph | Multi-threaded ephemeral ordered set using coarse RwLock over OrderedSetStEph. |
| 142 | 43 | OrderedSetStEph | No | Eph | Single-threaded ephemeral ordered set implementation extending AVLTreeSetStEph. |
| 143 | 43 | OrderedSetStPer | No | Per | Single-threaded persistent ordered set implementation extending AVLTreeSetStPer. |
| 144 | 43 | OrderedSpecsAndLemmas | — | — | Shared spec functions and proof lemmas for the Chapter 43 OrderedTable modules. |
| 145 | 43 | OrderedTableMtEph | Yes | Eph | Multi-threaded ephemeral ordered table using coarse RwLock over OrderedTableStEph. |
| 146 | 43 | OrderedTableMtPer | Yes | Per | Multi-threaded persistent ordered table using coarse RwLock over OrderedTableStPer. |
| 147 | 43 | OrderedTableStEph | No | Eph | Single-threaded ephemeral ordered table backed by ParamBST<Pair<K,V>>. |
| 148 | 43 | OrderedTableStPer | No | Per | Single-threaded persistent ordered table backed by ParamBST<Pair<K,V>>. |
| 149 | 44 | DocumentIndex | — | — | Chapter 44: Document Indexing and Searching implementation. |
| 150 | 45 | BalancedTreePQ | — | — | Chapter 45: Priority Queue implementation using Balanced Trees (AVL Tree). |
| 151 | 45 | BinaryHeapPQ | — | — | Chapter 45: Priority Queue implementation using Binary Heap |
| 152 | 45 | HeapsortExample | — | — | Chapter 45: Heapsort Example - Algorithm 45.2 using all Priority Queue implementations |
| 153 | 45 | LeftistHeapPQ | — | — | Chapter 45: Priority Queue implementation using Leftist Heap (Data Structure 45.3) |
| 154 | 45 | SortedListPQ | — | — | Chapter 45: Priority Queue implementation using Sorted List |
| 155 | 45 | UnsortedListPQ | — | — | Chapter 45: Priority Queue implementation using Unsorted List |
| 156 | 47 | ChainedHashTable | — | — | Chained Hash Table - Sequential Ephemeral (Chapter 47). |
| 157 | 47 | DoubleHashFlatHashTableStEph | No | Eph | Double Hashing Flat Hash Table - Sequential Ephemeral (Chapter 47). |
| 158 | 47 | FlatHashTable | — | — | Flat Hash Table - Sequential Ephemeral (Chapter 47). |
| 159 | 47 | LinkedListChainedHashTableStEph | No | Eph | LinkedList Chained Hash Table - Sequential Ephemeral (Chapter 47). |
| 160 | 47 | LinProbFlatHashTableStEph | No | Eph | Linear Probing Flat Hash Table - Sequential Ephemeral (Chapter 47). |
| 161 | 47 | ParaHashTableStEph | No | Eph | Parametric Nested Hash Table - Sequential Ephemeral (Chapter 47, Section 1.1). |
| 162 | 47 | QuadProbFlatHashTableStEph | No | Eph | Quadratic Probing Flat Hash Table - Sequential Ephemeral (Chapter 47). |
| 163 | 47 | StructChainedHashTable | — | — | Struct Chained Hash Table - Sequential Ephemeral (Chapter 47). |
| 164 | 47 | VecChainedHashTableStEph | No | Eph | Vec Chained Hash Table - Sequential Ephemeral (Chapter 47). |
| 165 | 49 | MinEditDistMtEph | Yes | Eph | Chapter 49: Minimum Edit Distance - ephemeral, multi-threaded. |
| 166 | 49 | MinEditDistMtPer | Yes | Per | Chapter 49: Minimum Edit Distance - persistent, multi-threaded. |
| 167 | 49 | MinEditDistStEph | No | Eph | Chapter 49: Minimum Edit Distance - ephemeral, single-threaded. |
| 168 | 49 | MinEditDistStPer | No | Per | Chapter 49: Minimum Edit Distance - persistent, single-threaded. |
| 169 | 49 | SubsetSumMtEph | Yes | Eph | Chapter 49: Subset Sum - ephemeral, multi-threaded. |
| 170 | 49 | SubsetSumMtPer | Yes | Per | Chapter 49: Subset Sum - persistent, multi-threaded. |
| 171 | 49 | SubsetSumStEph | No | Eph | Chapter 49: Subset Sum - ephemeral, single-threaded. |
| 172 | 49 | SubsetSumStPer | No | Per | Chapter 49: Subset Sum - persistent, single-threaded. |
| 173 | 50 | MatrixChainMtEph | Yes | Eph | Chapter 50: Matrix Chain Multiplication - ephemeral, multi-threaded. |
| 174 | 50 | MatrixChainMtPer | Yes | Per | Chapter 50: Matrix Chain Multiplication - persistent, multi-threaded. |
| 175 | 50 | MatrixChainStEph | No | Eph | Chapter 50: Matrix Chain Multiplication - ephemeral, single-threaded. |
| 176 | 50 | MatrixChainStPer | No | Per | Chapter 50: Matrix Chain Multiplication - persistent, single-threaded. |
| 177 | 50 | OptBinSearchTreeMtEph | Yes | Eph | Chapter 50: Optimal Binary Search Tree - ephemeral, multi-threaded. |
| 178 | 50 | OptBinSearchTreeMtPer | Yes | Per | Chapter 50: Optimal Binary Search Tree - persistent, multi-threaded. |
| 179 | 50 | OptBinSearchTreeStEph | No | Eph | Chapter 50: Optimal Binary Search Tree - ephemeral, single-threaded. |
| 180 | 50 | OptBinSearchTreeStPer | No | Per | Chapter 50: Optimal Binary Search Tree - persistent, single-threaded. |
| 181 | 51 | BottomUpDPMtEph | Yes | Eph | Bottom-Up Dynamic Programming - Ephemeral Multi-Threaded Implementation |
| 182 | 51 | BottomUpDPMtPer | Yes | Per | Bottom-Up Dynamic Programming - Persistent Multi-Threaded Implementation |
| 183 | 51 | BottomUpDPStEph | No | Eph | Bottom-Up Dynamic Programming - Ephemeral Single-Threaded Implementation |
| 184 | 51 | BottomUpDPStPer | No | Per | Bottom-Up Dynamic Programming - Persistent Single-Threaded Implementation |
| 185 | 51 | SeqSpecsAndLemmas | — | — | Shared spec functions and proof lemmas for dynamic programming algorithms (Chapter 51). |
| 186 | 51 | TopDownDPMtEph | Yes | Eph | Top-Down Dynamic Programming - Ephemeral Multi-Threaded Implementation |
| 187 | 51 | TopDownDPMtPer | Yes | Per | Top-Down Dynamic Programming - Persistent Multi-Threaded Implementation |
| 188 | 51 | TopDownDPStEph | No | Eph | Top-Down Dynamic Programming - Ephemeral Single-Threaded Implementation |
| 189 | 51 | TopDownDPStPer | No | Per | Top-Down Dynamic Programming - Persistent Single-Threaded Implementation. |
| 190 | 52 | AdjMatrixGraphMtEph | Yes | Eph | Chapter 52: Adjacency Matrix Graph (ephemeral, multi-threaded). |
| 191 | 52 | AdjMatrixGraphMtPer | Yes | Per | Chapter 52: Adjacency Matrix Graph (persistent, multi-threaded). |
| 192 | 52 | AdjMatrixGraphStEph | No | Eph | Chapter 52: Adjacency Matrix Graph (ephemeral, single-threaded). |
| 193 | 52 | AdjMatrixGraphStPer | No | Per | Chapter 52: Adjacency Matrix Graph (persistent, single-threaded). |
| 194 | 52 | AdjSeqGraphMtEph | Yes | Eph | Chapter 52: Adjacency Sequence Graph (ephemeral, multi-threaded). |
| 195 | 52 | AdjSeqGraphMtPer | Yes | Per | Chapter 52: Adjacency Sequence Graph (persistent, multi-threaded). |
| 196 | 52 | AdjSeqGraphStEph | No | Eph | Chapter 52: Adjacency Sequence Graph (ephemeral, single-threaded). |
| 197 | 52 | AdjSeqGraphStPer | No | Per | Chapter 52: Adjacency Sequence Graph (persistent, single-threaded). |
| 198 | 52 | AdjTableGraphMtPer | Yes | Per | Chapter 52: Adjacency Table Graph representation (persistent, multi-threaded with TRUE parallelism). |
| 199 | 52 | AdjTableGraphSpecsAndLemmas | — | — | Shared spec functions and proof lemmas for the graph modules in Chap52. |
| 200 | 52 | AdjTableGraphStEph | No | Eph | (no description) |
| 201 | 52 | AdjTableGraphStPer | No | Per | Chapter 52: Adjacency Table Graph representation (persistent, single-threaded). |
| 202 | 52 | EdgeSetGraphMtEph | Yes | Eph | Chapter 52: Edge Set Graph representation (ephemeral, multi-threaded). |
| 203 | 52 | EdgeSetGraphMtPer | Yes | Per | Chapter 52: Edge Set Graph representation (persistent, multi-threaded with TRUE parallelism). |
| 204 | 52 | EdgeSetGraphStEph | No | Eph | Chapter 52: Edge Set Graph representation (ephemeral, single-threaded). |
| 205 | 52 | EdgeSetGraphStPer | No | Per | Chapter 52: Edge Set Graph representation (persistent, single-threaded). |
| 206 | 53 | GraphSearchMtPer | Yes | Per | Chapter 53: Generic Graph Search (persistent, multi-threaded). |
| 207 | 53 | GraphSearchStEph | No | Eph | Chapter 53: Generic Graph Search (ephemeral, single-threaded). |
| 208 | 53 | GraphSearchStPer | No | Per | Chapter 53: Generic Graph Search (persistent, single-threaded). |
| 209 | 53 | PQMinStEph | No | Eph | Chapter 53: Min-Priority Queue Search - ephemeral, single-threaded. |
| 210 | 53 | PQMinStPer | No | Per | Chapter 53: Min-Priority Queue Search - persistent, single-threaded. |
| 211 | 54 | BFSMtEph | Yes | Eph | Breadth-First Search - Parallel Ephemeral (Chapter 54, Algorithms 54.5 and 54.6). |
| 212 | 54 | BFSMtPer | Yes | Per | Breadth-First Search - Parallel Persistent (Chapter 54, Algorithms 54.5 and 54.6). |
| 213 | 54 | BFSSpecsAndLemmas | — | — | Shared BFS specifications and lemmas over abstract Seq types. |
| 214 | 54 | BFSStEph | No | Eph | Breadth-First Search - Sequential Ephemeral (Chapter 54, Algorithms 54.5 and 54.6). |
| 215 | 54 | BFSStPer | No | Per | Breadth-First Search - Sequential Persistent (Chapter 54, Algorithms 54.5 and 54.6). |
| 216 | 55 | CycleDetectStEph | No | Eph | Cycle Detection - Sequential Ephemeral (Chapter 55, Algorithm 55.10). |
| 217 | 55 | CycleDetectStPer | No | Per | Cycle Detection - Sequential Persistent (Chapter 55, Algorithm 55.10). |
| 218 | 55 | DFSSpecsAndLemmas | — | — | Shared DFS specs and lemmas used by all Chap55 files. |
| 219 | 55 | DFSStEph | No | Eph | Depth-First Search - Sequential Ephemeral (Chapter 55, Algorithm 55.7). |
| 220 | 55 | DFSStPer | No | Per | Depth-First Search - Sequential Persistent (Chapter 55, Algorithm 55.2). |
| 221 | 55 | SCCStEph | No | Eph | Strongly Connected Components - Sequential Ephemeral (Chapter 55, Algorithm 55.18). |
| 222 | 55 | SCCStPer | No | Per | Strongly Connected Components - Sequential Persistent (Chapter 55, Algorithm 55.18). |
| 223 | 55 | TopoSortStEph | No | Eph | Topological Sort - Sequential Ephemeral (Chapter 55, Algorithm 55.13). |
| 224 | 55 | TopoSortStPer | No | Per | Topological Sort - Sequential Persistent (Chapter 55, Algorithm 55.13). |
| 225 | 56 | AllPairsResultStEphF64 | No | Eph | All-Pairs Shortest Path Result Structure - Sequential Ephemeral (Float Weights) |
| 226 | 56 | AllPairsResultStEphI64 | No | Eph | All-Pairs Shortest Path Result Structure - Sequential Ephemeral (Integer Weights) |
| 227 | 56 | AllPairsResultStPerF64 | No | Per | All-Pairs Shortest Path Result Structure - Sequential Persistent (Float Weights) |
| 228 | 56 | AllPairsResultStPerI64 | No | Per | All-Pairs Shortest Path Result Structure - Sequential Persistent (Integer Weights) |
| 229 | 56 | PathWeightUtilsStEph | No | Eph | Shortest Path Utility Functions - Sequential Ephemeral (Integer Weights) |
| 230 | 56 | PathWeightUtilsStPer | No | Per | Shortest Path Utility Functions - Sequential Persistent |
| 231 | 56 | SSSPResultStEphF64 | No | Eph | Single-Source Shortest Path Result Structure - Sequential Ephemeral (Float Weights) |
| 232 | 56 | SSSPResultStEphI64 | No | Eph | Single-Source Shortest Path Result Structure - Sequential Ephemeral (Integer Weights) |
| 233 | 56 | SSSPResultStPerF64 | No | Per | Single-Source Shortest Path Result Structure - Sequential Persistent (Float Weights) |
| 234 | 56 | SSSPResultStPerI64 | No | Per | Single-Source Shortest Path Result Structure - Sequential Persistent (Integer Weights) |
| 235 | 57 | DijkstraStEphF64 | No | Eph | Dijkstra's Algorithm - Single Source Shortest Path (SSSP+) for non-negative float edge weights |
| 236 | 57 | DijkstraStEphU64 | No | Eph | Dijkstra's Algorithm - Single Source Shortest Path (SSSP+) for non-negative integer edge weights |
| 237 | 57 | StackStEph | No | Eph | Stack - Sequential Ephemeral implementation |
| 238 | 58 | BellmanFordStEphF64 | No | Eph | Bellman-Ford's Algorithm - Single Source Shortest Path with arbitrary edge weights (float) |
| 239 | 58 | BellmanFordStEphI64 | No | Eph | Bellman-Ford's Algorithm - Single Source Shortest Path with arbitrary edge weights (integer) |
| 240 | 59 | JohnsonMtEphF64 | Yes | Eph | Chapter 59: Johnson's Algorithm - Multi-threaded Ephemeral Float Weights |
| 241 | 59 | JohnsonMtEphI64 | Yes | Eph | Chapter 59: Johnson's Algorithm - Multi-threaded Ephemeral Integer Weights |
| 242 | 59 | JohnsonStEphF64 | No | Eph | Chapter 59: Johnson's Algorithm - Single-threaded Ephemeral Float Weights |
| 243 | 59 | JohnsonStEphI64 | No | Eph | Chapter 59: Johnson's Algorithm - Single-threaded Ephemeral Integer Weights |
| 244 | 61 | EdgeContractionMtEph | Yes | Eph | Chapter 61: Edge Contraction - Multi-threaded Ephemeral Implementation |
| 245 | 61 | EdgeContractionStEph | No | Eph | Chapter 61: Edge Contraction - Sequential Ephemeral Implementation |
| 246 | 61 | VertexMatchingMtEph | Yes | Eph | Chapter 61: Vertex Matching - Multi-threaded Ephemeral Implementation |
| 247 | 61 | VertexMatchingStEph | No | Eph | Chapter 61: Vertex Matching - Sequential Ephemeral Implementation |
| 248 | 62 | StarContractionMtEph | Yes | Eph | Chapter 62: Star Contraction - Multi-threaded Ephemeral Implementation |
| 249 | 62 | StarContractionStEph | No | Eph | Chapter 62: Star Contraction - Sequential Ephemeral Implementation |
| 250 | 62 | StarPartitionMtEph | Yes | Eph | Chapter 62: Star Partition - Multi-threaded Ephemeral Implementation |
| 251 | 62 | StarPartitionStEph | No | Eph | Chapter 62: Star Partition - Sequential Ephemeral Implementation |
| 252 | 63 | ConnectivityMtEph | Yes | Eph | Chapter 63: Graph Connectivity - Multi-threaded Ephemeral Implementation |
| 253 | 63 | ConnectivityStEph | No | Eph | Chapter 63: Graph Connectivity - Sequential Ephemeral Implementation |
| 254 | 64 | SpanTreeMtEph | Yes | Eph | Chapter 64: Minimum Spanning Trees - Spanning Tree via Star Contraction (Parallel) |
| 255 | 64 | SpanTreeStEph | No | Eph | Chapter 64: Minimum Spanning Trees - Spanning Tree via Star Contraction (Sequential) |
| 256 | 64 | TSPApproxStEph | No | Eph | Chapter 64: TSP 2-Approximation via MST (Sequential) |
| 257 | 65 | KruskalStEph | No | Eph | Chapter 65: Kruskal's MST Algorithm (Sequential Ephemeral) |
| 258 | 65 | PrimStEph | No | Eph | Chapter 65: Prim's MST Algorithm (Sequential Ephemeral) |
| 259 | 65 | UnionFindArrayStEph | No | Eph | Union-Find (Disjoint Set Union) — Array-based, Sequential Ephemeral. |
| 260 | 65 | UnionFindNoPCStEph | No | Eph | Union-Find (Disjoint Set Union) — HashMap-based, Sequential Ephemeral, no path compression. |
| 261 | 65 | UnionFindPCStEph | No | Eph | Union-Find with Path Compression — HashMap-based, Sequential Ephemeral. |
| 262 | 66 | BoruvkaMtEph | Yes | Eph | Chapter 66: Borůvka's MST Algorithm (Parallel Ephemeral) |
| 263 | 66 | BoruvkaStEph | No | Eph | Chapter 66: Borůvka's MST Algorithm (Sequential Ephemeral) |
