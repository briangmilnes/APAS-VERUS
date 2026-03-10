# b: bool Return Name Audit (2026-03-08)

Convention: named return values should describe what the boolean means, not use generic `b`.

Excludes: experiments, vstdplus.

## All `(b: bool)` Return Types

| # | Chap | File | Function | Ensures | Strength | Rename to |
|---|------|------|----------|---------|----------|-----------|
| 1 | 06 | LabDirGraphStEph.rs | `has_arc` | `b == (exists \|l\| ...)` | Strong | `has_arc` |
| 2 | 06 | LabDirGraphMtEph.rs | `has_arc` | `b == (exists \|l\| ...)` | Strong | `has_arc` |
| 3 | 06 | LabUnDirGraphStEph.rs | `has_edge` | `b == (exists \|l\| ...)` | Strong | `has_edge` |
| 4 | 06 | LabUnDirGraphMtEph.rs | `has_edge` | `b == (exists \|l\| ...)` | Strong | `has_edge` |
| 5 | 23 | BalBinTreeStEph.rs | `is_leaf` | `b == (spec_size() == 0)` | Strong | `is_leaf` |
| 6 | 37 | BSTPlainStEph.rs | `is_empty` | `b == (spec_size() == 0)` | Strong | `is_empty` |
| 7 | 37 | BSTPlainMtEph.rs | `is_empty` | `ensures true` | **None** | `is_empty` |
| 8 | 37 | BSTAVLStEph.rs | `is_empty` | `b == (spec_size() == 0)` | Strong | `is_empty` |
| 9 | 37 | BSTAVLMtEph.rs | `is_empty` | `ensures true` | **None** | `is_empty` |
| 10 | 37 | BSTBBAlphaStEph.rs | `is_empty` | `b == (spec_size() == 0)` | Strong | `is_empty` |
| 11 | 37 | BSTBBAlphaMtEph.rs | `is_empty` | `ensures true` | **None** | `is_empty` |
| 12 | 37 | BSTRBStEph.rs | `is_empty` | `b == (spec_size() == 0)` | Strong | `is_empty` |
| 13 | 44 | DocumentIndex.rs | `eq` | `b == (self == other)` | Strong | `is_equal` |
| 14 | 45 | BalancedTreePQ.rs | `is_empty` | `b == (len() == 0)` | Strong | `is_empty` |
| 15 | 45 | SortedListPQ.rs | `is_empty` | `b == (spec_size() == 0)` | Strong | `is_empty` |
| 16 | 45 | UnsortedListPQ.rs | `is_empty` | `b == (spec_size() == 0)` | Strong | `is_empty` |
| 17 | 45 | LeftistHeapPQ.rs | `is_leftist` | `leaf ==> b` | **Weak** | `is_leftist` |
| 18 | 45 | LeftistHeapPQ.rs | `is_heap` | `leaf ==> b` | **Weak** | `is_heap` |
| 19 | 45 | LeftistHeapPQ.rs | `is_empty` | `b == (spec_size() == 0)` | Strong | `is_empty` |
| 20 | 45 | LeftistHeapPQ.rs | `is_valid_leftist_heap` | `size==0 ==> b` | **Weak** | `is_valid` |
| 21 | 56 | SSSPResultStEphI64.rs | `is_reachable` | case-split biconditional | Strong | `is_reachable` |
| 22 | 56 | SSSPResultStEphF64.rs | `is_reachable` | case-split biconditional | Strong | `is_reachable` |
| 23 | 56 | SSSPResultStPerI64.rs | `is_reachable` | case-split biconditional | Strong | `is_reachable` |
| 24 | 56 | AllPairsResultStEphI64.rs | `is_reachable` | case-split biconditional | Strong | `is_reachable` |
| 25 | 56 | AllPairsResultStPerI64.rs | `is_reachable` | case-split biconditional | Strong | `is_reachable` |

## Summary

| Metric | Count |
|--------|-------|
| Total `(b: bool)` functions | 25 |
| Strong specs (`b == expr`) | 19 |
| Weak specs (one-directional `==>`) | 3 |
| No spec (`ensures true`) | 3 |
| Files affected | 18 |
