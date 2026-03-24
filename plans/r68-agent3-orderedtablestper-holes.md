# R68 Agent 3: OrderedTableStPer Hole Burndown

## Goal

Reduce holes in `src/Chap43/OrderedTableStPer.rs` from 58 toward 0. Same two categories
as OrderedTableStEph: type axiom assumes (mechanical) and admits (proof work).

## Current Holes (58)

Same pattern as OrderedTableStEph but with ~15 more axiom assumes because StPer
constructors (empty, singleton) need explicit axiom assumes where StEph constructors
don't ensure wf.

### Category 1: Redundant Type Axiom Assumes

The 4-pack pattern (`spec_pair_key_determines_order`, `obeys_cmp_spec::<K>`,
`view_ord_consistent::<K>`, `obeys_feq_fulls::<K,V>`) repeats in many functions.

**Key insight**: `spec_orderedtablestper_wf()` includes all the axiom predicates as
conjuncts. Any function with `requires self.spec_orderedtablestper_wf()` gets them for
free. Replace assumes with asserts that extract from wf.

For constructor functions (empty, singleton, from_sorted_entries) that have no `&self`,
the axiom assumes may need to stay unless the trait requires includes them.

### Category 2: Admits

Similar to StEph — admits in:
- intersection_iter, union_iter, difference_iter
- restrict_iter, subtract_iter
- split_key, get_key_range, split_rank_key
- map (value tracking)

**Strategy**: Bridge ParamBST Set ensures to OrderedTable Map postconditions via
`spec_pair_set_to_map`. The functions are StPer (persistent) so they return new values
rather than mutating. The proof pattern is:

1. ParamBST operation produces `result_tree` with `result_tree@ == expected_set`
2. Prove `spec_pair_set_to_map(result_tree@) == expected_map`
3. Prove `spec_key_unique_pairs_set(result_tree@)` (key uniqueness preserved)

Write bridge lemmas for the pair-set-to-map connection.

### Category 3: Other

- Iterator assume — standard, leave
- PartialEq/Clone assumes — standard workaround, leave
- Constructor axiom assumes — check if trait requires provide them

## Reference

**Read first**: `src/Chap43/OrderedTableStEph.rs` — Agent 2 is working the same pattern
on StEph. The StPer file should follow the same proof strategy. The key difference is
StPer returns new values (persistent) while StEph mutates in place.

**Also read**: `src/Chap38/BSTParaStEph.rs` — ParamBST ensures for split, union, etc.

## Approach

1. **Read** OrderedTableStPer.rs — understand wf, the full hole list
2. **Read** BSTParaStEph.rs — ParamBST ensures
3. **Phase 1**: Eliminate redundant axiom assumes in methods that have wf in requires
4. **Phase 2**: Attack admits — start with simpler ops (difference, restrict, subtract),
   then harder (intersection, union with value merging)
5. **Validate** after each phase

## Constraints

- Do NOT modify BSTParaStEph.rs, OrderedTableStEph.rs, or any file outside your assigned files.
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- Standard pattern holes (iterator, eq/clone) stay.
- Run validate, rtt, ptt sequentially.
