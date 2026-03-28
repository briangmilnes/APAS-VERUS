# R95 Agent 1 — Strengthen OrderedTableMtPer ensures

## Objective

Strengthen ensures on `find`, `insert`, and `delete` in `OrderedTableMtPer` (Chap43)
to match the StPer/StEph Table API, unblocking downstream callers in Chap52.

## Changes

| # | Chap | File | Function | Before ensures | After ensures |
|---|------|------|----------|---------------|---------------|
| 1 | 43 | OrderedTableMtPer.rs | find | (none) | `Some(v) => contains_key && [k@]==v@, None => !contains_key` |
| 2 | 43 | OrderedTableMtPer.rs | insert | `dom().finite()` | `dom() =~= self@.dom().insert(k@), wf()` |
| 3 | 43 | OrderedTableMtPer.rs | delete | `dom().finite()` | `updated@ == self@.remove(k@), wf()` |

## Approach

The fundamental issue is the RwLock ghost view gap: `inner@` (the locked StPer table)
cannot be proven equal to `self@` (the ghost field) because `RwLockPredicate` only
carries `spec_orderedtablestper_wf()`, not view equality. The same gap exists in
first_key, last_key, previous_key, next_key, rank_key, select_key (which use
`assume(inner@ =~= self@)`).

Per the prompt guidance ("external_body with strong ensures is better than a proved
body with weak ensures"), added `#[verifier::external_body]` to find/insert/delete
with strong ensures matching the StPer API.

## Hole count

| # | Chap | File | Holes before | Holes after | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 43 | OrderedTableMtPer.rs | 0 | 3 | +3 |

The 3 new holes are `external_body` on find/insert/delete. The previous code had 0
actionable holes because the RWLOCK_GHOST assumes were classified as structural false
positives. The trade-off is intentional: strong ensures unlock ~16 assumes in
AdjTableGraphMtPer (Chap52).

## Ensures comparison with StPer

| Function | StPer ensures | MtPer ensures (new) | Match? |
|----------|--------------|-------------------|--------|
| find | `Some(v) => contains_key(k@) && [k@]==v@, None => !contains_key(k@)` | Same | Yes |
| insert | `dom() =~= self@.dom().insert(k@), wf()` | Same (wf adapted to MtPer) | Yes |
| delete | `table@ == self@.remove(k@), wf()` | Same (wf adapted to MtPer) | Yes |

## Validation

- `scripts/validate.sh`: 5383 verified, 0 errors
- `scripts/rtt.sh`: 3083 passed
- `scripts/ptt.sh`: 157 passed
- Global holes: 43
- Chap43 holes: 3 (all in OrderedTableMtPer.rs)

## What this unblocks

AdjTableGraphMtPer (Chap52) has ~16 assumes about table find/insert/delete behavior.
With strong ensures now available:
- `find` ensures let callers know `Some(v) => contains_key && value matches`
- `insert` ensures let callers know the domain grows by exactly one key
- `delete` ensures let callers know the full map is `self@.remove(k@)`

These specs are the foundation for proving AdjTableGraphMtPer's graph operations.
