# Agent 3 — R98 Report: AdjTableGraphMtPer rwlock:predicate assumes

## Summary

Proved 4 of 5 rwlock:predicate assumes in `src/Chap52/AdjTableGraphMtPer.rs` using
`insert_wf` value-level ensures + graph closure chain proofs. Also proved 2 algorithmic
assumes as bonus (postcondition facts that became provable via insert_wf).

## Technique

The key enabler was `OrderedTableMtPer::insert_wf` (added in R97), which provides:
- `updated@[k@] == v@` (value at inserted key)
- `forall|k2 != k@| self@.contains_key(k2) ==> updated@[k2] == self@[k2]` (preservation)

This unlocks graph closure proofs: for each vertex u2 in the updated graph, we can
determine whether its neighbor set is unchanged (from self, where graph closure holds)
or is `Set::empty()` (newly inserted vertex), and in either case prove all neighbors
are in the domain.

Additional tools used:
- `lemma_cloned_view_eq` (R97) for `u.clone()@ == u@`
- `lemma_len_subset` for finiteness via subset of finite domain
- Ghost variables (`u_in_orig`, `adj_after_u`, `orig_adj`) for tracking intermediate state

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 52 | AdjTableGraphMtPer.rs | 12 | 5 | -7 |

## Assumes Proved (7)

| # | Chap | File | Line | Category | What |
|---|------|------|------|----------|------|
| 1 | 52 | AdjTableGraphMtPer.rs | 323 | rwlock:predicate | `insert_vertex` graph wf after insert |
| 2 | 52 | AdjTableGraphMtPer.rs | 402 | rwlock:predicate | `insert_edge` u_neighbors set wf |
| 3 | 52 | AdjTableGraphMtPer.rs | 403 | algorithmic | `insert_edge` u_neighbors overflow bound |
| 4 | 52 | AdjTableGraphMtPer.rs | 415 | rwlock:predicate | `insert_edge` graph wf after final insert |
| 5 | 52 | AdjTableGraphMtPer.rs | 416 | algorithmic | `insert_edge` postcondition v@ in neighbors |
| 6 | 52 | AdjTableGraphMtPer.rs | 448 | rwlock:predicate | `delete_edge` graph wf after insert |
| 7 | 52 | AdjTableGraphMtPer.rs | 449 | algorithmic | `delete_edge` postcondition edge removed |

## Remaining Assumes (5)

| # | Chap | File | Line | Category | What Blocks It |
|---|------|------|------|----------|----------------|
| 1 | 52 | AdjTableGraphMtPer.rs | 228 | algorithmic | `num_edges` partial sum overflow — needs domain iteration ensures |
| 2 | 52 | AdjTableGraphMtPer.rs | 240 | algorithmic | `num_edges` sum correctness — needs domain-value correspondence |
| 3 | 52 | AdjTableGraphMtPer.rs | 362 | algorithmic | `delete_vertex` graph wf — blocked by weak `map` value ensures |
| 4 | 52 | AdjTableGraphMtPer.rs | 363 | algorithmic | `delete_vertex` dom exclusion — blocked by weak `map`+`delete` dom ensures |
| 5 | 52 | AdjTableGraphMtPer.rs | 463 | algorithmic | `insert_edge` capacity — tight overflow bound (requires +3 not +2) |

## Verification

- `scripts/validate.sh`: 5388 verified, 0 errors
- `scripts/rtt.sh`: 3083 passed
- `scripts/ptt.sh`: 157 passed
