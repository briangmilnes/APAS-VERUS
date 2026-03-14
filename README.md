# APAS-VERUS

Formally verified implementations of algorithms from "Algorithms Parallel and Sequential" (APAS) by Acar and Blelloch.
Rust code with machine-checked proofs in the [Verus](https://github.com/verus-lang/verus) verification framework, by Brian Milnes.

Claude used for the code generation and proving in Verus. Although the AIs are improving,
even with 50+ Cursor rules (in .cursor), they made so many bad judgements that I had to
develop two software engineering tools to clean things up:

- [veracity](https://github.com/briangmilnes/veracity) - Verus code analysis tools (proof hole detection, spec strength review, function search).
- [rusticate](https://github.com/briangmilnes/rusticate) - Rust code style and structure review tools.

**All 44 algorithm chapters verified, 34 with zero proof holes**

**3976 verified, 0 errors | 2600 runtime tests | 147 proof time tests | 311 proof holes**

## Project Structure

- `src/` - Verified algorithm implementations organized by chapter
- `src/vstdplus/` - Extensions to the Verus standard library (24 modules)
- `src/standards/` - Verus coding standards and patterns (14 files)
- `tests/` - Rust unit tests for algorithm correctness
- `benches/` - Performance benchmarks using Criterion

## Proof State

Full verification: **3976 verified, 0 errors**

| # | Metric | Count |
|---|--------|-------|
| 1 | Chapters verified | 44 |
| 2 | Chapters with zero proof holes | 34 |
| 3 | Clean modules (no holes) | 282 |
| 4 | Holed modules | 161 |
| 5 | Total verified modules | 443 |
| 6 | Clean proof functions | 650 |
| 7 | Runtime tests (RTT) | 2600 |
| 8 | Proof time tests (PTT) | 147 |

### Proof Holes: 311 total (algorithm chapters only)

| # | Hole Type | Count | Pct | Notes |
|---|-----------|-------|-----|-------|
| 1 | `external_body` | 172 | 55% | Thread boundaries, opaque types, Verus limitations |
| 2 | `assume()` | 135 | 43% | Proof obligations not yet discharged |
| 3 | `external` | 2 | 1% | External function specifications |
| 4 | `trivial spec*wf { true }` | 2 | 1% | Well-formedness specs returning true |

Holes tracked by [veracity](https://github.com/briangmilnes/veracity). 10 chapters remain holed; 34 are fully clean.

## Algorithm Status

### Chapter 02: Scheduling - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | HFSchedulerMtEph | Mt | Help-First Scheduler, `pool.join(fa, fb)` |
| 2 | FibonacciHFScheduler | Mt | `fib_pool` using HFScheduler |

### Chapter 03: Insertion Sort - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | InsertionSortStEph | St | Generic, multiset preservation proven |

### Chapter 05: Sets, Relations, Mappings - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | SetStEph | St | Ephemeral set, custom iterator + ghost |
| 2 | SetMtEph | Mt | Multi-threaded set |
| 3 | RelationStEph | St | Binary relations |
| 4 | MappingStEph | St | Key-value mappings |
| 5 | KleeneStPer | St | Kleene closure of relations |

### Chapter 06: Graphs - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | DirGraphStEph | St | Directed graph |
| 2 | DirGraphMtEph | Mt | Parallel directed graph |
| 3 | UnDirGraphStEph | St | Undirected graph |
| 4 | UnDirGraphMtEph | Mt | Parallel undirected graph |
| 5 | LabDirGraphStEph | St | Labeled directed graph |
| 6 | LabDirGraphMtEph | Mt | Parallel labeled directed graph |
| 7 | LabUnDirGraphStEph | St | Labeled undirected graph |
| 8 | LabUnDirGraphMtEph | Mt | Parallel labeled undirected graph |
| 9-20 | WeightedDirGraphStEph{U8..Isize} | St | All 12 integer types |

### Chapter 11: Fibonacci - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | FibonacciStEph | St | Sequential with overflow proofs |
| 2 | FibonacciMtEph2Threads | Mt | 2-thread parallel |
| 3 | FibonacciMtPerAllThreads | Mt | ParaPair! macro |
| 4 | FibonacciMtEphRecomputes | Mt | Recomputation variant |
| 5 | FibonacciMtPerTSM | Mt | Tokenized state machine |

### Chapter 12: Concurrency Primitives - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | Exercise12_1 | Mt | Ticket lock via fetch-and-add |
| 2 | Exercise12_2 | Mt | CAS-based fetch-and-add |
| 3 | Exercise12_5 | Mt | Lock-free Treiber stack |

Atomics and raw pointers not supported by Verus — external_body wrappers required.

### Chapter 17: MathSeq - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | MathSeq | St | Vec-backed dense sequence, uses `HashMapWithView` |

### Chapter 18: Sequences - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | ArraySeq | St | Base array sequence |
| 2 | ArraySeqStPer | St | Persistent array sequence |
| 3 | ArraySeqStEph | St | Ephemeral array sequence |
| 4 | LinkedListStPer | St | Persistent linked list |
| 5 | LinkedListStEph | St | Ephemeral linked list |
| 6 | ArraySeqMtEph | Mt | Parallel array sequence |
| 7 | ArraySeqMtPer | Mt | Parallel persistent array sequence |

All with custom iterators and ForLoopGhostIterator.

### Chapter 19: Sequences (Advanced) - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | ArraySeqStPer | St | Compositional algorithms (tabulate, flatten) |
| 2 | ArraySeqStEph | St | Ephemeral: clone+set update |
| 3 | ArraySeqMtEph | Mt | Parallel map/filter/reduce via fork-join |
| 4 | ArraySeqMtEphSlice | Mt | Slice-based parallel operations |

### Chapter 21: Trees and Algorithms - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | Algorithm21_1 | St | Tree size |
| 2 | Algorithm21_2 | St | Tree reduce |
| 3 | Algorithm21_5 | St | Tree map |
| 4 | Algorithm21_6 | St | Tree filter |
| 5 | Exercise21_5 | St | |
| 6 | Exercise21_6 | St | |
| 7 | Exercise21_7 | St | |
| 8 | Exercise21_8 | St | |
| 9 | Exercise21_9 | St | |
| 10 | Problem21_1 | St | |
| 11 | Problem21_4 | St | |

### Chapter 23: Trees - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | PrimTreeSeqStPer | St | Primitive tree sequence |
| 2 | BalBinTreeStEph | St | Balanced binary tree |

### Chapter 26: Divide and Conquer - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | DivConReduceStPer | St | Divide-and-conquer reduce |
| 2 | MergeSortStPer | St | Merge sort |
| 3 | ScanDCStPer | St | Divide-and-conquer scan |
| 4 | DivConReduceMtPer | Mt | Parallel divide-and-conquer reduce |
| 5 | MergeSortMtPer | Mt | Parallel merge sort |
| 6 | ScanDCMtPer | Mt | Parallel divide-and-conquer scan |

### Chapter 27: Scan and Reduce - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | ReduceContractStEph | St | Contraction-based reduce |
| 2 | ReduceContractMtEph | Mt | Parallel contraction reduce |
| 3 | ScanContractStEph | St | Contraction-based scan |
| 4 | ScanContractMtEph | Mt | Parallel contraction scan |

### Chapter 28: Max Contiguous Subsequence Sum - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | MCSSSpec | — | Shared specification module |
| 2 | MaxContigSubSumBruteStEph | St | Brute force O(n^2) |
| 3 | MaxContigSubSumReducedStEph | St | Reduced sequential |
| 4 | MaxContigSubSumDivConStEph | St | Divide-and-conquer |
| 5 | MaxContigSubSumIterStEph | St | Iterative (Kadane's) |
| 6 | MaxContigSubSumReducedMcsseStEph | St | Reduced with MCSS element |
| 7 | MaxContigSubSumDivConOptStEph | St | Optimized divide-and-conquer |
| 8 | MaxContigSubSumDivConMtEph | Mt | Parallel divide-and-conquer |
| 9 | MaxContigSubSumDivConOptMtEph | Mt | Parallel optimized divide-and-conquer |

### Chapter 30: Probability - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | Probability | St | Discrete probability foundations |

### Chapter 35: Order Statistics - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | OrderStatSelectStEph | St | Sequential selection |
| 2 | OrderStatSelectStPer | St | Persistent selection |
| 3 | OrderStatSelectMtEph | Mt | Parallel selection |
| 4 | OrderStatSelectMtPer | Mt | Parallel persistent selection |

### Chapter 36: QuickSort - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | QuickSortStEph | St | Sequential quicksort |
| 2 | QuickSortMtEph | Mt | Parallel quicksort |

### Chapter 37: BST Variants - ✅ VERIFIED

8 unconditional + 8 behind `all_chapters` feature gate.

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | AVLTreeSeq | St | AVL tree sequence (base) |
| 2 | AVLTreeSeqStEph | St | Ephemeral AVL tree sequence |
| 3 | AVLTreeSeqStPer | St | Persistent AVL tree sequence |
| 4 | AVLTreeSeqMtPer | Mt | Parallel AVL tree sequence (gated) |
| 5 | BSTPlainStEph | St | Plain BST |
| 6 | BSTAVLStEph | St | AVL-balanced BST |
| 7 | BSTRBStEph | St | Red-black BST |
| 8 | BSTRBMtEph | Mt | Parallel red-black BST (gated) |
| 9 | BSTSplayStEph | St | Splay BST |
| 10 | BSTSplayMtEph | Mt | Parallel splay BST (gated) |
| 11 | BSTBBAlphaStEph | St | BB-alpha weight-balanced BST |
| 12-16 | BSTSet{Plain,AVL,RB,Splay,BBAlpha}MtEph | Mt | BST-backed sets (gated) |

### Chapter 38: Parallel BST - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | BSTParaStEph | St | Parallel BST operations |
| 2 | BSTParaMtEph | Mt | Multi-threaded parallel BST |

### Chapter 39: Treaps - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | BSTTreapStEph | St | Randomized treap |
| 2 | BSTTreapMtEph | Mt | Parallel treap |
| 3 | BSTParaTreapMtEph | Mt | Parallel operations on treap |
| 4 | BSTSetTreapMtEph | Mt | Treap-backed set |

### Chapter 40: BST Key-Value - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | BSTKeyValueStEph | St | Key-value BST |
| 2 | BSTSizeStEph | St | Size-augmented BST |
| 3 | BSTReducedStEph | St | Reduced (augmented) BST |

### Chapter 41: Sets via BST - ✅ VERIFIED

6 unconditional + 1 behind `all_chapters` feature gate.

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | ArraySetStEph | St | Array-backed set |
| 2 | ArraySetEnumMtEph | Mt | Parallel array set (enum-based) |
| 3 | AVLTreeSetStEph | St | AVL tree set |
| 4 | AVLTreeSetStPer | St | Persistent AVL tree set |
| 5 | AVLTreeSetMtEph | Mt | Parallel AVL tree set |
| 6 | AVLTreeSetMtPer | Mt | Parallel persistent AVL tree set (gated) |
| 7 | Example41_3 | St | Textbook example |

### Chapter 42: Hash Tables - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | TableStEph | St | Ephemeral hash table |
| 2 | TableStPer | St | Persistent hash table |
| 3 | TableMtEph | Mt | Parallel hash table |
| 4 | Example42_1 | St | Textbook example |

### Chapter 43: Ordered Tables - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | OrderedTableStEph | St | Ordered table |
| 2 | OrderedTableStPer | St | Persistent ordered table |
| 3 | OrderedTableMtEph | Mt | Parallel ordered table |
| 4 | OrderedTableMtPer | Mt | Parallel persistent ordered table |
| 5 | AugOrderedTableStEph | St | Augmented ordered table |
| 6 | AugOrderedTableStPer | St | Persistent augmented ordered table |
| 7 | AugOrderedTableMtEph | Mt | Parallel augmented ordered table |
| 8 | OrderedSetStEph | St | Ordered set |
| 9 | OrderedSetStPer | St | Persistent ordered set |
| 10 | OrderedSetMtEph | Mt | Parallel ordered set |
| 11 | Example43_1 | St | Textbook example |

### Chapter 44: Document Index - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | DocumentIndex | St | Document indexing |
| 2 | Example44_1 | St | Textbook example |

### Chapter 45: Priority Queues - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | UnsortedListPQ | St | Unsorted list priority queue |
| 2 | SortedListPQ | St | Sorted list priority queue |
| 3 | BinaryHeapPQ | St | Binary heap with bubble_down verified |
| 4 | BalancedTreePQ | St | Balanced tree priority queue |
| 5 | LeftistHeapPQ | St | Leftist heap priority queue |
| 6 | HeapsortExample | St | Heapsort using BinaryHeapPQ |
| 7 | Example45_2 | St | Textbook example |

### Chapter 47: Hashing - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | ChainedHashTable | St | Base chained hash table |
| 2 | StructChainedHashTable | St | Struct-based chained hash table |
| 3 | VecChainedHashTableStEph | St | Vec-backed chained hash table |
| 4 | LinkedListChainedHashTableStEph | St | Linked-list chained hash table |
| 5 | FlatHashTable | St | Base flat (open-addressing) hash table |
| 6 | LinProbFlatHashTableStEph | St | Linear probing |
| 7 | QuadProbFlatHashTableStEph | St | Quadratic probing |
| 8 | DoubleHashFlatHashTableStEph | St | Double hashing |
| 9 | ParaHashTableStEph | St | Parallel hash table |

### Chapter 49: Dynamic Programming I - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | SubsetSumStEph | St | Subset sum |
| 2 | SubsetSumStPer | St | Persistent subset sum |
| 3 | SubsetSumMtEph | Mt | Parallel subset sum |
| 4 | SubsetSumMtPer | Mt | Parallel persistent subset sum |
| 5 | MinEditDistStEph | St | Minimum edit distance |
| 6 | MinEditDistStPer | St | Persistent min edit distance |
| 7 | MinEditDistMtEph | Mt | Parallel min edit distance |
| 8 | MinEditDistMtPer | Mt | Parallel persistent min edit distance |

### Chapter 50: Dynamic Programming II - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | MatrixChainStEph | St | Matrix chain multiplication |
| 2 | MatrixChainStPer | St | Persistent matrix chain |
| 3 | MatrixChainMtEph | Mt | Parallel matrix chain |
| 4 | MatrixChainMtPer | Mt | Parallel persistent matrix chain |
| 5 | OptBinSearchTreeStEph | St | Optimal binary search tree |
| 6 | OptBinSearchTreeStPer | St | Persistent optimal BST |
| 7 | OptBinSearchTreeMtEph | Mt | Parallel optimal BST |
| 8 | OptBinSearchTreeMtPer | Mt | Parallel persistent optimal BST |

### Chapter 51: Dynamic Programming III - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | BottomUpDPStEph | St | Bottom-up DP |
| 2 | BottomUpDPStPer | St | Persistent bottom-up DP |
| 3 | BottomUpDPMtEph | Mt | Parallel bottom-up DP |
| 4 | BottomUpDPMtPer | Mt | Parallel persistent bottom-up DP |
| 5 | TopDownDPStEph | St | Top-down (memoized) DP |
| 6 | TopDownDPStPer | St | Persistent top-down DP |
| 7 | TopDownDPMtEph | Mt | Parallel top-down DP |
| 8 | TopDownDPMtPer | Mt | Parallel persistent top-down DP |

### Chapter 52: Graph Representations - ✅ VERIFIED (ZERO HOLES)

8 unconditional + 2 behind `all_chapters` feature gate.

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | AdjSeqGraphStEph | St | Adjacency sequence graph |
| 2 | AdjSeqGraphStPer | St | Persistent adjacency sequence |
| 3 | AdjSeqGraphMtEph | Mt | Parallel adjacency sequence |
| 4 | AdjSeqGraphMtPer | Mt | Parallel persistent adjacency sequence |
| 5 | AdjMatrixGraphStEph | St | Adjacency matrix graph |
| 6 | AdjMatrixGraphStPer | St | Persistent adjacency matrix |
| 7 | AdjMatrixGraphMtEph | Mt | Parallel adjacency matrix |
| 8 | AdjMatrixGraphMtPer | Mt | Parallel persistent adjacency matrix |
| 9 | AdjTableGraphMtPer | Mt | Adjacency table (gated) |
| 10 | EdgeSetGraphMtPer | Mt | Edge set graph (gated) |

### Chapter 53: Graph Search - ✅ VERIFIED

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | PQMinStEph | St | Priority queue minimum |
| 2 | PQMinStPer | St | Persistent PQ minimum |
| 3 | GraphSearchStEph | St | Generic graph search |
| 4 | GraphSearchStPer | St | Persistent graph search |
| 5 | GraphSearchMtPer | Mt | Parallel persistent graph search |

### Chapter 54: BFS - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | BFSStEph | St | Breadth-first search |
| 2 | BFSStPer | St | Persistent BFS |
| 3 | BFSMtEph | Mt | Parallel BFS |
| 4 | BFSMtPer | Mt | Parallel persistent BFS |

### Chapter 55: DFS - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | DFSStEph | St | Depth-first search |
| 2 | DFSStPer | St | Persistent DFS |
| 3 | TopoSortStEph | St | Topological sort |
| 4 | TopoSortStPer | St | Persistent topological sort |
| 5 | SCCStEph | St | Strongly connected components |
| 6 | SCCStPer | St | Persistent SCC |
| 7 | CycleDetectStEph | St | Cycle detection |
| 8 | CycleDetectStPer | St | Persistent cycle detection |

### Chapter 56: Shortest Paths (Results) - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | SSSPResultStEphI64 | St | Single-source result (i64) |
| 2 | SSSPResultStPerI64 | St | Persistent SSSP result (i64) |
| 3 | AllPairsResultStEphI64 | St | All-pairs result (i64) |
| 4 | AllPairsResultStPerI64 | St | Persistent all-pairs result (i64) |
| 5 | PathWeightUtilsStEph | St | Path weight utilities |
| 6 | PathWeightUtilsStPer | St | Persistent path weight utilities |
| 7 | SSSPResultStEphF64 | St | Single-source result (f64) |
| 8 | SSSPResultStPerF64 | St | Persistent SSSP result (f64) |
| 9 | AllPairsResultStEphF64 | St | All-pairs result (f64) |
| 10 | AllPairsResultStPerF64 | St | Persistent all-pairs result (f64) |

### Chapter 57: Dijkstra - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | StackStEph | St | Stack for graph algorithms |
| 2 | DijkstraStEphI64 | St | Dijkstra shortest paths (i64) |
| 3 | DijkstraStEphF64 | St | Dijkstra shortest paths (f64) |

### Chapter 58: Bellman-Ford - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | BellmanFordStEphI64 | St | Bellman-Ford shortest paths (i64) |
| 2 | BellmanFordStEphF64 | St | Bellman-Ford shortest paths (f64) |

### Chapter 59: Johnson - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | JohnsonStEphI64 | St | Johnson's all-pairs shortest paths (i64) |
| 2 | JohnsonStEphF64 | St | Johnson's all-pairs shortest paths (f64) |
| 3 | JohnsonMtEphI64 | Mt | Parallel Johnson (i64) |
| 4 | JohnsonMtEphF64 | Mt | Parallel Johnson (f64) |

### Chapter 61: Matching - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | VertexMatchingStEph | St | Vertex matching |
| 2 | VertexMatchingMtEph | Mt | Parallel vertex matching |
| 3 | EdgeContractionStEph | St | Edge contraction |
| 4 | EdgeContractionMtEph | Mt | Parallel edge contraction |

### Chapter 62: Star Partition - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | StarPartitionStEph | St | Star partition |
| 2 | StarContractionStEph | St | Star contraction |
| 3 | StarPartitionMtEph | Mt | Parallel star partition |
| 4 | StarContractionMtEph | Mt | Parallel star contraction |

### Chapter 63: Connectivity - ✅ VERIFIED (ZERO HOLES)

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | ConnectivityStEph | St | Graph connectivity |
| 2 | ConnectivityMtEph | Mt | Parallel graph connectivity |

### Chapter 64: Spanning Trees - ✅ VERIFIED (`all_chapters`)

Behind `all_chapters` feature gate.

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | SpanTreeStEph | St | Spanning tree |
| 2 | SpanTreeMtEph | Mt | Parallel spanning tree |
| 3 | TSPApproxStEph | St | TSP approximation |

### Chapter 65: MST (Kruskal, Prim) - ✅ VERIFIED (ZERO HOLES) (`all_chapters`)

Behind `all_chapters` feature gate.

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | UnionFindStEph | St | Union-find data structure |
| 2 | KruskalStEph | St | Kruskal's MST algorithm |
| 3 | PrimStEph | St | Prim's MST algorithm |

### Chapter 66: MST (Boruvka) - ✅ VERIFIED (`all_chapters`)

Behind `all_chapters` feature gate.

| # | Algorithm | St/Mt | Notes |
|---|-----------|-------|-------|
| 1 | BoruvkaStEph | St | Boruvka's MST algorithm |
| 2 | BoruvkaMtEph | Mt | Parallel Boruvka's MST |

---

**Legend:**
- ✅ VERIFIED (ZERO HOLES) - All functions verified, no proof holes of any kind
- ✅ VERIFIED - All files verify, some accepted holes (PartialEq bridge, RwLock, external_body for atomics) or proof work remaining

### vstdplus Library Extensions

| # | Module | Description |
|---|--------|-------------|
| 1 | `accept` | Accepted proof hole tracking |
| 2 | `arithmetic/power2_plus` | Power of 2 lemmas (`lemma_pow2_mono`, bounds) |
| 3 | `arc_rwlock` | Arc<RwLock> bridge functions (`new_arc_rwlock`, `clone_arc_rwlock`) |
| 4 | `checked_int` | Overflow-checked signed integers (`CheckedI8`..`CheckedI128`) |
| 5 | `checked_nat` | Overflow-checked unsigned integers (`CheckedU8`..`CheckedU128`) |
| 6 | `clone_plus` | `ClonePlus` trait for Verus-compatible cloning |
| 7 | `feq` | Functional equality |
| 8 | `float` | `FloatTotalOrder` trait and IEEE 754 axioms |
| 9 | `hash_map_with_view_plus` | Enhanced `HashMap` with view specs |
| 10 | `hash_set_specs` | HashSet specification helpers |
| 11 | `hash_set_with_view_plus` | Enhanced `HashSet` with iterator specs |
| 12 | `hashed_checked_u32` | Hash-compatible checked u32 |
| 13 | `monoid` | Monoid trait for algebraic structures |
| 14 | `multiset` | Multiset operations and lemmas |
| 15 | `partial_order` | `PartialOrdered` trait |
| 16 | `pervasives_plus` | Common utility functions |
| 17 | `rand` | Verified random number generation interface |
| 18 | `seq` | Sequence lemmas |
| 19 | `seq_set` | Lemmas connecting `Seq` and `Set` operations, weighted sums |
| 20 | `smart_ptrs` | Arc/Box specification helpers |
| 21 | `sqrt` | Integer square root |
| 22 | `threads_plus` | Verified thread primitives (`spawn_plus`, `JoinHandlePlus`) |
| 23 | `total_order` | `TotalOrdered` trait for all 12 integer types |
| 24 | `VecQueue` | Verified queue using `Vec` |

### Standards Library

14 Verus coding standards demonstrating project patterns. Each is a compilable, verified example.

| # | Standard | Covers |
|---|----------|--------|
| 1 | `mod_standard` | Module layout, TOC, use-statement ordering |
| 2 | `view_standard` | View trait implementation |
| 3 | `deep_view_standard` | DeepView trait implementation |
| 4 | `iterators_standard` | Collection iterator protocol (10 components) |
| 5 | `mut_standard` | Mutable reference patterns in Verus |
| 6 | `multi_struct_standard` | Recursive enum with per-type traits |
| 7 | `table_of_contents_standard` | Section ordering inside/outside verus! |
| 8 | `using_closures_standard` | Closure patterns for fork-join |
| 9 | `wrapping_iterators_standard` | Wrapping Rust iterators for Verus |
| 10 | `rwlock_standard` | RwLockPredicate naming and invariants |
| 11 | `tsm_standard` | Tokenized state machine patterns |
| 12 | `arc_standard` | Arc deref pattern for verification |
| 13 | `hfscheduler_standard` | Help-First Scheduler (Arc<RwLock> + join) |
| 14 | `arc_rwlock_coarse_standard` | Coarse-grained Arc<RwLock> concurrency |

---

## Documentation

API documentation with Verus specifications (requires/ensures):

- [Browse docs/verusdoc/apas_verus/](docs/verusdoc/apas_verus/index.html) - Generated with `scripts/verusdoc.sh`

To regenerate:
```bash
./scripts/verusdoc.sh
```

## Building and Testing

All scripts live in `scripts/`, auto-detect the worktree root, and strip ANSI escape codes for Emacs `M-x compile`.

### Scripts

| # | Script | Usage | Purpose |
|---|--------|-------|---------|
| 1 | `scripts/validate.sh` | `validate.sh [full\|dev\|exp] [--time]` | Verus verification |
| 2 | `scripts/check.sh` | `check.sh` | `cargo check --lib` |
| 3 | `scripts/rtt.sh` | `rtt.sh [filter]` | Runtime tests (`-j 6`, 120s timeout) |
| 4 | `scripts/ptt.sh` | `ptt.sh [filter]` | Compile PTT lib + proof time tests (`-j 6`) |
| 5 | `scripts/holes.sh` | `holes.sh [dir-or-file]` | Proof hole detection |
| 6 | `scripts/validate-check-rtt-ptt.sh` | `validate-check-rtt-ptt.sh` | Full pipeline (stops on first failure) |
| 7 | `scripts/merge-agent.sh` | `merge-agent.sh <branch>` | Merge an agent branch + validate |
| 8 | `scripts/reset-agent-to-main.sh` | `reset-agent-to-main.sh` | Reset agent branch to `origin/main` + force push |

### Verification

```bash
scripts/validate.sh dev            # dev mode (skip cfg-gated modules)
scripts/validate.sh full --time    # full verification with timing breakdown
scripts/validate.sh exp            # experiments only
```

### Compilation Check

```bash
scripts/check.sh                   # cargo check --lib
```

### Runtime Tests (RTTs)

```bash
scripts/rtt.sh                     # all tests
scripts/rtt.sh bst                 # case-insensitive filter on test names
```

### Proof Time Tests (PTTs)

```bash
scripts/ptt.sh                     # compile lib + all PTTs
scripts/ptt.sh Chap05              # compile lib + filtered PTTs
```

### Proof Holes

```bash
scripts/holes.sh                   # all of src/
scripts/holes.sh src/Chap05/       # one chapter
scripts/holes.sh src/Chap05/SetStEph.rs  # one file
```

### Full Pipeline

```bash
scripts/validate-check-rtt-ptt.sh  # validate (dev) -> check -> RTT -> PTT
```

### Benchmarking

```bash
cargo bench                        # all benchmarks
cargo bench --bench BenchInsertionSortStEph  # specific benchmark
```

## Further Documentation

- [docs/Scripts.md](docs/Scripts.md) — detailed reference for every script in `scripts/`
- [docs/WorkingWithMultipleAgentsInWorktrees.md](docs/WorkingWithMultipleAgentsInWorktrees.md) — merge procedure, conflict resolution, and agent reset workflow

## Development Setup

1. Install [Verus](https://github.com/verus-lang/verus) (see `~/projects/verus/BUILD.md`)
2. Install Rust toolchain (pinned in `rust-toolchain.toml`)
3. Clone this repository
4. Run `scripts/validate.sh dev` to verify
5. Run `scripts/rtt.sh` to run tests

## Verification Approach

We use Verus to prove:
- **Functional correctness**: Algorithms satisfy their specifications (e.g., sorted output, correct traversal order)
- **Memory safety**: No undefined behavior, proper bounds checking
- **Resource properties**: Multiset preservation (e.g., sorting doesn't lose/add elements)

For generic algorithms, we use traits like `TotalOrdered` to abstract over ordering relationships while maintaining provability.

## License

Copyright (C) 2025 Acar, Blelloch and Milnes

## References

- [Algorithms Parallel and Sequential](http://www.parallel-algorithms-book.com/)
- [Verus Documentation](https://verus-lang.github.io/verus/)
