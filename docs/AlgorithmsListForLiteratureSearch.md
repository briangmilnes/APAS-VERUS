# Algorithms List for Literature Search

Generated 2026-04-12 from `main`. Covers all `src/Chap*/` files
(excluding `Example*.rs` and `Problem*.rs` per CLAUDE.md).

Intended as input for literature research: one row per distinct
algorithm × variant combination, with a description suitable as a
search prompt for an AI researcher or a human reviewer looking up
prior formal verification / publication.

**Columns:**
- **#** — row number (stable for referencing).
- **Chap** — APAS chapter number (from *A Practical Approach to Data
  Structures*, Acar–Blelloch).
- **Algorithm** — algorithm / data-structure base name, with the
  variant suffixes (`StEph`, `StPer`, `MtEph`, `MtPer`) stripped.
- **Mt?** — `Yes` if the variant is multi-threaded (file name
  contains `Mt`), `No` if sequential, `-` if the file is foundational
  (no variant axis).
- **Per/Eph** — `Per` if persistent (functional / copy-on-write),
  `Eph` if ephemeral (mutable), `-` if not applicable.
- **File** — source file base name (without `.rs`). When multiple
  files differ only in numeric weight type (e.g., F64/I8/.../U128),
  they are collapsed into one row with a `(×N numeric types: …)`
  annotation. Other meaningful suffixes (e.g., `Slice`, `2Threads`,
  `TSM`, `Recomputes`, `AllThreads`) are kept as separate rows with
  a `(variant: …)` annotation.
- **Description** — one-sentence description of the algorithm, kept
  implementation-agnostic and neutral enough to use as a search query.

**Variant naming recap** (APAS-VERUS convention):

- **StEph** = sequential ephemeral (classic mutable data structure).
- **StPer** = sequential persistent (functional / copy-on-write).
- **MtEph** = multi-threaded ephemeral (thread-safe mutable, usually
  coarse-locked over an RwLock).
- **MtPer** = multi-threaded persistent (thread-safe persistent,
  usually Arc-shared).

Raw total: 263 source files. Collapsed into 243 rows by merging
pure-numeric-type variant families.

