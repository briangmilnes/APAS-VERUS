# Agent 2 — Round 59 Report

## Assignment

Close 12 capacity assumes in Chap53 graph search and priority queue files.

## Result

Chap53 was already clean (0 holes) at round start — the capacity assumes had been
replaced with proven assertions in a prior round using `lemma_len_subset` and proper
`vertex_universe` capacity requires/invariants.

Redirected effort to Chap52 AdjTableGraph assumes (the largest remaining hole cluster).

## Changes Made

### Table domain() wf ensures (Chap42)

Added `domain.spec_arraysetsteph_wf()` to the `domain()` ensures in both:
- `src/Chap42/TableStEph.rs` (trait + impl)
- `src/Chap42/TableStPer.rs` (trait + impl)

The impl already maintained `keys.spec_arraysetsteph_wf()` in its loop invariant but
didn't expose it in the ensures. Pure ensures strengthening, no body changes.

### AdjTableGraph domain wf assumes (Chap52)

Removed 2 assumes that depended on domain() wf:
- `src/Chap52/AdjTableGraphStEph.rs`: `assume(domain.spec_arraysetsteph_wf())` in `delete_vertex`
- `src/Chap52/AdjTableGraphStPer.rs`: `assume(domain.spec_arraysetsteph_wf())` in `delete_vertex`

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 42 | TableStEph.rs | 0 | 0 | 0 |
| 2 | 42 | TableStPer.rs | 0 | 0 | 0 |
| 3 | 52 | AdjTableGraphStEph.rs | 8 | 7 | -1 |
| 4 | 52 | AdjTableGraphStPer.rs | 8 | 7 | -1 |
| 5 | 52 | AdjTableGraphMtPer.rs | 21 | 21 | 0 |
| 6 | 53 | GraphSearchStEph.rs | 0 | 0 | 0 |
| 7 | 53 | GraphSearchStPer.rs | 0 | 0 | 0 |
| 8 | 53 | PQMinStEph.rs | 0 | 0 | 0 |
| 9 | 53 | PQMinStPer.rs | 0 | 0 | 0 |

**Project total: 42 → 40 holes (-2)**

## Remaining Chap52 Blockers

The remaining 7 + 7 + 21 assumes in AdjTableGraph files are all blocked by the
**table clone gap**: when `TableStEph::insert()` and `TableStEph::delete()` rebuild
their backing `ArraySeqStEphS`, they clone entries via `clone_plus()`. This preserves
views (`cloned@ == self@`) but not exec-level properties like
`spec_avltreesetsteph_wf()` of stored neighbor sets.

The `ClonePreservesWf` trait and `clone_wf()` exist and DO preserve wf, but the
table doesn't use them. Three potential fixes (all infrastructure-level):

1. **Table uses `clone_wf()`**: Add `V: ClonePreservesWf` bound to insert/delete.
   Cascades to all table callers. Most impactful.
2. **Table moves entries instead of cloning**: Refactor to use ownership transfer.
   No new bounds needed but significant implementation change.
3. **Add `spec_stored_value` preservation ensures**: Requires proving exec equality
   through clone, which isn't possible with current clone ensures.

These are outside the scope of a single agent round.

## Cascade Impacts

The `domain()` ensures strengthening is backwards-compatible — it adds a new ensures
clause without changing any requires. No callers needed updates.

## Verification

```
verification results:: 5386 verified, 0 errors
RTT: 3083 passed
PTT: 157 passed
```
