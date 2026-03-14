# Agent 4 — Round 13 (Full) Report

## Summary

Two passes this round. Pass 1: probe function verification (-5). Pass 2 (restart):
clone_entry pattern for ChainedHashTable (-2), compute_second_hash factoring (+1 -2 net -1),
iter() while-loop rewrite (-1). Between passes, Agents 1+3 merged (-55 from 267→217).

- **Holes**: 267 → 213 (agent4 contribution: -4 on top of 217 base)
- **Verified**: 4004 → 4024
- **RTT**: 2600 pass
- **PTT**: 147 pass

## Hole Changes by File

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 47 | ParaHashTableStEph.rs | 4 | 2 | -2 | call_hash_fn + compute_second_hash remain |
| 2 | 47 | DoubleHashFlatHashTableStEph.rs | 2 | 0 | -2 | second_hash + probe both verified |
| 3 | 47 | ChainedHashTable.rs | 2 | 0 | -2 | clone_entry pattern |
| 4 | 47 | LinProbFlatHashTableStEph.rs | 0 | 0 | 0 | CLEAN (pass 1) |
| 5 | 47 | QuadProbFlatHashTableStEph.rs | 0 | 0 | 0 | CLEAN (pass 1) |
| 6 | 41 | AVLTreeSetMtEph.rs | 10 | 9 | -1 | iter() verified |
| 7 | 41 | AVLTreeSetMtPer.rs | 11 | 11 | 0 | all blocked |
| 8 | 42 | TableMtEph.rs | 11 | 11 | 0 | all blocked |
| 9 | 39 | BSTTreapMtEph.rs | 8 | 8 | 0 | all view bridge |

## Chapters Closed

None. Chap47 has 2 irreducible external_body utilities (call_hash_fn, compute_second_hash).

## Techniques Used

### 1. clone_entry pattern (Chap47 ChainedHashTable, -2 holes)
Verus cannot derive Clone for `(Key, Value)` tuples. Added `fn clone_entry(&self) -> Self`
to `EntryTrait` with element-wise cloning (`.0.clone()`, `.1.clone()`). Implemented for
Vec, LinkedListStEphS, ChainList, FlatEntry. Enabled clone + vec.set workaround for
insert_chained and delete_chained, bypassing both IndexMut and tuple-Clone limitations.

### 2. compute_second_hash factoring (Chap47, net -1)
Extracted `compute_second_hash` as single external_body utility in ParaHashTableStEph.
Both `double_hash_probe` and `second_hash` now delegate to it and are verified.

### 3. Probe verification (Chap47, -5 holes in pass 1)
Replace raw `(hash_fn)(key, table_size)` with `call_hash_fn` (tight ensures).
Use wrapping arithmetic + `% table_size` for automatic bound proof.

### 4. iter() while-loop rewrite (Chap41 MtEph, -1 hole)
Trivial ensures (pos==0, bounds) didn't need view bridge. Used
`seq_to_set_is_finite` proof for inner to_seq() requires.

## Remaining Holes — What Blocks Them

### Chap47 (2 holes) — irreducible utilities
- `call_hash_fn`: external_body wrapping Fn closure dispatch
- `compute_second_hash`: external_body wrapping std::hash API

### Chap41 MtEph (9 holes)
- 2 assume (size, find): **view bridge** — ghost_set_view outside RwLock
- 5 external_body (to_seq, filter, intersection, difference, union): **parallel** — join(), nested fns
- 2 unsafe (Send, Sync): required for Ghost<Set<V>>

### Chap41 MtPer (11 holes)
- 1 assume (size): view bridge (seq length vs set cardinality)
- 1 assume (find feq): type axiom (obeys_feq_full not provable for generic T)
- 1 assume (find not-found): missing sorted invariant in AVLTreeSeqMtPer wf
- 7 external_body (from_seq, filter, intersection, difference, union, delete, insert): parallel
- 1 external_body (cmp): no requires allowed on Ord::cmp trait method

### Chap42 TableMtEph (11 holes)
- 8 external_body: parallel (join + unverified closures + for-range loops)
- 3 external_body (intersection, union, difference): sequential code but proves require
  sorted-entries invariant not tracked in spec_tablemteph_wf

### Chap39 BSTTreapMtEph (8 holes)
- 8 assume: view bridge — unit struct RwLock invariant can't carry instance-specific ghost data

## Verification

```
verification results:: 4024 verified, 0 errors
RTT: 2600 tests run: 2600 passed
PTT: 147 tests run: 147 passed
Total holes: 213
```
