# Agent 2 Round 11 Report

## Summary

Strengthened TableStPer insert ensures and verified TableStPer collect_by_key
(removed external_body). 3992 verified, 0 errors; 2600 RTTs pass.

## Holes Before/After

| # | Chap | File                      | Before | After | Delta |
|---|------|---------------------------|--------|-------|-------|
| 1 | 42   | TableMtEph.rs             | 13     | 13    | 0     |
| 2 | 42   | TableStEph.rs             | 1      | 1     | 0     |
| 3 | 42   | TableStPer.rs             | 2      | 1     | -1    |
| 4 | 42   | Example42_1.rs            | 0      | 0     | 0     |
|   |      | **Chap42 Total**          | **16** | **15**| **-1**|
|   |      | **Chap47 Total**          | **10** | **10**| **0** |

## What Was Done

### Strengthened TableStPer insert ensures (+31 verified)
- Added existential ensures for combine case: `self@.contains_key(key@) ==>
  (exists|old_v: V, r: V| old_v@ == self@[key@] && combine.ensures((&old_v, &value), r)
  && updated@[key@] == r@)`.
- Added `forall|v1: &V, v2: &V| combine.requires((v1, v2))` to requires.
- Updated all callers across Chap42 and Chap43 (intersection, union, insert in
  OrderedTable/AugOrderedTable StPer/StEph/MtEph).
- Cascading `obeys_feq_full::<Pair<K, V>>()` additions to callers.

### Verified TableStPer collect_by_key (-1 hole)
- Removed `#[verifier::external_body]` from `collect_by_key`.
- Added requires: `obeys_view_eq::<K>()`, `obeys_feq_clone::<V>()`,
  `obeys_feq_clone::<K>()`, `obeys_feq_full::<ArraySeqStPerS<V>>()`,
  `obeys_feq_full::<Pair<K, ArraySeqStPerS<V>>>()`.
- Wrote 4 proof lemmas: `lemma_spec_collect_domain_step`,
  `lemma_spec_collect_key_step`, `lemma_spec_collect_key_not_in_domain`,
  `lemma_spec_collect_key_len_bound`.
- Used delete-then-insert pattern to avoid combine closure existential:
  delete key first (if exists), then insert with fresh value. This ensures
  insert always hits the `!self@.contains_key(key@)` branch.
- Element-by-element view proof via `spec_index` trigger: fired append
  ensures through `let ghost ai = appended.spec_index(j)` pattern.

## Chapters Closed

None.

## Verification Counts

- 3992 verified (up from 3961)
- 0 errors
- 2600 RTTs pass

## Techniques Used

- Existential ensures strengthening with `combine.ensures` for insert.
- Delete-then-insert pattern to avoid existential combine reasoning.
- Standalone step/domain unfolding lemmas via `subrange(0, i+1).drop_last()`.
- `spec_index` trigger firing for `Seq::map` view chain: `let ghost ai =
  appended.spec_index(j)` creates the trigger term that fires append ensures.
- Ghost variable capture inside match arms to carry proofs past arm boundaries.

## Remaining Holes and Blockers

### Chap42 (15 holes)
- **11 MtEph external_body functions** (domain, tabulate, map, filter,
  intersection, union, difference, delete, insert, restrict, subtract): All
  use `join()` or sequential merges. Verifying requires named closures with
  explicit ensures, ghost view captures, and post-join proof blocks. The
  sequential merge functions (intersection, union, difference) also rely on
  sorted-order invariant which is not captured in `spec_tablemteph_wf`.
  Adding requires to these trait methods would cascade to 20+ callers across
  Chap37, Chap41, Chap42, Chap43, Chap44.
- **2 MtEph assumes** (singleton, entries): Standard `obeys_feq_clone` pattern.
- **1 StEph assume** (entries): Standard `obeys_feq_clone` pattern.
- **1 StPer assume** (collect): Standard `obeys_feq_clone` pattern.

### Chap47 (10 holes)
- **8 probe/hash functions**: Blocked by hash function closure pattern.
  `H: Fn(&Key, usize) -> usize` is a generic parameter whose ensures are
  opaque. Fixing requires adding `hash_fn.requires`/`hash_fn.ensures`
  preconditions and threading a hash-function-valid spec through all hash
  table operations and their callers. Significant cross-file refactoring.
- **2 ChainedHashTable functions** (insert_chained, delete_chained): Blocked
  by Verus tuple Clone limitation.

## Commit Hash

(see git log after commit)
