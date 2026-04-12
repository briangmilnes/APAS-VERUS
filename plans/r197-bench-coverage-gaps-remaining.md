# R197 Benchmark Coverage Gaps Remaining

Generated: 2026-04-11. These are the benching opportunities not addressed in R197.

## High Priority (not yet benched)

| # | Chap | File | Asymptotic class | Notes |
|---|------|------|-----------------|-------|
| 1 | 37 | BSTAVLStEph | O(log n) AVL rotate/insert | AVL tree with rotations |
| 2 | 37 | BSTSplayStEph | O(log n) amortized splay | Note: splay requires `iter_batched` |
| 3 | 37 | BSTTreapStEph (Chap39) | O(log n) expected treap | Randomized BST |
| 4 | 37 | BSTRBStEph | O(log n) red-black | Red-black tree |
| 5 | 37 | BSTBBAlphaStEph | O(log n) weight-balanced | BB-alpha balanced |
| 6 | 41 | AVLTreeSetStEph | O(log n) set | Set ops on AVL tree |
| 7 | 43 | OrderedTableStEph | O(log n) ordered table | insert/find/previous/first |
| 8 | 43 | OrderedSetStEph | O(log n) ordered set | |
| 9 | 45 | BinaryHeapPQ | O(log n) PQ | insert/delete-min |
| 10 | 45 | LeftistHeapPQ | O(log n) leftist heap merge | |
| 11 | 47 | VecChainedHashTableStEph | O(1) amortized chained hash | Needs `HashTable` setup |
| 12 | 47 | LinProbFlatHashTableStEph | O(1) amortized linear probe | Needs `HashTable` setup |
| 13 | 57 | DijkstraStEphU64 | O((V+E) log V) | Graph construction required |
| 14 | 58 | BellmanFordStEphI64 | O(VE) | Graph construction required |
| 15 | 64 | SpanTreeStEph | O(E log V) MST | |
| 16 | 65 | KruskalStEph | O(E log E) MST | Uses UnionFindPC internally |
| 17 | 65 | PrimStEph | O(E log V) Prim MST | |
| 18 | 66 | BoruvkaStEph | O(E log V) Borůvka MST | |

## Medium Priority

| # | Chap | File | Asymptotic class | Notes |
|---|------|------|-----------------|-------|
| 1 | 05 | SetStEph | O(n) | Simple sequence-backed set |
| 2 | 21 | Algorithm21_1 | O(n) scan | |
| 3 | 21 | Algorithm21_2 | O(n) reduce | |
| 4 | 26 | ETSPStEph | O(n log n) ETSP | TSP approximation |
| 5 | 26 | ScanDCStPer | O(n log n) | Divide-and-conquer scan |
| 6 | 28 | MaxContigSubSumBruteStEph | O(n³) | Brute-force baseline |
| 7 | 28 | MaxContigSubSumDivConStEph | O(n log n) | D&C max-contiguous |
| 8 | 37 | BSTKeyValueStEph (Chap40) | O(log n) map | Key-value BST map |
| 9 | 42 | TableStEph | O(n) table | |
| 10 | 50 | MatrixChainStEph | O(n³) | Matrix chain DP |
| 11 | 53 | GraphSearchStEph | O(V+E) | BFS/DFS graph search |
| 12 | 54 | BFSStEph | O(V+E) | BFS on graph |
| 13 | 55 | DFSStEph | O(V+E) | DFS on graph |
| 14 | 55 | TopoSortStEph | O(V+E) | Topological sort |
| 15 | 63 | ConnectivityStEph | O(V log V) | Connected components |

## Low Priority / Skip

- Mt files (Chap36 QuickSortMtEph, etc.): require multi-thread setup; timing depends on scheduler
- Pure-spec files (MathSeq, ContractSpecsAndLemmas, etc.): no executable functions
- Chap02 HFSchedulerMtEph: scheduler infrastructure
- Chap30 Probability: math utilities only
- Chap18 LinkedListStEph: sequential linked list, not a target algorithm

## Notes on HashTable Benchmarks

`VecChainedHashTableStEph` and `LinProbFlatHashTableStEph` require constructing a
`HashTable<Key, Value, Entry, Metrics, H>` struct directly. The `H: Fn(&Key, usize) -> usize`
hash parameter requires a concrete closure type. Suggested pattern:

```rust
fn build_hash_table(capacity: usize) -> HashTable<u64, u64, Vec<(u64, u64)>, (), impl Fn(&u64, usize) -> usize> {
    HashTable {
        table: ArraySeqStEphS { seq: (0..capacity).map(|_| Vec::new()).collect() },
        metrics: (),
        hash: |k: &u64, cap| (*k as usize) % cap,
    }
}
```

This requires reading `src/Chap47/FlatHashTable.rs` to confirm the struct fields.

## Notes on Graph Benchmarks

Dijkstra, BellmanFord, Kruskal, Prim, Borůvka all require constructing a weighted graph.
The `WeightedDirGraphStEphI64` or `WeightedDirGraphStEphU64` types from Chap06 can be
used. All are medium complexity to set up — budget ~1hr per bench.
