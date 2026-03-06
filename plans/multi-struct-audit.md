# Multi-Struct Standard Audit (2026-03-06)

Criteria: modules with **multiple structs** OR **a struct wrapped in an enum**.
Excludes: iterator structs (iterator standard), Inv-only predicates, PhantomData markers.

## All Files

| # | Cat | Chap | File | Full Path | Structs / Enum | Per-type traits? | Conforms? |
|---|-----|------|------|-----------|----------------|------------------|-----------|
| 1 | A | 23 | BalBinTreeStEph.rs | src/Chap23/BalBinTreeStEph.rs | `BalBinNode<T>` + enum `BalBinTree` | No — 1 trait | **No** |
| 2 | A | 23 | PrimTreeSeqStPer.rs | src/Chap23/PrimTreeSeqStPer.rs | `PrimTreeSeqStS<T>` + enum `PrimTreeSeqStTree` | No — 1 trait | **No** |
| 3 | A | 38 | BSTParaStEph.rs | src/Chap38/BSTParaStEph.rs | `NodeInner<T>`, `ParamBST<T>` + enum `Exposed` | No — 1 trait | **No** |
| 4 | A | 38 | BSTParaMtEph.rs | src/Chap38/BSTParaMtEph.rs | `NodeInner<T>`, `ParamBST<T>` + enum `Exposed` | No — 1 trait | **No** |
| 5 | A | 39 | BSTParaTreapMtEph.rs | src/Chap39/BSTParaTreapMtEph.rs | `NodeInner<T>`, `ParamTreap<T>` + enum `Exposed` | No — 1 trait | **No** |
| 6 | A | 45 | LeftistHeapPQ.rs | src/Chap45/LeftistHeapPQ.rs | `LeftistHeapPQ<T>` + enum `LeftistHeapNode` | Partial — 2 traits | **Partial** |
| 7 | A | 47 | FlatHashTable.rs | src/Chap47/FlatHashTable.rs | enum `FlatEntry { Empty, Occupied, Deleted }` | No | **No** |
| 8 | A | 02 | HFSchedulerMtEph.rs | src/Chap02/HFSchedulerMtEph.rs | `ExTaskState<T>` + enum `TaskState` | No | **N/A** |
| 9 | B | 37 | AVLTreeSeq.rs | src/Chap37/AVLTreeSeq.rs | `AVLTreeNode<T>` + `AVLTreeS<T>` | No — 1 trait | **No** |
| 10 | B | 37 | AVLTreeSeqStEph.rs | src/Chap37/AVLTreeSeqStEph.rs | `AVLTreeNode<T>` + `AVLTreeSeqStEphS<T>` | No — 1 trait | **No** |
| 11 | B | 37 | AVLTreeSeqStPer.rs | src/Chap37/AVLTreeSeqStPer.rs | `Node<T>` + `AVLTreeSeqStPerS<T>` | No — 1 trait | **No** |
| 12 | B | 37 | AVLTreeSeqMtPer.rs | src/Chap37/AVLTreeSeqMtPer.rs | `Node<T>` + `AVLTreeSeqMtPerS<T>` | No — 1 trait | **No** |
| 13 | B | 37 | BSTSplayStEph.rs | src/Chap37/BSTSplayStEph.rs | `Node<T>` + `BSTSplayStEph<T>` | No — 1 trait | **No** |
| 14 | B | 39 | BSTTreapStEph.rs | src/Chap39/BSTTreapStEph.rs | `Node<T>` + `BSTTreapStEph<T>` | No — 1 trait | **No** |
| 15 | B | 39 | BSTTreapMtEph.rs | src/Chap39/BSTTreapMtEph.rs | `Node<T>` + `BSTTreapMtEph<T>` | No — 1 trait | **No** |
| 16 | B | 40 | BSTKeyValueStEph.rs | src/Chap40/BSTKeyValueStEph.rs | `Node<K,V>` + `BSTKeyValueStEph<K,V>` | No — 1 trait | **No** |
| 17 | B | 40 | BSTSizeStEph.rs | src/Chap40/BSTSizeStEph.rs | `Node<T>` + `BSTSizeStEph<T>` | No — 1 trait | **No** |
| 18 | B | 40 | BSTReducedStEph.rs | src/Chap40/BSTReducedStEph.rs | `Node<K,V,R>` + `BSTReducedStEph<K,V,R,Op>` | No — 1 trait | **No** |
| 19 | C | 47 | StructChainedHashTable.rs | src/Chap47/StructChainedHashTable.rs | `Node<K,V>`, `ChainList<K,V>`, main | Node→Chain→Table | **No** |
| 20 | C | 47 | ParaHashTableStEph.rs | src/Chap47/ParaHashTableStEph.rs | `LoadAndSize`, `HashTable<K,V,E,M,H>` | Helper + main | **No** |
| 21 | C | 50 | MatrixChainStEph.rs | src/Chap50/MatrixChainStEph.rs | `MatrixDim`, `MatrixChainStEphS` | Helper + main | **No** |
| 22 | C | 50 | MatrixChainStPer.rs | src/Chap50/MatrixChainStPer.rs | `MatrixDim`, `MatrixChainStPerS` | Helper + main | **No** |
| 23 | C | 50 | MatrixChainMtEph.rs | src/Chap50/MatrixChainMtEph.rs | `MatrixDim`, `MatrixChainMtEphS` | Helper + main | **No** |
| 24 | C | 50 | MatrixChainMtPer.rs | src/Chap50/MatrixChainMtPer.rs | `MatrixDim`, `MatrixChainMtPerS` | Helper + main | **No** |
| 25 | C | 50 | OptBinSearchTreeStEph.rs | src/Chap50/OptBinSearchTreeStEph.rs | `KeyProb<T>`, `OBSTStEphS<T>` | Helper + main | **No** |
| 26 | C | 50 | OptBinSearchTreeStPer.rs | src/Chap50/OptBinSearchTreeStPer.rs | `KeyProb<T>`, `OBSTStPerS<T>` | Helper + main | **No** |
| 27 | C | 50 | OptBinSearchTreeMtEph.rs | src/Chap50/OptBinSearchTreeMtEph.rs | `KeyProb<T>`, `OBSTMtEphS<T>` | Helper + main | **No** |
| 28 | C | 50 | OptBinSearchTreeMtPer.rs | src/Chap50/OptBinSearchTreeMtPer.rs | `KeyProb<T>`, `OBSTMtPerS<T>` | Helper + main | **No** |
| 29 | C | 53 | GraphSearchStEph.rs | src/Chap53/GraphSearchStEph.rs | `SearchResult<V>`, `SelectAll`, `SelectOne` | Result + strategies | **No** |
| 30 | C | 53 | GraphSearchStPer.rs | src/Chap53/GraphSearchStPer.rs | `SearchResult<V>`, `SelectAll`, `SelectOne` | Result + strategies | **No** |
| 31 | C | 53 | GraphSearchMtPer.rs | src/Chap53/GraphSearchMtPer.rs | `SearchResult<V>`, `SelectAll`, `SelectOne` | Result + strategies | **No** |
| 32 | C | 54 | BFSStEph.rs | src/Chap54/BFSStEph.rs | `BFSTreeS`, `BFSStEph` | Result + algorithm | **No** |
| 33 | C | 54 | BFSStPer.rs | src/Chap54/BFSStPer.rs | `BFSTreeS`, `BFSStPer` | Result + algorithm | **No** |
| 34 | C | 54 | BFSMtEph.rs | src/Chap54/BFSMtEph.rs | `BFSTreeS`, `BFSMtEph` | Result + algorithm | **No** |
| 35 | C | 54 | BFSMtPer.rs | src/Chap54/BFSMtPer.rs | `BFSTreeS`, `BFSMtPer` | Result + algorithm | **No** |
| 36 | C | 66 | BoruvkaStEph.rs | src/Chap66/BoruvkaStEph.rs | `LabeledEdge<V>`, `BoruvkaStEph` | Helper + algorithm | **No** |
| 37 | C | — | Types.rs | src/Types.rs | `Triple`, `Quadruple`, `KeyVal`, `Edge`, etc. | Unrelated types | **N/A** |

