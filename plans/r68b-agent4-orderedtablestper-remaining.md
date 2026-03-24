# R68b Agent 4: OrderedTableStPer Remaining Holes

## Goal

Reduce remaining holes in `src/Chap43/OrderedTableStPer.rs` toward 0. Agent 3 (R68)
already attacked this file. Your job: burn down whatever holes remain.

## Setup

1. Run `scripts/holes.sh src/Chap43/OrderedTableStPer.rs` to see current holes
2. Read `src/Chap43/OrderedTableStPer.rs` — understand what Agent 3 already fixed

## Hole Categories (from pre-Agent-3 baseline, 58 holes)

### Type Axiom Assumes (~36 holes pre-Agent-3)

The 4-pack pattern repeats in many functions:
```
assume(spec_pair_key_determines_order::<K, V>());
assume(vstd::laws_cmp::obeys_cmp_spec::<K>());
assume(view_ord_consistent::<K>());
assume(obeys_feq_fulls::<K, V>());
```
Plus 3-pack variants:
```
assume(obeys_feq_fulls::<K, V>());
assume(obeys_feq_full::<Pair<K, V>>());
assume(vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>());
assume(view_ord_consistent::<Pair<K, V>>());
assume(spec_pair_key_determines_order::<K, V>());
assume(vstd::laws_cmp::obeys_cmp_spec::<K>());
assume(view_ord_consistent::<K>());
```

**Key insight**: `spec_orderedtablestper_wf()` includes all these predicates as conjuncts.
Functions with `requires self.spec_orderedtablestper_wf()` already have them in scope.
Replace assumes with asserts.

For constructors (empty, singleton, from_sorted_entries) with no `&self` — check if
trait requires already include the axioms. If not, those assumes may need to stay.

### Admits (~12 holes pre-Agent-3)

Real proof obligations in: intersection_iter, union_iter, difference_iter, restrict_iter,
subtract_iter, split_key, get_key_range, split_rank_key, map.

**Strategy**: Bridge ParamBST's Set ensures to OrderedTable's Map postconditions via
`spec_pair_set_to_map`. The proof chain:
1. ParamBST operation produces `result_tree@` (a Set)
2. Prove `spec_pair_set_to_map(result_tree@) == expected_map_result`
3. Prove `spec_key_unique_pairs_set(result_tree@)` (key uniqueness preserved)

Write bridge lemmas for pair-set-to-map connections:
- Intersection: pair_set_to_map(s1 ∩ s2) == map1 ∩ map2 (when keys unique)
- Difference: pair_set_to_map(s1 \ s2) == map1.remove_keys(map2.dom())
- Union: pair_set_to_map(s1 ∪ s2) — more complex due to value precedence

**Reference**: Read `src/Chap38/BSTParaStEph.rs` for ParamBST ensures on split, union,
intersect, difference. Also look at `src/Chap43/OrderedTableStEph.rs` — same proof
pattern, may have bridge lemmas you can adapt.

### Permanent Holes

- `assume(iter_invariant(self))` — standard iterator pattern, leave
- Clone/PartialEq assumes — standard workaround, leave
- `fn_missing_wf_ensures` on `from_sorted_entries` — add `ensures result.spec_orderedtablestper_wf()`

## Approach

1. Run `scripts/holes.sh` to see what Agent 3 left
2. Read the file, understand remaining holes
3. Phase 1: Eliminate any remaining type axiom assumes (mechanical)
4. Phase 2: Attack remaining admits (real proofs)
5. Validate, rtt, ptt sequentially

## Constraints

- Do NOT modify BSTParaStEph.rs, OrderedTableStEph.rs, or any file outside Chap43.
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- Standard pattern holes (iterator, eq/clone) stay.
- Run validate, rtt, ptt sequentially.
