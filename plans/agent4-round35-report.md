# Agent 4 — Round 35 Report

## Summary

Removed 19 external_body across three Chap43 files. Replaced with real implementations
(delegation bodies, loop-based ordering ops) backed by assumes for semantic postconditions.
Reverted unauthorized accept() usage to assume() per user correction.

## Results

- 4215 verified, 0 errors
- 2613 RTT pass
- 161 total holes (was 171 in R34)

## Holes Before/After

| # | Chap | File | Before | After | Delta | Technique |
|---|------|------|--------|-------|-------|-----------|
| 1 | 43 | OrderedSetMtEph.rs | 15 | 14 | -1 | 7 ext_body→delegation+assume, filter proved |
| 2 | 43 | OrderedTableMtEph.rs | 7 | 17 | +10 | 6 ext_body→loop+assume (granular holes) |
| 3 | 43 | AugOrderedTableMtEph.rs | 8 | 2 | -6 | 6 ext_body→delegation (no new assumes) |

**Net: +3 holes across assigned files (19 external_body removed, 29 assumes added).**

Note: hole count increased because external_body = 1 hole regardless of postcondition
count, while each assume = 1 hole. The 29 assumes document exactly what remains unproved.

## Changes Detail

### OrderedSetMtEph.rs (Chap43): 15 → 14 (-1)

**7 original assumes preserved (RwLock ghost boundary pattern)**
- L241: `size` — `assume(count == self@.len())`
- L269: `find` — `assume(found == self@.contains(x@))`
- L434-435: `split` — `assume(left/right.spec_orderedsetsteph_wf())`
- L459: `get_range` — `assume(range.spec_orderedsetsteph_wf())`
- L495-496: `split_rank` — `assume(left/right.spec_orderedsetsteph_wf())`

**6 external_body → delegation + assume bridge**
- `first`, `last`, `previous`, `next`, `rank`, `select`
- Pattern: `use_type_invariant(self)` → `acquire_read` → call StEph method
  → `assume(inner@ =~= self@)` → `release_read`
- Single blanket assume per delegation bridges all postconditions from
  inner (StEph) view to self (MtEph) ghost view

**1 external_body → write delegation (filter)**
- Pattern: `acquire_write` → call `locked_val.filter(f, Ghost(spec_pred))`
  → capture `ghost new_view = locked_val@` → `release_write` → update ghost
- Fully proved, no new assumes

**1 external_body remaining (to_seq)**
- Returns `ArraySeqStPerS<T>` but StEph returns `AVLTreeSeqStPerS<T>`
- Body converts via clone loop — needs deep clone bridge proof

**1 warning fixed**
- `from_st`: added `s.spec_orderedsetmteph_wf()` to ensures

### OrderedTableMtEph.rs (Chap43): 7 → 17 (+10)

**1 external_body removed (filter) — fully proved**
- Postcondition `filtered@.dom().finite()` chains through `from_sorted_entries`

**5 external_body → loop implementations + assumes**
- `first_key`, `last_key`, `previous_key`, `next_key`, `rank_key`
- Pattern: `lemma_entries_to_map_finite` → iterate entries → find min/max/count
  → assume semantic postconditions (membership, ordering, dom length)
- Loop invariants: bounds, spec_len, decreases
- Assumes per function: first/last 4 each, previous/next 3 each, rank 2 = 16 total

**1 external_body remaining (select_key)**
- Uses Vec::sort() which has no Verus specs

**1 warning remaining (from_sorted_entries)**
- `fn_missing_wf_ensures` — blocked on upstream TableMtEph

### AugOrderedTableMtEph.rs (Chap43): 8 → 2 (-6)

**6 external_body → pure delegations (no new assumes)**
- `first_key`, `last_key`, `previous_key`, `next_key`, `rank_key`, `select_key`
- Pattern: `proof { lemma_aug_view(self); }` → `self.base_table.<method>()`
- View is `open spec fn view(&self) -> Map { self.base_table@ }` so
  postconditions transfer directly from OrderedTableMtEph
- Zero new assumes — postconditions proved through delegation chain

**2 external_body remaining**
- `calculate_reduction`: loop over entries with reducer closure — needs
  requires cascade (wf, closure requires) to all callers
- `reduce_range_parallel`: uses ParaPair! for fork-join parallelism — kept
  per rule against sequentializing Mt code

## Key Pattern: Blanket Assume Bridge

For read-delegation operations (first, last, previous, next, rank, select),
a single `assume(inner@ =~= self@)` bridges ALL postconditions from the
inner StEph's view to the outer MtEph's ghost view. This works because:
- Set extensional equality (`=~=`) means `inner@.contains(x) == self@.contains(x)` for all x
- All postconditions (membership, finiteness, len, filter) depend only on
  containment, which transfers through =~=
- One assume per delegation instead of per-postcondition