## Already Conforming

| # | Location | File | Full Path | Notes |
|---|----------|------|-----------|-------|
| 1 | std | multi_struct_standard.rs | src/standards/multi_struct_standard.rs | Reference impl |
| 2 | std | arc_rwlock_coarse_standard.rs | src/standards/arc_rwlock_coarse_standard.rs | Reference impl with Arc<RwLock> |
| 3 | exp | tree_module_style.rs | src/experiments/tree_module_style.rs | Original experiment |

## Summary

| Category | Count | Description |
|----------|-------|-------------|
| A: Struct-in-enum | 8 | 6 need work, 1 partial, 1 N/A |
| B: Node + Wrapper | 10 | All need per-type traits |
| C: Multi-struct non-tree | 19 | Helper + main pattern |
| Conforming | 3 | Standards + experiment |
| **Total needing work** | **37** | |

## Notes

- **Category A** files are the highest priority — they have the exact pattern the standard describes.
- **Category B** files (Node + Wrapper) need `NodeTrait` added alongside the existing wrapper trait.
- **Category C** is the broadest category — many are helper+main pairs (MatrixDim+MatrixChain, KeyProb+OBST, BFSTreeS+BFS, etc.) where per-type traits may be overkill.
- **Types.rs** (#37) is a collection of unrelated types, not a module with structurally-related types.
- **HFSchedulerMtEph.rs** (#8) is a concurrency utility, not a data structure module.
- **FlatHashTable.rs** (#7) enum represents entry states, not tree variants.
