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

### New invariant: View-generation (`spec_set_pair_view_generated`)

Every element in the BST's set has a concrete `Pair<K,V>` preimage under View. This is
a sound, provable property: BSTs built from Pair operations only insert `p@` for concrete
Pair values. Subsets, unions, and inserts all preserve the property.

| # | Chap | File | Description |
|---|------|------|-------------|
| 3 | 43 | OrderedTableStEph.rs | Added `spec_set_pair_view_generated` spec + 3 helper lemmas |
| 4 | 43 | OrderedTableStPer.rs | Added `spec_set_pair_view_generated` spec |
| 5 | 43 | OrderedTableStEph.rs | Added to `spec_orderedtablesteph_wf` |
| 6 | 43 | OrderedTableStPer.rs | Added to `spec_orderedtablestper_wf` and `find_pre` |

Maintained through 9 functions (tabulate, map, intersection, union in both StEph/StPer,
plus from_sorted_entries in StPer) by adding loop invariants and `lemma_view_gen_insert`
calls after each BST insert.

### Annotation updates (O(n) → O(lg n))

| # | Chap | File | Functions updated |
|---|------|------|-------------------|
| 7 | 43 | OrderedTableStEph.rs | find, lookup, find_iter, insert, delete_fn |
| 8 | 43 | OrderedTableStPer.rs | find, find_iter, insert, delete_fn |
| 9 | 43 | OrderedTableMtEph.rs | find, lookup |
| 10 | 43 | OrderedTableMtPer.rs | find |
| 11 | 43 | AugOrderedTableStEph.rs | find |
| 12 | 43 | AugOrderedTableStPer.rs | find |
| 13 | 43 | AugOrderedTableMtEph.rs | find |

### StPer find_pre strengthened

`spec_orderedtablestper_find_pre` now includes `spec_pair_key_determines_order`,
`view_ord_consistent::<Pair<K,V>>()`, `obeys_cmp_spec::<Pair<K,V>>()`, and
`spec_set_pair_view_generated`. These were already in `wf`, so all callers with `wf`
are automatically satisfied.

## Proof technique

The BST descent compares `k` with `root_pair.0` at each node via `expose()`. The proof
for the None (not found) case works by contradiction:

1. Assume `spec_pair_set_to_map(tree@).contains_key(k@)`
2. Get witness `vv` via `lemma_map_contains_pair_in_set`
3. Eliminate left subtree (recursive postcondition), root (key mismatch)
4. Conclude the pair is in right subtree: `right@.contains((k@, vv))`
5. Use View-generation (not surjectivity!) to get a Pair preimage: right@ is
   View-generated (subset of tree@, which is View-generated from wf), so
   `exists|p: Pair| p@ == (k@, vv)`. Choose the witness `p_wit`.
6. Assert `right@.contains(p_wit@)` — triggers the expose quantifier
7. Expose quantifier fires: `p_wit.cmp_spec(&root_pair) == Greater`
8. But `lemma_cmp_equal_congruent` + key ordering gives `p_wit.cmp_spec(&root_pair) == Less`
9. Contradiction: `Less != Greater`

The critical distinction from View surjectivity: we only need preimages for elements
actually IN the set, not for all possible view values. This is sound for all types
(NonZeroU64, Vec-backed sequences, etc.) because only concrete values can be inserted.

## Validation

```
verification results:: 5473 verified, 0 errors
RTT: 3583 tests run: 3583 passed, 0 skipped
PTT: 221 tests run: 221 passed, 0 skipped
```
