# Agent 3 Round 35 Report

## Summary

Proved ordering operations in AugOrderedTableStEph.rs and AugOrderedTableStPer.rs
by removing `external_body` annotations and adding `lemma_aug_view` proof hints.
The key insight: `AugOrderedTable@` is defined as `self.base_table@`, so the
base OrderedTable's ensures flow directly through the delegation.

## Holes Before/After

| # | Chap | File | Before | After | Removed |
|---|------|------|--------|-------|---------|
| 1 | 43 | AugOrderedTableStEph.rs | 10 | 3 | 7 |
| 2 | 43 | AugOrderedTableStPer.rs | 8 | 2 | 6 |
| **Total** | | | **18** | **5** | **-13** |

## Functions Proved

### AugOrderedTableStEph.rs (7 external_body removed)

| # | Chap | Function | Technique |
|---|------|----------|-----------|
| 1 | 43 | first_key | lemma_aug_view + delegation |
| 2 | 43 | last_key | lemma_aug_view + delegation |
| 3 | 43 | previous_key | lemma_aug_view + delegation |
| 4 | 43 | next_key | lemma_aug_view + delegation |
| 5 | 43 | rank_key | lemma_aug_view + delegation |
| 6 | 43 | select_key | lemma_aug_view + delegation |
| 7 | 43 | map | lemma_aug_view + delegation |

### AugOrderedTableStPer.rs (6 external_body removed)

| # | Chap | Function | Technique |
|---|------|----------|-----------|
| 1 | 43 | first_key | lemma_aug_view + delegation |
| 2 | 43 | last_key | lemma_aug_view + delegation |
| 3 | 43 | previous_key | lemma_aug_view + delegation |
| 4 | 43 | next_key | lemma_aug_view + delegation |
| 5 | 43 | rank_key | lemma_aug_view + delegation |
| 6 | 43 | select_key | lemma_aug_view + delegation |

## Remaining Holes

### AugOrderedTableStEph.rs (3 remaining)

| # | Chap | Function | Type | Blocker |
|---|------|----------|------|---------|
| 1 | 43 | calculate_reduction | external_body | reducer.requires + base.size requires wf |
| 2 | 43 | join_key | external_body | base.join_key requires wf + reducer.requires |
| 3 | 43 | clone | external_body | OrderedTableStEph clone lacks ensures |

### AugOrderedTableStPer.rs (2 remaining)

| # | Chap | Function | Type | Blocker |
|---|------|----------|------|---------|
| 1 | 43 | calculate_reduction | assume | reducer.requires unprovable without trait constraint |
| 2 | 43 | join_key | assume | reducer.requires unprovable without trait constraint |

## Verification

- `scripts/validate.sh`: 4203 verified, 0 errors
- Project total holes: 146
- Chap43 holes: 85

## Technique

All 13 proofs used the same pattern: the `lemma_aug_view` proof function
establishes `self@ =~= self.base_table@`. Since the base OrderedTable
methods have ensures about `self@`, and AugOrderedTable's `self@` is
literally `self.base_table@`, the postconditions transfer directly.

The remaining holes are blocked by the reducer closure: calling
`reducer(&v1, &v2)` requires proving `reducer.requires((&v1, &v2))`,
which is not available without either (a) an `assume`, or (b) adding
`forall|v1: &V, v2: &V| reducer.requires((v1, v2))` to the function's
requires clause and cascading to all callers. The clone is blocked by
OrderedTableStEph's clone lacking `ensures cloned@ == self@`.
