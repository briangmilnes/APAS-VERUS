# Agent 3 Round 35 Report

## Summary

**Phase 1:** Proved ordering operations in AugOrderedTableStEph.rs and AugOrderedTableStPer.rs
by removing `external_body` annotations and adding `lemma_aug_view` proof hints.

**Phase 2:** Proved calculate_reduction, join_key, and clone in both files by lifting
closure `requires` into function preconditions and cascading to callers. For StEph, the
reducer totality condition was added to `spec_augorderedtablesteph_wf`. For StPer, the
same approach was used, plus a `lemma_reducer_clone_total` bridge for methods that
construct new structs with cloned reducers (analogous to the eq/clone bridge pattern).

## Holes Before/After

| # | Chap | File | Before | After | Removed |
|---|------|------|--------|-------|---------|
| 1 | 43 | AugOrderedTableStEph.rs | 10 | 0 | 10 |
| 2 | 43 | AugOrderedTableStPer.rs | 8 | 1 | 7 |
| **Total** | | | **18** | **1** | **-17** |

## Functions Proved

### AugOrderedTableStEph.rs (10 holes removed)

| # | Chap | Function | Technique |
|---|------|----------|-----------|
| 1 | 43 | first_key | lemma_aug_view + delegation |
| 2 | 43 | last_key | lemma_aug_view + delegation |
| 3 | 43 | previous_key | lemma_aug_view + delegation |
| 4 | 43 | next_key | lemma_aug_view + delegation |
| 5 | 43 | rank_key | lemma_aug_view + delegation |
| 6 | 43 | select_key | lemma_aug_view + delegation |
| 7 | 43 | map | lemma_aug_view + delegation |
| 8 | 43 | calculate_reduction | requires cascade + loop invariant |
| 9 | 43 | join_key | wf provides reducer.requires |
| 10 | 43 | clone | assume bridge (base_cloned@ == self.base_table@) |

### AugOrderedTableStPer.rs (7 holes removed)

| # | Chap | Function | Technique |
|---|------|----------|-----------|
| 1 | 43 | first_key | lemma_aug_view + delegation |
| 2 | 43 | last_key | lemma_aug_view + delegation |
| 3 | 43 | previous_key | lemma_aug_view + delegation |
| 4 | 43 | next_key | lemma_aug_view + delegation |
| 5 | 43 | rank_key | lemma_aug_view + delegation |
| 6 | 43 | select_key | lemma_aug_view + delegation |
| 7 | 43 | calculate_reduction | requires cascade + loop invariant |
| 8 | 43 | join_key | wf provides reducer.requires (assume removed) |

## Remaining Holes

### AugOrderedTableStEph.rs (0 remaining)

Clean. The clone assume is classified as a structural false positive (eq/clone bridge).

### AugOrderedTableStPer.rs (1 remaining)

| # | Chap | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 43 | lemma_reducer_clone_total | assume | Clone bridge for closures |

This is the closure-clone bridge: `assume(forall|v1: &V, v2: &V| cloned.requires((v1, v2)))`.
Analogous to the eq/clone bridge pattern for values. Verus cannot prove that cloning a
closure preserves its `requires`. Called by 10 methods that construct new structs with
`self.reducer.clone()` and ensure `spec_augorderedtablestper_wf()` on output.

## Key Design Decisions

### StEph: Reducer in wf
Added `forall|v1: &V, v2: &V| #[trigger] self.reducer.requires((v1, v2))` to
`spec_augorderedtablesteph_wf`. StEph methods are ephemeral (mutate self), so they
don't construct new structs with cloned reducers. The cascade works cleanly.

### StPer: Reducer in wf + clone bridge
Same wf change, but StPer methods are persistent (return new values). Every method
that ensures wf on output must prove the cloned reducer is total. A single proof fn
`lemma_reducer_clone_total` concentrates the assume in one place. All 10 methods call it.

### StPer: Explicit requires on 5 trait methods
Added `requires self.spec_augorderedtablestper_wf()` to split_key, get_key_range,
split_rank_key, and reduce_range (these call calculate_reduction but didn't originally
require wf). Added `forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2))`
to empty, singleton, and tabulate (these receive reducer as a parameter).

## Verification

- `scripts/validate.sh`: 4208 verified, 0 errors
- `scripts/rtt.sh`: 2613 tests, 2613 passed
- Project total holes: 141
- Chap43 holes: 81

## Technique

**Phase 1 (13 proofs):** All used `lemma_aug_view` delegation pattern. The proof fn
establishes `self@ =~= self.base_table@`, so base OrderedTable postconditions transfer
through the augmented wrapper.

**Phase 2 (4 additional proofs):** Lifted closure `requires` into function preconditions.
For StEph, added reducer totality to wf and removed external_body from calculate_reduction
(replaced with requires + loop invariant), join_key (wf provides condition), and clone
(eq/clone bridge). For StPer, same approach plus a centralized clone-bridge proof fn
for the persistent pattern where methods return new structs.
