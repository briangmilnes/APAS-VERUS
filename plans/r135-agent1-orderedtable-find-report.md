# R135 Agent 1 Report — Fix OrderedTable find: O(n) → O(lg n)

## Summary

Rewrote `bst_find_by_key` in `OrderedTableStEph.rs` and `OrderedTableStPer.rs` from
O(n) linear scan (in_order + loop) to O(lg n) recursive BST descent using `expose()`.

## Changes

### Core algorithm change

| # | Chap | File | Description |
|---|------|------|-------------|
| 1 | 43 | OrderedTableStEph.rs | New `bst_find_by_key` using recursive BST descent via `expose()` |
| 2 | 43 | OrderedTableStPer.rs | Same change, plus `find_pre` updated with Pair axioms |

### New axiom: View surjectivity

| # | File | Description |
|---|------|-------------|
| 3 | vstdplus/feq.rs | Added `spec_view_has_preimage`, `spec_view_surjective`, and broadcast `axiom_view_surjective` |

The proof requires bridging from Pair-level BST quantifiers (from `expose()`) to view-level
map properties. The expose postcondition `forall|t: Pair<K,V>| right@.contains(t@) ==> ...`
quantifies over exec-level Pair values, but the map-level conclusion needs view-level
(K::V, V::V) reasoning. The bridge requires View surjectivity: given a view-level value
`vv: V::V`, there exists a `V` with `v@ == vv`. This is axiomatized via `admit()` in the
`group_feq_axioms` broadcast group, consistent with the existing feq/view_eq axiom pattern.

### Annotation updates (O(n) → O(lg n))

| # | Chap | File | Functions updated |
|---|------|------|-------------------|
| 4 | 43 | OrderedTableStEph.rs | find, lookup, find_iter, insert, delete_fn |
| 5 | 43 | OrderedTableStPer.rs | find, find_iter, insert, delete_fn |
| 6 | 43 | OrderedTableMtEph.rs | find, lookup |
| 7 | 43 | OrderedTableMtPer.rs | find |
| 8 | 43 | AugOrderedTableStEph.rs | find |
| 9 | 43 | AugOrderedTableStPer.rs | find |
| 10 | 43 | AugOrderedTableMtEph.rs | find |

### StPer find_pre strengthened

`spec_orderedtablestper_find_pre` now includes `spec_pair_key_determines_order`,
`view_ord_consistent::<Pair<K,V>>()`, and `obeys_cmp_spec::<Pair<K,V>>()` — required
for the BST descent proof. These were already in `wf`, so all callers with `wf` are
automatically satisfied.

## Proof technique

The BST descent compares `k` with `root_pair.0` at each node via `expose()`. The proof
for the None (not found) case works by contradiction:

1. Assume `spec_pair_set_to_map(tree@).contains_key(k@)`
2. Get witness `vv` via `lemma_map_contains_pair_in_set`
3. Eliminate left subtree (recursive postcondition), root (key mismatch)
4. Conclude the pair is in right subtree: `right@.contains((k@, vv))`
5. Use View surjectivity to get `v_wit: V` with `v_wit@ == vv`
6. Construct ghost Pair: `p_wit = Pair(*k, v_wit)`, assert `right@.contains(p_wit@)`
7. Expose quantifier trigger fires: `p_wit.cmp_spec(&root_pair) == Greater`
8. But `lemma_cmp_equal_congruent` + key ordering gives `p_wit.cmp_spec(&root_pair) == Less`
9. Contradiction

## Validation

```
verification results:: 5471 verified, 0 errors
RTT: 3583 tests run: 3583 passed, 0 skipped
PTT: 221 tests run: 221 passed, 0 skipped
```
