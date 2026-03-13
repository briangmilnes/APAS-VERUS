# Agent 4 Round 4 Report

## Summary

192 → 182 holes (−10), 15 warnings fixed, 3713 verified, 0 errors.

## Changes

### Phase 1: Fix requires_true warnings (15 fixes, 0 holes reduced)

Removed `requires true,` from 15 functions across 5 chapters:

| # | Chap | File | Function |
|---|------|------|----------|
| 1 | 45 | BinaryHeapPQ.rs | `parent()` |
| 2 | 45 | LeftistHeapPQ.rs | `total_order_le()` |
| 3 | 47 | ParaHashTableStEph.rs | `createTable()` |
| 4 | 47 | ParaHashTableStEph.rs | `metrics()` |
| 5 | 47 | ParaHashTableStEph.rs | `loadAndSize()` |
| 6 | 47 | StructChainedHashTable.rs | `chain_insert()` |
| 7 | 47 | StructChainedHashTable.rs | `chain_lookup()` |
| 8 | 47 | StructChainedHashTable.rs | `chain_delete()` |
| 9 | 43 | OrderedSetMtEph.rs | `from_st()` |
| 10 | 43 | OrderedTableMtPer.rs | `from_st_table()` |
| 11 | 43 | AugOrderedTableStPer.rs | `calculate_reduction()` |
| 12 | 49 | SubsetSumMtEph.rs | `clone_arc_memo()` |
| 13 | 49 | SubsetSumMtPer.rs | `clone_arc_memo()` |
| 14 | 49 | MinEditDistMtEph.rs | `clone_arc_memo()` |
| 15 | 49 | MinEditDistMtPer.rs | `clone_arc_memo()` |

### Phase 2: Remove external holes in Chap45 (−1 hole)

| # | File | Change | Result |
|---|------|--------|--------|
| 1 | BalancedTreePQ.rs | Removed `#[verifier::external]` from Default impl, added ensures | −1 hole |
| 2 | Example45_2.rs | Attempted removal — Verus rejects calls to functions outside verus! | Reverted |
| 3 | HeapsortExample.rs | Skipped — same limitation as Example45_2 | No change |

### Phase 3: Prove BottomUpDP St loops in Chap51 (−2 holes)

Rewrote `med_bottom_up` in both St files from `external_body` to fully verified.

| # | File | Technique | Result |
|---|------|-----------|--------|
| 1 | BottomUpDPStEph.rs | Row-by-row construction with push, case-split assertions | −1 hole |
| 2 | BottomUpDPStPer.rs | Same pattern adapted for persistent (&self) variant | −1 hole |

Key technique: Build each DP row as a fresh Vec using only `push` (no nested Vec mutation).
Connect exec values to `spec_med` via case-split proof blocks that trigger Verus to use
the correct invariant clause for base-case vs inner cells.

### Phase 4: Prove BSTParaStEph comparison lemmas and split assumes (−7 holes)

Eliminated all 7 assumes from BSTParaStEph.rs using `view_ord_consistent` to bridge
the vstd axiom gap for Equal substitution under `cmp_spec`.

**Comparison lemmas (−3 assumes):**

| # | Lemma | Technique |
|---|-------|-----------|
| 1 | `lemma_cmp_eq_subst` | Added `view_ord_consistent` to requires. Equal case: view equality chain contradicts Less. Greater case: solver handles via transitivity. |
| 2 | `lemma_cmp_equal_congruent` | Added `view_ord_consistent`. `assert(a@ == b@)` lets solver eliminate all 6 mismatch cases via transitivity or view equality. |
| 3 | `lemma_cmp_equal_congruent_right` | Symmetric to #2. `assert(b@ == c@)` suffices. |

**Split function (−4 assumes):**

| # | Match arm | Elements | Technique |
|---|-----------|----------|-----------|
| 4 | Less | rebuilt > key | 3-way case split: lr elements from recursive split; right elements via antisymmetry+transitivity; root_key via eq_subst. |
| 5 | Greater | rebuilt < key | 3-way case split: rl from recursive split; left via antisymmetry+transitivity; root_key via congruence. |
| 6 | Equal | left < key | Right congruence: `Equal(kval, rk)` → `t cmp kval == t cmp rk`. |
| 7 | Equal | right > key | Same right congruence lemma. |

## Hole Counts

| # | Chap | Before | After | Change |
|---|------|--------|-------|--------|
| 1 | 38 | 32 | 25 | −7 |
| 2 | 43 | 92 | 92 | 0 |
| 3 | 45 | 17 | 16 | −1 |
| 4 | 47 | 39 | 39 | 0 |
| 5 | 49 | 4 | 4 | 0 |
| 6 | 51 | 8 | 6 | −2 |
| **Total** | | **192** | **182** | **−10** |

## Not Attempted (and why)

| Category | Holes | Reason |
|----------|-------|--------|
| Chap47 hash tables (39) | external_body | Full hash table verification is research-level |
| Chap43 St BTreeMap wrappers (53) | external_body | Delegate to external Rust container |
| Chap43 Mt remaining ops (32) | external_body | Complex closures, map/filter/reduce |
| Chap43 AugOrderedTableStPer (2) | assume | Closure requires — Verus limitation |
| Chap38 StEph remaining (7) | 6 ext_body + 1 accept | Arc removed, now plain RwLock + ghost shadow |
| Chap38 MtEph (19) | external_body | Concurrent BST with per-node locking |
| Chap49 Mt concurrent memo (4) | external_body | Recursive parallel memoization |
| Chap51 Mt parallel DP (4) | external_body | Concurrent diagonal pebbling |
| Chap51 TopDownDP dummy inv (2) | dummy_rwlock_predicate | HashMap lacks View in vstd |
| Chap45 Example45_2 (1) | external | Calls functions outside verus! |
| Chap45 HeapsortExample (1) | external | Same limitation + nested function |

## Verification

```
verification results:: 3713 verified, 0 errors
```
