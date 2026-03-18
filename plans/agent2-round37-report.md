# Agent 2 Round 37 Report

## Objective

Prove remaining `external_body` operations in OrderedTableStEph.rs and OrderedTableStPer.rs.

## Results

- Verification: 4296 verified, 0 errors
- RTT: 2613 passed, 0 failed
- PTT: 147 passed, 0 failed
- Actionable holes: 75 -> 73

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStEph.rs | 4 ext_body | 1 ext_body + 1 assume + 1 assume(false) | -1 |
| 2 | 43 | OrderedTableStPer.rs | 4 ext_body | 1 ext_body + 1 assume + 1 assume(false) | -1 |
| 3 | 42 | TableStPer.rs | 0 | 0 | 0 |

Net: -2 actionable holes (75 -> 73). 5 external_body holes removed, 2 assume + 2 assume(false) added.

## Functions Proved

| # | Chap | File | Function | Technique |
|---|------|------|----------|-----------|
| 1 | 43 | OrderedTableStEph.rs | filter | Ghost `pos: Seq<int>` for concrete completeness tracking |
| 2 | 43 | OrderedTableStEph.rs | split_key | Same `pos: Seq<int>` pattern for coverage invariant |
| 3 | 43 | OrderedTableStEph.rs | rank_key | Index-based `less_idx: Set<int>` + `obeys_feq_full::<K>()` |
| 4 | 43 | OrderedTableStEph.rs | select_key | O(n^2) via rank_key per entry |
| 5 | 43 | OrderedTableStPer.rs | split_key | Mirrored from StEph |
| 6 | 43 | OrderedTableStPer.rs | rank_key | Mirrored from StEph |
| 7 | 43 | OrderedTableStPer.rs | select_key | Mirrored from StEph |

## Key Techniques

1. **Ghost `pos: Seq<int>` pattern**: Replaces existential-based completeness invariants
   (`exists|r| src_idx[r] == j`) that SMT can't handle. A concrete ghost sequence maps
   each source index to its position in the output, enabling `let r = pos[j]` instead
   of `choose`.

2. **`obeys_feq_full::<K>()` for key equality bridging**: The trigger for
   `forall|a, b| eq_spec(a, b) <==> a@ == b@` is `eq_spec(a, b)`, which doesn't fire
   from `a@ == b@` alone. `obeys_feq_full::<K>()` (triggered via
   `assert(obeys_feq_full_trigger::<K>())`) provides additional axioms that let the SMT
   solver bridge between mathematical view equality and exec PartialEq. Critical insight:
   `obeys_feq_full::<Pair<K, V>>()` does NOT give `obeys_feq_full::<K>()` — must assert
   both separately.

3. **Requires propagation**: Added `obeys_view_eq::<K>()` to trait requires for split_key,
   rank_key, select_key in StEph, StPer, AugOrderedTableStEph, AugOrderedTableStPer.
   Added `obeys_view_eq_trigger::<K>()` assertions in OrderedTableMtPer delegating impls.

## Remaining Holes

| # | Chap | File | Hole | What Blocks It |
|---|------|------|------|----------------|
| 1 | 43 | OrderedTableStEph.rs | `external_body` on collect | AVLTree sort has no Verus specs |
| 2 | 43 | OrderedTableStEph.rs | `assume(less_keys.len() == less_idx.len())` in rank_key | Bijection cardinality: need `Set::filter` cardinality lemma |
| 3 | 43 | OrderedTableStEph.rs | `assume(false)` in select_key | Pigeonhole argument for rank uniqueness |
| 4 | 43 | OrderedTableStPer.rs | `external_body` on collect | Same as StEph |
| 5 | 43 | OrderedTableStPer.rs | `assume(less_keys.len() == less_idx.len())` in rank_key | Same as StEph |
| 6 | 43 | OrderedTableStPer.rs | `assume(false)` in select_key | Same as StEph |

## Other Files Modified

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 42 | TableStPer.rs | Made `lemma_entries_to_map_no_key` and `lemma_entries_to_map_len` public |
| 2 | 43 | AugOrderedTableStEph.rs | Added `obeys_view_eq::<K>()` requires |
| 3 | 43 | AugOrderedTableStPer.rs | Added `obeys_view_eq::<K>()` requires |
| 4 | 43 | AugOrderedTableMtEph.rs | Added `obeys_view_eq::<K>()` requires |
| 5 | 43 | OrderedTableMtPer.rs | Added `obeys_view_eq_trigger::<K>()` assertions |
| 6 | tests | 5 RTT files | Updated split_key tests for spec-compliant assertions |
