# Agent 2 Round 10 Report

## Summary

Removed 3 proof holes across Chap47 and Chap42. Strengthened trait specs with
cascading requires through Chap42/43 hierarchy. 3961 verified, 0 errors; 2600 RTTs pass.

## Holes Before/After

| # | Chap | File                      | Before | After | Delta |
|---|------|---------------------------|--------|-------|-------|
| 1 | 47   | ParaHashTableStEph.rs     | 5      | 4     | -1    |
| 2 | 47   | ChainedHashTable.rs       | 2      | 2     | 0     |
| 3 | 47   | DoubleHashFlatHashTableStEph.rs | 2 | 2     | 0     |
| 4 | 47   | LinProbFlatHashTableStEph.rs | 1   | 1     | 0     |
| 5 | 47   | QuadProbFlatHashTableStEph.rs | 1   | 1     | 0     |
| 6 | 42   | TableMtEph.rs             | 15     | 13    | -2    |
| 7 | 42   | TableStEph.rs             | 1      | 1     | 0     |
| 8 | 42   | TableStPer.rs             | 2      | 2     | 0     |
| 9 | 42   | Example42_1.rs            | 0      | 0     | 0     |
|   |      | **Chap47 Total**          | **11** | **10**| **-1**|
|   |      | **Chap42 Total**          | **18** | **16**| **-2**|

## What Was Done

### Chap47: compute_load_factor removed (-1 hole)
- Changed `LoadAndSize.load` from `f64` to `usize` (integer element count).
- Removed `compute_load_factor` external_body function entirely.
- `loadAndSize` now returns raw `num_elements` and `current_size`.
- Updated 4 RTT test files to use integer assertions.

### Chap42: TableMtEph size and find verified (-2 holes)
- Ported 5 proof lemmas from TableStEph to TableMtEph (standalone pattern):
  `lemma_entries_to_map_key_in_seq`, `lemma_entries_to_map_contains_key`,
  `lemma_entries_to_map_len`, `lemma_entries_to_map_no_key`, `lemma_entries_to_map_get`.
- Verified `size` using `lemma_entries_to_map_len` (removed external_body).
- Verified `find` as linear scan with loop invariant tracking `spec_keys_no_dups`
  (removed external_body).
- Added `requires self.spec_tablemteph_wf()` to `size` trait method.
- Added `requires self.spec_tablemteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()`
  to `find` trait method.
- Added `ensures .spec_tablemteph_wf()` to `empty` and `singleton`.

### Chap43: Cascading trait requires (no hole change)
- Strengthened `spec_orderedtablemteph_wf` to include `base_table.spec_tablemteph_wf()`.
- Strengthened `spec_augorderedtablemteph_wf` to include
  `base_table.spec_orderedtablemteph_wf()`.
- Added `requires self.spec_*_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()` to
  `find` and `lookup` in OrderedTableMtEphTrait and AugOrderedTableMtEphTrait.
- Added `requires self.spec_*_wf()` to `size` and `is_empty` in both traits.

## Chapters Closed

None.

## Verification Counts

- 3961 verified (up from 3953)
- 0 errors
- 2600 RTTs pass

## Techniques Used

- Integer arithmetic replacing f64 (compute_load_factor).
- Standalone lemma porting (StEph to MtEph).
- Linear scan with loop invariant and `spec_keys_no_dups`.
- Cascading requires propagation through trait hierarchy.

## Remaining Holes and Blockers

### Chap47 (10 holes)
- **8 probe/hash functions** (call_hash_fn, linear_probe, quadratic_probe, double_hash_probe,
  3 probe impls, second_hash): Blocked by Verus inability to reason about opaque `Fn` closures.
  Verus requires `FnSpec` or function items; the `H: Fn(&Key, usize) -> usize` parameters
  are runtime function pointers whose bodies are opaque to the verifier.
- **2 ChainedHashTable functions** (insert_chained, delete_chained): Blocked by Verus tuple
  Clone limitation. Verus cannot derive `(Key, Value): Clone` from component Clone bounds.
  Vec::set workaround requires cloning the entry, which for `Vec<(K,V)>` needs tuple Clone.

### Chap42 (16 holes)
- **11 MtEph external_body functions** (domain, tabulate, map, filter, intersection, union,
  difference, delete, insert, restrict, subtract): All use `join()` for parallel execution.
  Verifying requires named closures with explicit ensures, ghost view captures, and
  post-join proof blocks. Significant per-function proof effort.
- **2 MtEph assumes** (singleton, entries): Standard `obeys_feq_clone` pattern.
- **1 StPer external_body** (collect_by_key): Blocked by weak `insert` spec — ensures don't
  specify combined value when key already exists, only when key is new.
- **1 StEph assume** (entries): Standard `obeys_feq_clone` pattern.
- **1 StPer assume** (collect): Standard `obeys_feq_clone` pattern.

## Commit Hash

(see git log after commit)
