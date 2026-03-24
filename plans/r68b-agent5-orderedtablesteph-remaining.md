# R68b Agent 5: OrderedTableStEph Remaining Holes

## Goal

Reduce remaining holes in `src/Chap43/OrderedTableStEph.rs` toward 0. Agent 2 (R68)
already attacked this file. Your job: burn down whatever holes remain.

## Setup

1. Run `scripts/holes.sh src/Chap43/OrderedTableStEph.rs` to see current holes
2. Read `src/Chap43/OrderedTableStEph.rs` — understand what Agent 2 already fixed

## Hole Categories (from pre-Agent-2 baseline, 43 holes)

### Type Axiom Assumes (~28 holes pre-Agent-2)

The 4-pack pattern repeats in 7+ functions:
```
assume(spec_pair_key_determines_order::<K, V>());
assume(vstd::laws_cmp::obeys_cmp_spec::<K>());
assume(view_ord_consistent::<K>());
assume(obeys_feq_fulls::<K, V>());
```

**Key insight**: `spec_orderedtablesteph_wf()` includes all four predicates as conjuncts.
Functions with `requires self.spec_orderedtablesteph_wf()` already have them in scope.
Replace assumes with asserts extracting from wf.

For `tabulate` — takes `(f, keys)` not `&self`, no wf in scope. Those assumes may stay
unless the trait requires includes the axioms.

For `from_sorted_entries` constructor — also has 2 axiom assumes plus no wf ensures.

### Admits (~12 holes pre-Agent-2)

Real proof obligations bridging ParamBST's Set ensures to OrderedTable's Map postconditions
via `spec_pair_set_to_map`. Functions:

| # | Function | What needs proving |
|---|----------|--------------------|
| 1 | map | Mapped result contains all original keys with f-applied values |
| 2 | intersection | Map domain = self.dom() ∩ other.dom() |
| 3 | union | Map domain = self.dom() ∪ other.dom(), values correct |
| 4 | difference | Map domain = self.dom() \ other.dom() |
| 5 | restrict | Map domain = self.dom() ∩ keys@ |
| 6 | subtract | Map domain = self.dom() \ keys@ |
| 7 | first_key | TotalOrder::le connection to BST min_key |
| 8 | split_key | Left/right domain partition |
| 9 | get_key_range | Range domain correct |
| 10 | rank_key | Count = dom.filter(|k| le(k,x)).len() |
| 11 | select_key | Rank/domain proofs |
| 12 | split_rank_key | Left/right partition by rank |

**Strategy**: The proof chain is:
1. ParamBST operation produces `result_tree@` (a `Set<(K::V, V::V)>`)
2. Prove `spec_pair_set_to_map(result_tree@) == expected_map_result`
3. Prove `spec_key_unique_pairs_set(result_tree@)` (key uniqueness preserved)

Write bridge lemmas. For set operations (intersection, union, difference): ParamBST
already ensures the correct Set; prove the map conversion preserves the domain/value
relationship.

**Reference**: Read `src/Chap38/BSTParaStEph.rs` for ParamBST ensures. Also read
`src/Chap43/OrderedTableStPer.rs` — Agent 3/4 work the same proof pattern.

### Permanent Holes

- `assume(iter_invariant(self))` in Iterator::next — standard, leave
- `assume(obeys_cmp_spec/view_ord_consistent)` in `from_sorted_entries` — constructor
  axioms, likely stay

## Approach

1. Run `scripts/holes.sh` to see what Agent 2 left
2. Read the file, understand remaining holes
3. Phase 1: Eliminate any remaining type axiom assumes (mechanical)
4. Phase 2: Attack remaining admits (real proofs)
5. Validate, rtt, ptt sequentially

## Constraints

- Do NOT modify BSTParaStEph.rs, OrderedTableStPer.rs, or any file outside Chap43.
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- Iterator assume stays (standard pattern).
- Run validate, rtt, ptt sequentially.
