# R137 Agent 3 — Experiment: AtomicBool spinlock + PCell for Mt module

## Summary

Created `src/experiments/atomic_spinlock_pcell_mt.rs` — same 10-operation
Vec-backed set as `bst_plain_mt_tsm.rs`, but using AtomicBool spinlock + PCell
instead of RwLock + TSM. Zero assumes, zero accepts, zero external_body.

## Architecture

```
SetMtAtomic {
    atomic: AtomicBool<_, Option<PointsTo<SetInner>>, _>,  // spinlock + ghost state
    cell: PCell<SetInner>,                                  // data storage
}
```

The `struct_with_invariants!` macro ties the atomic to the cell:
- When **unlocked** (false): `PointsTo` stored in atomic ghost state. Proves cell contents.
- When **locked** (true): `PointsTo` taken by the acquiring thread. No ghost state in atomic.

## Comparison with bst_plain_mt_tsm.rs

| | RwLock + TSM | AtomicBool + PCell |
|---|---|---|
| Assumes | 0 | 0 |
| Accepts | 0 | 0 |
| External_body | 0 | 0 |
| TSM boilerplate | 68 lines (state machine + transitions) | 0 lines |
| Lock mechanism | RwLock acquire_write/acquire_read | AtomicBool CAS spinloop |
| Predicate | RwLockPredicate (8 lines) | struct_with_invariants! invariant (10 lines) |
| Write pattern | acquire_write → mutate → step TSM → release_write | CAS acquire → take → mutate → put → store release |
| Read pattern | acquire_read → borrow → release_read | CAS acquire → take → read → put → store release |
| Total verified fns | 25 | 25 |

The PCell approach eliminates the entire TSM (state machine definition, transitions,
inductive proofs). The PointsTo IS the proof — `cell.take(perm)` proves what you got,
`cell.put(perm, val)` updates the proof. No ghost count tracking needed.

## Operations implemented

All 9 operations from bst_plain_mt_tsm.rs:
1. `new_empty()` — construct PCell + AtomicBool
2. `mt_insert(&self, val)` — acquire, take, mutate, put, release
3. `mt_delete(&self, val)` — same pattern
4. `mt_contains(&self, val)` — acquire, take, read, put, release
5. `mt_size(&self)` — same
6. `mt_is_empty(&self)` — same
7. `mt_find(&self, val)` — same
8. `mt_minimum(&self)` — same
9. `mt_maximum(&self)` — same

## View question

All operations take `&self`. With `&self`, multiple threads can call insert
concurrently. A `ghost_count: Ghost<nat>` field would go stale between release
and the next acquire by another thread. Conclusion: **View via ghost field
requires `&mut self` for writes** (or accept). This experiment uses Approach B
(specs on return values only, no View).

To get View, writes would need `&mut self`, which guarantees sole ownership
and lets the ghost field stay in sync. But `&mut self` prevents concurrent
writes, which is the whole point of the spinlock.

## Read pattern note

PCell's `borrow` API requires `Tracked<&PointsTo<T>>` with careful lifetime
management. This experiment uses `take`/`put` for reads (slightly less efficient
at runtime — moves the value out and back). For a production Mt module, the
`borrow` pattern would avoid the move. For an experiment proving the architecture,
take/put is functionally identical.

## Validation

- Verification: 5506 verified (with experiment), 5481 (without) — 0 errors
- RTT: 3616 passed (test exercises all 9 operations)
- PTT: 221 passed
