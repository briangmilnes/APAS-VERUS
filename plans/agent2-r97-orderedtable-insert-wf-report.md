# Agent 2 R97 Report: Add insert_wf to OrderedTableStPer and OrderedTableMtPer

## Summary

Added `insert_wf` to both `OrderedTableStPer` (Chap43) and `OrderedTableMtPer` (Chap43).
This extends the `insert_wf` API chain from R96 (TableStEph, TableStPer) into the ordered
table layer so that AdjTableGraphMtPer callers can use it.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | OrderedTableStPer.rs | Added `insert_wf` to trait + external_body impl |
| 2 | 43 | OrderedTableMtPer.rs | Added `insert_wf` to trait + external_body impl |

## Design

`insert_wf` provides strictly stronger ensures than `insert`:

```
ensures
    table@.dom() =~= self@.dom().insert(k@),
    table@[k@] == v@,                                          // NEW: value mapping
    forall|k2| k2 != k@ && self@.contains_key(k2)             // NEW: other keys preserved
        ==> table@[k2] == self@[k2],
    table.spec_orderedtablestper_wf();
```

Both impls are `external_body`:
- StPer delegates to `self.insert(k, v)` — the value mapping is correct by construction
  but not provable from `insert`'s weak ensures alone.
- MtPer acquires the read lock, calls `inner.insert_wf(k, v)`, wraps result in new RwLock.

## Verification

- Isolate Chap43: 2573 verified, 0 errors
- Full: 5388 verified, 0 errors (2 pre-existing trigger warnings in graph code)
- RTT: 3083 passed
- PTT: 157 passed

## Steps Used

1 of 20 (read files + add trait/impl to both files + validate + test).
