# R157 Agent 2 — StEph Union/Intersection Delegation Report

## Summary

Delegated OrderedTableStEph's `union` and `intersection` to OrdKeyMap's `union_with`
and `intersect_with`, using the same trigger bridge technique that agent 3 (R156)
used for OrderedTableStPer. Deleted all dead code left behind.

## Changes

### Task A: Delegate intersection to OrdKeyMap::intersect_with

Replaced ~210-line iterative implementation with 3-line delegation:
```rust
fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F) {
    self.tree = self.tree.intersect_with(&other.tree, &f);
    proof { lemma_pair_set_to_map_dom_finite(self.tree.inner@); }
}
```
No trigger bridge needed — OrdKeyMap's ensures match StEph's directly.

### Task B: Delegate union to OrdKeyMap::union_with

Replaced ~390-line two-phase iterative implementation with trigger bridge delegation:
```rust
fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F) {
    let ghost old_map = self@;
    let ghost other_map = other@;
    let result = self.tree.union_with(&other.tree, &f);
    self.tree = result;
    proof {
        assert forall|k: K::V|
            #[trigger] old_map.contains_key(k) && other_map.contains_key(k) implies
            (exists|v1: V, v2: V, r: V| ...) by {
            let vk = result@[k];
            assert(result@[k] == vk);
        };
        lemma_pair_set_to_map_dom_finite(self.tree.inner@);
    }
}
```
Trigger bridge materializes `result@[k]` to fire Z3's existential instantiation.

### Task C: Dead code cleanup

Deleted 8 proof lemmas only used by the old iterative implementations:
- `lemma_view_gen_subset` — subset of view-generated set
- `lemma_view_gen_union` — union of view-generated sets
- `lemma_cmp_equal_congruent` — cmp_spec equal-substitution
- `lemma_cmp_antisymmetry` — cmp_spec Less→Greater
- `lemma_key_unique_remove` — key uniqueness under set remove
- `lemma_key_unique_disjoint_union` — key uniqueness under disjoint union
- `lemma_set_to_map_remove_pair` — map after set remove
- `lemma_set_to_map_union_root` — map after union+insert

### Task D: Remaining methods

Methods still using `self.tree.inner` directly:
- `domain` — returns ArraySetStEph, no OrdKeyMap equivalent
- `tabulate` — takes ArraySetStEph keys, no equivalent
- `filter` — uses ParamBST::filter with adapter (could delegate to OrdKeyMap::filter but marginal gain)
- `reduce` — table takes `(R, &K, &V) -> R`, OrdKeyMap takes `(&V, &V) -> V` — different semantics
- `collect` — table returns AVLTreeSeqStPerS, OrdKeyMap returns Vec
- `restrict` — takes ArraySetStEph, no OrdKeyMap equivalent
- `subtract` — takes ArraySetStEph, no OrdKeyMap equivalent

These can't trivially delegate due to different parameter/return types.

## Line Count

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| OrderedTableStEph.rs | 3017 | 2183 | -834 |

## Verification

| Step | Result |
|------|--------|
| `validate.sh isolate Chap43` | 2809 verified, 0 errors |
| `validate.sh` (full) | 5753 verified, 0 errors |
| `rtt.sh` | 3752 passed, 0 skipped |
