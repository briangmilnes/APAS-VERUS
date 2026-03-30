# R106 Agent 2 — View Conformance Report

## Summary

- **Phase 2 errors (View/return type):** 14 → 12 (2 fixed, 7 false positives, 5 return type — intentional)
- **Phase 4 errors (spec weakening):** 7 → 2 (5 fixed, 2 blocked)
- **Total real errors fixed:** 7
- **Verified count:** 5433 verified, 0 errors
- **RTT:** 3083 passed

## Fixes Applied

| # | Chap | File | Fix |
|---|------|------|-----|
| 1 | 37 | BSTRBMtEph.rs | View Link\<T\> → BalBinTree\<T\>: added link_to_bbt conversion + 3 bridge lemmas |
| 2 | 38 | BSTParaMtEph.rs | insert ensures self@ =~= old(self)@.insert(key@) |
| 3 | 38 | BSTParaMtEph.rs | delete ensures self@ =~= old(self)@.remove(key@) |
| 4 | 41 | AVLTreeSetMtEph.rs | from_seq ensures constructed@ =~= seq@.to_set() |
| 5 | 43 | OrderedSetStEph.rs | filter ensures subset_of + containment + pred |
| 6 | 06 | LabUnDirGraphMtEph.rs | add_labeled_edge ensures V/A update spec |

## False Positives in veracity-compare-par-mut

The tool reports 7 "View mismatch" errors that are **false positives**. In each case, the
St/StPer file already has the correct abstract View type — the tool is comparing the Mt's
LockedX wrapper (or GhostIter) against the wrong St struct.

| # | Reported error | Actual St View | Verdict |
|---|---------------|----------------|---------|
| 1 | SetMtEph:998 "StEph has Seq\<T\>" | SetStEph: Set\<T::V\> | **False positive** |
| 2 | DirGraphMtEph:829 "StEph has Seq\<V\>" | DirGraphStEph: GraphView\<V::V\> | **False positive** |
| 3 | LabDirGraphMtEph:743 "StEph has Seq\<V\>" | LabDirGraphStEph: LabGraphView\<V::V,L::V\> | **False positive** |
| 4 | LabUnDirGraphMtEph:700 "StEph has Seq\<V\>" | LabUnDirGraphStEph: LabGraphView\<V::V,L::V\> | **False positive** |
| 5 | UnDirGraphMtEph:586 "StEph has Seq\<V\>" | UnDirGraphStEph: GraphView\<V::V\> | **False positive** |
| 6 | OrderedSetMtEph:92 "StPer has Seq\<T\>" | OrderedSetStPer: Set\<T::V\> | **False positive** |
| 7 | OrderedTableMtPer:99 "StPer has Seq\<Pair\>" | OrderedTableStPer: Map\<K::V,V::V\> | **False positive** |

**Root cause hypothesis:** The tool's "StEph has View = Seq\<T\>" likely comes from parsing
the Iter or GhostIterator struct's View (which IS Seq-based) instead of the main data
structure's View. All MtEph files have inner+locked structs plus iter+ghost iter; the tool
may be comparing LockedX against the wrong View impl within the St file.

## AVLTreeSetMtEph GhostIter (error #8)

Line 96 is `AVLTreeSetMtEphGhostIter` (View = Seq\<T::V\>), not the main struct
(View = Set\<T::V\> at line 84). **False positive** — GhostIter's View is correctly
Seq-based for iteration tracking.

## Return Type Mismatches (Priority 3 — intentional)

| # | Chap | Error | Reason |
|---|------|-------|--------|
| 1 | 37 | iter returns MtPer vs StPer iter types | Different iterator types per variant |
| 2 | 37 | insert returns Result vs () | Mt uses Result for lock capacity errors |
| 3 | 43 | to_seq returns ArraySeqStPerS vs AVLTreeSeqStPerS | Different backing seq types |
| 4 | 43 | domain returns OrderedSetMtEph vs ArraySetStEph | Different set implementations |

These are intentional — different variants use different backing types.

## Blocked Spec Weakening

| # | Chap | File | Blocker |
|---|------|------|---------|
| 1 | 41 | AVLTreeSetMtPer.rs | from_seq blocked by values_in_order ensures true |
| 2 | 43 | AugOrderedTableMtEph.rs | get_key_range blocked by OrderedTableMtEph's weak spec |

## BSTRBMtEph View Change Details

Changed View from `Link<T> = Option<Box<Node<T>>>` to `BalBinTree<T>`.

Added spec conversion `link_to_bbt(link: Link<T>) -> BalBinTree<T>` that strips color
and size fields, preserving tree structure and keys.

Added 3 bridge lemmas connecting Link-based and BalBinTree-based spec fns:
- `lemma_link_to_bbt_size`: link_spec_size(l) == link_to_bbt(l).spec_size()
- `lemma_link_to_bbt_contains`: link_contains(l, t) == link_to_bbt(l).tree_contains(t)
- `lemma_link_to_bbt_height`: link_height(l) == link_to_bbt(l).spec_height()

Layer 1 (internal rotations, insert_link, find_link, etc.) unchanged — still operates on
Link<T>. Layer 2 (locked wrapper trait) ensures converted to BalBinTree operations.
