# Agent 2 Round 35 Report

## Summary

Proved `split_rank_key` in both OrderedTableStEph.rs and OrderedTableStPer.rs.
Proved `map` in OrderedTableStEph.rs (previous session).
Strengthened `from_sorted_entries` and `collect` ensures in both files.
Made TableStEph and TableStPer lemmas public to support the proofs.
Propagated `requires` changes to AugOrderedTableStEph and AugOrderedTableStPer.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStEph.rs | 12 | 10 | -2 |
| 2 | 43 | OrderedTableStPer.rs | 10 | 9 | -1 |
| 3 | 43 | AugOrderedTableStEph.rs | 10 | 10 | 0 |
| 4 | 43 | AugOrderedTableStPer.rs | 8 | 8 | 0 |

Net reduction: 3 holes.

## Holes Proved

| # | Chap | File | Function | Technique |
|---|------|------|----------|-----------|
| 1 | 43 | OrderedTableStEph.rs | map | Collect entries, apply f, rebuild via from_sorted_entries |
| 2 | 43 | OrderedTableStEph.rs | split_rank_key | Collect entries, split at index, prove no_dups/subset/disjoint/coverage |
| 3 | 43 | OrderedTableStPer.rs | split_rank_key | Same pattern as StEph but with &self (persistent, no mutation) |

## Infrastructure Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 42 | TableStEph.rs | Made 7 lemmas public (key_in_seq, contains_key, len, no_key, get, dom_subset, dom_same_keys) |
| 2 | 42 | TableStPer.rs | Made 4 lemmas public (get, dom_subset, contains_key, key_in_seq) |
| 3 | 43 | OrderedTableStEph.rs | Strengthened from_sorted_entries (spec_keys_no_dups requires, table@ =~= spec_entries_to_map ensures, wf ensures) |
| 4 | 43 | OrderedTableStPer.rs | Strengthened from_sorted_entries (same pattern) |
| 5 | 43 | OrderedTableStPer.rs | Strengthened collect ensures (conditional spec_entries_to_map, spec_keys_no_dups) |
| 6 | 43 | OrderedTableStEph.rs | Strengthened collect ensures (conditional spec_entries_to_map, spec_keys_no_dups) |
| 7 | 43 | AugOrderedTableStEph.rs | Added wf requires to get_key_range, split_rank_key, reduce_range trait defs |
| 8 | 43 | AugOrderedTableStPer.rs | Added wf requires to split_rank_key trait def |

## Key Techniques

- **Broadcast trigger pattern**: Used `assert(obeys_feq_full_trigger::<Pair<K, V>>())` to
  trigger feq axioms without adding feq to trait requires (avoids cascading requires changes).
- **clone_plus()**: Used instead of clone() to get `ensures cloned(*self, res)` for view preservation.
- **Direct table construction**: Built OrderedTable directly (not via Table::from_sorted_entries)
  to maintain proof access to entries for the view equality proof.
- **spec_index intermediate assertion**: Required in from_sorted_entries to help Verus chain
  `seq@[j]` through `spec_index(j)@` to `elements@[j]@`.

## Remaining Holes (Blocked)

The remaining holes in OrderedTableStEph (10) and OrderedTableStPer (9) fall into categories:

1. **TotalOrder bridging** (first_key, last_key, previous_key, next_key, rank_key, select_key):
   Need exec-level Ord comparison results to prove view-level ordering properties. Blocked
   until TotalOrder infrastructure provides this bridge.

2. **Reference PartialOrd** (get_key_range): Verus error "core::cmp::impls::impl&%10::le"
   not supported. The `&pair.0 >= k1` comparison uses the blanket `impl PartialOrd for &T`
   which Verus cannot verify.

3. **split_key**: Requires key comparison to find the split point, same TotalOrder issue.

4. **collect**: Wrapper around BTree internals, external_body is appropriate.

5. **filter** (StEph only): Requires closure + key comparison infrastructure.

## Verification

- 4198 verified, 0 errors
- 2613 runtime tests passed
