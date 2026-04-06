# R149 Agent 2 Report: Add Missing Debug/Display Impls (Chap23-Chap43)

## Summary

Added missing `impl Debug` and `impl Display` outside `verus!` for structs flagged
by veracity rule [14] across 17 files in Chap23, Chap26, Chap37, Chap38, Chap39,
Chap40, and Chap43.

## Changes by Chapter

| # | Chap | File | Structs Fixed | Impls Added |
|---|------|------|---------------|-------------|
| 1 | 23 | BalBinTreeStEph.rs | BalBinTree, BalBinNode, InOrderIter, PreOrderIter, PostOrderIter, InOrderGhostIterator, PreOrderGhostIterator, PostOrderGhostIterator | 14 (Display for tree/node, Debug+Display for 3 iters + 3 ghost iters) |
| 2 | 23 | PrimTreeSeqStPer.rs | PrimTreeSeqStS, PrimTreeSeqStTree, PrimTreeSeqStIter, PrimTreeSeqStGhostIterator | 8 (Display for 2 types, Debug+Display for iter + ghost iter) |
| 3 | 26 | ETSPStEph.rs | Point, Edge | 4 (Debug+Display for each) |
| 4 | 26 | ETSPMtEph.rs | Point, Edge | 4 (Debug+Display for each) |
| 5 | 37 | AVLTreeSeqMtPer.rs | AVLTreeSeqMtPerBorrowIter, AVLTreeSeqMtPerGhostIterator | 4 (Debug+Display for each) |
| 6 | 38 | BSTParaMtEph.rs | BSTParaMtEphInv, Exposed, NodeInner, ParamBST | 8 (Debug+Display for Inv+Exposed, Display for NodeInner+ParamBST) |
| 7 | 38 | BSTParaStEph.rs | BSTParaStEphInv, Exposed, NodeInner, ParamBST | 5 (Debug+Display for Inv, Display for Exposed+NodeInner+ParamBST) |
| 8 | 39 | BSTTreapStEph.rs | ExposedTreap | 2 (Debug+Display) |
| 9 | 40 | BSTKeyValueStEph.rs | Lnk | 2 (Debug+Display) |
| 10 | 40 | BSTReducedStEph.rs | Lnk | 2 (Debug+Display) |
| 11 | 40 | BSTSizeStEph.rs | Lnk | 2 (Debug+Display) |
| 12 | 43 | OrderedSetMtEph.rs | OrderedSetMtEphInv | 2 (Debug+Display) |
| 13 | 43 | OrderedSetStEph.rs | OrderedSetStEphIter, OrderedSetStEphGhostIterator | 4 (Debug+Display for each) |
| 14 | 43 | OrderedSetStPer.rs | OrderedSetStPerIter, OrderedSetStPerGhostIterator | 4 (Debug+Display for each) |
| 15 | 43 | OrderedTableMtEph.rs | OrderedTableMtEphInv | 2 (Debug+Display) |
| 16 | 43 | OrderedTableMtPer.rs | OrderedTableMtPerInv | 2 (Debug+Display) |
| 17 | 43 | OrderedTableStEph.rs | OrderedTableStEphGhostIterator | 2 (Debug+Display) |
| 18 | 43 | OrderedTableStPer.rs | OrderedTableStPerGhostIterator | 2 (Debug+Display) |

**Total impls added: 73**

## Remaining [14] Warnings (4 — unfixable without verus! changes)

| # | Chap | File | Struct | Reason |
|---|------|------|--------|--------|
| 1 | 23 | PrimTreeSeqStPer.rs | PrimTreeSeqStTreeView | Ghost enum — does not exist at runtime, cannot impl traits outside verus! |
| 2 | 38 | BSTParaStEph.rs | Exposed | Has `#[derive(Debug)]` inside verus! — adding duplicate Debug impl outside would cause conflict |
| 3 | 38 | BSTParaStEph.rs | NodeInner | Has `#[derive(Debug)]` inside verus! — adding duplicate Debug impl outside would cause conflict |

These are veracity false positives: PrimTreeSeqStTreeView is a ghost type (no runtime
representation), and Exposed/NodeInner already have Debug via derive inside verus!.

## Validation

- Chap23: 746 verified, 0 errors
- Chap26: 1169 verified, 0 errors
- Chap37: 1946 verified, 0 errors
- Chap38: 1156 verified, 0 errors
- Chap39: 1295 verified, 0 errors
- Chap40: 1250 verified, 0 errors
- Chap43: 2758 verified, 0 errors
- RTTs: 3690 passed, 0 skipped