| # | Chap | Algorithm | Mt? | Per/Eph | File | Description |
|---|------|-----------|-----|---------|------|-------------|
| 1 | 02 | FibonacciHFScheduler | - | - | FibonacciHFScheduler | Fibonacci computed via the Help-First work-stealing scheduler; demonstrates fork-join parallelism with a bounded global thread pool. |
| 2 | 02 | HFScheduler | Yes | Eph | HFSchedulerMtEph | Help-First (HF) work-stealing scheduler for fork-join parallelism with a bounded global pool; Blelloch-style scheduling primitive. |
| 3 | 03 | InsertionSort | No | Eph | InsertionSortStEph | Insertion sort on a mutable slice/array; O(n^2) comparison-based stable in-place sort. |
| 4 | 05 | Kleene | No | Per | KleeneStPer | Kleene star/plus closure over a finite alphabet; iterative fixed-point computation for formal language theory. |
| 5 | 05 | Mapping | No | Eph | MappingStEph | Finite mapping (partial function) from keys to values; set-theoretic function abstraction built over relations. |
| 6 | 05 | Relation | No | Eph | RelationStEph | Binary relation as a set of pairs; set-theoretic abstraction supporting composition, restriction, domain/range operations. |
| 7 | 05 | Set | Yes | Eph | SetMtEph | Unordered finite set data structure built on hash set; supports insert, contains, union, intersection, difference. |
| 8 | 05 | Set | No | Eph | SetStEph | Unordered finite set data structure built on hash set; supports insert, contains, union, intersection, difference. |
| 9 | 06 | DirGraph | Yes | Eph | DirGraphMtEph | Directed graph represented as (V, A) with arcs as a set of ordered pairs; set-of-arcs representation. |
| 10 | 06 | DirGraph | No | Eph | DirGraphStEph | Directed graph represented as (V, A) with arcs as a set of ordered pairs; set-of-arcs representation. |
| 11 | 06 | LabDirGraph | Yes | Eph | LabDirGraphMtEph | Labeled directed graph; directed arcs carry labels/weights from a finite set. |
| 12 | 06 | LabDirGraph | No | Eph | LabDirGraphStEph | Labeled directed graph; directed arcs carry labels/weights from a finite set. |
| 13 | 06 | LabUnDirGraph | Yes | Eph | LabUnDirGraphMtEph | Labeled undirected graph; edges are unordered pairs carrying labels/weights. |
| 14 | 06 | LabUnDirGraph | No | Eph | LabUnDirGraphStEph | Labeled undirected graph; edges are unordered pairs carrying labels/weights. |
| 15 | 06 | UnDirGraph | Yes | Eph | UnDirGraphMtEph | Undirected graph as a set of unordered edge pairs; symmetric arc representation. |
| 16 | 06 | UnDirGraph | No | Eph | UnDirGraphStEph | Undirected graph as a set of unordered edge pairs; symmetric arc representation. |
| 17 | 06 | WeightedDirGraph | No | Eph | WeightedDirGraphStEphF64 (×13 numeric types: F64,I128,I16,I32,I64,I8,Isize,U128,U16,U32,U64,U8,Usize) | Weighted directed graph parameterized over numeric weight type (F64/I8/I16/I32/I64/I128/Isize/U8/U16/U32/U64/U128/Usize); arcs carry numeric weights for shortest-path and MST algorithms. |
| 18 | 11 | Fibonacci | Yes | Eph | FibonacciMtEph2Threads (variant: 2Threads) | Fibonacci number computation; basic recursion, iteration, and parallel divide-and-conquer variants. |
| 19 | 11 | Fibonacci | Yes | Eph | FibonacciMtEphRecomputes (variant: Recomputes) | Fibonacci number computation; basic recursion, iteration, and parallel divide-and-conquer variants. |
| 20 | 11 | Fibonacci | Yes | Per | FibonacciMtPerAllThreads (variant: AllThreads) | Fibonacci number computation; basic recursion, iteration, and parallel divide-and-conquer variants. |
| 21 | 11 | Fibonacci | Yes | Per | FibonacciMtPerTSM (variant: TSM) | Fibonacci number computation; basic recursion, iteration, and parallel divide-and-conquer variants. |
| 22 | 11 | Fibonacci | No | Eph | FibonacciStEph | Fibonacci number computation; basic recursion, iteration, and parallel divide-and-conquer variants. |
| 23 | 12 | Exercise12_1 | - | - | Exercise12_1 | Chapter 12 exercise on recurrences/asymptotic analysis. |
| 24 | 12 | Exercise12_2 | - | - | Exercise12_2 | Chapter 12 exercise on recurrences/asymptotic analysis. |
| 25 | 12 | Exercise12_5 | - | - | Exercise12_5 | Chapter 12 exercise on recurrences/asymptotic analysis. |
| 26 | 17 | MathSeq | - | - | MathSeq | Mathematical sequence (integer-indexed Seq[T]); foundational abstraction for APAS sequence algorithms. |
| 27 | 18 | ArraySeq | Yes | Eph | ArraySeqMtEph | Array-backed sequence (Vec-based); Chapter 18 reference implementation of the sequence ADT. |
| 28 | 18 | ArraySeq | Yes | Eph | ArraySeqMtEphSlice (variant: Slice) | Array-backed sequence (Vec-based); Chapter 18 reference implementation of the sequence ADT. |
| 29 | 18 | ArraySeq | Yes | Per | ArraySeqMtPer | Array-backed sequence (Vec-based); Chapter 18 reference implementation of the sequence ADT. |
| 30 | 18 | ArraySeq | - | - | ArraySeq | Array-backed sequence (Vec-based); Chapter 18 reference implementation of the sequence ADT. |
| 31 | 18 | ArraySeqSpecsAndLemmas | - | - | ArraySeqSpecsAndLemmas | Shared specs and lemmas for Chapter 18 ArraySeq implementations. |
| 32 | 18 | ArraySeq | No | Eph | ArraySeqStEph | Array-backed sequence (Vec-based); Chapter 18 reference implementation of the sequence ADT. |
| 33 | 18 | ArraySeq | No | Per | ArraySeqStPer | Array-backed sequence (Vec-based); Chapter 18 reference implementation of the sequence ADT. |
| 34 | 18 | LinkedList | No | Eph | LinkedListStEph | Singly-linked-list sequence; Chapter 18 alternative representation contrasting with array sequence cost. |
| 35 | 18 | LinkedList | No | Per | LinkedListStPer | Singly-linked-list sequence; Chapter 18 alternative representation contrasting with array sequence cost. |
| 36 | 19 | ArraySeq | Yes | Eph | ArraySeqMtEph | Chapter 19 array sequence variant (enumerable/bounded case); refines Chapter 18's ArraySeq. |
| 37 | 19 | ArraySeq | Yes | Eph | ArraySeqMtEphSlice (variant: Slice) | Chapter 19 array sequence variant (enumerable/bounded case); refines Chapter 18's ArraySeq. |
| 38 | 19 | ArraySeqSpecsAndLemmas | - | - | ArraySeqSpecsAndLemmas | Shared specs and lemmas for Chapter 19 ArraySeq implementations. |
| 39 | 19 | ArraySeq | No | Eph | ArraySeqStEph | Chapter 19 array sequence variant (enumerable/bounded case); refines Chapter 18's ArraySeq. |
| 40 | 19 | ArraySeq | No | Per | ArraySeqStPer | Chapter 19 array sequence variant (enumerable/bounded case); refines Chapter 18's ArraySeq. |
| 41 | 21 | Algorithm21_1 | - | - | Algorithm21_1 | Chapter 21 Algorithm 21.1; sequence manipulation algorithm from APAS Chapter 21. |
| 42 | 21 | Algorithm21_2 | - | - | Algorithm21_2 | Chapter 21 Algorithm 21.2; sequence manipulation algorithm from APAS Chapter 21. |
| 43 | 21 | Algorithm21_5 | - | - | Algorithm21_5 | Chapter 21 Algorithm 21.5; sequence manipulation algorithm from APAS Chapter 21. |
| 44 | 21 | Algorithm21_6 | - | - | Algorithm21_6 | Chapter 21 Algorithm 21.6; sequence manipulation algorithm from APAS Chapter 21. |
| 45 | 21 | Exercise21_5 | - | - | Exercise21_5 | Chapter 21 exercise 21.5. |
| 46 | 21 | Exercise21_6 | - | - | Exercise21_6 | Chapter 21 exercise 21.6. |
| 47 | 21 | Exercise21_7 | - | - | Exercise21_7 | Chapter 21 exercise 21.7. |
| 48 | 21 | Exercise21_8 | - | - | Exercise21_8 | Chapter 21 exercise 21.8. |
| 49 | 21 | Exercise21_9 | - | - | Exercise21_9 | Chapter 21 exercise 21.9. |
| 50 | 23 | BalBinTree | No | Eph | BalBinTreeStEph | Balanced binary tree; height-balanced tree used as substrate for various sequence representations. |
| 51 | 23 | PrimTreeSeq | No | Per | PrimTreeSeqStPer | Primitive-tree-based sequence; sequence ADT implemented over a primitive (perfectly balanced) binary tree. |
| 52 | 26 | DivConReduce | Yes | Per | DivConReduceMtPer | Divide-and-conquer reduce (fold over a monoid); canonical parallel reduction pattern with O(log n) span. |
| 53 | 26 | DivConReduce | No | Per | DivConReduceStPer | Divide-and-conquer reduce (fold over a monoid); canonical parallel reduction pattern with O(log n) span. |
| 54 | 26 | ETSP | Yes | Eph | ETSPMtEph | Euclidean TSP approximation via divide-and-conquer; 2D Euclidean Traveling Salesman Problem approximation. |
| 55 | 26 | ETSP | No | Eph | ETSPStEph | Euclidean TSP approximation via divide-and-conquer; 2D Euclidean Traveling Salesman Problem approximation. |
| 56 | 26 | MergeSort | Yes | Per | MergeSortMtPer | Merge sort with divide-and-conquer recursion; O(n log n) comparison-based stable sort, Blelloch-style parallel variant. |
| 57 | 26 | MergeSort | No | Per | MergeSortStPer | Merge sort with divide-and-conquer recursion; O(n log n) comparison-based stable sort, Blelloch-style parallel variant. |
| 58 | 26 | ScanDC | Yes | Per | ScanDCMtPer | Scan (prefix sums) via divide-and-conquer; parallel prefix computation over an associative operator. |
| 59 | 26 | ScanDC | No | Per | ScanDCStPer | Scan (prefix sums) via divide-and-conquer; parallel prefix computation over an associative operator. |
| 60 | 27 | ContractSpecsAndLemmas | - | - | ContractSpecsAndLemmas | Shared specs and lemmas for contraction-based parallel algorithms. |
| 61 | 27 | ReduceContract | Yes | Eph | ReduceContractMtEph | Reduce via the contraction pattern; iterative halving of the input with combining step per round. |
| 62 | 27 | ReduceContract | No | Eph | ReduceContractStEph | Reduce via the contraction pattern; iterative halving of the input with combining step per round. |
| 63 | 27 | ScanContract | Yes | Eph | ScanContractMtEph | Scan via the contraction pattern; iterative halving prefix-sum computation. |
| 64 | 27 | ScanContract | No | Eph | ScanContractStEph | Scan via the contraction pattern; iterative halving prefix-sum computation. |
| 65 | 28 | MaxContigSubSumBrute | No | Eph | MaxContigSubSumBruteStEph | Maximum contiguous subsequence sum, brute force O(n^3); Kadane problem baseline. |
| 66 | 28 | MaxContigSubSumDivCon | Yes | Eph | MaxContigSubSumDivConMtEph | Maximum contiguous subsequence sum via divide-and-conquer; O(n log n) recursive combining. |
| 67 | 28 | MaxContigSubSumDivConOpt | Yes | Eph | MaxContigSubSumDivConOptMtEph | Maximum contiguous subsequence sum, optimized divide-and-conquer; single-pass recursive version. |
| 68 | 28 | MaxContigSubSumDivConOpt | No | Eph | MaxContigSubSumDivConOptStEph | Maximum contiguous subsequence sum, optimized divide-and-conquer; single-pass recursive version. |
| 69 | 28 | MaxContigSubSumDivCon | No | Eph | MaxContigSubSumDivConStEph | Maximum contiguous subsequence sum via divide-and-conquer; O(n log n) recursive combining. |
| 70 | 28 | MaxContigSubSumIter | No | Eph | MaxContigSubSumIterStEph | Maximum contiguous subsequence sum, iterative Kadane's algorithm; O(n) sequential. |
| 71 | 28 | MaxContigSubSumOpt | Yes | Eph | MaxContigSubSumOptMtEph | Maximum contiguous subsequence sum, optimized version using running aggregates. |
| 72 | 28 | MaxContigSubSumOpt | No | Eph | MaxContigSubSumOptStEph | Maximum contiguous subsequence sum, optimized version using running aggregates. |
| 73 | 28 | MaxContigSubSumReducedMcsse | No | Eph | MaxContigSubSumReducedMcsseStEph | Maximum contiguous subsequence sum via the MCSSE monoid; parallel reduction with four-component state. |
| 74 | 28 | MaxContigSubSumReduced | No | Eph | MaxContigSubSumReducedStEph | Maximum contiguous subsequence sum via a monoid reduction; parallel O(log n) span using algebraic combining. |
| 75 | 28 | MCSSSpec | - | - | MCSSSpec | Specification for maximum contiguous subsequence sum variants; shared correctness spec. |
| 76 | 30 | Probability | - | - | Probability | Probability abstractions; combinatorial probability utilities for randomized algorithm analysis. |
| 77 | 35 | OrderStatSelect | Yes | Eph | OrderStatSelectMtEph | Order statistic selection; quickselect-style k-th smallest element in linear expected time, with parallel divide-and-conquer variant. |
| 78 | 35 | OrderStatSelect | Yes | Per | OrderStatSelectMtPer | Order statistic selection; quickselect-style k-th smallest element in linear expected time, with parallel divide-and-conquer variant. |
| 79 | 35 | OrderStatSelect | No | Eph | OrderStatSelectStEph | Order statistic selection; quickselect-style k-th smallest element in linear expected time, with parallel divide-and-conquer variant. |
| 80 | 35 | OrderStatSelect | No | Per | OrderStatSelectStPer | Order statistic selection; quickselect-style k-th smallest element in linear expected time, with parallel divide-and-conquer variant. |
| 81 | 36 | QuickSort | Yes | Eph | QuickSortMtEph | Quicksort with three-way partitioning; randomized O(n log n) expected, parallel divide-and-conquer variant. |
| 82 | 36 | QuickSort | Yes | Eph | QuickSortMtEphSlice (variant: Slice) | Quicksort with three-way partitioning; randomized O(n log n) expected, parallel divide-and-conquer variant. |
| 83 | 36 | QuickSort | No | Eph | QuickSortStEph | Quicksort with three-way partitioning; randomized O(n log n) expected, parallel divide-and-conquer variant. |
| 84 | 37 | AVLTreeSeq | Yes | Per | AVLTreeSeqMtPer | Sequence ADT over an AVL-balanced tree; O(log n) insert/delete with sequence semantics. |
| 85 | 37 | AVLTreeSeq | - | - | AVLTreeSeq | Sequence ADT over an AVL-balanced tree; O(log n) insert/delete with sequence semantics. |
| 86 | 37 | AVLTreeSeq | No | Eph | AVLTreeSeqStEph | Sequence ADT over an AVL-balanced tree; O(log n) insert/delete with sequence semantics. |
| 87 | 37 | AVLTreeSeq | No | Per | AVLTreeSeqStPer | Sequence ADT over an AVL-balanced tree; O(log n) insert/delete with sequence semantics. |
| 88 | 37 | BSTAVL | Yes | Eph | BSTAVLMtEph | AVL self-balancing binary search tree; height-balanced BST with O(log n) operations. |
| 89 | 37 | BSTAVL | No | Eph | BSTAVLStEph | AVL self-balancing binary search tree; height-balanced BST with O(log n) operations. |
| 90 | 37 | BSTBBAlpha | Yes | Eph | BSTBBAlphaMtEph | BB[alpha] weight-balanced binary search tree; size-balanced BST with tunable alpha balance factor. |
| 91 | 37 | BSTBBAlpha | No | Eph | BSTBBAlphaStEph | BB[alpha] weight-balanced binary search tree; size-balanced BST with tunable alpha balance factor. |
| 92 | 37 | BSTPlain | Yes | Eph | BSTPlainMtEph | Unbalanced binary search tree; O(h) operations with worst-case O(n) height. |
| 93 | 37 | BSTPlain | No | Eph | BSTPlainStEph | Unbalanced binary search tree; O(h) operations with worst-case O(n) height. |
| 94 | 37 | BSTRB | Yes | Eph | BSTRBMtEph | Red-black self-balancing binary search tree; 2-3-4 tree equivalent with color invariants. |
| 95 | 37 | BSTRB | No | Eph | BSTRBStEph | Red-black self-balancing binary search tree; 2-3-4 tree equivalent with color invariants. |
| 96 | 37 | BSTSetAVL | Yes | Eph | BSTSetAVLMtEph | Set ADT over an AVL-balanced BST; ordered set with O(log n) operations. |
| 97 | 37 | BSTSetBBAlpha | Yes | Eph | BSTSetBBAlphaMtEph | Set ADT over a BB[alpha] weight-balanced BST; ordered set with O(log n) operations. |
| 98 | 37 | BSTSetPlain | Yes | Eph | BSTSetPlainMtEph | Set ADT over an unbalanced BST; O(h) operations. |
| 99 | 37 | BSTSetRB | Yes | Eph | BSTSetRBMtEph | Set ADT over a red-black BST; ordered set with O(log n) operations. |
| 100 | 37 | BSTSetSplay | Yes | Eph | BSTSetSplayMtEph | Set ADT over a splay tree; amortized O(log n) operations with self-adjusting access. |
| 101 | 37 | BSTSpecsAndLemmas | - | - | BSTSpecsAndLemmas | Shared BST specifications and lemmas across balanced/unbalanced variants. |
| 102 | 37 | BSTSplay | Yes | Eph | BSTSplayMtEph | Splay tree; self-adjusting binary search tree with amortized O(log n) operations. |
| 103 | 37 | BSTSplay | No | Eph | BSTSplayStEph | Splay tree; self-adjusting binary search tree with amortized O(log n) operations. |
| 104 | 38 | BSTPara | Yes | Eph | BSTParaMtEph | Parallel BST (Blelloch-style parallel ordered tree); split/union/intersection in O(log^2 n) span. |
| 105 | 38 | BSTParaSpecsAndLemmas | - | - | BSTParaSpecsAndLemmas | Shared specifications and lemmas for parallel BST operations. |
| 106 | 38 | BSTPara | No | Eph | BSTParaStEph | Parallel BST (Blelloch-style parallel ordered tree); split/union/intersection in O(log^2 n) span. |
| 107 | 39 | BSTParaTreap | Yes | Eph | BSTParaTreapMtEph | Treap (randomized search tree) with parallel split/union; randomized balance via priorities. |
| 108 | 39 | BSTSetTreap | Yes | Eph | BSTSetTreapMtEph | Set ADT over a treap; randomized-balance ordered set. |
| 109 | 39 | BSTTreap | Yes | Eph | BSTTreapMtEph | Treap binary search tree; randomized-balance BST with O(log n) expected operations. |
| 110 | 39 | BSTTreapSpecsAndLemmas | - | - | BSTTreapSpecsAndLemmas | Shared specifications and lemmas for treap variants. |
| 111 | 39 | BSTTreap | No | Eph | BSTTreapStEph | Treap binary search tree; randomized-balance BST with O(log n) expected operations. |
| 112 | 40 | BSTKeyValue | No | Eph | BSTKeyValueStEph | Key-value BST; binary search tree mapping keys to values (dictionary ADT). |
| 113 | 40 | BSTReduced | No | Eph | BSTReducedStEph | Reduced BST with an aggregation monoid at each node; supports range reductions. |
| 114 | 40 | BSTSize | No | Eph | BSTSizeStEph | Size-augmented BST; each node stores subtree size, enabling O(log n) rank/select. |
| 115 | 41 | ArraySetEnum | Yes | Eph | ArraySetEnumMtEph | Enumerable array set variant; set over an enumerable element type. |
| 116 | 41 | ArraySet | No | Eph | ArraySetStEph | Array-backed ordered set; sorted-vector set with O(n) insert/delete but O(log n) lookup. |
| 117 | 41 | AVLTreeSet | Yes | Eph | AVLTreeSetMtEph | Set ADT backed by an AVL tree; balanced ordered set. |
| 118 | 41 | AVLTreeSet | Yes | Per | AVLTreeSetMtPer | Set ADT backed by an AVL tree; balanced ordered set. |
| 119 | 41 | AVLTreeSet | No | Eph | AVLTreeSetStEph | Set ADT backed by an AVL tree; balanced ordered set. |
| 120 | 41 | AVLTreeSet | No | Per | AVLTreeSetStPer | Set ADT backed by an AVL tree; balanced ordered set. |
| 121 | 41 | OrdKeyMap | - | - | OrdKeyMap | Ordered key-value map; sorted map ADT. |
| 122 | 42 | Table | Yes | Eph | TableMtEph | Table (associative map) ADT; fundamental APAS key-value collection with parallel operations. |
| 123 | 42 | TableSpecsAndLemmas | - | - | TableSpecsAndLemmas | Shared table specifications and lemmas. |
| 124 | 42 | Table | No | Eph | TableStEph | Table (associative map) ADT; fundamental APAS key-value collection with parallel operations. |
| 125 | 42 | Table | No | Per | TableStPer | Table (associative map) ADT; fundamental APAS key-value collection with parallel operations. |
| 126 | 43 | AugOrderedTable | Yes | Eph | AugOrderedTableMtEph | Augmented ordered table with reduction monoid; ordered key-value map supporting range queries via augmentation. |
| 127 | 43 | AugOrderedTable | No | Eph | AugOrderedTableStEph | Augmented ordered table with reduction monoid; ordered key-value map supporting range queries via augmentation. |
| 128 | 43 | AugOrderedTable | No | Per | AugOrderedTableStPer | Augmented ordered table with reduction monoid; ordered key-value map supporting range queries via augmentation. |
| 129 | 43 | OrderedSet | Yes | Eph | OrderedSetMtEph | Ordered set ADT; sorted-element set with split/rank. |
| 130 | 43 | OrderedSet | No | Eph | OrderedSetStEph | Ordered set ADT; sorted-element set with split/rank. |
| 131 | 43 | OrderedSet | No | Per | OrderedSetStPer | Ordered set ADT; sorted-element set with split/rank. |
| 132 | 43 | OrderedSpecsAndLemmas | - | - | OrderedSpecsAndLemmas | Shared ordered-table/set specifications and lemmas. |
| 133 | 43 | OrderedTable | Yes | Eph | OrderedTableMtEph | Ordered key-value table; sorted key-value map with split/join. |
| 134 | 43 | OrderedTable | Yes | Per | OrderedTableMtPer | Ordered key-value table; sorted key-value map with split/join. |
| 135 | 43 | OrderedTable | No | Eph | OrderedTableStEph | Ordered key-value table; sorted key-value map with split/join. |
| 136 | 43 | OrderedTable | No | Per | OrderedTableStPer | Ordered key-value table; sorted key-value map with split/join. |
| 137 | 44 | DocumentIndex | - | - | DocumentIndex | Document index (inverted index) built over ordered tables; maps terms to document postings. |
| 138 | 45 | BalancedTreePQ | - | - | BalancedTreePQ | Priority queue backed by a balanced tree; O(log n) insert/delete-min. |
| 139 | 45 | BinaryHeapPQ | - | - | BinaryHeapPQ | Binary heap priority queue; array-embedded complete binary tree with O(log n) ops. |
| 140 | 45 | HeapsortExample | - | - | HeapsortExample | Heapsort example built on a priority queue; O(n log n) in-place sort. |
| 141 | 45 | LeftistHeapPQ | - | - | LeftistHeapPQ | Leftist heap (mergeable heap); O(log n) insert/delete-min/merge via recursive merging. |
| 142 | 45 | SortedListPQ | - | - | SortedListPQ | Sorted-list priority queue; O(n) insert, O(1) delete-min. |
| 143 | 45 | UnsortedListPQ | - | - | UnsortedListPQ | Unsorted-list priority queue; O(1) insert, O(n) delete-min. |
| 144 | 47 | ChainedHashTable | - | - | ChainedHashTable | Hash table with separate chaining for collision resolution. |
| 145 | 47 | DoubleHashFlatHashTable | No | Eph | DoubleHashFlatHashTableStEph | Open-addressing hash table with double hashing probe sequence. |
| 146 | 47 | FlatHashTable | - | - | FlatHashTable | Base open-addressing flat hash table; in-place key storage with probing. |
| 147 | 47 | LinkedListChainedHashTable | No | Eph | LinkedListChainedHashTableStEph | Chained hash table using linked lists for collision buckets. |
| 148 | 47 | LinProbFlatHashTable | No | Eph | LinProbFlatHashTableStEph | Open-addressing hash table with linear probing. |
| 149 | 47 | ParaHashTable | No | Eph | ParaHashTableStEph | Parallel hash table; lock-striped concurrent hash map. |
| 150 | 47 | QuadProbFlatHashTable | No | Eph | QuadProbFlatHashTableStEph | Open-addressing hash table with quadratic probing. |
| 151 | 47 | StructChainedHashTable | - | - | StructChainedHashTable | Chained hash table using struct-based buckets. |
| 152 | 47 | VecChainedHashTable | No | Eph | VecChainedHashTableStEph | Chained hash table with Vec-backed buckets; dynamic bucket growth. |
| 153 | 49 | MinEditDist | Yes | Eph | MinEditDistMtEph | Minimum edit distance (Levenshtein) via dynamic programming; O(m*n) DP table. |
| 154 | 49 | MinEditDist | Yes | Per | MinEditDistMtPer | Minimum edit distance (Levenshtein) via dynamic programming; O(m*n) DP table. |
| 155 | 49 | MinEditDist | No | Eph | MinEditDistStEph | Minimum edit distance (Levenshtein) via dynamic programming; O(m*n) DP table. |
| 156 | 49 | MinEditDist | No | Per | MinEditDistStPer | Minimum edit distance (Levenshtein) via dynamic programming; O(m*n) DP table. |
| 157 | 49 | SubsetSum | Yes | Eph | SubsetSumMtEph | Subset sum via dynamic programming; pseudo-polynomial O(n*target) DP. |
| 158 | 49 | SubsetSum | Yes | Per | SubsetSumMtPer | Subset sum via dynamic programming; pseudo-polynomial O(n*target) DP. |
| 159 | 49 | SubsetSum | No | Eph | SubsetSumStEph | Subset sum via dynamic programming; pseudo-polynomial O(n*target) DP. |
| 160 | 49 | SubsetSum | No | Per | SubsetSumStPer | Subset sum via dynamic programming; pseudo-polynomial O(n*target) DP. |
| 161 | 50 | MatrixChain | Yes | Eph | MatrixChainMtEph | Matrix chain multiplication optimization via dynamic programming; O(n^3) DP over parenthesization. |
| 162 | 50 | MatrixChain | Yes | Per | MatrixChainMtPer | Matrix chain multiplication optimization via dynamic programming; O(n^3) DP over parenthesization. |
| 163 | 50 | MatrixChain | No | Eph | MatrixChainStEph | Matrix chain multiplication optimization via dynamic programming; O(n^3) DP over parenthesization. |
| 164 | 50 | MatrixChain | No | Per | MatrixChainStPer | Matrix chain multiplication optimization via dynamic programming; O(n^3) DP over parenthesization. |
| 165 | 50 | OptBinSearchTree | Yes | Eph | OptBinSearchTreeMtEph | Optimal binary search tree via dynamic programming; O(n^3) DP minimizing weighted path length. |
| 166 | 50 | OptBinSearchTree | Yes | Per | OptBinSearchTreeMtPer | Optimal binary search tree via dynamic programming; O(n^3) DP minimizing weighted path length. |
| 167 | 50 | OptBinSearchTree | No | Eph | OptBinSearchTreeStEph | Optimal binary search tree via dynamic programming; O(n^3) DP minimizing weighted path length. |
| 168 | 50 | OptBinSearchTree | No | Per | OptBinSearchTreeStPer | Optimal binary search tree via dynamic programming; O(n^3) DP minimizing weighted path length. |
| 169 | 51 | BottomUpDP | Yes | Eph | BottomUpDPMtEph | Bottom-up dynamic programming framework. |
| 170 | 51 | BottomUpDP | Yes | Per | BottomUpDPMtPer | Bottom-up dynamic programming framework. |
| 171 | 51 | BottomUpDP | No | Eph | BottomUpDPStEph | Bottom-up dynamic programming framework. |
| 172 | 51 | BottomUpDP | No | Per | BottomUpDPStPer | Bottom-up dynamic programming framework. |
| 173 | 51 | SeqSpecsAndLemmas | - | - | SeqSpecsAndLemmas | Shared sequence specifications and lemmas for DP. |
| 174 | 51 | TopDownDP | Yes | Eph | TopDownDPMtEph | Top-down (memoized) dynamic programming framework. |
| 175 | 51 | TopDownDP | Yes | Per | TopDownDPMtPer | Top-down (memoized) dynamic programming framework. |
| 176 | 51 | TopDownDP | No | Eph | TopDownDPStEph | Top-down (memoized) dynamic programming framework. |
| 177 | 51 | TopDownDP | No | Per | TopDownDPStPer | Top-down (memoized) dynamic programming framework. |
| 178 | 52 | AdjMatrixGraph | Yes | Eph | AdjMatrixGraphMtEph | Adjacency matrix graph representation; V×V boolean/weight matrix, O(V^2) space, O(1) edge lookup. |
| 179 | 52 | AdjMatrixGraph | Yes | Per | AdjMatrixGraphMtPer | Adjacency matrix graph representation; V×V boolean/weight matrix, O(V^2) space, O(1) edge lookup. |
| 180 | 52 | AdjMatrixGraph | No | Eph | AdjMatrixGraphStEph | Adjacency matrix graph representation; V×V boolean/weight matrix, O(V^2) space, O(1) edge lookup. |
| 181 | 52 | AdjMatrixGraph | No | Per | AdjMatrixGraphStPer | Adjacency matrix graph representation; V×V boolean/weight matrix, O(V^2) space, O(1) edge lookup. |
| 182 | 52 | AdjSeqGraph | Yes | Eph | AdjSeqGraphMtEph | Adjacency sequence graph representation; list-of-lists graph, O(V+E) space. |
| 183 | 52 | AdjSeqGraph | Yes | Per | AdjSeqGraphMtPer | Adjacency sequence graph representation; list-of-lists graph, O(V+E) space. |
| 184 | 52 | AdjSeqGraph | No | Eph | AdjSeqGraphStEph | Adjacency sequence graph representation; list-of-lists graph, O(V+E) space. |
| 185 | 52 | AdjSeqGraph | No | Per | AdjSeqGraphStPer | Adjacency sequence graph representation; list-of-lists graph, O(V+E) space. |
| 186 | 52 | AdjTableGraph | Yes | Per | AdjTableGraphMtPer | Adjacency table graph representation; map-of-sets graph with keyed adjacency lookup. |
| 187 | 52 | AdjTableGraphSpecsAndLemmas | - | - | AdjTableGraphSpecsAndLemmas | Shared specifications and lemmas for adjacency-table graph variants. |
| 188 | 52 | AdjTableGraph | No | Eph | AdjTableGraphStEph | Adjacency table graph representation; map-of-sets graph with keyed adjacency lookup. |
| 189 | 52 | AdjTableGraph | No | Per | AdjTableGraphStPer | Adjacency table graph representation; map-of-sets graph with keyed adjacency lookup. |
| 190 | 52 | EdgeSetGraph | Yes | Eph | EdgeSetGraphMtEph | Edge-set graph representation; graph as a set of edges (V, A⊆V×V), O(E) space. |
| 191 | 52 | EdgeSetGraph | Yes | Per | EdgeSetGraphMtPer | Edge-set graph representation; graph as a set of edges (V, A⊆V×V), O(E) space. |
| 192 | 52 | EdgeSetGraph | No | Eph | EdgeSetGraphStEph | Edge-set graph representation; graph as a set of edges (V, A⊆V×V), O(E) space. |
| 193 | 52 | EdgeSetGraph | No | Per | EdgeSetGraphStPer | Edge-set graph representation; graph as a set of edges (V, A⊆V×V), O(E) space. |
| 194 | 53 | GraphSearch | Yes | Per | GraphSearchMtPer | Generic graph search framework; priority-queue-based traversal shared across BFS/DFS/Dijkstra. |
| 195 | 53 | GraphSearch | No | Eph | GraphSearchStEph | Generic graph search framework; priority-queue-based traversal shared across BFS/DFS/Dijkstra. |
| 196 | 53 | GraphSearch | No | Per | GraphSearchStPer | Generic graph search framework; priority-queue-based traversal shared across BFS/DFS/Dijkstra. |
| 197 | 53 | PQMin | No | Eph | PQMinStEph | Priority queue min-extraction operation. |
| 198 | 53 | PQMin | No | Per | PQMinStPer | Priority queue min-extraction operation. |
| 199 | 54 | BFS | Yes | Eph | BFSMtEph | Breadth-first search; parallel level-synchronous BFS in addition to sequential. |
| 200 | 54 | BFS | Yes | Per | BFSMtPer | Breadth-first search; parallel level-synchronous BFS in addition to sequential. |
| 201 | 54 | BFSSpecsAndLemmas | - | - | BFSSpecsAndLemmas | Shared BFS specifications and lemmas. |
| 202 | 54 | BFS | No | Eph | BFSStEph | Breadth-first search; parallel level-synchronous BFS in addition to sequential. |
| 203 | 54 | BFS | No | Per | BFSStPer | Breadth-first search; parallel level-synchronous BFS in addition to sequential. |
| 204 | 55 | CycleDetect | No | Eph | CycleDetectStEph | Cycle detection in a directed graph; DFS-based or coloring-based cycle predicate. |
| 205 | 55 | CycleDetect | No | Per | CycleDetectStPer | Cycle detection in a directed graph; DFS-based or coloring-based cycle predicate. |
| 206 | 55 | DFSSpecsAndLemmas | - | - | DFSSpecsAndLemmas | Shared DFS specifications and lemmas. |
| 207 | 55 | DFS | No | Eph | DFSStEph | Depth-first search; standard iterative/recursive DFS traversal. |
| 208 | 55 | DFS | No | Per | DFSStPer | Depth-first search; standard iterative/recursive DFS traversal. |
| 209 | 55 | SCC | No | Eph | SCCStEph | Strongly connected components; Tarjan-style or Kosaraju-style SCC decomposition. |
| 210 | 55 | SCC | No | Per | SCCStPer | Strongly connected components; Tarjan-style or Kosaraju-style SCC decomposition. |
| 211 | 55 | TopoSort | No | Eph | TopoSortStEph | Topological sort of a DAG; Kahn-style or DFS-finish-order topological ordering. |
| 212 | 55 | TopoSort | No | Per | TopoSortStPer | Topological sort of a DAG; Kahn-style or DFS-finish-order topological ordering. |
| 213 | 56 | AllPairsResult | No | Eph | AllPairsResultStEphF64 (×2 numeric types: F64,I64) | All-pairs shortest path result container (distance + predecessor matrices). |
| 214 | 56 | AllPairsResult | No | Per | AllPairsResultStPerF64 (×2 numeric types: F64,I64) | All-pairs shortest path result container (distance + predecessor matrices). |
| 215 | 56 | PathWeightUtils | No | Eph | PathWeightUtilsStEph | Path-weight utility functions; shared helpers for shortest-path computations. |
| 216 | 56 | PathWeightUtils | No | Per | PathWeightUtilsStPer | Path-weight utility functions; shared helpers for shortest-path computations. |
| 217 | 56 | SSSPResult | No | Eph | SSSPResultStEphF64 (×2 numeric types: F64,I64) | Single-source shortest path result container (distance + predecessor arrays). |
| 218 | 56 | SSSPResult | No | Per | SSSPResultStPerF64 (×2 numeric types: F64,I64) | Single-source shortest path result container (distance + predecessor arrays). |
| 219 | 57 | Dijkstra | No | Eph | DijkstraStEphF64 (×2 numeric types: F64,U64) | Dijkstra's single-source shortest-path algorithm with priority queue; O((V+E) log V). |
| 220 | 57 | Stack | No | Eph | StackStEph | Stack data structure. |
| 221 | 58 | BellmanFord | No | Eph | BellmanFordStEphF64 (×2 numeric types: F64,I64) | Bellman-Ford single-source shortest-path algorithm; O(V*E), handles negative edge weights. |
| 222 | 59 | Johnson | Yes | Eph | JohnsonMtEphF64 (×2 numeric types: F64,I64) | Johnson's all-pairs shortest-path algorithm; reweighting plus n Dijkstra runs, O(V^2 log V + V*E). |
| 223 | 59 | Johnson | No | Eph | JohnsonStEphF64 (×2 numeric types: F64,I64) | Johnson's all-pairs shortest-path algorithm; reweighting plus n Dijkstra runs, O(V^2 log V + V*E). |
| 224 | 61 | EdgeContraction | Yes | Eph | EdgeContractionMtEph | Edge contraction primitive; parallel random mate contraction for MST/connectivity. |
| 225 | 61 | EdgeContraction | No | Eph | EdgeContractionStEph | Edge contraction primitive; parallel random mate contraction for MST/connectivity. |
| 226 | 61 | VertexMatching | Yes | Eph | VertexMatchingMtEph | Vertex matching for parallel contraction; random matching step of edge contraction. |
| 227 | 61 | VertexMatching | No | Eph | VertexMatchingStEph | Vertex matching for parallel contraction; random matching step of edge contraction. |
| 228 | 62 | StarContraction | Yes | Eph | StarContractionMtEph | Star contraction; parallel contraction of star-shaped subgraphs, alternative to random-mate matching. |
| 229 | 62 | StarContraction | No | Eph | StarContractionStEph | Star contraction; parallel contraction of star-shaped subgraphs, alternative to random-mate matching. |
| 230 | 62 | StarPartition | Yes | Eph | StarPartitionMtEph | Star partition; partitioning of vertices into stars for contraction. |
| 231 | 62 | StarPartition | No | Eph | StarPartitionStEph | Star partition; partitioning of vertices into stars for contraction. |
| 232 | 63 | Connectivity | Yes | Eph | ConnectivityMtEph | Connected components / graph connectivity; parallel connectivity via contraction. |
| 233 | 63 | Connectivity | No | Eph | ConnectivityStEph | Connected components / graph connectivity; parallel connectivity via contraction. |
| 234 | 64 | SpanTree | Yes | Eph | SpanTreeMtEph | Minimum spanning tree via contraction; parallel MST using star contraction. |
| 235 | 64 | SpanTree | No | Eph | SpanTreeStEph | Minimum spanning tree via contraction; parallel MST using star contraction. |
| 236 | 64 | TSPApprox | No | Eph | TSPApproxStEph | TSP 2-approximation via MST; Christofides-style MST-based TSP heuristic. |
| 237 | 65 | Kruskal | No | Eph | KruskalStEph | Kruskal's minimum spanning tree algorithm; sorted-edge greedy with union-find for cycle detection. |
| 238 | 65 | Prim | No | Eph | PrimStEph | Prim's minimum spanning tree algorithm; priority-queue-based growing tree. |
| 239 | 65 | UnionFindArray | No | Eph | UnionFindArrayStEph | Union-find (disjoint set union) with array-backed parent and union-by-rank; no path compression. |
| 240 | 65 | UnionFindNoPC | No | Eph | UnionFindNoPCStEph | Union-find (disjoint set union) with hashmap backing and union-by-rank; no path compression. |
| 241 | 65 | UnionFindPC | No | Eph | UnionFindPCStEph | Union-find with hashmap backing, union-by-rank, and path compression; near-O(alpha(n)) amortized. |
| 242 | 66 | Boruvka | Yes | Eph | BoruvkaMtEph | Borůvka's minimum spanning tree algorithm; parallel MST by iteratively contracting minimum edges. |
| 243 | 66 | Boruvka | No | Eph | BoruvkaStEph | Borůvka's minimum spanning tree algorithm; parallel MST by iteratively contracting minimum edges. |
