# veracity-compare-par-mut: False positives on View type comparison

## Bug

The tool reports 7 View mismatch errors that are false positives. In each case,
the tool compares the Mt variant's main struct View against the wrong View impl
in the St file — picking up the iterator, GhostIterator, or LockedX wrapper's
View instead of the main data structure's View.

## Examples

```
src/Chap05/SetMtEph.rs:998: error: View = Set<T::V> but StEph has View = Seq<T>
```
SetStEph's main struct has `View = Set<T::V>` (line 79). The `Seq<T>` comes from
SetStEphBorrowIter's View (line 874). The tool matched against the wrong impl.

```
src/Chap06/DirGraphMtEph.rs:829: error: View = GraphView<V::V> but StEph has View = Seq<V>
```
DirGraphStEph's main struct has `View = GraphView<V::V>`. The `Seq<V>` is from
the iterator's GhostIter View.

```
src/Chap41/AVLTreeSetMtEph.rs:96: error: View = Seq<T::V> but StPer has View = Set<T::V>
```
Line 96 is AVLTreeSetMtEphGhostIter, not the main struct. Main struct View at
line 84 is `Set<T::V>`, matching StPer.

## All 7 false positives

| # | Reported error | Line is actually | Real main View |
|---|---------------|-----------------|----------------|
| 1 | SetMtEph:998 | LockedSetMtEph | SetStEph: Set<T::V> ✓ |
| 2 | DirGraphMtEph:829 | LockedDirGraphMtEph | DirGraphStEph: GraphView<V::V> ✓ |
| 3 | LabDirGraphMtEph:743 | LockedLabDirGraphMtEph | LabDirGraphStEph: LabGraphView ✓ |
| 4 | LabUnDirGraphMtEph:700 | LockedLabUnDirGraphMtEph | LabUnDirGraphStEph: LabGraphView ✓ |
| 5 | UnDirGraphMtEph:586 | LockedUnDirGraphMtEph | UnDirGraphStEph: GraphView ✓ |
| 6 | OrderedSetMtEph:92 | GhostIter or inner | OrderedSetStPer: Set<T::V> ✓ |
| 7 | OrderedTableMtPer:99 | LockedOrderedTableMtPer | OrderedTableStPer: Map<K::V,V::V> ✓ |

## Root cause

Mt files have multiple structs with View impls: the inner data struct, the
LockedX wrapper, iterators, and GhostIterators. The tool needs to identify
which View impl belongs to the **main exported struct** (the one named in the
trait) and compare only that one against the St counterpart.

## Suggested fix

When matching View impls across St/Mt files, filter to the struct whose name
matches the module's primary type pattern:
- `FooStEph` / `FooMtEph` / `FooStPer` / `FooMtPer`

Skip View impls on structs named `*Iter`, `*GhostIter`, `Locked*`, `*Inner`,
`*Inv`. These are infrastructure types with intentionally different Views.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.
