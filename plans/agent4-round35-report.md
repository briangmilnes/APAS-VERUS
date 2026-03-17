# Agent 4 — Round 35 Report

## Summary

Proved Mt delegation wrappers in OrderedSetMtEph.rs and OrderedTableMtEph.rs.
Converted RwLock ghost boundary assumes to accepts, removed external_body from
ordering delegations and filter operations.

## Results

- 4199 verified, 0 errors
- 2613 RTT pass
- 143 total holes (was 171 in R34, -28 net including merge baseline shift)

## Holes Before/After

| # | Chap | File | Before | After | Delta | Technique |
|---|------|------|--------|-------|-------|-----------|
| 1 | 43 | OrderedSetMtEph.rs | 15 | 1 | -14 | assume→accept, delegation+accept bridge |
| 2 | 43 | OrderedTableMtEph.rs | 7 | 6 | -1 | external_body removal (filter) |

**Net: -15 holes across assigned files.**

## Changes Detail

### OrderedSetMtEph.rs (Chap43): 15 → 1 (-14)

**7 assumes → accepts (RwLock ghost boundary pattern)**
- L241: `size` — `accept(count == self@.len())`
- L269: `find` — `accept(found == self@.contains(x@))`
- L434-435: `split` — `accept(left/right.spec_orderedsetsteph_wf())`
- L459: `get_range` — `accept(range.spec_orderedsetsteph_wf())`
- L495-496: `split_rank` — `accept(left/right.spec_orderedsetsteph_wf())`

All match the RwLock ghost bridge pattern from R34 BSTTreapMtEph + the
toplevel_coarse_rwlocks_for_mt_modules standard.

**6 external_body → delegation + accept bridge**
- `first`, `last`, `previous`, `next`, `rank`, `select`
- Pattern: `use_type_invariant(self)` → `acquire_read` → call StEph method
  → `accept(inner@ =~= self@)` → `release_read`
- Single blanket accept per delegation bridges all postconditions from
  inner (StEph) view to self (MtEph) ghost view

**1 external_body → write delegation (filter)**
- Pattern: `acquire_write` → call `locked_val.filter(f, Ghost(spec_pred))`
  → capture `ghost new_view = locked_val@` → `release_write` → update ghost
- StEph filter ensures `spec_orderedsetsteph_wf()` (satisfies RwLock inv)
  and `self@.finite()` (satisfies Mt postcondition)

**1 external_body remaining (to_seq)**
- Returns `ArraySeqStPerS<T>` but StEph returns `AVLTreeSeqStPerS<T>`
- Body converts via clone loop — needs deep clone bridge proof

**1 warning fixed**
- `from_st`: added `s.spec_orderedsetmteph_wf()` to ensures (trivially
  provable since `spec_orderedsetmteph_wf` is just `self@.finite()`)

### OrderedTableMtEph.rs (Chap43): 7 → 6 (-1)

**1 external_body removed (filter)**
- Body already had correct implementation: collect → filter loop → from_sorted_entries
- Postcondition `filtered@.dom().finite()` chains through:
  `from_vec` ensures `spec_avltreeseqstper_wf()` → `from_sorted_entries` ensures `dom().finite()`

**6 external_body remaining (ordering operations)**
- `first_key`, `last_key`, `previous_key`, `next_key`, `rank_key`, `select_key`
- These are NOT delegations — they are direct algorithmic implementations
  (iterate over entries, find min/max/count)
- Proof requires: loop invariants with TotalOrder, clone bridges,
  connecting `base_table.entries()` to `self@.dom()`, existential quantifiers
  in filter specs
- Too complex for this round; left as external_body

**1 warning remaining (from_sorted_entries)**
- `fn_missing_wf_ensures` — needs `spec_orderedtablemteph_wf()` in ensures
- Blocked: upstream `TableMtEph::from_sorted_entries` doesn't ensure
  `spec_keys_no_dups` (part of wf)

## Key Pattern: Blanket Accept Bridge

For read-delegation operations (first, last, previous, next, rank, select),
a single `accept(inner@ =~= self@)` bridges ALL postconditions from the
inner StEph's view to the outer MtEph's ghost view. This works because:
- Set extensional equality (`=~=`) means `inner@.contains(x) == self@.contains(x)` for all x
- All postconditions (membership, finiteness, len, filter) depend only on
  containment, which transfers through =~=
- One accept per delegation instead of per-postcondition
