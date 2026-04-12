# Agent 2 — Round 201 Report

## Summary

R201 task: Iterator expansion — BST/Map/Table/Graph coverage.

Implemented full 10-component iterators for 6 new collection types and wrote
PTT files covering all 4 borrow-loop/for patterns per type.

---

## Phase 1: Audit

Reviewed 18 candidate files. Selected 6 targets based on missing iterator coverage:

| # | Chap | File | Reason Selected |
|---|------|------|-----------------|
| 1 | 37 | BSTPlainMtEph.rs | No iterator; has underlying StEph iterator |
| 2 | 37 | BSTRBMtEph.rs | No iterator; MtKey so no obeys_feq_clone needed |
| 3 | 41 | OrdKeyMap.rs | No iterator; wraps BSTParaStEph via OrdKeyMapIter |
| 4 | 42 | TableMtEph.rs | No iterator; wraps ArraySeqMtEph, yields &'a Pair<K,V> |
| 5 | 43 | OrderedSetMtEph.rs | No iterator; wraps OrderedSetStEph |
| 6 | 43 | OrderedTableMtPer.rs | No iterator; wraps OrderedTableStPer |

---

## Phase 2: Implementation

Full 10-component iterators added to all 6 files.

| # | Chap | File | Iterator Type | Pattern | Assume? |
|---|------|------|---------------|---------|---------|
| 1 | 37 | BSTPlainMtEph.rs | BSTPlainMtEphIter\<T\> | Snapshot+pos | Yes (hand-rolled) |
| 2 | 37 | BSTRBMtEph.rs | BSTRBMtEphIter\<T\> | Snapshot+pos | Yes (hand-rolled) |
| 3 | 41 | OrdKeyMap.rs | OrdKeyMapIter\<K,V\> | Snapshot+pos | Yes (hand-rolled) |
| 4 | 42 | TableMtEph.rs | TableMtEphIter\<'a,K,V\> | Wrapping (slice iter) | No (proven via inner) |
| 5 | 43 | OrderedSetMtEph.rs | OrderedSetMtEphIter\<T\> | Snapshot+pos | Yes (hand-rolled) |
| 6 | 43 | OrderedTableMtPer.rs | OrderedTableMtPerIter\<K,V\> | Wrapping (StPer iter) | No (proven via inner) |

Key design notes:
- **TableMtEph**: Iterator wraps `ArraySeqMtEphIter` (which itself wraps `std::slice::Iter`)
  yielding `&'a Pair<K,V>`. PTT loop body uses `*x` to dereference. View delegates to inner
  so `next()` needs no `assume`.
- **OrderedTableMtPer**: Wraps `OrderedTableStPerIter` (fully proven). View is `self.inner@`
  so Verus auto-connects postconditions through the delegation; no `assume` needed.
- **MtKey trait**: `BSTRBMtEph`, `OrdKeyMap`, `OrderedSetMtEph`, `OrderedTableMtPer` all
  use `MtKey` which includes `Clone`, so `obeys_feq_clone` is NOT required in test functions.

---

## Phase 3: Validation

Full codebase verify before PTTs:

```
verification results:: 5764 verified, 0 errors
```

All chapters clean. RTTs were already passing from R200 baseline.

---

## Phase 4: PTT Files

6 new PTT files written, each covering 4 patterns (loop-borrow-iter, loop-borrow-into,
for-borrow-iter, for-borrow-into):

| # | Chap | PTT File | Tests | Result |
|---|------|----------|-------|--------|
| 1 | 37 | ProveBSTPlainMtEph.rs | 4 | PASS |
| 2 | 37 | ProveBSTRBMtEph.rs | 4 | PASS |
| 3 | 41 | ProveOrdKeyMap.rs | 4 | PASS |
| 4 | 42 | ProveTableMtEph.rs | 4 | PASS |
| 5 | 43 | ProveOrderedSetMtEph.rs | 4 | PASS |
| 6 | 43 | ProveOrderedTableMtPer.rs | 4 | PASS |

`rust_verify_test/Cargo.toml` updated with all 6 new `[[test]]` entries.
New Chap42 PTT directory created.

PTT run result: **261 tests run: 261 passed, 0 skipped** (up from 237 in R199).

---

## Metrics

| Metric | Before R201 | After R201 | Delta |
|--------|-------------|------------|-------|
| Verified | 5728 | 5764 | +36 |
| PTT count | 237 | 261 | +24 |
| New iterators | — | 6 | +6 |

---

## Iterator Assumes Table

Hand-rolled snapshot iterators (not wrapping a proven `std::iter::Iterator`) require
`assume(iter_invariant_<module>(self))` in `next()` per the policy in memory. New assumes:

| # | Chap | File | Assume location |
|---|------|------|-----------------|
| 1 | 37 | BSTPlainMtEph.rs | BSTPlainMtEphIter::next() |
| 2 | 37 | BSTRBMtEph.rs | BSTRBMtEphIter::next() |
| 3 | 41 | OrdKeyMap.rs | OrdKeyMapIter::next() |
| 4 | 43 | OrderedSetMtEph.rs | OrderedSetMtEphIter::next() |

Wrapping iterators (TableMtEph, OrderedTableMtPer) have no assumes — inner's proven
ensures flow through view delegation automatically.
