# Agent 2 Round 74 Report

## Assignment
Prove/eliminate 15 holes in `src/Chap43/OrderedTableStEph.rs`:
- 13 `assume` holes in `singleton` and `tabulate` (type-axiom pattern)
- 2 `external_body` holes: `rank_key_iter` and `select_key`
- Fix `fn_missing_wf_ensures` warning on `from_sorted_entries`

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 43 | OrderedTableStEph.rs | 15 | 0 | -15 |
| 2 | 43 | OrderedTableMtEph.rs | 2 | 2 | 0 |

OrderedTableStEph.rs is now **clean**: 0 holes, 14 clean proof functions.

## Holes Eliminated

### 13 type-axiom assumes in singleton/tabulate
- **Strategy**: Combined broadcast triggers (`obeys_feq_full_trigger::<T>()`) for feq axioms with explicit `requires` on trait methods for non-broadcast axioms (`obeys_cmp_spec`, `view_ord_consistent`, `spec_pair_key_determines_order`).
- **Trait bound fix**: Changed `OrderedTableStEphTrait<K: StT + Ord, V: StT>` to `V: StT + Ord` (struct and all related traits already had `V: Ord`).
- **Caller updates**: Added type-axiom requires to singleton in AugOrderedTableStEph, OrderedTableMtEph, AugOrderedTableMtEph. Updated PTT file.

### 2 external_body holes: rank_key_iter and select_key
- **Strategy**: Adapted proven implementations from `OrderedTableStPer.rs`.
- **rank_key_iter**: Ghost `filter_pred` closure + `counted_keys: Set<K::V>`. Loop invariants track provenance, pairwise distinct keys, counted set bijection. Three-way comparison proof (Less: add to set; Equal/Greater: prove filter_pred false). Post-loop: extensional equality of counted_keys with dom.filter, then `lemma_len_filter`.
- **select_key**: Loop with `rank_key` call per candidate. Invariant tracks that found key satisfies rank spec. Proof uses `lemma_pair_in_set_map_contains`.

### fn_missing_wf_ensures on from_sorted_entries
- Added `result.spec_orderedtablesteph_wf()` to ensures.
- Added requires: type axioms + unique-keys condition on input entries.
- Loop invariant tracks `spec_key_unique_pairs_set(tree@)` + provenance (every tree element came from entries[0..i]).
- Uniqueness maintenance proof: contradiction via pairwise distinct entry keys.
- Updated MtEph `from_sorted_entries` caller with new requires.

## Files Modified

| # | Chap | File | Changes |
|---|------|------|---------|
| 1 | 43 | OrderedTableStEph.rs | Trait V bound, 13 assumes replaced, rank_key_iter/select_key proved, from_sorted_entries wf |
| 2 | 43 | AugOrderedTableStEph.rs | singleton requires (type axioms) |
| 3 | 43 | OrderedTableMtEph.rs | singleton requires, from_sorted_entries requires |
| 4 | 43 | AugOrderedTableMtEph.rs | singleton requires (type axioms) |
| 5 | 43 | ProveOrderedTableStEph.rs (PTT) | Test requires (type axioms) |

## Validation

- `scripts/validate.sh`: 4736 verified, 0 errors
- `scripts/rtt.sh`: 2619 tests passed
- `scripts/ptt.sh`: 157 tests passed

## Bonus (not attempted)
- 4 iterator external_body holes in Mt files (OrderedTableMtEph/AugOrderedTableMtEph iter/into_iter) were not attempted. These are RwLock boundary holes marked `veracity: accept`.
