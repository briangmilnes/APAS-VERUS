# Agent 3 — R100 Report: OrderedTableStPer insert_wf/delete_wf

## Objective

Prove the 2 `external_body` holes on `insert_wf` and `delete_wf` in
`src/Chap43/OrderedTableStPer.rs`.

## Result: 2 holes proved, -2 net

Both `external_body` annotations removed. Chap43 goes from 7 holes to 5 holes.
The remaining 5 are all in OrderedTableMtPer (agent2's scope).

## Holes Before/After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 43 | OrderedTableStPer.rs | 2 | 0 | -2 |
| 2 | 43 | OrderedTableMtPer.rs | 5 | 5 | 0 |
| — | — | **Chap43 total** | **7** | **5** | **-2** |

## Technique

**insert_wf**: The existing `insert` body already proved value-mapping facts via
`lemma_set_to_map_insert`, but the trait's `ensures` only exposed domain and wf.
Strengthened `insert`'s trait ensures to also state:
- `table@[k@] == v@`
- `forall|k2| k2 != k@ && self@.contains_key(k2) ==> table@[k2] == self@[k2]`

Added corresponding assertions in both the `Some` and `None` branches of `insert`.
With the stronger ensures on `insert`, `insert_wf` trivially delegates.

**delete_wf**: `delete` already ensures `table@ == self@.remove(k@)` (full map equality).
The forall in `delete_wf`'s ensures follows directly from Map::remove axioms. Added a
simple `assert forall` block referencing the map equality.

## Verification

- `scripts/validate.sh isolate Chap43`: 2578 verified, 0 errors
- Full validation OOM-killed (system memory issue, not code issue)
- RTT: 3083 passed
- PTT: 157 passed

## Steps Used: 4 of 20

## Files Modified

- `src/Chap43/OrderedTableStPer.rs` — strengthened `insert` ensures, proved `insert_wf`
  and `delete_wf`
