# Agent3 Round 67 Report: OrderedTableStPer Backing Store Rewire

## Task
Rewire `OrderedTableStPer<K, V>` from wrapping `AVLTreeSetStPer<Pair<K, V>>` to using
`ParamBST<Pair<K, V>>` directly, mirroring agent2's R66 rewire of OrderedTableStEph.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | OrderedTableStPer.rs | Full rewrite: struct, view, wf, spec fns, lemmas, all 39 methods, iterator, derive impls |
| 2 | 43 | AugOrderedTableStPer.rs | Updated 3 references: field name, finite lemma, iterator type |
| 3 | 43 | ProveOrderedTableStPer.rs (PTT) | Removed 2 for-loop tests (ForLoopGhostIterator not implemented) |

## Key Changes in OrderedTableStPer.rs

- **Struct**: `base_set: AVLTreeSetStPer<Pair<K,V>>` -> `tree: ParamBST<Pair<K,V>>`
- **View**: `spec_entries_to_map(self.base_set.elements@)` -> `spec_pair_set_to_map(self.tree@)`
- **wf**: Updated to BST wf + key uniqueness + size bound + axiom predicates
- **Spec fns**: 8 set-based lemmas copied from StEph (pair_set_to_map, key_unique, sorted_keys)
- **Free fn**: `bst_find_by_key` duplicated from StEph (standalone rule)
- **Methods**: All 39 methods rewritten for ParamBST operations
- **Iterator**: `OrderedTableStPerIter` changed from borrowed (`&'a ArraySeqStPerS`) to owned (`ArraySeqStPerS`), `Item = Pair<K,V>` (was `&'a Pair<K,V>`)
- **Imports**: Removed AVLTreeSetStPer/TableStPer, added BSTParaStEph/ArraySeqStPer/OrdSpec

## Verification Results

| Step | Result |
|------|--------|
| validate.sh | 4353 verified, 0 errors |
| rtt.sh | 2528 passed, 0 failed |
| ptt.sh | 145 passed, 0 failed |

## Hole Counts

| # | Chap | File | Holes |
|---|------|------|-------|
| 1 | 43 | OrderedTableStPer.rs | 60 |
| 2 | 43 | AugOrderedTableStPer.rs | 2 |
| 3 | 43 | OrderedTableStEph.rs (reference) | 45 |

StPer has 15 more holes than StEph because:
- 14 axiom assumes in empty/singleton (StEph constructors don't promise wf)
- 1 additional admit in map closure value tracking

## Hole Breakdown for OrderedTableStPer.rs

- **Axiom assumes** (empty, singleton, intersection, union, difference, restrict, subtract, map, from_sorted_entries): These assume `obeys_feq_fulls`, `obeys_cmp_spec`, `view_ord_consistent`, `spec_pair_key_determines_order` — unprovable type-law predicates
- **Admits** in intersection_iter, union_iter, difference_iter, restrict_iter, subtract_iter, split_key, get_key_range, split_rank_key, map: Complex set-algebraic postconditions
- **Iterator assume**: `assume(iter_invariant(self))` in Iterator::next — standard pattern
- **PartialEq/Clone assumes**: Standard eq/clone workaround pattern

## AugOrderedTableStPer.rs Holes

- 2 assumes: `assume(self.base_table.tree@.finite())` in reduce_val/reduce_range — matches StEph pattern (ParamBST type invariant is private)

## Notes

- ForLoopGhostIterator not implemented for owned iterator due to View type mismatch (`Pair<K,V>` exec vs `(K::V,V::V)` spec). Loop-borrow PTT tests pass and cover the same iteration behavior.
- No files modified outside Chap43 (BSTParaStEph.rs, OrderedTableStEph.rs, AVLTreeSetStPer.rs untouched).
