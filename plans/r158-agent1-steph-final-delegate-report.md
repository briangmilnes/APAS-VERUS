# R158 Agent 1: OrderedTableStEph Final Delegation to OrdKeyMap

## Summary

Delegated 8 OrderedTableStEph methods to OrdKeyMap, reducing OrderedTableStEph.rs
from 2738 to 2176 lines (-562 lines, -20.5%).

## Verification

- **Full validate**: 5756 verified, 0 errors, 0 trigger warnings
- **RTT**: 3776 passed, 0 failed
- **PTT**: skipped per instructions

## Methods Delegated

| # | Chap | File | Method | Lines Removed | Delegated To |
|---|------|------|--------|--------------|--------------|
| 1 | 43 | OrderedTableStEph.rs | domain | ~55 | OrdKeyMap::domain |
| 2 | 43 | OrderedTableStEph.rs | tabulate | ~140 | OrdKeyMap::tabulate |
| 3 | 43 | OrderedTableStEph.rs | map | ~115 | OrdKeyMap::map_values |
| 4 | 43 | OrderedTableStEph.rs | filter | ~58 | OrdKeyMap::filter |
| 5 | 43 | OrderedTableStEph.rs | intersection | ~205 | OrdKeyMap::intersect_with |
| 6 | 43 | OrderedTableStEph.rs | union | ~388 | OrdKeyMap::union_with |
| 7 | 43 | OrderedTableStEph.rs | collect | ~30 | OrdKeyMap::collect + from_vec |
| 8 | 43 | OrderedTableStEph.rs | clone | ~3 | OrdKeyMap::clone |

## Proof Hints Required

- **union**: Needed explicit proof block to bridge the "both keys" existential postcondition.
  OrdKeyMap::union_with triggers on `combined@[k]` but OrderedTable triggers on
  `old(self)@.contains_key(k)`. Ghost capture of `old_tree` + `#[trigger]` annotation
  + explicit `let ghost _v = combined@[k]` resolved it.
- **intersection, union, collect**: Needed `lemma_pair_set_to_map_dom_finite(self.tree.inner@)`
  for the `self@.dom().finite()` postcondition.
- **collect**: Needed `lemma_pair_set_to_map_len` to prove `entries@.len() < usize::MAX` for
  `AVLTreeSeqStPerS::from_vec`.

## Methods Not Delegated

| # | Chap | File | Method | Reason |
|---|------|------|--------|--------|
| 1 | 43 | OrderedTableStEph.rs | restrict | OrdKeyMap::restrict requires keys.spec_arraysetsteph_wf(); trait doesn't |
| 2 | 43 | OrderedTableStEph.rs | subtract | Same as restrict |
| 3 | 43 | OrderedTableStEph.rs | reduce | Different semantics: OrdKeyMap Fn(&V,&V)->V vs OrderedTable Fn(R,&K,&V)->R |
| 4 | 43 | OrderedTableStEph.rs | singleton | No OrdKeyMap::singleton; would need new+insert which requires obeys_view_eq |
| 5 | 43 | OrderedTableStEph.rs | iter/into_iter | Needs in_order traversal — structural BST operation |
| 6 | 43 | OrderedTableStEph.rs | from_sorted_entries | Free function building OrdKeyMap from entries — no matching OrdKeyMap op |

## Dead Proof Lemmas (Zero External Callers)

These lemmas are now only called from the bypassed `#[cfg(never)]` union block:

| # | Chap | File | Lemma | Previously Called From |
|---|------|------|-------|----------------------|
| 1 | 43 | OrderedTableStEph.rs | lemma_view_gen_insert | tabulate, map, intersection, union |
| 2 | 43 | OrderedTableStEph.rs | lemma_key_unique_subset | filter |

Note: `lemma_pair_set_to_map_len` is still alive — used in the new collect proof block.

## Remaining self.tree.inner References

After delegation, 25 `self.tree.inner` references remain outside the bypassed union block:
- **reduce**: 2 (in_order traversal + finite proof)
- **intersection/union/difference proofs**: 3 (lemma_pair_set_to_map_dom_finite)
- **restrict**: 3 (full manual implementation)
- **subtract**: 3 (full manual implementation)
- **collect proof**: 3 (lemma_pair_set_to_map_len + dom_finite)
- **split_key_iter proof**: 1 (dom_finite)
- **iter/into_iter**: 4 (in_order + len)
- **singleton/from_sorted_entries**: 2 (OrdKeyMap { inner: ... } construction)

## Line Count

| State | Lines |
|-------|-------|
| Before | 2738 |
| After | 2176 |
| Delta | -562 |
