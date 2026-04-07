# R152 Agent 2 Report: OrdKeyMap<K,V> in Chap38

## Summary

Built `src/Chap38/OrdKeyMap.rs` — an ordered key-value map backed by
`ParamBST<Pair<K,V>>` with `View = Map<K::V, V::V>`. This is the bridge layer
between ParamBST's `Set<(K::V, V::V)>` view and the `Map<K::V, V::V>` view
that OrderedTable needs. The module lives in Chap38, shared by all consumers.

## What was built

| # | Chap | File | Component | Lines | Status |
|---|------|------|-----------|-------|--------|
| 1 | 38 | OrdKeyMap.rs | struct + View impl | 12 | verified |
| 2 | 38 | OrdKeyMap.rs | 4 bridge spec fns | 30 | verified |
| 3 | 38 | OrdKeyMap.rs | 17 bridge proof lemmas | 470 | verified |
| 4 | 38 | OrdKeyMap.rs | OrdKeyMapTrait (7 methods) | 50 | verified |
| 5 | 38 | OrdKeyMap.rs | ordkeymap_find (recursive) | 150 | verified |
| 6 | 38 | OrdKeyMap.rs | ordkeymap_split (recursive) | 340 | verified |
| 7 | 38 | OrdKeyMap.rs | impl (new/size/is_empty/find/insert/delete/split) | 250 | verified |
| 8 | 38 | OrdKeyMap.rs | Debug/Display outside verus! | 12 | compiled |

Total: 1,467 lines.

## Trait methods implemented

| # | Method | Ensures (Map-level) | Delegates to |
|---|--------|---------------------|--------------|
| 1 | `new()` | `self@ == Map::empty()` | `ParamBST::new()` |
| 2 | `size()` | `count == self@.dom().len()` | `self.inner.size()` |
| 3 | `is_empty()` | `is_empty == self@.dom().is_empty()` | `self.inner.is_empty()` |
| 4 | `find(&self, k)` | `Some(v) => self@[k@] == v@` | `ordkeymap_find` (BST descent) |
| 5 | `insert(&mut self, k, v)` | `self@[k@] == v@, dom insert` | find + BST delete/insert |
| 6 | `delete(&mut self, k)` | `self@ == old@.remove(k@)` | find + BST delete |
| 7 | `split(&self, k)` | left/right partition + found | `ordkeymap_split` (BST descent) |

## Bridge lemmas copied from OrderedTableStEph

All 17 bridge lemmas were copied (not moved — OrderedTableStEph retains its copies
per the rules). These are:

- `lemma_pair_set_to_map_dom_finite`, `lemma_pair_set_to_map_len`
- `lemma_pair_in_set_map_contains`, `lemma_map_contains_pair_in_set`
- `lemma_key_unique_insert`, `lemma_key_unique_remove`, `lemma_key_unique_subset`,
  `lemma_key_unique_empty`, `lemma_key_unique_disjoint_union`
- `lemma_set_to_map_union_root`, `lemma_set_to_map_insert`, `lemma_set_to_map_remove_pair`,
  `lemma_set_to_map_empty`
- `lemma_view_gen_subset`, `lemma_view_gen_insert`, `lemma_view_gen_union`
- `lemma_cmp_equal_congruent`, `lemma_sorted_keys_pairwise_distinct`
- `lemma_cmp_antisymmetry`

## Not implemented this round

- `union`, `intersect`, `difference` — these need split-based recursive
  implementations with value-combining functions, not simple ParamBST delegation.
  ~250+ lines each with complex proofs.
- `next`, `prev`, `rank`, `select` — Phase 3 stretch items.
- OrderedTable migration (Phase 4) — explicitly deferred per prompt.

## Verification

- Isolate Chap38: 1,184 verified, 0 errors (10s)
- Full validation: 5,730 verified, 0 errors (115s)
- RTTs: 3,690 passed, 0 skipped
- No assumes, accepts, or external_body added.
- No modifications to OrderedTableStEph or any Chap43 file.

## Design decisions

1. **wf predicate mirrors OrderedTableStEph's**: includes BST wf, key uniqueness,
   size bound, feq/cmp/view axioms, pair key order, view generation. This means
   consumers can rely on OrdKeyMap wf without re-stating these preconditions.

2. **insert replaces unconditionally**: unlike OrderedTable's insert which takes a
   `combine: F` for duplicate keys, OrdKeyMap's insert always replaces the value.
   This is the simpler Map semantics. OrderedTable can add combine-on-top.

3. **Recursive helper functions**: `ordkeymap_find` and `ordkeymap_split` are free
   functions (not methods) because they recurse on `ParamBST`, not on `OrdKeyMap`.
   The trait methods delegate to them.

4. **obeys_view_eq in requires**: insert and delete require `obeys_view_eq::<K>()`
   because they call find internally, matching OrderedTableStEph's pattern.
