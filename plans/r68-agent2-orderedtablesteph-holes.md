# R68 Agent 2: OrderedTableStEph Hole Burndown

## Goal

Reduce holes in `src/Chap43/OrderedTableStEph.rs` from 43 toward 0. Two categories:
type axiom assumes (mechanical elimination) and admits (real proof work).

## Current Holes (43)

**31 assumes**, **12 admits**.

### Category 1: Redundant Type Axiom Assumes (~28 holes)

The 4-pack pattern appears in 7 functions:
```
assume(spec_pair_key_determines_order::<K, V>());
assume(vstd::laws_cmp::obeys_cmp_spec::<K>());
assume(view_ord_consistent::<K>());
assume(obeys_feq_fulls::<K, V>());
```

These appear in: `tabulate`, `map`, `filter`, `intersection`, `difference`, `restrict`,
`subtract`.

**Key insight**: `spec_orderedtablesteph_wf()` already includes ALL FOUR predicates as
conjuncts. Any function with `requires self.spec_orderedtablesteph_wf()` has them in
scope. For `&self`/`&mut self` methods, these assumes should be replaceable with:

```rust
proof {
    // Type axioms flow from self.spec_orderedtablesteph_wf()
    assert(spec_pair_key_determines_order::<K, V>());
    assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
    assert(view_ord_consistent::<K>());
    assert(obeys_feq_fulls::<K, V>());
}
```

If the assert doesn't go through directly, try:
```rust
proof {
    let wf = self.spec_orderedtablesteph_wf();
    // Unfold wf to extract conjuncts.
}
```

**Exception**: `tabulate` takes `(f, keys)` not `&self` — no wf in scope. Those assumes
may stay. Check the trait signature for what requires are available.

### Category 2: Admits (12 holes)

These are real proof obligations. By function:

| # | Function | Line | What needs proving |
|---|----------|------|--------------------|
| 1 | map | 1542 | Map view correctness: mapped result contains all original keys with f-applied values |
| 2 | intersection | 1733 | Map domain = self.dom() ∩ other.dom(), values correct |
| 3 | union | 1912 | Map domain = self.dom() ∪ other.dom(), values correct |
| 4 | difference | 1995 | Map domain = self.dom() \ other.dom() |
| 5 | restrict | 2075 | Map domain = self.dom() ∩ keys@ |
| 6 | subtract | 2154 | Map domain = self.dom() \ keys@ |
| 7 | first_key | 2231 | TotalOrder::le connection to BST min_key |
| 8 | split_key | 2666 | Left/right domain partition correct |
| 9 | get_key_range | 2748 | Range domain correct |
| 10 | rank_key | 2791 | Count = dom.filter(|k| le(k,x)).len() |
| 11 | select_key | 2845 | Rank/domain proofs |
| 12 | split_rank_key | 2954 | Left/right partition by rank |

**Strategy for admits**: The core challenge is bridging ParamBST's Set ensures to
OrderedTable's Map postconditions via `spec_pair_set_to_map`. Key lemma needed:

For intersection/union/difference: ParamBST already ensures the correct Set result. You
need to prove that `spec_pair_set_to_map(result_set)` equals the expected Map operation.
Write bridge lemmas like:
- `lemma_pair_set_to_map_intersection` — if set = s1 ∩ s2 and keys unique, then map = m1 ∩ m2
- `lemma_pair_set_to_map_union` — similar
- `lemma_pair_set_to_map_difference` — similar

For navigation ops (first_key, split_key, etc.): ParamBST's split ensures give you
left/right sets with ordering. Bridge to Map domain properties.

### Category 3: Other (3 holes)

- `assume(iter_invariant(self))` in Iterator::next — standard pattern, leave as-is
- `assume(obeys_cmp_spec/view_ord_consistent)` in `from_sorted_entries` — constructor,
  no wf in scope, likely must stay
- `fn_missing_wf_ensures` on `from_sorted_entries` — add `ensures result.spec_orderedtablesteph_wf()`

## Approach

1. **Read** OrderedTableStEph.rs thoroughly — understand the wf predicate and what it includes
2. **Read** BSTParaStEph.rs — understand what ensures ParamBST provides for split, union,
   intersection, difference
3. **Phase 1**: Eliminate redundant axiom assumes in methods that have wf in requires (~28 holes)
4. **Phase 2**: Add `ensures wf` to `from_sorted_entries` (1 hole)
5. **Phase 3**: Attack admits starting with the easiest (difference, restrict, subtract —
   simpler set operations) then harder (intersection, union — value merging)
6. **Validate** after each phase

## Constraints

- Do NOT modify BSTParaStEph.rs, OrderedTableStPer.rs, or any file outside Chap43.
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures to make proofs easier.
- Iterator assume stays (standard pattern).
- Run validate, rtt, ptt sequentially.
